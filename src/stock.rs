//! Pexels stock-image integration — `stock-image` CLI / `stock_image` MCP.
//!
//! SlideForge's image slides (`image_headline`, `image_quote`, `image_caption`,
//! …) accept either a remote `http(s)` URL or a base64 `data:` URI. The blitz
//! renderer fetches `http(s)` sub-resources natively, so a Pexels CDN URL
//! works as-is in exports; `--embed` additionally downloads the top result and
//! inlines it as a `data:` URI for fully-offline, deterministic decks.
//!
//! Pexels API: `https://api.pexels.com/v1/search?query=…&orientation=…&per_page=N`
//! with the API key passed via the `Authorization` header (NOT prefixed with
//! "Bearer"). A free key is issued from pexels.com/api; rate limits default to
//! 200 req/hr / 20k req/mo. Keep the key in `PEXELS_API_KEY` so it never
//! lands in slide params.

use base64::Engine as _;
use serde::{Deserialize, Serialize};

/// A single Pexels photo with the src variants SlideForge needs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPhoto {
    pub id: u64,
    pub alt: String,
    pub photographer: String,
    pub page_url: String,
    /// 800×1200 crop — ideal full-bleed for 4:5 / instagram_portrait.
    pub portrait: String,
    /// 1200×627 crop — ideal for 16:9 / linkedin_landscape.
    pub landscape: String,
    /// 940×650 @DPR2 source — highest-quality generically-cropped variant.
    pub large2x: String,
    /// Unmodified original.
    pub original: String,
}

impl StockPhoto {
    /// Best `src` variant for a slide orientation. Portrait slides default to
    /// `portrait` (aspect-correct), landscape to `landscape`; anything else
    /// (square, hero bg) uses `large2x` for resolution.
    pub fn best_url(&self, orientation: &str) -> String {
        match orientation {
            "landscape" => {
                if !self.landscape.is_empty() {
                    self.landscape.clone()
                } else {
                    self.large2x.clone()
                }
            }
            "square" => {
                if !self.large2x.is_empty() {
                    self.large2x.clone()
                } else {
                    self.original.clone()
                }
            }
            _ => {
                if !self.portrait.is_empty() {
                    self.portrait.clone()
                } else {
                    self.large2x.clone()
                }
            }
        }
    }
}

/// Percent-encode a search query for the Pexels API path.
fn urlencode(input: &str) -> String {
    url::form_urlencoded::byte_serialize(input.as_bytes()).collect()
}

/// Build a blocking HTTP client with a browser-ish User-Agent. Pexels blocks
/// bare tool UAs (urllib/curl) with 403 on both the API and the CDN.
fn new_client() -> Result<reqwest::blocking::Client, String> {
    reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(25))
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36")
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {e}"))
}

/// Search Pexels for `query`. Returns up to `count` photos (1–10 clamped).
/// `orientation` filters results: portrait | landscape | square.
pub fn search(
    query: &str,
    orientation: &str,
    count: usize,
    api_key: &str,
) -> Result<Vec<StockPhoto>, String> {
    let client = new_client()?;
    let per_page = count.clamp(1, 10);
    let orient = match orientation {
        "landscape" | "square" => orientation,
        _ => "portrait",
    };
    let url = format!(
        "https://api.pexels.com/v1/search?query={}&orientation={}&per_page={}",
        urlencode(query),
        orient,
        per_page
    );
    let resp = client
        .get(&url)
        .header("Authorization", api_key)
        .send()
        .map_err(|e| format!("Pexels request failed: {e}"))?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().unwrap_or_default();
        return Err(format!(
            "Pexels API returned HTTP {} — {}",
            status,
            body.chars().take(160).collect::<String>()
        ));
    }
    // reqwest is built without the "json" feature — parse bytes manually.
    let body = resp
        .bytes()
        .map_err(|e| format!("Pexels response read failed: {e}"))?;
    let json: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|e| format!("Pexels response parse failed: {e}"))?;
    let photos = json["photos"].as_array().cloned().unwrap_or_default();
    let mut out = Vec::with_capacity(photos.len());
    for p in photos {
        let src = &p["src"];
        out.push(StockPhoto {
            id: p["id"].as_u64().unwrap_or(0),
            alt: p["alt"].as_str().unwrap_or("").to_string(),
            photographer: p["photographer"].as_str().unwrap_or("").to_string(),
            page_url: p["url"].as_str().unwrap_or("").to_string(),
            portrait: src["portrait"].as_str().unwrap_or("").to_string(),
            landscape: src["landscape"].as_str().unwrap_or("").to_string(),
            large2x: src["large2x"].as_str().unwrap_or("").to_string(),
            original: src["original"].as_str().unwrap_or("").to_string(),
        });
    }
    Ok(out)
}

/// Download `url` and inline it as a base64 `data:` URI. Returns
/// (data_uri, mime, size_bytes). The result is fully offline-deterministic —
/// the deck embeds the pixels, so no network is needed at render/export time.
pub fn embed_data_uri(url: &str) -> Result<(String, String, usize), String> {
    let client = new_client()?;
    let resp = client
        .get(url)
        .send()
        .map_err(|e| format!("Image download failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("Image download returned HTTP {}", resp.status()));
    }
    let bytes = resp
        .bytes()
        .map_err(|e| format!("Image read failed: {e}"))?
        .to_vec();
    let lower = url.to_lowercase();
    let mime = if lower.contains(".png") {
        "image/png"
    } else if lower.contains(".webp") {
        "image/webp"
    } else {
        "image/jpeg"
    };
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    let size = bytes.len();
    Ok((format!("data:{mime};base64,{b64}"), mime.to_string(), size))
}

/// Convenience: search then inline the best result as a data URI.
/// Returns (photo, data_uri, mime, size_bytes).
pub fn search_and_embed(
    query: &str,
    orientation: &str,
    api_key: &str,
) -> Result<(StockPhoto, String, String, usize), String> {
    let photos = search(query, orientation, 1, api_key)?;
    let photo = photos
        .into_iter()
        .next()
        .ok_or_else(|| format!("No Pexels results for '{query}'"))?;
    let url = photo.best_url(orientation);
    let (uri, mime, size) = embed_data_uri(&url)?;
    Ok((photo, uri, mime, size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_url_prefers_orientation() {
        let p = StockPhoto {
            id: 1,
            alt: String::new(),
            photographer: String::new(),
            page_url: String::new(),
            portrait: "https://cdn/portrait".into(),
            landscape: "https://cdn/landscape".into(),
            large2x: "https://cdn/large2x".into(),
            original: "https://cdn/original".into(),
        };
        assert_eq!(p.best_url("portrait"), "https://cdn/portrait");
        assert_eq!(p.best_url("landscape"), "https://cdn/landscape");
        assert_eq!(p.best_url("square"), "https://cdn/large2x");
        assert_eq!(p.best_url(""), "https://cdn/portrait");
    }

    #[test]
    fn best_url_falls_back_when_empty() {
        let p = StockPhoto {
            id: 2,
            alt: String::new(),
            photographer: String::new(),
            page_url: String::new(),
            portrait: String::new(),
            landscape: String::new(),
            large2x: "https://cdn/large2x".into(),
            original: "https://cdn/original".into(),
        };
        assert_eq!(p.best_url("portrait"), "https://cdn/large2x");
        assert_eq!(p.best_url("landscape"), "https://cdn/large2x");
    }

    #[test]
    fn urlencode_query() {
        assert_eq!(urlencode("dark portrait"), "dark+portrait");
        assert_eq!(urlencode("a/b&c"), "a%2Fb%26c");
    }
}
