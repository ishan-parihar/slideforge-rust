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
        let (primary_key, alt_key) = if slide_type == "qr_destination" {
            if param == "destination_url" {
                (param.as_str(), Some("url"))
            } else if param == "cta_text" {
                (param.as_str(), Some("button_text"))
            } else {
                (param.as_str(), None)
            }
        } else {
            (param.as_str(), None)
        };

        if let Some(alt) = alt_key {
            let primary_val = params.get(primary_key);
            let alt_val = params.get(alt);

            let is_non_empty_str_or_arr = |val: Option<&serde_json::Value>| match val {
                Some(serde_json::Value::String(s)) => !s.trim().is_empty(),
                Some(serde_json::Value::Array(arr)) => !arr.is_empty(),
                _ => false,
            };

            let is_missing_or_null_or_non_str_arr = |val: Option<&serde_json::Value>| match val {
                None | Some(serde_json::Value::Null) => true,
                Some(serde_json::Value::String(_)) | Some(serde_json::Value::Array(_)) => false,
                _ => true,
            };

            if is_missing_or_null_or_non_str_arr(primary_val)
                && is_missing_or_null_or_non_str_arr(alt_val)
            {
                result.add_error(format!(
                    "Missing required param '{primary_key}' for slide type '{slide_type}'"
                ));
            } else {
                let primary_ok = is_non_empty_str_or_arr(primary_val);
                let alt_ok = is_non_empty_str_or_arr(alt_val);

                if !primary_ok && !alt_ok {
                    if primary_val
                        .map(|v| v.is_string() || v.is_array())
                        .unwrap_or(false)
                    {
                        let val = primary_val.unwrap();
                        match val {
                            serde_json::Value::Array(_) => {
                                result.add_warning(format!(
                                    "Required param '{primary_key}' is an empty array for slide type '{slide_type}'"
                                ));
                            }
                            _ => {
                                result.add_warning(format!(
                                    "Required param '{primary_key}' is present but empty for slide type '{slide_type}'"
                                ));
                            }
                        }
                    } else if alt_val
                        .map(|v| v.is_string() || v.is_array())
                        .unwrap_or(false)
                    {
                        let val = alt_val.unwrap();
                        match val {
                            serde_json::Value::Array(_) => {
                                result.add_warning(format!(
                                    "Required param '{alt}' is an empty array for slide type '{slide_type}'"
                                ));
                            }
                            _ => {
                                result.add_warning(format!(
                                    "Required param '{alt}' is present but empty for slide type '{slide_type}'"
                                ));
                            }
                        }
                    }
                }
            }
        } else {
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
    }

    if slide_type == "qr_destination" {
        let has_heading = params
            .get("heading")
            .and_then(|v| v.as_str())
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
            || params
                .get("headline")
                .and_then(|v| v.as_str())
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false);
        let has_caption = params
            .get("caption")
            .and_then(|v| v.as_str())
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
            || params
                .get("description")
                .and_then(|v| v.as_str())
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false);
        if !has_heading && !has_caption {
            result.add_warning(
                "qr_destination should include heading or caption so users know why to scan.",
            );
        }

        let has_short_url = params
            .get("short_url")
            .and_then(|v| v.as_str())
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);
        if !has_short_url {
            result.add_warning("qr_destination should include short_url as a manual fallback for users who cannot scan.");
        }

        let cta_text_val = params
            .get("cta_text")
            .and_then(|v| v.as_str())
            .or_else(|| params.get("button_text").and_then(|v| v.as_str()))
            .unwrap_or("");
        if cta_text_val.chars().count() > 34 {
            result.add_warning(
                "qr_destination cta_text should be 34 characters or fewer for slide readability.",
            );
        }

        let dest_url_val = params
            .get("destination_url")
            .and_then(|v| v.as_str())
            .or_else(|| params.get("url").and_then(|v| v.as_str()))
            .unwrap_or("");
        if !dest_url_val.is_empty()
            && !dest_url_val.starts_with("http://")
            && !dest_url_val.starts_with("https://")
        {
            result.add_warning("qr_destination destination_url should be an absolute http(s) URL.");
        }
    }

    result
}

