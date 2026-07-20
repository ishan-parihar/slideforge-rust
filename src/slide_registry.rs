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
            "best_for": ["opening", "section-intro"],
            "example": {
                "headline": "Ship slides in minutes, not days",
                "subheadline": "AI-driven carousel composition",
                "badge": "NEW",
                "variant": "gradient"
            }
        },
        "feature": {
            "description": "Single-feature highlight with icon, title, and description",
            "required_params": ["title", "description"],
            "optional_params": ["icon", "icon_color", "badge", "image_url", "cta_text", "cta_url", "variant"],
            "variants": ["icon-top", "icon-left", "icon-right", "image-split", "minimal"],
            "default_variant": "icon-top",
            "layout_family": "feature",
            "best_for": ["features", "benefits", "product-details"],
            "example": {
                "title": "Lightning fast",
                "description": "Generate a 10-slide carousel in under 12 seconds.",
                "icon": "⚡",
                "icon_color": "#FFB400",
                "variant": "icon-top"
            }
        },
        "list": {
            "description": "Bulleted or numbered list of items",
            "required_params": ["title", "items"],
            "optional_params": ["ordered", "icon", "icon_color", "columns", "show_numbers", "variant"],
            "variants": ["bullet", "numbered", "checklist", "icon-list", "two-column"],
            "default_variant": "bullet",
            "layout_family": "list",
            "best_for": ["features", "steps", "requirements", "comparison"],
            "example": {
                "title": "Why teams switch",
                "items": [
                    "Native Rust rendering pipeline",
                    "MCP-native agent ergonomics",
                    "Pixel-perfect typography control",
                    "Validator-catch design regressions"
                ],
                "icon": "✓",
                "variant": "checklist"
            }
        },
        "quote": {
            "description": "Testimonial or pull-quote with optional attribution",
            "required_params": ["quote"],
            "optional_params": ["author", "role", "company", "avatar_url", "rating", "logo_url", "variant"],
            "variants": ["centered", "card", "large-quote", "with-avatar", "minimal"],
            "default_variant": "centered",
            "layout_family": "social-proof",
            "best_for": ["social-proof", "testimonials", "credibility"],
            "example": {
                "quote": "We shipped our deck in an afternoon — usually it takes a week.",
                "author": "Maya Chen",
                "role": "Head of Growth",
                "company": "Northwind Labs",
                "rating": 5,
                "variant": "with-avatar"
            }
        },
        "cta": {
            "description": "Call-to-action slide with button and supporting copy",
            "required_params": ["headline", "button_text"],
            "optional_params": ["subheadline", "button_url", "secondary_text", "background_color", "urgency_text", "variant"],
            "variants": ["centered", "split", "banner", "minimal", "dark"],
            "default_variant": "centered",
            "layout_family": "cta",
            "best_for": ["closing", "conversion", "next-step"],
            "example": {
                "headline": "Ready to ship your first deck?",
                "subheadline": "Free for the first 100 creators.",
                "button_text": "Start building",
                "button_url": "https://example.com/signup",
                "variant": "centered"
            }
        },
        "comparison": {
            "description": "Side-by-side comparison grid (e.g. Before/After, Plan A vs Plan B)",
            "required_params": ["title", "columns", "rows"],
            "optional_params": ["highlight_column", "show_checkmarks", "footer_note", "variant"],
            "variants": ["table", "cards", "vs-split", "feature-matrix"],
            "default_variant": "table",
            "layout_family": "data",
            "best_for": ["comparison", "pricing", "features"],
            "example": {
                "title": "Free vs Pro",
                "columns": ["Free", "Pro"],
                "rows": [
                    ["Slides per month", "5", "Unlimited"],
                    ["Custom typography", "—", "✓"],
                    ["Brand presets", "1", "Unlimited"]
                ],
                "highlight_column": 1,
                "variant": "table"
            }
        },
        "stat_row": {
            "description": "Grid of key statistics or metrics",
            "required_params": ["stats"],
            "optional_params": ["title", "columns", "show_icons", "variant"],
            "variants": ["grid", "row", "cards", "minimal", "dark"],
            "default_variant": "grid",
            "layout_family": "data",
            "best_for": ["data", "proof-points", "results"],
            "example": {
                "title": "By the numbers",
                "stats": [
                    {"label": "Active decks", "value": "12k+"},
                    {"label": "Avg render time", "value": "11s"},
                    {"label": "Failure rate", "value": "0.4%"}
                ],
                "columns": 3,
                "variant": "grid"
            }
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
            "variants": ["2-col", "3-col", "4-col", "masonry", "minimal", "dense", "compact", "list-dense"],
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
            "variants": ["debunk"],
            "default_variant": "debunk",
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
            "variants": ["theme-bg", "image-bg", "minimal", "with-heading", "without-heading", "with-caption", "with-cta", "full-conversion", "split-card", "poster", "stacked-badge", "compact"],
            "default_variant": "full-conversion",
            "layout_family": "conversion",
            "best_for": ["conversion", "closing", "off-platform", "blog", "donation", "digital-product", "newsletter", "link-hub"],
            "example": {
                "heading": "Scan to subscribe",
                "caption": "Or visit the link in our bio.",
                "destination_url": "https://example.com/newsletter",
                "cta_text": "Scan to sign up",
                "short_url": "example.com/news",
                "variant": "full-conversion"
            }
        },
        "scatter_plot": {
            "description": "Scatter plot showing correlation between two variables with data points",
            "required_params": ["title", "data"],
            "optional_params": ["x_label", "y_label", "variant", "background_image", "image_opacity", "padding"],
            "variants": ["default"],
            "default_variant": "default",
            "layout_family": "data-viz",
            "best_for": ["data", "correlation", "analysis"]
        },
        "gauge": {
            "description": "Single-value gauge showing a percentage or 0-100 metric",
            "required_params": ["value", "label"],
            "optional_params": ["title", "variant", "background_image", "image_opacity", "padding"],
            "variants": ["default"],
            "default_variant": "default",
            "layout_family": "data-viz",
            "best_for": ["metrics", "progress", "health-check"]
        },
        "radar_chart": {
            "description": "Radar/spider chart comparing multiple dimensions (e.g. skill assessment)",
            "required_params": ["title", "data"],
            "optional_params": ["variant", "background_image", "image_opacity", "padding"],
            "variants": ["default"],
            "default_variant": "default",
            "layout_family": "data-viz",
            "best_for": ["comparison", "assessment", "multi-dimension"]
        },
        "progress_rings": {
            "description": "Multiple circular progress rings showing completion across workstreams",
            "required_params": ["title", "items"],
            "optional_params": ["description", "variant", "background_image", "image_opacity", "padding"],
            "variants": ["default"],
            "default_variant": "default",
            "layout_family": "data-viz",
            "best_for": ["progress", "project-tracking", "multi-metric"]
        },
        "comparison_bars": {
            "description": "Side-by-side horizontal bars comparing two entities on a single metric",
            "required_params": ["title", "comparison"],
            "optional_params": ["description", "variant", "background_image", "image_opacity", "padding"],
            "variants": ["default"],
            "default_variant": "default",
            "layout_family": "data-viz",
            "best_for": ["comparison", "versus", "benchmark"]
        },
        "metric_grid": {
            "description": "2x2 grid of key metrics with values, labels, and trends",
            "required_params": ["title", "metrics"],
            "optional_params": ["variant", "background_image", "image_opacity", "padding"],
            "variants": ["default"],
            "default_variant": "default",
            "layout_family": "data-viz",
            "best_for": ["metrics", "kpi", "dashboard"]
        },
        "funnel_chart": {
            "description": "Funnel chart showing conversion stages with decreasing values",
            "required_params": ["title", "steps"],
            "optional_params": ["variant", "background_image", "image_opacity", "padding"],
            "variants": ["default"],
            "default_variant": "default",
            "layout_family": "data-viz",
            "best_for": ["conversion", "pipeline", "stages"]
        },
        "table": {
            "description": "Data table with headers and rows for structured information",
            "required_params": ["headers", "rows"],
            "optional_params": ["title", "caption", "variant", "background_image", "image_opacity", "padding"],
            "variants": ["default"],
            "default_variant": "default",
            "layout_family": "data-viz",
            "best_for": ["data", "comparison", "structured"]
        },
        "metric_sparkline": {
            "description": "Single metric with sparkline trend line and context text",
            "required_params": ["value", "label", "data"],
            "optional_params": ["trend", "context", "variant", "background_image", "image_opacity", "padding"],
            "variants": ["default"],
            "default_variant": "default",
            "layout_family": "data-viz",
            "best_for": ["metrics", "trend", "timeseries"]
        },
        "column_chart": {
            "description": "Vertical bar chart with labels and values for categorical comparison. Supports both single-series (flat [{label, value}]) and multi-series grouped columns (nested [{label, series: [{name, value}]}]).",
            "required_params": ["title", "data"],
            "optional_params": ["caption", "variant", "background_image", "image_opacity", "padding"],
            "variants": ["default"],
            "default_variant": "default",
            "layout_family": "data-viz",
            "best_for": ["comparison", "ranking", "distribution", "grouped-data"],
            "example": {
                "title": "Workforce Composition",
                "data": [
                    {
                        "label": "1970",
                        "series": [
                            {"name": "Men", "value": 58},
                            {"name": "Women", "value": 42}
                        ]
                    },
                    {
                        "label": "1980",
                        "series": [
                            {"name": "Men", "value": 53},
                            {"name": "Women", "value": 47}
                        ]
                    }
                ]
            }
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
///
/// The returned JSON always exposes a `usage_hint` string that summarizes
/// expected param types — e.g. "required: headline (string), subheadline (string)".
/// This makes the schema discoverable from a single call without requiring the
/// caller to read the validator source.
pub fn get_slide_type_info(slide_type: &str) -> Option<Value> {
    let registry = get_registry();
    let mut entry = registry
        .as_object()
        .and_then(|m: &serde_json::Map<String, Value>| m.get(slide_type))
        .cloned()?;

    let required = entry
        .get("required_params")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let optional = entry
        .get("optional_params")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let hint = build_usage_hint(slide_type, &required, &optional);

    if let Some(obj) = entry.as_object_mut() {
        obj.entry("usage_hint".to_string())
            .or_insert(Value::String(hint.clone()));
    }

    Some(entry)
}

/// Heuristic mapping from param name to inferred JSON type, for usage hints
/// only. The hard validator (`validate::validate_slide_spec`) remains the
/// source of truth — this is purely for LLM discoverability.
fn param_type_hint(slide_type: &str, name: &str) -> &'static str {
    // qr_destination aliases
    if slide_type == "qr_destination" {
        if name == "destination_url" {
            return "string (URL) — alias: url";
        }
        if name == "cta_text" {
            return "string — alias: button_text";
        }
    }

    let array_like = [
        "items", "features", "cards", "rows", "columns", "stats", "steps",
        "tags", "data", "values", "labels", "metrics", "options", "entries",
        "questions", "answers", "rows_data",
    ];
    let object_like = ["qr_payload"];
    let bool_like = [
        "show_progress", "show_numbers", "show_dates", "show_icons",
        "show_checkmarks", "show_divider", "show_cta", "ordered",
        "is_myth", "is_fact", "show_caption",
    ];
    let color_like = [
        "background_color", "text_color", "border_color", "icon_color",
        "color", "primary_color", "accent_color",
    ];
    let url_like = [
        "image_url", "avatar_url", "logo_url", "background_image",
        "button_url", "url", "destination_url",
    ];
    let number_like = [
        "rating", "count", "index", "max_width", "max_height",
    ];

    if array_like.contains(&name) {
        "array"
    } else if object_like.contains(&name) {
        "object"
    } else if bool_like.contains(&name) {
        "bool"
    } else if color_like.contains(&name) {
        "string (hex color, e.g. \"#FF5500\")"
    } else if url_like.contains(&name) {
        "string (URL)"
    } else if number_like.contains(&name) {
        "number"
    } else {
        "string"
    }
}

