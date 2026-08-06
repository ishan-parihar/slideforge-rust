//! HTML → PNG export via the Blitz rendering engine (stylo layout + vello raster).
//!
//! This is the Chromium-free export backend. It replaces the former
//! `headless_chrome` path entirely: no browser download, no subprocess, and a
//! ~6× smaller memory footprint (~90 MB peak vs ~550 MB for Chrome).
//!
//! The Blitz stack (0.3.0-beta.1) is used with `blitz-net`'s `Provider`, which
//! resolves `file://`, `data:`, and `http(s)` sub-resources natively — so
//! image-backed slides, Google Fonts `<link>`s, and per-slide stylesheets all
//! render without a custom network provider.
//!
//! Architecture:
//! - `render_document_to_png` — the raw engine call: parse HTML → resolve →
//!   wait for sub-resource fetches → paint to a vello-cpu RGBA buffer → PNG.
//! - `render_html_to_png` — single-document preview (backed `preview-slide`).
//! - `export_slides` — per-slide extraction from a carousel document + one
//!   standalone render per slide at the target canvas size (backed `export`).
//!
//! All entry points are SYNCHRONOUS. Blitz's `HtmlDocument` is not `Send`,
//! so holding it across a `.await` would make any enclosing async future
//! non-`Send` and break the MCP tool macro's Send bound. Instead the resolve
//! wait loop uses `std::thread::sleep`: the `blitz-net` provider fetches on
//! its own tokio tasks, so sub-resources still complete while we block.

use anyrender::{PaintScene as _, render_to_buffer};
use anyrender_vello_cpu::VelloCpuImageRenderer;
use base64::Engine as _;
use blitz_dom::{DocumentConfig, util::Color};
use blitz_html::HtmlDocument;
use blitz_net::Provider;
use blitz_paint::paint_scene;
use blitz_traits::shell::{ColorScheme, Viewport};
use peniko::Fill;
use peniko::kurbo::Rect;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use url::Url;

/// Render an HTML document to PNG bytes at the given physical pixel size.
///
/// `width`/`height` are the LOGICAL layout dimensions; `scale` is the device
/// pixel ratio (output pixels = logical × scale). The document is laid out at
/// `width`×`height` CSS pixels and rasterized at `width*scale`×`height*scale`,
/// which is how HiDPI export works (e.g. 420×525 logical at 2.571 → 1080×1350).
pub fn render_document_to_png(
    html: &str,
    base_url: &str,
    width: u32,
    height: u32,
    scale: f32,
) -> Result<Vec<u8>, String> {
    let net = Arc::new(Provider::new(None));
    let phys_w = (width as f64 * scale as f64).round() as u32;
    let phys_h = (height as f64 * scale as f64).round() as u32;

    let mut document = HtmlDocument::from_html(
        html,
        DocumentConfig {
            base_url: Some(base_url.to_string()),
            net_provider: Some(Arc::clone(&net) as _),
            viewport: Some(Viewport::new(phys_w, phys_h, scale, ColorScheme::Light)),
            ..Default::default()
        },
    );

    // Poll resolve until all sub-resources (images, fonts, stylesheets) are
    // fetched. Bounded so pathological pages can't hang the exporter. Blocks
    // on the calling thread; the provider's fetch tasks run on the ambient
    // tokio runtime's worker threads.
    let mut rounds = 0u32;
    loop {
        document.resolve(0.0);
        if net.is_empty() || rounds > 500 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
        rounds += 1;
    }
    document.as_mut().resolve(0.0);

    // Paint the document into an RGBA buffer via vello-cpu (no GPU needed).
    let buffer = render_to_buffer::<VelloCpuImageRenderer, _>(
        |scene| {
            scene.fill(
                Fill::NonZero,
                Default::default(),
                Color::WHITE,
                Default::default(),
                &Rect::new(0.0, 0.0, phys_w as f64, phys_h as f64),
            );
            paint_scene(scene, document.as_mut(), scale as f64, phys_w, phys_h, 0, 0);
        },
        phys_w,
        phys_h,
    );

    // Encode to PNG.
    let mut out = Vec::new();
    let mut encoder = png::Encoder::new(&mut out, phys_w, phys_h);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
    writer
        .write_image_data(&buffer)
        .map_err(|e| e.to_string())?;
    writer.finish().map_err(|e| e.to_string())?;
    Ok(out)
}

