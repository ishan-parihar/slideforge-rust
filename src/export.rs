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
use blitz_dom::{DocumentConfig, util::Color};
use blitz_html::HtmlDocument;
use blitz_net::Provider;
use blitz_paint::paint_scene;
use blitz_traits::shell::{ColorScheme, Viewport};
use peniko::Fill;
use peniko::kurbo::Rect;
use regex::Regex;
use std::fs;
use std::path::Path;
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
    let file_url = Url::from_file_path(&abs_html_path)
        .map_err(|_| "Could not build file:// URL for HTML path".to_string())?;

    let png = render_document_to_png(&html, file_url.as_str(), 800, 1000, 1.0)?;
    fs::write(output_path, png).map_err(|e| e.to_string())?;
    Ok(())
}

// ── Carousel → per-slide extraction ──────────────────────────────────────────

/// Extract the complete `<div id="slide-{index}" ...>...</div>` element from a
/// carousel document, using balanced `<div>` matching so nested slide markup is
/// captured whole.
fn extract_slide_element(html: &str, index: usize) -> Option<String> {
    let marker = format!("id=\"slide-{}\"", index);
    let start = html.find(&marker)?;
    // Backtrack to the opening '<' of this element's start tag.
    let open = html[..start].rfind('<')?;
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
/// (all `<style>` blocks, all stylesheet `<link>` tags, composition width, composition height).
fn extract_carousel_parts(html: &str) -> (String, String, u32, u32) {
    let style_re = Regex::new(r"(?s)<style[^>]*>.*?</style>").unwrap();
    let styles: Vec<String> = style_re
        .find_iter(html)
        .map(|m| m.as_str().to_string())
        .collect();
    let link_re = Regex::new(r#"(?i)<link[^>]*rel=["']stylesheet["'][^>]*>"#).unwrap();
    let links: Vec<String> = link_re
        .find_iter(html)
        .map(|m| m.as_str().to_string())
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
    (styles.join("\n"), links.join("\n"), base_w, base_h)
}

/// Build a standalone document that renders exactly ONE slide at the target
/// canvas size. The carousel's style blocks + font links are preserved so
/// per-slide `#slide-N` css_vars, typology fonts, and bleed rules all apply;
/// the slide is wrapped in the same vector scale container the carousel uses
/// (`transform: scale(canvas_w / base_w)`), so output matches the Chrome
/// export exactly.
fn build_standalone_slide_doc(
    carousel: &str,
    slide_element: &str,
    canvas_w: u32,
    canvas_h: u32,
) -> String {
    let (styles, links, base_w, base_h) = extract_carousel_parts(carousel);
    let scale = canvas_w as f64 / base_w as f64;
    let sf = format!("{:.6}", scale);
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
{links}
<style>
{styles}
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

/// Export every slide in a carousel HTML document to a PNG of the target canvas
/// size. Each slide is extracted from the carousel and rendered standalone at
/// `width`×`height` via Blitz (no JS / no browser subprocess).
pub fn export_slides(
    html_path: &str,
    output_dir: &str,
    total_slides: usize,
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
    for i in 0..total_slides {
        let slide_element = extract_slide_element(&carousel, i).ok_or_else(|| {
            format!("Could not locate slide #{} in the carousel document", i + 1)
        })?;
        let doc = build_standalone_slide_doc(&carousel, &slide_element, width, height);
        let png = render_document_to_png(&doc, &base_url, width, height, 1.0)?;

        let slide_name = format!("slide_{}.png", i + 1);
        let slide_path = out.join(&slide_name);
        fs::write(&slide_path, png).map_err(|e| e.to_string())?;
        paths.push(slide_path.to_string_lossy().to_string());
    }

    Ok(paths)
}
