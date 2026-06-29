use serde_json::{json, Value};

/// Returns the full slide type registry as a JSON object.
pub fn get_registry() -> Value {
    json!({
        "hero": {
            "description": "Hook slide with bold headline — grabs attention at the opening",
            "required_params": ["headline"],
            "optional_params": ["subheadline", "background_color", "text_color", "badge", "cta_text", "cta_url"],
            "variants": ["centered", "left-aligned", "split", "dark", "gradient"],
            "default_variant": "centered",
            "layout_family": "hero",
            "best_for": ["opening", "section-intro"]
        },
        "feature": {
            "description": "Single-feature highlight with icon, title, and description",
            "required_params": ["title", "description"],
            "optional_params": ["icon", "icon_color", "badge", "image_url", "cta_text", "cta_url", "variant"],
            "variants": ["icon-top", "icon-left", "icon-right", "image-split", "minimal"],
            "default_variant": "icon-top",
            "layout_family": "feature",
            "best_for": ["features", "benefits", "product-details"]
        },
        "list": {
            "description": "Bulleted or numbered list of items",
            "required_params": ["title", "items"],
            "optional_params": ["ordered", "icon", "icon_color", "columns", "show_numbers", "variant"],
            "variants": ["bullet", "numbered", "checklist", "icon-list", "two-column"],
            "default_variant": "bullet",
            "layout_family": "list",
            "best_for": ["features", "steps", "requirements", "comparison"]
        },
        "quote": {
            "description": "Testimonial or pull-quote with optional attribution",
            "required_params": ["quote"],
            "optional_params": ["author", "role", "company", "avatar_url", "rating", "logo_url", "variant"],
            "variants": ["centered", "card", "large-quote", "with-avatar", "minimal"],
            "default_variant": "centered",
            "layout_family": "social-proof",
            "best_for": ["social-proof", "testimonials", "credibility"]
        },
        "cta": {
            "description": "Call-to-action slide with button and supporting copy",
            "required_params": ["headline", "button_text"],
            "optional_params": ["subheadline", "button_url", "secondary_text", "background_color", "urgency_text", "variant"],
            "variants": ["centered", "split", "banner", "minimal", "dark"],
            "default_variant": "centered",
            "layout_family": "cta",
            "best_for": ["closing", "conversion", "next-step"]
        },
        "comparison": {
            "description": "Side-by-side comparison grid (e.g. Before/After, Plan A vs Plan B)",
            "required_params": ["title", "columns", "rows"],
            "optional_params": ["highlight_column", "show_checkmarks", "footer_note", "variant"],
            "variants": ["table", "cards", "vs-split", "feature-matrix"],
            "default_variant": "table",
            "layout_family": "data",
            "best_for": ["comparison", "pricing", "features"]
        },
        "stat_row": {
            "description": "Grid of key statistics or metrics",
            "required_params": ["stats"],
            "optional_params": ["title", "columns", "show_icons", "variant"],
            "variants": ["grid", "row", "cards", "minimal", "dark"],
            "default_variant": "grid",
            "layout_family": "data",
            "best_for": ["data", "proof-points", "results"]
        },
        "timeline": {
            "description": "Process or chronological timeline",
            "required_params": ["title", "steps"],
            "optional_params": ["orientation", "show_dates", "icon", "variant"],
            "variants": ["horizontal", "vertical", "numbered", "arrow-chain"],
            "default_variant": "horizontal",
            "layout_family": "process",
            "best_for": ["process", "roadmap", "journey", "steps"]
        },
        "callout": {
            "description": "Highlighted callout card for emphasis or important notices",
            "required_params": ["text"],
            "optional_params": ["title", "icon", "color", "border_color", "variant"],
            "variants": ["info", "warning", "success", "tip", "highlight"],
            "default_variant": "highlight",
            "layout_family": "callout",
            "best_for": ["emphasis", "key-takeaway", "warning", "tip"]
        },
        "split_features": {
            "description": "Two-column feature list, often icon+text pairs side by side",
            "required_params": ["title", "features"],
            "optional_params": ["icon_color", "columns", "show_divider", "variant"],
            "variants": ["two-column", "three-column", "icon-grid", "minimal"],
            "default_variant": "two-column",
            "layout_family": "feature",
            "best_for": ["features", "benefits", "product-overview"]
        },
        "grid_cards": {
            "description": "Grid of cards each with icon, title, and description",
            "required_params": ["title", "cards"],
            "optional_params": ["columns", "card_style", "icon_color", "show_cta", "variant"],
            "variants": ["2-col", "3-col", "4-col", "masonry", "minimal"],
            "default_variant": "3-col",
            "layout_family": "feature",
            "best_for": ["features", "use-cases", "categories", "team"]
        },
        "headline_subheadline": {
            "description": "Bold headline paired with a subheadline for emphasis",
            "required_params": ["headline", "subheadline"],
            "optional_params": ["badge", "text_align", "background_color", "variant"],
            "variants": ["centered", "left-aligned", "large", "minimal"],
            "default_variant": "centered",
            "layout_family": "hero",
            "best_for": ["opening", "section-intro", "key-message"]
        },
        "definition": {
            "description": "Term definition with supporting context or elaboration",
            "required_params": ["term", "definition"],
            "optional_params": ["context", "example", "icon", "variant"],
            "variants": ["card", "inline", "highlighted", "minimal"],
            "default_variant": "card",
            "layout_family": "educational",
            "best_for": ["educational", "glossary", "concept-intro"]
        },
        "text_block": {
            "description": "Title plus body text paragraph(s)",
            "required_params": ["title", "body"],
            "optional_params": ["subtitle", "text_align", "max_width", "variant"],
            "variants": ["left", "centered", "narrow", "wide"],
            "default_variant": "left",
            "layout_family": "content",
            "best_for": ["content", "explanation", "narrative"]
        },
        "text_columns": {
            "description": "Multi-column text layout for dense content",
            "required_params": ["title", "columns"],
            "optional_params": ["column_titles", "show_dividers", "variant"],
            "variants": ["two-column", "three-column", "equal", "sidebar"],
            "default_variant": "two-column",
            "layout_family": "content",
            "best_for": ["comparison", "content", "two-sides"]
        },
        "metric_card": {
            "description": "Single prominent metric with optional trend indicator",
            "required_params": ["metric", "value"],
            "optional_params": ["trend", "trend_direction", "label", "icon", "color", "variant"],
            "variants": ["card", "hero-number", "with-trend", "minimal"],
            "default_variant": "card",
            "layout_family": "data",
            "best_for": ["data", "kpi", "proof-points"]
        },
        "chart": {
            "description": "Data chart — bar, line, pie, donut, scatter, etc.",
            "required_params": ["title", "chart_type", "data"],
            "optional_params": ["x_label", "y_label", "legend", "colors", "show_values", "variant"],
            "variants": ["bar", "line", "pie", "donut", "scatter", "area", "horizontal-bar"],
            "default_variant": "bar",
            "layout_family": "data",
            "best_for": ["data", "trends", "comparison", "results"]
        }
    })
}

