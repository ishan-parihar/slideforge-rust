use std::collections::HashMap;

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
            // Retired 2026-07-30: surface an actionable migration message so
            // agents see the type as removed rather than "unknown".
            const REMOVED: &[&str] = &[
                "feature", "list", "callout", "grid_cards",
                "text_columns", "image_stat", "cta", "checklist_action_plan",
            ];
            if REMOVED.contains(&slide_type) {
                let replacement = match slide_type {
                    "feature" => "split_features (single-feature beat) or case_study_result",
                    "list" => "timeline (ordered steps) or quote_slide",
                    "callout" => "myth_fact (educational contrast) or image_callout",
                    "grid_cards" => "split_features (2 cols) or case_study_result (results grid)",
                    "text_columns" => "split_features (2 cols) or quote_slide",
                    "image_stat" => "image_callout, image_caption, or metric_grid",
                    "cta" => "qr_destination (the canonical closing CTA slide)",
                    "checklist_action_plan" => "timeline (ordered steps), process_map (workflows), or split_features (action items)",
                    _ => "an appropriate alternative",
                };
                result.add_error(format!(
                    "{slide_type} slide type has been removed. Use {replacement} instead."
                ));
                return result;
            }
            let valid_types = crate::slide_registry::list_slide_types();
            result.add_error(format!(
                "Unknown slide type: '{}'. Valid types: {}",
                slide_type,
                valid_types.join(", ")
            ));
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
                                result.add_error(format!(
                                    "Required param '{primary_key}' is an empty array for slide type '{slide_type}' — fill it or call validate_and_fix"
                                ));
                            }
                            _ => {
                                result.add_error(format!(
                                    "Required param '{primary_key}' is present but empty for slide type '{slide_type}' — fill it or call validate_and_fix"
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
                                result.add_error(format!(
                                    "Required param '{alt}' is an empty array for slide type '{slide_type}' — fill it or call validate_and_fix"
                                ));
                            }
                            _ => {
                                result.add_error(format!(
                                    "Required param '{alt}' is present but empty for slide type '{slide_type}' — fill it or call validate_and_fix"
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
                    result.add_error(format!(
                        "Required param '{param}' is present but empty for slide type '{slide_type}' — fill it or call validate_and_fix"
                    ));
                }
                Some(Value::Array(arr)) if arr.is_empty() => {
                    result.add_error(format!(
                        "Required param '{param}' is an empty array for slide type '{slide_type}' — fill it or call validate_and_fix"
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

    if slide_type == "cta" {
        // cta slide type removed. Surface an actionable error so agents get
        // explicit feedback instead of silent-empty renders.
        result.add_error("cta slide type has been removed. Use qr_destination for the closing slide (deck-level marketing constraint: exactly one CTA, always final).");
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
///
/// Removed slide types (feature, list, callout, grid_cards, text_columns,
/// image_stat, cta) surface an actionable error pointing the AI agent at the
/// replacement type. No auto-fix is attempted for them.
///
/// Returns a `ValidationResult` describing errors, warnings, and fixes.
pub fn validate_and_fix_slide(slide_type: &str, params: &mut Value) -> ValidationResult {
    // Removed slide types: surface an actionable error so the AI agent gets
    // explicit feedback instead of silent-empty renders. No auto-fix attempted.
    if matches!(slide_type, "feature" | "list" | "callout" | "grid_cards" | "text_columns" | "image_stat" | "cta" | "checklist_action_plan") {
        let replacement = match slide_type {
            "feature" => "split_features (single-feature beat) or case_study_result",
            "list" => "timeline (ordered steps) or quote_slide",
            "callout" => "myth_fact (educational contrast) or image_callout",
            "grid_cards" => "split_features (2 cols) or case_study_result (results grid)",
            "text_columns" => "split_features (2 cols) or quote_slide",
            "image_stat" => "image_callout, image_caption, or metric_grid",
            "checklist_action_plan" => "timeline (ordered steps), process_map (workflows), or split_features (action items)",
            "cta" => "qr_destination (the canonical closing CTA slide)",
            _ => "an appropriate alternative",
        };
        let mut result = ValidationResult::default();
        result.add_error(format!(
            "{slide_type} slide type has been removed. Use {replacement} instead."
        ));
        return result;
    }

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
        // before_after_story, problem_solution, definition:
        // NO auto-fix — missing/empty required params must produce hard errors
        // so the AI agent gets actionable feedback on which fields to fill.
        // Auto-filling with wrong-shaped placeholders silently produces empty tiles.
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

// ── Composition validation ──────────────────────────────────────────────────

/// Data-visualization slide types — used for dataviz pacing constraints.
///
/// `column_chart` retired (merged into chart_type="bar_vertical"); `stat_row`
/// retired (folded into metric_grid). They remain accepted by the dispatcher for
/// legacy JSON, but no longer count as dataviz pacing units.
const DATAVIZ_TYPES: &[&str] = &[
    "chart",
    "scatter_plot",
    "gauge",
    "radar_chart",
    "progress_rings",
    "comparison_bars",
    "metric_grid",
    "funnel_chart",
    "table",
];

/// A single slide entry in a composition.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CompositionSlide {
    pub slide_type: String,
    pub arc: String,
    pub bg_style: Option<String>,
}

/// Count bounds for an arc position.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArcCount {
    pub min: usize,
    pub max: usize,
}

/// An arc position definition (hook, evidence, proof, action).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArcPosition {
    /// Locked slide-types (for hook/action). Mutually exclusive with `pool`.
    #[serde(default)]
    pub types: Vec<String>,
    /// Flexible slide-types pool (for evidence/proof). Mutually exclusive with `types`.
    #[serde(default)]
    pub pool: Vec<String>,
    pub count: ArcCount,
}

/// Composition constraints.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CompositionConstraints {
    #[serde(default = "default_true")]
    pub no_consecutive_same_type: bool,
    #[serde(default = "default_bg_rhythm")]
    pub bg_rhythm: String,
    pub max_slides: usize,
    pub min_slides: usize,
    #[serde(default = "default_max_consecutive_dataviz")]
    pub max_consecutive_dataviz: usize,
    #[serde(default = "default_true")]
    pub require_narrative_after_dataviz: bool,
}

fn default_true() -> bool { true }
fn default_bg_rhythm() -> String { "alternating_dark_light".to_string() }
fn default_max_consecutive_dataviz() -> usize { 2 }

/// Request for composition validation.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CompositionRequest {
    /// The slide composition to validate.
    pub composition: Vec<CompositionSlide>,
    /// Arc structure defining pools and count bounds.
    pub arc_structure: std::collections::HashMap<String, ArcPosition>,
    /// Composition constraints.
    pub constraints: CompositionConstraints,
}

/// Result of composition validation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompositionValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl CompositionValidationResult {
    fn ok() -> Self {
        Self { valid: true, errors: vec![], warnings: vec![] }
    }
    fn err(msg: impl Into<String>) -> Self {
        Self { valid: false, errors: vec![msg.into()], warnings: vec![] }
    }
    fn add_error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
        self.valid = false;
    }
    fn add_warning(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }
}

/// Validate a carousel composition against arc structure and constraints.
///
/// Checks:
/// 1. Arc position counts (min/max)
/// 2. Pool membership (every type is allowed in its arc)
/// 3. No consecutive same slide_type
/// 4. DLD rhythm (no consecutive same bg_style)
/// 5. Total slide count within bounds
/// 6. Dataviz pacing (max N consecutive dataviz, narrative follows)
pub fn validate_composition(request: &CompositionRequest) -> CompositionValidationResult {
    let mut result = CompositionValidationResult::ok();
    let c = &request.constraints;
    let comp = &request.composition;

    // ── Total slide count ───────────────────────────────────────────────────
    if comp.len() < c.min_slides {
        result.add_error(format!(
            "Composition has {} slides, minimum is {}",
            comp.len(),
            c.min_slides
        ));
    }
    if comp.len() > c.max_slides {
        result.add_error(format!(
            "Composition has {} slides, maximum is {}",
            comp.len(),
            c.max_slides
        ));
    }

    // ── Group by arc ────────────────────────────────────────────────────────
    let mut arc_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for slide in comp {
        *arc_counts.entry(slide.arc.clone()).or_insert(0) += 1;
    }

    // ── Arc position counts ─────────────────────────────────────────────────
    for (arc_name, arc_def) in &request.arc_structure {
        let count = arc_counts.get(arc_name).copied().unwrap_or(0);
        if count < arc_def.count.min {
            result.add_error(format!(
                "{} arc has {} slides, minimum is {}",
                arc_name,
                count,
                arc_def.count.min
            ));
        }
        if count > arc_def.count.max {
            result.add_error(format!(
                "{} arc has {} slides, maximum is {}",
                arc_name,
                count,
                arc_def.count.max
            ));
        }
    }

    // ── Pool membership ─────────────────────────────────────────────────────
    for (i, slide) in comp.iter().enumerate() {
        if let Some(arc_def) = request.arc_structure.get(&slide.arc) {
            let allowed = if !arc_def.types.is_empty() {
                arc_def.types.contains(&slide.slide_type)
            } else {
                arc_def.pool.contains(&slide.slide_type)
            };
            if !allowed {
                let mut available: Vec<&str> = arc_def
                    .types.iter().chain(arc_def.pool.iter())
                    .map(|s| s.as_str())
                    .collect();
                available.sort();
                result.add_error(format!(
                    "Slide {} ({}) is '{}' but {} arc only allows: {:?}",
                    i + 1,
                    slide.arc,
                    slide.slide_type,
                    slide.arc,
                    available
                ));
            }
        } else {
            result.add_error(format!(
                "Slide {} references unknown arc '{}'",
                i + 1,
                slide.arc
            ));
        }
    }

    // ── No consecutive same type ────────────────────────────────────────────
    if c.no_consecutive_same_type && comp.len() >= 2 {
        for window in comp.windows(2) {
            if window[0].slide_type == window[1].slide_type {
                result.add_error(format!(
                    "'{}' appears consecutively at positions {} and {}",
                    window[0].slide_type,
                    comp.iter().position(|s| std::ptr::eq(s, &window[0])).unwrap_or(0) + 1,
                    comp.iter().position(|s| std::ptr::eq(s, &window[1])).unwrap_or(0) + 1
                ));
            }
        }
    }

    // ── DLD rhythm (bg_style) ──────────────────────────────────────────────
    if c.bg_rhythm == "alternating_dark_light" && comp.len() >= 2 {
        for window in comp.windows(2) {
            let bg0 = window[0].bg_style.as_deref().unwrap_or("light");
            let bg1 = window[1].bg_style.as_deref().unwrap_or("light");
            if bg0 == bg1 {
                result.add_warning(format!(
                    "Background rhythm break: slides {} and {} both use '{}' bg_style",
                    comp.iter().position(|s| std::ptr::eq(s, &window[0])).unwrap_or(0) + 1,
                    comp.iter().position(|s| std::ptr::eq(s, &window[1])).unwrap_or(0) + 1,
                    bg0
                ));
            }
        }
    }

    // ── Dataviz pacing ──────────────────────────────────────────────────────
    if comp.len() >= 2 {
        let mut consecutive_dataviz = 0usize;
        let mut dataviz_start = 0usize;
        for (i, slide) in comp.iter().enumerate() {
            if DATAVIZ_TYPES.contains(&slide.slide_type.as_str()) {
                if consecutive_dataviz == 0 {
                    dataviz_start = i;
                }
                consecutive_dataviz += 1;
            } else {
                consecutive_dataviz = 0;
            }
            // Check at every position if we've exceeded the limit
            if consecutive_dataviz > c.max_consecutive_dataviz {
                // Check if the next slide is narrative (if it exists)
                if i + 1 < comp.len() {
                    let next = &comp[i + 1];
                    if DATAVIZ_TYPES.contains(&next.slide_type.as_str()) {
                        result.add_error(format!(
                            "{} consecutive dataviz slides at positions {}–{}, requires a narrative slide after position {}",
                            consecutive_dataviz,
                            dataviz_start + 1,
                            i + 1,
                            i + 1
                        ));
                    }
                } else {
                    // Last slide is dataviz and we've exceeded the limit
                    result.add_error(format!(
                        "{} consecutive dataviz slides at positions {}–{}, carousel ends with dataviz (requires narrative or closing slide)",
                        consecutive_dataviz,
                        dataviz_start + 1,
                        i + 1
                    ));
                }
            }
        }
    }

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
    fn test_hero_empty_headline_errors() {
        let params = json!({ "headline": "   " });
        let r = validate_slide_spec("hero", &params);
        // Empty string headline must be an error, not a silent warning.
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("headline")));
    }

    #[test]
    fn test_list_removed_surface_error() {
        // list slide type was removed 2026-07-30. The validator must surface an actionable
        // error pointing at the replacement type, not silently accept whatever empty-items
        // content is provided.
        let params = json!({ "title": "My List", "items": [] });
        let r = validate_slide_spec("list", &params);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("removed")));
    }

    #[test]
    fn test_cta_removed_surface_error() {
        // cta slide type was removed 2026-07-30 (deck-level marketing constraint:
        // exactly one closing CTA, now provided by qr_destination). The validator
        // must surface an actionable error instead of silently dropping the slide.
        let params = json!({ "headline": "Get started today" });
        let r = validate_slide_spec("cta", &params);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("removed")));
    }

    #[test]
    fn test_removed_slide_types_surface_actionable_error() {
        for removed in ["feature", "list", "callout", "grid_cards", "text_columns", "image_stat"] {
            let mut params = json!({"title": "test"});
            let r = validate_and_fix_slide(removed, &mut params);
            assert!(!r.valid, "{removed} should be invalid");
            assert!(
                r.errors.iter().any(|e| e.contains("removed")),
                "{removed} errors missing 'removed': {:?}",
                r.errors
            );
        }
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
    fn test_fix_list_does_not_placeholder() {
        // list slide type was removed 2026-07-30 — the auto-fix that injected
        // a placeholder item is gone. The validator must surface an actionable
        // error and refrain from silently patching empty items arrays.
        let mut params = json!({ "title": "Steps", "items": [] });
        let r = validate_and_fix_slide("list", &mut params);
        assert!(!r.valid, "list must be invalid after 2026-07-30 purge");
        assert!(!r.fixes.iter().any(|f| f.contains("placeholder")));
    }

    #[test]
    fn test_fix_cta_default_button_text() {
        // cta auto-fix was removed 2026-07-30. The validator now surfaces
        // an error and refrains from silently injecting "Learn More".
        let mut params = json!({ "headline": "Join us" });
        let r = validate_and_fix_slide("cta", &mut params);
        assert!(!r.valid, "cta must be invalid");
        assert!(!r.fixes.iter().any(|f| f.contains("button_text")));
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
    fn test_validate_design_flags_hardcoded_low_alpha_text_bypass() {
        let html = r#"
            <div class="slide bg-dark">
                <span style="color:rgba(255,255,255,0.6);">Low alpha white</span>
            </div>
        "#;
        let report = validate_design(html);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.r#type == "hardcoded_rgba_text_bypass"));
    }

    #[test]
    fn test_validate_design_flags_hardcoded_low_alpha_text_bypass_dark_text() {
        let html = r#"
            <div class="slide bg-light">
                <span style="color:rgba(0,0,0,0.5);">Low alpha black</span>
            </div>
        "#;
        let report = validate_design(html);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.r#type == "hardcoded_rgba_text_bypass"));
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
    fn test_validate_design_flags_grid_cards_overflow_risk() {
        // 6 cards × 500 chars each = 3000 chars total — exceeds the 2500-char grid
        // container budget enforced in validate_design. The validator must flag this
        // even though the renderer's very_dense scaling tier can fit ~600 chars per
        // card in some variants; the universal ceiling exists because compact/masonry
        // variants cannot absorb that mass.
        let html = format!(
            r#"<div class="slide bg-light">
                <div style="display:grid;grid-template-columns:repeat(3, 1fr);gap:14px;width:100%;margin-top:16px;">
                    <div style="padding:24px;"><h3>Card 1</h3><p>{}</p></div>
                    <div style="padding:24px;"><h3>Card 2</h3><p>{}</p></div>
                    <div style="padding:24px;"><h3>Card 3</h3><p>{}</p></div>
                    <div style="padding:24px;"><h3>Card 4</h3><p>{}</p></div>
                    <div style="padding:24px;"><h3>Card 5</h3><p>{}</p></div>
                    <div style="padding:24px;"><h3>Card 6</h3><p>{}</p></div>
                </div>
            </div>"#,
            "a".repeat(500), "b".repeat(500), "c".repeat(500),
            "d".repeat(500), "e".repeat(500), "f".repeat(500)
        );
        let report = validate_design(&html);
        assert!(
            report.issues.iter().any(|i| i.r#type == "grid_cards_overflow_risk"),
            "should flag grid cards overflow risk when total text mass in grid exceeds threshold"
        );
    }

    #[test]
    fn test_validate_design_does_not_flag_grid_when_text_outside_grid_exceeds_threshold() {
        // Regression: the grid text-mass budget must scope to the grid container, NOT the
        // whole slide. Titles/eyebrows/captions outside the grid are not part of the
        // overflow surface and must not trip the threshold. If they do, every preset with
        // a hero title + a 2x2 grid produces a false positive.
        let html = r#"<div class="slide bg-light">
            <h1 style="font-size:31px;">A really long product headline that adds tons of characters and would push the slide-wide total past the grid threshold.</h1>
            <p style="font-size:16px;">And a long subtitle / eyebrow that adds even more characters to the slide-wide total.</p>
            <div style="display:grid;grid-template-columns:1fr 1fr;gap:14px;">
                <div style="padding:24px;"><h3>Card 1</h3><p>short</p></div>
                <div style="padding:24px;"><h3>Card 2</h3><p>short</p></div>
                <div style="padding:24px;"><h3>Card 3</h3><p>short</p></div>
                <div style="padding:24px;"><h3>Card 4</h3><p>short</p></div>
            </div>
        </div>"#;
        let report = validate_design(html);
        assert!(
            !report.issues.iter().any(|i| i.r#type == "grid_cards_overflow_risk"),
            "grid overflow check must scope text to the grid container, not the whole slide"
        );
    }


    #[test]
    fn test_validate_design_splits_id_first_slide_divs() {
        // Regression: the renderer emits `<div id="slide-0" class="slide slide--light">`
        // (id BEFORE class). The slide-split regex must be attribute-order agnostic
        // or validate_design silently validates zero slides.
        let html = r#"
            <div id="slide-0" class="slide slide--light"><div class="slide-composition"><p style="font-size:16px;">Fine text</p></div></div>
            <style>#slide-0 { --primary: #C62828; }</style>
            <div id="slide-1" class="slide slide--dark"><div class="slide-composition"><span style="font-size:9px;">tiny</span></div></div>
            <div id="slide-2" class="slide slide--mesh"><div class="slide-composition"><p style="font-size:16px;">OK</p></div></div>
        "#;
        let report = validate_design(html);
        assert_eq!(report.slide_count, 3, "expected all 3 slides to be detected");
        assert!(
            report
                .issues
                .iter()
                .any(|i| i.r#type == "tiny_text" && i.slide == 2),
            "per-slide issue must be attributed to slide 2"
        );
    }

    #[test]
    fn test_validate_design_bare_fragment_validates_as_one_slide() {
        // Compile-time validation passes a bare slide fragment (no `.slide` div).
        // The fallback must validate it as one slide instead of silently passing.
        let html = r#"
            <div style="position:relative;"><span style="font-size:9px;font-weight:700;">Q1</span></div>
        "#;
        let report = validate_design(html);
        assert_eq!(report.slide_count, 1);
        assert!(
            report
                .issues
                .iter()
                .any(|i| i.r#type == "tiny_text" && i.slide == 1)
        );
    }

    #[test]
    fn test_validate_design_warns_when_carousel_split_fails() {
        // A carousel whose slide divs carry a `class` attribute WITHOUT the exact
        // `slide` token (e.g. class="slide--light" only) — the split regex requires
        // an exact `slide` class, so it matches nothing and must warn. Keep the
        // `slide-composition` nodes intact so the carousel heuristic fires.
        let mut html = String::from(
            "<div id=\"slide-0\" class=\"slide--light\"><div class=\"slide-composition\">X</div></div>\n",
        );
        while html.len() < 21_000 {
            html.push_str(
                "<div id=\"slide-1\" class=\"slide--dark\"><div class=\"slide-composition\">Y</div></div>\n",
            );
        }
        let report = validate_design(&html);
        assert!(
            report.issues.iter().any(|i| i.r#type == "slide_split_failed"),
            "expected slide_split_failed warning, got {:?}",
            report.issues.iter().map(|i| &i.r#type).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_validate_design_flags_113px_quote_overflow() {
        // The harness quote (40 chars at the display tier) renders at 113px in a
        // narrow column and overflows by ~400px. The shared overflow model must
        // flag it as a text_overflow error even though the composition clips it.
        let html = r#"
            <div class="slide slide--light"><div class="slide-composition">
                <div class="slide-content" style="padding:16px 44px 20px;">
                    <blockquote style="font-family:Playfair Display;font-size:113px;font-weight:600;line-height:1.25;max-width:272px;">Design is the silent language of trust.</blockquote>
                </div>
            </div></div>
        "#;
        let report = validate_design(html);
        assert!(
            report
                .issues
                .iter()
                .any(|i| i.r#type == "text_overflow" && i.severity == "error"),
            "expected text_overflow error, issues: {:?}",
            report
                .issues
                .iter()
                .map(|i| (&i.r#type, &i.severity))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_validate_design_passes_short_text() {
        // A short, normally-sized text stack must NOT be flagged.
        let html = r#"
            <div class="slide slide--light"><div class="slide-composition">
                <div class="slide-content" style="padding:16px 44px 20px;">
                    <h2 style="font-size:31px;line-height:1.2;">Short title</h2>
                    <p style="font-size:15px;line-height:1.5;max-width:320px;">A compact supporting line.</p>
                </div>
            </div></div>
        "#;
        let report = validate_design(html);
        assert!(
            !report.issues.iter().any(|i| i.r#type == "text_overflow"),
            "short text must not overflow, issues: {:?}",
            report.issues.iter().map(|i| &i.r#type).collect::<Vec<_>>()
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
    fn test_qr_destination_empty_alternatives_errors() {
        let params = json!({
            "url": "",
            "button_text": "   ",
            "headline": "Read the full guide"
        });
        let r = validate_slide_spec("qr_destination", &params);
        // Empty required alternatives must be errors, not silent warnings.
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("url")));
        assert!(r.errors.iter().any(|e| e.contains("button_text")));
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

    // ── validate_composition ────────────────────────────────────────────────

    fn make_arc_def(types: Vec<&str>, pool: Vec<&str>, min: usize, max: usize) -> ArcPosition {
        ArcPosition {
            types: types.into_iter().map(String::from).collect(),
            pool: pool.into_iter().map(String::from).collect(),
            count: ArcCount { min, max },
        }
    }

    fn make_constraints(min: usize, max: usize) -> CompositionConstraints {
        CompositionConstraints {
            no_consecutive_same_type: true,
            bg_rhythm: "alternating_dark_light".to_string(),
            max_slides: max,
            min_slides: min,
            max_consecutive_dataviz: 2,
            require_narrative_after_dataviz: true,
        }
    }

    #[test]
    fn test_composition_valid_simple() {
        let mut arc_structure = std::collections::HashMap::new();
        arc_structure.insert("hook".into(), make_arc_def(vec!["hero"], vec![], 1, 1));
        arc_structure.insert("evidence".into(), make_arc_def(vec![], vec!["chart", "list"], 2, 4));
        arc_structure.insert("action".into(), make_arc_def(vec!["cta"], vec![], 1, 1));

        let request = CompositionRequest {
            composition: vec![
                CompositionSlide { slide_type: "hero".into(), arc: "hook".into(), bg_style: Some("dark".into()) },
                CompositionSlide { slide_type: "chart".into(), arc: "evidence".into(), bg_style: Some("light".into()) },
                CompositionSlide { slide_type: "list".into(), arc: "evidence".into(), bg_style: Some("dark".into()) },
                CompositionSlide { slide_type: "cta".into(), arc: "action".into(), bg_style: Some("light".into()) },
            ],
            arc_structure,
            constraints: make_constraints(3, 6),
        };
        let r = validate_composition(&request);
        assert!(r.valid, "errors: {:?}", r.errors);
    }

    #[test]
    fn test_composition_pool_violation() {
        let mut arc_structure = std::collections::HashMap::new();
        arc_structure.insert("evidence".into(), make_arc_def(vec![], vec!["chart", "list"], 1, 3));

        let request = CompositionRequest {
            composition: vec![
                CompositionSlide { slide_type: "cta".into(), arc: "evidence".into(), bg_style: None },
            ],
            arc_structure,
            constraints: make_constraints(1, 5),
        };
        let r = validate_composition(&request);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("cta") && e.contains("only allows")));
    }

    #[test]
    fn test_composition_arc_count_below_min() {
        let mut arc_structure = std::collections::HashMap::new();
        arc_structure.insert("evidence".into(), make_arc_def(vec![], vec!["chart"], 2, 5));

        let request = CompositionRequest {
            composition: vec![
                CompositionSlide { slide_type: "chart".into(), arc: "evidence".into(), bg_style: None },
            ],
            arc_structure,
            constraints: make_constraints(1, 5),
        };
        let r = validate_composition(&request);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("evidence arc has 1 slides, minimum is 2")));
    }

    #[test]
    fn test_composition_arc_count_above_max() {
        let mut arc_structure = std::collections::HashMap::new();
        arc_structure.insert("evidence".into(), make_arc_def(vec![], vec!["chart"], 1, 2));

        let request = CompositionRequest {
            composition: vec![
                CompositionSlide { slide_type: "chart".into(), arc: "evidence".into(), bg_style: None },
                CompositionSlide { slide_type: "chart".into(), arc: "evidence".into(), bg_style: None },
                CompositionSlide { slide_type: "chart".into(), arc: "evidence".into(), bg_style: None },
            ],
            arc_structure,
            constraints: make_constraints(1, 5),
        };
        let r = validate_composition(&request);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("evidence arc has 3 slides, maximum is 2")));
    }

    #[test]
    fn test_composition_consecutive_same_type() {
        let mut arc_structure = std::collections::HashMap::new();
        arc_structure.insert("evidence".into(), make_arc_def(vec![], vec!["chart", "list"], 2, 4));

        let request = CompositionRequest {
            composition: vec![
                CompositionSlide { slide_type: "chart".into(), arc: "evidence".into(), bg_style: Some("dark".into()) },
                CompositionSlide { slide_type: "chart".into(), arc: "evidence".into(), bg_style: Some("light".into()) },
            ],
            arc_structure,
            constraints: make_constraints(2, 4),
        };
        let r = validate_composition(&request);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("consecutively")));
    }

    #[test]
    fn test_composition_dld_rhythm_warning() {
        let mut arc_structure = std::collections::HashMap::new();
        arc_structure.insert("evidence".into(), make_arc_def(vec![], vec!["chart", "list"], 2, 4));

        let request = CompositionRequest {
            composition: vec![
                CompositionSlide { slide_type: "chart".into(), arc: "evidence".into(), bg_style: Some("dark".into()) },
                CompositionSlide { slide_type: "list".into(), arc: "evidence".into(), bg_style: Some("dark".into()) },
            ],
            arc_structure,
            constraints: make_constraints(2, 4),
        };
        let r = validate_composition(&request);
        // DLD rhythm is a warning, not an error
        assert!(r.valid);
        assert!(!r.warnings.is_empty());
        assert!(r.warnings.iter().any(|w| w.contains("Background rhythm break")));
    }

    #[test]
    fn test_composition_total_too_few() {
        let mut arc_structure = std::collections::HashMap::new();
        arc_structure.insert("hook".into(), make_arc_def(vec!["hero"], vec![], 1, 1));
        arc_structure.insert("action".into(), make_arc_def(vec!["cta"], vec![], 1, 1));

        let request = CompositionRequest {
            composition: vec![
                CompositionSlide { slide_type: "hero".into(), arc: "hook".into(), bg_style: None },
                CompositionSlide { slide_type: "cta".into(), arc: "action".into(), bg_style: None },
            ],
            arc_structure,
            constraints: make_constraints(5, 10),
        };
        let r = validate_composition(&request);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("2 slides, minimum is 5")));
    }

    #[test]
    fn test_composition_dataviz_pacing() {
        let mut arc_structure = std::collections::HashMap::new();
        arc_structure.insert("evidence".into(), make_arc_def(
            vec![],
            vec!["chart", "scatter_plot", "list", "definition"],
            5, 6,
        ));

        let request = CompositionRequest {
            composition: vec![
                CompositionSlide { slide_type: "chart".into(), arc: "evidence".into(), bg_style: Some("dark".into()) },
                CompositionSlide { slide_type: "scatter_plot".into(), arc: "evidence".into(), bg_style: Some("light".into()) },
                CompositionSlide { slide_type: "chart".into(), arc: "evidence".into(), bg_style: Some("dark".into()) },
                CompositionSlide { slide_type: "scatter_plot".into(), arc: "evidence".into(), bg_style: Some("light".into()) },
            ],
            arc_structure,
            constraints: make_constraints(4, 6),
        };
        let r = validate_composition(&request);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("consecutive dataviz")));
    }

    #[test]
    fn test_composition_unknown_arc() {
        let arc_structure = std::collections::HashMap::new();

        let request = CompositionRequest {
            composition: vec![
                CompositionSlide { slide_type: "hero".into(), arc: "mystery".into(), bg_style: None },
            ],
            arc_structure,
            constraints: make_constraints(1, 3),
        };
        let r = validate_composition(&request);
        assert!(!r.valid);
        assert!(r.errors.iter().any(|e| e.contains("unknown arc")));
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
        if trimmed.parse::<f32>().is_ok() && trimmed != "0" {
            return Some(format!("{property}:{trimmed}"));
        }
    }
    None
}

fn text_has_descender_risk(text: &str) -> bool {
    text.chars()
        .any(|ch| matches!(ch, 'g' | 'j' | 'p' | 'q' | 'y' | 'Q' | 'J'))
}

/// Parse `--text-{level}-size: Npx` declarations from a slide's `<style>` blocks
/// (the per-slide css_vars emitted by the renderer) so `var(--text-*-size)`
/// references can be resolved to concrete pixel sizes.
/// Parse the `.slide-content` vertical padding (top, bottom) from a slide
/// fragment, tolerating `var(--space-N)` tokens in any shorthand slot.
/// Returns `(top, bottom)` in px, defaulting to `(16.0, 20.0)` (the banded
/// body-region defaults used by the renderer) when no `.slide-content`
/// padding can be parsed.
fn parse_slide_content_padding(slide_html: &str) -> (f32, f32) {
    // Extract the padding declaration inside the .slide-content style attribute.
    let style_re = Regex::new(r#"class="slide-content"[^>]*style="([^"]*)""#).unwrap();
    let Some(cap) = style_re.captures(slide_html) else {
        return (16.0, 20.0);
    };
    let style = cap.get(1).map(|m| m.as_str()).unwrap_or("");
    let padding_re = Regex::new(r#"padding\s*:\s*([^;"]+)"#).unwrap();
    let Some(pcap) = padding_re.captures(style) else {
        return (16.0, 20.0);
    };
    let decl = pcap.get(1).map(|m| m.as_str()).unwrap_or("");
    // Extract numeric px tokens; var(--space-N, fallback) contributes its px
    // fallback when present, otherwise it is treated as 0 (horizontal slots
    // do not matter for vertical measurement).
    let token_re = Regex::new(r"(?:var\(--space-\d+,\s*)?([0-9.]+)px").unwrap();
    let values: Vec<f32> = token_re
        .captures_iter(decl)
        .filter_map(|c| c.get(1).and_then(|m| m.as_str().parse::<f32>().ok()))
        .collect();
    match values.len() {
        0 => (16.0, 20.0),
        1 => (values[0], values[0]),
        2 => (values[0], values[0]), // t b → top/bottom = t
        3 => (values[0], values[2]), // t r b
        _ => (values[0], values[2]), // t r b l → bottom = b
    }
}

fn parse_css_size_vars(slide_html: &str) -> HashMap<String, f32> {
    let mut map = HashMap::new();
    let re = Regex::new(r"(--text-[a-z0-9]+-size):\s*([0-9.]+)px").unwrap();
    for cap in re.captures_iter(slide_html) {
        if let Some(v) = cap.get(2).and_then(|m| m.as_str().parse::<f32>().ok()) {
            map.insert(cap.get(1).unwrap().as_str().to_string(), v);
        }
    }
    map
}

/// Resolve an element's font size from its inline style: an explicit px value,
/// a `var(--text-*-size)` reference (via `css_vars`), or a 16px fallback.
fn resolve_font_size(style: &str, css_vars: &HashMap<String, f32>) -> f32 {
    if let Some(px) = numeric_style_value(style, "font-size") {
        return px;
    }
    let var_re = Regex::new(r"var\(--text-([a-z0-9]+)-size\)").unwrap();
    if let Some(cap) = var_re.captures(style) {
        let key = format!("--text-{}-size", cap.get(1).unwrap().as_str());
        if let Some(v) = css_vars.get(&key) {
            return *v;
        }
    }
    16.0
}

/// Estimate the total rendered height of the slide's text stack. Walks leaf text
/// elements (p, h1-h6, blockquote, span, li) that directly contain text, resolves
/// their font size (inline px or css var), and sums wrapped line height plus
/// inter-block margins. Absolute-positioned elements (decorations) are skipped
/// because they do not contribute flow height.
fn estimate_slide_text_height(slide_html: &str, css_vars: &HashMap<String, f32>) -> f32 {
    let mut total = 0.0;
    let text_re = Regex::new(
        r#"(?s)<(p|h[1-6]|blockquote|span|li)\s+[^>]*style="([^"]*)"[^>]*>([^<]{2,})</"#,
    )
    .unwrap();
    // Blockquote slides (quote_slide) wrap their text in a glass card with a
    // decorative quote mark, divider, and attribution — QUOTE_CHROME_HEIGHT of
    // fixed chrome the text-sum alone never sees. The renderer's fit budgets
    // the quote text at QUOTE_TEXT_BUDGET; without this overhead the gate lets
    // borderline wall-of-text quotes through (browser-measured drift). Add it
    // once per blockquote. Constants shared with the renderer (single point).
    let mut blockquote_chrome = 0.0;
    let mut seen_blockquote = false;
    let absolute_re = Regex::new(r"position\s*:\s*absolute").unwrap();
    for cap in text_re.captures_iter(slide_html) {
        let style = cap.get(2).map(|m| m.as_str()).unwrap_or("");
        let absolute = absolute_re.is_match(style);
        let tag = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        // Blockquotes live inside glass cards (quote_slide), so their text column
        // is QUOTE_COLUMN_WIDTH (272px), not the full 332px DEFAULT_COLUMN_WIDTH.
        // Using the wider fallback underestimates wrapping and lets
        // borderline-overflowing quotes through the gate — exactly the drift the
        // browser measurement caught. Constant shared with the renderer's fit.
        let fallback_width = if tag == "blockquote" {
            crate::overflow_model::QUOTE_COLUMN_WIDTH
        } else {
            crate::overflow_model::DEFAULT_COLUMN_WIDTH
        };
        if absolute {
            continue;
        }
        let raw_text = cap.get(3).map(|m| m.as_str()).unwrap_or("");
        let plain = raw_text.replace("&amp;", "&").replace("&nbsp;", " ");
        if plain.trim().is_empty() {
            continue;
        }
        let font_size = resolve_font_size(style, css_vars);
        let line_height = numeric_style_value(style, "line-height").unwrap_or(1.2);
        let width = numeric_px_style_value(style, "max-width")
            .or_else(|| numeric_px_style_value(style, "width"))
            .unwrap_or(fallback_width);
        total +=
            crate::overflow_model::estimate_text_height(&plain, font_size, line_height, width);
        total += 8.0; // inter-block margin approximation
        if tag == "blockquote" && !seen_blockquote {
            seen_blockquote = true;
            blockquote_chrome = crate::overflow_model::QUOTE_CHROME_HEIGHT;
        }
    }
    total + blockquote_chrome
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

/// Sum the text length captured by `text_tag_re` strictly inside the FIRST grid container
/// (i.e. an element that uses `display:grid` / `grid-template-columns`). Walks back from the
/// first `grid-template-columns` occurrence to its containing `<div ...>` opening tag, then
/// pairs balanced `<div>` opens/closes to extract that grid's HTML substring. Titles, eyebrows,
/// captions and slide-level chrome outside the grid are intentionally excluded so the threshold
/// reflects the actual overflow surface, not the whole slide.
///
/// Falls back to the slide-wide total if no balanced grid container can be identified
/// (e.g. nested grid containers that re-open immediately, or pathological HTML).
fn grid_container_text_len(slide_html: &str, text_tag_re: &regex::Regex) -> usize {
    let grid_idx = slide_html.find("grid-template-columns").or_else(|| slide_html.find("display:grid"));
    let Some(grid_idx) = grid_idx else {
        return 0;
    };
    // Walk backward to the opening <div ...> that begins the grid container.
    // We require the start of <div ... > at depth 0 — i.e. unmatched open at this level.
    let bytes = slide_html.as_bytes();
    let mut depth: i32 = 0;
    let mut open_start: Option<usize> = None;
    let mut i = grid_idx;
    loop {
        // Find the previous <div ...> or </div> boundary at or before i
        let prev_close = slide_html[..i].rfind("</div>");
        let prev_open = slide_html[..i].rfind("<div");
        match (prev_close, prev_open) {
            (Some(c), Some(o)) if o > c => {
                // <div ...> is more recent than </div> at this level
                // Check if this <div is at depth 0 (i.e. we have depth unmatched closes since)
                if depth == 0 {
                    open_start = Some(o);
                    break;
                } else {
                    depth -= 1;
                    i = o;
                }
            }
            (Some(c), _) => {
                depth += 1;
                i = c;
            }
            (None, Some(o)) => {
                if depth == 0 {
                    open_start = Some(o);
                    break;
                } else {
                    depth -= 1;
                    i = o;
                }
            }
            (None, None) => break,
        }
        if i == 0 {
            break;
        }
    }
    let Some(open_start) = open_start else {
        // Could not balance — fall back to slide-wide total to preserve the old behavior
        // in pathological cases, but log via the issue's own fallback path.
        return text_tag_re
            .captures_iter(slide_html)
            .map(|cap| cap.get(3).map(|m| m.as_str().trim().len()).unwrap_or(0))
            .sum();
    };
    // Walk forward from open_start to find the matching </div> for the grid container.
    let open_end_rel = slide_html[open_start..].find('>').map(|p| open_start + p + 1);
    let Some(open_end) = open_end_rel else {
        return 0;
    };
    // depth starts at 1 after consuming the opening tag
    let mut depth: i32 = 1;
    let mut j = open_end;
    while j < bytes.len() {
        // Use simple substring matching for performance.
        let remaining = &slide_html[j..];
        let next_open = remaining.find("<div");
        let next_close = remaining.find("</div>");
        match (next_open, next_close) {
            (Some(o), Some(c)) if o < c => {
                // Make sure it's an actual <div ...> tag (not e.g. <divider>).
                let abs = j + o;
                let after = slide_html.as_bytes().get(abs + 4).copied();
                let is_div_tag = matches!(after, Some(b' ') | Some(b'>') | Some(b'\t') | Some(b'\n') | Some(b'\r'));
                if is_div_tag {
                    depth += 1;
                    j = abs + 4;
                } else {
                    j = abs + 4;
                }
            }
            (_, Some(c)) => {
                depth -= 1;
                let abs = j + c;
                if depth == 0 {
                    let grid_end = abs + "</div>".len();
                    let grid_html = &slide_html[open_start..grid_end];
                    return text_tag_re
                        .captures_iter(grid_html)
                        .map(|cap| cap.get(3).map(|m| m.as_str().trim().len()).unwrap_or(0))
                        .sum();
                }
                j = abs + "</div>".len();
            }
            (Some(o), None) => {
                let abs = j + o;
                let after = slide_html.as_bytes().get(abs + 4).copied();
                let is_div_tag = matches!(after, Some(b' ') | Some(b'>') | Some(b'\t') | Some(b'\n') | Some(b'\r'));
                if is_div_tag {
                    depth += 1;
                    j = abs + 4;
                } else {
                    j = abs + 4;
                }
            }
            (None, None) => break,
        }
    }
    // Could not balance — fall back
    text_tag_re
        .captures_iter(slide_html)
        .map(|cap| cap.get(3).map(|m| m.as_str().trim().len()).unwrap_or(0))
        .sum()
}

/// Per-slide layout geometry report for the `debug-layout` CLI/MCP tool.
///
/// Reports the banded-chrome geometry (36px header band, 40px footer band,
/// body region = composition − bands − content padding) alongside the
/// estimated text stack so a debugger can see exactly why a slide does or
/// does not fit — the same numbers the text-overflow gate uses.
pub fn debug_layout(html: &str) -> serde_json::Value {
    let slide_start_re = Regex::new(r#"<div\b[^>]*?\bclass="([^"]*)""#).unwrap();
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
    if slides.is_empty() {
        slides.push(html);
    }

    let mut per_slide = Vec::new();
    for (idx, slide_html) in slides.iter().enumerate() {
        let (pad_top, pad_bottom) = parse_slide_content_padding(slide_html);
        let available =
            crate::overflow_model::available_content_height(pad_top, pad_bottom);
        let css_vars = parse_css_size_vars(slide_html);
        let est = estimate_slide_text_height(slide_html, &css_vars);
        let overflow = est > available;
        per_slide.push(json!({
            "slide": idx + 1,
            "chrome": {
                "header_band": crate::overflow_model::CHROME_HEADER_HEIGHT,
                "footer_band": crate::overflow_model::CHROME_FOOTER_HEIGHT,
                "body_region": crate::overflow_model::COMPOSITION_HEIGHT
                    - crate::overflow_model::CHROME_HEADER_HEIGHT
                    - crate::overflow_model::CHROME_FOOTER_HEIGHT,
            },
            "content_padding": { "top": pad_top, "bottom": pad_bottom },
            "available_height": available,
            "estimated_text_height": est,
            "margin_px": (available - est).max(0.0),
            "overflow": overflow,
            "tight": !overflow && est > available * 0.92,
        }));
    }

    let total = per_slide.len();
    let overflow_count = per_slide
        .iter()
        .filter(|s| s["overflow"] == json!(true))
        .count();
    json!({
        "slides": per_slide,
        "total": total,
        "overflowing": overflow_count,
        "chrome_model": {
            "composition": crate::overflow_model::COMPOSITION_HEIGHT,
            "header_band": crate::overflow_model::CHROME_HEADER_HEIGHT,
            "footer_band": crate::overflow_model::CHROME_FOOTER_HEIGHT,
        },
    })
}

pub fn validate_design(html: &str) -> ValidationReport {
    let mut issues = Vec::new();    // Split the HTML into slides. Attribute-order agnostic: the renderer emits
    // `<div id="slide-0" class="slide slide--light">` (id BEFORE class), so the
    // matcher cannot require `class=` to be the first attribute. The exact
    // whitespace-token check below keeps slide-content / slide-composition /
    // slide__overlay from being mistaken for slide boundaries.
    let slide_start_re = Regex::new(r#"<div\b[^>]*?\bclass="([^"]*)""#).unwrap();
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

    // No `.slide` div found → the input is a bare slide fragment (compile-time
    // validation passes single-slide HTML). Validate it as one slide so the
    // per-slide checks actually run instead of silently passing zero slides.
    // Guard: a document this large is almost certainly a carousel whose slide
    // split regex failed to match — flag it instead of silently "passing" the
    // whole doc as one slide (the original silent-no-op bug class).
    if slides.is_empty() {
        let looks_like_carousel = html.len() > 20_000
            && html.matches("slide-composition").count() >= 2;
        if looks_like_carousel {
            issues.push(DesignIssue {
                slide: 1,
                r#type: "slide_split_failed".to_string(),
                severity: "warning".to_string(),
                detail: format!(
                    "No `.slide` divs matched in a {} byte document with {} `.slide-composition` nodes — the slide-split regex likely failed (attribute order?).",
                    html.len(),
                    html.matches("slide-composition").count()
                ),
                message: "Carousel slide-splitting did not find any slides.".to_string(),
                suggestion: "Verify the slide div markup (id/class attribute order) matches the split regex, and re-run validation.".to_string(),
            });
        }
        slides.push(html);
    }

    let slide_count = slides.len().max(1);

    // Regex for text tags without backreferences — only leaf text elements
    // (divs are skipped because they always wrap nested content and would
    // double-flag inner rgba bypass patterns).
    let text_tag_re =
        Regex::new(r#"(?s)<(p|h[1-6]|span)\s*([^>]*?)>(.*?)</(p|h[1-6]|span)>"#).unwrap();
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
    // Hardcoded low-alpha rgba text colors that bypass the design-token
    // system. `rgba(255,255,255,X<0.7)` white-text on dark bg and
    // `rgba(0,0,0,X<0.7)` black-text on light bg collapse against textured
    // backgrounds (mesh/gradient/hero) — the funnel_chart #22 and
    // image_gallery #34 contrast bugs were both caused by these patterns.
    let low_alpha_text_re = Regex::new(
        r#"rgba\(\s*(?:255\s*,\s*255\s*,\s*255|0\s*,\s*0\s*,\s*0)\s*,\s*(0?\.[0-9]+|0)\s*\)"#,
    )
    .unwrap();

    // New: Header/footer overflow detection
    // Check if slide-content content overflows the 420x525 composition bounds.
    // Body region = composition minus the real 36px header band and 40px footer
    // band (single calibration point in overflow_model.rs).
    const COMP_HEIGHT: f32 = crate::overflow_model::COMPOSITION_HEIGHT;
    const HEADER_HEIGHT: f32 = crate::overflow_model::CHROME_HEADER_HEIGHT;
    const FOOTER_HEIGHT: f32 = crate::overflow_model::CHROME_FOOTER_HEIGHT;
    const SAFE_CONTENT_HEIGHT: f32 = crate::overflow_model::SAFE_CONTENT_HEIGHT;

    for (idx, slide_html) in slides.iter().enumerate() {
        // Extract slide-content padding (var()-tolerant, banded body region)
        let (content_padding_top, content_padding_bottom) = parse_slide_content_padding(slide_html);

        // Calculate available content height
        let available_height = SAFE_CONTENT_HEIGHT - content_padding_top - content_padding_bottom;

        // Check for multi-item layouts that commonly overflow.
        // `comparison` and `checklist_action_plan` retired — multi-item overflow now
        // covered by process_map / pricing_plan. `before_after_story` (the comparison
        // redirect target) is 3-tile by definition so it cannot overflow.
        let multi_item_indicators = [
            ("process_map", 6, "steps"),
            ("pricing_plan", 3, "plans"),
        ];

        for (slide_type, threshold, item_name) in multi_item_indicators {
            if slide_html.contains(slide_type) {
                // Count items by looking for sequential numbers 
                let item_count = (1..=threshold + 2).filter(|i| slide_html.contains(&format!("0{:02}", i))).count().max(1);
                
                if item_count >= threshold {
                    let estimated_item_height = item_count as f32 * 50.0; // Rough estimate per item
                    if estimated_item_height > available_height {
                        issues.push(DesignIssue {
                            slide: idx + 1,
                            r#type: format!("{}_overflow", slide_type).to_string(),
                            severity: "error".to_string(),
                            detail: format!("{} slide with {} {} items overflows composition bounds. Estimated content height: {}px, available: {}px", 
                                         slide_type, item_count, item_name, estimated_item_height, available_height),
                            message: format!("The {} layout with {} items exceeds the safe composition height of {}px (420x525 total minus header/footer/padding).", 
                                         slide_type, item_count, SAFE_CONTENT_HEIGHT),
                            suggestion: format!("Implement dynamic scaling for {} slides with {}+ {} items: reduce padding, font sizes, and gaps to fit within {}px available space.", 
                                         slide_type, threshold, item_name, available_height),
                        });
                    }
                }
            }
        }
    }

    // Check for multiple competing CTAs or buttons in the slide array
    let mut cta_slides = Vec::new();
    for (idx, slide_html) in slides.iter().enumerate() {
        let has_button = slide_html.contains("class=\"btn\"") || slide_html.contains("class='btn'");
        let has_qr = slide_html.contains("data:image/svg+xml") && slide_html.contains("Scan");
        if has_button || has_qr {
            cta_slides.push(idx + 1);
        }
    }
    if cta_slides.len() > 1 {
        issues.push(DesignIssue {
            slide: cta_slides[0],
            r#type: "competing_ctas".to_string(),
            severity: "warning".to_string(),
            detail: format!("Multiple competing Call-To-Action (CTA) slides detected on slides: {:?}", cta_slides),
            message: "Each carousel-set must only have a single CTA slide to ensure maximum user engagement and focus.".to_string(),
            suggestion: "Remove the duplicate/competing CTA slides or convert them to standard content/informational layouts.".to_string(),
        });
    }

    // Check for non-interactive buttons on social platforms
    for (idx, slide_html) in slides.iter().enumerate() {
        let has_button = slide_html.contains("class=\"btn\"") || slide_html.contains("class='btn'");
        let has_qr = slide_html.contains("data:image/svg+xml") && slide_html.contains("Scan");
        if has_button && !has_qr {
            issues.push(DesignIssue {
                slide: idx + 1,
                r#type: "non_interactive_button".to_string(),
                severity: "warning".to_string(),
                detail: "Slide contains a web-styled button ('class=\"btn\"') without a companion QR code.".to_string(),
                message: "Web buttons are non-interactive on image-based social media platforms (Instagram, TikTok).".to_string(),
                suggestion: "Use a 'qr_destination' slide for scannable redirection, or frame the text as a 'Link in Bio' action.".to_string(),
            });
        }
    }

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

    // Check 1: progress placement. In the banded chrome architecture the
    // progress bar lives INSIDE the .slide-footer band (in-flow), so it can
    // never collide with slide-body content. Flag any absolutely-positioned
    // progress element (legacy overlay structure) that could overlap content.
    let progress_abs_re = Regex::new(
        r#"(?s)\.breadcrumb-progress\s*\{[^}]*position\s*:\s*absolute"#,
    )
    .unwrap();
    if progress_abs_re.is_match(html) {
        issues.push(DesignIssue {
            slide: 1,
            r#type: "progress_overlay_collision".to_string(),
            severity: "warning".to_string(),
            detail: "breadcrumb-progress uses position:absolute — the legacy overlay structure can overlap slide-body content.".to_string(),
            message: "Progress indicator must live inside the .slide-footer band so it cannot collide with slide content.".to_string(),
            suggestion: "Move breadcrumb-progress into the .slide-footer band as an in-flow flex child.".to_string(),
        });
    }

    // Check 2: full-bleed stretch rule presence. If any slide has
    // slide--full-bleed class, the CSS must contain the first-of-type
    // stretch rule with !important on width/height.
    let has_full_bleed_slide = html.contains("slide--full-bleed");
    if has_full_bleed_slide {
        let stretch_rule_re = Regex::new(
            r#"(?s)\.slide--full-bleed\s+\.slide-body\s*>\s*div:first-of-type\s*\{[^}]*width:\s*var\(--slide-width\)\s*!important[^}]*height:\s*var\(--slide-height\)\s*!important"#,
        )
        .unwrap();
        if !stretch_rule_re.is_match(html) {
            issues.push(DesignIssue {
                slide: 1,
                r#type: "missing_full_bleed_stretch_rule".to_string(),
                severity: "error".to_string(),
                detail: "Full-bleed slides are present but the CSS lacks the .slide--full-bleed .slide-body > div:first-of-type stretch rule with !important.".to_string(),
                message: "Background layers on full-bleed slides will be clipped to the 420x525 composition instead of filling the canvas.".to_string(),
                suggestion: "Add: .slide--full-bleed .slide-body > div:first-of-type { position:absolute!important; width:var(--slide-width)!important; height:var(--slide-height)!important; }".to_string(),
            });
        }

        // Check 2b: .slide-content must be vertically CENTERED on full-bleed
        // canvases. If the rule uses `top:0` instead of
        // `top: calc((var(--slide-height) - var(--composition-height)) / 2)`,
        // 9:16 slides will have content clumped at the top with empty bg below.
        let content_top_re =
            Regex::new(r#"(?s)\.slide--full-bleed\s+\.slide-content\s*\{[^}]*top\s*:\s*([^;]+);"#)
                .unwrap();
        if let Some(cap) = content_top_re.captures(html) {
            let top_val = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            // Acceptable: calc((var(--slide-height) - var(--composition-height)) / 2)
            // or any calc() expression referencing both --slide-height and --composition-height.
            // Flag: top:0 or top:0px (content clumped at top on 9:16).
            let is_centered = top_val.contains("calc(")
                && top_val.contains("--slide-height")
                && top_val.contains("--composition-height");
            let is_top_zero = top_val == "0" || top_val == "0px" || top_val == "0 !important";
            if is_top_zero && !is_centered {
                issues.push(DesignIssue {
                    slide: 1,
                    r#type: "full_bleed_content_top_anchored".to_string(),
                    severity: "error".to_string(),
                    detail: format!(
                        ".slide--full-bleed .slide-content uses top:{} — content is anchored to the top of the canvas instead of vertically centered.",
                        top_val
                    ),
                    message: "On portrait (9:16, 3:4) full-bleed canvases, content clumps at the top leaving a large empty band at the bottom.".to_string(),
                    suggestion: "Use: top: calc((var(--slide-height) - var(--composition-height)) / 2) !important; to vertically center the 420x525 composition within the canvas.".to_string(),
                });
            }
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

    // Check 5: orphan grid cells — DISABLED (causes perf issues with depth
    // scanning on complex nested HTML). The carousel_23 fix (adaptive grid
    // columns based on proof_points count) addresses the root cause. This
    // check can be re-enabled with a proper HTML parser in the future.

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

        if slide_html.contains("grid-template-columns") || slide_html.contains("display:grid") {
            // Scope the text-mass budget to the grid container only; titles/eyebrows/captions
            // outside the grid are not part of the overflow surface and must not be counted.
            // Threshold tracks the renderer's dynamic-scaling tier model — `grid_cards_slide`
            // was retired 2026-07-30; the surviving N-card grids (`split_features`, `case_study_result`,
            // `pricing_plan`) use the same `very_dense` scaling tiers and absorb up to ~600 chars
            // per card. 4 cards in that tier top out at ~2400 chars and still render inside the
            // 405px safe content height. Validator ceiling set at 2500 chars — a small margin above
            // the maximum observed safe composition.
            // Per directive #1638 the validator must share the renderer's model rather than
            // blocking compositions the dynamic-scaling pipeline can already place safely.
            let grid_text_len = grid_container_text_len(slide_html, &text_tag_re);
            const GRID_TEXT_BUDGET: usize = 2500;
            if grid_text_len > GRID_TEXT_BUDGET {
                issues.push(DesignIssue {
                    slide: slide_num,
                    r#type: "grid_cards_overflow_risk".to_string(),
                    severity: "error".to_string(),
                    detail: format!("Grid container contains {} total characters of text.", grid_text_len),
                    message: "Excessive text mass inside card grid creates unacceptable vertical overflow.".to_string(),
                    suggestion: "Reduce card text content or use compact/list-dense layout variant to fit within card bounds.".to_string(),
                });
            }
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
                        // Check for hardcoded low-alpha rgba text colors in any element's attributes
            let mut is_text_bypass = false;
            for part in style_str.split(';') {
                let part = part.trim();
                if part.starts_with("color:") || part.starts_with("border-color:") {
                    if let Some(caps) = low_alpha_text_re.captures(part) {
                        let alpha: f64 = caps
                            .get(1)
                            .and_then(|m| m.as_str().parse().ok())
                            .unwrap_or(1.0);
                        if alpha < 0.7 {
                            is_text_bypass = true;
                            break;
                        }
                    }
                }
            }

            if is_text_bypass {
                let display_text = if plain_text.len() > 20 {
                    format!("{}...", &plain_text[..20])
                } else {
                    plain_text.clone()
                };
                issues.push(DesignIssue {
                    slide: slide_num,
                    r#type: "hardcoded_rgba_text_bypass".to_string(),
                    severity: "error".to_string(),
                    detail: format!("Text '{}' uses hardcoded low-alpha rgba that bypasses the design-token color system.", display_text),
                    message: "Text with alpha < 0.7 on pure black or white uses low-contrast colors that can collapse against textured backgrounds.".to_string(),
                    suggestion: "Use colors.text_primary or colors.text_secondary from the design-token system, which guarantee contrast-⁠safe colors for the current theme.".to_string(),
                });
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

        // ── General text-overflow gate (shared overflow model) ────────────────
        // Estimates the slide's text stack against the available content height so
        // oversized display-tier text (e.g. 113px quotes, 80px headlines) is caught
        // even when the composition clips it with overflow:hidden. Uses the same
        // model as the renderer's automatic scaling so both sides agree.
        // Parse `.slide-content` vertical padding with the var()-tolerant
        // parser so `16px var(--space-6) 20px` shorthand resolves to
        // (top=16, bottom=20) instead of falling back to an over-conservative
        // (60,60). Shared with the renderer's banded body-region geometry.
        let (pad_top, pad_bottom) = parse_slide_content_padding(slide_html);
        let available =
            crate::overflow_model::available_content_height(pad_top, pad_bottom);
        let css_vars = parse_css_size_vars(slide_html);
        let est = estimate_slide_text_height(slide_html, &css_vars);
        if est > available {
            issues.push(DesignIssue {
                slide: slide_num,
                r#type: "text_overflow".to_string(),
                severity: "error".to_string(),
                detail: format!(
                    "Estimated text stack height {:.0}px exceeds available content height {:.0}px (composition {:.0}px − padding {:.0}/{:.0}).",
                    est,
                    available,
                    crate::overflow_model::COMPOSITION_HEIGHT,
                    pad_top,
                    pad_bottom
                ),
                message: "Text content overflows the slide body and will be clipped at export.".to_string(),
                suggestion: "Scale the component (reduce display-tier font sizes, paddings, and gaps) or shorten the copy so the text stack fits the available height.".to_string(),
            });
        } else if est > available * 0.92 {
            issues.push(DesignIssue {
                slide: slide_num,
                r#type: "text_overflow_tight".to_string(),
                severity: "warning".to_string(),
                detail: format!(
                    "Estimated text stack height {:.0}px is within 8% of the available {:.0}px.",
                    est, available
                ),
                message: "Text content nearly overflows the slide body.".to_string(),
                suggestion: "Give the text stack a small safety margin by reducing font size or padding.".to_string(),
            });
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
