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
//! output. Results are cached on disk (`~/.cache/slideforge/fonts/`, or
//! `$SLIDEFORGE_FONT_CACHE`) keyed by the stylesheet URL, so repeat exports are
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

/// Cache directory for vendored font stylesheets. `$SLIDEFORGE_FONT_CACHE`
/// wins; otherwise `~/.cache/slideforge/fonts`. `None` when neither is set
/// (rendering still works, just without the on-disk cache).
pub fn font_cache_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("SLIDEFORGE_FONT_CACHE") {
        return Some(PathBuf::from(dir));
    }
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache/slideforge/fonts"))
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
    let key = format!("{:016x}.css", fnv1a64(url));
    if let Some(file) = cache_dir.map(|d| d.join(&key)) {
        if let Ok(cached) = fs::read_to_string(&file) {
            if !cached.trim().is_empty() {
                return Some(cached);
            }
        }
    }
    let css = fetch_css(url)?;
    let inline = inline_font_css(&css, fetch_bytes)?;
    if let Some(file) = cache_dir.map(|d| d.join(&key)) {
        if let Some(dir) = file.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let _ = fs::write(file, &inline);
    }
    Some(inline)
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
        let key = format!("{:016x}.css", fnv1a64(url));
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
}
