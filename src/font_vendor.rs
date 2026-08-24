//! Deterministic web-font loading for the export renderer.
//!
//! blitz loads `@font-face` web fonts ASYNCHRONOUSLY (`fetch_font_face()` →
//! `FontInfoOverride` → `register_fonts()`), and the exporter's resolve loop
//! only waits on `net.is_empty()` — which does NOT guarantee every font
//! registration completed before the paint. When a paint races ahead of font
//! registration, text is shaped with per-character `find_font_for(query, ch)`
//! fallback that mixes a web font for one glyph and a system font for another
//! (the "different A in the header" symptom).
//!
//! This module eliminates the race by *vendoring*: resolve each Google Fonts
//! CSS2 URL at export time, download the woff2 subsets, and inline them as
//! `data:font/woff2;base64,…` `@font-face` src. A data: URI is available
//! synchronously when the document is parsed — no fetch, no race, deterministic
//! output. Results are cached on disk (`~/.cache/deckmill/fonts/`, or
//! `$DECKMILL_FONT_CACHE`) keyed by the stylesheet URL, so repeat exports are
//! offline after the first fetch.
//!
//! Failure is always graceful: if any fetch fails (offline, blocked network,
//! bogus font family), the caller falls back to the original `<link>` tags and
//! the renderer behaves as before (racy but functional). Nothing here can fail
//! an export.

use base64::Engine as _;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Modern browser UA — Google Fonts serves `woff2` (not `ttf`) only to UA
/// strings it recognizes as a current browser.
const GF_UA: &str =
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36";

/// Cache directory for vendored font stylesheets. `$DECKMILL_FONT_CACHE`
/// wins; otherwise `~/.cache/deckmill/fonts`. `None` when neither is set
/// (rendering still works, just without the on-disk cache).
pub fn font_cache_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("DECKMILL_FONT_CACHE") {
        return Some(PathBuf::from(dir));
    }
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache/deckmill/fonts"))
}

/// Small stable hash (FNV-1a) used as the cache key for a font stylesheet URL.
/// Stable across runs and Rust releases — important because the cache persists
/// on disk between processes. `pub(crate)` so the export tests can compute the
/// same cache filename instead of re-implementing the algorithm.
pub(crate) fn fnv1a64(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    h
}

/// Bump this whenever the vendored CSS post-processing changes so stale cache
/// files (written by an older binary) are not served verbatim. The glyph-race
/// fix (latin-subset collapse) changed the payload shape, so `v2` forces a
/// refetch+rewrite of every cached stylesheet once. `pub(crate)` so the export
/// tests can seed cache files with the exact on-disk key.
pub(crate) const CACHE_VERSION: &str = "v4";

