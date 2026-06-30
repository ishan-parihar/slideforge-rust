use serde_json::{Value, json};

use crate::slide_registry::get_slide_type_info;

/// Result of validating (and optionally auto-fixing) a slide spec.
#[derive(Debug, Default)]
pub struct ValidationResult {
    /// Whether the slide spec is valid (no errors).
    pub valid: bool,
    /// Hard errors — missing required params that cannot be auto-fixed.
    pub errors: Vec<String>,
    /// Soft warnings — e.g. required params present but empty.
    pub warnings: Vec<String>,
    /// Description of auto-fixes applied by `validate_and_fix_slide`.
    pub fixes: Vec<String>,
}

impl ValidationResult {
    fn new() -> Self {
        Self {
            valid: true,
            errors: vec![],
            warnings: vec![],
            fixes: vec![],
        }
    }

    /// Mark result as invalid and record an error message.
    fn add_error(&mut self, msg: impl Into<String>) {
        self.valid = false;
        self.errors.push(msg.into());
    }

    fn add_warning(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }

    fn add_fix(&mut self, msg: impl Into<String>) {
        self.fixes.push(msg.into());
    }
}

/// Validate a slide spec against the registry schema for `slide_type`.
///
/// Checks:
/// - `slide_type` must exist in the registry.
/// - Every `required_param` must be present in `params`.
/// - Required params that are present but empty strings produce warnings.
pub fn validate_slide_spec(slide_type: &str, params: &Value) -> ValidationResult {
    let mut result = ValidationResult::new();

    // 1. Look up the slide type in the registry.
    let info = match get_slide_type_info(slide_type) {
        Some(v) => v,
        None => {
            result.add_error(format!("Unknown slide type: '{slide_type}'"));
            return result;
        }
    };

    // 2. Retrieve required_params list.
    let required_params: Vec<String> = info
        .get("required_params")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    // 3. Validate each required param.
    for param in &required_params {
        match params.get(param) {
            None => {
                result.add_error(format!(
                    "Missing required param '{param}' for slide type '{slide_type}'"
                ));
            }
            Some(Value::String(s)) if s.trim().is_empty() => {
                result.add_warning(format!(
                    "Required param '{param}' is present but empty for slide type '{slide_type}'"
                ));
            }
            Some(Value::Array(arr)) if arr.is_empty() => {
                result.add_warning(format!(
                    "Required param '{param}' is an empty array for slide type '{slide_type}'"
                ));
            }
            _ => {} // present and non-empty — OK
        }
    }

    result
}

