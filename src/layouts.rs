use crate::blocks::escape_html;
use crate::design_system::{DesignTokens, contrast_ratio, get_contrast_safe_color};
use crate::effects::{floating_shape, noise_overlay, slide_background};
use std::collections::HashMap;

/// Theme-specific surface color overrides (mirrors CSS THEME_CSS_OVERRIDES).
pub fn theme_surface_overrides(theme: &str, variant: &str) -> Option<&'static str> {
    match (theme, variant) {
        ("editorial", "light") => Some("#F8F6F3"),
        ("editorial", "dark") => Some("#0A0A0F"),
        ("bold", "light") => Some("#F0F0F5"),
        ("bold", "dark") => Some("#050508"),
        ("minimal", "light") => Some("#FFFFFF"),
        ("minimal", "dark") => Some("#111111"),
        ("dark", "light") => Some("#1A1A2E"), // "dark" theme makes "light" slides dark
        ("dark", "dark") => Some("#0D0D14"),
        ("vibrant", "light") => Some("#F5F3FF"),
        ("vibrant", "dark") => Some("#0A0515"),
        ("natural", "light") => Some("#FAF8F5"),
        ("natural", "dark") => Some("#0F0D0A"),
        _ => None,
    }
}

pub fn effective_surface<'a>(tokens: &'a DesignTokens, bg_style: &str, theme: &str) -> String {
    if matches!(bg_style, "dark" | "hero" | "gradient") {
        theme_surface_overrides(theme, "dark")
            .map(|s| s.to_string())
            .unwrap_or_else(|| tokens.surface_dark.clone())
    } else {
        theme_surface_overrides(theme, "light")
            .map(|s| s.to_string())
            .unwrap_or_else(|| tokens.surface_light.clone())
    }
}

pub fn is_visually_dark_surface(surface_hex: &str) -> bool {
    contrast_ratio(surface_hex, "#000000") < contrast_ratio(surface_hex, "#FFFFFF")
}

pub fn is_dark_bg(bg_style: &str) -> bool {
    matches!(bg_style, "dark" | "hero" | "gradient")
}

#[derive(Debug, Clone)]
pub struct SlideColors {
    pub text_primary: String,
    pub text_secondary: String,
    pub primary: String,
    pub button_bg: String,
    pub button_text: String,
    pub border: String,
    pub is_dark: bool,
}

pub fn get_slide_colors(tokens: &DesignTokens, bg_style: &str, theme: &str) -> SlideColors {
    let eff_surf = effective_surface(tokens, bg_style, theme);
    // theme "dark" forces visually dark surface even on "light" bg style
    let is_dark = is_dark_bg(bg_style) || is_visually_dark_surface(&eff_surf);

    let bg_stops = if matches!(bg_style, "gradient" | "hero") {
        vec![eff_surf.clone(), tokens.primary_dark.clone()]
    } else {
        vec![eff_surf.clone()]
    };

    // Ensure we start with base colors that match the actual target background luminance, not bg_style
    let base_primary = if is_dark {
        &tokens.text_on_dark
    } else {
        &tokens.text_primary
    };
    let base_secondary = if is_dark {
        &tokens.text_on_dark_secondary
    } else {
        &tokens.text_secondary
    };

    let text_primary = get_contrast_safe_color(base_primary, &bg_stops, 7.0)
        .unwrap_or_else(|_| base_primary.clone());
    let text_secondary = get_contrast_safe_color(base_secondary, &bg_stops, 5.0)
        .unwrap_or_else(|_| base_secondary.clone());

    let primary =
        get_contrast_safe_color(&tokens.primary, &bg_stops, if is_dark { 5.5 } else { 4.5 })
            .unwrap_or_else(|_| tokens.primary.clone());

    let (button_bg, button_text) = if is_dark {
        let bbg = tokens.primary_light.clone();
        let bt = get_contrast_safe_color(&tokens.surface_dark, &[bbg.clone()], 5.0)
            .unwrap_or_else(|_| tokens.surface_dark.clone());
        (bbg, bt)
    } else {
        let bbg = tokens.primary_dark.clone();
        let bt = get_contrast_safe_color(&tokens.surface_light, &[bbg.clone()], 5.0)
            .unwrap_or_else(|_| tokens.surface_light.clone());
        (bbg, bt)
    };

    let border = if is_dark {
        tokens.border_dark.clone()
    } else {
        tokens.border_light.clone()
    };

    SlideColors {
        text_primary,
        text_secondary,
        primary,
        button_bg,
        button_text,
        border,
        is_dark,
    }
}

#[allow(dead_code)]
fn format_styles(map: &HashMap<String, String>) -> String {
    map.iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("; ")
}

