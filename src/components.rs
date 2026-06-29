// components.rs — Rust port of carousel-mcp slide generators.
//
// Each public function builds slide HTML using the layout/block/effects
// helpers and returns a serde_json::Value with keys:
//   html        — the rendered slide HTML string
//   background  — the bg_style passed in
//   variant     — the effective variant resolved inside the function
//   theme       — the theme passed in

#[allow(unused_imports)]
use serde_json::{json, Value};
#[allow(unused_imports)]
use crate::design_system::DesignTokens;
#[allow(unused_imports)]
use crate::blocks::{
    escape_html, gradient_text, text_block, heading_block, icon_block, stat_block,
    quote_block, badge_block, button_block, attribution_block, list_item_block,
    dot_marker, divider_block,
};
#[allow(unused_imports)]
use crate::layouts::{
    get_slide_colors, is_dark_bg, slide_base, hero_layout, centered_layout,
    stack_layout, split_layout, grid_layout,
};
#[allow(unused_imports)]
use crate::effects::glass_surface;

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Returns (open_tag, close_tag) for a glass container on dark slides.
fn get_glass_container(tokens: &DesignTokens, is_dark: bool) -> (String, String) {
    if is_dark {
        let radius = tokens.radii.get("md").cloned().unwrap_or_else(|| "10px".to_string());
        (
            format!(
                r#"<div style="background:rgba(255,255,255,0.04);border:1px solid rgba(255,255,255,0.08);backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px);border-radius:{};padding:32px;">"#,
                radius
            ),
            "</div>".to_string(),
        )
    } else {
        (String::new(), String::new())
    }
}

/// Card styling tuple: (card_bg, card_border, card_blur) for dark/light contexts.
fn card_styles(tokens: &DesignTokens, is_dark: bool) -> (String, String, String) {
    if is_dark {
        (
            "rgba(255,255,255,0.04)".to_string(),
            "1px solid rgba(255,255,255,0.08)".to_string(),
            "backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px);".to_string(),
        )
    } else {
        (
            tokens.surface_light.clone(),
            format!("1px solid {}25", tokens.border_light),
            String::new(),
        )
    }
}