/// Render a single HTML document (a preview wrapper) to a PNG file.
///
/// The input file is rendered at an 800×1000 logical viewport (matching the
/// former Chrome preview window) at 1:1 scale. The slide fragment centers
/// itself via the body flexbox the caller wraps it in.
pub fn render_html_to_png(html_path: &str, output_path: &str, _scale: f32) -> Result<(), String> {
    let abs_html_path = fs::canonicalize(html_path)
        .map_err(|e| format!("Could not canonicalize HTML path: {}", e))?;
    let html = fs::read_to_string(&abs_html_path).map_err(|e| e.to_string())?;
    // Deterministic fonts: inline every Google-Fonts stylesheet as data-URI
    // @font-face rules so no per-glyph async-fetch race can hit the preview.
    let html = crate::font_vendor::vendor_font_links(&html, crate::font_vendor::font_cache_dir().as_deref());
    let file_url = Url::from_file_path(&abs_html_path)
        .map_err(|_| "Could not build file:// URL for HTML path".to_string())?;

    let png = render_document_to_png(&html, file_url.as_str(), 800, 1000, 1.0)?;
    fs::write(output_path, png).map_err(|e| e.to_string())?;
    Ok(())
}

// ── Carousel → per-slide extraction ──────────────────────────────────────────

/// Locate the opening-tag index of the slide element for a given slide index.
///
/// New-format carousels tag every slide with `id="slide-{index}"`. Legacy
/// decks (older harness output, campaign presets) only carry `class="slide ..."`
/// on each slide div — so when the id marker is absent we fall back to matching
/// the N-th `<div class="slide` element (slides are emitted in order, 0-based).
fn find_slide_open(html: &str, index: usize) -> Option<usize> {
    let marker = format!("id=\"slide-{}\"", index);
    if let Some(start) = html.find(&marker) {
        return html[..start].rfind('<');
    }
    // Legacy fallback: count `class="slide` (with a word boundary so nested
    // `slide-composition`/`slide-content` divs are skipped) until index.
    let class_marker = "class=\"slide";
    let mut seen = 0usize;
    let mut search_from = 0usize;
    while let Some(pos) = html[search_from..].find(class_marker) {
        let abs = search_from + pos;
        // The char after `class="slide` must not be `-` or ` ` + more class
        // words like `slide-composition`; exact `slide` then space/quote/\n.
        let after = html[abs + class_marker.len()..].chars().next();
        if matches!(after, Some(' ') | Some('\"') | Some('\n') | Some('\t')) {
            if seen == index {
                return html[..abs].rfind('<');
            }
            seen += 1;
        }
        search_from = abs + class_marker.len();
    }
    None
}