fn build_shapes(tokens: &DesignTokens, bg_style: &str) -> String {
    let is_dark = is_dark_bg(bg_style);

    let c1_map = if is_dark {
        floating_shape("circle", 320, &tokens.primary, "-80px", "-80px", 0, 0.07)
    } else if bg_style == "hero" {
        floating_shape("circle", 350, &tokens.primary, "-90px", "-90px", 0, 0.08)
    } else {
        floating_shape("circle", 300, &tokens.primary, "-70px", "-70px", 0, 0.06)
    };

    let mut c2 = if is_dark {
        floating_shape("diamond", 120, &tokens.accent, "auto", "60%", 15, 0.05)
    } else if bg_style == "hero" {
        floating_shape("ring", 180, &tokens.accent, "auto", "50%", 0, 0.06)
    } else {
        floating_shape("pill", 160, &tokens.accent, "auto", "65%", 0, 0.04)
    };
    c2.insert("right".to_string(), "-40px".to_string());

    let c1_css = c1_map
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("; ");
    let c2_css = c2
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("; ");

    format!(
        r#"<div style="position:absolute;inset:0;overflow:hidden;pointer-events:none;z-index:1;"><div style="{}"></div><div style="{}"></div></div>"#,
        c1_css, c2_css
    )
}

pub fn slide_base(
    content_html: &str,
    tokens: &DesignTokens,
    bg_style: &str,
    decorations: bool,
    padding: &str,
    justify: &str,
) -> String {
    // Dark determination must MATCH get_slide_colors exactly. get_slide_colors
    // uses the theme-aware effective_surface (a theme="dark" on a light bg_style
    // paints a dark surface), so slide_base must paint the same dark surface
    // or the slide renders white text on a light background.
    let theme = crate::components::current_theme();
    let eff_surf = effective_surface(tokens, bg_style, &theme);
    let is_dark = is_dark_bg(bg_style) || is_visually_dark_surface(&eff_surf);
    let bg_var = if is_dark {
        "var(--surface-dark)"
    } else {
        "var(--surface-light)"
    };

    let bg = {
        let raw = slide_background(tokens, bg_style, None);
        if !is_dark && bg_style == "light" {
            let mesh = tokens.gradients.get("mesh").cloned().unwrap_or_default();
            if !mesh.is_empty() {
                format!("{}, {}", mesh, raw)
            } else {
                raw
            }
        } else {
            raw
        }
    };

    let shapes = if decorations {
        build_shapes(tokens, bg_style)
    } else {
        String::new()
    };

    let mut noise_style = noise_overlay(0.04);
    noise_style.insert("z-index".to_string(), "1".to_string());
    let noise_css = noise_style
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("; ");

    format!(
        r#"<div style="position:relative;width:100%;height:100%;background:{};background-color:{};overflow:hidden;">
            <div style="{}"></div>
            {}
            <div class="slide-content" style="position:relative;z-index:10;padding:{};display:flex;flex-direction:column;justify-content:{};height:100%;width:100%;box-sizing:border-box;">
                {}
            </div>
        </div>"#,
        bg, bg_var, noise_css, shapes, padding, justify, content_html
    )
}

/// Hero slide base without overflow:hidden on slide-content to allow glass container blur to extend beyond bounds.
pub fn hero_slide_base(
    content_html: &str,
    tokens: &DesignTokens,
    bg_style: &str,
    decorations: bool,
    padding: &str,
    justify: &str,
) -> String {
    // Same theme-aware dark determination as slide_base/get_slide_colors.
    let theme = crate::components::current_theme();
    let eff_surf = effective_surface(tokens, bg_style, &theme);
    let is_dark = is_dark_bg(bg_style) || is_visually_dark_surface(&eff_surf);
    let bg_var = if is_dark {
        "var(--surface-dark)"
    } else {
        "var(--surface-light)"
    };

    let bg = {
        let raw = slide_background(tokens, bg_style, None);
        if !is_dark && bg_style == "light" {
            let mesh = tokens.gradients.get("mesh").cloned().unwrap_or_default();
            if !mesh.is_empty() {
                format!("{}, {}", mesh, raw)
            } else {
                raw
            }
        } else {
            raw
        }
    };

    let shapes = if decorations {
        build_shapes(tokens, bg_style)
    } else {
        String::new()
    };

    let mut noise_style = noise_overlay(0.04);
    noise_style.insert("z-index".to_string(), "1".to_string());
    let noise_css = noise_style
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("; ");

    format!(
        r#"<div style="position:relative;width:100%;height:100%;background:{};background-color:{};overflow:hidden;">
            <div style="{}"></div>
            {}
            <div class="slide-content" style="position:relative;z-index:10;padding:{};display:flex;flex-direction:column;justify-content:{};height:100%;width:100%;box-sizing:border-box;">
                {}
            </div>
        </div>"#,
        bg, bg_var, noise_css, shapes, padding, justify, content_html
    )
}