pub fn validate_layout(
    slide_type: &str,
    params: &Value,
    rendered_html: Option<&str>,
    aspect_ratio: Option<&str>,
) -> ValidationResult {
    let mut result = validate_slide_spec(slide_type, params);

    if let Some(html) = rendered_html.filter(|html| !html.trim().is_empty()) {
        let report = validate_design(html);
        for issue in report.issues {
            let msg = format!(
                "{}: {} Suggestion: {}",
                issue.r#type, issue.message, issue.suggestion
            );
            if issue.severity == "error" {
                result.add_error(msg);
            } else {
                result.add_warning(msg);
            }
        }
    }

    if let Some(ratio) = aspect_ratio.filter(|ratio| !ratio.trim().is_empty()) {
        if !matches!(ratio, "4:5" | "3:4" | "1:1" | "9:16" | "16:9" | "4:3") {
            result.add_warning(format!(
                "Unknown aspect ratio '{ratio}' may not preserve SlideForge composition constraints."
            ));
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

    #[test]
    fn test_validate_design_invalid_dimension_unit_errors() {
        let html = r#"
            <div class="slide bg-light">
                <div style="position:relative;width:316px;height:238;margin:0 auto;">
                    <img src="test.jpg" style="display:block;width:100%;height:100%;object-fit:cover;" />
                </div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(report.error_count >= 1);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "invalid_dimension")
        );
    }

    #[test]
    fn test_validate_design_bottom_image_caption_warns() {
        let html = r#"
            <div class="slide bg-light">
                <div style="position:relative;width:100%;height:86px;overflow:hidden;">
                    <img src="test.jpg" style="display:block;width:100%;height:100%;object-fit:cover;" />
                    <div style="padding:6px;background:rgba(0,0,0,0.4);position:absolute;bottom:0;left:0;right:0;font-size:10px;">Design Phase</div>
                </div>
                <div style="font-size:13px;margin-top:8px;">Section caption</div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "image_caption_overlay")
        );
    }

    #[test]
    fn test_validate_design_narrow_text_column_warns() {
        let html = r#"
            <div class="slide bg-light">
                <div style="width:82px;font-size:16px;line-height:1.2;">Sub 100ms latency improves global delivery</div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "text_constriction")
        );
    }

    #[test]
    fn test_validate_design_flags_descender_clipping_risk() {
        let html = r#"
            <div class="slide bg-light">
                <h2 style="font-size:42px;line-height:0.86;overflow:hidden;">Scalability</h2>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "text_vertical_clipping")
        );
    }

    #[test]
    fn test_validate_design_full_bleed_visible_overflow_is_ok_when_slide_clips() {
        // overflow:visible on full-bleed composition is now correct behavior.
        // The .slide element's overflow:hidden provides the clip boundary.
        let html = r#"
            <style>
              .slide { overflow: hidden; }
              .slide--full-bleed .slide-composition { overflow: visible; }
            </style>
            <div class="slide slide--full-bleed">
                <div class="slide-composition"><div style="position:relative;width:100%;height:100%;"></div></div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            !report
                .issues
                .iter()
                .any(|issue| issue.r#type == "aspect_bleed_overflow"),
            "Should not flag overflow:visible when slide has overflow:hidden"
        );
    }

    #[test]
    fn test_validate_design_full_bleed_visible_overflow_errors_without_slide_clip() {
        // If slide element lacks overflow:hidden, flag it.
        let html = r#"
            <style>
              .slide--full-bleed .slide-composition { overflow: visible; }
            </style>
            <div class="slide slide--full-bleed">
                <div class="slide-composition"><div style="position:relative;width:100%;height:100%;"></div></div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "aspect_bleed_overflow")
        );
    }

    #[test]
    fn test_validate_design_flags_edge_blur_without_clipping() {
        let html = r#"
            <div class="slide slide--full-bleed">
                <div class="slide-composition" style="overflow:visible;">
                    <div style="position:absolute;left:-80px;top:-80px;width:260px;height:260px;filter:blur(50px);"></div>
                </div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "edge_effect_bleed")
        );
    }

    #[test]
    fn test_validate_design_flags_one_word_per_line_risk() {
        let html = r#"
            <div class="slide bg-light">
                <p style="width:86px;font-size:18px;line-height:1.2;">Validate the funnel event map</p>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "one_word_line_risk")
        );
    }

    #[test]
    fn test_validate_design_flags_squished_component_box() {
        let html = r#"
            <div class="slide bg-light">
                <div style="width:132px;padding:24px;display:flex;flex-direction:column;box-shadow:0 4px 12px rgba(0,0,0,0.1);">
                    <h3 style="font-size:18px;">Operational Scale</h3>
                </div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "component_constriction")
        );
    }

    #[test]
    fn test_validate_design_flags_tiny_overlay_and_progress_css() {
        let html = r#"
            <style>
              .overlay__url { font-size: 9.5px; }
              .breadcrumb-chip { height: 1px; }
            </style>
            <div class="slide bg-light">
                <div class="slide__overlay"><span class="overlay__url">example.com</span></div>
                <div class="breadcrumb-progress"><div class="breadcrumb-chip"></div></div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "tiny_overlay_text")
        );
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "tiny_progress_indicator")
        );
    }

    #[test]
    fn test_validate_design_flags_thick_progress_indicator() {
        let html = r#"
            <style>
              .breadcrumb-chip { height: 6px; }
            </style>
            <div class="slide bg-light">
                <div class="breadcrumb-progress"><div class="breadcrumb-chip"></div></div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "progress_indicator_too_thick")
        );
    }

    #[test]
    fn test_validate_design_optimal_progress_thickness_passes() {
        // 2px default and 3px active should both pass
        let html = r#"
            <style>
              .breadcrumb-chip { height: 2px; }
              .breadcrumb-chip.active { height: 3px; }
            </style>
            <div class="slide bg-light">
                <div class="breadcrumb-progress"><div class="breadcrumb-chip active"></div></div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            !report
                .issues
                .iter()
                .any(|i| i.r#type == "tiny_progress_indicator"),
            "2px should not be flagged as too thin"
        );
        assert!(
            !report
                .issues
                .iter()
                .any(|i| i.r#type == "progress_indicator_too_thick"),
            "3px should not be flagged as too thick"
        );
    }

    #[test]
    fn test_validate_design_flags_tiny_inline_component_text() {
        let html = r#"
            <div class="slide bg-light">
                <span style="font-size:9px;font-weight:700;">Q1</span>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "tiny_text")
        );
    }

    #[test]
    fn test_validate_design_flags_slide_body_overflow() {
        let html = r#"
            <div class="slide bg-light">
                <div style="position:absolute;left:360px;top:120px;width:120px;height:180px;background:#fff;box-shadow:0 4px 12px rgba(0,0,0,0.1);"></div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "slide_body_overflow")
        );
    }

    #[test]
    fn test_validate_design_flags_distorted_component_aspect_ratio() {
        let html = r#"
            <div class="slide bg-light">
                <div style="width:360px;height:44px;display:grid;grid-template-columns:1fr 1fr;background:#fff;border:1px solid #ddd;"></div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "component_aspect_distortion")
        );
    }

    #[test]
    fn test_validate_design_flags_distorted_image_frame_aspect_ratio() {
        let html = r#"
            <div class="slide bg-light">
                <div style="position:relative;width:320px;height:48px;overflow:hidden;">
                    <img src="test.jpg" style="display:block;width:100%;height:100%;object-fit:cover;" />
                </div>
            </div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.r#type == "image_aspect_distortion")
        );
    }

    #[test]
    fn test_validate_layout_routes_rendered_html_issues() {
        let params = json!({ "headline": "Hello" });
        let html = r#"
            <div class="slide bg-light">
                <div style="position:absolute;left:390px;top:20px;width:80px;height:80px;background:#fff;border:1px solid #ddd;"></div>
            </div>
        "#;
        let result = validate_layout("hero", &params, Some(html), Some("9:16"));
        assert!(!result.valid);
        assert!(
            result
                .errors
                .iter()
                .any(|error| error.contains("slide_body_overflow"))
        );
    }

    #[test]
    fn test_qr_destination_requires_url_and_cta() {
        let params = json!({"heading": "Read the full guide"});
        let r = validate_slide_spec("qr_destination", &params);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("destination_url")));
        assert!(r.errors.iter().any(|e| e.contains("cta_text")));
    }

    #[test]
    fn test_qr_destination_warns_without_heading_or_caption() {
        let params = json!({
            "destination_url": "https://example.com/guide",
            "cta_text": "Scan to read"
        });
        let r = validate_slide_spec("qr_destination", &params);
        assert!(r.valid);
        assert!(r.warnings.iter().any(|w| w.contains("heading")));
    }

    #[test]
    fn test_qr_destination_accepts_alternatives() {
        let params = json!({
            "url": "https://example.com/guide",
            "button_text": "Scan to read",
            "headline": "Read the full guide",
            "short_url": "ex.co"
        });
        let r = validate_slide_spec("qr_destination", &params);
        assert!(r.valid);
        assert!(r.errors.is_empty());
        assert!(r.warnings.is_empty());
    }

    #[test]
    fn test_qr_destination_description_fallback_suppresses_warning() {
        let params = json!({
            "destination_url": "https://example.com/guide",
            "cta_text": "Scan to read",
            "description": "This is a fallback caption",
            "short_url": "ex.co"
        });
        let r = validate_slide_spec("qr_destination", &params);
        assert!(r.valid);
        assert!(r.warnings.is_empty());
    }

    #[test]
    fn test_qr_destination_empty_alternatives_warn() {
        let params = json!({
            "url": "",
            "button_text": "   ",
            "headline": "Read the full guide"
        });
        let r = validate_slide_spec("qr_destination", &params);
        assert!(r.valid);
        assert!(r.warnings.iter().any(|w| w.contains("url")));
        assert!(r.warnings.iter().any(|w| w.contains("button_text")));
    }

    #[test]
    fn test_qr_destination_warnings() {
        // Test short_url absent
        let params = json!({
            "destination_url": "https://example.com/guide",
            "cta_text": "Scan to read",
            "heading": "Scan Me"
        });
        let r = validate_slide_spec("qr_destination", &params);
        assert!(r.warnings.iter().any(|w| w.contains("short_url")));

        // Test cta_text too long (over 34 chars)
        let params = json!({
            "destination_url": "https://example.com/guide",
            "cta_text": "Scan to read the full developer guide right now",
            "short_url": "ex.co",
            "heading": "Scan Me"
        });
        let r = validate_slide_spec("qr_destination", &params);
        assert!(r.warnings.iter().any(|w| w.contains("cta_text")));

        // Test destination_url not absolute http(s)
        let params = json!({
            "destination_url": "ftp://example.com/guide",
            "cta_text": "Scan to read",
            "short_url": "ex.co",
            "heading": "Scan Me"
        });
        let r = validate_slide_spec("qr_destination", &params);
        assert!(r.warnings.iter().any(|w| w.contains("destination_url")));
    }

    #[test]
    fn test_qr_destination_fallback_null_and_non_string() {
        // Test fallback for cta_text and destination_url when primary key is null/non-string
        let params = json!({
            "destination_url": null,
            "url": "https://example.com/from-fallback",
            "cta_text": 12345, // non-string value
            "button_text": "Button Fallback",
            "heading": "Fallback Test",
            "short_url": "ex.co"
        });
        let r = validate_slide_spec("qr_destination", &params);
        // Should compile/run and be valid since fallback keys are valid non-empty strings.
        assert!(r.valid, "errors: {:?}", r.errors);
        assert!(r.warnings.is_empty(), "warnings: {:?}", r.warnings);

        // Also test when both are null/non-string (should treat as missing/error)
        let params = json!({
            "destination_url": null,
            "url": 123,
            "cta_text": true,
            "button_text": null,
            "heading": "Fallback Test",
            "short_url": "ex.co"
        });
        let r = validate_slide_spec("qr_destination", &params);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("destination_url")));
        assert!(r.errors.iter().any(|e| e.contains("cta_text")));
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