/// Validate a slide spec and attempt to apply safe automatic fixes.
///
/// Fixes applied:
/// - `hero`  — missing `subheadline` → set to `""`
/// - `list`  — `items` is empty array → insert a placeholder string
/// - `cta`   — missing `button_text` → set to `"Learn More"`
///
/// Returns a `ValidationResult` describing errors, warnings, and fixes.
pub fn validate_and_fix_slide(slide_type: &str, params: &mut Value) -> ValidationResult {
    // First run the pure validation pass.
    let mut result = validate_slide_spec(slide_type, params);

    // Ensure params is an object so we can mutate it.
    let obj = match params.as_object_mut() {
        Some(o) => o,
        None => return result,
    };

    match slide_type {
        "hero" => {
            if !obj.contains_key("subheadline") {
                obj.insert("subheadline".to_string(), json!(""));
                result.add_fix("hero: added default empty 'subheadline'");
            }
        }
        "list" => {
            if let Some(Value::Array(items)) = obj.get("items") {
                if items.is_empty() {
                    obj.insert(
                        "items".to_string(),
                        json!(["Add your first list item here"]),
                    );
                    result.add_fix("list: replaced empty 'items' array with placeholder item");
                }
            }
        }
        "cta" => {
            if !obj.contains_key("button_text") {
                obj.insert("button_text".to_string(), json!("Learn More"));
                result.add_fix("cta: set default 'button_text' to \"Learn More\"");
            }
        }
        // quote: 'author' is optional — no fix needed.
        _ => {}
    }

    // Re-run validation after fixes so `valid` and `errors` reflect the final state.
    let post_fix = validate_slide_spec(slide_type, params);
    result.valid = post_fix.valid;
    result.errors = post_fix.errors;
    result.warnings = post_fix.warnings;

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ── validate_slide_spec ──────────────────────────────────────────────────

    #[test]
    fn test_unknown_slide_type() {
        let params = json!({});
        let r = validate_slide_spec("banana_slide", &params);
        assert!(!r.valid);
        assert!(r.errors[0].contains("Unknown slide type"));
    }

    #[test]
    fn test_hero_missing_headline() {
        let params = json!({});
        let r = validate_slide_spec("hero", &params);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("headline")));
    }

    #[test]
    fn test_hero_valid() {
        let params = json!({ "headline": "Welcome to SlideForge" });
        let r = validate_slide_spec("hero", &params);
        assert!(r.valid, "errors: {:?}", r.errors);
        assert!(r.errors.is_empty());
    }

    #[test]
    fn test_hero_empty_headline_warns() {
        let params = json!({ "headline": "   " });
        let r = validate_slide_spec("hero", &params);
        // Empty string headline is still "present" but we warn.
        assert!(!r.warnings.is_empty());
    }

    #[test]
    fn test_list_empty_items_warns() {
        let params = json!({ "title": "My List", "items": [] });
        let r = validate_slide_spec("list", &params);
        assert!(r.warnings.iter().any(|w| w.contains("items")));
    }

    #[test]
    fn test_cta_missing_button_text() {
        let params = json!({ "headline": "Get started today" });
        let r = validate_slide_spec("cta", &params);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("button_text")));
    }

    // ── validate_and_fix_slide ───────────────────────────────────────────────

    #[test]
    fn test_fix_hero_adds_subheadline() {
        let mut params = json!({ "headline": "Hello World" });
        let r = validate_and_fix_slide("hero", &mut params);
        assert!(r.valid, "errors: {:?}", r.errors);
        assert_eq!(params["subheadline"], "");
        assert!(r.fixes.iter().any(|f| f.contains("subheadline")));
    }

    #[test]
    fn test_fix_list_placeholder_item() {
        let mut params = json!({ "title": "Steps", "items": [] });
        let r = validate_and_fix_slide("list", &mut params);
        let items = params["items"].as_array().unwrap();
        assert!(!items.is_empty());
        assert!(r.fixes.iter().any(|f| f.contains("placeholder")));
    }

    #[test]
    fn test_fix_cta_default_button_text() {
        let mut params = json!({ "headline": "Join us" });
        let r = validate_and_fix_slide("cta", &mut params);
        assert_eq!(params["button_text"], "Learn More");
        assert!(r.fixes.iter().any(|f| f.contains("button_text")));
        // After fix, should be valid.
        assert!(r.valid, "errors: {:?}", r.errors);
    }

    #[test]
    fn test_quote_missing_author_is_ok() {
        // 'author' is optional for quote — only 'quote' is required.
        let mut params = json!({ "quote": "\"The best tool is the one you use.\"" });
        let r = validate_and_fix_slide("quote", &mut params);
        assert!(r.valid, "errors: {:?}", r.errors);
        assert!(r.fixes.is_empty());
    }

    #[test]
    fn test_validate_design_warning() {
        let html = r#"
            <div class="slide bg-dark">
                <div style="position:absolute;inset:0;background-image:url('test.jpg');opacity:0.5;z-index:0;"></div>
                <h1 style="color:#ffffff;">My Large Title</h1>
                <p style="color:#cccccc;text-shadow:0 2px 4px rgba(0,0,0,0.5);">My Shadowed Text</p>
            </div>
        "#;
        let report = validate_design(html);
        assert_eq!(report.slide_count, 1);
        assert_eq!(report.warning_count, 1);
        assert!(!report.issues.is_empty());
        assert_eq!(report.issues[0].r#type, "contrast");
        assert!(report.issues[0].detail.contains("My Large Title"));
        // The shadowed text should not trigger warning
        assert!(!report.issues[0].detail.contains("My Shadowed Text"));
    }

    #[test]
    fn test_validate_design_framed_image_text_is_not_background_warning() {
        let html = r#"
            <div class="slide bg-light">
                <div style="display:grid;grid-template-columns:1fr 1fr;gap:20px;">
                    <div style="position:relative;width:100%;height:240px;border-radius:12px;overflow:hidden;">
                        <img src="test.jpg" style="display:block;width:100%;height:100%;object-fit:cover;" />
                    </div>
                    <div>
                        <h2 style="color:#111827;">Visible Caption</h2>
                        <p style="color:#374151;">Readable supporting copy.</p>
                    </div>
                </div>
            </div>
        "#;
        let report = validate_design(html);
        assert_eq!(report.error_count, 0);
        assert_eq!(report.warning_count, 0);
    }

    #[test]
    fn test_validate_design_low_opacity_content_image_errors() {
        let html = r#"
            <div class="slide bg-light">
                <img src="test.jpg" style="display:block;width:100%;height:240px;object-fit:cover;opacity:0.32;" />
            </div>
        "#;
        let report = validate_design(html);
        assert_eq!(report.error_count, 1);
        assert_eq!(report.issues[0].r#type, "image_visibility");
    }
}

use crate::design_system::contrast_ratio;
use regex::Regex;

#[derive(Debug, serde::Serialize)]
pub struct DesignIssue {
    pub slide: usize,
    pub r#type: String,
    pub severity: String,
    pub detail: String,
    pub message: String,
    pub suggestion: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ValidationReport {
    pub passed: bool,
    pub issues: Vec<DesignIssue>,
    pub slide_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
}

fn style_value<'a>(style: &'a str, property: &str) -> Option<&'a str> {
    style.split(';').find_map(|decl| {
        let (key, value) = decl.split_once(':')?;
        if key.trim().eq_ignore_ascii_case(property) {
            Some(value.trim())
        } else {
            None
        }
    })
}