/// Collapse Google Fonts' per-subset `@font-face` fan-out to ONE face per
/// (family, weight, style): the face whose `unicode-range` covers the basic
/// Latin block (`U+0000-00FF`).
///
/// WHY: Google Fonts CSS2 serves a family as several subset faces (latin,
/// latin-ext, vietnamese, cyrillic…) with different `unicode-range`s. The
/// export renderer (blitz/stylo) does NOT implement `unicode-range` — it
/// registers every face and fontdb/parley matches by family+weight+style only,
/// so a family may resolve to a *non-latin* subset (e.g. latin-ext). Those
/// subsets contain letters but often lack ASCII digits (`0x30` is absent from
/// latin-ext), so digits and other missing glyphs silently fall back to system
/// fonts — the "mixed A / numbers / whole-word lettering" export bug.
/// Keeping only the Latin face per weight makes font resolution deterministic
/// and gives every ASCII/Latin-1/punctuation glyph a single source.
///
/// Emoji and rare scripts are unaffected: parley routes them through its own
/// emoji/fallback stacks (see `select_font`'s `is_emoji` branch), and our
/// generated content is ASCII + Latin-1 + common punctuation.
///
/// Conservative: faces of a family that has NO latin subset at all (e.g. a
/// non-Latin-only family) are preserved verbatim — collapse only drops the
/// non-latin subset faces of families that DO ship a latin face.
fn collapse_to_latin_faces(css: &str) -> String {
    let face_re = Regex::new(r"(?s)@font-face\s*\{[^{}]*\}").unwrap();
    let fam_re = Regex::new(r"(?i)font-family:\s*'([^']+)'").unwrap();
    let wt_re = Regex::new(r"(?i)font-weight:\s*([^;]+)").unwrap();
    let st_re = Regex::new(r"(?i)font-style:\s*([^;]+)").unwrap();
    let ur_re = Regex::new(r"(?i)unicode-range:\s*([^;]+)").unwrap();

    // First pass: parse every block; decide per-family whether a latin face
    // exists for that family.
    struct Parsed {
        family: String,
        weight: String,
        style: String,
        is_latin: bool,
        block: String,
    }
    let mut parsed: Vec<Parsed> = Vec::new();
    let mut families_with_latin: Vec<String> = Vec::new();
    let mut any = false;
    let mut last = 0usize;
    for face in face_re.find_iter(css) {
        let block = face.as_str();
        last = face.end();
        let family = fam_re
            .captures(block)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        // Skip blocks without a parseable family (never collapse those away).
        if family.is_empty() {
            continue;
        }
        any = true;
        let weight = wt_re
            .captures(block)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        let style = st_re
            .captures(block)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        let is_latin = ur_re
            .captures(block)
            .map(|c| c.get(1).is_some_and(|m| m.as_str().contains("U+0000-00FF")))
            .unwrap_or(false);
        if is_latin && !families_with_latin.contains(&family) {
            families_with_latin.push(family.clone());
        }
        parsed.push(Parsed { family, weight, style, is_latin, block: block.to_string() });
    }
    if !any {
        return css.to_string();
    }
    // If NO family ships a latin face, keep everything unchanged.
    if families_with_latin.is_empty() {
        return css.to_string();
    }
    // Second pass: keep latin faces (one per family+weight+style) plus EVERY
    // face of families that have no latin alternative (their subsets are all
    // they have — dropping them would strand those families entirely).
    let mut kept: Vec<String> = Vec::new();
    let mut seen: Vec<(String, String, String)> = Vec::new();
    for p in &parsed {
        let key = (p.family.clone(), p.weight.clone(), p.style.clone());
        let family_has_latin = families_with_latin.contains(&p.family);
        if p.is_latin {
            if !seen.contains(&key) {
                seen.push(key);
                kept.push(p.block.clone());
            }
        } else if !family_has_latin {
            // Non-latin family: keep all its faces.
            kept.push(p.block.clone());
        }
    }
    if kept.is_empty() {
        return css.to_string();
    }
    // Preserve any trailing content after the last @font-face (comments etc.)
    let tail = &css[last..];
    format!("{}\n{}", kept.join("\n"), tail)
}