fn numeric_px_style_value(style: &str, property: &str) -> Option<f32> {
    let raw = style_value(style, property)?;
    let trimmed = raw.trim();
    if !trimmed.ends_with("px") {
        return None;
    }
    trimmed.trim_end_matches("px").parse::<f32>().ok()
}

fn style_has_unitless_dimension(style: &str) -> Option<String> {
    for property in ["width", "height", "left", "top", "right", "bottom"] {
        let Some(raw) = style_value(style, property) else {
            continue;
        };
        let trimmed = raw.trim();
        if trimmed.parse::<f32>().is_ok() {
            return Some(format!("{property}:{trimmed}"));
        }
    }
    None
}

fn text_has_descender_risk(text: &str) -> bool {
    text.chars()
        .any(|ch| matches!(ch, 'g' | 'j' | 'p' | 'q' | 'y' | 'Q' | 'J'))
}

fn has_overflow_hidden(style: &str) -> bool {
    style_value(style, "overflow")
        .map(|value| value.eq_ignore_ascii_case("hidden"))
        .unwrap_or(false)
        || style_value(style, "overflow-y")
            .map(|value| value.eq_ignore_ascii_case("hidden"))
            .unwrap_or(false)
}

fn style_has_edge_bleed_effect(style: &str) -> bool {
    let has_effect = style.contains("filter:blur")
        || style.contains("-webkit-filter:blur")
        || style.contains("box-shadow:");
    let has_negative_edge = ["left", "top", "right", "bottom"]
        .iter()
        .filter_map(|property| style_value(style, property))
        .any(|value| value.trim_start().starts_with('-'));
    has_effect && has_negative_edge
}

fn one_word_line_risk(width: f32, font_size: f32, word_count: usize) -> bool {
    word_count >= 4 && font_size >= 12.0 && width / font_size < 7.0
}