/// Extract the complete slide element (from its opening `<div ...>` through the
/// matching closing `</div>`) from a carousel document, using balanced `<div>`
/// matching so nested slide markup is captured whole.
fn extract_slide_element(html: &str, index: usize) -> Option<String> {
    let open = find_slide_open(html, index)?;
    let bytes = html.as_bytes();
    let mut depth = 0i32;
    let mut i = open;
    let n = bytes.len();
    while i < n {
        if bytes[i] == b'<' {
            if html[i..].starts_with("<div") {
                depth += 1;
                i += 4;
            } else if html[i..].starts_with("</div>") {
                depth -= 1;
                i += 6;
                if depth == 0 {
                    return Some(html[open..i].to_string());
                }
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    None
}

/// Extract the carousel-level assets a standalone slide render needs:
/// (base `<style>` blocks WITHOUT the per-slide `#slide-N` scoped blocks, the
/// target slide's own per-slide style block, all stylesheet `<link>` tags,
/// composition width, composition height).
///
/// Per-slide `#slide-N { ... }` blocks are emitted AFTER each slide div. Only
/// the block matching the target slide index is kept — including all N blocks
/// on a 210-slide deck made every standalone doc re-parse the full carousel
/// CSS (≈11s/slide). Global styles carry the layout/chrome rules; the per-slide
/// block carries that slide's css_vars (surface/type/font tokens).
fn extract_carousel_parts(html: &str, slide_index: usize) -> (String, String, String, u32, u32) {
    let style_re = Regex::new(r"(?s)<style[^>]*>.*?</style>").unwrap();
    let mut global_styles = Vec::new();
    let mut per_slide = String::new();
    // Match `#slide-{idx}` only when NOT followed by another digit, so the
    // marker for slide 1 (`#slide-1`) does not also match `#slide-10`…`#slide-19`.
    // (The `regex` crate has no look-around, so a `[^0-9]|$` boundary is used.)
    let slide_css_re =
        Regex::new(&format!(r"#slide-{}([^0-9]|$)", slide_index)).unwrap();
    for m in style_re.find_iter(html) {
        let block = m.as_str();
        if block.contains("#slide-") {
            // Only keep the scoped block(s) that target THIS slide. Append
            // rather than overwrite in case more than one block applies.
            if slide_css_re.is_match(block) {
                per_slide.push_str(block);
                per_slide.push('\n');
            }
        } else {
            global_styles.push(block.to_string());
        }
    }
    let link_re = Regex::new(r#"(?i)<link[^>]*rel=["']stylesheet["'][^>]*>"#).unwrap();
    let links: Vec<String> = link_re
        .find_iter(html)
        .map(|m| {
            // Legacy carousels may embed raw-space / comma-weight font URLs that
            // render fine in Chrome but break blitz-net's fetch (white render).
            // Sanitize the href the same way render_carousel_html does.
            let tag = m.as_str();
            if let Some(href_start) = tag.find("href=\"") {
                let href_start = href_start + "href=\"".len();
                if let Some(href_end) = tag[href_start..].find('\"') {
                    let href = &tag[href_start..href_start + href_end];
                    let fixed = crate::slides::sanitize_font_url(href);
                    let mut out = String::from(&tag[..href_start]);
                    out.push_str(&fixed);
                    out.push_str(&tag[href_start + href_end..]);
                    return out;
                }
            }
            tag.to_string()
        })
        .collect();
    // Composition dimensions come from the carousel's :root block
    // (`--slide-width` / `--slide-height`), which the renderer sets to the
    // canvas-adapted base composition size.
    let w_re = Regex::new(r"--slide-width\s*:\s*(\d+)px").unwrap();
    let h_re = Regex::new(r"--slide-height\s*:\s*(\d+)px").unwrap();
    let base_w = w_re
        .captures(html)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(420);
    let base_h = h_re
        .captures(html)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(525);
    (global_styles.join("\n"), per_slide, links.join("\n"), base_w, base_h)
}

/// Web-font family names a slide's own `#slide-N` css_vars declares, from
/// `--font-heading` / `--font-body` (or legacy `--heading` / `--body`), e.g.
/// `--font-heading: 'Playfair Display', serif;` → `playfair display`.
fn slide_font_families(per_slide_css: &str) -> Vec<String> {
    let var_re =
        Regex::new(r#"--(?:font-)?(?:heading|body)\s*:\s*['\"]?([^,'\";]+)"#).unwrap();
    var_re
        .captures_iter(per_slide_css)
        .map(|c| c.get(1).expect("family group").as_str().trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Keep only the stylesheet links whose `family=` params intersect the slide's
/// own font families. A carousel can carry 10 font pairings, but each slide
/// renders only ONE of them — embedding all 10 (each fanning out to many woff2
/// subsets) made every standalone render ≈3× slower. Drops the unrelated links
/// so the vendored doc carries only what this slide actually shapes.
///
/// Conservative by design: if no family could be parsed, or NO link matched,
/// all links are returned (can't prove a family is unused → keep everything).
fn filter_links_for_slide(links: &str, per_slide_css: &str) -> String {
    let families = slide_font_families(per_slide_css);
    if families.is_empty() || !links.contains("fonts.googleapis.com") {
        return links.to_string();
    }
    let link_re = Regex::new(r#"(?i)<link[^>]*rel=["']stylesheet["'][^>]*>"#).unwrap();
    let href_re = Regex::new(r#"href="([^"]+)""#).unwrap();
    let mut kept: Vec<String> = Vec::new();
    for m in link_re.find_iter(links) {
        let tag = m.as_str();
        let keep = href_re
            .captures(tag)
            .and_then(|c| c.get(1))
            .map(|h| {
                let href = h.as_str();
                if !href.contains("fonts.googleapis.com") {
                    return true;
                }
                let mut link_fams: Vec<String> = Vec::new();
                // Parse the query string AFTER the ? — the first family= sits
                // right after it, so splitting the whole href on & would skip
                // the leading family pair.
                let query = href.split('?').nth(1).unwrap_or("");
                for part in query.split('&') {
                    if let Some(f) = part.strip_prefix("family=") {
                        // family=Playfair+Display is ONE family whose name is
                        // URL-encoded with + for spaces — decode, don't split.
                        let name = f.split(':').next().unwrap_or("");
                        link_fams.push(name.replace("+", " ").trim().to_lowercase());
                    }
                }
                link_fams.iter().any(|lf| families.iter().any(|f| f == lf))
            })
            .unwrap_or(true);
        if keep {
            kept.push(tag.to_string());
        }
    }
    if kept.is_empty() {
        links.to_string()
    } else {
        kept.join("\n")
    }
}

/// Build a standalone document that renders exactly ONE slide at the target
/// canvas size. The carousel's style blocks + font links are preserved so
/// per-slide `#slide-N` css_vars, typology fonts, and bleed rules all apply;
/// the slide is wrapped in the same vector scale container the carousel uses
/// (`transform: scale(canvas_w / base_w)`), so output matches the Chrome
/// export exactly.
fn build_standalone_slide_doc(
    carousel: &str,
    slide_index: usize,
    slide_element: &str,
    canvas_w: u32,
    canvas_h: u32,
) -> String {
    let (styles, per_slide, links, base_w, base_h) = extract_carousel_parts(carousel, slide_index);
    // Per-slide font subsetting + deterministic vendoring: keep only the links
    // for THIS slide's families, inline them as data-URI @font-face CSS so the
    // render has zero remote font fetches and no per-glyph fallback race.
    let slide_links = filter_links_for_slide(&links, &per_slide);
    let fonts_html = crate::font_vendor::vendor_font_links(
        &slide_links,
        crate::font_vendor::font_cache_dir().as_deref(),
    );
    let scale = canvas_w as f64 / base_w as f64;
    let sf = format!("{:.6}", scale);
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
{fonts_html}
<style>
{styles}
{per_slide}
</style>
</head>
<body style="margin:0;padding:0;overflow:hidden;">

<div style="width:{canvas_w}px;height:{canvas_h}px;overflow:hidden;position:relative;margin:0 auto;">
  <div style="transform:scale({sf});transform-origin:top left;width:{base_w}px;height:{base_h}px;">
    <div class="carousel-viewport" style="width:{base_w}px;height:{base_h}px;overflow:hidden;position:relative;">
      <div class="carousel-track" style="display:flex;height:100%;">
{slide_element}
      </div>
    </div>
  </div>
</div>

</body>
</html>"#
    )
}

/// Export slides from a carousel HTML document to PNGs of the target canvas
/// size. Each slide is extracted from the carousel and rendered standalone at
/// `width`×`height` via Blitz (no JS / no browser subprocess).
///
/// `start` is a 1-based slide number to begin at; `count` is how many slides to
/// render from `start` onward (slides `[start, start+count)`, 1-based). Output
/// files keep their GLOBAL carousel index (`slide_{global}.png`) so chunked
/// exports can be invoked repeatedly into the same directory. Defaulting
/// `start=1` keeps `export_slides(path, dir, 1, N, w, h)` identical to the
/// pre-chunk behavior (renders slides 1..N).
pub fn export_slides(
    html_path: &str,
    output_dir: &str,
    start: usize,
    count: usize,
    width: u32,
    height: u32,
) -> Result<Vec<String>, String> {
    let out = Path::new(output_dir);
    fs::create_dir_all(out).map_err(|e| e.to_string())?;

    let abs_html_path = fs::canonicalize(html_path)
        .map_err(|e| format!("Could not canonicalize HTML path: {}", e))?;
    let carousel = fs::read_to_string(&abs_html_path).map_err(|e| e.to_string())?;
    let file_url = Url::from_file_path(&abs_html_path)
        .map_err(|_| "Could not build file:// URL for HTML path".to_string())?;
    let base_url = file_url.to_string();

    let mut paths = Vec::new();
    // 0-based loop range for the requested 1-based [start, start+count) span.
    for i in (start - 1)..(start - 1 + count) {
        let slide_element = extract_slide_element(&carousel, i).ok_or_else(|| {
            format!("Could not locate slide #{} in the carousel document", i + 1)
        })?;
        let doc = build_standalone_slide_doc(&carousel, i, &slide_element, width, height);
        let png = render_document_to_png(&doc, &base_url, width, height, 1.0)?;

        let slide_name = format!("slide_{}.png", i + 1);
        let slide_path = out.join(&slide_name);
        fs::write(&slide_path, png).map_err(|e| e.to_string())?;
        paths.push(slide_path.to_string_lossy().to_string());
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::extract_carousel_parts;

    /// The per-slide marker must match exactly — `#slide-1` must not also pull
    /// in the `#slide-10`…`#slide-19` scoped blocks (substring collision).
    #[test]
    fn per_slide_marker_matches_exact_slide() {
        let html = r#"<html><head>
<style>:root { --slide-width: 420px; --slide-height: 525px; }</style>
</head><body>
<div id="slide-1" class="slide slide--dark"><div class="slide-composition">ONE</div></div>
<style>#slide-1 { --surface: #123456; }</style>
<div id="slide-10" class="slide slide--dark"><div class="slide-composition">TEN</div></div>
<style>#slide-10 { --surface: #abcdef; }</style>
<div id="slide-19" class="slide slide--dark"><div class="slide-composition">NINETEEN</div></div>
<style>#slide-19 { --surface: #fedcba; }</style>
</body></html>"#;

        // Slide index 1 must only carry its own block, not slide 10's or 19's.
        let (global, per_slide, _, w, h) = extract_carousel_parts(html, 1);
        assert!(per_slide.contains("--surface: #123456"), "own block kept");
        assert!(!per_slide.contains("--surface: #abcdef"), "slide-10 block excluded");
        assert!(!per_slide.contains("--surface: #fedcba"), "slide-19 block excluded");
        assert!(global.contains("--slide-width"), "global :root kept");
        assert_eq!((w, h), (420, 525));

        // And slide 10 gets ITS block, not slide 1's.
        let (_, per_slide10, ..) = extract_carousel_parts(html, 10);
        assert!(per_slide10.contains("--surface: #abcdef"));
        assert!(!per_slide10.contains("--surface: #123456"));
    }

    /// Legacy carousels have no `#slide-` blocks — every style block is global.
    #[test]
    fn legacy_carousel_all_styles_global() {
        let html = r#"<html><head>
<style>:root { --slide-width: 420px; --slide-height: 525px; }
.slide--dark { background: var(--surface-dark, #010105); }</style>
</head><body>
<div class="slide slide--dark"><div class="slide-composition">X</div></div>
</body></html>"#;
        let (global, per_slide, ..) = extract_carousel_parts(html, 0);
        assert!(per_slide.is_empty(), "no per-slide block in legacy deck");
        assert!(global.contains(".slide--dark"));
        assert!(global.contains("--slide-width"));
    }

    /// Per-slide font subsetting: a slide declaring `--font-heading: Playfair
    /// Display` keeps only the link that carries that family, dropping the
    /// other nine pairings.
    #[test]
    fn filter_links_for_slide_keeps_only_needed_families() {
        let links = [
            r#"<link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@700&display=swap" rel="stylesheet">"#,
            r#"<link href="https://fonts.googleapis.com/css2?family=Playfair+Display:wght@300;600&family=DM+Sans:wght@400;500;600&display=swap" rel="stylesheet">"#,
            r#"<link href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@300;600&display=swap" rel="stylesheet">"#,
        ]
        .join("\n");
        let per_slide = "--font-heading: 'Playfair Display', serif;\n--font-body: 'DM Sans', sans-serif;";
        let out = super::filter_links_for_slide(&links, per_slide);
        assert!(out.contains("Playfair+Display"), "heading family link kept");
        assert!(out.contains("DM+Sans"), "body family link kept");
        assert!(!out.contains("Jakarta"), "unrelated link dropped");
        assert!(!out.contains("Grotesk"), "unrelated link dropped");
    }

    /// When no family can be parsed from the per-slide block, ALL links are
    /// kept (conservative — can't prove a family is unused).
    #[test]
    fn filter_links_for_slide_falls_back_to_all_on_unknown_families() {
        let links = r#"<link href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@300&display=swap" rel="stylesheet">"#;
        let out = super::filter_links_for_slide(links, "--primary: #123;");
        assert!(out.contains("Space+Grotesk"), "all links kept when unknown");
    }

    /// Phase-2 regression: a standalone slide doc built from a carousel with a
    /// Google-Fonts link must carry the vendored data-URI @font-face CSS and
    /// ZERO remote font references (fonts.googleapis / fonts.gstatic) — the
    /// network-blocked guarantee. The font cache is seeded so no real fetch
    /// happens during the test.
    #[test]
    fn standalone_doc_vendors_fonts_and_has_no_remote_refs() {
        use super::build_standalone_slide_doc;

        let font_url = "https://fonts.googleapis.com/css2?family=Bangers&display=swap";
        // Seed the font cache with a canned vendored stylesheet for that URL.
        let cache = std::env::temp_dir().join(format!("sf_export_font_test_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&cache);
        let key = format!("{}-{:016x}.css", crate::font_vendor::CACHE_VERSION, crate::font_vendor::fnv1a64(font_url));
        let vendored_css = "<style>@font-face { font-family:'Bangers'; src: url(data:font/woff2;base64,AAAA) format('woff2'); }</style>";
        std::fs::write(cache.join(&key), vendored_css).unwrap();

        let html = format!(
            r#"<html><head>
<link rel="stylesheet" href="{font_url}">
<style>:root {{ --slide-width: 420px; --slide-height: 525px; }}</style>
</head><body>
<div id="slide-0" class="slide slide--dark"><div class="slide-composition">X</div></div>
</body></html>"#
        );
        let slide_element = super::extract_slide_element(&html, 0).expect("slide 0");

        // SAFETY: single-threaded test; env mutation is scoped to this test.
        unsafe { std::env::set_var("SLIDEFORGE_FONT_CACHE", &cache) };
        let doc = build_standalone_slide_doc(&html, 0, &slide_element, 420, 525);
        unsafe { std::env::remove_var("SLIDEFORGE_FONT_CACHE") };
        let _ = std::fs::remove_dir_all(&cache);

        assert!(doc.contains("data:font/woff2;base64,AAAA"), "vendored font CSS present");
        assert!(
            !doc.contains("fonts.googleapis.com") && !doc.contains("fonts.gstatic.com"),
            "no remote font reference survives in the standalone doc"
        );
    }
}