/// Merge per-weight `@font-face` rules that share the SAME variable-font blob
/// into ONE face declaring a weight RANGE (`font-weight: min max`).
///
/// WHY: Google Fonts CSS2 serves a variable font as one woff2 file per
/// (family, style) with a SEPARATE `@font-face` block per requested weight
/// (same `src`, different `font-weight`). The export renderer's fontique
/// registers faces keyed by blob and keeps only the FIRST weight for a given
/// file, so a `font-weight: 900` request falls back to the lightest registered
/// instance — headlines render thin (the "system-font / weak lineweight"
/// export bug, visible on comment_cta/definition/stat slides). A single face
/// with `font-weight: 600 900` (range syntax) lets the variable font serve any
/// weight in the range: verified empirically — identical multi-face payload
/// rendered at 14k dark pixels vs 26k for the range form at weight 900 (~2x
/// ink). Static fonts have a distinct blob per weight, so they are never
/// merged and keep their exact weight.
///
/// Runs AFTER `collapse_to_latin_faces` so the per-subset fan-out is already
/// reduced; the merge then collapses the remaining per-weight fan-out.
fn merge_variable_weight_faces(css: &str) -> String {
    let face_re = Regex::new(r"(?s)@font-face\s*\{[^{}]*\}").unwrap();
    let fam_re = Regex::new(r"(?i)font-family:\s*'([^']+)'").unwrap();
    let wt_re = Regex::new(r"(?i)font-weight:\s*([^;]+)").unwrap();
    let st_re = Regex::new(r"(?i)font-style:\s*([^;]+)").unwrap();
    let src_re = Regex::new(r"(?i)url\(([^)]+)\)").unwrap();

    struct Parsed {
        key: (String, String, String),
        weight: u32,
        block: String,
    }
    let mut parsed: Vec<Parsed> = Vec::new();
    let mut any = false;
    for face in face_re.find_iter(css) {
        let block = face.as_str();
        let family = fam_re
            .captures(block)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        if family.is_empty() {
            continue;
        }
        any = true;
        let weight = wt_re
            .captures(block)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().trim().parse::<u32>().ok())
            .unwrap_or(400);
        let style = st_re
            .captures(block)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        let src = src_re
            .captures(block)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        parsed.push(Parsed {
            key: (family, style, src),
            weight,
            block: block.to_string(),
        });
    }
    if !any {
        return css.to_string();
    }

    // Group weights per (family, style, blob). Only groups with MORE than one
    // weight are merged into a range — a single-face group is left untouched
    // (its declared weight is already exact).
    let mut groups: Vec<(String, String, String, u32, u32, String)> = Vec::new(); // key, min, max, sample block
    let mut seen: Vec<(String, String, String)> = Vec::new();
    for p in &parsed {
        if let Some(g) = groups.iter_mut().find(|g| g.0 == p.key.0 && g.1 == p.key.1 && g.2 == p.key.2) {
            if p.weight < g.3 {
                g.3 = p.weight;
            }
            if p.weight > g.4 {
                g.4 = p.weight;
            }
        } else {
            groups.push((p.key.0.clone(), p.key.1.clone(), p.key.2.clone(), p.weight, p.weight, p.block.clone()));
        }
    }
    if groups.len() == parsed.len() {
        return css.to_string(); // no group has more than one weight → nothing to merge
    }

    // Rewrite one block per merged group: `font-weight: N;` → `font-weight: min max;`.
    let weight_line_re =
        Regex::new(r"(?i)font-weight:\s*[^;]+;").unwrap();
    let mut out = String::with_capacity(css.len());
    let mut last = 0usize;
    for face in face_re.find_iter(css) {
        let block = face.as_str();
        out.push_str(&css[last..face.start()]);
        last = face.end();
        let family = fam_re
            .captures(block)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        let style = st_re
            .captures(block)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        let src = src_re
            .captures(block)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let g = groups
            .iter()
            .find(|g| g.0 == family && g.1 == style && g.2 == src);
        match g {
            Some(g) if g.3 != g.4 => {
                // Emit the merged range only for the FIRST face of the group
                // (all blocks in a group are identical except font-weight, so
                // comparing to the sample block identity via family/style/src
                // is enough — subsequent faces of the group are skipped).
                if block == g.5 {
                    let merged = format!("font-weight: {} {};", g.3, g.4);
                    let out_block = weight_line_re.replace(block, merged.as_str()).to_string();
                    out.push_str(&out_block);
                }
            }
            _ => out.push_str(block),
        }
    }
    out.push_str(&css[last..]);
    out
}