fn component_constriction_risk(style: &str) -> bool {
    let Some(width) = numeric_px_style_value(style, "width")
        .or_else(|| numeric_px_style_value(style, "max-width"))
    else {
        return false;
    };
    let padding = numeric_px_style_value(style, "padding").unwrap_or(0.0);
    let inner_width = width - (padding * 2.0);
    let component_like = style.contains("display:flex")
        || style.contains("display:grid")
        || style.contains("box-shadow:")
        || style.contains("border:");

    // Exempt small square icon badges, avatars, and progress rings:
    // containers ≤100px wide and roughly square (|w − h| < 10px) are
    // icon/badge/avatar/ring containers, not text containers. The
    // constriction check is meant for text-bearing cards/columns.
    if component_like && width <= 100.0 {
        if let Some(height) = numeric_px_style_value(style, "height") {
            if (width - height).abs() < 10.0 {
                return false;
            }
        }
    }

    // Exempt thin decorative dividers/lines: elements with height < 10px
    // or width < 10px are separators/accent lines, not text containers.
    if let Some(height) = numeric_px_style_value(style, "height") {
        if height < 10.0 || width < 10.0 {
            return false;
        }
    }

    component_like && width <= 170.0 && inner_width < 110.0
}

fn tiny_text_risk(font_size: f32) -> bool {
    // Threshold is 9.5px so that 10px micro-labels (chart axis labels, stat
    // captions, metric subscripts) pass, but anything smaller is still flagged.
    font_size > 0.0 && font_size < 9.5
}

/// Returns true if the text is a single emoji or icon glyph (non-ASCII
/// alphanumeric). Emoji icons in small badges legitimately use ~10px
/// font-size and should not trigger tiny_text warnings.
fn is_emoji_or_icon_text(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.chars().count() > 2 {
        return false;
    }
    trimmed
        .chars()
        .all(|c| !c.is_ascii_alphanumeric() && c != ' ')
}

fn component_like_style(style: &str) -> bool {
    style.contains("display:flex")
        || style.contains("display:grid")
        || style.contains("box-shadow:")
        || style.contains("border:")
        || style.contains("background:#")
        || style.contains("background:rgba")
        || style.contains("background-color:")
}

fn distorted_component_ratio(width: f32, height: f32) -> bool {
    if width < 80.0 || height < 32.0 {
        return false;
    }
    let ratio = width / height.max(1.0);
    !(0.28..=4.2).contains(&ratio)
}

fn distorted_image_ratio(width: f32, height: f32) -> bool {
    if width < 80.0 || height < 40.0 {
        return false;
    }
    let ratio = width / height.max(1.0);
    !(0.45..=2.8).contains(&ratio)
}

fn rect_overflows_slide_body(x: f32, y: f32, w: f32, h: f32) -> bool {
    const BODY_W: f32 = 420.0;
    const BODY_H: f32 = 525.0;
    x < 0.0 || y < 0.0 || x + w > BODY_W || y + h > BODY_H
}