/// Returns a sorted list of all slide type keys.
pub fn list_slide_types() -> Vec<String> {
    let registry = get_registry();
    let mut types: Vec<String> = registry
        .as_object()
        .map(|m: &serde_json::Map<String, Value>| {
            m.keys().cloned().collect::<Vec<String>>()
        })
        .unwrap_or_default();
    types.sort();
    types
}

/// Returns the metadata for a single slide type, or None if not found.
pub fn get_slide_type_info(slide_type: &str) -> Option<Value> {
    let registry = get_registry();
    registry
        .as_object()
        .and_then(|m: &serde_json::Map<String, Value>| m.get(slide_type))
        .cloned()
}

/// Returns slide types appropriate for the given context.
///
/// Valid context values: "opening", "closing", "data", "social-proof", "features"
pub fn get_slide_types_for_context(context: &str) -> Vec<String> {
    let registry = get_registry();
    let map: serde_json::Map<String, Value> = match registry.into() {
        Value::Object(m) => m,
        _ => return vec![],
    };

    map.iter()
        .filter_map(|(slide_type, info): (&String, &Value)| {
            let best_for = info.get("best_for")?.as_array()?;
            let matches = best_for.iter().any(|v: &Value| {
                v.as_str().map(|s| s == context).unwrap_or(false)
            });
            if matches {
                Some(slide_type.clone())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_has_all_types() {
        let types = list_slide_types();
        let expected = [
            "callout", "chart", "comparison", "cta", "definition",
            "feature", "grid_cards", "headline_subheadline", "hero",
            "list", "metric_card", "quote", "split_features",
            "stat_row", "text_block", "text_columns", "timeline",
        ];
        for t in &expected {
            assert!(types.contains(&t.to_string()), "Missing type: {t}");
        }
    }

    #[test]
    fn test_get_slide_type_info_hero() {
        let info = get_slide_type_info("hero").expect("hero should exist");
        assert!(info.get("required_params").is_some());
        assert_eq!(info["default_variant"], "centered");
    }

    #[test]
    fn test_get_slide_type_info_unknown() {
        assert!(get_slide_type_info("nonexistent").is_none());
    }

    #[test]
    fn test_get_slide_types_for_context_opening() {
        let types = get_slide_types_for_context("opening");
        assert!(types.contains(&"hero".to_string()));
    }

    #[test]
    fn test_get_slide_types_for_context_data() {
        let types = get_slide_types_for_context("data");
        assert!(types.contains(&"chart".to_string()));
        assert!(types.contains(&"stat_row".to_string()));
    }
}