/// Rewrite a Google Fonts CSS2 stylesheet so every `@font-face` `src` that
/// points at a remote `http(s)` URL becomes a `data:font/woff2;base64,…` URI.
///
/// `fetch_bytes` is injected for testability; it must return the woff2 bytes
/// for a remote URL. Repeated remote URLs are fetched once and reused.
///
/// Returns `None` if any required fetch fails, OR if the stylesheet produced
/// zero inlined fonts (e.g. Google answered 200 with an unusable/empty page) —
/// in both cases the caller keeps the original `<link>` as the fallback.
/// Non-`@font-face` regions (comments, `/* latin */` annotations) and
/// non-remote `url(...)` values pass through untouched, and `unicode-range`
/// declarations are preserved verbatim.
pub fn inline_font_css(css: &str, fetch_bytes: &dyn Fn(&str) -> Option<Vec<u8>>) -> Option<String> {
    // Google Fonts CSS2 emits flat `@font-face { ... }` blocks (no nesting).
    let face_re = Regex::new(r"(?s)@font-face\s*\{[^{}]*\}").unwrap();
    // url(https://...woff2) | url('https://...') | url("https://...")
    // (the `regex` crate has no backreferences, so use alternation groups)
    let url_re =
        Regex::new(r#"(?i)url\(\s*(?:'([^']+)'|"([^"]+)"|([^'")\s]+))\s*\)"#).unwrap();

    let mut fetched: HashMap<&str, Option<Vec<u8>>> = HashMap::new();
    let mut produced = false;
    let mut rebuilt = String::with_capacity(css.len() + 4096);
    let mut last = 0usize;
    for face in face_re.find_iter(css) {
        rebuilt.push_str(&css[last..face.start()]);
        last = face.end();
        let block = face.as_str();
        let mut out_block = String::with_capacity(block.len());
        let mut bl = 0usize;
        for cap in url_re.captures_iter(block) {
            let full = cap.get(0).expect("group 0 present");
            let target = cap
                .get(1)
                .or_else(|| cap.get(2))
                .or_else(|| cap.get(3))
                .expect("one url group matches")
                .as_str();
            out_block.push_str(&block[bl..full.start()]);
            if target.starts_with("http://") || target.starts_with("https://") {
                let bytes = match fetched.get(target) {
                    Some(Some(b)) => Some(b.clone()),
                    Some(None) => None,
                    None => {
                        let r = fetch_bytes(target);
                        fetched.insert(target, r.clone());
                        r
                    }
                };
                match bytes {
                    Some(bytes) => {
                        // Google Fonts CSS2 with a modern UA serves woff2; the
                        // format() hint in the block is preserved verbatim.
                        let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
                        out_block.push_str(&format!("url(data:font/woff2;base64,{b64})"));
                        produced = true;
                    }
                    None => return None,
                }
            } else {
                out_block.push_str(full.as_str());
            }
            bl = full.end();
        }
        out_block.push_str(&block[bl..]);
        rebuilt.push_str(&out_block);
    }
    rebuilt.push_str(&css[last..]);
    if !produced {
        return None;
    }
    Some(rebuilt)
}

/// GET a stylesheet URL with a shared client. `None` on network/HTTP failure.
fn fetch_css_with(client: &reqwest::blocking::Client, url: &str) -> Option<String> {
    let resp = client.get(url).header("Accept", "text/css").send().ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.text().ok()
}

/// GET a woff2 file (fonts.gstatic.com or similar) with a shared client.
fn fetch_bytes_with(client: &reqwest::blocking::Client, url: &str) -> Option<Vec<u8>> {
    let resp = client.get(url).send().ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.bytes().ok().map(|b| b.to_vec())
}

/// Build one blocking client reused for every request in a vendor pass (one
/// TLS session for CSS + all subsets instead of a handshake per fetch).
fn new_client() -> Option<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent(GF_UA)
        .build()
        .ok()
}

/// Vendor a single Google Fonts CSS2 URL into an inline `<style>` block.
///
/// Cache hit → return cached CSS immediately (no network). Cache miss → fetch
/// the stylesheet + woff2 subsets, inline as data URIs, write the cache, and
/// return. `None` means "could not vendor" (offline / blocked / bogus family)
/// and the caller should keep the original `<link>`.
pub fn vendor_font_css(url: &str, cache_dir: Option<&Path>) -> Option<String> {
    let client = new_client()?;
    let fetch_css = |u: &str| fetch_css_with(&client, u);
    let fetch_bytes = |u: &str| fetch_bytes_with(&client, u);
    vendor_font_css_with_fetch(url, cache_dir, &fetch_css, &fetch_bytes)
}

