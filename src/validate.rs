use serde_json::{json, Value};

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
}