fn build_usage_hint(slide_type: &str, required: &[String], optional: &[String]) -> String {
    let req_strs: Vec<String> = required
        .iter()
        .map(|p| format!("{} ({})", p, param_type_hint(slide_type, p)))
        .collect();
    let opt_strs: Vec<String> = optional
        .iter()
        .map(|p| format!("{} ({})", p, param_type_hint(slide_type, p)))
        .collect();

    let mut hint = format!(
        "Call generate_slide with slide_type=\"{}\". Required: {}. Optional: {}.",
        slide_type,
        if req_strs.is_empty() {
            "none".to_string()
        } else {
            req_strs.join(", ")
        },
        if opt_strs.is_empty() {
            "none".to_string()
        } else {
            opt_strs.join(", ")
        }
    );

    // Append a concrete parameter example if the registry defines one.
    let registry = get_registry();
    if let Some(entry) = registry
        .as_object()
        .and_then(|m: &serde_json::Map<String, Value>| m.get(slide_type))
    {
        if let Some(example) = entry.get("example").and_then(|v| v.as_object()) {
            let example_json = serde_json::to_string(example).unwrap_or_default();
            hint.push_str(&format!(" Example params: {}.", example_json));
        }
    }

    hint
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

    let matched = map
        .iter()
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
        let preferred = vec![
            "qr_destination".to_string(),
            "cta".to_string(),
            "pricing_plan".to_string(),
        ];
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
        assert!(
            info["best_for"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v == "conversion")
        );
        assert!(
            info["variants"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v == "full-conversion")
        );
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

    #[test]
    fn test_qr_destination_exposes_varied_styles() {
        let info = get_slide_type_info("qr_destination").expect("qr destination should exist");
        let variants = info["variants"].as_array().expect("variants array");
        for variant in ["split-card", "poster", "stacked-badge", "compact"] {
            assert!(
                variants.iter().any(|v| v.as_str() == Some(variant)),
                "missing QR variant {variant}"
            );
        }
    }

    #[test]
    fn test_get_slide_type_info_includes_usage_hint() {
        // Every well-defined slide type must return a usage_hint string that
        // names its required and optional params. This is what makes the
        // schema discoverable in a single tool call.
        for slide_type in ["hero", "feature", "list", "quote", "cta", "qr_destination"] {
            let info = get_slide_type_info(slide_type)
                .unwrap_or_else(|| panic!("type {slide_type} missing"));
            let hint = info["usage_hint"]
                .as_str()
                .unwrap_or_else(|| panic!("type {slide_type} missing usage_hint"));
            assert!(
                hint.contains("generate_slide"),
                "hint for {slide_type} lacks tool call: {hint}"
            );
            assert!(
                hint.to_lowercase().contains("required"),
                "hint for {slide_type} lacks required section: {hint}"
            );
            assert!(
                hint.to_lowercase().contains("optional"),
                "hint for {slide_type} lacks optional section: {hint}"
            );
        }
    }

    #[test]
    fn test_qr_destination_hint_includes_aliases_and_example() {
        // The QR aliases (destination_url/url, cta_text/button_text) were a
        // frequent agent mistake; the hint must surface them explicitly.
        let info = get_slide_type_info("qr_destination").expect("present");
        let hint = info["usage_hint"].as_str().expect("str");
        assert!(hint.contains("alias: url"), "destination_url alias missing: {hint}");
        assert!(hint.contains("alias: button_text"), "cta_text alias missing: {hint}");
        // And the example field must be exposed too.
        let example = info["example"].as_object().expect("example object");
        assert!(example.contains_key("destination_url"));
        assert!(example.contains_key("cta_text"));
    }

    #[test]
    fn test_qr_destination_usage_hint_contains_array_param_hint() {
        // stat_row's `stats` is an array — confirm the hint calls that out
        // so agents pass an array, not a string.
        let info = get_slide_type_info("stat_row").expect("present");
        let hint = info["usage_hint"].as_str().expect("str");
        assert!(hint.contains("(array)"), "stats param should be hinted as array: {hint}");
    }

    #[test]
    fn test_column_chart_registry_documents_multi_series() {
        // The column_chart slide type must document multi-series support
        // in both its description and its example.
        let info = get_slide_type_info("column_chart").expect("column_chart exists");
        let desc = info["description"].as_str().expect("description");
        assert!(
            desc.contains("multi-series") || desc.contains("grouped"),
            "column_chart description should mention multi-series support: {desc}"
        );
        // Example must show nested series format
        let example = info.get("example").and_then(|v| v.as_object());
        assert!(example.is_some(), "column_chart must have an example");
        let data = example
            .unwrap()
            .get("data")
            .and_then(|v| v.as_array())
            .expect("data array");
        // First item should contain a "series" key
        let first = data.first().expect("non-empty data");
        let series = first.get("series").and_then(|v| v.as_array());
        assert!(series.is_some(), "example data items must contain series array");
        let series_arr = series.unwrap();
        assert!(
            series_arr.len() >= 2,
            "example should have at least 2 series entries"
        );
        assert!(
            series_arr[0].get("name").is_some(),
            "each series entry must have a name"
        );
        assert!(
            series_arr[0].get("value").is_some(),
            "each series entry must have a value"
        );
    }

    #[test]
    fn test_column_chart_accepts_flat_and_grouped_data_shapes() {
        // Validate that the registry's data param is typed as "array" and
        // that both flat and nested series structures are documented.
        let info = get_slide_type_info("column_chart").expect("present");
        let hint = info["usage_hint"].as_str().expect("str");
        // The data param should be hinted as array
        assert!(
            hint.contains("(array)"),
            "data param should be hinted as array: {hint}"
        );
        // The description must reference both single and multi-series
        let desc = info["description"].as_str().unwrap();
        assert!(
            desc.contains("flat") || desc.contains("single-series") || desc.contains("[{label"),
            "description should document flat/single-series format: {desc}"
        );
    }
}