fn vendor_font_css_with_fetch(
    url: &str,
    cache_dir: Option<&Path>,
    fetch_css: &dyn Fn(&str) -> Option<String>,
    fetch_bytes: &dyn Fn(&str) -> Option<Vec<u8>>,
) -> Option<String> {
    let key = format!("{}-{:016x}.css", CACHE_VERSION, fnv1a64(url));
    if let Some(file) = cache_dir.map(|d| d.join(&key)) {
        if let Ok(cached) = fs::read_to_string(&file) {
            if !cached.trim().is_empty() {
                return Some(cached);
            }
        }
    }
    let css = fetch_css(url)?;
    let inline = inline_font_css(&css, fetch_bytes)?;
    // Collapse the per-subset fan-out to a single latin face per weight, then
    // merge same-blob variable-font weights into a range — both BEFORE
    // caching, so cache hits and fresh fetches ship the deterministic set.
    let collapsed = collapse_to_latin_faces(&inline);
    let merged = merge_variable_weight_faces(&collapsed);
    if let Some(file) = cache_dir.map(|d| d.join(&key)) {
        if let Some(dir) = file.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let _ = fs::write(file, &merged);
    }
    Some(merged)
}

/// Replace every Google-Fonts stylesheet `<link>` in a document fragment
/// (either a carousel's extracted `<head>` links, or a full HTML document for
/// preview rendering) with an inline vendored `<style>` block. Non-font links
/// pass through unchanged. On any vendor failure the original link is kept, so
/// rendering never breaks — it degrades to the (racy) network path.
pub fn vendor_font_links(html_or_links: &str, cache_dir: Option<&Path>) -> String {
    let Some(client) = new_client() else {
        return html_or_links.to_string();
    };
    let fetch_css = |u: &str| fetch_css_with(&client, u);
    let fetch_bytes = |u: &str| fetch_bytes_with(&client, u);
    vendor_font_links_with_fetch(html_or_links, cache_dir, &fetch_css, &fetch_bytes)
}

