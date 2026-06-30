use serde_json::{Value, json};

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
        "section_divider": {
            "description": "Chapter opener or section break with kicker, title, and subtitle",
            "required_params": ["title"],
            "optional_params": ["subtitle", "kicker", "background_image", "image_opacity", "variant"],
            "variants": ["chapter", "minimal", "statement"],
            "default_variant": "chapter",
            "layout_family": "hero",
            "best_for": ["opening", "section-intro", "chapter-break"]
        },
        "problem_solution": {
            "description": "Problem and solution pair with optional proof points",
            "required_params": ["problem", "solution"],
            "optional_params": ["title", "proof_points", "variant"],
            "variants": ["split", "proof-grid"],
            "default_variant": "split",
            "layout_family": "strategy",
            "best_for": ["education", "sales", "positioning"]
        },
        "myth_fact": {
            "description": "Myth versus fact educational contrast slide",
            "required_params": ["myth", "fact"],
            "optional_params": ["explanation", "variant"],
            "variants": ["split", "debunk"],
            "default_variant": "split",
            "layout_family": "educational",
            "best_for": ["education", "thought-leadership"]
        },
        "checklist_action_plan": {
            "description": "Action checklist with numbered execution steps",
            "required_params": ["title", "items"],
            "optional_params": ["variant"],
            "variants": ["numbered", "checklist"],
            "default_variant": "numbered",
            "layout_family": "process",
            "best_for": ["how-to", "closing", "steps"]
        },
        "case_study_result": {
            "description": "Case study with challenge, solution, and result proof points",
            "required_params": ["challenge", "solution", "results"],
            "optional_params": ["client", "title", "variant"],
            "variants": ["summary", "results-grid"],
            "default_variant": "summary",
            "layout_family": "social-proof",
            "best_for": ["case-study", "proof", "results"]
        },
        "pricing_plan": {
            "description": "Pricing or offer stack with up to three plans",
            "required_params": ["title", "plans"],
            "optional_params": ["variant"],
            "variants": ["cards", "offer-stack"],
            "default_variant": "cards",
            "layout_family": "conversion",
            "best_for": ["pricing", "offer", "conversion"]
        },
        "testimonial_avatar": {
            "description": "Testimonial quote with person/avatar attribution",
            "required_params": ["quote", "author"],
            "optional_params": ["role", "avatar_url", "variant"],
            "variants": ["centered", "profile"],
            "default_variant": "centered",
            "layout_family": "social-proof",
            "best_for": ["testimonial", "social-proof"]
        },
        "logo_cloud": {
            "description": "Grid of customer, press, or partner names/logos",
            "required_params": ["title", "logos"],
            "optional_params": ["variant"],
            "variants": ["grid", "strip"],
            "default_variant": "grid",
            "layout_family": "social-proof",
            "best_for": ["credibility", "logos", "partners"]
        },
        "faq": {
            "description": "Frequently asked questions or objection handling slide",
            "required_params": ["title", "questions"],
            "optional_params": ["variant"],
            "variants": ["stack", "compact"],
            "default_variant": "stack",
            "layout_family": "content",
            "best_for": ["objections", "education", "support"]
        },
        "process_map": {
            "description": "Process map or workflow with sequential steps",
            "required_params": ["title", "steps"],
            "optional_params": ["variant"],
            "variants": ["numbered", "map"],
            "default_variant": "numbered",
            "layout_family": "process",
            "best_for": ["process", "workflow", "roadmap"]
        },
        "before_after_story": {
            "description": "Text and metric before/after transformation story",
            "required_params": ["title", "before", "after"],
            "optional_params": ["metric", "variant"],
            "variants": ["split", "metric"],
            "default_variant": "split",
            "layout_family": "story",
            "best_for": ["transformation", "results", "case-study"]
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
        },
        "image_caption": {
            "description": "Single image with caption/description below or overlay",
            "required_params": ["image_url", "caption"],
            "optional_params": ["description", "layout", "variant", "image_filter", "image_overlay", "image_frame", "image_mask", "image_position", "image_mix_blend", "image_opacity"],
            "variants": ["image-top", "image-bottom", "image-left", "image-right", "image-overlay"],
            "default_variant": "image-top",
            "layout_family": "image",
            "best_for": ["image", "visual", "caption"]
        },
        "image_headline": {
            "description": "Image with large headline overlay",
            "required_params": ["image_url", "headline"],
            "optional_params": ["subheadline", "overlay_position", "variant", "image_filter", "image_overlay", "image_frame", "image_mask", "image_position", "image_mix_blend", "image_opacity"],
            "variants": ["center", "bottom", "top"],
            "default_variant": "bottom",
            "layout_family": "image",
            "best_for": ["image", "hero", "visual"]
        },
        "image_quote": {
            "description": "Image with quote overlay",
            "required_params": ["image_url", "quote"],
            "optional_params": ["author", "role", "variant", "image_filter", "image_overlay", "image_frame", "image_mask", "image_position", "image_mix_blend", "image_opacity"],
            "variants": ["default"],
            "default_variant": "default",
            "layout_family": "image",
            "best_for": ["image", "quote", "social-proof"]
        },
        "image_callout": {
            "description": "Image with annotation/callout pointing to a feature",
            "required_params": ["image_url", "callouts"],
            "optional_params": ["description", "variant", "image_filter", "image_overlay", "image_frame", "image_mask", "image_position", "image_mix_blend", "image_opacity"],
            "variants": ["default"],
            "default_variant": "default",
            "layout_family": "image",
            "best_for": ["image", "diagram", "annotated"]
        },
        "image_stat": {
            "description": "Image with prominent statistic overlay",
            "required_params": ["image_url", "stat_value", "stat_label"],
            "optional_params": ["description", "layout", "variant", "image_filter", "image_overlay", "image_frame", "image_mask", "image_position", "image_mix_blend", "image_opacity"],
            "variants": ["image-left", "image-right", "image-top", "image-bottom"],
            "default_variant": "image-left",
            "layout_family": "image",
            "best_for": ["image", "stat", "data"]
        },
        "image_gallery": {
            "description": "Grid of 2-6 images in various layouts",
            "required_params": ["images"],
            "optional_params": ["layout", "title", "section_caption", "variant", "image_filter", "image_overlay", "image_frame", "image_mask", "image_position", "image_mix_blend", "image_opacity"],
            "variants": ["2-grid", "3-grid", "4-grid", "featured-1-2", "featured-2-1"],
            "default_variant": "2-grid",
            "layout_family": "image",
            "best_for": ["image", "gallery", "portfolio"]
        },
        "image_collage": {
            "description": "Artistic collage of overlapping images",
            "required_params": ["images"],
            "optional_params": ["style", "title", "section_caption", "variant", "image_filter", "image_overlay", "image_frame", "image_mask", "image_position", "image_mix_blend", "image_opacity"],
            "variants": ["scattered", "layered", "geometric", "editorial_stack", "mosaic", "filmstrip"],
            "default_variant": "scattered",
            "layout_family": "image",
            "best_for": ["image", "collage", "visual"]
        },
        "image_comparison": {
            "description": "Two images side by side for before/after comparison",
            "required_params": ["before_image", "after_image"],
            "optional_params": ["before_label", "after_label", "description", "divider_style", "variant", "image_filter", "image_overlay", "image_frame", "image_mask", "image_position", "image_mix_blend", "image_opacity"],
            "variants": ["line", "arrow"],
            "default_variant": "line",
            "layout_family": "image",
            "best_for": ["image", "comparison", "before-after"]
        },
        "qr_destination": {
            "description": "Conversion slide with scannable QR code, heading, caption, CTA, and optional short URL fallback",
            "required_params": ["destination_url", "cta_text"],
            "optional_params": ["heading", "caption", "short_url", "brand_name", "brand_logo", "incentive_text", "qr_alt_text", "variant", "background_image", "image_opacity", "padding"],
            "variants": ["theme-bg", "image-bg", "minimal", "with-heading", "without-heading", "with-caption", "with-cta", "full-conversion"],
            "default_variant": "full-conversion",
            "layout_family": "conversion",
            "best_for": ["conversion", "closing", "off-platform", "blog", "donation", "digital-product", "newsletter", "link-hub"]
        }
    })
}