/// Inject a background image into slide HTML, mirroring Python's `_inject_background_image`.
///
/// Inserts an absolutely-positioned `<div>` with the image URL and a contrast scrim
/// inside the first `<div style="position:relative;width:100%;height:100%;…>` found in the HTML.
/// If `image_url` is empty the original HTML is returned unchanged.
fn inject_background_image(html: String, image_url: &str, opacity: f32, is_dark: bool) -> String {
    if image_url.is_empty() {
        return html;
    }
    // Calibrate opacity to preserve text contrast
    let bg_opacity = if is_dark {
        opacity.max(0.25).min(0.55)
    } else {
        opacity.max(0.06).min(0.18)
    };
    // Minimal contrast scrim
    let overlay_html = if is_dark {
        r#"<div style="position:absolute;inset:0;background:rgba(0,0,0,0.30);z-index:1;"></div>"#
    } else {
        r#"<div style="position:absolute;inset:0;background:rgba(255,255,255,0.30);z-index:1;"></div>"#
    };
    let image_div = format!(
        r#"<div style="position:absolute;inset:0;background-image:url('{}');background-size:cover;background-position:center;opacity:{:.2};z-index:0;"></div>{}"#,
        image_url, bg_opacity, overlay_html
    );
    // Find the slide's root positioned container and inject after its opening tag
    if let Some(pos) = html.find("position:relative;width:100%;height:100%;") {
        // Walk back to find the '<' of this opening tag
        if let Some(tag_start) = html[..pos].rfind('<') {
            // Find the closing '>' of this tag
            if let Some(tag_end) = html[pos..].find('>') {
                let insert_at = pos + tag_end + 1;
                let mut result = html.clone();
                result.insert_str(insert_at, &format!("\n{}", image_div));
                return result;
            }
        }
    }
    html
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. hero_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Hero hook slide with gradient headline and optional badge.
///
/// Variants:
/// - `left-aligned` (default): content on left side.
/// - `centered`: fully centred layout.
/// - `split`: two-column grid, headline left, right empty (visual area).
pub fn hero_slide(
    tokens: &DesignTokens,
    headline: &str,
    subheadline: &str,
    badge: &str,
    bg_style: &str,
    decorations: bool,
    variant: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let badge_html = if !badge.is_empty() {
        badge_block(badge, tokens, Some(&colors.primary), "0 0 16px")
    } else {
        String::new()
    };

    let gradient_colors = if is_dark {
        ("#FFFFFF", colors.text_primary.as_str())
    } else {
        (colors.text_primary.as_str(), colors.text_secondary.as_str())
    };

    let (gc, gx) = get_glass_container(tokens, is_dark);

    let effective_variant = variant;

    let html = if effective_variant == "split" {
        let headline_html = heading_block(
            headline, tokens, "display",
            None, true,
            Some((gradient_colors.0, gradient_colors.1)),
            "left", "0", true,
        );
        let sub_html = if !subheadline.is_empty() {
            text_block(subheadline, tokens, "body", Some(&colors.text_secondary), false, None, "left", None, "8px 0 0")
        } else {
            String::new()
        };
        let left_content = format!("{}{}{}{}{}", gc, badge_html, headline_html, sub_html, gx);
        split_layout(&left_content, "", tokens, bg_style, "0", "1fr 1fr", true)
    } else {
        let align = if effective_variant == "centered" { "center" } else { "left" };
        let headline_html = heading_block(
            headline, tokens, "display",
            None, true,
            Some((gradient_colors.0, gradient_colors.1)),
            align, "0", true,
        );
        let sub_html = if !subheadline.is_empty() {
            text_block(subheadline, tokens, "body", Some(&colors.text_secondary), false, None, align, None, "8px 0 0")
        } else {
            String::new()
        };
        let content = if align == "center" {
            format!(
                r#"{}{}<div style="text-align:center">{}</div>{}{}
"#,
                gc, badge_html, headline_html, sub_html, gx
            )
        } else {
            format!("{}{}{}{}{}", gc, badge_html, headline_html, sub_html, gx)
        };
        hero_layout(&content, tokens, bg_style, decorations, align)
    };

    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);

    json!({
        "html": html,
        "background": bg_style,
        "variant": effective_variant,
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. feature_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Single-feature slide with icon, title, description, and optional stat number.
///
/// Variants: `stacked` (default) | `icon-left` | `icon-right` | `minimal`
pub fn feature_slide(
    tokens: &DesignTokens,
    icon: &str,
    title: &str,
    description: &str,
    number: &str,
    bg_style: &str,
    variant: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let (gc, gx) = get_glass_container(tokens, is_dark);

    let stat_prefix = if !number.is_empty() {
        format!(
            r#"<span style="color:{};font-weight:800;margin-right:12px;font-family:{};">{}</span>"#,
            colors.primary, tokens.heading_font, escape_html(number)
        )
    } else {
        String::new()
    };
    let full_title = format!("{}{}", stat_prefix, escape_html(title));

    let effective_variant = variant;
    let padding = "80px 52px 80px";
    let justify = "center";

    let html = match effective_variant {
        "icon-left" => {
            let icon_html = icon_block(icon, tokens, Some(&colors.primary), "56px", "28px", "0");
            let title_html = heading_block(&full_title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 8px", false);
            let desc_html = text_block(description, tokens, "body", Some(&colors.text_secondary), false, None, "left", None, "0");
            let content = format!(
                r#"{}<div style="display:flex;align-items:center;gap:24px;"><div style="flex-shrink:0;">{}</div><div>{}{}</div></div>{}"#,
                gc, icon_html, title_html, desc_html, gx
            );
            slide_base(&content, tokens, bg_style, false, padding, justify)
        }
        "icon-right" => {
            let icon_html = icon_block(icon, tokens, Some(&colors.primary), "56px", "28px", "0");
            let title_html = heading_block(&full_title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 8px", false);
            let desc_html = text_block(description, tokens, "body", Some(&colors.text_secondary), false, None, "left", None, "0");
            let content = format!(
                r#"{}<div style="display:flex;align-items:center;gap:24px;"><div>{}{}</div><div style="flex-shrink:0;">{}</div></div>{}"#,
                gc, title_html, desc_html, icon_html, gx
            );
            slide_base(&content, tokens, bg_style, false, padding, justify)
        }
        "minimal" => {
            let title_html = heading_block(&full_title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 8px", false);
            let desc_html = text_block(description, tokens, "body", Some(&colors.text_secondary), false, None, "left", None, "0");
            let content = format!("{}{}{}{}", gc, title_html, desc_html, gx);
            centered_layout(&content, tokens, bg_style, false, padding, justify)
        }
        _ => {
            // stacked (default)
            let icon_html = icon_block(icon, tokens, Some(&colors.primary), "56px", "28px", "0 0 16px");
            let title_html = heading_block(&full_title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 8px", false);
            let desc_html = text_block(description, tokens, "body", Some(&colors.text_secondary), false, None, "left", None, "0");
            let inner = format!("{}{}{}", icon_html, title_html, desc_html);
            let content = format!("{}{}{}", gc, inner, gx);
            stack_layout(&content, tokens, bg_style, "0", false, padding, justify)
        }
    };

    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);

    json!({
        "html": html,
        "background": bg_style,
        "variant": effective_variant,
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. list_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Bulleted or numbered list slide.
///
/// Variants: `bulleted` (default) | `numbered` | `card` | `grid`
///
/// Each item in `items` is a JSON object with optional keys `label`/`title` and `sub`/`description`.
pub fn list_slide(
    tokens: &DesignTokens,
    title: &str,
    items: Vec<Value>,
    bg_style: &str,
    numbered: bool,
    variant: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let heading = heading_block(title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 12px", true);

    let (card_bg, card_border, card_blur) = card_styles(tokens, is_dark);
    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    let radius_md = tokens.radii.get("md").cloned().unwrap_or_else(|| "10px".to_string());
    let shadow_sm = tokens.shadows.get("sm").cloned().unwrap_or_else(|| "none".to_string());

    let effective_variant = variant;

    let content = match effective_variant {
        "card" => {
            let mut rows = String::new();
            for (i, item) in items.iter().enumerate() {
                let label = item.get("label").or_else(|| item.get("title"))
                    .and_then(|v| v.as_str()).unwrap_or("");
                let sub = item.get("sub").or_else(|| item.get("description"))
                    .and_then(|v| v.as_str()).unwrap_or("");
                let marker = if numbered {
                    format!(
                        r#"<span style="color:{};font-weight:700;margin-right:8px;font-size:13px;">{}</span>"#,
                        tokens.primary, i + 1
                    )
                } else {
                    String::new()
                };
                let sub_html = if !sub.is_empty() {
                    format!(
                        r#"<div style="font-size:{}px;color:{};margin-top:4px;">{}</div>"#,
                        caption_fs, colors.text_secondary, escape_html(sub)
                    )
                } else {
                    String::new()
                };
                rows.push_str(&format!(
                    r#"<div style="background:{};border:{};{}border-radius:{};padding:14px 16px;margin-bottom:10px;box-shadow:{};">
                        <div style="display:flex;align-items:flex-start;">{}<div>
                            <div style="font-family:{};font-size:{}px;font-weight:500;color:{};">{}</div>
                            {}
                        </div></div>
                    </div>"#,
                    card_bg, card_border, card_blur, radius_md, shadow_sm,
                    marker,
                    tokens.body_font, body_fs, colors.text_primary, escape_html(label),
                    sub_html
                ));
            }
            format!("{}<div style=\"margin-top:16px;\">{}</div>", heading, rows)
        }
        "grid" => {
            let mut rows = String::new();
            for (i, item) in items.iter().enumerate() {
                let label = item.get("label").or_else(|| item.get("title"))
                    .and_then(|v| v.as_str()).unwrap_or("");
                let sub = item.get("sub").or_else(|| item.get("description"))
                    .and_then(|v| v.as_str()).unwrap_or("");
                let marker = if numbered {
                    format!(
                        r#"<span style="color:{};font-weight:700;margin-right:6px;font-size:13px;">{}</span>"#,
                        tokens.primary, i + 1
                    )
                } else {
                    String::new()
                };
                let sub_html = if !sub.is_empty() {
                    format!(
                        r#"<div style="font-size:{}px;color:{};margin-top:2px;">{}</div>"#,
                        caption_fs, colors.text_secondary, escape_html(sub)
                    )
                } else {
                    String::new()
                };
                rows.push_str(&format!(
                    r#"<div style="width:calc(50% - 8px);margin-bottom:10px;">
                        <div style="display:flex;align-items:flex-start;">{}<div>
                            <div style="font-family:{};font-size:{}px;font-weight:500;color:{};">{}</div>
                            {}
                        </div></div>
                    </div>"#,
                    marker,
                    tokens.body_font, body_fs, colors.text_primary, escape_html(label),
                    sub_html
                ));
            }
            format!(
                r#"{}<div style="display:flex;flex-wrap:wrap;gap:16px;margin-top:16px;">{}</div>"#,
                heading, rows
            )
        }
        _ => {
            // bulleted or numbered
            let is_numbered = numbered || effective_variant == "numbered";
            let mut rows = String::new();
            for (i, item) in items.iter().enumerate() {
                let label = item.get("label").or_else(|| item.get("title"))
                    .and_then(|v| v.as_str()).unwrap_or("");
                let sub = item.get("sub").or_else(|| item.get("description"))
                    .and_then(|v| v.as_str()).unwrap_or("");
                let marker = if is_numbered {
                    format!(
                        r#"<span style="color:{};font-weight:700;margin-right:12px;font-size:14px;">{}</span>"#,
                        tokens.primary, i + 1
                    )
                } else {
                    let bullet_char = if matches!(theme, "editorial" | "natural" | "vibrant") { "✦" } else { "▪" };
                    format!(
                        r#"<span style="color:{};margin-right:12px;font-size:12px;line-height:1.5;">{}</span>"#,
                        tokens.primary, bullet_char
                    )
                };
                let sub_html = if !sub.is_empty() {
                    format!(
                        r#"<div style="font-size:{}px;color:{};margin-top:4px;">{}</div>"#,
                        caption_fs, colors.text_secondary, escape_html(sub)
                    )
                } else {
                    String::new()
                };
                rows.push_str(&format!(
                    r#"<div style="display:flex;align-items:flex-start;margin-bottom:12px;">
                        {}
                        <div>
                            <div style="font-family:{};font-size:{}px;font-weight:500;color:{};">{}</div>
                            {}
                        </div>
                    </div>"#,
                    marker,
                    tokens.body_font, body_fs, colors.text_primary, escape_html(label),
                    sub_html
                ));
            }
            format!("{}<div style=\"margin-top:16px;\">{}</div>", heading, rows)
        }
    };

    let html = slide_base(&content, tokens, bg_style, false, "80px 52px 80px", "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": effective_variant,
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. quote_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Testimonial/quote slide with attribution.
///
/// Variants: `centered` (default) | `left-accent` | `attribution-below`
pub fn quote_slide(
    tokens: &DesignTokens,
    quote: &str,
    author: &str,
    role: &str,
    bg_style: &str,
    variant: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    // Dynamic font size based on quote length
    let quote_font_size = if quote.len() < 60 {
        format!("{}px", tokens.type_scale.get("display").unwrap().font_size)
    } else if quote.len() < 120 {
        format!("{}px", tokens.type_scale.get("headline").unwrap().font_size)
    } else {
        format!("{}px", tokens.type_scale.get("title").unwrap().font_size)
    };

    let headline_fw = tokens.type_scale.get("headline").unwrap().font_weight;
    let glass_variant = if is_dark { "dark" } else { "light" };
    let radius_md = tokens.radii.get("md").cloned().unwrap_or_else(|| "10px".to_string());
    let g_styles = glass_surface(tokens, glass_variant, &radius_md);
    let shadow_lg = tokens.shadows.get("lg").cloned().unwrap_or_else(|| "none".to_string());

    let glass_styles_str = g_styles.iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("; ");

    let glass_open = format!(
        r#"<div style="{};padding:32px;box-shadow:{};">"#,
        glass_styles_str, shadow_lg
    );
    let glass_close = "</div>";

    let effective_variant = variant;

    let html = match effective_variant {
        "left-accent" => {
            let q = format!(
                r#"<blockquote style="font-family:{};font-size:{};font-weight:{};line-height:1.2;color:{};margin:0;max-width:100%;border-left:4px solid {};padding-left:24px;text-wrap:balance;">{}</blockquote>"#,
                tokens.heading_font, quote_font_size, headline_fw, colors.text_primary, tokens.primary, escape_html(quote)
            );
            let attr = if !author.is_empty() {
                attribution_block(author, role, tokens, Some(&colors.text_primary), "20px 0 0", "left")
            } else {
                String::new()
            };
            let content = format!("{}{}{}{}", glass_open, q, attr, glass_close);
            slide_base(&content, tokens, bg_style, false, "80px 44px 80px", "center")
        }
        "attribution-below" => {
            let q = format!(
                r#"<blockquote style="font-family:{};font-size:{};font-weight:{};line-height:1.2;color:{};margin:0;text-align:center;text-wrap:balance;">{}</blockquote>"#,
                tokens.heading_font, quote_font_size, headline_fw, colors.text_primary, escape_html(quote)
            );
            let attr = if !author.is_empty() {
                attribution_block(author, role, tokens, Some(&colors.text_primary), "0", "center")
            } else {
                String::new()
            };
            let content = format!(
                r#"{}{}<div style="margin-top:32px;text-align:center;">{}</div>{}"#,
                glass_open, q, attr, glass_close
            );
            slide_base(&content, tokens, bg_style, false, "80px 44px 80px", "center")
        }
        _ => {
            // centered (default)
            let q = format!(
                r#"<blockquote style="font-family:{};font-size:{};font-weight:{};line-height:1.2;color:{};margin:0;text-align:center;text-wrap:balance;">{}</blockquote>"#,
                tokens.heading_font, quote_font_size, headline_fw, colors.text_primary, escape_html(quote)
            );
            let attr = if !author.is_empty() {
                attribution_block(author, role, tokens, Some(&colors.text_primary), "0", "center")
            } else {
                String::new()
            };
            let content = format!(
                r#"{}{}<div style="margin-top:32px;text-align:center;">{}</div>{}"#,
                glass_open, q, attr, glass_close
            );
            slide_base(&content, tokens, bg_style, false, "80px 44px 80px", "center")
        }
    };

    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);

    json!({
        "html": html,
        "background": bg_style,
        "variant": effective_variant,
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. cta_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Call-to-action slide with gradient headline and button.
///
/// Variants: `centered` (default) | `left` | `right` | `minimal`
pub fn cta_slide(
    tokens: &DesignTokens,
    headline: &str,
    button_text: &str,
    button_url: &str,
    subtext: &str,
    bg_style: &str,
    variant: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let (gc, gx) = get_glass_container(tokens, is_dark);

    let gradient_colors = if is_dark {
        ("#FFFFFF", colors.text_primary.as_str())
    } else {
        (colors.text_primary.as_str(), colors.text_secondary.as_str())
    };

    let effective_variant = variant;

    let html = if effective_variant == "minimal" {
        let headline_html = heading_block(headline, tokens, "headline", Some(&colors.text_primary), false, None, "center", "0 0 12px", true);
        let sub_html = if !subtext.is_empty() {
            text_block(subtext, tokens, "body", Some(&colors.text_secondary), false, None, "center", None, "0 0 20px")
        } else {
            String::new()
        };
        let btn = button_block(button_text, button_url, Some(tokens), Some(&colors.button_bg), Some(&colors.button_text), "0");
        let content = format!(
            r#"{}<div style="text-align:center">{}{}{}</div>{}"#,
            gc, headline_html, sub_html, btn, gx
        );
        centered_layout(&content, tokens, bg_style, false, "80px 64px 80px", "center")
    } else {
        let align = match effective_variant {
            "left" => "left",
            "right" => "right",
            _ => "center",
        };
        let headline_html = heading_block(
            headline, tokens, "display",
            None, true, Some((gradient_colors.0, gradient_colors.1)),
            align, "0 0 12px", true,
        );
        let sub_html = if !subtext.is_empty() {
            text_block(subtext, tokens, "body", Some(&colors.text_secondary), false, None, align, None, "0 0 20px")
        } else {
            String::new()
        };
        let btn = button_block(button_text, button_url, Some(tokens), Some(&colors.button_bg), Some(&colors.button_text), "0");
        let content = format!(
            r#"{}<div style="text-align:{}">{}{}{}</div>{}"#,
            gc, align, headline_html, sub_html, btn, gx
        );
        hero_layout(&content, tokens, bg_style, true, "center")
    };

    let html = inject_background_image(html, background_image, image_opacity, is_dark);

    json!({
        "html": html,
        "background": bg_style,
        "variant": effective_variant,
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. comparison_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Side-by-side comparison grid.
///
/// Variants: `default` (horizontal CSS grid) | `stacked` | `horizontal` (chips)
pub fn comparison_slide(
    tokens: &DesignTokens,
    title: &str,
    left_label: &str,
    left_items: Vec<String>,
    right_label: &str,
    right_items: Vec<String>,
    bg_style: &str,
    variant: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let heading = heading_block(title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 12px", true);

    let (gc, gx) = get_glass_container(tokens, is_dark);

    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let title_fs = tokens.type_scale.get("title").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    let radius_sm = tokens.radii.get("sm").cloned().unwrap_or_else(|| "6px".to_string());

    let effective_variant = variant;

    let content = match effective_variant {
        "stacked" => {
            let mut left_rows = String::new();
            for item in &left_items {
                left_rows.push_str(&format!(
                    r#"<div style="font-family:{};font-size:{}px;color:{};padding:6px 0;">{}</div>"#,
                    tokens.body_font, body_fs, colors.text_primary, escape_html(item)
                ));
            }
            let mut right_rows = String::new();
            for item in &right_items {
                right_rows.push_str(&format!(
                    r#"<div style="font-family:{};font-size:{}px;color:{};padding:6px 0;">{}</div>"#,
                    tokens.body_font, body_fs, colors.text_primary, escape_html(item)
                ));
            }
            let left_html = format!(
                r#"<div style="margin-bottom:16px;"><div style="font-family:{};font-size:{}px;font-weight:700;color:{};margin-bottom:8px;">{}</div>{}</div>"#,
                tokens.heading_font, title_fs, tokens.primary, escape_html(left_label), left_rows
            );
            let right_html = format!(
                r#"<div style="margin-bottom:16px;"><div style="font-family:{};font-size:{}px;font-weight:700;color:{};margin-bottom:8px;">{}</div>{}</div>"#,
                tokens.heading_font, title_fs, tokens.accent, escape_html(right_label), right_rows
            );
            format!("{}{}{}{}{}", gc, heading, left_html, right_html, gx)
        }
        "horizontal" => {
            let left_chips: String = left_items.iter().map(|item| format!(
                r#"<span style="font-family:{};font-size:{}px;color:{};padding:4px 10px;background:{};border-radius:{};">{}</span>"#,
                tokens.body_font, body_fs, colors.text_primary, tokens.surface_light, radius_sm, escape_html(item)
            )).collect();
            let right_chips: String = right_items.iter().map(|item| format!(
                r#"<span style="font-family:{};font-size:{}px;color:{};padding:4px 10px;background:{};border-radius:{};">{}</span>"#,
                tokens.body_font, body_fs, colors.text_primary, tokens.surface_light, radius_sm, escape_html(item)
            )).collect();
            let left_html = format!(
                r#"<div style="margin-bottom:12px;"><div style="font-family:{};font-size:{}px;font-weight:600;color:{};margin-bottom:6px;text-transform:uppercase;letter-spacing:0.05em;">{}</div><div style="display:flex;flex-wrap:wrap;gap:8px;">{}</div></div>"#,
                tokens.heading_font, caption_fs, tokens.primary, escape_html(left_label), left_chips
            );
            let right_html = format!(
                r#"<div style="margin-bottom:12px;"><div style="font-family:{};font-size:{}px;font-weight:600;color:{};margin-bottom:6px;text-transform:uppercase;letter-spacing:0.05em;">{}</div><div style="display:flex;flex-wrap:wrap;gap:8px;">{}</div></div>"#,
                tokens.heading_font, caption_fs, tokens.accent, escape_html(right_label), right_chips
            );
            format!(
                r#"{}{}<div style="margin-top:16px;">{}{}</div>{}"#,
                gc, heading, left_html, right_html, gx
            )
        }
        _ => {
            // default — CSS grid side-by-side with row alignment
            let max_len = left_items.len().max(right_items.len());
            let mut grid_rows = String::new();
            for i in 0..max_len {
                let l_item = left_items.get(i).map(|s| s.as_str()).unwrap_or("");
                let r_item = right_items.get(i).map(|s| s.as_str()).unwrap_or("");
                grid_rows.push_str(&format!(
                    r#"<div style="font-family:{};font-size:{}px;color:{};padding:10px 0;border-bottom:1px solid {}20;display:flex;align-items:center;min-height:38px;box-sizing:border-box;">{}</div>
                       <div style="font-family:{};font-size:{}px;color:{};padding:10px 0;border-bottom:1px solid {}20;display:flex;align-items:center;min-height:38px;box-sizing:border-box;">{}</div>"#,
                    tokens.body_font, body_fs, colors.text_primary, tokens.border_light, escape_html(l_item),
                    tokens.body_font, body_fs, colors.text_primary, tokens.border_light, escape_html(r_item),
                ));
            }
            format!(
                r#"{}{}<div style="display:grid;grid-template-columns:1fr 1fr;gap:0 32px;margin-top:16px;width:100%;box-sizing:border-box;align-items:stretch;">
                    <div style="font-family:{};font-size:{}px;font-weight:700;color:{};padding-bottom:10px;border-bottom:2px solid {};">{}</div>
                    <div style="font-family:{};font-size:{}px;font-weight:700;color:{};padding-bottom:10px;border-bottom:2px solid {};">{}</div>
                    {}
                </div>{}"#,
                gc, heading,
                tokens.heading_font, title_fs, tokens.primary, tokens.primary, escape_html(left_label),
                tokens.heading_font, title_fs, tokens.accent, tokens.accent, escape_html(right_label),
                grid_rows,
                gx
            )
        }
    };

    let html = slide_base(&content, tokens, bg_style, false, "80px 52px 80px", "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": effective_variant,
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. stat_row_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Grid of key statistics.
///
/// Variants: `auto` (default) | `compact` | `expanded` | `horizontal`
///
/// Each stat in `stats` is a JSON object with optional keys `value`, `label`, `sub`.
pub fn stat_row_slide(
    tokens: &DesignTokens,
    title: &str,
    stats: Vec<Value>,
    bg_style: &str,
    variant: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let heading = heading_block(title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 12px", true);

    let (card_bg, card_border, card_blur) = card_styles(tokens, is_dark);

    let micro_fs = tokens.type_scale.get("micro").map(|s| s.font_size).unwrap_or(10);
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    let title_fs = tokens.type_scale.get("title").unwrap().font_size;
    let headline_fs = tokens.type_scale.get("headline").unwrap().font_size;
    let display_fs = tokens.type_scale.get("display").unwrap().font_size;
    let radius_md = tokens.radii.get("md").cloned().unwrap_or_else(|| "10px".to_string());
    let radius_sm = tokens.radii.get("sm").cloned().unwrap_or_else(|| "6px".to_string());
    let shadow_sm = tokens.shadows.get("sm").cloned().unwrap_or_else(|| "none".to_string());
    let shadow_md = tokens.shadows.get("md").cloned().unwrap_or_else(|| "none".to_string());

    let label_color = if is_dark { &colors.text_secondary } else { &colors.text_primary };

    let effective_variant = variant;

    let grid = match effective_variant {
        "horizontal" => {
            let mut g = format!(r#"<div style="display:flex;gap:16px;margin-top:16px;">"#);
            for item in &stats {
                let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
                let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                let sub = item.get("sub").and_then(|v| v.as_str()).unwrap_or("");
                let sub_html = if !sub.is_empty() {
                    format!(r#"<div style="font-size:{}px;color:{};margin-top:4px;">{}</div>"#, micro_fs, colors.text_secondary, escape_html(sub))
                } else { String::new() };
                g.push_str(&format!(
                    r#"<div style="flex:1;text-align:center;padding:12px;background:{};border:{};{}border-radius:{};">
                        <div style="font-family:{};font-size:{}px;font-weight:700;color:{};line-height:1;">{}</div>
                        <div style="font-size:{}px;color:{};margin-top:6px;">{}</div>
                        {}
                    </div>"#,
                    card_bg, card_border, card_blur, radius_md,
                    tokens.heading_font, headline_fs, tokens.primary, escape_html(val),
                    caption_fs, colors.text_secondary, escape_html(label),
                    sub_html
                ));
            }
            g.push_str("</div>");
            g
        }
        "compact" => {
            let mut g = format!(r#"<div style="display:flex;gap:8px;margin-top:12px;">"#);
            for item in &stats {
                let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
                let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                let sub = item.get("sub").and_then(|v| v.as_str()).unwrap_or("");
                let sub_html = if !sub.is_empty() {
                    format!(r#"<div style="font-size:{}px;color:{};margin-top:2px;">{}</div>"#, micro_fs, colors.text_secondary, escape_html(sub))
                } else { String::new() };
                g.push_str(&format!(
                    r#"<div style="flex:1;padding:10px 8px;background:{};border:{};{}border-radius:{};text-align:center;">
                        <div style="font-family:{};font-size:{}px;font-weight:700;color:{};line-height:1;">{}</div>
                        <div style="font-size:{}px;color:{};margin-top:4px;">{}</div>
                        {}
                    </div>"#,
                    card_bg, card_border, card_blur, radius_sm,
                    tokens.heading_font, title_fs, tokens.primary, escape_html(val),
                    micro_fs, colors.text_secondary, escape_html(label),
                    sub_html
                ));
            }
            g.push_str("</div>");
            g
        }
        "expanded" => {
            let mut g = format!(r#"<div style="display:flex;gap:20px;margin-top:20px;">"#);
            for item in &stats {
                let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
                let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                let sub = item.get("sub").and_then(|v| v.as_str()).unwrap_or("");
                let sub_html = if !sub.is_empty() {
                    format!(r#"<div style="font-size:{}px;color:{};margin-top:8px;">{}</div>"#, caption_fs, colors.text_secondary, escape_html(sub))
                } else { String::new() };
                g.push_str(&format!(
                    r#"<div style="flex:1;background:{};border:{};{}border-radius:{};padding:24px 20px;box-shadow:{};text-align:center;">
                        <div style="font-size:{}px;color:{};text-transform:uppercase;letter-spacing:0.05em;margin-bottom:10px;">{}</div>
                        <div style="font-family:{};font-size:{}px;font-weight:700;color:{};line-height:1;">{}</div>
                        {}
                    </div>"#,
                    card_bg, card_border, card_blur, radius_md, shadow_md,
                    caption_fs, colors.text_secondary, escape_html(label),
                    tokens.heading_font, display_fs, tokens.primary, escape_html(val),
                    sub_html
                ));
            }
            g.push_str("</div>");
            g
        }
        _ => {
            // auto
            if stats.len() <= 3 {
                let mut g = format!(r#"<div style="display:flex;gap:12px;margin-top:16px;">"#);
                for item in &stats {
                    let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
                    let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                    let sub = item.get("sub").and_then(|v| v.as_str()).unwrap_or("");
                    let sub_html = if !sub.is_empty() {
                        format!(r#"<div style="font-size:{}px;color:{};margin-top:4px;">{}</div>"#, micro_fs, colors.text_secondary, escape_html(sub))
                    } else { String::new() };
                    g.push_str(&format!(
                        r#"<div style="flex:1;background:{};border:{};{}border-radius:{};padding:16px 12px;box-shadow:{};">
                            <div style="font-size:{}px;color:{};text-transform:uppercase;letter-spacing:0.05em;margin-bottom:8px;">{}</div>
                            <div style="font-family:{};font-size:{}px;font-weight:700;color:{};line-height:1;">{}</div>
                            {}
                        </div>"#,
                        card_bg, card_border, card_blur, radius_md, shadow_sm,
                        micro_fs, label_color, escape_html(label),
                        tokens.heading_font, headline_fs, tokens.primary, escape_html(val),
                        sub_html
                    ));
                }
                g.push_str("</div>");
                g
            } else {
                let mut g = format!(r#"<div style="display:flex;gap:16px;flex-wrap:wrap;width:100%;margin-top:16px;">"#);
                for item in &stats {
                    let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
                    let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                    let sub = item.get("sub").and_then(|v| v.as_str()).unwrap_or("");
                    let sub_html = if !sub.is_empty() {
                        format!(r#"<div style="font-size:{}px;color:{};margin-top:4px;">{}</div>"#, micro_fs, colors.text_secondary, escape_html(sub))
                    } else { String::new() };
                    g.push_str(&format!(
                        r#"<div style="width:calc(50% - 8px);min-width:120px;background:{};border:{};{}border-radius:{};padding:12px;box-shadow:{};">
                            <div style="font-size:{}px;color:{};text-transform:uppercase;letter-spacing:0.05em;margin-bottom:8px;">{}</div>
                            <div style="font-family:{};font-size:{}px;font-weight:700;color:{};line-height:1;">{}</div>
                            {}
                        </div>"#,
                        card_bg, card_border, card_blur, radius_md, shadow_sm,
                        micro_fs, label_color, escape_html(label),
                        tokens.heading_font, title_fs, tokens.primary, escape_html(val),
                        sub_html
                    ));
                }
                g.push_str("</div>");
                g
            }
        }
    };

    let content = format!("{}{}", heading, grid);
    let html = slide_base(&content, tokens, bg_style, false, "80px 52px 80px", "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": effective_variant,
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. timeline_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Vertical process timeline.
///
/// Variants: `vertical` (default) | `horizontal` | `compact`
///
/// Each step in `steps` is a JSON object with `title` and `description`.
pub fn timeline_slide(
    tokens: &DesignTokens,
    title: &str,
    steps: Vec<Value>,
    bg_style: &str,
    variant: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let heading = heading_block(title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 12px", true);

    let (gc, gx) = get_glass_container(tokens, is_dark);

    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;

    let effective_variant = variant;

    let steps_html = match effective_variant {
        "horizontal" => {
            let mut s = format!(r#"<div style="display:flex;gap:16px;margin-top:16px;">"#);
            for (i, step) in steps.iter().enumerate() {
                let step_title = step.get("title").and_then(|v| v.as_str()).unwrap_or("");
                let step_desc = step.get("description").and_then(|v| v.as_str()).unwrap_or("");
                s.push_str(&format!(
                    r#"<div style="flex:1;text-align:center;">
                        <div style="width:32px;height:32px;border-radius:50%;background:{}12;border:2px solid {};display:inline-flex;align-items:center;justify-content:center;font-size:13px;font-weight:700;color:{};margin-bottom:8px;">{}</div>
                        <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0 0 4px;">{}</h3>
                        <p style="font-family:{};font-size:{}px;color:{};margin:0;line-height:1.3;">{}</p>
                    </div>"#,
                    tokens.primary, tokens.primary, tokens.primary, i + 1,
                    tokens.body_font, body_fs, colors.text_primary, escape_html(step_title),
                    tokens.body_font, caption_fs, colors.text_secondary, escape_html(step_desc)
                ));
            }
            s.push_str("</div>");
            s
        }
        "compact" => {
            let mut s = String::new();
            for (i, step) in steps.iter().enumerate() {
                let step_title = step.get("title").and_then(|v| v.as_str()).unwrap_or("");
                let step_desc = step.get("description").and_then(|v| v.as_str()).unwrap_or("");
                s.push_str(&format!(
                    r#"<div style="display:flex;gap:12px;align-items:flex-start;margin-bottom:10px;">
                        <div style="width:28px;height:28px;border-radius:50%;background:{}12;border:2px solid {};display:flex;align-items:center;justify-content:center;font-size:12px;font-weight:700;color:{};flex-shrink:0;">{}</div>
                        <div>
                            <span style="font-family:{};font-size:{}px;font-weight:600;color:{};">{}</span>
                            <span style="font-family:{};font-size:{}px;color:{};margin-left:8px;">{}</span>
                        </div>
                    </div>"#,
                    tokens.primary, tokens.primary, tokens.primary, i + 1,
                    tokens.body_font, body_fs, colors.text_primary, escape_html(step_title),
                    tokens.body_font, caption_fs, colors.text_secondary, escape_html(step_desc)
                ));
            }
            s
        }
        _ => {
            // vertical (default)
            let mut s = String::new();
            let n = steps.len();
            for (i, step) in steps.iter().enumerate() {
                let step_title = step.get("title").and_then(|v| v.as_str()).unwrap_or("");
                let step_desc = step.get("description").and_then(|v| v.as_str()).unwrap_or("");
                let is_last = i == n - 1;
                let line_html = if is_last {
                    String::new()
                } else {
                    format!(
                        r#"<div style="position:absolute;left:17px;top:36px;bottom:-16px;width:2px;background:{}30;"></div>"#,
                        tokens.primary
                    )
                };
                s.push_str(&format!(
                    r#"<div style="display:flex;gap:18px;position:relative;margin-bottom:16px;">
                        {}
                        <div style="width:36px;height:36px;border-radius:50%;background:{}12;border:2px solid {};display:flex;align-items:center;justify-content:center;font-size:14px;font-weight:700;color:{};flex-shrink:0;z-index:2;">{}</div>
                        <div>
                            <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0 0 4px;">{}</h3>
                            <p style="font-family:{};font-size:{}px;color:{};margin:0;line-height:1.4;">{}</p>
                        </div>
                    </div>"#,
                    line_html,
                    tokens.primary, tokens.primary, tokens.primary, i + 1,
                    tokens.body_font, body_fs, colors.text_primary, escape_html(step_title),
                    tokens.body_font, caption_fs, colors.text_secondary, escape_html(step_desc)
                ));
            }
            s
        }
    };

    let content = format!(
        r#"{}{}<div style="margin-top:16px;">{}</div>{}"#,
        gc, heading, steps_html, gx
    );
    let html = slide_base(&content, tokens, bg_style, false, "80px 52px 80px", "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": effective_variant,
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 9. callout_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Highlighted callout card (info/warning/success/danger).
///
/// Variant selects accent colour: `info` | `warning` | `success` | `danger`
pub fn callout_slide(
    tokens: &DesignTokens,
    title: &str,
    text: &str,
    icon: &str,
    variant: &str,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let (accent_color, _fill_color) = match variant {
        "warning" => ("#F59E0B", "#F59E0B10"),
        "success" => ("#10B981", "#10B98110"),
        "danger"  => ("#EF4444", "#EF444410"),
        _         => (tokens.primary.as_str(), ""), // info — use primary
    };

    let glass_variant = if is_dark { "dark" } else { "light" };
    let radius_lg = tokens.radii.get("lg").cloned().unwrap_or_else(|| "12px".to_string());
    let g_styles = glass_surface(tokens, glass_variant, &radius_lg);
    let shadow_md = tokens.shadows.get("md").cloned().unwrap_or_else(|| "none".to_string());

    // Build callout glass style string and augment with left border
    let callout_styles_str = {
        let mut parts: Vec<String> = g_styles.iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        parts.push(format!("border-left: 6px solid {}", accent_color));
        parts.join("; ")
    };

    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let title_fs = tokens.type_scale.get("title").unwrap().font_size;

    let icon_box_style = format!(
        "width:44px;height:44px;min-width:44px;border-radius:50%;background:{}1a;color:{};display:flex;align-items:center;justify-content:center;font-size:20px;box-shadow:inset 0 1px 3px rgba(255,255,255,0.1);",
        accent_color, accent_color
    );
    let glow_style = format!(
        "box-shadow: 0 8px 32px -4px {}1f, {};",
        accent_color, shadow_md
    );

    let watermark_html = format!(
        r#"<div style="position:absolute;right:-40px;bottom:-30px;font-size:240px;opacity:0.04;pointer-events:none;z-index:1;user-select:none;transform:rotate(12deg);line-height:1;font-family:system-ui;">{}</div>"#,
        icon
    );
    let glow_orb = format!(
        r#"<div style="position:absolute;left:-80px;top:-80px;width:260px;height:260px;border-radius:50%;background:{};opacity:0.06;filter:blur(50px);-webkit-filter:blur(50px);pointer-events:none;z-index:1;"></div>"#,
        accent_color
    );

    let content = format!(
        r#"<div style="position:relative;width:100%;height:100%;display:flex;align-items:center;justify-content:center;box-sizing:border-box;">
            {}
            {}
            <div style="{};padding:36px 32px;{}width:100%;box-sizing:border-box;position:relative;z-index:2;">
                <div style="display:flex;align-items:center;gap:18px;margin-bottom:18px;">
                    <div style="{}">{}</div>
                    <div>
                        <span style="font-size:9.5px;font-weight:700;letter-spacing:0.1em;text-transform:uppercase;color:{};margin-bottom:4px;display:block;">{}</span>
                        <h2 style="font-family:{};font-size:{}px;font-weight:800;color:{};margin:0;line-height:1.2;">{}</h2>
                    </div>
                </div>
                <p style="font-family:{};font-size:{}px;line-height:1.65;color:{};margin:0;overflow-wrap:break-word;word-break:break-word;">{}</p>
            </div>
        </div>"#,
        glow_orb,
        watermark_html,
        callout_styles_str,
        glow_style,
        icon_box_style,
        escape_html(icon),
        accent_color, escape_html(variant),
        tokens.heading_font, title_fs, colors.text_primary, escape_html(title),
        tokens.body_font, body_fs, colors.text_secondary, escape_html(text)
    );

    let html = slide_base(&content, tokens, bg_style, false, "80px 52px 80px", "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": variant,
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 10. split_features_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Two-column split: heading on left, feature list on right.
///
/// Each feature in `features` is a JSON object with `title` and `description`.
pub fn split_features_slide(
    tokens: &DesignTokens,
    title: &str,
    features: Vec<Value>,
    bg_style: &str,
    theme: &str,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let heading = heading_block(title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 12px", true);

    let (gc, gx) = get_glass_container(tokens, is_dark);

    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;

    let mut features_html = String::new();
    for feat in &features {
        let t = feat.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let d = feat.get("description").and_then(|v| v.as_str()).unwrap_or("");
        features_html.push_str(&format!(
            r#"<div style="margin-bottom:20px;">
                <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0 0 6px;">{}</h3>
                <p style="font-family:{};font-size:{}px;color:{};margin:0;line-height:1.45;">{}</p>
            </div>"#,
            tokens.body_font, body_fs, colors.text_primary, escape_html(t),
            tokens.body_font, caption_fs, colors.text_secondary, escape_html(d)
        ));
    }

    // Use a split layout with heading on left, features on right
    let left_content = format!("{}{}", gc, heading);
    let right_content = format!("{}{}", features_html, gx);

    let html = split_layout(&left_content, &right_content, tokens, bg_style, "32px", "1fr 1fr", false);
    json!({
        "html": html,
        "background": bg_style,
        "variant": "default",
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 11. grid_cards_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Card grid for features, services, or offerings.
///
/// Each card in `cards` is a JSON object with `icon`, `title`, `description`.
/// Columns: 3 for ≤3 cards, 2 for ≤2 cards (asymmetric 2fr/1fr).
pub fn grid_cards_slide(
    tokens: &DesignTokens,
    title: &str,
    cards: Vec<Value>,
    bg_style: &str,
    theme: &str,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let heading = heading_block(title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 12px", true);

    let (card_bg, card_border, card_blur) = card_styles(tokens, is_dark);

    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    let title_fs = tokens.type_scale.get("title").unwrap().font_size;
    let radius_md = tokens.radii.get("md").cloned().unwrap_or_else(|| "10px".to_string());
    let shadow_sm = tokens.shadows.get("sm").cloned().unwrap_or_else(|| "none".to_string());

    let card_html = if cards.len() >= 3 {
        // 3-column grid
        let mut items_html = String::new();
        for card in cards.iter().take(3) {
            let ico = card.get("icon").and_then(|v| v.as_str()).unwrap_or("⚡");
            let t = card.get("title").and_then(|v| v.as_str()).unwrap_or("");
            let d = card.get("description").and_then(|v| v.as_str()).unwrap_or("");
            let desc_html = if !d.is_empty() {
                format!(
                    r#"<p style="font-family:{};font-size:{}px;color:{};margin:8px 0 0;line-height:1.5;overflow-wrap:break-word;word-break:break-word;">{}</p>"#,
                    tokens.body_font, caption_fs, colors.text_secondary, escape_html(d)
                )
            } else { String::new() };
            items_html.push_str(&format!(
                r#"<div style="background:{};border:{};{}border-radius:{};padding:20px;box-shadow:{};display:flex;flex-direction:column;">
                    <div style="font-size:28px;margin-bottom:12px;">{}</div>
                    <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0;">{}</h3>
                    {}
                </div>"#,
                card_bg, card_border, card_blur, radius_md, shadow_sm,
                escape_html(ico),
                tokens.body_font, title_fs, colors.text_primary, escape_html(t),
                desc_html
            ));
        }
        format!(
            r#"<div style="display:grid;grid-template-columns:repeat(3,1fr);gap:16px;width:100%;margin-top:16px;">{}</div>"#,
            items_html
        )
    } else if cards.len() >= 2 {
        // Asymmetric 2fr/1fr for 2 cards
        let card1 = &cards[0];
        let card2 = &cards[1];

        let ico1 = card1.get("icon").and_then(|v| v.as_str()).unwrap_or("⚡");
        let t1 = card1.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let d1 = card1.get("description").and_then(|v| v.as_str()).unwrap_or("");
        let desc1 = if !d1.is_empty() {
            format!(r#"<p style="font-family:{};font-size:{}px;color:{};margin:8px 0 0;line-height:1.5;">{}</p>"#, tokens.body_font, caption_fs, colors.text_secondary, escape_html(d1))
        } else { String::new() };

        let ico2 = card2.get("icon").and_then(|v| v.as_str()).unwrap_or("⚡");
        let t2 = card2.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let d2 = card2.get("description").and_then(|v| v.as_str()).unwrap_or("");
        let desc2 = if !d2.is_empty() {
            format!(r#"<p style="font-family:{};font-size:{}px;color:{};margin:8px 0 0;line-height:1.5;">{}</p>"#, tokens.body_font, caption_fs, colors.text_secondary, escape_html(d2))
        } else { String::new() };

        format!(
            r#"<div style="display:grid;grid-template-columns:2fr 1fr;gap:16px;width:100%;margin-top:16px;">
                <div style="background:{};border:{};{}border-radius:{};padding:24px 20px;box-shadow:{};display:flex;flex-direction:column;">
                    <div style="font-size:32px;margin-bottom:16px;">{}</div>
                    <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0;">{}</h3>
                    {}
                </div>
                <div style="background:{};border:{};{}border-radius:{};padding:24px 20px;box-shadow:{};display:flex;flex-direction:column;">
                    <div style="font-size:32px;margin-bottom:16px;">{}</div>
                    <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0;">{}</h3>
                    {}
                </div>
            </div>"#,
            card_bg, card_border, card_blur, radius_md, shadow_sm,
            escape_html(ico1),
            tokens.body_font, title_fs, colors.text_primary, escape_html(t1),
            desc1,
            card_bg, card_border, card_blur, radius_md, shadow_sm,
            escape_html(ico2),
            tokens.body_font, title_fs, colors.text_primary, escape_html(t2),
            desc2
        )
    } else if !cards.is_empty() {
        // Single card fallback
        let card = &cards[0];
        let ico = card.get("icon").and_then(|v| v.as_str()).unwrap_or("⚡");
        let t = card.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let d = card.get("description").and_then(|v| v.as_str()).unwrap_or("");
        let desc_html = if !d.is_empty() {
            format!(r#"<p style="font-family:{};font-size:{}px;color:{};margin:8px 0 0;line-height:1.5;">{}</p>"#, tokens.body_font, caption_fs, colors.text_secondary, escape_html(d))
        } else { String::new() };
        format!(
            r#"<div style="background:{};border:{};{}border-radius:{};padding:24px 20px;box-shadow:{};display:flex;flex-direction:column;width:100%;margin-top:16px;">
                <div style="font-size:32px;margin-bottom:16px;">{}</div>
                <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0;">{}</h3>
                {}
            </div>"#,
            card_bg, card_border, card_blur, radius_md, shadow_sm,
            escape_html(ico),
            tokens.body_font, title_fs, colors.text_primary, escape_html(t),
            desc_html
        )
    } else {
        String::new()
    };

    let content = format!("{}{}", heading, card_html);
    let html = slide_base(&content, tokens, bg_style, false, "80px 48px 80px", "center");
    json!({
        "html": html,
        "background": bg_style,
        "variant": "default",
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 12. headline_subheadline_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Big display heading + body text, centred or left-aligned.
pub fn headline_subheadline_slide(
    tokens: &DesignTokens,
    headline: &str,
    subheadline: &str,
    bg_style: &str,
    theme: &str,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let gradient_colors = if is_dark {
        ("#FFFFFF", colors.text_primary.as_str())
    } else {
        (colors.text_primary.as_str(), colors.text_secondary.as_str())
    };

    let heading_html = heading_block(
        headline, tokens, "display",
        None, true, Some((gradient_colors.0, gradient_colors.1)),
        "center", "0 0 16px", true,
    );

    let sub_html = if !subheadline.is_empty() {
        text_block(subheadline, tokens, "body", Some(&colors.text_secondary), false, None, "center", None, "16px 0 0")
    } else {
        String::new()
    };

    let content = format!(
        r#"<div style="text-align: center;">{}{}</div>"#,
        heading_html, sub_html
    );

    let html = slide_base(&content, tokens, bg_style, false, "80px 48px 80px", "center");
    json!({
        "html": html,
        "background": bg_style,
        "variant": "center",
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 13. definition_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Term + definition layout (glossary-style).
/// `context` maps to the `category` field in the Python version.
pub fn definition_slide(
    tokens: &DesignTokens,
    term: &str,
    definition: &str,
    context: &str,
    bg_style: &str,
    theme: &str,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);

    let display_fs = tokens.type_scale.get("display").unwrap().font_size;
    let title_fs = tokens.type_scale.get("title").unwrap().font_size;

    let category_html = if !context.is_empty() {
        format!(
            r#"<span style="font-family:{};font-size:11px;font-weight:700;letter-spacing:0.1em;text-transform:uppercase;color:{};display:block;margin-bottom:12px;">{}</span>"#,
            tokens.body_font, colors.primary, escape_html(context)
        )
    } else {
        String::new()
    };

    let term_html = format!(
        r#"<h2 style="font-family:{};font-size:{}px;font-weight:700;color:{};margin:0 0 16px;line-height:1.15;">{}</h2>"#,
        tokens.heading_font, display_fs, colors.text_primary, escape_html(term)
    );

    let divider_html = format!(
        r#"<div style="width:100%;height:1px;background:{};opacity:0.3;margin-bottom:20px;"></div>"#,
        colors.border
    );

    let def_html = format!(
        r#"<p style="font-family:{};font-size:{}px;font-weight:400;color:{};margin:0 0 24px;line-height:1.5;">{}</p>"#,
        tokens.body_font, title_fs, colors.text_secondary, escape_html(definition)
    );

    let content = format!(
        r#"<div style="width:100%;text-align:left;">{}{}{}{}</div>"#,
        category_html, term_html, divider_html, def_html
    );

    let html = slide_base(&content, tokens, bg_style, false, "80px 52px 80px", "center");
    json!({
        "html": html,
        "background": bg_style,
        "variant": "default",
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 14. text_block_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Title + multi-line body text (article-style).
/// `body` is a single string; newlines produce separate paragraphs.
pub fn text_block_slide(
    tokens: &DesignTokens,
    title: &str,
    body: &str,
    bg_style: &str,
    theme: &str,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let body_fs = 14i32;

    let title_html = if !title.is_empty() {
        format!(
            r#"<h2 style="font-family:{};font-size:22px;font-weight:700;color:{};margin:0 0 16px;line-height:1.2;">{}</h2>"#,
            tokens.heading_font, colors.text_primary, escape_html(title)
        )
    } else {
        String::new()
    };

    // Split body on newlines into paragraphs
    let mut body_html = String::new();
    for para in body.split('\n') {
        let p = para.trim();
        if !p.is_empty() {
            body_html.push_str(&format!(
                r#"<p style="font-family:{};font-size:{}px;color:{};margin:0 0 12px;line-height:1.6;">{}</p>"#,
                tokens.body_font, body_fs, colors.text_secondary, escape_html(p)
            ));
        }
    }

    let content = format!(
        r#"<div style="max-width:360px;margin:0 auto;text-align:left;width:100%;">{}{}</div>"#,
        title_html, body_html
    );

    let html = slide_base(&content, tokens, bg_style, false, "80px 52px 80px", "center");
    json!({
        "html": html,
        "background": bg_style,
        "variant": "medium",
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// dispatch_slide — routes by slide_type string to the correct generator
// ─────────────────────────────────────────────────────────────────────────────

/// Route a slide type name + JSON params to the appropriate slide generator.
///
/// This is the single entry-point used by `mcp_server::generate_slide`.
/// `params` is a JSON object whose keys mirror the Python component kwargs.
/// Unknown slide types return an `Err` with a description.
pub fn dispatch_slide(
    slide_type: &str,
    tokens: &DesignTokens,
    params: &Value,
    bg_style: &str,
    theme: &str,
    _archetype: &str,
) -> Result<Value, String> {
    let p = params;
    let s = |key: &str| p.get(key).and_then(|v| v.as_str()).unwrap_or("").to_string();
    let b = |key: &str, default: bool| p.get(key).and_then(|v| v.as_bool()).unwrap_or(default);
    let f = |key: &str, default: f32| -> f32 {
        p.get(key).and_then(|v| v.as_f64()).map(|x| x as f32).unwrap_or(default)
    };

    let bg_img = s("background_image");
    let img_opacity = f("image_opacity", 0.4);

    let mut result: Result<Value, String> = match slide_type {
        "hero" => Ok(hero_slide(
            tokens,
            &s("headline"),
            &s("subheadline"),
            &s("badge"),
            bg_style,
            b("decorations", true),
            &s("variant").if_empty("left-aligned"),
            theme,
            &bg_img,
            img_opacity,
        )),
        "feature" => Ok(feature_slide(
            tokens,
            &s("icon"),
            &s("title"),
            &s("description"),
            &s("number"),
            bg_style,
            &s("variant").if_empty("stacked"),
            theme,
            &bg_img,
            img_opacity,
        )),
        "list" => {
            let items = p.get("items")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(list_slide(
                tokens,
                &s("title"),
                items,
                bg_style,
                b("numbered", false),
                &s("variant").if_empty("bulleted"),
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "quote" => Ok(quote_slide(
            tokens,
            &s("quote"),
            &s("author"),
            &s("role"),
            bg_style,
            &s("variant").if_empty("centered"),
            theme,
            &bg_img,
            img_opacity,
        )),
        "cta" => Ok(cta_slide(
            tokens,
            &s("headline"),
            &s("button_text").if_empty("Learn More"),
            &s("button_url").if_empty("#"),
            &s("subtext"),
            bg_style,
            &s("variant").if_empty("centered"),
            theme,
            &bg_img,
            img_opacity,
        )),
        "comparison" => {
            let left_items: Vec<String> = p.get("left_items")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();
            let right_items: Vec<String> = p.get("right_items")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();
            Ok(comparison_slide(
                tokens,
                &s("title"),
                &s("left_label"),
                left_items,
                &s("right_label"),
                right_items,
                bg_style,
                &s("variant").if_empty("default"),
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "stat_row" => {
            let stats = p.get("stats")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(stat_row_slide(
                tokens,
                &s("title"),
                stats,
                bg_style,
                &s("variant").if_empty("auto"),
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "timeline" => {
            let steps = p.get("steps")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(timeline_slide(
                tokens,
                &s("title"),
                steps,
                bg_style,
                &s("variant").if_empty("vertical"),
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "callout" => Ok(callout_slide(
            tokens,
            &s("title"),
            &s("text"),
            &s("icon").if_empty("💡"),
            &s("variant").if_empty("info"),
            bg_style,
            theme,
            &bg_img,
            img_opacity,
        )),
        "split_features" => {
            let features = p.get("features")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(split_features_slide(
                tokens,
                &s("title"),
                features,
                bg_style,
                theme,
            ))
        }
        "grid_cards" => {
            let cards = p.get("cards")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(grid_cards_slide(
                tokens,
                &s("title"),
                cards,
                bg_style,
                theme,
            ))
        }
        "headline_subheadline" => Ok(headline_subheadline_slide(
            tokens,
            &s("headline"),
            &s("subheadline"),
            bg_style,
            theme,
        )),
        "definition" => Ok(definition_slide(
            tokens,
            &s("term"),
            &s("definition"),
            &s("context"),
            bg_style,
            theme,
        )),
        "text_block" => Ok(text_block_slide(
            tokens,
            &s("title"),
            &s("body"),
            bg_style,
            theme,
        )),
        other => Err(format!("Unknown slide type: '{}'", other)),
    };

    if let Ok(ref mut val) = result {
        if let Some(obj) = val.as_object_mut() {
            obj.insert("archetype".to_string(), serde_json::json!(_archetype));
        }
    }
    result
}

/// Helper trait for defaulting empty strings.
trait IfEmpty {
    fn if_empty(self, default: &str) -> String;
}

impl IfEmpty for String {
    fn if_empty(self, default: &str) -> String {
        if self.is_empty() { default.to_string() } else { self }
    }
}