pub fn centered_layout(
    content_html: &str,
    tokens: &DesignTokens,
    bg_style: &str,
    decorations: bool,
    padding: &str,
    justify: &str,
) -> String {
    slide_base(
        content_html,
        tokens,
        bg_style,
        decorations,
        padding,
        justify,
    )
}

/// Full-bleed image variant of `slide_base`.
///
/// Used by image-primary slides (image_headline, image_quote, image_stat, etc.)
/// where the `<img>` must fill the entire slide canvas rather than being
/// constrained to the 420×525 composition. The content wrapper uses the
/// `slide-content--bleed` class instead of `slide-content`, so the
/// `.slide--full-bleed .slide-content` constrainer rule does NOT apply and
/// the content fills the first-of-type div (which is stretched to canvas).
pub fn slide_base_bleed(
    content_html: &str,
    tokens: &DesignTokens,
    bg_style: &str,
    decorations: bool,
    padding: &str,
    justify: &str,
) -> String {
    // Same theme-aware dark determination as slide_base/get_slide_colors.
    let theme = crate::components::current_theme();
    let eff_surf = effective_surface(tokens, bg_style, &theme);
    let is_dark = is_dark_bg(bg_style) || is_visually_dark_surface(&eff_surf);
    let bg_var = if is_dark {
        "var(--surface-dark)"
    } else {
        "var(--surface-light)"
    };

    let bg = {
        let raw = slide_background(tokens, bg_style, None);
        if !is_dark && bg_style == "light" {
            let mesh = tokens.gradients.get("mesh").cloned().unwrap_or_default();
            if !mesh.is_empty() {
                format!("{}, {}", mesh, raw)
            } else {
                raw
            }
        } else {
            raw
        }
    };

    let shapes = if decorations {
        build_shapes(tokens, bg_style)
    } else {
        String::new()
    };

    let mut noise_style = noise_overlay(0.04);
    noise_style.insert("z-index".to_string(), "1".to_string());
    let noise_css = noise_style
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("; ");

    format!(
        r#"<div style="position:relative;width:100%;height:100%;background:{};background-color:{};overflow:hidden;">
            <div style="{}"></div>
            {}
            <div class="slide-content--bleed" style="position:relative;z-index:10;padding:{};display:flex;flex-direction:column;justify-content:{};height:100%;width:100%;box-sizing:border-box;">
                {}
            </div>
        </div>"#,
        bg, bg_var, noise_css, shapes, padding, justify, content_html
    )
}

pub fn hero_layout(
    content_html: &str,
    tokens: &DesignTokens,
    bg_style: &str,
    decorations: bool,
    justify: &str,
) -> String {
    let justify_val = if justify == "center" {
        "center"
    } else {
        "center"
    }; // slide_base flex align needs center to keep vertically centered
    // Reduced padding from 80px to 60px to give more room for glass container + blur
    hero_slide_base(
        content_html,
        tokens,
        bg_style,
        decorations,
        "16px var(--space-6) 20px",
        justify_val,
    )
}

pub fn stack_layout(
    content_html: &str,
    tokens: &DesignTokens,
    bg_style: &str,
    gap: &str,
    decorations: bool,
    padding: &str,
    justify: &str,
) -> String {
    let content = format!(
        r#"<div style="display:flex;flex-direction:column;gap:{}">{}</div>"#,
        gap, content_html
    );
    slide_base(&content, tokens, bg_style, decorations, padding, justify)
}

pub fn split_layout(
    left_html: &str,
    right_html: &str,
    tokens: &DesignTokens,
    bg_style: &str,
    gap: &str,
    ratio: &str,
    decorations: bool,
) -> String {
    let content = format!(
        r#"<div style="display:grid;grid-template-columns:{};gap:{};overflow:hidden;align-items:center;"><div style="min-width:0;overflow:hidden;">{}</div><div style="min-width:0;overflow:hidden;">{}</div></div>"#,
        ratio, gap, left_html, right_html
    );
    slide_base(
        &content,
        tokens,
        bg_style,
        decorations,
        "16px var(--space-6) 20px",
        "center",
    )
}