fn vendor_font_links_with_fetch(
    html_or_links: &str,
    cache_dir: Option<&Path>,
    fetch_css: &dyn Fn(&str) -> Option<String>,
    fetch_bytes: &dyn Fn(&str) -> Option<Vec<u8>>,
) -> String {
    let link_re = Regex::new(r#"(?i)<link[^>]*rel=["']stylesheet["'][^>]*>"#).unwrap();
    let href_re = Regex::new(r#"href="([^"]+)""#).unwrap();

    let mut out = String::with_capacity(html_or_links.len() + 8192);
    let mut last = 0usize;
    for m in link_re.find_iter(html_or_links) {
        let tag = m.as_str();
        out.push_str(&html_or_links[last..m.start()]);
        if let Some(href) = href_re.captures(tag).and_then(|c| c.get(1)) {
            if href.as_str().contains("fonts.googleapis.com/css2") {
                if let Some(style) =
                    vendor_font_css_with_fetch(href.as_str(), cache_dir, fetch_css, fetch_bytes)
                {
                    // The vendored payload is raw CSS (`/* latin */\n@font-face{...}`),
                    // NOT a full <style> element. Injecting it bare into the <head>
                    // makes the HTML parser treat `@font-face{...}` as malformed
                    // markup, which can collapse the whole document into a raw-text
                    // render (flat body-gray + source text) in blitz. Wrap it.
                    out.push_str("<style>\n");
                    out.push_str(&style);
                    out.push_str("\n</style>");
                    last = m.end();
                    continue;
                }
            }
        }
        out.push_str(tag);
        last = m.end();
    }
    out.push_str(&html_or_links[last..]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The pure inlining core must rewrite every remote url(...) in an
    /// @font-face block to a data: URI while preserving the rest of the block.
    #[test]
    fn inline_font_css_turns_remote_urls_into_data_uris() {
        let css = "/* latin */\n@font-face {\n  font-family: 'Bangers';\n  font-style: normal;\n  font-weight: 400;\n  src: url(https://fonts.gstatic.com/s/bangers/v24/xxx.woff2) format('woff2');\n  unicode-range: U+0000-00FF;\n}\n";
        let out = inline_font_css(css, &|u| {
            assert!(u.starts_with("https://fonts.gstatic.com/"));
            Some(vec![0xde, 0xad, 0xbe, 0xef])
        })
        .expect("fetch succeeded");
        assert!(
            out.contains("url(data:font/woff2;base64,3q2+7w==)"),
            "data URI with base64 of the fake bytes"
        );
        assert!(!out.contains("fonts.gstatic.com"), "no remote URL remains");
        assert!(out.contains("unicode-range: U+0000-00FF"), "unicode-range kept");
        assert!(out.contains("/* latin */"), "comments kept");
        assert!(out.contains("format('woff2')"), "format hint kept");
    }

    /// Single-quoted and unquoted url(...) forms must both inline.
    #[test]
    fn inline_font_css_handles_quoted_and_unquoted_urls() {
        let css = "@font-face { src: url('https://a.gstatic.com/x.woff2') format('woff2'); }\n\
                   @font-face { src: url(https://b.gstatic.com/y.woff2) format('woff2'); }\n";
        let out = inline_font_css(css, &|_| Some(vec![1, 2, 3])).unwrap();
        assert_eq!(out.matches("data:font/woff2;base64,").count(), 2);
        assert!(!out.contains("gstatic.com"));
    }

    /// The same remote URL appearing in multiple blocks must be fetched once.
    #[test]
    fn inline_font_css_dedupes_repeated_urls() {
        let css = "@font-face { src: url(https://g/x.woff2) format('woff2'); }\n\
                   @font-face { src: url(https://g/x.woff2) format('woff2'); }\n";
        let calls = std::cell::Cell::new(0);
        let out = inline_font_css(css, &|_| {
            calls.set(calls.get() + 1);
            Some(vec![1, 2, 3])
        })
        .unwrap();
        assert_eq!(calls.get(), 1, "deduped single fetch");
        assert_eq!(out.matches("data:font/woff2;base64,").count(), 2);
    }

    /// When any woff2 fetch fails (network blocked), inlining must return None
    /// so the caller can fall back to the network <link>.
    #[test]
    fn inline_font_css_returns_none_on_fetch_failure() {
        let css = "@font-face { src: url(https://fonts.gstatic.com/x.woff2) format('woff2'); }\n";
        assert!(inline_font_css(css, &|_| None).is_none());
    }

    /// A 200 with unusable content (no @font-face with remote src) must also
    /// return None — replacing the <link> with an empty <style> would silently
    /// strand fonts on the system fallback.
    #[test]
    fn inline_font_css_returns_none_when_nothing_inlined() {
        let css = "<html><body>font not found</body></html>";
        assert!(inline_font_css(css, &|_| Some(vec![1, 2, 3])).is_none());
    }

    /// A network-blocked vendor pass must keep the original <link> intact —
    /// exports never fail on fonts, they degrade gracefully to the network
    /// path.
    #[test]
    fn vendor_links_falls_back_to_original_on_network_block() {
        let links = r#"<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Bangers&display=swap">"#;
        let out = vendor_font_links_with_fetch(links, None, &|_| None, &|_| None);
        assert_eq!(out, links, "blocked network → original link preserved");
    }

    /// Successful vendor replaces the font link with an inline style block and
    /// leaves unrelated stylesheet links untouched.
    #[test]
    fn vendor_links_inlines_fonts_and_keeps_other_links() {
        let css = "@font-face { src: url(https://fonts.gstatic.com/x.woff2) format('woff2'); }\n";
        let other = r#"<link rel="stylesheet" href="styles.css">"#;
        let links = format!(
            r#"<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Bangers&display=swap">{}"#,
            other
        );
        let out = vendor_font_links_with_fetch(
            &links,
            None,
            &|_| Some(css.to_string()),
            &|_| Some(vec![1, 2, 3]),
        );
        assert!(out.contains("data:font/woff2;base64,"), "vendored data-URI style present");
        assert!(out.contains("styles.css"), "non-font link kept");
        assert!(!out.contains("fonts.googleapis.com/css2"), "font link replaced");
    }

    /// Regression: the vendored payload is raw CSS, so it MUST be injected
    /// wrapped in a `<style>` element. Injecting it bare into the `<head>`
    /// makes the HTML parser treat `@font-face{...}` as malformed markup,
    /// collapsing the whole document into a raw-text render in blitz (the
    /// flat-body-gray + source-text export bug).
    #[test]
    fn vendor_links_wraps_vendored_css_in_style_tags() {
        let css = "/* latin */\n@font-face { src: url(https://fonts.gstatic.com/x.woff2) format('woff2'); }\n";
        let links = r#"<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Bangers&display=swap">"#;
        let out = vendor_font_links_with_fetch(
            &links,
            None,
            &|_| Some(css.to_string()),
            &|_| Some(vec![1, 2, 3]),
        );
        assert!(
            out.contains("<style>\n/* latin */") && out.trim_end().ends_with("</style>"),
            "vendored CSS wrapped in <style>…</style>: {out}"
        );
        assert!(
            !out.starts_with("/* latin */"),
            "raw CSS must not be injected bare at the document start"
        );
        assert!(
            !out.contains("\n@font-face") || out.contains("<style>"),
            "@font-face must live inside a <style> element: {out}"
        );
    }

    /// A seeded cache file must be served without any fetch (fetchers would
    /// return stale data, but the cache path must not touch them).
    #[test]
    fn vendor_font_css_serves_cache_without_fetch() {
        let dir = std::env::temp_dir().join(format!("sf_font_cache_hit_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let url = "https://fonts.googleapis.com/css2?family=Roboto&display=swap";
        let key = format!("{}-{:016x}.css", CACHE_VERSION, fnv1a64(url));
        fs::write(dir.join(&key), "<style>/* cached */</style>").unwrap();

        let got = vendor_font_css_with_fetch(
            url,
            Some(&dir),
            &|_| panic!("css fetch must not run on cache hit"),
            &|_| panic!("bytes fetch must not run on cache hit"),
        )
        .expect("cache hit");
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(got, "<style>/* cached */</style>");
    }

    /// The latin-subset collapse keeps exactly one face per (family, weight,
    /// style) — the face whose unicode-range covers U+0000-00FF — dropping the
    /// latin-ext/cyrillic/vietnamese subset faces that lack ASCII digits and
    /// would otherwise cause per-glyph system fallback in blitz.
    #[test]
    fn collapse_keeps_only_latin_face_per_weight() {
        let css = r#"
@font-face { font-family: 'DM Sans'; font-weight: 400; font-style: normal; src: url(data:font/woff2;base64,AAAA); unicode-range: U+0100-02BA, U+02BD-02C5; }
@font-face { font-family: 'DM Sans'; font-weight: 400; font-style: normal; src: url(data:font/woff2;base64,BBBB); unicode-range: U+0000-00FF, U+0131, U+2000-206F; }
@font-face { font-family: 'DM Sans'; font-weight: 500; font-style: normal; src: url(data:font/woff2;base64,CCCC); unicode-range: U+0000-00FF; }
@font-face { font-family: 'DM Sans'; font-weight: 500; font-style: normal; src: url(data:font/woff2;base64,DDDD); unicode-range: U+0400-045F; }
"#;
        let out = collapse_to_latin_faces(css);
        // latin faces kept (BBBB for 400, CCCC for 500); latin-ext (AAAA) and
        // cyrillic (DDDD) dropped.
        assert!(out.contains("BBBB"), "latin 400 face kept");
        assert!(out.contains("CCCC"), "latin 500 face kept");
        assert!(!out.contains("AAAA"), "latin-ext face dropped");
        assert!(!out.contains("DDDD"), "cyrillic face dropped");
        assert_eq!(out.matches("@font-face").count(), 2, "one face per weight");
    }

    /// When NO face covers the basic Latin block (non-Latin-only family), the
    /// stylesheet must pass through unchanged — never dropped to empty.
    #[test]
    fn collapse_passthrough_when_no_latin_face() {
        let css = r#"@font-face { font-family: 'Noto Sans JP'; src: url(data:font/woff2;base64,AAAA); unicode-range: U+3040-30FF; }"#;
        let out = collapse_to_latin_faces(css);
        assert!(out.contains("Noto Sans JP"), "no-latin family preserved");
        assert!(out.contains("U+3040-30FF"));
    }

    /// Mixed payload: a latin-capable family gets collapsed to its latin faces,
    /// while a separate non-latin family keeps ALL its faces (it has no latin
    /// alternative to fall back on — dropping them would strand the family).
    #[test]
    fn collapse_keeps_non_latin_family_faces_when_other_family_has_latin() {
        let css = r#"
@font-face { font-family: 'DM Sans'; font-weight: 400; font-style: normal; src: url(data:font/woff2;base64,AAAA); unicode-range: U+0100-02BA; }
@font-face { font-family: 'DM Sans'; font-weight: 400; font-style: normal; src: url(data:font/woff2;base64,BBBB); unicode-range: U+0000-00FF; }
@font-face { font-family: 'Noto Sans JP'; font-weight: 400; font-style: normal; src: url(data:font/woff2;base64,CCCC); unicode-range: U+3040-30FF; }
"#;
        let out = collapse_to_latin_faces(css);
        // DM Sans: latin-ext dropped, latin kept.
        assert!(!out.contains("AAAA"), "DM Sans latin-ext dropped");
        assert!(out.contains("BBBB"), "DM Sans latin face kept");
        // Noto Sans JP: preserved entirely.
        assert!(out.contains("CCCC"), "non-latin family faces preserved");
        assert!(out.contains("U+3040-30FF"));
    }

    /// Collapse must survive a Google Fonts payload with no unicode-range at
    /// all (some CDNs omit it) — every face is kept.
    #[test]
    fn collapse_passthrough_when_no_unicode_range() {
        let css = r#"@font-face { font-family: 'X'; src: url(data:font/woff2;base64,AAAA); }"#;
        let out = collapse_to_latin_faces(css);
        assert!(out.contains("@font-face"));
        assert!(out.contains("'X'"));
    }

    /// Same-blob variable-font weights must merge into ONE face with a weight
    /// RANGE. blitz/stylo's fontique keeps only the first face per blob, so the
    /// multi-face form renders every request at the lightest weight (the
    /// thin-headline export bug); the range form serves any weight in range.
    #[test]
    fn merge_collapses_same_blob_weights_to_range() {
        let css = r#"
@font-face { font-family: 'Playfair Display'; font-style: normal; font-weight: 600; src: url(data:font/woff2;base64,XXXX); }
@font-face { font-family: 'Playfair Display'; font-style: normal; font-weight: 700; src: url(data:font/woff2;base64,XXXX); }
@font-face { font-family: 'Playfair Display'; font-style: normal; font-weight: 900; src: url(data:font/woff2;base64,XXXX); }
@font-face { font-family: 'Playfair Display'; font-style: italic; font-weight: 600; src: url(data:font/woff2;base64,YYYY); }
@font-face { font-family: 'Playfair Display'; font-style: italic; font-weight: 900; src: url(data:font/woff2;base64,YYYY); }
@font-face { font-family: 'Static'; font-style: normal; font-weight: 400; src: url(data:font/woff2;base64,ZZZZ); }
"#;
        let out = merge_variable_weight_faces(css);
        // normal range 600 900 (single face emitted for the merged group)
        assert!(out.contains("font-weight: 600 900;"), "merged normal range: {out}");
        // italic range 600 900
        assert!(out.contains("font-weight: 600 900;"), "merged italic range present");
        // static single-weight face untouched (only its own weight declared)
        assert!(
            out.contains("'Static'") && out.contains("font-weight: 400;"),
            "static single-face group preserved: {out}"
        );
        // exactly 3 faces remain (2 merged + 1 static)
        assert_eq!(out.matches("@font-face").count(), 3, "merged count: {out}");
    }

    /// Faces with distinct blobs (static fonts, one blob per weight) must NOT
    /// be merged — each keeps its exact declared weight.
    #[test]
    fn merge_keeps_distinct_blobs_separate() {
        let css = r#"
@font-face { font-family: 'Mono'; font-style: normal; font-weight: 400; src: url(data:font/woff2;base64,AAA); }
@font-face { font-family: 'Mono'; font-style: normal; font-weight: 700; src: url(data:font/woff2;base64,BBB); }
"#;
        let out = merge_variable_weight_faces(css);
        assert_eq!(out.matches("@font-face").count(), 2, "no merge for distinct blobs");
        assert!(out.contains("font-weight: 400;"));
        assert!(out.contains("font-weight: 700;"));
    }

    /// Payload with no @font-face blocks passes through unchanged.
    #[test]
    fn merge_passthrough_when_no_faces() {
        let css = "/* latin */\nbody { color: red; }";
        assert_eq!(merge_variable_weight_faces(css), css);
    }
}