fn first_hex_color(value: &str) -> Option<String> {
    let re = Regex::new(r"#([0-9a-fA-F]{6})\b").ok()?;
    re.captures(value)
        .and_then(|cap| cap.get(0).map(|m| m.as_str().to_string()))
}

fn inline_contrast(style: &str) -> Option<f32> {
    let fg = style_value(style, "color").and_then(first_hex_color)?;
    let bg = style_value(style, "background-color")
        .and_then(first_hex_color)
        .or_else(|| style_value(style, "background").and_then(first_hex_color))?;
    Some(contrast_ratio(&fg, &bg))
}

fn numeric_style_value(style: &str, property: &str) -> Option<f32> {
    let raw = style_value(style, property)?;
    let numeric = raw
        .trim()
        .trim_end_matches("px")
        .trim_end_matches('%')
        .parse::<f32>()
        .ok()?;
    Some(numeric)
}

#[derive(Clone, Copy)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    let gap = 2.0;
    a.x < b.x + b.w + gap && a.x + a.w + gap > b.x && a.y < b.y + b.h + gap && a.y + a.h + gap > b.y
}

pub fn validate_design(html: &str) -> ValidationReport {
    let mut issues = Vec::new();

    // Split the HTML into slides. Each slide starts with <div class="slide
    let slide_starts: Vec<_> = html
        .match_indices("<div class=\"slide")
        .map(|(idx, _)| idx)
        .collect();
    let mut slides = Vec::new();
    for i in 0..slide_starts.len() {
        let start = slide_starts[i];
        let end = if i + 1 < slide_starts.len() {
            slide_starts[i + 1]
        } else {
            html.len()
        };
        slides.push(&html[start..end]);
    }

    let slide_count = slides.len().max(1);

    // Regex for text tags without backreferences
    let text_tag_re =
        Regex::new(r#"(?s)<(p|h[1-6]|span|div)\s*([^>]*?)>(.*?)</(p|h[1-6]|span|div)>"#).unwrap();
    let style_re = Regex::new(r#"style="([^"]*?)""#).unwrap();
    let img_re = Regex::new(r#"<img\s+[^>]*style="([^"]*)""#).unwrap();
    let frame_re = Regex::new(r#"<div\s+style="([^"]*position:absolute;[^"]*left:[^"]*top:[^"]*width:[^"]*height:[^"]*)"[^>]*>\s*<div[^>]*>\s*<img"#).unwrap();

    for (slide_idx, slide_html) in slides.iter().enumerate() {
        let slide_num = slide_idx + 1;

        let has_background_image = slide_html.contains("background-image")
            || slide_html.contains("background-size:cover")
            || slide_html.contains("background-size: cover");

        for img_cap in img_re.captures_iter(slide_html) {
            let style = img_cap.get(1).map(|m| m.as_str()).unwrap_or("");
            if let Some(opacity) = numeric_style_value(style, "opacity") {
                if opacity < 0.75 {
                    issues.push(DesignIssue {
                        slide: slide_num,
                        r#type: "image_visibility".to_string(),
                        severity: "error".to_string(),
                        detail: format!("Content image opacity is {:.2}, which can make the image appear washed out.", opacity),
                        message: "Content image opacity is too low for a primary image.".to_string(),
                        suggestion: "Keep primary content images near full opacity; reserve opacity controls for background images and overlays.".to_string(),
                    });
                }
            }
        }

        let mut frames = Vec::new();
        for cap in frame_re.captures_iter(slide_html) {
            let style = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let Some(x) = numeric_style_value(style, "left") else {
                continue;
            };
            let Some(y) = numeric_style_value(style, "top") else {
                continue;
            };
            let Some(w) = numeric_style_value(style, "width") else {
                continue;
            };
            let Some(h) = numeric_style_value(style, "height") else {
                continue;
            };
            frames.push(Rect { x, y, w, h });
        }
        for i in 0..frames.len() {
            for j in (i + 1)..frames.len() {
                if rects_overlap(frames[i], frames[j]) {
                    issues.push(DesignIssue {
                        slide: slide_num,
                        r#type: "image_frame_overlap".to_string(),
                        severity: "error".to_string(),
                        detail: format!("Image frames {} and {} overlap.", i + 1, j + 1),
                        message: "Image collage frames overlap each other.".to_string(),
                        suggestion: "Use non-overlapping frame slots or increase gap/available canvas height.".to_string(),
                    });
                }
            }
        }

        // Find all text elements inside this slide
        for cap in text_tag_re.captures_iter(slide_html) {
            let tag_open = &cap[1];
            let attrs = &cap[2];
            let text_content = cap[3].trim();
            let tag_close = &cap[4];

            if tag_open != tag_close {
                continue;
            }

            // Skip if the text content is empty or contains only tags
            let plain_text = Regex::new(r"<[^>]*>")
                .unwrap()
                .replace_all(text_content, "")
                .trim()
                .to_string();
            if plain_text.is_empty() || plain_text.len() < 3 {
                continue;
            }

            // Get inline style of the text element
            let mut style_str = "";
            if let Some(style_cap) = style_re.captures(attrs) {
                style_str = style_cap.get(1).map(|m| m.as_str()).unwrap_or("");
            }

            let has_bg =
                style_str.contains("background:") || style_str.contains("background-color:");
            let has_shadow = style_str.contains("text-shadow:");
            if let Some(ratio) = inline_contrast(style_str) {
                if ratio < 4.5 {
                    let display_text = if plain_text.len() > 20 {
                        format!("{}...", &plain_text[..20])
                    } else {
                        plain_text.clone()
                    };
                    issues.push(DesignIssue {
                        slide: slide_num,
                        r#type: "contrast".to_string(),
                        severity: "error".to_string(),
                        detail: format!("Text '{}' has {:.2}:1 inline contrast.", display_text, ratio),
                        message: "Text color does not meet minimum contrast against its inline background.".to_string(),
                        suggestion: "Use a contrast-safe text color or a darker/lighter backing surface.".to_string(),
                    });
                }
            }

            if has_background_image && !has_bg && !has_shadow {
                let display_text = if plain_text.len() > 20 {
                    format!("{}...", &plain_text[..20])
                } else {
                    plain_text.clone()
                };

                issues.push(DesignIssue {
                    slide: slide_num,
                    r#type: "contrast".to_string(),
                    severity: "warning".to_string(),
                    detail: format!("Text '{}' is placed directly over an image background without a backing shape or text-shadow.", display_text),
                    message: format!("Text '{}' is placed directly over an image background without a backing shape or text-shadow.", display_text),
                    suggestion: "Wrap text in a card with semi-transparent background (glassmorphism), add a dark overlay over the image, or add a text-shadow.".to_string(),
                });
            }
        }
    }

    let error_count = issues.iter().filter(|i| i.severity == "error").count();
    let warning_count = issues.iter().filter(|i| i.severity == "warning").count();
    let info_count = issues.iter().filter(|i| i.severity == "info").count();

    ValidationReport {
        passed: error_count == 0,
        issues,
        slide_count,
        error_count,
        warning_count,
        info_count,
    }
}