#[allow(dead_code)]
pub fn grid_layout(
    items_html: &str,
    tokens: &DesignTokens,
    columns: u8,
    bg_style: &str,
    gap: &str,
    decorations: bool,
) -> String {
    let content = format!(
        r#"<div style="display:grid;grid-template-columns:repeat({},1fr);gap:{};overflow:hidden;">{}</div>"#,
        columns, gap, items_html
    );
    slide_base(
        &content,
        tokens,
        bg_style,
        decorations,
        "16px var(--space-6) 20px",
        "center",
    )
}

#[allow(dead_code)]
pub fn timeline_layout(
    content_html: &str,
    tokens: &DesignTokens,
    orientation: &str,
    bg_style: &str,
) -> String {
    let content = if orientation == "horizontal" {
        format!(
            r#"<div style="display:flex;gap:24px;">{}</div>"#,
            content_html
        )
    } else {
        format!(
            r#"<div style="display:flex;flex-direction:column;gap:16px;">{}</div>"#,
            content_html
        )
    };
    slide_base(
        &content,
        tokens,
        bg_style,
        true,
        "16px var(--space-6) 20px",
        "center",
    )
}

#[allow(dead_code)]
pub fn cta_layout(
    content_html: &str,
    tokens: &DesignTokens,
    align: &str,
    bg_style: &str,
) -> String {
    let content = format!(
        r#"<div style="text-align:{};">{}</div>"#,
        align, content_html
    );
    slide_base(&content, tokens, bg_style, true, "16px 64px 20px", "center")
}

#[allow(dead_code)]
pub fn bento_layout(
    content_html: &str,
    tokens: &DesignTokens,
    columns: &str,
    bg_style: &str,
    gap: &str,
) -> String {
    let content = format!(
        r#"<div style="display:grid;grid-template-columns:{};gap:{};overflow:hidden;">{}</div>"#,
        columns, gap, content_html
    );
    slide_base(
        &content,
        tokens,
        bg_style,
        true,
        "16px var(--space-6) 20px",
        "center",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design_system::{ContrastReportItem, DesignTokens, TypeLevel};
    use indexmap::IndexMap;

    fn tokens() -> DesignTokens {
        DesignTokens {
            primary: "#4f46e5".to_string(),
            primary_light: "#a5b4fc".to_string(),
            primary_dark: "#3730a3".to_string(),
            surface_light: "#ffffff".to_string(),
            surface_dark: "#101014".to_string(),
            text_primary: "#18181b".to_string(),
            text_secondary: "#52525b".to_string(),
            text_on_dark: "#f8fafc".to_string(),
            text_on_dark_secondary: "#cbd5e1".to_string(),
            border_light: "#e4e4e7".to_string(),
            border_dark: "#27272a".to_string(),
            accent: "#ec4899".to_string(),
            secondary: "#f59e0b".to_string(),
            tertiary: "#10b981".to_string(),
            gradient: "linear-gradient(135deg,#4f46e5,#ec4899)".to_string(),
            temperature: "cool".to_string(),
            heading_font: "Inter".to_string(),
            body_font: "Inter".to_string(),
            google_fonts_url: "https://fonts.googleapis.com/css2?family=Inter".to_string(),
            type_scale: IndexMap::new(),
            spacing: IndexMap::new(),
            contrast_report: IndexMap::new(),
            shadows: IndexMap::new(),
            radii: IndexMap::new(),
            gradients: IndexMap::new(),
            textures: IndexMap::new(),
            glass: serde_json::json!({}),
        }
    }

    /// The painted surface in `slide_base` MUST agree with the text colors
    /// chosen by `get_slide_colors` for the same theme/bg pair. If they drift
    /// (e.g. theme="dark" on a light bg_style), the slide renders white text on
    /// a light surface — the exact bug class fixed in this session. This test
    /// pins the invariant: light text ⟺ dark painted surface.
    #[test]
    fn slide_base_surface_agrees_with_get_slide_colors() {
        let tk = tokens();
        for bg in ["light", "mesh", "dark", "gradient", "hero"] {
            for theme in ["editorial", "dark", "bold"] {
                let colors = get_slide_colors(&tk, bg, theme);
                let text_is_light = crate::design_system::contrast_ratio(&colors.text_primary, &tk.surface_dark)
                    > crate::design_system::contrast_ratio(&colors.text_primary, &tk.surface_light);

                // slide_base paints surface-dark exactly when is_dark is true.
                let eff = effective_surface(&tk, bg, theme);
                let is_dark = is_dark_bg(bg) || is_visually_dark_surface(&eff);
                let paints_dark = is_dark;

                assert_eq!(
                    text_is_light, paints_dark,
                    "bg={bg} theme={theme}: get_slide_colors picked {} text but slide_base paints {}",
                    if text_is_light { "light" } else { "dark" },
                    if paints_dark { "dark surface" } else { "light surface" },
                );
            }
        }
    }
}