/// Returns a sorted list of all slide type keys.
pub fn list_slide_types() -> Vec<String> {
    let registry = get_registry();
    let mut types: Vec<String> = registry
        .as_object()
        .map(|m: &serde_json::Map<String, Value>| m.keys().cloned().collect::<Vec<String>>())
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
    if context == "off-platform" {
        return vec!["qr_destination".to_string(), "cta".to_string()];
    }

    let registry = get_registry();
    let map: serde_json::Map<String, Value> = match registry.into() {
        Value::Object(m) => m,
        _ => return vec![],
    };

    let matched = map.iter()
        .filter_map(|(slide_type, info): (&String, &Value)| {
            let best_for = info.get("best_for")?.as_array()?;
            let matches = best_for
                .iter()
                .any(|v: &Value| v.as_str().map(|s| s == context).unwrap_or(false));
            if matches {
                Some(slide_type.clone())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    if context == "conversion" {
        let preferred = vec!["qr_destination".to_string(), "cta".to_string(), "pricing_plan".to_string()];
        let mut result = Vec::new();
        for p in preferred {
            if matched.contains(&p) {
                result.push(p);
            }
        }
        for m in matched {
            if !result.contains(&m) {
                result.push(m);
            }
        }
        result
    } else {
        matched
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_has_all_types() {
        let types = list_slide_types();
        let expected = [
            "callout",
            "chart",
            "comparison",
            "cta",
            "definition",
            "feature",
            "grid_cards",
            "headline_subheadline",
            "hero",
            "list",
            "metric_card",
            "quote",
            "split_features",
            "stat_row",
            "text_block",
            "text_columns",
            "section_divider",
            "problem_solution",
            "myth_fact",
            "checklist_action_plan",
            "case_study_result",
            "pricing_plan",
            "testimonial_avatar",
            "logo_cloud",
            "faq",
            "process_map",
            "before_after_story",
            "timeline",
            "image_caption",
            "image_headline",
            "image_quote",
            "image_callout",
            "image_stat",
            "image_gallery",
            "image_collage",
            "image_comparison",
            "qr_destination",
        ];
        for t in &expected {
            assert!(types.contains(&t.to_string()), "Missing type: {t}");
        }
    }

    #[test]
    fn test_qr_destination_registry_metadata() {
        let info = get_slide_type_info("qr_destination").expect("qr_destination exists");
        assert_eq!(info["layout_family"], "conversion");
        assert!(info["best_for"].as_array().unwrap().iter().any(|v| v == "conversion"));
        assert!(info["variants"].as_array().unwrap().iter().any(|v| v == "full-conversion"));
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

    #[test]
    fn test_get_slide_types_for_context_conversion_includes_qr() {
        let types = get_slide_types_for_context("conversion");
        assert!(types.contains(&"qr_destination".to_string()));
        assert!(types.contains(&"cta".to_string()));
    }

    #[test]
    fn test_get_slide_types_for_context_off_platform_prefers_qr() {
        let types = get_slide_types_for_context("off-platform");
        assert_eq!(types.first().map(|s| s.as_str()), Some("qr_destination"));
    }
}