fn has_recent_backing_container(slide_html: &str, element_start: usize) -> bool {
    let lookback_start = element_start.saturating_sub(900);
    let context = &slide_html[lookback_start..element_start];
    if (context.contains("padding:")
        || context.contains("box-shadow:")
        || context.contains("border:"))
        && (context.contains("background:rgba")
            || context.contains("background:#")
            || context.contains("background-color:")
            || context.contains("backdrop-filter:"))
    {
        return true;
    }
    let Some(last_div_start) = context.rfind("<div") else {
        return false;
    };
    let candidate = &context[last_div_start..];
    if candidate.contains("</div>") {
        return false;
    }
    candidate.contains("background:")
        || candidate.contains("background-color:")
        || candidate.contains("backdrop-filter:")
        || candidate.contains("box-shadow:")
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

/// Check if slide HTML contains overflow:hidden on the .slide element,
/// which provides the clip boundary for full-bleed compositions.
fn slide_has_slide_level_clip(slide_html: &str) -> bool {
    // The generated CSS always includes: .slide { ... overflow: hidden; }
    // In the actual slide HTML, the class attribute contains 'slide--full-bleed'
    // and the slide element clips via CSS. We check if the slide HTML has both
    // overflow:hidden patterns that would indicate slide-level clipping.
    let has_overflow_hidden_css =
        slide_html.contains("overflow: hidden") || slide_html.contains("overflow:hidden");
    // Also check for the .slide--full-bleed class which implies .slide has overflow:hidden
    let has_full_bleed = slide_html.contains("slide--full-bleed");
    // The base .slide CSS always has overflow:hidden; full-bleed compositions rely on this.
    // We consider the slide-level clip present if overflow:hidden appears in CSS or class attrs.
    has_full_bleed && has_overflow_hidden_css
}

pub fn validate_design(html: &str) -> ValidationReport {
    let mut issues = Vec::new();

    // Split the HTML into slides. Each slide starts with <div class="slide
    let slide_start_re = Regex::new(r#"<div\s+class="([^"]*)""#).unwrap();
    let slide_starts: Vec<_> = slide_start_re
        .captures_iter(html)
        .filter_map(|cap| {
            let class_attr = cap.get(1)?.as_str();
            if class_attr.split_whitespace().any(|class| class == "slide") {
                cap.get(0).map(|m| m.start())
            } else {
                None
            }
        })
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
    let styled_text_re =
        Regex::new(r#"(?s)<(?:p|h[1-6]|span|div)\s+[^>]*style="([^"]*)"[^>]*>([^<]{1,})</"#)
            .unwrap();
    let style_re = Regex::new(r#"style="([^"]*?)""#).unwrap();
    let img_re = Regex::new(r#"<img\s+[^>]*style="([^"]*)""#).unwrap();
    let any_style_re = Regex::new(r#"style="([^"]*)""#).unwrap();
    let image_card_re =
        Regex::new(r#"(?s)<div\s+style="([^"]*position:relative[^"]*)"[^>]*>.*?<img"#).unwrap();
    let bottom_caption_re = Regex::new(
        r#"(?s)<div\s+style="[^"]*position:relative[^"]*"[^>]*>.*?<img.*?<div\s+style="([^"]*position:absolute;[^"]*bottom:0[^"]*)""#,
    )
    .unwrap();
    let frame_re = Regex::new(r#"<div\s+style="([^"]*position:absolute;[^"]*left:[^"]*top:[^"]*width:[^"]*height:[^"]*)"[^>]*>\s*<div[^>]*>\s*<img"#).unwrap();
    let full_bleed_visible_re = Regex::new(
        r#"(?s)\.slide--full-bleed\s+\.slide-composition\s*\{[^}]*overflow\s*:\s*visible"#,
    )
    .unwrap();
    let tiny_overlay_re = Regex::new(
        r#"(?s)\.overlay__(?:brand|topic|url|hashtags)[^{]*\{[^}]*font-size\s*:\s*([0-9.]+)px"#,
    )
    .unwrap();
    let tiny_progress_re =
        Regex::new(r#"(?s)\.breadcrumb-chip(?:\.active)?[^{]*\{[^}]*height\s*:\s*([0-9.]+)px"#)
            .unwrap();

    // Note: overflow:visible on full-bleed compositions is intentional and correct.
    // The .slide element's overflow:hidden clips at the final slide boundary.
    // We only flag it if the slide element itself lacks overflow:hidden.
    if full_bleed_visible_re.is_match(html) {
        let slide_has_clip = html.contains(".slide")
            && (html.contains("overflow: hidden") || html.contains("overflow:hidden"));
        if !slide_has_clip {
            issues.push(DesignIssue {
                slide: 1,
                r#type: "aspect_bleed_overflow".to_string(),
                severity: "error".to_string(),
                detail: ".slide--full-bleed .slide-composition allows overflow:visible without slide-level clipping.".to_string(),
                message: "Full-bleed compositions need slide-level overflow:hidden to clip backgrounds at the final slide bounds.".to_string(),
                suggestion: "Ensure the .slide element has overflow:hidden so backgrounds bleed correctly while effects are clipped.".to_string(),
            });
        }
    }

    for cap in tiny_overlay_re.captures_iter(html) {
        let font_size = cap
            .get(1)
            .and_then(|m| m.as_str().parse::<f32>().ok())
            .unwrap_or(0.0);
        if font_size < 11.5 {
            issues.push(DesignIssue {
                slide: 1,
                r#type: "tiny_overlay_text".to_string(),
                severity: "warning".to_string(),
                detail: format!("Overlay text CSS uses {:.1}px font size.", font_size),
                message:
                    "Corner overlay text is too small for reliable exported-slide readability."
                        .to_string(),
                suggestion:
                    "Use at least 11.5px for overlay metadata in the 420x525 base composition."
                        .to_string(),
            });
        }
    }

    for cap in tiny_progress_re.captures_iter(html) {
        let height = cap
            .get(1)
            .and_then(|m| m.as_str().parse::<f32>().ok())
            .unwrap_or(0.0);
        if height < 1.5 {
            issues.push(DesignIssue {
                slide: 1,
                r#type: "tiny_progress_indicator".to_string(),
                severity: "warning".to_string(),
                detail: format!("Progress chip CSS uses {:.1}px height.", height),
                message: "Progress indicators are too thin to remain visible after export scaling."
                    .to_string(),
                suggestion: "Use at least 1.5px base height, with a larger active state."
                    .to_string(),
            });
        }
        if height > 4.0 {
            issues.push(DesignIssue {
                slide: 1,
                r#type: "progress_indicator_too_thick".to_string(),
                severity: "warning".to_string(),
                detail: format!("Progress chip CSS uses {:.1}px height, which is visually heavy at export scale.", height),
                message: "Progress indicators should be thin and refined for premium slide aesthetics."
                    .to_string(),
                suggestion: "Use 2px default height and 3px active height for optimal visual weight."
                    .to_string(),
            });
        }
    }

    // ─── New build-time checks: progress-overlay spacing, full-bleed
    //     stretch rule, image-trapped-in-content, and canvas-size
    //     awareness for overlay/breadcrumb anchoring. ───────────────

    // Parse canvas dimensions from :root CSS variables.
    let canvas_width_re = Regex::new(r#"(?s)--slide-width:\s*([0-9.]+)px"#).unwrap();
    let canvas_height_re = Regex::new(r#"(?s)--slide-height:\s*([0-9.]+)px"#).unwrap();
    let comp_width_re = Regex::new(r#"(?s)--composition-width:\s*([0-9.]+)px"#).unwrap();
    let comp_height_re = Regex::new(r#"(?s)--composition-height:\s*([0-9.]+)px"#).unwrap();
    let canvas_w = canvas_width_re
        .captures(html)
        .and_then(|c| c.get(1).and_then(|m| m.as_str().parse::<f32>().ok()))
        .unwrap_or(420.0);
    let canvas_h = canvas_height_re
        .captures(html)
        .and_then(|c| c.get(1).and_then(|m| m.as_str().parse::<f32>().ok()))
        .unwrap_or(525.0);
    let comp_w = comp_width_re
        .captures(html)
        .and_then(|c| c.get(1).and_then(|m| m.as_str().parse::<f32>().ok()))
        .unwrap_or(420.0);
    let comp_h = comp_height_re
        .captures(html)
        .and_then(|c| c.get(1).and_then(|m| m.as_str().parse::<f32>().ok()))
        .unwrap_or(525.0);
    let is_full_bleed_canvas = (canvas_w - comp_w).abs() > 1.0 || (canvas_h - comp_h).abs() > 1.0;

    // Check 1: progress-overlay spacing. Parse .breadcrumb-progress { bottom: Npx }
    // and .slide__overlay { padding: Apx Bpx } to ensure the breadcrumb sits
    // at least 12px below the overlay-bottom text.
    let progress_bottom_re = Regex::new(
        r#"(?s)\.breadcrumb-progress\s*\{[^}]*bottom\s*:\s*(?:var\(--space-\d+,\s*)?([0-9.]+)px"#,
    )
    .unwrap();
    let overlay_padding_re = Regex::new(
        r#"(?s)\.slide__overlay\s*\{[^}]*padding\s*:\s*(?:var\(--space-\d+,\s*)?([0-9.]+)px"#,
    )
    .unwrap();
    let progress_bottom = progress_bottom_re
        .captures(html)
        .and_then(|c| c.get(1).and_then(|m| m.as_str().parse::<f32>().ok()));
    let overlay_padding_bottom = overlay_padding_re
        .captures(html)
        .and_then(|c| c.get(1).and_then(|m| m.as_str().parse::<f32>().ok()));
    if let (Some(pb), Some(op)) = (progress_bottom, overlay_padding_bottom) {
        // Overlay-bottom text height ≈ 15px (11.5px font × 1.3 line-height).
        let overlay_text_top_from_bottom = op + 15.0;
        // Breadcrumb chip top from bottom = pb + chip_height (2-3px).
        let chip_top_from_bottom = pb + 3.0;
        let gap = chip_top_from_bottom - overlay_text_top_from_bottom;
        // If breadcrumb is ABOVE overlay text (positive gap) and gap < 12px,
        // OR if breadcrumb is BELOW overlay text (negative gap) and overlap > 0,
        // flag it.
        if gap.abs() < 12.0 && gap > -20.0 {
            issues.push(DesignIssue {
                slide: 1,
                r#type: "progress_overlay_collision".to_string(),
                severity: "warning".to_string(),
                detail: format!(
                    "breadcrumb-progress bottom {:.0}px is only {:.0}px from overlay-bottom text (padding {:.0}px + ~15px text).",
                    pb, gap.abs(), op
                ),
                message: "Progress indicator sits too close to the bottom overlay text, hurting visual separation.".to_string(),
                suggestion: "Move breadcrumb-progress to bottom:8px (below the overlay text) or increase to bottom:60px+ (above the overlay text with clear breathing room).".to_string(),
            });
        }
    }

    // Check 2: full-bleed stretch rule presence. If any slide has
    // slide--full-bleed class, the CSS must contain the first-of-type
    // stretch rule with !important on width/height.
    let has_full_bleed_slide = html.contains("slide--full-bleed");
    if has_full_bleed_slide {
        let stretch_rule_re = Regex::new(
            r#"(?s)\.slide--full-bleed\s+\.slide-composition\s*>\s*div:first-of-type\s*\{[^}]*width:\s*var\(--slide-width\)\s*!important[^}]*height:\s*var\(--slide-height\)\s*!important"#,
        )
        .unwrap();
        if !stretch_rule_re.is_match(html) {
            issues.push(DesignIssue {
                slide: 1,
                r#type: "missing_full_bleed_stretch_rule".to_string(),
                severity: "error".to_string(),
                detail: "Full-bleed slides are present but the CSS lacks the .slide--full-bleed .slide-composition > div:first-of-type stretch rule with !important.".to_string(),
                message: "Background layers on full-bleed slides will be clipped to the 420x525 composition instead of filling the canvas.".to_string(),
                suggestion: "Add: .slide--full-bleed .slide-composition > div:first-of-type { position:absolute!important; width:var(--slide-width)!important; height:var(--slide-height)!important; }".to_string(),
            });
        }
    }

    // Check 3: content images trapped in slide-content for full-bleed slides.
    // Image-primary slides (image_headline, image_quote) use padding:0 on
    // slide-content because the image fills the entire slide. If such a slide
    // uses plain .slide-content (not --bleed) on a full-bleed canvas, the img
    // will be clipped to 420x525. We detect this by looking for:
    //   class="slide-content" with padding:0  →  image-primary pattern
    //   AND a full-size img (width:100%;height:100%) inside a height:100% div chain
    //   AND no slide-content--bleed usage
    // Content slides (image_gallery, split_features, etc.) use non-zero padding
    // so they won't match.
    if is_full_bleed_canvas && has_full_bleed_slide {
        let uses_bleed_variant = html.contains("slide-content--bleed");
        if !uses_bleed_variant {
            // Check for image-primary pattern: slide-content with padding:0
            // containing a height:100% div chain with a full-size img.
            let image_primary_re = Regex::new(
                r#"(?s)class="slide-content"[^>]*padding:\s*0[^>]*>.*?<div\s+style="[^"]*height:\s*100%[^"]*"[^>]*>\s*<div\s+style="[^"]*height:\s*100%[^"]*"[^>]*>\s*<img\s+[^>]*style="[^"]*width:\s*100%[^"]*height:\s*100%"#,
            )
            .unwrap();
            if image_primary_re.is_match(html) {
                issues.push(DesignIssue {
                    slide: 1,
                    r#type: "full_bleed_image_trapped_in_content".to_string(),
                    severity: "error".to_string(),
                    detail: format!(
                        "Full-bleed slide (canvas {:.0}x{:.0}) has an image-primary layout (slide-content padding:0) with a full-size <img> inside .slide-content (constrained to {:.0}x{:.0}).",
                        canvas_w, canvas_h, comp_w, comp_h
                    ),
                    message: "Content image is trapped in the 420x525 composition and will not fill the canvas, leaving visible bands.".to_string(),
                    suggestion: "Use slide_base_bleed() (which emits .slide-content--bleed) for image-primary slides so the image fills the canvas.".to_string(),
                });
            }
        }
    }

    // Check 4: bg-image mask creating visible bands on full-bleed slides.
    // A mask like linear-gradient(to bottom, black 70%, transparent 100%)
    // fades 30% of the canvas to transparent, creating bands on full-bleed.
    if is_full_bleed_canvas {
        let mask_re =
            Regex::new(r#"(?s)mask-image:\s*linear-gradient\([^)]*black\s+(\d+)%,\s*transparent"#)
                .unwrap();
        for cap in mask_re.captures_iter(html) {
            let pct = cap
                .get(1)
                .and_then(|m| m.as_str().parse::<f32>().ok())
                .unwrap_or(100.0);
            if pct < 85.0 {
                issues.push(DesignIssue {
                    slide: 1,
                    r#type: "bg_image_mask_band".to_string(),
                    severity: "warning".to_string(),
                    detail: format!(
                        "Background image mask fades to transparent at {}%, creating visible bands on the {:.0}x{:.0} canvas.",
                        pct, canvas_w, canvas_h
                    ),
                    message: "Aggressive bg-image masks create empty bands on full-bleed canvases.".to_string(),
                    suggestion: "Use black 90%+ in the mask gradient, or remove the mask for full-bleed slides.".to_string(),
                });
            }
        }
    }

    for (slide_idx, slide_html) in slides.iter().enumerate() {
        let slide_num = slide_idx + 1;

        let has_background_image = slide_html.contains("background-image")
            || slide_html.contains("background-size:cover")
            || slide_html.contains("background-size: cover");

        for style_cap in any_style_re.captures_iter(slide_html) {
            let style = style_cap.get(1).map(|m| m.as_str()).unwrap_or("");
            if let Some(detail) = style_has_unitless_dimension(style) {
                issues.push(DesignIssue {
                    slide: slide_num,
                    r#type: "invalid_dimension".to_string(),
                    severity: "error".to_string(),
                    detail: format!("Style uses unitless positional dimension '{detail}'."),
                    message: "CSS width/height/position dimensions need explicit units."
                        .to_string(),
                    suggestion:
                        "Use px, %, rem, or another explicit CSS unit for positional dimensions."
                            .to_string(),
                });
            }
            let width = numeric_px_style_value(style, "width");
            let height = numeric_px_style_value(style, "height");
            if let (Some(w), Some(h)) = (width, height) {
                if component_like_style(style) && distorted_component_ratio(w, h) {
                    issues.push(DesignIssue {
                        slide: slide_num,
                        r#type: "component_aspect_distortion".to_string(),
                        severity: "warning".to_string(),
                        detail: format!(
                            "Component-like box has {:.1}:1 aspect ratio ({}x{}px).",
                            w / h.max(1.0),
                            w,
                            h
                        ),
                        message: "Component geometry is extremely horizontal or vertical and can distort the composition across aspect-ratio exports.".to_string(),
                        suggestion: "Keep cards/components within a moderate aspect ratio, or switch to a stacked layout for narrow/tall cases.".to_string(),
                    });
                }
                if let (Some(x), Some(y)) = (
                    numeric_px_style_value(style, "left")
                        .or_else(|| numeric_style_value(style, "left")),
                    numeric_px_style_value(style, "top")
                        .or_else(|| numeric_style_value(style, "top")),
                ) {
                    if component_like_style(style)
                        && !style_has_edge_bleed_effect(style)
                        && rect_overflows_slide_body(x, y, w, h)
                    {
                        issues.push(DesignIssue {
                            slide: slide_num,
                            r#type: "slide_body_overflow".to_string(),
                            severity: "error".to_string(),
                            detail: format!(
                                "Component bounds left {:.0}, top {:.0}, width {:.0}, height {:.0} exceed the 420x525 slide body.",
                                x, y, w, h
                            ),
                            message: "Component layout overflows the SlideForge base slide body.".to_string(),
                            suggestion: "Keep body components within the 420x525 composition bounds; reserve only backgrounds for aspect-ratio bleed.".to_string(),
                        });
                    }
                }
            }
            if component_constriction_risk(style) {
                issues.push(DesignIssue {
                    slide: slide_num,
                    r#type: "component_constriction".to_string(),
                    severity: "warning".to_string(),
                    detail: format!("Component-like box is constrained by style '{style}'."),
                    message: "A card/component has too little inner width after padding and can collapse its content.".to_string(),
                    suggestion: "Increase available width, reduce padding for the compact variant, or stack the component in a wider single-column layout.".to_string(),
                });
            }
            // Only flag edge_effect_bleed when there is NO slide-level overflow:hidden
            // to clip the effect. The .slide element's overflow:hidden provides the
            // clip boundary for full-bleed compositions with overflow:visible.
            if slide_html.contains("slide--full-bleed")
                && style_has_edge_bleed_effect(style)
                && !has_overflow_hidden(style)
                && !slide_has_slide_level_clip(slide_html)
            {
                issues.push(DesignIssue {
                    slide: slide_num,
                    r#type: "edge_effect_bleed".to_string(),
                    severity: "error".to_string(),
                    detail: format!("Edge effect can bleed from full-bleed slide: '{style}'."),
                    message: "Blurred shadows/glows near negative edges can leak during aspect-ratio transmutation.".to_string(),
                    suggestion: "Clip the full-bleed wrapper at the final slide bounds or move the effect inside a clipped background layer.".to_string(),
                });
            }
        }

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

        for cap in image_card_re.captures_iter(slide_html) {
            let style = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            if let Some(height) = numeric_style_value(style, "height") {
                if height > 0.0 && height < 96.0 {
                    issues.push(DesignIssue {
                        slide: slide_num,
                        r#type: "image_constriction".to_string(),
                        severity: "warning".to_string(),
                        detail: format!("Image frame height is only {:.0}px.", height),
                        message: "Image frame is too short to carry a clear visual.".to_string(),
                        suggestion: "Increase the frame height or switch to a layout with fewer image slots.".to_string(),
                    });
                }
            }
            if let (Some(width), Some(height)) = (
                numeric_px_style_value(style, "width"),
                numeric_px_style_value(style, "height"),
            ) {
                if distorted_image_ratio(width, height) {
                    issues.push(DesignIssue {
                        slide: slide_num,
                        r#type: "image_aspect_distortion".to_string(),
                        severity: "warning".to_string(),
                        detail: format!(
                            "Image frame has {:.1}:1 aspect ratio ({}x{}px).",
                            width / height.max(1.0),
                            width,
                            height
                        ),
                        message: "Image frame aspect ratio is distorted enough to damage visual composition.".to_string(),
                        suggestion: "Use a less extreme image frame ratio or crop inside a stable frame with object-fit:cover.".to_string(),
                    });
                }
            }
        }

        for cap in bottom_caption_re.captures_iter(slide_html) {
            let style = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            issues.push(DesignIssue {
                slide: slide_num,
                r#type: "image_caption_overlay".to_string(),
                severity: "warning".to_string(),
                detail: format!("Image caption uses bottom absolute positioning: '{style}'."),
                message: "Bottom image captions can visually collide with adjacent captions or obscure the image.".to_string(),
                suggestion: "Move image labels to a top chip, outside the frame, or reserve explicit caption space below the frame.".to_string(),
            });
        }

        for cap in styled_text_re.captures_iter(slide_html) {
            let style = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let plain_text = cap.get(2).map(|m| m.as_str()).unwrap_or("").trim();
            let word_count = plain_text.split_whitespace().count();
            let width = numeric_px_style_value(style, "width")
                .or_else(|| numeric_px_style_value(style, "max-width"));
            let font_size = numeric_style_value(style, "font-size").unwrap_or(12.0);
            if tiny_text_risk(font_size) && !is_emoji_or_icon_text(plain_text) {
                issues.push(DesignIssue {
                    slide: slide_num,
                    r#type: "tiny_text".to_string(),
                    severity: "warning".to_string(),
                    detail: format!("Text '{}' uses {:.1}px font size.", plain_text, font_size),
                    message: "Inline text is too small for reliable exported-slide readability."
                        .to_string(),
                    suggestion:
                        "Use at least 10.5px for micro-labels, and 11.5px or larger for metadata."
                            .to_string(),
                });
            }
            if text_has_descender_risk(plain_text) && has_overflow_hidden(style) {
                let line_height = numeric_style_value(style, "line-height").unwrap_or(1.2);
                let line_height_px = if line_height <= 4.0 {
                    line_height * font_size
                } else {
                    line_height
                };
                if line_height_px < font_size * 1.08 {
                    issues.push(DesignIssue {
                        slide: slide_num,
                        r#type: "text_vertical_clipping".to_string(),
                        severity: "error".to_string(),
                        detail: format!(
                            "Text '{}' has descenders with overflow hidden and tight line-height.",
                            plain_text
                        ),
                        message: "Text descenders may be clipped at the bottom of their container."
                            .to_string(),
                        suggestion: "Increase line-height to at least 1.1, add vertical padding, or remove overflow hidden on the text element.".to_string(),
                    });
                }
            }
            if let Some(width) = width {
                if one_word_line_risk(width, font_size, word_count) {
                    issues.push(DesignIssue {
                        slide: slide_num,
                        r#type: "one_word_line_risk".to_string(),
                        severity: "warning".to_string(),
                        detail: format!(
                            "Text '{}' has only {:.1} font-size units of line width.",
                            plain_text,
                            width / font_size
                        ),
                        message: "Text width is likely to create one-word-per-line wrapping.".to_string(),
                        suggestion: "Give the text a wider column, reduce type size, or switch to a stacked layout.".to_string(),
                    });
                }
                if width > 0.0 && width < 120.0 && font_size >= 12.0 && word_count >= 3 {
                    issues.push(DesignIssue {
                        slide: slide_num,
                        r#type: "text_constriction".to_string(),
                        severity: "warning".to_string(),
                        detail: format!(
                            "Text '{}' is constrained to {:.0}px at {:.0}px font size.",
                            plain_text, width, font_size
                        ),
                        message: "Text container is narrow enough to force poor one-word-per-line wrapping.".to_string(),
                        suggestion: "Use a wider text area, smaller type, or stack the content vertically.".to_string(),
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
            let element_start = cap.get(0).map(|m| m.start()).unwrap_or(0);
            let tag_open = &cap[1];
            let attrs = &cap[2];
            let text_content = cap[3].trim();
            let tag_close = &cap[4];

            if tag_open != tag_close {
                continue;
            }
            if tag_open == "div" && text_content.contains('<') {
                continue;
            }

            // Skip if the text content is empty or contains only tags
            let plain_text = Regex::new(r"<[^>]*>")
                .unwrap()
                .replace_all(text_content, "")
                .trim()
                .to_string();
            if plain_text.is_empty() {
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
            if text_has_descender_risk(&plain_text) && has_overflow_hidden(style_str) {
                let line_height = numeric_style_value(style_str, "line-height").unwrap_or(1.2);
                let font_size = numeric_style_value(style_str, "font-size").unwrap_or(16.0);
                let line_height_px = if line_height <= 4.0 {
                    line_height * font_size
                } else {
                    line_height
                };
                if line_height_px < font_size * 1.08 {
                    issues.push(DesignIssue {
                        slide: slide_num,
                        r#type: "text_vertical_clipping".to_string(),
                        severity: "error".to_string(),
                        detail: format!(
                            "Text '{}' has descenders with overflow hidden and tight line-height.",
                            plain_text
                        ),
                        message: "Text descenders may be clipped at the bottom of their container."
                            .to_string(),
                        suggestion: "Increase line-height to at least 1.1, add vertical padding, or remove overflow hidden on the text element.".to_string(),
                    });
                }
            }
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

            if has_background_image
                && !has_bg
                && !has_shadow
                && !has_recent_backing_container(slide_html, element_start)
            {
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

            let word_count = plain_text.split_whitespace().count();
            let width = numeric_px_style_value(style_str, "width")
                .or_else(|| numeric_px_style_value(style_str, "max-width"));
            let font_size = numeric_style_value(style_str, "font-size").unwrap_or(12.0);
            if tiny_text_risk(font_size) && !is_emoji_or_icon_text(&plain_text) {
                issues.push(DesignIssue {
                    slide: slide_num,
                    r#type: "tiny_text".to_string(),
                    severity: "warning".to_string(),
                    detail: format!("Text '{}' uses {:.1}px font size.", plain_text, font_size),
                    message: "Inline text is too small for reliable exported-slide readability."
                        .to_string(),
                    suggestion:
                        "Use at least 10.5px for micro-labels, and 11.5px or larger for metadata."
                            .to_string(),
                });
            }
            if let Some(width) = width {
                if one_word_line_risk(width, font_size, word_count) {
                    issues.push(DesignIssue {
                        slide: slide_num,
                        r#type: "one_word_line_risk".to_string(),
                        severity: "warning".to_string(),
                        detail: format!(
                            "Text '{}' has only {:.1} font-size units of line width.",
                            plain_text,
                            width / font_size
                        ),
                        message: "Text width is likely to create one-word-per-line wrapping.".to_string(),
                        suggestion: "Give the text a wider column, reduce type size, or switch to a stacked layout.".to_string(),
                    });
                }
                if width > 0.0 && width < 120.0 && font_size >= 12.0 && word_count >= 3 {
                    issues.push(DesignIssue {
                        slide: slide_num,
                        r#type: "text_constriction".to_string(),
                        severity: "warning".to_string(),
                        detail: format!(
                            "Text '{}' is constrained to {:.0}px at {:.0}px font size.",
                            plain_text, width, font_size
                        ),
                        message: "Text container is narrow enough to force poor one-word-per-line wrapping.".to_string(),
                        suggestion: "Use a wider text area, smaller type, or stack the content vertically.".to_string(),
                    });
                }
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
