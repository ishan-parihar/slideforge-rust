// components.rs — Rust port of carousel-mcp slide generators.
//
// Each public function builds slide HTML using the layout/block/effects
// helpers and returns a serde_json::Value with keys:
//   html        — the rendered slide HTML string
//   background  — the bg_style passed in
//   variant     — the effective variant resolved inside the function
//   theme       — the theme passed in

#[allow(unused_imports)]
use crate::blocks::{
    attribution_block, badge_block, button_block, divider_block, dot_marker, escape_html,
    gradient_text, heading_block, icon_block, list_item_block, quote_block, stat_block, text_block,
};
use crate::dataviz::{
    render_svg_gauge_chart, render_svg_line_chart, render_svg_radar_chart, render_svg_scatter_plot,
};
#[allow(unused_imports)]
use crate::design_system::DesignTokens;
#[allow(unused_imports)]
use crate::effects::glass_surface;
#[allow(unused_imports)]
use crate::layouts::{
    centered_layout, get_slide_colors, grid_layout, hero_layout, is_dark_bg, slide_base,
    slide_base_bleed, split_layout, stack_layout,
};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::{Value, json};

use qrcode::QrCode;
use qrcode::render::svg;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTreatment {
    pub image_filter: String,
    pub image_position: String,
    pub image_frame: String,
    pub image_overlay: String,
    pub image_mix_blend: String,
    pub image_mask: String,
    pub image_animation: String,
    pub image_content: String,
    pub image_opacity: Option<f32>,
}

impl Default for ImageTreatment {
    fn default() -> Self {
        Self {
            image_filter: "none".to_string(),
            image_position: "full-bleed".to_string(),
            image_frame: "none".to_string(),
            image_overlay: "gradient".to_string(),
            image_mix_blend: "normal".to_string(),
            image_mask: "none".to_string(),
            image_animation: "none".to_string(),
            image_content: "photography".to_string(),
            image_opacity: None,
        }
    }
}

impl ImageTreatment {
    pub fn editorial_preset() -> Self {
        Self {
            image_filter: "grayscale".to_string(),
            image_position: "full-bleed".to_string(),
            image_frame: "sharp".to_string(),
            image_overlay: "duotone".to_string(),
            image_mix_blend: "normal".to_string(),
            image_mask: "fade-bottom".to_string(),
            image_animation: "none".to_string(),
            image_content: "photography".to_string(),
            image_opacity: None,
        }
    }
    pub fn bold_preset() -> Self {
        Self {
            image_filter: "high-contrast".to_string(),
            image_position: "full-bleed".to_string(),
            image_frame: "rounded".to_string(),
            image_overlay: "solid".to_string(),
            image_mix_blend: "normal".to_string(),
            image_mask: "none".to_string(),
            image_animation: "subtle-zoom".to_string(),
            image_content: "photography".to_string(),
            image_opacity: None,
        }
    }
    pub fn minimal_preset() -> Self {
        Self {
            image_filter: "none".to_string(),
            image_position: "center".to_string(),
            image_frame: "sharp".to_string(),
            image_overlay: "none".to_string(),
            image_mix_blend: "normal".to_string(),
            image_mask: "none".to_string(),
            image_animation: "none".to_string(),
            image_content: "photography".to_string(),
            image_opacity: None,
        }
    }
    pub fn dark_preset() -> Self {
        Self {
            image_filter: "duotone-cool".to_string(),
            image_position: "full-bleed".to_string(),
            image_frame: "rounded".to_string(),
            image_overlay: "vignette".to_string(),
            image_mix_blend: "normal".to_string(),
            image_mask: "none".to_string(),
            image_animation: "subtle-zoom".to_string(),
            image_content: "photography".to_string(),
            image_opacity: None,
        }
    }
    pub fn vibrant_preset() -> Self {
        Self {
            image_filter: "high-contrast".to_string(),
            image_position: "full-bleed".to_string(),
            image_frame: "rounded".to_string(),
            image_overlay: "gradient".to_string(),
            image_mix_blend: "normal".to_string(),
            image_mask: "none".to_string(),
            image_animation: "ken-burns".to_string(),
            image_content: "abstract".to_string(),
            image_opacity: None,
        }
    }
    pub fn natural_preset() -> Self {
        Self {
            image_filter: "vintage".to_string(),
            image_position: "full-bleed".to_string(),
            image_frame: "organic".to_string(),
            image_overlay: "tint".to_string(),
            image_mix_blend: "normal".to_string(),
            image_mask: "fade-bottom".to_string(),
            image_animation: "fade-in".to_string(),
            image_content: "photography".to_string(),
            image_opacity: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStyle {
    pub border_radius: String,
    pub shadow: String,
    pub surface: String,
    pub border: String,
    pub variant: String,
}

impl Default for ComponentStyle {
    fn default() -> Self {
        Self {
            border_radius: "rounded".to_string(),
            shadow: "subtle".to_string(),
            surface: "flat".to_string(),
            border: "solid".to_string(),
            variant: "filled".to_string(),
        }
    }
}

impl ComponentStyle {
    pub fn editorial_preset() -> Self {
        Self {
            border_radius: "sharp".to_string(),
            shadow: "none".to_string(),
            surface: "textured".to_string(),
            border: "solid".to_string(),
            variant: "outlined".to_string(),
        }
    }
    pub fn bold_preset() -> Self {
        Self {
            border_radius: "material-round".to_string(),
            shadow: "dramatic".to_string(),
            surface: "gradient".to_string(),
            border: "none".to_string(),
            variant: "filled".to_string(),
        }
    }
    pub fn minimal_preset() -> Self {
        Self {
            border_radius: "sharp".to_string(),
            shadow: "none".to_string(),
            surface: "flat".to_string(),
            border: "none".to_string(),
            variant: "ghost".to_string(),
        }
    }
    pub fn dark_preset() -> Self {
        Self {
            border_radius: "squircle".to_string(),
            shadow: "colored".to_string(),
            surface: "glass".to_string(),
            border: "glow".to_string(),
            variant: "glass".to_string(),
        }
    }
    pub fn vibrant_preset() -> Self {
        Self {
            border_radius: "pill".to_string(),
            shadow: "colored".to_string(),
            surface: "gradient".to_string(),
            border: "gradient".to_string(),
            variant: "gradient".to_string(),
        }
    }
    pub fn natural_preset() -> Self {
        Self {
            border_radius: "organic".to_string(),
            shadow: "subtle".to_string(),
            surface: "paper".to_string(),
            border: "solid".to_string(),
            variant: "filled".to_string(),
        }
    }
}

pub fn resolve_image_treatment_preset(theme: &str, archetype: &str) -> ImageTreatment {
    if let Some(treatment) = image_treatment_for_theme(theme) {
        return treatment;
    }

    match archetype {
        "brand_story" | "behind_scenes" | "brand_storyteller" => ImageTreatment::natural_preset(),
        "tutorial" | "case_study" | "educator" => ImageTreatment::editorial_preset(),
        "creator" => ImageTreatment::vibrant_preset(),
        "thought_leader" => ImageTreatment::bold_preset(),
        "startup_pitch" => ImageTreatment::minimal_preset(),
        "data_analyst" => ImageTreatment::editorial_preset(),
        _ => ImageTreatment::default(),
    }
}

pub fn resolve_component_style_preset(theme: &str, archetype: &str) -> ComponentStyle {
    let mut style = ComponentStyle::default();
    match archetype {
        "brand_story" | "behind_scenes" | "brand_storyteller" => {
            style = ComponentStyle::natural_preset();
        }
        "tutorial" | "case_study" | "educator" => {
            style = ComponentStyle::editorial_preset();
        }
        "creator" => {
            style = ComponentStyle::vibrant_preset();
        }
        "thought_leader" => {
            style = ComponentStyle::bold_preset();
        }
        "startup_pitch" => {
            style = ComponentStyle::minimal_preset();
        }
        "data_analyst" => {
            style = ComponentStyle::editorial_preset();
        }
        _ => {
            style = match theme {
                "editorial" => ComponentStyle::editorial_preset(),
                "bold" => ComponentStyle::bold_preset(),
                "minimal" => ComponentStyle::minimal_preset(),
                "dark" => ComponentStyle::dark_preset(),
                "vibrant" => ComponentStyle::vibrant_preset(),
                "natural" => ComponentStyle::natural_preset(),
                _ => ComponentStyle::default(),
            };
        }
    }
    style
}

pub fn resolve_archetype_preset(
    archetype: &str,
    slide_type: &str,
) -> Option<crate::archetypes::ArchetypePreset> {
    if archetype.is_empty() {
        return None;
    }
    let arch = crate::archetypes::get_archetype(archetype)?;
    Some(crate::archetypes::get_slide_preset(&arch, slide_type))
}

pub fn render_qr_svg_data_uri(destination_url: &str) -> Result<String, String> {
    let code = QrCode::new(destination_url.as_bytes())
        .map_err(|e| format!("Failed to generate QR code: {e}"))?;
    let svg = code
        .render::<svg::Color>()
        .min_dimensions(256, 256)
        .dark_color(svg::Color("#0B0A0F"))
        .light_color(svg::Color("#FFFFFF"))
        .build();
    let encoded = svg
        .replace('#', "%23")
        .replace('<', "%3C")
        .replace('>', "%3E")
        .replace('"', "'");
    Ok(format!("data:image/svg+xml;utf8,{encoded}"))
}

pub fn render_themed_image(
    image_url: &str,
    tokens: &DesignTokens,
    treatment: &ImageTreatment,
    width: &str,
    height: &str,
    alt: &str,
    is_dark: bool,
) -> String {
    if image_url.is_empty() {
        return String::new();
    }

    let mut treatment = treatment.clone();

    // Clean up treatments based on visual context
    if !is_dark {
        treatment.image_overlay = "none".to_string();
        treatment.image_mix_blend = "normal".to_string();
    } else {
        treatment.image_mix_blend = "normal".to_string();
    }

    // Synchronize circle frame and mask to prevent conflicts
    if treatment.image_mask == "circle" || treatment.image_frame == "circle" {
        treatment.image_mask = "circle".to_string();
        treatment.image_frame = "circle".to_string();
    }

    let filter_css = image_filter_css(&treatment.image_filter, is_dark);

    // Position mapping
    let mut pos_css = "object-fit: cover;".to_string();
    match treatment.image_position.as_str() {
        "center" | "top" | "bottom" | "left" | "right" => {
            pos_css.push_str(&format!(" object-position: {};", treatment.image_position));
        }
        "full-bleed" => {
            pos_css.push_str(" object-position: center;");
        }
        _ => {}
    }

    // Frame mapping
    let mut frame_css = String::new();
    let fr = treatment.image_frame.as_str();
    if fr == "rounded" {
        frame_css = format!(
            "border-radius: {};",
            tokens
                .radii
                .get("lg")
                .cloned()
                .unwrap_or_else(|| "var(--space-1)".to_string())
        );
    } else if fr == "squircle" {
        frame_css = format!("border-radius: var(--radius-md);");
    } else if fr == "sharp" {
        frame_css = "border-radius: 0;".to_string();
    } else if fr == "pill" {
        frame_css = format!(
            "border-radius: {};",
            tokens
                .radii
                .get("lg")
                .cloned()
                .unwrap_or_else(|| "var(--radius-lg)".to_string())
        );
    } else if fr == "organic" {
        frame_css = format!(
            "border-radius: var(--radius-md) var(--radius-lg) var(--radius-md) var(--radius-lg);"
        );
    } else if fr == "circle" {
        frame_css = format!(
            "border-radius: {};",
            tokens
                .radii
                .get("lg")
                .cloned()
                .unwrap_or_else(|| "var(--radius-lg)".to_string())
        );
    } else if fr == "polaroid" {
        frame_css = format!("border: var(--space-2) solid white; box-shadow: var(--shadow-md);");
    }

    // Overlay mapping
    let mut overlay_html = String::new();
    match treatment.image_overlay.as_str() {
        "gradient" => {
            overlay_html = r#"<div style="position:absolute;inset:0;background:linear-gradient(to bottom, transparent, rgba(0,0,0,0.75));z-index:2;"></div>"#.to_string();
        }
        "solid" => {
            overlay_html = r#"<div style="position:absolute;inset:0;background:rgba(0,0,0,0.45);z-index:2;"></div>"#.to_string();
        }
        "duotone" => {
            overlay_html = r#"<div style="position:absolute;inset:0;background:linear-gradient(135deg, rgba(99, 102, 241, 0.4), rgba(236, 72, 153, 0.4));z-index:2;"></div>"#.to_string();
        }
        "vignette" => {
            overlay_html = r#"<div style="position:absolute;inset:0;background:radial-gradient(circle, transparent 50%, rgba(0,0,0,0.75) 100%);z-index:2;"></div>"#.to_string();
        }
        "tint" => {
            overlay_html = format!(
                r#"<div style="position:absolute;inset:0;background:{}4D;z-index:2;"></div>"#,
                tokens.primary
            );
        }
        _ => {}
    }

    // Mix Blend mapping
    let mut blend_css = String::new();
    let mb = treatment.image_mix_blend.as_str();
    if mb != "normal" {
        let val = if mb == "screen-lighten" {
            "lighten"
        } else {
            mb
        };
        blend_css = format!("mix-blend-mode: {};", val);
    }

    // Mask mapping
    let mut mask_css = String::new();
    match treatment.image_mask.as_str() {
        "circle" => {
            mask_css = "clip-path: circle(50% at 50% 50%);".to_string();
        }
        "fade-bottom" => {
            mask_css = "-webkit-mask-image: linear-gradient(to bottom, black 90%, transparent 100%); mask-image: linear-gradient(to bottom, black 90%, transparent 100%);".to_string();
        }
        "fade-top" => {
            mask_css = "-webkit-mask-image: linear-gradient(to top, black 90%, transparent 100%); mask-image: linear-gradient(to top, black 90%, transparent 100%);".to_string();
        }
        "fade-sides" => {
            mask_css = "-webkit-mask-image: linear-gradient(to right, transparent 0%, black 10%, black 90%, transparent 100%); mask-image: linear-gradient(to right, transparent 0%, black 10%, black 90%, transparent 100%);".to_string();
        }
        "diagonal" => {
            mask_css = "clip-path: polygon(0 0, 100% 0, 100% 85%, 0 100%);".to_string();
        }
        "wave" => {
            mask_css = "clip-path: polygon(0 0, 100% 0, 100% 85%, 80% 90%, 60% 85%, 40% 90%, 20% 85%, 0 90%);".to_string();
        }
        _ => {}
    }

    let anim_css = "";
    let opacity_css = if let Some(op) = treatment.image_opacity {
        format!("opacity:{:.2};", op)
    } else {
        "".to_string()
    };

    // Build container
    // Escape the image URL for safe insertion into HTML src="..." attribute.
    // This prevents malformed HTML if the URL contains ", <, >, or &.
    // Data URIs (data:image/...;base64,...) are safe to escape — base64
    // alphabet doesn't include those chars, and the data: scheme prefix is
    // left intact.
    let safe_image_url = escape_html(image_url);
    if fr == "polaroid" {
        format!(
            r#"<div style="position:relative;width:{};height:{};background:white;padding:var(--space-1) var(--space-1) var(--space-4) var(--space-1);box-shadow:var(--shadow-md);overflow:hidden;box-sizing:border-box;">
                <div style="position:relative;width:100%;height:100%;overflow:hidden;background:transparent;">
                    <img src="{}" alt="{}" style="display:block;width:100%;height:100%;{}{}{}{}{}{}" />
                    {}
                </div>
            </div>"#,
            width,
            height,
            safe_image_url,
            escape_html(alt),
            pos_css,
            filter_css,
            blend_css,
            mask_css,
            anim_css,
            opacity_css,
            overlay_html
        )
    } else {
        format!(
            r#"<div style="position:relative;width:{};height:{};{}{}overflow:hidden;background:transparent;box-sizing:border-box;">
                <img src="{}" alt="{}" style="display:block;width:100%;height:100%;{}{}{}{}{}" />
                {}
            </div>"#,
            width,
            height,
            frame_css,
            mask_css,
            safe_image_url,
            escape_html(alt),
            pos_css,
            filter_css,
            blend_css,
            anim_css,
            opacity_css,
            overlay_html
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Returns (open_tag, close_tag) for a glass container on dark slides.
fn get_glass_container(tokens: &DesignTokens, is_dark: bool) -> (String, String) {
    if is_dark {
        let radius = tokens
            .radii
            .get("md")
            .cloned()
            .unwrap_or_else(|| "var(--radius-md)".to_string());
        (
            format!(
                r#"<div style="background:rgba(255,255,255,0.04);border:1px solid rgba(255,255,255,0.08);backdrop-filter:var(--glass-dark-blur);-webkit-backdrop-filter:var(--glass-dark-blur);border-radius:{};padding:var(--space-8);">"#,
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
            "backdrop-filter:var(--glass-dark-blur);-webkit-backdrop-filter:var(--glass-dark-blur);".to_string(),
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
use std::cell::RefCell;

thread_local! {
    static CURRENT_THEME: RefCell<String> = RefCell::new(String::new());
    static CURRENT_ARCHETYPE: RefCell<String> = RefCell::new(String::new());
    static CURRENT_TOKENS: RefCell<Option<DesignTokens>> = RefCell::new(None);
    static CURRENT_BG_STYLE: RefCell<String> = RefCell::new(String::new());
    static CURRENT_PARAMS: RefCell<Value> = RefCell::new(json!({}));
}

fn image_treatment_for_theme(theme: &str) -> Option<ImageTreatment> {
    match theme {
        "editorial" => Some(ImageTreatment::editorial_preset()),
        "bold" => Some(ImageTreatment::bold_preset()),
        "minimal" => Some(ImageTreatment::minimal_preset()),
        "dark" => Some(ImageTreatment::dark_preset()),
        "vibrant" => Some(ImageTreatment::vibrant_preset()),
        "natural" => Some(ImageTreatment::natural_preset()),
        _ => None,
    }
}

fn apply_current_image_overrides(treatment: &mut ImageTreatment) {
    CURRENT_PARAMS.with(|params| {
        let params = params.borrow();
        let Some(obj) = params.as_object() else {
            return;
        };

        if let Some(value) = obj
            .get("image_filter")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        {
            treatment.image_filter = value.to_string();
        }
        if let Some(value) = obj
            .get("image_position")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        {
            treatment.image_position = value.to_string();
        }
        if let Some(value) = obj
            .get("image_frame")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        {
            treatment.image_frame = value.to_string();
        }
        if let Some(value) = obj
            .get("image_overlay")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        {
            treatment.image_overlay = value.to_string();
        }
        if let Some(value) = obj
            .get("image_mix_blend")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        {
            treatment.image_mix_blend = value.to_string();
        }
        if let Some(value) = obj
            .get("image_mask")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        {
            treatment.image_mask = value.to_string();
        }
        if let Some(value) = obj
            .get("image_animation")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        {
            treatment.image_animation = value.to_string();
        }
        if let Some(value) = obj
            .get("image_content")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        {
            treatment.image_content = value.to_string();
        }
    });
}

fn resolve_current_image_treatment(theme: &str, archetype: &str) -> ImageTreatment {
    let mut treatment = resolve_image_treatment_preset(theme, archetype);
    apply_current_image_overrides(&mut treatment);
    treatment
}

fn current_component_radius(tokens: &DesignTokens, role: &str) -> String {
    let theme = CURRENT_THEME.with(|t| t.borrow().clone());
    let archetype = CURRENT_ARCHETYPE.with(|a| a.borrow().clone());
    let style = resolve_component_style_preset(&theme, &archetype);

    match style.border_radius.as_str() {
        "sharp" => "0".to_string(),
        "material-round" => tokens
            .radii
            .get("lg")
            .cloned()
            .unwrap_or_else(|| "var(--radius-lg)".to_string()),
        "squircle" => "var(--radius-md)".to_string(),
        "pill" => {
            if matches!(role, "chip" | "button") {
                tokens
                    .radii
                    .get("pill")
                    .cloned()
                    .unwrap_or_else(|| "var(--radius-pill)".to_string())
            } else {
                "var(--radius-lg)".to_string()
            }
        }
        "organic" => {
            "var(--radius-md) var(--radius-lg) var(--radius-md) var(--radius-lg)".to_string()
        }
        _ => tokens
            .radii
            .get(if role == "frame" { "lg" } else { "md" })
            .cloned()
            .unwrap_or_else(|| "var(--radius-md)".to_string()),
    }
}

fn image_filter_css(filter: &str, is_dark: bool) -> &'static str {
    match filter {
        "grayscale" => {
            if is_dark {
                "filter: grayscale(82%) contrast(1.08) brightness(1.12) saturate(0.92);"
            } else {
                "filter: grayscale(78%) contrast(1.06) brightness(1.04) saturate(0.96);"
            }
        }
        "sepia" => {
            if is_dark {
                "filter: sepia(22%) saturate(1.08) contrast(1.05) brightness(1.10);"
            } else {
                "filter: sepia(18%) saturate(1.06) contrast(1.03) brightness(1.03);"
            }
        }
        "duotone-warm" => {
            if is_dark {
                "filter: sepia(24%) saturate(1.16) hue-rotate(-8deg) contrast(1.06) brightness(1.10);"
            } else {
                "filter: sepia(18%) saturate(1.12) hue-rotate(-6deg) contrast(1.04) brightness(1.03);"
            }
        }
        "duotone-cool" => {
            if is_dark {
                "filter: grayscale(42%) sepia(12%) hue-rotate(178deg) saturate(1.08) contrast(1.08) brightness(1.12);"
            } else {
                "filter: grayscale(32%) sepia(10%) hue-rotate(178deg) saturate(1.05) contrast(1.05) brightness(1.02);"
            }
        }
        "high-contrast" => {
            if is_dark {
                "filter: contrast(1.16) saturate(1.14) brightness(1.08);"
            } else {
                "filter: contrast(1.12) saturate(1.10) brightness(1.02);"
            }
        }
        "soft" => {
            if is_dark {
                "filter: contrast(0.98) saturate(0.94) brightness(1.12);"
            } else {
                "filter: contrast(0.98) saturate(0.94) brightness(1.04);"
            }
        }
        "vintage" => {
            if is_dark {
                "filter: sepia(20%) saturate(1.10) contrast(1.06) brightness(1.10);"
            } else {
                "filter: sepia(16%) saturate(1.08) contrast(1.04) brightness(1.03);"
            }
        }
        _ => "",
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

    let theme = CURRENT_THEME.with(|t| t.borrow().clone());
    let archetype = CURRENT_ARCHETYPE.with(|a| a.borrow().clone());
    let tokens_opt = CURRENT_TOKENS.with(|tok| tok.borrow().clone());
    let bg_style = CURRENT_BG_STYLE.with(|bg| bg.borrow().clone());

    if let Some(tokens) = tokens_opt {
        let mut treatment = resolve_image_treatment_preset(&theme, &archetype);
        apply_current_image_overrides(&mut treatment);

        // Calibrate opacity to preserve text contrast
        let bg_opacity = if is_dark {
            opacity.max(0.25).min(0.55)
        } else {
            opacity.max(0.06).min(0.18)
        };

        let filter_css = image_filter_css(&treatment.image_filter, is_dark);

        // Map positioning
        let mut pos_css = "background-size: cover;".to_string();
        match treatment.image_position.as_str() {
            "center" | "top" | "bottom" | "left" | "right" => {
                pos_css.push_str(&format!(
                    " background-position: {};",
                    treatment.image_position
                ));
            }
            "full-bleed" => {
                pos_css.push_str(" background-position: center;");
            }
            _ => {}
        }

        // Map mask
        let mut mask_css = "";
        match treatment.image_mask.as_str() {
            "fade-bottom" => {
                mask_css = "-webkit-mask-image: linear-gradient(to bottom, black 90%, transparent 100%); mask-image: linear-gradient(to bottom, black 90%, transparent 100%);"
            }
            "fade-top" => {
                mask_css = "-webkit-mask-image: linear-gradient(to top, black 90%, transparent 100%); mask-image: linear-gradient(to top, black 90%, transparent 100%);"
            }
            "fade-sides" => {
                mask_css = "-webkit-mask-image: linear-gradient(to right, transparent 0%, black 10%, black 90%, transparent 100%); mask-image: linear-gradient(to right, transparent 0%, black 10%, black 90%, transparent 100%);"
            }
            _ => {}
        }

        // Full-slide background images must stay flush to the slide edge.
        let frame_css = "";

        // Overlay
        let ov = treatment.image_overlay.as_str();
        let overlay_html = if is_dark {
            match ov {
                "gradient" => r#"<div style="position:absolute;inset:0;background:linear-gradient(to bottom, rgba(0,0,0,0.35), rgba(0,0,0,0.60));z-index:1;"></div>"#.to_string(),
                "solid" => r#"<div style="position:absolute;inset:0;background:rgba(0,0,0,0.45);z-index:1;"></div>"#.to_string(),
                "duotone" => format!(r#"<div style="position:absolute;inset:0;background:linear-gradient(135deg, {}55, {}44);z-index:1;"></div>"#, tokens.primary, tokens.accent),
                "vignette" => r#"<div style="position:absolute;inset:0;background:radial-gradient(circle, transparent 40%, rgba(0,0,0,0.60) 100%);z-index:1;"></div>"#.to_string(),
                "tint" => format!(r#"<div style="position:absolute;inset:0;background:{}44;z-index:1;"></div>"#, tokens.primary),
                _ => r#"<div style="position:absolute;inset:0;background:rgba(0,0,0,0.30);z-index:1;"></div>"#.to_string(),
            }
        } else {
            match ov {
                "gradient" => r#"<div style="position:absolute;inset:0;background:linear-gradient(to bottom, rgba(255,255,255,0.15), rgba(255,255,255,0.55));z-index:1;"></div>"#.to_string(),
                "solid" => r#"<div style="position:absolute;inset:0;background:rgba(255,255,255,0.45);z-index:1;"></div>"#.to_string(),
                "duotone" => format!(r#"<div style="position:absolute;inset:0;background:linear-gradient(135deg, {}33, {}22);z-index:1;"></div>"#, tokens.primary, tokens.accent),
                "vignette" => r#"<div style="position:absolute;inset:0;background:radial-gradient(circle, transparent 50%, rgba(0,0,0,0.30) 100%);z-index:1;"></div>"#.to_string(),
                "tint" => format!(r#"<div style="position:absolute;inset:0;background:{}22;z-index:1;"></div>"#, tokens.primary),
                _ => r#"<div style="position:absolute;inset:0;background:rgba(255,255,255,0.30);z-index:1;"></div>"#.to_string(),
            }
        };

        // Escape the image URL for safe insertion into CSS url('...') literal.
        // Backslash-escape single quotes and backslashes so the CSS literal
        // can't be broken. Data URIs and http(s) URLs are safe to escape.
        let safe_bg_url = image_url.replace('\\', "\\\\").replace('\'', "\\'");
        let image_div = format!(
            r#"<div style="position:absolute;inset:0;background-image:url('{}');{}opacity:{:.2};z-index:0;{}{}{}"></div>{}"#,
            safe_bg_url, pos_css, bg_opacity, filter_css, mask_css, frame_css, overlay_html
        );

        if let Some(pos) = html.find("position:relative;width:100%;height:100%;") {
            if let Some(tag_end) = html[pos..].find('>') {
                let insert_at = pos + tag_end + 1;
                let mut result = html.clone();
                result.insert_str(insert_at, &format!("\n{}", image_div));
                return result;
            }
        }
        html
    } else {
        // Fallback simple background image injector
        let bg_opacity = if is_dark {
            opacity.max(0.25).min(0.55)
        } else {
            opacity.max(0.06).min(0.18)
        };

        let filter_css = "";

        let overlay_html = if is_dark {
            r#"<div style="position:absolute;inset:0;background:linear-gradient(to bottom, rgba(0,0,0,0.35), rgba(0,0,0,0.60));z-index:1;"></div>"#
        } else {
            r#"<div style="position:absolute;inset:0;background:rgba(255,255,255,0.45);z-index:1;"></div>"#
        };

        let safe_bg_url = image_url.replace('\\', "\\\\").replace('\'', "\\'");
        let image_div = format!(
            r#"<div style="position:absolute;inset:0;background-image:url('{}');background-size:cover;background-position:center;opacity:{:.2};z-index:0;{}"></div>{}"#,
            safe_bg_url, bg_opacity, filter_css, overlay_html
        );

        if let Some(pos) = html.find("position:relative;width:100%;height:100%;") {
            if let Some(tag_end) = html[pos..].find('>') {
                let insert_at = pos + tag_end + 1;
                let mut result = html.clone();
                result.insert_str(insert_at, &format!("\n{}", image_div));
                return result;
            }
        }
        html
    }
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

    // For light hero slides with a background image, use a light glass
    // container so text has a backing and passes contrast validation.
    let (gc, gx) = if !is_dark && !background_image.is_empty() && gc.is_empty() {
        let radius = tokens
            .radii
            .get("md")
            .cloned()
            .unwrap_or_else(|| "var(--radius-md)".to_string());
        (
            format!(
                r#"<div style="background:rgba(255,255,255,0.72);backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px);border:1px solid rgba(0,0,0,0.06);border-radius:{};padding:var(--space-6);box-shadow:var(--shadow-lg);">"#,
                radius
            ),
            "</div>".to_string(),
        )
    } else {
        (gc, gx)
    };

    let effective_variant = variant;

    let html = if effective_variant == "split" {
        let headline_html = heading_block(
            headline,
            tokens,
            "title",
            None,
            true,
            Some((gradient_colors.0, gradient_colors.1)),
            "left",
            "0 0 4px",
            true,
        );
        let sub_html = if !subheadline.is_empty() {
            text_block(
                subheadline,
                tokens,
                "body",
                Some(&colors.text_secondary),
                false,
                None,
                "left",
                Some("100%"),
                "8px 0 0",
            )
        } else {
            String::new()
        };
        let left_content = format!("{}{}{}{}{}", gc, badge_html, headline_html, sub_html, gx);
        let right_visual = if !background_image.is_empty() {
            let safe_bg = background_image.replace('\\', "\\\\").replace('\'', "\\'");
            format!(
                r#"<div style="position:relative;width:100%;height:var(--space-28);border-radius:var(--radius-md);overflow:hidden;box-shadow:var(--shadow-lg);background-image:url('{}');background-size:cover;background-position:center;opacity:{};"></div>"#,
                safe_bg, image_opacity
            )
        } else {
            format!(
                r#"<div style="position:relative;width:100%;height:var(--space-28);min-height:180px;background:linear-gradient(135deg, {}12, {}08);border:1px solid {}30;border-radius:var(--radius-md);overflow:hidden;box-shadow:var(--shadow-lg);display:flex;align-items:center;justify-content:center;">
                    <div style="position:absolute;width:200px;height:200px;border-radius:50%;background:{};opacity:0.12;filter:blur(40px);-webkit-filter:blur(40px);left:-20px;top:-20px;"></div>
                    <div style="position:absolute;width:140px;height:140px;border-radius:50%;background:{};opacity:0.10;filter:blur(30px);-webkit-filter:blur(30px);right:-10px;bottom:20%;"></div>
                    <div style="position:absolute;width:80px;height:80px;border-radius:var(--radius-md);background:{};opacity:0.08;transform:rotate(12deg);left:30%;top:30%;"></div>
                    <div style="position:absolute;width:60px;height:60px;border-radius:var(--radius-lg);background:{};opacity:0.06;transform:rotate(-8deg);right:25%;bottom:25%;"></div>
                </div>"#,
                colors.primary,
                colors.primary,
                colors.primary,
                colors.primary,
                colors.button_bg,
                colors.primary,
                colors.primary
            )
        };
        split_layout(
            &left_content,
            &right_visual,
            tokens,
            bg_style,
            "var(--space-3)",
            "1.2fr 1fr",
            true,
        )
    } else {
        let align = if effective_variant == "centered" {
            "center"
        } else {
            "left"
        };
        let headline_html = heading_block(
            headline,
            tokens,
            "display",
            None,
            true,
            Some((gradient_colors.0, gradient_colors.1)),
            align,
            "0",
            true,
        );
        let sub_html = if !subheadline.is_empty() {
            text_block(
                subheadline,
                tokens,
                "body",
                Some(&colors.text_secondary),
                false,
                None,
                align,
                None,
                "8px 0 0",
            )
        } else {
            String::new()
        };
        let content = if align == "center" {
            let decor = format!(
                r#"<div style="width:var(--space-6);height:2px;background:{};margin:var(--space-2) auto;border-radius:1px;opacity:0.85;"></div>"#,
                colors.primary
            );
            format!(
                r#"<div style="display:flex;flex-direction:column;align-items:center;justify-content:center;text-align:center;width:100%;height:100%;max-width:400px;margin:0 auto;box-sizing:border-box;">{}{}{}{}{}</div>"#,
                gc, badge_html, headline_html, decor, sub_html
            )
        } else {
            format!("{}{}{}{}{}", gc, badge_html, headline_html, sub_html, gx)
        };
        hero_layout(&content, tokens, bg_style, decorations, align)
    };

    // For split variant, image is already used in right_visual — skip full-slide injection
    let html = if effective_variant == "split" && !background_image.is_empty() {
        html
    } else {
        inject_background_image(html, background_image, image_opacity, colors.is_dark)
    };

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
            colors.primary,
            tokens.heading_font,
            escape_html(number)
        )
    } else {
        String::new()
    };
    let full_title = format!("{}{}", stat_prefix, escape_html(title));

    let effective_variant = variant;
    let padding = "var(--space-10) var(--space-6) var(--space-10)";
    let justify = "center";

    let html = match effective_variant {
        "icon-left" => {
            let icon_html = icon_block(
                icon,
                tokens,
                Some(&colors.primary),
                "var(--space-7)",
                "var(--space-3)",
                "0",
            );
            let title_html = heading_block(
                &full_title,
                tokens,
                "headline",
                Some(&colors.text_primary),
                false,
                None,
                "left",
                "0 0 var(--space-1)",
                false,
            );
            let desc_html = text_block(
                description,
                tokens,
                "body",
                Some(&colors.text_secondary),
                false,
                None,
                "left",
                None,
                "0",
            );
            let content = format!(
                r#"{}<div style="display:flex;align-items:center;gap:var(--space-3);"><div style="flex-shrink:0;">{}</div><div>{}{}</div></div>{}"#,
                gc, icon_html, title_html, desc_html, gx
            );
            slide_base(&content, tokens, bg_style, false, padding, justify)
        }
        "icon-right" => {
            let icon_html = icon_block(
                icon,
                tokens,
                Some(&colors.primary),
                "var(--space-7)",
                "var(--space-3)",
                "0",
            );
            let title_html = heading_block(
                &full_title,
                tokens,
                "headline",
                Some(&colors.text_primary),
                false,
                None,
                "left",
                "0 0 var(--space-1)",
                false,
            );
            let desc_html = text_block(
                description,
                tokens,
                "body",
                Some(&colors.text_secondary),
                false,
                None,
                "left",
                None,
                "0",
            );
            let content = format!(
                r#"{}<div style="display:flex;align-items:center;gap:var(--space-3);"><div>{}{}</div><div style="flex-shrink:0;">{}</div></div>{}"#,
                gc, title_html, desc_html, icon_html, gx
            );
            slide_base(&content, tokens, bg_style, false, padding, justify)
        }
        "minimal" => {
            let title_html = heading_block(
                &full_title,
                tokens,
                "headline",
                Some(&colors.text_primary),
                false,
                None,
                "left",
                "0 0 var(--space-1)",
                false,
            );
            let desc_html = text_block(
                description,
                tokens,
                "body",
                Some(&colors.text_secondary),
                false,
                None,
                "left",
                None,
                "0",
            );
            let content = format!("{}{}{}{}", gc, title_html, desc_html, gx);
            centered_layout(&content, tokens, bg_style, false, padding, justify)
        }
        _ => {
            // stacked (default)
            let icon_html = icon_block(
                icon,
                tokens,
                Some(&colors.primary),
                "var(--space-7)",
                "var(--space-3)",
                "0 0 16px",
            );
            let title_html = heading_block(
                &full_title,
                tokens,
                "headline",
                Some(&colors.text_primary),
                false,
                None,
                "left",
                "0 0 var(--space-1)",
                false,
            );
            let desc_html = text_block(
                description,
                tokens,
                "body",
                Some(&colors.text_secondary),
                false,
                None,
                "left",
                None,
                "0",
            );
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

    let heading = heading_block(
        title,
        tokens,
        "headline",
        Some(&colors.text_primary),
        false,
        None,
        "left",
        "0 0 12px",
        true,
    );

    let (card_bg, card_border, card_blur) = card_styles(tokens, is_dark);
    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    let radius_md = current_component_radius(tokens, "card");
    let shadow_sm = tokens
        .shadows
        .get("sm")
        .cloned()
        .unwrap_or_else(|| "none".to_string());

    let effective_variant = variant;

    let content = match effective_variant {
        "card" => {
            let mut rows = String::new();
            for (i, item) in items.iter().enumerate() {
                let label = item
                    .get("label")
                    .or_else(|| item.get("title"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let sub = item
                    .get("sub")
                    .or_else(|| item.get("description"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let marker = if numbered {
                    format!(
                        r#"<span style="color:{};font-weight:700;margin-right:8px;font-size:var(--text-sm);">{}</span>"#,
                        tokens.primary,
                        i + 1
                    )
                } else {
                    String::new()
                };
                let sub_html = if !sub.is_empty() {
                    format!(
                        r#"<div style="font-size:{}px;color:{};margin-top:4px;">{}</div>"#,
                        caption_fs,
                        colors.text_secondary,
                        escape_html(sub)
                    )
                } else {
                    String::new()
                };
                rows.push_str(&format!(
                    r#"<div style="background:{};border:{};{}border-radius:{};padding:var(--space-2) 16px;margin-bottom:10px;box-shadow:{};">
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
                let label = item
                    .get("label")
                    .or_else(|| item.get("title"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let sub = item
                    .get("sub")
                    .or_else(|| item.get("description"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let marker = if numbered {
                    format!(
                        r#"<span style="color:{};font-weight:700;margin-right:6px;font-size:var(--text-sm);">{}</span>"#,
                        tokens.primary,
                        i + 1
                    )
                } else {
                    String::new()
                };
                let sub_html = if !sub.is_empty() {
                    format!(
                        r#"<div style="font-size:{}px;color:{};margin-top:2px;">{}</div>"#,
                        caption_fs,
                        colors.text_secondary,
                        escape_html(sub)
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
                r#"{}<div style="display:flex;flex-wrap:wrap;gap:var(--space-2);margin-top:16px;">{}</div>"#,
                heading, rows
            )
        }
        _ => {
            // bulleted or numbered
            let is_numbered = numbered || effective_variant == "numbered";
            let mut rows = String::new();
            for (i, item) in items.iter().enumerate() {
                let label = item
                    .get("label")
                    .or_else(|| item.get("title"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let sub = item
                    .get("sub")
                    .or_else(|| item.get("description"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let marker = if is_numbered {
                    format!(
                        r#"<span style="color:{};font-weight:700;margin-right:12px;font-size:var(--text-sm);">{}</span>"#,
                        tokens.primary,
                        i + 1
                    )
                } else {
                    let bullet_char = if matches!(theme, "editorial" | "natural" | "vibrant") {
                        "✦"
                    } else {
                        "▪"
                    };
                    format!(
                        r#"<span style="color:{};margin-right:12px;font-size:12px;line-height:1.5;">{}</span>"#,
                        tokens.primary, bullet_char
                    )
                };
                let sub_html = if !sub.is_empty() {
                    format!(
                        r#"<div style="font-size:{}px;color:{};margin-top:4px;">{}</div>"#,
                        caption_fs,
                        colors.text_secondary,
                        escape_html(sub)
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

    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        "80px var(--space-6) 80px",
        "center",
    );
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
    let radius_md = current_component_radius(tokens, "card");
    let g_styles = glass_surface(tokens, glass_variant, &radius_md);
    let shadow_lg = tokens
        .shadows
        .get("lg")
        .cloned()
        .unwrap_or_else(|| "none".to_string());

    let glass_styles_str = g_styles
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("; ");

    let glass_open = format!(
        r#"<div style="{};padding:var(--space-4);box-shadow:{};">"#,
        glass_styles_str, shadow_lg
    );
    let glass_close = "</div>";

    let effective_variant = variant;

    let html = match effective_variant {
        "left-accent" => {
            let q = format!(
                r#"<blockquote style="font-family:{};font-size:{};font-weight:{};line-height:1.2;color:{};margin:0;max-width:100%;border-left:4px solid {};padding-left:24px;text-wrap:balance;">{}</blockquote>"#,
                tokens.heading_font,
                quote_font_size,
                headline_fw,
                colors.text_primary,
                tokens.primary,
                escape_html(quote)
            );
            let attr = if !author.is_empty() {
                attribution_block(
                    author,
                    role,
                    tokens,
                    Some(&colors.text_primary),
                    "20px 0 0",
                    "left",
                )
            } else {
                String::new()
            };
            let content = format!("{}{}{}{}", glass_open, q, attr, glass_close);
            slide_base(
                &content,
                tokens,
                bg_style,
                false,
                "80px 44px 80px",
                "center",
            )
        }
        "attribution-below" => {
            let q = format!(
                r#"<blockquote style="font-family:{};font-size:{};font-weight:{};line-height:1.2;color:{};margin:0;text-align:center;text-wrap:balance;">{}</blockquote>"#,
                tokens.heading_font,
                quote_font_size,
                headline_fw,
                colors.text_primary,
                escape_html(quote)
            );
            let attr = if !author.is_empty() {
                attribution_block(
                    author,
                    role,
                    tokens,
                    Some(&colors.text_primary),
                    "0",
                    "center",
                )
            } else {
                String::new()
            };
            let content = format!(
                r#"{}{}<div style="margin-top:32px;text-align:center;">{}</div>{}"#,
                glass_open, q, attr, glass_close
            );
            slide_base(
                &content,
                tokens,
                bg_style,
                false,
                "80px 44px 80px",
                "center",
            )
        }
        _ => {
            // centered (default)
            let q = format!(
                r#"<blockquote style="font-family:{};font-size:{};font-weight:{};line-height:1.2;color:{};margin:0;text-align:center;text-wrap:balance;">{}</blockquote>"#,
                tokens.heading_font,
                quote_font_size,
                headline_fw,
                colors.text_primary,
                escape_html(quote)
            );
            let attr = if !author.is_empty() {
                attribution_block(
                    author,
                    role,
                    tokens,
                    Some(&colors.text_primary),
                    "0",
                    "center",
                )
            } else {
                String::new()
            };
            let content = format!(
                r#"{}{}<div style="margin-top:32px;text-align:center;">{}</div>{}"#,
                glass_open, q, attr, glass_close
            );
            slide_base(
                &content,
                tokens,
                bg_style,
                false,
                "80px 44px 80px",
                "center",
            )
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
        let headline_html = heading_block(
            headline,
            tokens,
            "headline",
            Some(&colors.text_primary),
            false,
            None,
            "center",
            "0 0 12px",
            true,
        );
        let sub_html = if !subtext.is_empty() {
            text_block(
                subtext,
                tokens,
                "body",
                Some(&colors.text_secondary),
                false,
                None,
                "center",
                None,
                "0 0 20px",
            )
        } else {
            String::new()
        };
        let btn = button_block(
            button_text,
            button_url,
            Some(tokens),
            Some(&colors.button_bg),
            Some(&colors.button_text),
            "0",
        );
        let content = format!(
            r#"{}<div style="text-align:center">{}{}{}</div>{}"#,
            gc, headline_html, sub_html, btn, gx
        );
        centered_layout(
            &content,
            tokens,
            bg_style,
            false,
            "80px 64px 80px",
            "center",
        )
    } else {
        let align = match effective_variant {
            "left" => "left",
            "right" => "right",
            _ => "center",
        };
        let headline_html = heading_block(
            headline,
            tokens,
            "display",
            None,
            true,
            Some((gradient_colors.0, gradient_colors.1)),
            align,
            "0 0 12px",
            true,
        );
        let sub_html = if !subtext.is_empty() {
            text_block(
                subtext,
                tokens,
                "body",
                Some(&colors.text_secondary),
                false,
                None,
                align,
                None,
                "0 0 20px",
            )
        } else {
            String::new()
        };
        let btn = button_block(
            button_text,
            button_url,
            Some(tokens),
            Some(&colors.button_bg),
            Some(&colors.button_text),
            "0",
        );
        let content = format!(
            r#"<div style="display:flex;flex-direction:column;align-items:center;justify-content:center;text-align:{};width:100%;max-width:320px;margin:0 auto;box-sizing:border-box;">{}{}{}{}{}</div>"#,
            align, gc, headline_html, sub_html, btn, gx
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
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    highlight_column: Option<usize>,
    show_checkmarks: bool,
    bg_style: &str,
    variant: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let heading = heading_block(
        title,
        tokens,
        "headline",
        Some(&colors.text_primary),
        false,
        None,
        "left",
        "0 0 12px",
        true,
    );

    let (gc, gx) = get_glass_container(tokens, is_dark);

    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let title_fs = tokens.type_scale.get("title").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    let radius_sm = tokens
        .radii
        .get("sm")
        .cloned()
        .unwrap_or_else(|| "6px".to_string());
    let radius_md = current_component_radius(tokens, "card");
    let (card_bg, card_border, card_blur) = card_styles(tokens, is_dark);

    let effective_variant = variant;
    let num_cols = columns.len().max(1);

    let content = if columns.is_empty() {
        // Fallback: render title + empty state
        format!(
            "{}{}<div style=\"font-family:{};font-size:{}px;color:{};text-align:center;padding:var(--space-6);\">No columns provided.</div>{}",
            gc, heading, tokens.body_font, body_fs, colors.text_secondary, gx
        )
    } else { match effective_variant {
        "cards" => {
            // Each row rendered as a card with column values side-by-side
            let mut cards_html = String::new();
            for row in &rows {
                let label = row.first().map(|s| s.as_str()).unwrap_or("");
                let mut values_html = String::new();
                for (ci, col_name) in columns.iter().enumerate() {
                    let val = row.get(ci + 1).map(|s| s.as_str()).unwrap_or("");
                    let is_highlighted = highlight_column == Some(ci);
                    let val_color = if is_highlighted {
                        tokens.primary.clone()
                    } else {
                        colors.text_primary.clone()
                    };
                    values_html.push_str(&format!(
                        r#"<div style="flex:1;text-align:center;">
                            <div style="font-family:{};font-size:{}px;font-weight:600;color:{};">{}</div>
                            <div style="font-family:{};font-size:{}px;color:{};margin-top:2px;">{}</div>
                        </div>"#,
                        tokens.heading_font, caption_fs, colors.text_secondary, escape_html(col_name),
                        tokens.body_font, body_fs, val_color, escape_html(val)
                    ));
                }
                cards_html.push_str(&format!(
                    r#"<div style="background:{};border:{};{}border-radius:{};padding:12px 16px;margin-bottom:10px;box-shadow:0 1px 2px rgba(0,0,0,0.05);">
                        <div style="font-family:{};font-size:{}px;font-weight:600;color:{};margin-bottom:8px;">{}</div>
                        <div style="display:flex;gap:8px;">{}</div>
                    </div>"#,
                    card_bg, card_border, card_blur, radius_md,
                    tokens.body_font, body_fs, colors.text_primary, escape_html(label),
                    values_html
                ));
            }
            format!("{}{}<div style=\"margin-top:12px;\">{}</div>{}", gc, heading, cards_html, gx)
        }
        "vs-split" => {
            // Two-column split: left column = first data column, right column = second data column
            let col_a = columns.get(0).map(|s| s.as_str()).unwrap_or("");
            let col_b = columns.get(1).map(|s| s.as_str()).unwrap_or("");
            let mut left_rows = String::new();
            let mut right_rows = String::new();
            for row in &rows {
                let label = row.first().map(|s| s.as_str()).unwrap_or("");
                let val_a = row.get(1).map(|s| s.as_str()).unwrap_or("");
                let val_b = row.get(2).map(|s| s.as_str()).unwrap_or("");
                left_rows.push_str(&format!(
                    r#"<div style="font-family:{};font-size:{}px;color:{};padding:8px 0;border-bottom:1px solid {}20;">{}<div style=\"font-weight:600;margin-top:2px;\">{}</div></div>"#,
                    tokens.body_font, body_fs, colors.text_primary, tokens.border_light, escape_html(label), escape_html(val_a)
                ));
                right_rows.push_str(&format!(
                    r#"<div style="font-family:{};font-size:{}px;color:{};padding:8px 0;border-bottom:1px solid {}20;">{}<div style=\"font-weight:600;margin-top:2px;\">{}</div></div>"#,
                    tokens.body_font, body_fs, colors.text_primary, tokens.border_light, escape_html(label), escape_html(val_b)
                ));
            }
            let left_label_color = if is_dark { "#FFFFFF".to_string() } else { tokens.primary.clone() };
            let right_label_color = colors.text_secondary.clone();
            format!(
                r#"{}{}<div style="display:grid;grid-template-columns:1fr 1fr;gap:0 24px;margin-top:16px;">
                    <div><div style="font-family:{};font-size:{}px;font-weight:700;color:{};padding-bottom:8px;border-bottom:2px solid {};margin-bottom:4px;">{}</div>{}</div>
                    <div><div style="font-family:{};font-size:{}px;font-weight:700;color:{};padding-bottom:8px;border-bottom:2px solid {};margin-bottom:4px;">{}</div>{}</div>
                </div>{}"#,
                gc, heading,
                tokens.heading_font, title_fs, left_label_color, tokens.primary, escape_html(col_a), left_rows,
                tokens.heading_font, title_fs, right_label_color, tokens.border_light, escape_html(col_b), right_rows,
                gx
            )
        }
        "feature-matrix" => {
            // Grid with checkmarks and feature labels
            let mut grid_rows = String::new();
            // Header row
            let mut header_cells = String::from(&format!(
                r#"<div style="font-family:{};font-size:{}px;font-weight:600;color:{};padding:10px 12px;text-align:left;">Feature</div>"#,
                tokens.heading_font, caption_fs, colors.text_secondary
            ));
            for (ci, col) in columns.iter().enumerate() {
                let is_hl = highlight_column == Some(ci);
                let hdr_color = if is_hl { tokens.primary.clone() } else { colors.text_secondary.clone() };
                header_cells.push_str(&format!(
                    r#"<div style="font-family:{};font-size:{}px;font-weight:700;color:{};padding:10px 12px;text-align:center;">{}</div>"#,
                    tokens.heading_font, caption_fs, hdr_color, escape_html(col)
                ));
            }
            grid_rows.push_str(&format!(
                r#"<div style="display:contents;">{}</div>"#, header_cells
            ));
            // Data rows
            for row in &rows {
                let label = row.first().map(|s| s.as_str()).unwrap_or("");
                let mut cells = format!(
                    r#"<div style="font-family:{};font-size:{}px;color:{};padding:10px 12px;border-top:1px solid {}20;text-align:left;">{}</div>"#,
                    tokens.body_font, body_fs, colors.text_primary, tokens.border_light, escape_html(label)
                );
                for (ci, _) in columns.iter().enumerate() {
                    let val = row.get(ci + 1).map(|s| s.as_str()).unwrap_or("");
                    let is_hl = highlight_column == Some(ci);
                    let display_val = if show_checkmarks && (val == "✓" || val == "✔") {
                        format!(r#"<span style="color:{};font-size:18px;font-weight:700;">✓</span>"#, tokens.primary)
                    } else if show_checkmarks && (val == "—" || val == "-") {
                        format!(r#"<span style="color:{};font-size:14px;">—</span>"#, colors.text_secondary)
                    } else {
                        let cell_color = if is_hl { tokens.primary.clone() } else { colors.text_primary.clone() };
                        format!(r#"<span style="color:{};">{}</span>"#, cell_color, escape_html(val))
                    };
                    cells.push_str(&format!(
                        r#"<div style="font-family:{};font-size:{}px;padding:10px 12px;border-top:1px solid {}20;text-align:center;font-weight:500;">{}</div>"#,
                        tokens.body_font, body_fs, tokens.border_light, display_val
                    ));
                }
                grid_rows.push_str(&format!(
                    r#"<div style="display:contents;">{}</div>"#, cells
                ));
            }
            format!(
                r#"{}{}<div style="display:grid;grid-template-columns:{};margin-top:16px;width:100%;box-sizing:border-box;">
                    {}
                </div>{}"#,
                gc, heading,
                format!("auto {}", "1fr ".repeat(num_cols.saturating_sub(1).max(1))),
                grid_rows, gx
            )
        }
        _ => {
            // table (default) — clean data table with header row
            let mut grid_rows = String::new();
            // Header
            let mut header_cells = String::new();
            for (ci, col) in columns.iter().enumerate() {
                let is_hl = highlight_column == Some(ci);
                let hdr_color = if is_hl { tokens.primary.clone() } else { colors.text_primary.clone() };
                let border_bottom = if is_hl { format!("border-bottom:2px solid {}", tokens.primary) } else { "border-bottom:1px solid".to_string() };
                header_cells.push_str(&format!(
                    r#"<div style="font-family:{};font-size:{}px;font-weight:700;color:{};padding:12px 16px;{} {}20;display:flex;align-items:center;min-height:42px;box-sizing:border-box;">{}</div>"#,
                    tokens.heading_font, title_fs, hdr_color, border_bottom, tokens.border_light, escape_html(col)
                ));
            }
            grid_rows.push_str(&header_cells);
            // Data rows
            for row in &rows {
                for ci in 0..num_cols {
                    let val = row.get(ci).map(|s| s.as_str()).unwrap_or("");
                    let is_hl = highlight_column == Some(ci);
                    let cell_color = if is_hl { tokens.primary.clone() } else { colors.text_primary.clone() };
                    let display_val = if show_checkmarks && (val == "✓" || val == "✔") {
                        format!(r#"<span style="color:{};font-size:16px;font-weight:700;">✓</span>"#, tokens.primary)
                    } else if show_checkmarks && (val == "—" || val == "-") {
                        format!(r#"<span style="color:{};">—</span>"#, colors.text_secondary)
                    } else {
                        escape_html(val)
                    };
                    grid_rows.push_str(&format!(
                        r#"<div style="font-family:{};font-size:{}px;color:{};padding:12px 16px;border-bottom:1px solid {}20;display:flex;align-items:center;min-height:42px;box-sizing:border-box;">{}</div>"#,
                        tokens.body_font, body_fs, cell_color, tokens.border_light, display_val
                    ));
                }
            }
            format!(
                r#"{}{}<div style="display:grid;grid-template-columns:{};margin-top:16px;width:100%;box-sizing:border-box;align-items:stretch;">
                    {}
                </div>{}"#,
                gc, heading,
                "1fr ".repeat(num_cols.max(1)),
                grid_rows,                gx
            )
        }
    } };

    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        "80px var(--space-6) 80px",
        "center",
    );
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

    let heading = heading_block(
        title,
        tokens,
        "headline",
        Some(&colors.text_primary),
        false,
        None,
        "left",
        "0 0 12px",
        true,
    );

    let (card_bg, card_border, card_blur) = card_styles(tokens, is_dark);

    let micro_fs = tokens
        .type_scale
        .get("micro")
        .map(|s| s.font_size)
        .unwrap_or(10);
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    let title_fs = tokens.type_scale.get("title").unwrap().font_size;
    let headline_fs = tokens.type_scale.get("headline").unwrap().font_size;
    let display_fs = tokens.type_scale.get("display").unwrap().font_size;
    let radius_md = tokens
        .radii
        .get("md")
        .cloned()
        .unwrap_or_else(|| "var(--space-1)".to_string());
    let radius_sm = tokens
        .radii
        .get("sm")
        .cloned()
        .unwrap_or_else(|| "6px".to_string());
    let shadow_sm = tokens
        .shadows
        .get("sm")
        .cloned()
        .unwrap_or_else(|| "none".to_string());
    let shadow_md = tokens
        .shadows
        .get("md")
        .cloned()
        .unwrap_or_else(|| "none".to_string());

    let label_color = if is_dark {
        &colors.text_secondary
    } else {
        &colors.text_primary
    };

    let effective_variant = variant;

    let grid = match effective_variant {
        "horizontal" => {
            let mut g = format!(
                r#"<div style="display:flex;gap:var(--space-2);margin-top:16px;overflow:hidden;">"#
            );
            for item in &stats {
                let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
                let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                let sub = item.get("sub").and_then(|v| v.as_str()).unwrap_or("");
                let sub_html = if !sub.is_empty() {
                    format!(
                        r#"<div style="font-size:{}px;color:{};margin-top:4px;">{}</div>"#,
                        micro_fs,
                        colors.text_secondary,
                        escape_html(sub)
                    )
                } else {
                    String::new()
                };
                g.push_str(&format!(
                    r#"<div style="flex:1;text-align:center;padding:var(--space-1);background:{};border:{};{}border-radius:{};">
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
            let mut g =
                format!(r#"<div style="display:flex;gap:var(--space-1);margin-top:12px;">"#);
            for item in &stats {
                let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
                let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                let sub = item.get("sub").and_then(|v| v.as_str()).unwrap_or("");
                let sub_html = if !sub.is_empty() {
                    format!(
                        r#"<div style="font-size:{}px;color:{};margin-top:2px;">{}</div>"#,
                        micro_fs,
                        colors.text_secondary,
                        escape_html(sub)
                    )
                } else {
                    String::new()
                };
                g.push_str(&format!(
                    r#"<div style="flex:1;padding:var(--space-1) var(--space-2);background:{};border:{};{}border-radius:{};text-align:center;">
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
            let mut g =
                format!(r#"<div style="display:flex;gap:var(--space-2);margin-top:20px;">"#);
            for item in &stats {
                let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
                let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                let sub = item.get("sub").and_then(|v| v.as_str()).unwrap_or("");
                let sub_html = if !sub.is_empty() {
                    format!(
                        r#"<div style="font-size:{}px;color:{};margin-top:8px;">{}</div>"#,
                        caption_fs,
                        colors.text_secondary,
                        escape_html(sub)
                    )
                } else {
                    String::new()
                };
                g.push_str(&format!(
                    r#"<div style="flex:1;background:{};border:{};{}border-radius:{};padding:var(--space-3) 20px;box-shadow:{};text-align:center;">
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
                let mut g = format!(
                    r#"<div style="display:flex;gap:var(--space-1);margin-top:16px;overflow:hidden;align-items:stretch;">"#
                );
                for item in &stats {
                    let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
                    let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                    let sub = item.get("sub").and_then(|v| v.as_str()).unwrap_or("");
                    let sub_html = if !sub.is_empty() {
                        format!(
                            r#"<div style="font-size:{}px;color:{};margin-top:4px;">{}</div>"#,
                            micro_fs,
                            colors.text_secondary,
                            escape_html(sub)
                        )
                    } else {
                        String::new()
                    };
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
                let mut g = format!(
                    r#"<div style="display:flex;gap:var(--space-2);flex-wrap:wrap;width:100%;margin-top:16px;box-sizing:border-box;">"#
                );
                for item in &stats {
                    let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
                    let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                    let sub = item.get("sub").and_then(|v| v.as_str()).unwrap_or("");
                    let sub_html = if !sub.is_empty() {
                        format!(
                            r#"<div style="font-size:{}px;color:{};margin-top:4px;">{}</div>"#,
                            micro_fs,
                            colors.text_secondary,
                            escape_html(sub)
                        )
                    } else {
                        String::new()
                    };
                    g.push_str(&format!(
                        r#"<div style="width:calc(50% - 8px);box-sizing:border-box;min-width:120px;background:{};border:{};{}border-radius:{};padding:var(--space-1);box-shadow:{};">
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
    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        "80px var(--space-6) 80px",
        "center",
    );
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

    let heading = heading_block(
        title,
        tokens,
        "headline",
        Some(&colors.text_primary),
        false,
        None,
        "left",
        "0 0 12px",
        true,
    );

    let (gc, gx) = get_glass_container(tokens, is_dark);

    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;

    let effective_variant = variant;

    let steps_html = match effective_variant {
        "horizontal" => {
            let mut s = format!(
                r#"<div style="display:flex;gap:var(--space-2);margin-top:16px;overflow:hidden;">"#
            );
            for (i, step) in steps.iter().enumerate() {
                let step_title = step.get("title").and_then(|v| v.as_str()).unwrap_or("");
                let step_desc = step
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                s.push_str(&format!(
                    r#"<div style="flex:1;text-align:center;">
                        <div style="width:32px;height:32px;border-radius:50%;background:{}12;border:2px solid {};display:inline-flex;align-items:center;justify-content:center;font-size:var(--text-sm);font-weight:700;color:{};margin-bottom:8px;">{}</div>
                        <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0 0 4px;overflow-wrap:break-word;">{}</h3>
                        <p style="font-family:{};font-size:{}px;color:{};margin:0;line-height:1.45;">{}</p>
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
                let step_desc = step
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                s.push_str(&format!(
                    r#"<div style="display:flex;gap:var(--space-1);align-items:flex-start;margin-bottom:10px;">
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
                let step_desc = step
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
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
                        <div style="width:36px;height:36px;border-radius:50%;background:{}12;border:2px solid {};display:flex;align-items:center;justify-content:center;font-size:var(--text-sm);font-weight:700;color:{};flex-shrink:0;z-index:2;">{}</div>
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
    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        "80px var(--space-6) 80px",
        "center",
    );
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
        "danger" => ("#EF4444", "#EF444410"),
        _ => (tokens.primary.as_str(), ""), // info — use primary
    };

    let glass_variant = if is_dark { "dark" } else { "light" };
    let radius_lg = tokens
        .radii
        .get("lg")
        .cloned()
        .unwrap_or_else(|| "var(--space-1)".to_string());
    let g_styles = glass_surface(tokens, glass_variant, &radius_lg);
    let shadow_md = tokens
        .shadows
        .get("md")
        .cloned()
        .unwrap_or_else(|| "none".to_string());

    // Build callout glass style string and augment with left border
    let callout_styles_str = {
        let mut parts: Vec<String> = g_styles
            .iter()
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
        r#"<div style="position:absolute;right:var(--space-5);bottom:var(--space-4);font-size:240px;opacity:0.04;pointer-events:none;z-index:1;user-select:none;transform:rotate(12deg);line-height:1;font-family:system-ui;">{}</div>"#,
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
        accent_color,
        escape_html(variant),
        tokens.heading_font,
        title_fs,
        colors.text_primary,
        escape_html(title),
        tokens.body_font,
        body_fs,
        colors.text_secondary,
        escape_html(text)
    );

    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        "80px var(--space-6) 80px",
        "center",
    );
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
    left_content_html: &str,
    image_url: &str,
    bg_style: &str,
    variant: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
    archetype: &str,
    padding: &str,
) -> Value {
    let arch_preset = resolve_archetype_preset(archetype, "split_features");
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let heading = heading_block(
        title,
        tokens,
        "headline",
        Some(&colors.text_primary),
        false,
        None,
        "left",
        "0 0 12px",
        true,
    );

    let use_glass = arch_preset
        .as_ref()
        .map(|ap| ap.glass)
        .unwrap_or(theme == "dark")
        || is_dark;
    let (gc, gx) = get_glass_container(tokens, use_glass);

    let mut effective_variant = variant.to_string();
    if let Some(ref ap) = arch_preset {
        if !ap.variant.is_empty() && variant == "default" {
            effective_variant = ap.variant.clone();
        }
    }

    let mut effective_img = image_url.to_string();
    if effective_img.is_empty() && left_content_html.is_empty() {
        effective_img = "https://images.unsplash.com/photo-1460925895917-afdab827c52f".to_string();
    }

    let left_visual = if !effective_img.is_empty() {
        let mut treatment = resolve_current_image_treatment(theme, archetype);
        if treatment.image_frame == "circle"
            || treatment.image_frame == "pill"
            || treatment.image_frame == "none"
        {
            treatment.image_frame = "rounded".to_string();
        }
        // Image fills the entire left column height (true 50/50 split).
        // The parent grid cell controls the height; the image uses 100%.
        render_themed_image(
            &effective_img,
            tokens,
            &treatment,
            "100%",
            "100%",
            title,
            is_dark,
        )
    } else {
        let mut visual = left_content_html.trim().to_string();
        if (visual.starts_with("<div") && visual.contains("font-size") && visual.len() < 180)
            || visual.len() < 10
        {
            let c_bg = if is_dark {
                "rgba(255,255,255,0.03)"
            } else {
                "rgba(0,0,0,0.02)"
            };
            let c_border = if is_dark {
                "1px solid rgba(255,255,255,0.08)".to_string()
            } else {
                format!("1px solid {}30", colors.border)
            };
            visual = format!(
                r#"<div style="background:{};border:{};border-radius:{};height:260px;display:flex;align-items:center;justify-content:center;box-shadow:{};box-sizing:border-box;position:relative;overflow:hidden;">
                    <div style="position:absolute;width:120px;height:120px;border-radius:50%;background:{};opacity:0.08;filter:blur(30px);-webkit-filter:blur(30px);"></div>
                    <div style="position:relative;z-index:2;transform:scale(1.2);">{}</div>
                </div>"#,
                c_bg,
                c_border,
                tokens
                    .radii
                    .get("lg")
                    .cloned()
                    .unwrap_or_else(|| "var(--space-1)".to_string()),
                tokens
                    .shadows
                    .get("sm")
                    .cloned()
                    .unwrap_or_else(|| "none".to_string()),
                colors.primary,
                left_content_html
            );
        }
        visual
    };

    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;

    let feature_card_bg = if is_dark {
        "rgba(255,255,255,0.06)"
    } else {
        "rgba(255,255,255,0.92)"
    };
    let feature_radius = current_component_radius(tokens, "card");
    let feature_shadow = tokens
        .shadows
        .get("sm")
        .cloned()
        .unwrap_or_else(|| "none".to_string());
    let image_feature_layout = !effective_img.is_empty();
    let mut feature_cards = Vec::new();
    for (idx, feat) in features.iter().enumerate() {
        let t = feat.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let d = feat
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let icon = feat
            .get("icon")
            .and_then(|v| v.as_str())
            .unwrap_or(if idx == 0 { "✦" } else { "→" });
        let badge_size = if image_feature_layout { 26 } else { 30 };
        let badge = visual_badge_html(tokens, &colors, &json!({"icon": icon}), t, badge_size);
        let card_padding = if image_feature_layout {
            "14px"
        } else {
            "var(--space-1)"
        };
        let card_gap = if image_feature_layout {
            "12px"
        } else {
            "var(--space-1)"
        };
        let card_margin = if image_feature_layout {
            "0"
        } else {
            "0 0 12px"
        };
        let title_size = if image_feature_layout {
            body_fs.saturating_sub(2)
        } else {
            body_fs
        };
        let desc_size = if image_feature_layout {
            caption_fs.saturating_sub(1)
        } else {
            caption_fs
        };
        feature_cards.push(format!(
            r#"<div style="background:{};border:1px solid {};border-radius:{};box-shadow:{};padding:{};display:flex;gap:{};align-items:flex-start;margin:{};box-sizing:border-box;min-width:0;">
                {}
                <div style="min-width:0;">
                    <h3 style="font-family:{};font-size:{}px;font-weight:800;color:{};margin:0 0 5px;line-height:1.2;overflow-wrap:break-word;word-break:break-word;">{}</h3>
                    <p style="font-family:{};font-size:{}px;color:{};margin:0;line-height:1.45;">{}</p>
                </div>
            </div>"#,
            feature_card_bg,
            colors.border,
            feature_radius,
            feature_shadow,
            card_padding,
            card_gap,
            card_margin,
            badge,
            tokens.body_font, title_size, colors.text_primary, escape_html(t),
            tokens.body_font, desc_size, colors.text_secondary, escape_html(d)
        ));
    }
    let features_html = feature_cards.join("");

    let content = if image_feature_layout {
        // True 50/50 split: image fills the left column full-height, while
        // the heading + feature cards stack in the right column. This fixes
        // the "ugly asymmetric layout" where the heading floated on top and
        // the two cards carried all the weight. Now image and text share
        // equal compositional weight (50/50), and the heading is anchored
        // inside the text column — not floating above everything.
        format!(
            r#"<div style="display:grid;grid-template-columns:1fr 1fr;gap:20px;width:100%;height:100%;align-items:stretch;overflow:hidden;">
                <div style="min-width:0;min-height:0;overflow:hidden;display:flex;align-items:stretch;">{}</div>
                <div style="min-width:0;min-height:0;overflow:hidden;display:flex;flex-direction:column;justify-content:center;gap:14px;">
                    {}
                    <div style="display:flex;flex-direction:column;gap:10px;overflow:hidden;">{}</div>
                </div>
            </div>"#,
            left_visual, heading, features_html
        )
    } else if effective_variant == "reversed" {
        format!(
            r#"{}{}
            <div style="display:grid;grid-template-columns:1.02fr 1fr;gap:var(--space-2);margin-top:16px;align-items:center;overflow:hidden;">
                <div>{}</div>
                <div>{}</div>
            </div>{}"#,
            gc, heading, features_html, left_visual, gx
        )
    } else if effective_variant == "stacked" {
        format!(
            r#"{}{}
            <div style="margin-top:16px;">{}</div>
            <div style="display:grid;grid-template-columns:1fr 1fr;gap:var(--space-2);margin-top:16px;overflow:hidden;">{}</div>{}"#,
            gc, heading, left_visual, features_html, gx
        )
    } else {
        format!(
            r#"{}{}
            <div style="display:grid;grid-template-columns:1fr 1.02fr;gap:var(--space-2);margin-top:16px;align-items:start;overflow:hidden;">
                <div style="min-width:0;overflow:hidden;">{}</div>
                <div style="min-width:0;overflow:hidden;">{}</div>
            </div>{}"#,
            gc, heading, left_visual, features_html, gx
        )
    };

    let padding_val = if padding.is_empty() {
        if image_feature_layout {
            "48px 36px 56px"
        } else {
            "80px var(--space-6) 80px"
        }
    } else {
        padding
    };
    let html = slide_base(&content, tokens, bg_style, false, padding_val, "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": effective_variant,
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 11. grid_cards_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Card grid for features, services, or offerings.
///
/// Each card in `cards` is a JSON object with `icon`, `title`, `description`.
pub fn grid_cards_slide(
    tokens: &DesignTokens,
    title: &str,
    cards: Vec<Value>,
    bg_style: &str,
    variant: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
    archetype: &str,
    padding: &str,
) -> Value {
    let arch_preset = resolve_archetype_preset(archetype, "grid_cards");
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let heading = heading_block(
        title,
        tokens,
        "headline",
        Some(&colors.text_primary),
        false,
        None,
        "left",
        "0 0 12px",
        true,
    );

    let use_glass = arch_preset
        .as_ref()
        .map(|ap| ap.glass)
        .unwrap_or(theme == "dark")
        || is_dark;
    let (card_bg, card_border, card_blur) = if use_glass {
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
    };

    let mut effective_variant = variant.to_string();
    if let Some(ref ap) = arch_preset {
        if !ap.variant.is_empty() && variant == "default" {
            effective_variant = ap.variant.clone();
        }
    }

    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    let title_fs = tokens.type_scale.get("title").unwrap().font_size;
    let radius_md = tokens
        .radii
        .get("md")
        .cloned()
        .unwrap_or_else(|| "var(--space-1)".to_string());
    let shadow_sm = tokens
        .shadows
        .get("sm")
        .cloned()
        .unwrap_or_else(|| "none".to_string());

    let render_single_card = |card: &Value,
                              card_padding: &str,
                              icon_size: u32,
                              font_size_title: i32,
                              font_size_caption: i32|
     -> String {
        let ico = card.get("icon").and_then(|v| v.as_str()).unwrap_or("⚡");
        let t = card.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let d = card
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let desc_html = if !d.is_empty() {
            format!(
                r#"<p style="font-family:{};font-size:{}px;color:{};margin:0;line-height:1.45;overflow-wrap:break-word;word-break:break-word;">{}</p>"#,
                tokens.body_font,
                font_size_caption,
                colors.text_secondary,
                escape_html(d)
            )
        } else {
            String::new()
        };

        let icon_color = card
            .get("icon_color")
            .and_then(|v| v.as_str())
            .unwrap_or(&colors.primary);
        let icon_html = crate::blocks::render_icon(ico, icon_color, icon_size);

        format!(
            r#"<div style="flex:1;min-width:0;background:{};border:{};{}border-radius:{};padding:{};box-shadow:{};display:flex;flex-direction:column;box-sizing:border-box;">
                <div style="margin-bottom:10px;display:flex;align-items:center;">{}</div>
                <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0 0 6px;line-height:1.2;overflow-wrap:break-word;word-break:break-word;">{}</h3>
                {}
            </div>"#,
            card_bg,
            card_border,
            card_blur,
            radius_md,
            card_padding,
            shadow_sm,
            icon_html,
            tokens.body_font,
            font_size_title,
            colors.text_primary,
            escape_html(t),
            desc_html
        )
    };

    let max_title_len = cards
        .iter()
        .map(|c| c.get("title").and_then(|v| v.as_str()).unwrap_or("").len())
        .max()
        .unwrap_or(0);
    let max_desc_len = cards
        .iter()
        .map(|c| {
            c.get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .len()
        })
        .max()
        .unwrap_or(0);

    let card_html = if effective_variant == "2-col" {
        let mut items_html = String::new();
        let t_fs = if max_title_len > 15 {
            title_fs - 2
        } else {
            title_fs
        };
        let c_fs = if max_desc_len > 60 {
            caption_fs - 1
        } else {
            caption_fs
        };
        for card in cards.iter().take(2) {
            items_html.push_str(&render_single_card(card, "24px 20px", 28, t_fs, c_fs));
        }
        format!(
            r#"<div style="display:flex;gap:var(--space-2);width:100%;margin-top:16px;">{}</div>"#,
            items_html
        )
    } else if effective_variant == "3-col" {
        let mut items_html = String::new();
        let t_fs = if max_title_len > 12 { 13 } else { 14 };
        let c_fs = if max_desc_len > 40 { 10 } else { 11 };
        let card_padding = if max_desc_len > 40 {
            "12px 8px"
        } else {
            "16px 12px"
        };
        let icon_size = if max_desc_len > 40 { 18 } else { 20 };
        for card in cards.iter().take(3) {
            items_html.push_str(&render_single_card(
                card,
                card_padding,
                icon_size,
                t_fs,
                c_fs,
            ));
        }
        format!(
            r#"<div style="display:grid;grid-template-columns:repeat(3, 1fr);gap:var(--space-2);width:100%;margin-top:16px;">{}</div>"#,
            items_html
        )
    } else if effective_variant == "4-col" {
        let mut items_html = String::new();
        let has_descriptions = cards.iter().any(|c| {
            c.get("description")
                .and_then(|v| v.as_str())
                .map(|s| !s.is_empty())
                .unwrap_or(false)
        });
        let (t_fs, c_fs, card_padding, icon_size) = if has_descriptions {
            (
                if max_title_len > 12 { 14 } else { 16 },
                if max_desc_len > 50 { 11 } else { 12 },
                "16px 14px",
                22,
            )
        } else {
            (if max_title_len > 10 { 11 } else { 12 }, 10, "10px 6px", 16)
        };
        for card in cards.iter().take(4) {
            items_html.push_str(&render_single_card(
                card,
                card_padding,
                icon_size,
                t_fs,
                c_fs,
            ));
        }
        if has_descriptions {
            format!(
                r#"<div style="display:grid;grid-template-columns:repeat(2, 1fr);gap:14px;width:100%;margin-top:16px;">{}</div>"#,
                items_html
            )
        } else {
            format!(
                r#"<div style="display:flex;gap:var(--space-1);width:100%;margin-top:16px;">{}</div>"#,
                items_html
            )
        }
    } else if effective_variant == "masonry" {
        let mut items_html = String::new();
        for (i, card) in cards.iter().take(4).enumerate() {
            let ico = card.get("icon").and_then(|v| v.as_str()).unwrap_or("⚡");
            let t = card.get("title").and_then(|v| v.as_str()).unwrap_or("");
            let d = card
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let width = if i < 2 {
                "calc(50% - 8px)"
            } else {
                "calc(33.33% - 11px)"
            };
            let desc_html = if !d.is_empty() {
                format!(
                    r#"<p style="font-family:{};font-size:{}px;color:{};margin:0;line-height:1.5;overflow-wrap:break-word;word-break:break-word;">{}</p>"#,
                    tokens.body_font,
                    caption_fs,
                    colors.text_secondary,
                    escape_html(d)
                )
            } else {
                String::new()
            };
            let icon_color = card
                .get("icon_color")
                .and_then(|v| v.as_str())
                .unwrap_or(&colors.primary);
            let icon_html = crate::blocks::render_icon(ico, icon_color, 20);
            items_html.push_str(&format!(
                r#"<div style="width:{};background:{};border:{};{}border-radius:{};padding:16px;box-shadow:{};display:flex;flex-direction:column;box-sizing:border-box;">
                    <div style="margin-bottom:8px;display:flex;align-items:center;">{}</div>
                    <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0 0 6px;">{}</h3>
                    {}
                </div>"#,
                width, card_bg, card_border, card_blur, radius_md, shadow_sm,
                icon_html,
                tokens.body_font, caption_fs, colors.text_primary, escape_html(t),
                desc_html
            ));
        }
        format!(
            r#"<div style="display:flex;flex-wrap:wrap;gap:var(--space-2);margin-top:16px;width:100%;">{}</div>"#,
            items_html
        )
    } else {
        if cards.len() >= 4 {
            // Default for 4+ cards: 2x2 grid
            let mut items_html = String::new();
            let t_fs4 = if max_title_len > 12 { 14 } else { 16 };
            let c_fs4 = if max_desc_len > 50 { 11 } else { 12 };
            for card in cards.iter().take(4) {
                items_html.push_str(&render_single_card(card, "16px 14px", 22, t_fs4, c_fs4));
            }
            format!(
                r#"<div style="display:grid;grid-template-columns:repeat(2, 1fr);gap:14px;width:100%;margin-top:16px;">{}</div>"#,
                items_html
            )
        } else if cards.len() == 3 {
            // Default for 3 cards: 3 columns
            let t_fs3 = if max_title_len > 12 { 13 } else { 14 };
            let c_fs3 = if max_desc_len > 40 { 10 } else { 11 };
            let pad3 = if max_desc_len > 40 {
                "12px 8px"
            } else {
                "16px 12px"
            };
            let ico3 = if max_desc_len > 40 { 18 } else { 20 };
            let mut items_html = String::new();
            for card in cards.iter().take(3) {
                items_html.push_str(&render_single_card(card, pad3, ico3, t_fs3, c_fs3));
            }
            format!(
                r#"<div style="display:grid;grid-template-columns:repeat(3, 1fr);gap:var(--space-2);width:100%;margin-top:16px;">{}</div>"#,
                items_html
            )
        } else if cards.len() == 2 {
            let card1 = &cards[0];
            let card2 = &cards[1];

            let ico1 = card1.get("icon").and_then(|v| v.as_str()).unwrap_or("⚡");
            let t1 = card1.get("title").and_then(|v| v.as_str()).unwrap_or("");
            let d1 = card1
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let desc1 = if !d1.is_empty() {
                format!(
                    r#"<p style="font-family:{};font-size:{}px;color:{};margin:var(--space-1) 0 0;line-height:1.5;">{}</p>"#,
                    tokens.body_font,
                    caption_fs,
                    colors.text_secondary,
                    escape_html(d1)
                )
            } else {
                String::new()
            };

            let ico2 = card2.get("icon").and_then(|v| v.as_str()).unwrap_or("⚡");
            let t2 = card2.get("title").and_then(|v| v.as_str()).unwrap_or("");
            let d2 = card2
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let desc2 = if !d2.is_empty() {
                format!(
                    r#"<p style="font-family:{};font-size:{}px;color:{};margin:var(--space-1) 0 0;line-height:1.5;">{}</p>"#,
                    tokens.body_font,
                    caption_fs,
                    colors.text_secondary,
                    escape_html(d2)
                )
            } else {
                String::new()
            };

            let icon_color1 = card1
                .get("icon_color")
                .and_then(|v| v.as_str())
                .unwrap_or(&colors.primary);
            let icon_color2 = card2
                .get("icon_color")
                .and_then(|v| v.as_str())
                .unwrap_or(&colors.primary);
            let icon_html1 = crate::blocks::render_icon(ico1, icon_color1, 32);
            let icon_html2 = crate::blocks::render_icon(ico2, icon_color2, 32);

            format!(
                r#"<div style="display:grid;grid-template-columns:2fr 1fr;gap:var(--space-2);width:100%;margin-top:16px;">
                    <div style="background:{};border:{};{}border-radius:{};padding:var(--space-3) 20px;box-shadow:{};display:flex;flex-direction:column;box-sizing:border-box;min-width:0;">
                        <div style="margin-bottom:16px;display:flex;align-items:center;">{}</div>
                        <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0;">{}</h3>
                        {}
                    </div>
                    <div style="background:{};border:{};{}border-radius:{};padding:var(--space-3) 20px;box-shadow:{};display:flex;flex-direction:column;box-sizing:border-box;min-width:0;">
                        <div style="margin-bottom:16px;display:flex;align-items:center;">{}</div>
                        <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0;">{}</h3>
                        {}
                    </div>
                </div>"#,
                card_bg,
                card_border,
                card_blur,
                radius_md,
                shadow_sm,
                icon_html1,
                tokens.body_font,
                title_fs,
                colors.text_primary,
                escape_html(t1),
                desc1,
                card_bg,
                card_border,
                card_blur,
                radius_md,
                shadow_sm,
                icon_html2,
                tokens.body_font,
                title_fs,
                colors.text_primary,
                escape_html(t2),
                desc2
            )
        } else if !cards.is_empty() {
            let card = &cards[0];
            let ico = card.get("icon").and_then(|v| v.as_str()).unwrap_or("⚡");
            let t = card.get("title").and_then(|v| v.as_str()).unwrap_or("");
            let d = card
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let desc_html = if !d.is_empty() {
                format!(
                    r#"<p style="font-family:{};font-size:{}px;color:{};margin:var(--space-1) 0 0;line-height:1.5;">{}</p>"#,
                    tokens.body_font,
                    caption_fs,
                    colors.text_secondary,
                    escape_html(d)
                )
            } else {
                String::new()
            };
            let icon_color = card
                .get("icon_color")
                .and_then(|v| v.as_str())
                .unwrap_or(&colors.primary);
            let icon_html = crate::blocks::render_icon(ico, icon_color, 32);
            format!(
                r#"<div style="background:{};border:{};{}border-radius:{};padding:var(--space-3) 20px;box-shadow:{};display:flex;flex-direction:column;width:100%;margin-top:16px;box-sizing:border-box;min-width:0;">
                    <div style="margin-bottom:16px;display:flex;align-items:center;">{}</div>
                    <h3 style="font-family:{};font-size:{}px;font-weight:600;color:{};margin:0;">{}</h3>
                    {}
                </div>"#,
                card_bg,
                card_border,
                card_blur,
                radius_md,
                shadow_sm,
                icon_html,
                tokens.body_font,
                title_fs,
                colors.text_primary,
                escape_html(t),
                desc_html
            )
        } else {
            String::new()
        }
    };

    let content = format!(
        r#"<div style="width:100%;box-sizing:border-box;max-width:100%;overflow:hidden;">{}{}</div>"#,
        heading, card_html
    );
    let padding_val = if padding.is_empty() {
        "80px 48px 80px"
    } else {
        padding
    };
    let html = slide_base(&content, tokens, bg_style, false, padding_val, "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": effective_variant,
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
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let gradient_colors = if is_dark {
        ("#FFFFFF", colors.text_primary.as_str())
    } else {
        (colors.text_primary.as_str(), colors.text_secondary.as_str())
    };

    let heading_html = heading_block(
        headline,
        tokens,
        "display",
        None,
        true,
        Some((gradient_colors.0, gradient_colors.1)),
        "center",
        "0 0 16px",
        true,
    );

    let decor_html = format!(
        r#"<div style="width: 60px; height: 3.5px; background: {}; margin: 24px auto; border-radius: 2px; box-shadow: 0 2px 10px {}40;"></div>"#,
        colors.primary, colors.primary
    );

    let sub_html = if !subheadline.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:15px;color:{};margin:0;line-height:1.6;font-weight:400;text-wrap:balance;text-shadow:0 1px 2px rgba(0,0,0,0.05);">{}</p>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(subheadline)
        )
    } else {
        String::new()
    };

    let content = format!(
        r#"<div style="text-align: center; max-width: 500px; margin: 0 auto; width:100%; position:relative; z-index:3;">
            {}
            {}
            {}
        </div>"#,
        heading_html, decor_html, sub_html
    );

    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        "80px 48px 80px",
        "center",
    );
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
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
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let display_fs = tokens.type_scale.get("display").unwrap().font_size;
    let title_fs = tokens.type_scale.get("title").unwrap().font_size;

    let category_html = if !context.is_empty() {
        format!(
            r#"<span style="font-family:{};font-size:11px;font-weight:700;letter-spacing:0.1em;text-transform:uppercase;color:{};display:block;margin-bottom:12px;">{}</span>"#,
            tokens.body_font,
            colors.primary,
            escape_html(context)
        )
    } else {
        String::new()
    };

    let term_html = format!(
        r#"<h2 style="font-family:{};font-size:{}px;font-weight:700;color:{};margin:0 0 16px;line-height:1.15;">{}</h2>"#,
        tokens.heading_font,
        display_fs,
        colors.text_primary,
        escape_html(term)
    );

    let divider_html = format!(
        r#"<div style="width:100%;height:1px;background:{};opacity:0.3;margin-bottom:20px;"></div>"#,
        colors.border
    );

    let def_html = format!(
        r#"<p style="font-family:{};font-size:{}px;font-weight:400;color:{};margin:0 0 24px;line-height:1.5;">{}</p>"#,
        tokens.body_font,
        title_fs,
        colors.text_secondary,
        escape_html(definition)
    );

    let content = format!(
        r#"<div style="width:100%;text-align:left;max-width:320px;margin:0 auto;box-sizing:border-box;display:flex;flex-direction:column;justify-content:center;height:100%;">
            {}{}{}{}
        </div>"#,
        category_html, term_html, divider_html, def_html
    );

    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        "80px 48px 90px",
        "center",
    );
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
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
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let body_fs = 14i32;

    let title_html = if !title.is_empty() {
        format!(
            r#"<h2 style="font-family:{};font-size:22px;font-weight:700;color:{};margin:0 0 16px;line-height:1.2;">{}</h2>"#,
            tokens.heading_font,
            colors.text_primary,
            escape_html(title)
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
        r#"<div style="max-width:320px;margin:0 auto;text-align:left;width:100%;box-sizing:border-box;display:flex;flex-direction:column;justify-content:center;height:100%;overflow:hidden;padding-bottom:var(--space-2);">
            {}
            <div style="margin-top:4px;">{}</div>
        </div>"#,
        title_html, body_html
    );

    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        "80px 48px var(--space-12)",
        "center",
    );
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": "medium",
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 15. metric_card_slide
// ─────────────────────────────────────────────────────────────────────────────

pub fn metric_card_slide(
    tokens: &DesignTokens,
    value: &str,
    label: &str,
    trend: &str,
    context: &str,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let r_val = current_component_radius(tokens, "card");
    let s_bg = if is_dark {
        "rgba(255,255,255,0.05)"
    } else {
        tokens.surface_light.as_str()
    };
    let s_border = format!("1px solid {}33", colors.border);
    let shadow_val = tokens
        .shadows
        .get("md")
        .cloned()
        .unwrap_or_else(|| "none".to_string());

    let trend_html = if !trend.is_empty() {
        let t_color =
            if trend.contains('↓') || trend.contains('-') || trend.to_lowercase().contains("down")
            {
                "#EF4444"
            } else {
                "#10B981"
            };
        format!(
            r#"<span style="font-family:{};font-size:var(--text-sm);font-weight:600;color:{};display:block;margin-bottom:8px;">{}</span>"#,
            tokens.body_font,
            t_color,
            escape_html(trend)
        )
    } else {
        String::new()
    };

    let ctx_html = if !context.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:var(--text-sm);color:{};margin:0;line-height:1.4;">{}</p>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(context)
        )
    } else {
        String::new()
    };

    let card_html = format!(
        r#"<div style="background:{};border:{};box-shadow:{};border-radius:{};padding:var(--space-4) 24px;text-align:center;width:100%;box-sizing:border-box;">
            <span style="font-family:{};font-size:var(--text-display-size, 52px);font-weight:900;color:{};margin:0;line-height:1;">{}</span>
            <h3 style="font-family:{};font-size:var(--text-base);font-weight:600;color:{};margin:var(--space-1) 0 6px;line-height:1.2;">{}</h3>
            {}
            {}
        </div>"#,
        s_bg,
        s_border,
        shadow_val,
        r_val,
        tokens.heading_font,
        colors.primary,
        escape_html(value),
        tokens.body_font,
        colors.text_primary,
        escape_html(label),
        trend_html,
        ctx_html
    );

    let content = format!(
        r#"<div style="width:100%;display:flex;justify-content:center;align-items:center;">{}</div>"#,
        card_html
    );

    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        "80px var(--space-6) 80px",
        "center",
    );
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": "default",
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 16. chart_slide
// ─────────────────────────────────────────────────────────────────────────────

pub fn chart_slide(
    tokens: &DesignTokens,
    chart_type: &str,
    data: Vec<Value>,
    title: &str,
    caption: &str,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let heading = heading_block(
        title,
        tokens,
        "title",
        Some(&colors.text_primary),
        false,
        None,
        "left",
        "0 0 12px",
        true,
    );

    let mut vals = Vec::new();
    for item in &data {
        let val = item
            .get("value")
            .and_then(|v| {
                v.as_f64()
                    .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
            })
            .unwrap_or(0.0);
        vals.push(val);
    }
    let max_val = vals.iter().copied().fold(0.0, f64::max).max(1.0);
    let sum_val = vals.iter().sum::<f64>().max(1.0);

    let mut chart_html = String::new();

    if chart_type == "bar" {
        let max_val = vals.iter().copied().fold(0.0_f64, f64::max).max(1.0);
        let bar_colors: Vec<&str> = vec![
            &colors.primary,
            &tokens.accent,
            &tokens.primary,
            &colors.text_secondary,
            &colors.primary,
        ];
        let mut bars = String::new();
        for (idx, item) in data.iter().take(5).enumerate() {
            let lbl = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
            let val = item
                .get("value")
                .and_then(|v| {
                    v.as_f64()
                        .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
                })
                .unwrap_or(0.0);
            let pct = if max_val > 0.0 {
                (val / max_val) * 100.0
            } else {
                0.0
            };
            let bar_color = bar_colors.get(idx).copied().unwrap_or(&colors.primary);
            let val_display = if val >= 1000.0 {
                format!("{:.0}", val)
            } else if val >= 100.0 {
                format!("{:.0}", val)
            } else if val == val.floor() {
                format!("{:.0}", val)
            } else {
                format!("{:.1}", val)
            };
            bars.push_str(&format!(
                r#"<div style="margin-bottom:16px;width:100%;">
                    <div style="display:flex;justify-content:space-between;font-family:{};font-size:12px;font-weight:600;color:{};margin-bottom:6px;">
                        <span>{}</span>
                        <strong style="color:{};">{}</strong>
                    </div>
                    <div style="width:100%;height:12px;background:{}22;border-radius:6px;overflow:hidden;">
                        <div style="width:{:.1}%;min-width:8px;height:100%;background:{};border-radius:6px;transition:width 0.3s;"></div>
                    </div>
                </div>"#,
                tokens.body_font, colors.text_primary, escape_html(lbl),
                bar_color, val_display,
                bar_color, pct, bar_color
            ));
        }
        chart_html = format!(r#"<div style="width:100%;margin-top:16px;">{}</div>"#, bars);
    } else if chart_type == "pie" || chart_type == "donut" {
        let colors_list = vec![
            colors.primary.as_str(),
            colors.text_secondary.as_str(),
            "#F59E0B",
            "#10B981",
            "#EF4444",
        ];
        let mut legend_items = String::new();
        for (idx, item) in data.iter().take(5).enumerate() {
            let lbl = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
            let val = item
                .get("value")
                .and_then(|v| {
                    v.as_f64()
                        .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
                })
                .unwrap_or(0.0);
            let pct = (val / sum_val) * 100.0;
            let col = colors_list[idx % colors_list.len()];

            legend_items.push_str(&format!(
                r#"<div style="display:flex;align-items:center;gap:var(--space-1);font-family:{};font-size:11px;color:{};margin-bottom:6px;">
                    <div style="width:12px;height:12px;border-radius:3px;background:{};"></div>
                    <span style="flex:1;">{}</span>
                    <strong>{:.1}%</strong>
                </div>"#,
                tokens.body_font, colors.text_primary, col, escape_html(lbl), pct
            ));
        }

        let mut circle_style =
            "width:120px;height:120px;border-radius:50%;background:conic-gradient(".to_string();
        let mut current_deg = 0.0;
        let mut conic_parts = Vec::new();
        for (idx, item) in data.iter().take(5).enumerate() {
            let val = item
                .get("value")
                .and_then(|v| {
                    v.as_f64()
                        .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
                })
                .unwrap_or(0.0);
            let deg = (val / sum_val) * 360.0;
            let col = colors_list[idx % colors_list.len()];
            conic_parts.push(format!(
                "{} {:.1}deg {:.1}deg",
                col,
                current_deg,
                current_deg + deg
            ));
            current_deg += deg;
        }
        circle_style.push_str(&conic_parts.join(", "));
        circle_style.push_str(");position:relative;");

        let mut inner_circle = String::new();
        if chart_type == "donut" {
            let bg_color_repr = if is_dark {
                "var(--surface-dark, #010105)"
            } else {
                "var(--surface-light, #F3F5FC)"
            };
            inner_circle = format!(
                r#"<div style="position:absolute;width:60px;height:60px;border-radius:50%;background:{};left:30px;top:30px;z-index:2;"></div>"#,
                bg_color_repr
            );
        }

        chart_html = format!(
            r#"<div style="display:flex;align-items:center;gap:24px;width:100%;margin-top:20px;justify-content:center;">
                <div style="{}">
                    {}
                </div>
                <div style="flex:1;">
                    {}
                </div>
            </div>"#,
            circle_style, inner_circle, legend_items
        );
    } else if chart_type == "line" || chart_type == "area" {
        chart_html = render_svg_line_chart(&data, 320, 130, &colors, is_dark, chart_type == "area");
        chart_html = format!(
            r#"<div style="width:100%;margin-top:16px;">{}</div>"#,
            chart_html
        );
    } else if chart_type == "scatter" {
        chart_html = render_svg_scatter_plot(&data, 320, 130, &colors);
        chart_html = format!(
            r#"<div style="width:100%;margin-top:16px;">{}</div>"#,
            chart_html
        );
    } else {
        // Fallback column chart
        let mut cols = String::new();
        for (idx, item) in data.iter().take(6).enumerate() {
            let lbl = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
            let val = item
                .get("value")
                .and_then(|v| {
                    v.as_f64()
                        .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
                })
                .unwrap_or(0.0);
            let pct = (val / max_val) * 100.0;

            cols.push_str(&format!(
                r#"<div style="display:flex;flex-direction:column;align-items:center;flex:1;height:var(--space-18);justify-content:flex-end;">
                    <span style="font-family:{};font-size:10px;font-weight:600;color:{};margin-bottom:6px;">{:.0}</span>
                    <div style="width:14px;height:{:.1}%;background:{};border-radius:3px 3px 0 0;"></div>
                    <span style="font-family:{};font-size:10px;color:{};margin-top:6px;transform:rotate(-30deg);white-space:nowrap;">{}</span>
                </div>"#,
                tokens.body_font, colors.text_primary, val,
                pct, colors.primary,
                tokens.body_font, colors.text_secondary, escape_html(lbl)
            ));
        }

        chart_html = format!(
            r#"<div style="display:flex;gap:var(--space-1);width:100%;margin-top:24px;height:180px;border-bottom:1px solid {};padding-bottom:10px;box-sizing:border-box;">
                {}
            </div>"#,
            colors.border, cols
        );
    }

    let caption_html = if !caption.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:11px;color:{};margin:var(--space-2) 0 0;line-height:1.4;text-align:center;width:100%;">{}</p>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(caption)
        )
    } else {
        String::new()
    };

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;justify-content:center;">
            {}
            {}
            {}
        </div>"#,
        heading, chart_html, caption_html
    );

    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        "80px var(--space-6) 80px",
        "center",
    );
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": chart_type,
        "theme": theme
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// dispatch_slide — routes by slide_type string to the correct generator
// ─────────────────────────────────────────────────────────────────────────────

fn scatter_plot_slide(
    tokens: &DesignTokens,
    data: Vec<Value>,
    title: &str,
    _x_label: &str,
    _y_label: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let title_html = heading_block(
        title, tokens, "heading", None, true, None, "left", "0", false,
    );
    let svg = render_svg_scatter_plot(&data, 320, 180, &colors);
    let chart_bg = if colors.is_dark {
        "rgba(255,255,255,0.04)"
    } else {
        "#ffffff"
    };
    let chart_border = if colors.is_dark {
        "1px solid rgba(255,255,255,0.08)"
    } else {
        "1px solid rgba(0,0,0,0.08)"
    };
    let content = format!(
        r#"<div style="width:100%;">{}<div style="width:100%;height:220px;border-radius:10px;overflow:hidden;background:{};border:{};padding:var(--space-1);box-sizing:border-box;">{}</div></div>"#,
        title_html, chart_bg, chart_border, svg
    );
    let html = hero_layout(&content, tokens, bg_style, false, "left");
    let html = inject_background_image(html, bg_img, img_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "default", "theme": theme})
}

fn gauge_slide(
    tokens: &DesignTokens,
    value: f64,
    label: &str,
    title: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let title_html = heading_block(
        title, tokens, "heading", None, true, None, "left", "0", false,
    );
    let svg = render_svg_gauge_chart(value, 100.0, label, &colors);
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;align-items:center;">{}<div style="width:100%;max-width:300px;height:130px;margin:0 auto;">{}</div></div>"#,
        title_html, svg
    );
    let html = hero_layout(&content, tokens, bg_style, false, "center");
    let html = inject_background_image(html, bg_img, img_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "default", "theme": theme})
}

fn radar_chart_slide(
    tokens: &DesignTokens,
    data: Vec<Value>,
    title: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let title_html = heading_block(
        title, tokens, "heading", None, true, None, "left", "0", false,
    );
    let svg = render_svg_radar_chart(&data, 320, 280, &colors);
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;align-items:center;">{}<div style="width:100%;max-width:320px;height:280px;margin:0 auto;">{}</div></div>"#,
        title_html, svg
    );
    let html = hero_layout(&content, tokens, bg_style, false, "center");
    let html = inject_background_image(html, bg_img, img_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "default", "theme": theme})
}

fn progress_rings_slide(
    tokens: &DesignTokens,
    rings: Vec<Value>,
    title: &str,
    _description: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let heading = heading_block(title, tokens, "title", None, true, None, "left", "0", false);

    let rings_html: String = rings.iter().take(3).map(|ring| {
        let lbl = ring.get("label").or_else(|| ring.get("title")).and_then(|v| v.as_str()).unwrap_or("");
        let val = ring.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let deg = (val / 100.0) * 360.0;
        let ring_color = ring.get("color").and_then(|v| v.as_str()).unwrap_or(&colors.primary);
        let track_color = if is_dark { "rgba(255,255,255,0.08)" } else { "rgba(0,0,0,0.06)" };
        let inner_bg = if is_dark { "#1a1a2e" } else { "#ffffff" };

        format!(
            r#"<div style="display:flex;flex-direction:column;align-items:center;flex:1;min-width:var(--space-12);">
                <div style="position:relative;width:90px;height:90px;border-radius:50%;background:conic-gradient({0} 0deg {1:.0}deg, {2} {1:.0}deg 360deg);display:flex;align-items:center;justify-content:center;">
                    <div style="position:absolute;width:70px;height:70px;border-radius:50%;background:{3};z-index:2;display:flex;align-items:center;justify-content:center;">
                        <span style="font-family:{4};font-size:16px;font-weight:700;color:{5};">{1:.0}%</span>
                    </div>
                </div>
                <span style="font-family:{6};font-size:11px;font-weight:600;color:{7};margin-top:12px;text-align:center;text-transform:uppercase;letter-spacing:0.04em;">{8}</span>
            </div>"#,
            ring_color, deg, track_color, inner_bg, tokens.heading_font, colors.text_primary, tokens.body_font, colors.text_secondary, escape_html(lbl)
        )
    }).collect();

    let desc_html = if !_description.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:12px;color:{};margin:var(--space-2) 0 0;line-height:1.45;text-align:center;width:100%;">{}</p>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(_description)
        )
    } else {
        String::new()
    };

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;justify-content:center;align-items:center;">
            {}
            <div style="display:flex;gap:24px;width:100%;margin-top:24px;justify-content:center;align-items:center;">{}</div>
            {}
        </div>"#,
        heading, rings_html, desc_html
    );
    let html = hero_layout(&content, tokens, bg_style, false, "center");
    let html = inject_background_image(html, bg_img, img_opacity, is_dark);
    json!({"html": html, "background": bg_style, "variant": "radial", "theme": theme})
}

fn comparison_bars_slide(
    tokens: &DesignTokens,
    comparison: Value,
    title: &str,
    description: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let heading = heading_block(
        title, tokens, "headline", None, true, None, "left", "0", false,
    );

    let left = comparison.get("left").cloned().unwrap_or(json!({}));
    let right = comparison.get("right").cloned().unwrap_or(json!({}));

    let l_val = left.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let r_val = right.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let total = (l_val + r_val).max(1.0);

    let l_pct = (l_val / total) * 100.0;
    let r_pct = (r_val / total) * 100.0;

    let l_color = left
        .get("color")
        .and_then(|v| v.as_str())
        .unwrap_or(&colors.primary);
    let r_color = right
        .get("color")
        .and_then(|v| v.as_str())
        .unwrap_or(&colors.text_secondary);

    let l_unit = left.get("unit").and_then(|v| v.as_str()).unwrap_or("");
    let r_unit = right.get("unit").and_then(|v| v.as_str()).unwrap_or("");
    let l_space = if l_unit.is_empty() || l_unit == "%" || l_unit.starts_with('°') {
        ""
    } else {
        " "
    };
    let r_space = if r_unit.is_empty() || r_unit == "%" || r_unit.starts_with('°') {
        ""
    } else {
        " "
    };

    let bar_html = format!(
        r#"<div style="width:100%;margin-top:28px;">
            <div style="display:flex;justify-content:space-between;margin-bottom:8px;font-family:{};font-size:11px;color:{};font-weight:600;">
                <span>{}</span>
                <span>{}</span>
            </div>
            <div style="width:100%;height:16px;background:{}30;border-radius:8px;overflow:hidden;display:flex;">
                <div style="width:{:.1}%;height:100%;background:{};border-radius:8px 0 0 8px;"></div>
                <div style="width:{:.1}%;height:100%;background:{};border-radius:0 8px 8px 0;"></div>
            </div>
            <div style="display:flex;justify-content:space-between;margin-top:10px;font-family:{};font-size:24px;font-weight:800;letter-spacing:-0.01em;line-height:1.2;">
                <span style="color:{};">{:.0}{}{}</span>
                <span style="color:{};">{:.0}{}{}</span>
            </div>
        </div>"#,
        tokens.body_font,
        colors.text_primary,
        escape_html(left.get("label").and_then(|v| v.as_str()).unwrap_or("")),
        escape_html(right.get("label").and_then(|v| v.as_str()).unwrap_or("")),
        colors.border,
        l_pct,
        l_color,
        r_pct,
        r_color,
        tokens.heading_font,
        l_color,
        l_val,
        l_space,
        escape_html(l_unit),
        r_color,
        r_val,
        r_space,
        escape_html(r_unit)
    );

    let desc_html = if !description.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:12px;color:{};margin:var(--space-2) 0 0;line-height:1.45;text-align:center;width:100%;">{}</p>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(description)
        )
    } else {
        String::new()
    };

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;justify-content:center;">
            {}{}{}
        </div>"#,
        heading, bar_html, desc_html
    );
    let html = hero_layout(&content, tokens, bg_style, false, "center");
    let html = inject_background_image(html, bg_img, img_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "horizontal", "theme": theme})
}

fn metric_grid_slide(
    tokens: &DesignTokens,
    metrics: Vec<Value>,
    title: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let heading = heading_block(
        title, tokens, "headline", None, true, None, "left", "0", false,
    );

    let card_bg = if is_dark {
        "rgba(255,255,255,0.04)".to_string()
    } else {
        "rgba(0,0,0,0.02)".to_string()
    };
    let card_border = if is_dark {
        "1px solid rgba(255,255,255,0.08)".to_string()
    } else {
        format!("1px solid {}30", colors.border)
    };

    let grid_html: String = metrics.iter().take(4).map(|item| {
        let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
        let lbl = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
        let trend = item.get("trend").and_then(|v| v.as_str()).unwrap_or("");

        let trend_color = if trend.contains('+') || trend.to_lowercase().contains("up") { "#10B981" } else { "#EF4444" };
        let trend_badge = if !trend.is_empty() {
            format!(r#"<span style="font-size:10px;font-weight:700;color:{};background:{}18;padding:2px 6px;border-radius:4px;margin-left:6px;">{}</span>"#, trend_color, trend_color, escape_html(trend))
        } else {
            String::new()
        };

        format!(
            r#"<div style="background:{};border:{};border-radius:{};padding:var(--space-2);box-sizing:border-box;display:flex;flex-direction:column;justify-content:center;">
                <span style="font-family:{};font-size:10px;font-weight:600;color:{};text-transform:uppercase;letter-spacing:0.04em;margin-bottom:6px;">{}</span>
                <div style="display:flex;align-items:baseline;">
                    <span style="font-family:{};font-size:24px;font-weight:800;color:{};">{}</span>
                    {}
                </div>
            </div>"#,
            card_bg, card_border, tokens.radii.get("md").map(|s| s.as_str()).unwrap_or("8px"),
            tokens.body_font, colors.text_secondary, escape_html(lbl),
            tokens.heading_font, colors.text_primary, escape_html(val),
            trend_badge
        )
    }).collect();

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;justify-content:center;">
            {}
            <div style="display:grid;grid-template-columns:1fr 1fr;gap:var(--space-1);margin-top:16px;width:100%;box-sizing:border-box;">{}</div>
        </div>"#,
        heading, grid_html
    );
    let html = hero_layout(&content, tokens, bg_style, false, "center");
    let html = inject_background_image(html, bg_img, img_opacity, is_dark);
    json!({"html": html, "background": bg_style, "variant": "grid", "theme": theme})
}

fn funnel_chart_slide(
    tokens: &DesignTokens,
    steps: Vec<Value>,
    title: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let heading = heading_block(title, tokens, "title", None, true, None, "left", "0", false);

    let mut vals = Vec::new();
    for item in steps.iter().take(3) {
        let val_str = item.get("value").and_then(|v| v.as_str()).unwrap_or("0");
        let clean = val_str
            .replace("K", "000")
            .replace("M", "000000")
            .replace("$", "");
        vals.push(clean.parse::<f64>().unwrap_or(1.0));
    }
    let max_val = vals.iter().copied().fold(0.0, f64::max).max(1.0);

    let num_steps = steps.len().min(3);
    let funnel_html: String = steps.iter().take(3).enumerate().map(|(i, item)| {
        let lbl = item.get("label").or_else(|| item.get("title")).and_then(|v| v.as_str()).unwrap_or("");
        let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
        let current_val = vals.get(i).copied().unwrap_or(1.0);
        let width_pct = ((current_val / max_val) * 100.0).max(10.0) as u32;
        let opacity_pct = 1.0 - (i as f64 * 0.20);

        let arrow = if i < num_steps - 1 {
            format!(r#"<div style="text-align:center;font-size:11px;color:{}99;margin-bottom:6px;font-weight:bold;">↓</div>"#, colors.primary)
        } else {
            String::new()
        };

        // For narrow bars (< 35%), render text outside the bar
        if width_pct < 35 {
            let text_color = &colors.text_primary;
            format!(
                r#"<div style="position:relative;width:100%;margin:0 auto 6px;display:flex;align-items:center;gap:var(--space-1);">
                    <span style="font-family:{};font-size:10px;font-weight:700;color:{};text-transform:uppercase;letter-spacing:0.04em;flex:1;text-align:right;">{}</span>
                    <div style="width:{}%;background:{};opacity:{:.2};border-radius:4px;height:32px;min-width:8px;flex-shrink:0;"></div>
                    <strong style="font-family:{};font-size:12px;color:{};flex:1;">{}</strong>
                </div>{}"#,
                tokens.body_font, text_color, escape_html(lbl),
                width_pct, colors.primary, opacity_pct,
                tokens.body_font, text_color, escape_html(val),
                arrow
            )
        } else {
            format!(
                r#"<div style="width:{}%;background:{};opacity:{:.2};border-radius:4px;padding:8px 14px;box-sizing:border-box;margin:0 auto 6px;display:flex;justify-content:space-between;align-items:center;">
                    <span style="font-family:{};font-size:10px;font-weight:700;color:white;text-transform:uppercase;letter-spacing:0.04em;">{}</span>
                    <strong style="font-family:{};font-size:12px;color:white;">{}</strong>
                </div>{}"#,
                width_pct, colors.primary, opacity_pct,
                tokens.body_font, escape_html(lbl),
                tokens.body_font, escape_html(val),
                arrow
            )
        }
    }).collect();

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;justify-content:center;">
            {}
            <div style="width:100%;margin-top:20px;box-sizing:border-box;">{}</div>
        </div>"#,
        heading, funnel_html
    );
    let html = hero_layout(&content, tokens, bg_style, false, "center");
    let html = inject_background_image(html, bg_img, img_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "funnel", "theme": theme})
}

fn table_slide(
    tokens: &DesignTokens,
    headers: Vec<Value>,
    rows: Vec<Value>,
    title: &str,
    _caption: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let heading = heading_block(title, tokens, "title", None, true, None, "left", "0", false);

    let header_cells: Vec<String> = headers.iter().map(|h| {
        let text = h.as_str().unwrap_or("");
        format!("<th style=\"padding:8px 12px;text-align:left;font-family:{};font-size:11px;font-weight:700;color:{};border-bottom:2px solid {};text-transform:uppercase;letter-spacing:0.04em;\">{}</th>", tokens.body_font, colors.text_primary, colors.border, escape_html(text))
    }).collect();

    let body_rows: String = rows.iter().enumerate().map(|(idx, row)| {
        let cells: Vec<String> = row.as_array().map(|arr| {
            arr.iter().map(|cell| {
                let text = cell.as_str().unwrap_or("");
                let bg = if idx % 2 == 0 { colors.border.clone() + "15" } else { "transparent".to_string() };
                format!("<td style=\"padding:8px 12px;font-family:{};font-size:11px;color:{};background:{};border-bottom:1px solid {}20;\">{}</td>", tokens.body_font, colors.text_primary, bg, colors.border, escape_html(text))
            }).collect()
        }).unwrap_or_default();
        format!("<tr>{}</tr>", cells.join(""))
    }).collect();

    let table_html = format!(
        r#"<table style="width:100%;border-collapse:collapse;margin-top:12px;">
            <thead><tr>{}</tr></thead>
            <tbody>{}</tbody>
        </table>"#,
        header_cells.join(""),
        body_rows
    );

    let caption_html = if !_caption.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:10px;color:{};margin:var(--space-1) 0 0;line-height:1.4;text-align:center;width:100%;">{}</p>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(_caption)
        )
    } else {
        String::new()
    };

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;justify-content:center;">
            {}{}{}
        </div>"#,
        heading, table_html, caption_html
    );
    let html = hero_layout(&content, tokens, bg_style, false, "center");
    let html = inject_background_image(html, bg_img, img_opacity, is_dark);
    json!({"html": html, "background": bg_style, "variant": "default", "theme": theme})
}

fn metric_sparkline_slide(
    tokens: &DesignTokens,
    value: &str,
    label: &str,
    spark_values: Vec<Value>,
    trend: &str,
    _context: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let bg_color = if is_dark {
        "rgba(255,255,255,0.05)"
    } else {
        "rgba(0,0,0,0.02)"
    };
    let border_color = format!("1px solid {}33", colors.border);

    let trend_color = if trend.contains('↓') || trend.contains('-') {
        "#EF4444"
    } else {
        "#10B981"
    };
    let trend_html = if !trend.is_empty() {
        format!(
            r#"<span style="font-family:{};font-size:11px;font-weight:600;color:{};display:block;margin-bottom:8px;">{}</span>"#,
            tokens.body_font,
            trend_color,
            escape_html(trend)
        )
    } else {
        String::new()
    };

    let ctx_html = if !_context.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:11px;color:{};margin:0;line-height:1.4;">{}</p>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(_context)
        )
    } else {
        String::new()
    };

    let spark_points: Vec<String> = spark_values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let val = v.as_f64().unwrap_or(0.0);
            let x = (i as f64 / (spark_values.len() as f64 - 1.0).max(1.0)) * 280.0;
            let y = 40.0 - (val / 100.0) * 35.0;
            format!("{:.1},{:.1}", x, y)
        })
        .collect();
    let spark_html = if spark_points.len() > 1 {
        format!(
            r#"<svg width="280" height="40" viewBox="0 0 280 40"><polyline points="{}" fill="none" stroke="{}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>"#,
            spark_points.join(" "),
            colors.primary
        )
    } else {
        String::new()
    };

    let card_html = format!(
        r#"<div style="background:{};border:{};box-shadow:{};border-radius:{};padding:var(--space-3) 20px;text-align:center;width:100%;box-sizing:border-box;">
            <span style="font-family:{};font-size:52px;font-weight:900;color:{};margin:0;line-height:1;">{}</span>
            <h3 style="font-family:{};font-size:15px;font-weight:600;color:{};margin:var(--space-1) 0 6px;line-height:1.2;">{}</h3>
            {}
            <div style="margin:var(--space-2) auto;display:flex;justify-content:center;">{}</div>
            {}
        </div>"#,
        bg_color,
        border_color,
        tokens
            .shadows
            .get("md")
            .map(|s| s.as_str())
            .unwrap_or("none"),
        tokens
            .radii
            .get("lg")
            .map(|s| s.as_str())
            .unwrap_or("var(--space-1)"),
        tokens.heading_font,
        colors.primary,
        escape_html(value),
        tokens.body_font,
        colors.text_primary,
        escape_html(label),
        trend_html,
        spark_html,
        ctx_html
    );

    let content = format!(
        r#"<div style="width:100%;display:flex;justify-content:center;align-items:center;">{}</div>"#,
        card_html
    );
    let html = hero_layout(&content, tokens, bg_style, false, "center");
    let html = inject_background_image(html, bg_img, img_opacity, is_dark);
    json!({"html": html, "background": bg_style, "variant": "default", "theme": theme})
}

fn column_chart_slide(
    tokens: &DesignTokens,
    data: Vec<Value>,
    title: &str,
    _caption: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let heading = heading_block(title, tokens, "title", None, true, None, "left", "0", false);

    let vals: Vec<f64> = data
        .iter()
        .map(|item| item.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0))
        .collect();
    let max_val = vals.iter().copied().fold(0.0, f64::max).max(1.0);

    let bars: String = data.iter().zip(vals.iter()).map(|(item, val)| {
        let lbl = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
        let pct = (val / max_val) * 100.0;
        let val_display = if *val >= 1000.0 {
            format!("{:.0}", val)
        } else if *val == val.floor() {
            format!("{:.0}", val)
        } else {
            format!("{:.1}", val)
        };
        format!(
            r#"<div style="display:flex;flex-direction:column;align-items:center;flex:1;min-width:0;">
                <div style="font-family:{};font-size:10px;font-weight:800;color:{};line-height:1;margin-bottom:6px;text-align:center;">{}</div>
                <div style="width:100%;height:104px;display:flex;align-items:flex-end;justify-content:center;">
                    <div style="width:70%;height:{:.1}%;min-height:8px;background:{};border-radius:4px 4px 0 0;"></div>
                </div>
                <span style="font-family:{};font-size:10px;color:{};margin-top:6px;text-align:center;overflow:hidden;text-overflow:ellipsis;max-width:100%;">{}</span>
            </div>"#,
            tokens.body_font,
            colors.text_primary,
            val_display,
            pct,
            colors.primary,
            tokens.body_font,
            colors.text_secondary,
            escape_html(lbl)
        )
    }).collect();

    let chart_html = format!(
        r#"<div style="display:flex;gap:var(--space-1);width:100%;height:142px;margin-top:16px;overflow:hidden;">{}</div>"#,
        bars
    );

    let caption_html = if !_caption.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:10px;color:{};margin:var(--space-1) 0 0;line-height:1.4;text-align:center;width:100%;">{}</p>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(_caption)
        )
    } else {
        String::new()
    };

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;justify-content:center;">
            {}{}{}
        </div>"#,
        heading, chart_html, caption_html
    );
    let html = hero_layout(&content, tokens, bg_style, false, "center");
    let html = inject_background_image(html, bg_img, img_opacity, is_dark);
    json!({"html": html, "background": bg_style, "variant": "default", "theme": theme})
}

fn text_columns_slide(
    tokens: &DesignTokens,
    title: &str,
    columns: Vec<Value>,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let title_html = heading_block(
        title,
        tokens,
        "headline",
        Some(&colors.text_primary),
        true,
        None,
        "left",
        "0 0 var(--space-1)",
        false,
    );
    let cols: Vec<String> = columns.iter().map(|c| {
        let heading = c.get("heading").and_then(|v| v.as_str()).unwrap_or("");
        let body = c.get("body").and_then(|v| v.as_str()).unwrap_or("");
        let heading_html = if !heading.is_empty() {
            heading_block(heading, tokens, "title", Some(&colors.text_primary), false, None, "left", "0 0 var(--space-1)", false)
        } else {
            String::new()
        };
        format!(r#"<div style="flex:1;min-width:0;overflow:visible;padding-bottom:var(--space-1);">{}<div style="font-size:var(--text-sm);color:{};line-height:1.6;overflow-wrap:break-word;word-break:break-word;">{}</div></div>"#,
            heading_html, colors.text_secondary, body)
    }).collect();
    let columns_html = format!(
        r#"<div style="display:flex;gap:var(--space-3);width:100%;margin-top:var(--space-2);overflow:hidden;">{}</div>"#,
        cols.join("")
    );
    let content = format!("{}{}", title_html, columns_html);
    let html = hero_layout(&content, tokens, bg_style, false, "left");
    let html = inject_background_image(html, bg_img, img_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "default", "theme": theme})
}

fn simple_text(v: &Value, keys: &[&str]) -> String {
    keys.iter()
        .find_map(|key| v.get(*key).and_then(|x| x.as_str()))
        .unwrap_or("")
        .to_string()
}

fn visual_badge_html(
    tokens: &DesignTokens,
    colors: &crate::layouts::SlideColors,
    item: &Value,
    fallback: &str,
    size: i32,
) -> String {
    let logo_url = item.get("logo_url").and_then(|v| v.as_str()).unwrap_or("");
    let image_url = item
        .get("image_url")
        .or_else(|| item.get("brand_image"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let icon = item.get("icon").and_then(|v| v.as_str()).unwrap_or("");
    let source = if !logo_url.is_empty() {
        logo_url
    } else {
        image_url
    };
    if !source.is_empty() {
        return format!(
            r#"<img src="{}" alt="{}" style="width:{}px;height:{}px;border-radius:{};object-fit:cover;display:block;border:1px solid {};">"#,
            source,
            escape_html(fallback),
            size,
            size,
            current_component_radius(tokens, "chip"),
            colors.border
        );
    }

    let label = if !icon.is_empty() {
        icon.to_string()
    } else {
        fallback.chars().next().unwrap_or('•').to_string()
    };
    format!(
        r#"<div style="width:{}px;height:{}px;border-radius:{};background:{};color:white;display:flex;align-items:center;justify-content:center;font-family:{};font-size:{}px;font-weight:900;flex-shrink:0;">{}</div>"#,
        size,
        size,
        current_component_radius(tokens, "chip"),
        colors.primary,
        tokens.heading_font,
        (size as f32 * 0.42) as i32,
        escape_html(&label)
    )
}

fn render_compact_items(
    tokens: &DesignTokens,
    colors: &crate::layouts::SlideColors,
    items: &[Value],
    title_keys: &[&str],
    body_keys: &[&str],
) -> String {
    let radius = current_component_radius(tokens, "card");
    let card_bg = if colors.is_dark {
        "rgba(255,255,255,0.06)"
    } else {
        "rgba(255,255,255,0.92)"
    };
    items
        .iter()
        .take(4)
        .enumerate()
        .map(|(idx, item)| {
            let title = simple_text(item, title_keys);
            let body = simple_text(item, body_keys);
            let visual = visual_badge_html(tokens, colors, item, &title, 28);
            format!(
                r#"<div style="background:{};border:1px solid {};border-radius:{};padding:var(--space-2) 16px;box-sizing:border-box;">
                    <div style="display:flex;align-items:center;gap:10px;margin-bottom:8px;">{}<div style="font-family:{};font-size:11px;font-weight:800;color:{};text-transform:uppercase;">{:02}</div></div>
                    <div style="font-family:{};font-size:15px;font-weight:800;color:{};line-height:1.2;margin-bottom:5px;">{}</div>
                    <div style="font-family:{};font-size:12px;color:{};line-height:1.45;">{}</div>
                </div>"#,
                card_bg,
                colors.border,
                radius,
                visual,
                tokens.body_font,
                colors.primary,
                idx + 1,
                tokens.heading_font,
                colors.text_primary,
                escape_html(&title),
                tokens.body_font,
                colors.text_secondary,
                escape_html(&body)
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn section_divider_slide(
    tokens: &DesignTokens,
    title: &str,
    subtitle: &str,
    kicker: &str,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let content = format!(
        r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;justify-content:center;">
            <div style="font-family:{};font-size:12px;font-weight:800;color:{};letter-spacing:0.08em;text-transform:uppercase;margin-bottom:18px;">{}</div>
            <h1 style="font-family:{};font-size:{}px;font-weight:900;color:{};line-height:1.02;margin:0;max-width:320px;">{}</h1>
            <div style="width:76px;height:4px;background:{};border-radius:{};margin:var(--space-3) 0;"></div>
            <p style="font-family:{};font-size:15px;color:{};line-height:1.45;margin:0;max-width:300px;">{}</p>
        </div>"#,
        tokens.body_font,
        colors.primary,
        escape_html(kicker),
        tokens.heading_font,
        tokens
            .type_scale
            .get("display")
            .map(|t| t.font_size)
            .unwrap_or(40),
        colors.text_primary,
        escape_html(title),
        colors.primary,
        current_component_radius(tokens, "chip"),
        tokens.body_font,
        colors.text_secondary,
        escape_html(subtitle)
    );
    let html = slide_base(&content, tokens, bg_style, true, "80px 52px", "center");
    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "section_divider", "theme": theme})
}

pub fn problem_solution_slide(
    tokens: &DesignTokens,
    title: &str,
    problem: &str,
    solution: &str,
    proof_points: Vec<Value>,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let radius = current_component_radius(tokens, "card");
    let card_bg = if colors.is_dark {
        "rgba(255,255,255,0.06)"
    } else {
        "rgba(255,255,255,0.92)"
    };
    let points = render_compact_items(
        tokens,
        &colors,
        &proof_points,
        &["title", "label"],
        &["description", "body"],
    );
    // Adaptive grid: 1 item → single column (avoids empty right column);
    // 2 items → 1fr 1fr; 3-4 items → 1fr 1fr (wraps to 2 rows).
    let proof_count = proof_points.len().min(4);
    let proof_grid_cols = if proof_count <= 1 { "1fr" } else { "1fr 1fr" };
    let proof_grid_html = if proof_points.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div style="display:grid;grid-template-columns:{};gap:10px;">{}</div>"#,
            proof_grid_cols, points
        )
    };
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:18px;">
            <h2 style="font-family:{};font-size:28px;font-weight:900;color:{};margin:0;line-height:1.08;">{}</h2>
            <div style="display:grid;grid-template-columns:1fr 1fr;gap:14px;">
                <div style="border-radius:{};padding:18px;background:{};border:1px solid {};"><div style="font-family:{};font-size:11px;font-weight:800;color:#EF4444;margin-bottom:8px;">PROBLEM</div><p style="font-family:{};font-size:var(--text-sm);color:{};line-height:1.45;margin:0;">{}</p></div>
                <div style="border-radius:{};padding:18px;background:{};border:1px solid {};"><div style="font-family:{};font-size:11px;font-weight:800;color:{};margin-bottom:8px;">SOLUTION</div><p style="font-family:{};font-size:var(--text-sm);color:{};line-height:1.45;margin:0;">{}</p></div>
            </div>
            {}
        </div>"#,
        tokens.heading_font,
        colors.text_primary,
        escape_html(title),
        radius,
        card_bg,
        colors.border,
        tokens.body_font,
        tokens.body_font,
        colors.text_secondary,
        escape_html(problem),
        radius,
        card_bg,
        colors.border,
        tokens.body_font,
        colors.primary,
        tokens.body_font,
        colors.text_secondary,
        escape_html(solution),
        proof_grid_html
    );
    let html = slide_base(&content, tokens, bg_style, false, "72px 44px", "center");
    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "problem_solution", "theme": theme})
}

pub fn myth_fact_slide(
    tokens: &DesignTokens,
    myth: &str,
    fact: &str,
    explanation: &str,
    bg_style: &str,
    variant: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let (gc, gx) = get_glass_container(tokens, is_dark);
    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let title_fs = tokens.type_scale.get("title").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    let radius_md = current_component_radius(tokens, "card");
    let (card_bg, card_border, card_blur) = card_styles(tokens, is_dark);
    let shadow_lg = tokens.shadows.get("lg").cloned().unwrap_or_else(|| "none".to_string());

    let effective_variant = variant;

    let content = match effective_variant {
        "debunk" => {
            // Myth is shown crossed out, fact appears below with explanation
            let myth_html = format!(
                r#"<div style="background:{};border:{};{}border-radius:{};padding:var(--space-3) var(--space-4);margin-bottom:var(--space-3);box-shadow:{};position:relative;">
                    <div style="font-family:{};font-size:{}px;font-weight:600;color:{};text-decoration:line-through;text-decoration-color:{};text-decoration-thickness:2px;opacity:0.55;line-height:1.3;">{}</div>
                    <div style="position:absolute;top:50%;left:50%;transform:translate(-50%,-50%) rotate(-8deg);font-family:{};font-size:11px;font-weight:800;color:{};letter-spacing:0.12em;text-transform:uppercase;background:{};padding:3px 12px;border-radius:20px;">MYTH</div>
                </div>"#,
                card_bg, card_border, card_blur, radius_md, shadow_lg,
                tokens.body_font, body_fs, colors.text_secondary, tokens.primary,
                escape_html(myth),
                tokens.heading_font, tokens.primary, tokens.primary,
            );
            let fact_html = format!(
                r#"<div style="background:{};border-left:3px solid {};border-radius:{};padding:var(--space-3) var(--space-4);box-shadow:0 2px 8px rgba(0,0,0,0.06);">
                    <div style="font-family:{};font-size:10px;font-weight:800;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:6px;">FACT</div>
                    <div style="font-family:{};font-size:{}px;font-weight:600;color:{};line-height:1.3;">{}</div>
                </div>"#,
                card_bg, tokens.primary, radius_md,
                tokens.heading_font, tokens.primary,
                tokens.body_font, body_fs, colors.text_primary, escape_html(fact),
            );
            let explanation_html = if !explanation.is_empty() {
                format!(
                    r#"<div style="font-family:{};font-size:{}px;color:{};margin-top:var(--space-3);line-height:1.5;">{}</div>"#,
                    tokens.body_font, caption_fs, colors.text_secondary, escape_html(explanation)
                )
            } else {
                String::new()
            };
            format!("{}{}<div style=\"margin-top:16px;\">{}{}{}</div>{}", gc, heading_block("Myth vs Fact", tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 12px", true), myth_html, fact_html, explanation_html, gx)
        }
        _ => {
            // split (default) — myth and fact side by side
            let myth_html = format!(
                r#"<div style="flex:1;">
                    <div style="font-family:{};font-size:10px;font-weight:800;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:8px;">MYTH</div>
                    <div style="background:{};border:{};{}border-radius:{};padding:var(--space-3);box-shadow:{};">
                        <div style="font-family:{};font-size:{}px;font-weight:500;color:{};line-height:1.4;text-decoration:line-through;text-decoration-color:{};text-decoration-thickness:1.5px;opacity:0.6;">{}</div>
                    </div>
                </div>"#,
                tokens.heading_font, colors.text_secondary,
                card_bg, card_border, card_blur, radius_md, shadow_lg,
                tokens.body_font, body_fs, colors.text_secondary, tokens.primary,
                escape_html(myth),
            );
            let fact_html = format!(
                r#"<div style="flex:1;">
                    <div style="font-family:{};font-size:10px;font-weight:800;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:8px;">FACT</div>
                    <div style="background:{};border-left:3px solid {};border-radius:{};padding:var(--space-3);box-shadow:0 2px 8px rgba(0,0,0,0.06);">
                        <div style="font-family:{};font-size:{}px;font-weight:600;color:{};line-height:1.4;">{}</div>
                    </div>
                </div>"#,
                tokens.heading_font, tokens.primary,
                card_bg, tokens.primary, radius_md,
                tokens.body_font, body_fs, colors.text_primary, escape_html(fact),
            );
            let explanation_html = if !explanation.is_empty() {
                format!(
                    r#"<div style="font-family:{};font-size:{}px;color:{};margin-top:var(--space-3);line-height:1.5;text-align:center;">{}</div>"#,
                    tokens.body_font, caption_fs, colors.text_secondary, escape_html(explanation)
                )
            } else {
                String::new()
            };
            format!("{}{}<div style=\"display:flex;gap:var(--space-3);margin-top:16px;\">{}{}</div>{}{}", gc, heading_block("Myth vs Fact", tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 12px", true), myth_html, fact_html, explanation_html, gx)
        }
    };

    let html = slide_base(&content, tokens, bg_style, false, "80px var(--space-6) 80px", "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": effective_variant,
        "theme": theme
    })
}

pub fn checklist_action_plan_slide(
    tokens: &DesignTokens,
    title: &str,
    items: Vec<Value>,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let radius = current_component_radius(tokens, "card");
    let card_bg = if colors.is_dark {
        "rgba(255,255,255,0.06)"
    } else {
        "rgba(255,255,255,0.92)"
    };
    let rows = items
        .iter()
        .take(6)
        .enumerate()
        .map(|(idx, item)| {
            let label = simple_text(item, &["label", "title", "task"]);
            format!(
                r#"<div style="display:flex;gap:var(--space-1);align-items:flex-start;background:{};border:1px solid {};border-radius:{};padding:var(--space-1) 14px;">
                    <div style="width:24px;height:24px;border-radius:50%;background:{};color:white;display:flex;align-items:center;justify-content:center;font-family:{};font-size:12px;font-weight:800;flex-shrink:0;">{}</div>
                    <div style="font-family:{};font-size:var(--text-sm);font-weight:700;color:{};line-height:1.45;">{}</div>
                </div>"#,
                card_bg,
                colors.border,
                radius,
                colors.primary,
                tokens.body_font,
                idx + 1,
                tokens.body_font,
                colors.text_primary,
                escape_html(&label)
            )
        })
        .collect::<Vec<_>>()
        .join("");
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:18px;"><h2 style="font-family:{};font-size:30px;font-weight:900;color:{};margin:0;">{}</h2><div style="display:flex;flex-direction:column;gap:10px;">{}</div></div>"#,
        tokens.heading_font,
        colors.text_primary,
        escape_html(title),
        rows
    );
    let html = slide_base(&content, tokens, bg_style, false, "72px 44px", "center");
    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "checklist_action_plan", "theme": theme})
}

pub fn case_study_result_slide(
    tokens: &DesignTokens,
    client: &str,
    challenge: &str,
    solution: &str,
    results: Vec<Value>,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let title = if client.is_empty() {
        "Case Study"
    } else {
        client
    };
    problem_solution_slide(
        tokens,
        title,
        challenge,
        solution,
        results,
        bg_style,
        theme,
        background_image,
        image_opacity,
    )
}

pub fn pricing_plan_slide(
    tokens: &DesignTokens,
    title: &str,
    plans: Vec<Value>,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let radius = current_component_radius(tokens, "card");
    let card_bg = if colors.is_dark {
        "rgba(255,255,255,0.06)"
    } else {
        "rgba(255,255,255,0.92)"
    };
    let cards: Vec<String> = plans
        .iter()
        .take(3)
        .enumerate()
        .map(|(idx, plan)| {
            let name = simple_text(plan, &["name", "title"]);
            let price = simple_text(plan, &["price", "value"]);
            let desc = simple_text(plan, &["description", "caption"]);
            let visual = visual_badge_html(tokens, &colors, plan, &name, if idx == 0 { 36 } else { 30 });
            let price_size = if idx == 0 { 30 } else { 22 };
            let padding = if idx == 0 { "16px 18px" } else { "12px 14px" };
            format!(
                r#"<div style="min-width:0;background:{};border:1px solid {};border-radius:{};padding:{};box-sizing:border-box;height:100%;">
                    <div style="display:flex;align-items:center;gap:10px;margin-bottom:8px;min-width:0;">{}<div style="font-family:{};font-size:var(--text-sm);font-weight:900;color:{};white-space:nowrap;overflow:hidden;text-overflow:ellipsis;">{}</div></div>
                    <div style="font-family:{};font-size:{}px;font-weight:900;color:{};margin:var(--space-1) 0 6px;line-height:1;">{}</div>
                    <p style="font-family:{};font-size:11px;color:{};line-height:1.45;margin:0;">{}</p>
                </div>"#,
                card_bg,
                colors.border,
                radius,
                padding,
                visual,
                tokens.heading_font,
                colors.text_primary,
                escape_html(&name),
                tokens.heading_font,
                price_size,
                colors.primary,
                escape_html(&price),
                tokens.body_font,
                colors.text_secondary,
                escape_html(&desc)
            )
        })
        .collect();
    let plan_grid = if cards.len() >= 3 {
        format!(
            r#"<div style="display:grid;grid-template-columns:1.06fr 1fr;grid-template-rows:1fr 1fr;gap:10px;width:100%;min-width:0;">
                <div style="grid-row:1 / span 2;min-width:0;">{}</div>
                <div style="min-width:0;">{}</div>
                <div style="min-width:0;">{}</div>
            </div>"#,
            cards[0], cards[1], cards[2]
        )
    } else {
        format!(
            r#"<div style="display:grid;grid-template-columns:repeat({}, minmax(0, 1fr));gap:10px;width:100%;min-width:0;">{}</div>"#,
            cards.len().max(1),
            cards.join("")
        )
    };
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:18px;min-width:0;"><h2 style="font-family:{};font-size:28px;font-weight:900;color:{};margin:0;line-height:1.05;">{}</h2>{}</div>"#,
        tokens.heading_font,
        colors.text_primary,
        escape_html(title),
        plan_grid
    );
    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        "var(--space-9) var(--space-5)",
        "center",
    );
    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "pricing_plan", "theme": theme})
}

pub fn testimonial_avatar_slide(
    tokens: &DesignTokens,
    quote: &str,
    author: &str,
    role: &str,
    avatar_url: &str,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let avatar = if avatar_url.is_empty() {
        format!(
            r#"<div style="width:72px;height:72px;border-radius:50%;background:{};color:white;display:flex;align-items:center;justify-content:center;font-family:{};font-size:24px;font-weight:900;">{}</div>"#,
            colors.primary,
            tokens.heading_font,
            author.chars().next().unwrap_or('A')
        )
    } else {
        format!(
            r#"<img src="{}" alt="{}" style="width:72px;height:72px;border-radius:50%;object-fit:cover;border:3px solid {};">"#,
            avatar_url,
            escape_html(author),
            if colors.is_dark {
                "rgba(255,255,255,0.16)"
            } else {
                "white"
            }
        )
    };
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;align-items:center;text-align:center;gap:var(--space-2);">{}<p style="font-family:{};font-size:28px;font-weight:800;color:{};line-height:1.2;margin:0;">“{}”</p><div><div style="font-family:{};font-size:15px;font-weight:900;color:{};">{}</div><div style="font-family:{};font-size:12px;color:{};">{}</div></div></div>"#,
        avatar,
        tokens.heading_font,
        colors.text_primary,
        escape_html(quote),
        tokens.body_font,
        colors.text_primary,
        escape_html(author),
        tokens.body_font,
        colors.text_secondary,
        escape_html(role)
    );
    let html = slide_base(&content, tokens, bg_style, false, "80px 44px", "center");
    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "testimonial_avatar", "theme": theme})
}

pub fn logo_cloud_slide(
    tokens: &DesignTokens,
    title: &str,
    logos: Vec<Value>,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let radius = current_component_radius(tokens, "card");
    let card_bg = if colors.is_dark {
        "rgba(255,255,255,0.06)"
    } else {
        "rgba(255,255,255,0.92)"
    };
    let cells = logos
        .iter()
        .take(8)
        .map(|logo| {
            let label = logo
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| simple_text(logo, &["name", "label"]));
            let visual = if logo.is_object() {
                visual_badge_html(tokens, &colors, logo, &label, 30)
            } else {
                let shim = json!({"icon": label.chars().next().unwrap_or('•').to_string()});
                visual_badge_html(tokens, &colors, &shim, &label, 30)
            };
            format!(
                r#"<div style="height:58px;border-radius:{};border:1px solid {};background:{};display:flex;align-items:center;justify-content:flex-start;gap:10px;padding:0 14px;font-family:{};font-size:var(--text-sm);font-weight:800;color:{};box-sizing:border-box;">{}{}</div>"#,
                radius,
                colors.border,
                card_bg,
                tokens.body_font,
                colors.text_secondary,
                visual,
                escape_html(&label)
            )
        })
        .collect::<Vec<_>>()
        .join("");
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:24px;"><h2 style="font-family:{};font-size:28px;font-weight:900;color:{};margin:0;text-align:center;">{}</h2><div style="display:grid;grid-template-columns:1fr 1fr;gap:var(--space-1);">{}</div></div>"#,
        tokens.heading_font,
        colors.text_primary,
        escape_html(title),
        cells
    );
    let html = slide_base(&content, tokens, bg_style, false, "72px 44px", "center");
    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "logo_cloud", "theme": theme})
}

pub fn faq_slide(
    tokens: &DesignTokens,
    title: &str,
    questions: Vec<Value>,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let items = questions
        .into_iter()
        .map(|q| {
            json!({"label": format!("{} - {}", simple_text(&q, &["question", "q"]), simple_text(&q, &["answer", "a"]))})
        })
        .collect();
    checklist_action_plan_slide(
        tokens,
        title,
        items,
        bg_style,
        theme,
        background_image,
        image_opacity,
    )
}

pub fn process_map_slide(
    tokens: &DesignTokens,
    title: &str,
    steps: Vec<Value>,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    checklist_action_plan_slide(
        tokens,
        title,
        steps,
        bg_style,
        theme,
        background_image,
        image_opacity,
    )
}

pub fn before_after_story_slide(
    tokens: &DesignTokens,
    title: &str,
    before: &str,
    after: &str,
    metric: &str,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let radius = current_component_radius(tokens, "card");
    let card_bg = if colors.is_dark {
        "rgba(255,255,255,0.06)"
    } else {
        "rgba(255,255,255,0.92)"
    };
    let metric_html = if metric.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div style="margin-top:14px;border-radius:{};background:{};border:1px solid {};padding:var(--space-2) 16px;display:flex;align-items:center;gap:var(--space-1);">
                <div style="width:34px;height:34px;border-radius:{};background:{};color:white;display:flex;align-items:center;justify-content:center;font-family:{};font-size:16px;font-weight:900;flex-shrink:0;">↗</div>
                <div style="font-family:{};font-size:var(--text-sm);font-weight:800;color:{};line-height:1.45;">{}</div>
            </div>"#,
            radius,
            card_bg,
            colors.border,
            current_component_radius(tokens, "chip"),
            colors.primary,
            tokens.heading_font,
            tokens.body_font,
            colors.text_primary,
            escape_html(metric)
        )
    };
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:18px;">
            <h2 style="font-family:{};font-size:28px;font-weight:900;color:{};margin:0;line-height:1.08;">{}</h2>
            <div style="display:grid;grid-template-columns:1fr auto 1fr;gap:var(--space-1);align-items:stretch;">
                <div style="border-radius:{};padding:16px;background:{};border:1px solid {};box-sizing:border-box;">
                    <div style="font-family:{};font-size:11px;font-weight:900;color:#EF4444;margin-bottom:8px;letter-spacing:0.06em;">BEFORE</div>
                    <p style="font-family:{};font-size:var(--text-sm);color:{};line-height:1.45;margin:0;">{}</p>
                </div>
                <div style="display:flex;align-items:center;justify-content:center;color:{};font-family:{};font-size:22px;font-weight:900;">→</div>
                <div style="border-radius:{};padding:16px;background:{};border:1px solid {};box-sizing:border-box;">
                    <div style="font-family:{};font-size:11px;font-weight:900;color:{};margin-bottom:8px;letter-spacing:0.06em;">AFTER</div>
                    <p style="font-family:{};font-size:var(--text-sm);color:{};line-height:1.45;margin:0;">{}</p>
                </div>
            </div>
            {}
        </div>"#,
        tokens.heading_font,
        colors.text_primary,
        escape_html(title),
        radius,
        card_bg,
        colors.border,
        tokens.body_font,
        tokens.body_font,
        colors.text_secondary,
        escape_html(before),
        colors.primary,
        tokens.heading_font,
        radius,
        card_bg,
        colors.border,
        tokens.body_font,
        colors.primary,
        tokens.body_font,
        colors.text_secondary,
        escape_html(after),
        metric_html
    );
    let html = slide_base(&content, tokens, bg_style, false, "72px 44px", "center");
    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "before_after_story", "theme": theme})
}

pub fn qr_destination_slide(
    tokens: &DesignTokens,
    destination_url: &str,
    heading: &str,
    caption: &str,
    cta_text: &str,
    short_url: &str,
    incentive_text: &str,
    variant: &str,
    bg_style: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
    _archetype: &str,
    padding: &str,
    brand_name: &str,
    brand_logo: &str,
    qr_alt_text: &str,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let effective_variant = if variant.is_empty() {
        "full-conversion"
    } else {
        variant
    };
    let qr_src = render_qr_svg_data_uri(destination_url).unwrap_or_default();
    let radius = current_component_radius(tokens, "card");
    let qr_size_px = if matches!(effective_variant, "minimal" | "without-heading" | "poster") {
        208
    } else if matches!(effective_variant, "compact") {
        164
    } else {
        188
    };
    let qr_size = format!("{}px", qr_size_px);

    // Brand header outside QR card (above QR image) if present and not empty
    let brand_html = if !brand_logo.is_empty() || !brand_name.is_empty() {
        let logo_img = if !brand_logo.is_empty() {
            format!(
                r#"<img src="{}" alt="{}" style="max-height:24px;max-width:80px;object-fit:contain;display:block;" />"#,
                escape_html(brand_logo),
                escape_html(brand_name)
            )
        } else {
            String::new()
        };
        let name_text = if !brand_name.is_empty() {
            format!(
                r#"<span style="font-family:{};font-size:12px;font-weight:700;color:{};letter-spacing:-0.01em;white-space:nowrap;">{}</span>"#,
                tokens.body_font,
                colors.text_secondary,
                escape_html(brand_name)
            )
        } else {
            String::new()
        };
        format!(
            r#"<div style="display:flex;align-items:center;gap:var(--space-1);margin-bottom:12px;max-width:{};justify-content:center;overflow:hidden;">
                {}
                {}
            </div>"#,
            qr_size, logo_img, name_text
        )
    } else {
        String::new()
    };

    let effective_alt = if !qr_alt_text.is_empty() {
        qr_alt_text
    } else if !cta_text.is_empty() {
        cta_text
    } else {
        "Scan QR code"
    };

    let mut qr_elements = Vec::new();
    qr_elements.push(format!(
        r#"<img src="{}" alt="{}" style="max-width:100%;height:auto;width:{};display:block;" />"#,
        qr_src,
        escape_html(effective_alt),
        qr_size,
    ));
    // URL text removed from QR card: was causing white-on-white contrast and line-breaking
    // short_url parameter kept for API compatibility but no longer rendered in QR card

    let qr_html = format!(
        r#"<div style="background:#FFFFFF;border:1px solid rgba(0,0,0,0.08);border-radius:{};padding:var(--space-2);display:flex;flex-direction:column;align-items:center;gap:var(--space-1);box-shadow:0 14px 34px rgba(0,0,0,0.14);width:100%;max-width:{};">
            {}
        </div>"#,
        radius,
        qr_size,
        qr_elements.join("\n")
    );

    let cta_html = if !cta_text.is_empty() {
        format!(
            r#"<div style="margin-top:var(--space-2);background:{};color:{};font-family:{};font-size:var(--text-sm);font-weight:800;padding:var(--space-2) var(--space-2);border-radius:{};box-shadow:0 4px 12px rgba(0,0,0,0.08);text-align:center;letter-spacing:-0.01em;display:inline-block;overflow-wrap:break-word;">{}</div>"#,
            colors.primary,
            colors.button_text,
            tokens.heading_font,
            current_component_radius(tokens, "chip"),
            escape_html(cta_text)
        )
    } else {
        String::new()
    };

    let layout_padding = if !padding.is_empty() {
        padding
    } else if matches!(effective_variant, "minimal" | "with-cta" | "compact") {
        "80px 64px 80px"
    } else if matches!(effective_variant, "poster" | "stacked-badge") {
        "64px var(--space-6) 72px"
    } else {
        "80px var(--space-6) 80px"
    };

    let html = if matches!(effective_variant, "minimal" | "with-cta" | "compact") {
        let content = format!(
            r#"<div style="display:flex;flex-direction:column;align-items:center;justify-content:center;">{} {} {}</div>"#,
            brand_html, qr_html, cta_html
        );
        slide_base(&content, tokens, bg_style, false, layout_padding, "center")
    } else if matches!(effective_variant, "poster" | "stacked-badge") {
        let heading_html = if !heading.is_empty() {
            format!(
                r#"<h2 style="font-family:{};font-size:var(--text-xl);font-weight:800;color:{};margin:0;line-height:1.12;text-align:center;overflow-wrap:break-word;word-break:break-word;">{}</h2>"#,
                tokens.heading_font,
                colors.text_primary,
                escape_html(heading)
            )
        } else {
            String::new()
        };
        let caption_html = if !caption.is_empty() {
            format!(
                r#"<p style="font-family:{};font-size:var(--text-sm);line-height:1.5;color:{};margin:0;text-align:center;overflow-wrap:break-word;word-break:break-word;">{}</p>"#,
                tokens.body_font,
                colors.text_secondary,
                escape_html(caption)
            )
        } else {
            String::new()
        };
        let incentive_html = if !incentive_text.is_empty() {
            format!(
                r#"<div style="font-family:{};font-size:var(--text-sm);font-weight:800;color:{};background:{};border:1px solid {};border-radius:{};padding:var(--space-1) var(--space-2);text-align:center;">{}</div>"#,
                tokens.body_font,
                colors.text_primary,
                if colors.is_dark {
                    "rgba(255,255,255,0.06)"
                } else {
                    "rgba(0,0,0,0.035)"
                },
                colors.border,
                current_component_radius(tokens, "chip"),
                escape_html(incentive_text)
            )
        } else {
            String::new()
        };
        let stack_order = if effective_variant == "stacked-badge" {
            format!(
                r#"{heading_html}{incentive_html}<div style="display:flex;flex-direction:column;align-items:center;">{brand_html}{qr_html}{cta_html}</div>{caption_html}"#
            )
        } else {
            format!(
                r#"{brand_html}{heading_html}{caption_html}<div style="display:flex;flex-direction:column;align-items:center;">{qr_html}{cta_html}</div>{incentive_html}"#
            )
        };
        let content = format!(
            r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:var(--space-2);overflow:hidden;">{}</div>"#,
            stack_order
        );
        slide_base(&content, tokens, bg_style, false, layout_padding, "center")
    } else {
        let mut left_elements = Vec::new();

        let include_heading = matches!(
            effective_variant,
            "full-conversion" | "theme-bg" | "image-bg" | "with-heading" | "split-card"
        );
        let include_caption = matches!(
            effective_variant,
            "full-conversion"
                | "theme-bg"
                | "image-bg"
                | "with-caption"
                | "without-heading"
                | "split-card"
        );
        let include_incentive = matches!(
            effective_variant,
            "full-conversion" | "theme-bg" | "image-bg" | "without-heading" | "split-card"
        );

        if include_heading && !heading.is_empty() {
            let h_html = format!(
                r#"<h2 style="font-family:{};font-size:var(--text-lg);font-weight:700;color:{};margin:0 0 8px;line-height:1.25;letter-spacing:-0.015em;overflow-wrap:break-word;word-break:break-word;">{}</h2>"#,
                tokens.heading_font,
                colors.text_primary,
                escape_html(heading)
            );
            left_elements.push(h_html);
        }

        if include_caption && !caption.is_empty() {
            let c_html = format!(
                r#"<p style="font-family:{};font-size:var(--text-base);line-height:1.55;color:{};margin:0;overflow-wrap:break-word;word-break:break-word;">{}</p>"#,
                tokens.body_font,
                colors.text_secondary,
                escape_html(caption)
            );
            left_elements.push(c_html);
        }

        if include_incentive && !incentive_text.is_empty() {
            let badge_radius = current_component_radius(tokens, "chip");
            let inc_html = format!(
                r#"<div style="display:inline-flex;align-items:center;gap:var(--space-1);background:{};border:1px solid {};border-radius:{};padding:var(--space-1) var(--space-2);font-family:{};font-size:var(--text-sm);font-weight:700;color:{};align-self:flex-start;">
                    <span style="color:{};">🎁</span>
                    <span>{}</span>
                </div>"#,
                if colors.is_dark {
                    "rgba(255,255,255,0.06)"
                } else {
                    "rgba(0,0,0,0.03)"
                },
                colors.border,
                badge_radius,
                tokens.body_font,
                colors.text_primary,
                colors.primary,
                escape_html(incentive_text)
            );
            left_elements.push(inc_html);
        }

        let left_col = format!(
            r#"<div style="display:flex;flex-direction:column;justify-content:center;gap:var(--space-2);height:100%;min-width:0;">{}</div>"#,
            left_elements.join("\n")
        );

        let right_col = format!(
            r#"<div style="display:flex;flex-direction:column;align-items:center;justify-content:center;height:100%;min-width:0;">
                {}
                {}
                {}
            </div>"#,
            brand_html, qr_html, cta_html
        );

        let grid_cols = if effective_variant == "split-card" {
            "minmax(0, 1fr) minmax(0, 0.82fr)"
        } else {
            "1.2fr 1fr"
        };
        let content = format!(
            r#"<div style="display:grid;grid-template-columns:{};gap:var(--space-5);overflow:hidden;align-items:center;"><div style="min-width:0;overflow:hidden;">{}</div><div style="min-width:0;overflow:hidden;">{}</div></div>"#,
            grid_cols, left_col, right_col
        );
        slide_base(&content, tokens, bg_style, false, layout_padding, "center")
    };

    let bg_img_to_inject = if effective_variant == "image-bg" {
        background_image
    } else {
        ""
    };
    let html = inject_background_image(html, bg_img_to_inject, image_opacity, is_dark);

    json!({
        "html": html,
        "background": bg_style,
        "variant": effective_variant,
        "theme": theme
    })
}

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
    CURRENT_THEME.with(|t| *t.borrow_mut() = theme.to_string());
    CURRENT_ARCHETYPE.with(|a| *a.borrow_mut() = _archetype.to_string());
    CURRENT_TOKENS.with(|tok| *tok.borrow_mut() = Some(tokens.clone()));
    CURRENT_BG_STYLE.with(|bg| *bg.borrow_mut() = bg_style.to_string());
    CURRENT_PARAMS.with(|current| *current.borrow_mut() = params.clone());

    let p = params;
    let s = |key: &str| {
        p.get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };
    let b = |key: &str, default: bool| p.get(key).and_then(|v| v.as_bool()).unwrap_or(default);
    let f = |key: &str, default: f32| -> f32 {
        p.get(key)
            .and_then(|v| v.as_f64())
            .map(|x| x as f32)
            .unwrap_or(default)
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
            let items = p
                .get("items")
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
            let columns: Vec<String> = p
                .get("columns")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();
            let rows: Vec<Vec<String>> = p
                .get("rows")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_array())
                        .map(|inner| {
                            inner.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .collect()
                })
                .unwrap_or_default();
            let highlight_column = p.get("highlight_column").and_then(|v| v.as_u64()).map(|n| n as usize);
            let show_checkmarks = p.get("show_checkmarks").and_then(|v| v.as_bool()).unwrap_or(false);
            Ok(comparison_slide(
                tokens,
                &s("title"),
                columns,
                rows,
                highlight_column,
                show_checkmarks,
                bg_style,
                &s("variant").if_empty("table"),
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "stat_row" => {
            let stats = p
                .get("stats")
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
            let steps = p
                .get("steps")
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
            let features = p
                .get("features")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(split_features_slide(
                tokens,
                &s("title"),
                features,
                &s("left_content_html"),
                &s("image_url"),
                bg_style,
                &s("variant").if_empty("default"),
                &bg_img,
                img_opacity,
                theme,
                _archetype,
                &s("padding"),
            ))
        }
        "grid_cards" => {
            let cards = p
                .get("cards")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(grid_cards_slide(
                tokens,
                &s("title"),
                cards,
                bg_style,
                &s("variant").if_empty("default"),
                &bg_img,
                img_opacity,
                theme,
                _archetype,
                &s("padding"),
            ))
        }
        "headline_subheadline" => Ok(headline_subheadline_slide(
            tokens,
            &s("headline"),
            &s("subheadline"),
            bg_style,
            theme,
            &bg_img,
            img_opacity,
        )),
        "definition" => Ok(definition_slide(
            tokens,
            &s("term"),
            &s("definition"),
            &s("context"),
            bg_style,
            theme,
            &bg_img,
            img_opacity,
        )),
        "text_block" => Ok(text_block_slide(
            tokens,
            &s("title"),
            &s("body"),
            bg_style,
            theme,
            &bg_img,
            img_opacity,
        )),
        "metric_card" => Ok(metric_card_slide(
            tokens,
            &s("value").if_empty(&s("metric")),
            &s("label"),
            &s("trend"),
            &s("context"),
            bg_style,
            theme,
            &bg_img,
            img_opacity,
        )),
        "chart" => {
            let data = p
                .get("data")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(chart_slide(
                tokens,
                &s("chart_type").if_empty("bar"),
                data,
                &s("title"),
                &s("caption"),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "scatter_plot" => {
            let data = p
                .get("data")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(scatter_plot_slide(
                tokens,
                data,
                &s("title"),
                &s("x_label"),
                &s("y_label"),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "gauge" => Ok(gauge_slide(
            tokens,
            f("value", 0.0) as f64,
            &s("label"),
            &s("title"),
            bg_style,
            theme,
            &bg_img,
            img_opacity,
        )),
        "radar_chart" => {
            let data = p
                .get("data")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(radar_chart_slide(
                tokens,
                data,
                &s("title"),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "text_columns" => {
            let columns = p
                .get("columns")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(text_columns_slide(
                tokens,
                &s("title"),
                columns,
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "progress_rings" => {
            let items = p
                .get("items")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(progress_rings_slide(
                tokens,
                items,
                &s("title"),
                &s("description"),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "comparison_bars" => {
            let comparison = p.get("comparison").cloned().unwrap_or_else(|| {
                p.get("metrics")
                    .and_then(|v| v.as_array())
                    .and_then(|metrics| metrics.first())
                    .map(|metric| {
                        json!({
                            "left": {
                                "label": metric.get("left_label").and_then(|v| v.as_str()).unwrap_or("Before"),
                                "value": metric.get("left_value").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                "unit": metric.get("unit").and_then(|v| v.as_str()).unwrap_or("")
                            },
                            "right": {
                                "label": metric.get("right_label").and_then(|v| v.as_str()).unwrap_or("After"),
                                "value": metric.get("right_value").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                "unit": metric.get("unit").and_then(|v| v.as_str()).unwrap_or("")
                            }
                        })
                    })
                    .unwrap_or_else(|| json!({}))
            });
            Ok(comparison_bars_slide(
                tokens,
                comparison,
                &s("title"),
                &s("description"),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "metric_grid" => {
            let metrics = p
                .get("metrics")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(metric_grid_slide(
                tokens,
                metrics,
                &s("title"),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "funnel_chart" => {
            let steps = p
                .get("steps")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(funnel_chart_slide(
                tokens,
                steps,
                &s("title"),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "table" => {
            let headers = p
                .get("headers")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            let rows = p
                .get("rows")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(table_slide(
                tokens,
                headers,
                rows,
                &s("title"),
                &s("caption"),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "metric_sparkline" => {
            let data = p
                .get("data")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(metric_sparkline_slide(
                tokens,
                &s("value").if_empty(&s("metric")),
                &s("label"),
                data,
                &s("trend"),
                &s("context"),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "column_chart" => {
            let data = p
                .get("data")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(column_chart_slide(
                tokens,
                data,
                &s("title"),
                &s("caption"),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "section_divider" => Ok(section_divider_slide(
            tokens,
            &s("title").if_empty(&s("headline")),
            &s("subtitle").if_empty(&s("subheadline")),
            &s("kicker").if_empty(&s("label")),
            bg_style,
            theme,
            &bg_img,
            img_opacity,
        )),
        "problem_solution" => {
            let proof_points = p
                .get("proof_points")
                .or_else(|| p.get("points"))
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(problem_solution_slide(
                tokens,
                &s("title"),
                &s("problem"),
                &s("solution"),
                proof_points,
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "myth_fact" => Ok(myth_fact_slide(
            tokens,
            &s("myth"),
            &s("fact"),
            &s("explanation"),
            bg_style,
            &s("variant").if_empty("split"),
            theme,
            &bg_img,
            img_opacity,
        )),
        "checklist_action_plan" => {
            let items = p
                .get("items")
                .or_else(|| p.get("steps"))
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(checklist_action_plan_slide(
                tokens,
                &s("title"),
                items,
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "case_study_result" => {
            let results = p
                .get("results")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(case_study_result_slide(
                tokens,
                &s("client").if_empty(&s("title")),
                &s("challenge"),
                &s("solution"),
                results,
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "pricing_plan" => {
            let plans = p
                .get("plans")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(pricing_plan_slide(
                tokens,
                &s("title"),
                plans,
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "testimonial_avatar" => Ok(testimonial_avatar_slide(
            tokens,
            &s("quote"),
            &s("author"),
            &s("role"),
            &s("avatar_url"),
            bg_style,
            theme,
            &bg_img,
            img_opacity,
        )),
        "logo_cloud" => {
            let logos = p
                .get("logos")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(logo_cloud_slide(
                tokens,
                &s("title"),
                logos,
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "faq" => {
            let questions = p
                .get("questions")
                .or_else(|| p.get("items"))
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(faq_slide(
                tokens,
                &s("title"),
                questions,
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "process_map" => {
            let steps = p
                .get("steps")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(process_map_slide(
                tokens,
                &s("title"),
                steps,
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "before_after_story" => Ok(before_after_story_slide(
            tokens,
            &s("title"),
            &s("before"),
            &s("after"),
            &s("metric"),
            bg_style,
            theme,
            &bg_img,
            img_opacity,
        )),
        "image_caption" => Ok(image_caption_slide(
            tokens,
            &s("image_url"),
            &s("caption"),
            &s("description"),
            &s("layout").if_empty("image-top"),
            bg_style,
            &bg_img,
            img_opacity,
            theme,
            _archetype,
            &s("padding"),
        )),
        "image_headline" => Ok(image_headline_slide(
            tokens,
            &s("image_url"),
            &s("headline"),
            &s("subheadline"),
            &s("overlay_position").if_empty("bottom"),
            bg_style,
            &bg_img,
            img_opacity,
            theme,
            _archetype,
            &s("padding"),
        )),
        "image_quote" => Ok(image_quote_slide(
            tokens,
            &s("image_url"),
            &s("quote"),
            &s("author"),
            &s("role"),
            bg_style,
            &bg_img,
            img_opacity,
            theme,
            _archetype,
            &s("padding"),
        )),
        "image_callout" => {
            let callouts = p
                .get("callouts")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(image_callout_slide(
                tokens,
                &s("image_url"),
                callouts,
                &s("description"),
                bg_style,
                &bg_img,
                img_opacity,
                theme,
                _archetype,
                &s("padding"),
            ))
        }
        "image_stat" => Ok(image_stat_slide(
            tokens,
            &s("image_url"),
            &s("stat_value").if_empty(&s("value")),
            &s("stat_label").if_empty(&s("label")),
            &s("description"),
            &s("layout").if_empty("image-left"),
            bg_style,
            &bg_img,
            img_opacity,
            theme,
            _archetype,
            &s("padding"),
        )),
        "image_gallery" => {
            let images = p
                .get("images")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(image_gallery_slide(
                tokens,
                images,
                &s("layout").if_empty("2-grid"),
                &s("title"),
                &s("section_caption").if_empty(&s("caption")),
                bg_style,
                &bg_img,
                img_opacity,
                theme,
                _archetype,
                &s("padding"),
            ))
        }
        "image_collage" => {
            let images = p
                .get("images")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(image_collage_slide(
                tokens,
                images,
                &s("style").if_empty("scattered"),
                &s("title"),
                &s("section_caption").if_empty(&s("caption")),
                bg_style,
                &bg_img,
                img_opacity,
                theme,
                _archetype,
                &s("padding"),
            ))
        }
        "image_comparison" => Ok(image_comparison_slide(
            tokens,
            &s("before_image"),
            &s("after_image"),
            &s("before_label").if_empty("Before"),
            &s("after_label").if_empty("After"),
            &s("description"),
            &s("divider_style").if_empty("line"),
            bg_style,
            &bg_img,
            img_opacity,
            theme,
            _archetype,
            &s("padding"),
        )),
        "qr_destination" => Ok(qr_destination_slide(
            tokens,
            &s("destination_url").if_empty(&s("url")),
            &s("heading").if_empty(&s("headline")),
            &s("caption").if_empty(&s("description")),
            &s("cta_text").if_empty(&s("button_text").if_empty("Scan to open")),
            &s("short_url"),
            &s("incentive_text"),
            &s("variant").if_empty("full-conversion"),
            bg_style,
            &bg_img,
            img_opacity,
            theme,
            _archetype,
            &s("padding"),
            &s("brand_name"),
            &s("brand_logo"),
            &s("qr_alt_text"),
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
        if self.is_empty() {
            default.to_string()
        } else {
            self
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// New Image-Specific Slide Types
// ─────────────────────────────────────────────────────────────────────────────

pub fn image_caption_slide(
    tokens: &DesignTokens,
    image_url: &str,
    caption: &str,
    description: &str,
    layout: &str,
    bg_style: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
    archetype: &str,
    padding: &str,
) -> Value {
    let mut treatment = resolve_current_image_treatment(theme, archetype);
    if treatment.image_mask == "circle"
        || treatment.image_frame == "circle"
        || treatment.image_frame == "pill"
    {
        treatment.image_mask = "none".to_string();
        if treatment.image_frame != "sharp" {
            treatment.image_frame = "rounded".to_string();
        }
    }
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let img_height = if layout == "image-left" || layout == "image-right" {
        "100%"
    } else {
        "240px"
    };

    let img_html = render_themed_image(
        image_url, tokens, &treatment, "100%", img_height, caption, is_dark,
    );

    let caption_style = format!(
        "font-family:{};font-size:{}px;font-weight:700;color:{};margin:0 0 8px;line-height:1.2;",
        tokens.heading_font,
        tokens
            .type_scale
            .get("title")
            .map(|t| t.font_size)
            .unwrap_or(24),
        colors.text_primary
    );
    let desc_style = format!(
        "font-family:{};font-size:var(--text-sm);color:{};margin:0;line-height:1.45;",
        tokens.body_font, colors.text_secondary
    );

    let text_html = format!(
        r#"<div style="display:flex;flex-direction:column;justify-content:center;">
            <h3 style="{}">{}</h3>
            {}
        </div>"#,
        caption_style,
        escape_html(caption),
        if !description.is_empty() {
            format!(
                r#"<p style="{}">{}</p>"#,
                desc_style,
                escape_html(description)
            )
        } else {
            String::new()
        }
    );

    let content = match layout {
        "image-bottom" => {
            format!(
                r#"<div style="display:flex;flex-direction:column;gap:var(--space-2);width:100%;height:100%;justify-content:center;">
                    {}
                    {}
                </div>"#,
                text_html, img_html
            )
        }
        "image-left" => {
            format!(
                r#"<div style="display:grid;grid-template-columns:1.2fr 1fr;gap:var(--space-2);width:100%;height:100%;align-items:center;">
                    {}
                    {}
                </div>"#,
                img_html, text_html
            )
        }
        "image-right" => {
            format!(
                r#"<div style="display:grid;grid-template-columns:1fr 1.2fr;gap:var(--space-2);width:100%;height:100%;align-items:center;">
                    {}
                    {}
                </div>"#,
                text_html, img_html
            )
        }
        "image-overlay" => {
            let mut overlay_treatment = treatment.clone();
            overlay_treatment.image_frame = "sharp".to_string();
            overlay_treatment.image_mask = "none".to_string();
            overlay_treatment.image_overlay = "solid".to_string();
            let img_full = render_themed_image(
                image_url,
                tokens,
                &overlay_treatment,
                "100%",
                "100%",
                caption,
                true,
            );
            let overlay_content = format!(
                r#"<div style="position:relative;width:100%;height:100%;">
                    {}
                    <div style="position:absolute;bottom:0;left:0;right:0;padding:var(--space-3) 28px 84px 28px;z-index:3;color:white;text-shadow:0 2px 4px rgba(0,0,0,0.5);">
                        <h3 style="font-family:{};font-size:{}px;font-weight:700;color:white;margin:0 0 8px;text-shadow:0 2px 4px rgba(0,0,0,0.5);">{}</h3>
                        {}
                    </div>
                </div>"#,
                img_full,
                tokens.heading_font,
                tokens
                    .type_scale
                    .get("title")
                    .map(|t| t.font_size)
                    .unwrap_or(24),
                escape_html(caption),
                if !description.is_empty() {
                    format!(
                        r#"<p style="font-family:{};font-size:var(--text-sm);color:rgba(255,255,255,0.8);margin:0;line-height:1.4;text-shadow:0 2px 4px rgba(0,0,0,0.5);">{}</p>"#,
                        tokens.body_font,
                        escape_html(description)
                    )
                } else {
                    String::new()
                }
            );
            let html = slide_base(&overlay_content, tokens, "dark", false, "0", "stretch");
            return json!({
                "html": html,
                "background": "dark",
                "variant": layout,
                "theme": theme,
                "archetype": archetype
            });
        }
        _ => {
            // image-top
            format!(
                r#"<div style="display:flex;flex-direction:column;gap:var(--space-2);width:100%;height:100%;justify-content:center;">
                    {}
                    {}
                </div>"#,
                img_html, text_html
            )
        }
    };

    let padding_val = if padding.is_empty() {
        "80px var(--space-6) 80px"
    } else {
        padding
    };
    let html = slide_base(&content, tokens, bg_style, false, padding_val, "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": layout,
        "theme": theme,
        "archetype": archetype
    })
}

pub fn image_headline_slide(
    tokens: &DesignTokens,
    image_url: &str,
    headline: &str,
    subheadline: &str,
    overlay_position: &str,
    bg_style: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
    archetype: &str,
    padding: &str,
) -> Value {
    let mut treatment = resolve_current_image_treatment(theme, archetype);
    if treatment.image_mask == "circle"
        || treatment.image_frame == "circle"
        || treatment.image_frame == "pill"
    {
        treatment.image_mask = "none".to_string();
        if treatment.image_frame != "sharp" {
            treatment.image_frame = "rounded".to_string();
        }
    }

    treatment.image_frame = "sharp".to_string();
    treatment.image_mask = "none".to_string();
    treatment.image_overlay = "solid".to_string();

    let img_html = render_themed_image(
        image_url, tokens, &treatment, "100%", "100%", headline, true,
    );

    let v_align = match overlay_position {
        "center" => "center",
        "top" => "flex-start",
        _ => "flex-end",
    };

    let headline_style = format!(
        "font-family:{};font-size:{}px;font-weight:800;color:white;margin:0;line-height:1.15;text-shadow:0 2px 8px rgba(0,0,0,0.6);",
        tokens.heading_font,
        tokens
            .type_scale
            .get("display")
            .map(|t| t.font_size)
            .unwrap_or(40)
    );
    let sub_style = format!(
        "font-family:{};font-size:var(--text-sm);color:rgba(255,255,255,0.85);margin:var(--space-1) 0 0;line-height:1.45;text-shadow:0 2px 8px rgba(0,0,0,0.6);",
        tokens.body_font
    );

    let content = format!(
        r#"<div style="position:relative;width:100%;height:100%;">
            {}
            <div style="position:absolute;inset:0;padding:var(--space-10) var(--space-5) var(--space-10);display:flex;flex-direction:column;justify-content:{};z-index:3;text-shadow:0 2px 8px rgba(0,0,0,0.6);">
                <h2 style="{}">{}</h2>
                {}
            </div>
        </div>"#,
        img_html,
        v_align,
        headline_style,
        escape_html(headline),
        if !subheadline.is_empty() {
            format!(
                r#"<p style="{}">{}</p>"#,
                sub_style,
                escape_html(subheadline)
            )
        } else {
            String::new()
        }
    );

    let html = slide_base_bleed(&content, tokens, "dark", false, "0", "stretch");
    json!({
        "html": html,
        "background": "dark",
        "variant": overlay_position,
        "theme": theme,
        "archetype": archetype
    })
}

pub fn image_quote_slide(
    tokens: &DesignTokens,
    image_url: &str,
    quote: &str,
    author: &str,
    role: &str,
    bg_style: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
    archetype: &str,
    padding: &str,
) -> Value {
    let mut treatment = resolve_current_image_treatment(theme, archetype);
    treatment.image_frame = "sharp".to_string();
    treatment.image_mask = "none".to_string();
    treatment.image_overlay = "solid".to_string();

    let img_html = render_themed_image(image_url, tokens, &treatment, "100%", "100%", quote, true);

    let quote_style = format!(
        "font-family:{};font-size:{}px;font-style:italic;font-weight:600;color:white;margin:0 0 16px;line-height:1.45;text-shadow:0 2px 8px rgba(0,0,0,0.6);",
        tokens.heading_font,
        tokens
            .type_scale
            .get("headline")
            .map(|t| t.font_size)
            .unwrap_or(28)
    );
    let author_style = format!(
        "font-family:{};font-size:var(--text-sm);font-weight:600;color:white;margin:0;text-shadow:0 2px 8px rgba(0,0,0,0.6);",
        tokens.body_font
    );
    let role_style = format!(
        "font-family:{};font-size:11px;color:rgba(255,255,255,0.7);margin:var(--space-0) 0 0;text-shadow:0 2px 8px rgba(0,0,0,0.6);",
        tokens.body_font
    );

    let content = format!(
        r#"<div style="position:relative;width:100%;height:100%;">
            {}
            <div style="position:absolute;inset:0;padding:var(--space-10) var(--space-5) var(--space-10);display:flex;flex-direction:column;justify-content:center;align-items:center;text-align:center;z-index:3;text-shadow:0 2px 8px rgba(0,0,0,0.6);">
                <div style="font-size:var(--text-2xl);color:rgba(255,255,255,0.4);line-height:1;margin-bottom:var(--space-1);">“</div>
                <p style="{}">{}</p>
                {}
                {}
            </div>
        </div>"#,
        img_html,
        quote_style,
        escape_html(quote),
        if !author.is_empty() {
            format!(r#"<p style="{}">{}</p>"#, author_style, escape_html(author))
        } else {
            String::new()
        },
        if !role.is_empty() {
            format!(r#"<p style="{}">{}</p>"#, role_style, escape_html(role))
        } else {
            String::new()
        }
    );

    let html = slide_base_bleed(&content, tokens, "dark", false, "0", "stretch");
    json!({
        "html": html,
        "background": "dark",
        "variant": "default",
        "theme": theme,
        "archetype": archetype
    })
}

pub fn image_callout_slide(
    tokens: &DesignTokens,
    image_url: &str,
    callouts: Vec<Value>,
    description: &str,
    bg_style: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
    archetype: &str,
    padding: &str,
) -> Value {
    let mut treatment = resolve_current_image_treatment(theme, archetype);
    if treatment.image_mask == "circle"
        || treatment.image_frame == "circle"
        || treatment.image_frame == "pill"
    {
        treatment.image_mask = "none".to_string();
        if treatment.image_frame != "sharp" {
            treatment.image_frame = "rounded".to_string();
        }
    }
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let img_html = render_themed_image(
        image_url,
        tokens,
        &treatment,
        "100%",
        "240px",
        "Annotated Diagram",
        is_dark,
    );

    let mut markers = String::new();
    for (idx, c) in callouts.iter().enumerate() {
        let x = c.get("x").and_then(|v| v.as_f64()).unwrap_or(50.0);
        let y = c.get("y").and_then(|v| v.as_f64()).unwrap_or(50.0);
        let lbl = c.get("label").and_then(|v| v.as_str()).unwrap_or("");
        markers.push_str(&format!(
            r#"<div style="position:absolute;left:{:.1}%;top:{:.1}%;transform:translate(-50%,-50%);z-index:4;">
                <div style="width:24px;height:24px;border-radius:50%;background:{};color:white;display:flex;align-items:center;justify-content:center;font-family:{};font-size:12px;font-weight:700;box-shadow:0 0 0 4px rgba(255,255,255,0.4), 0 2px 8px rgba(0,0,0,0.3);cursor:pointer;" title="{}">
                    {}
                </div>
            </div>"#,
            x, y, colors.primary, tokens.body_font, escape_html(lbl), idx + 1
        ));
    }

    let desc_html = if !description.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:var(--text-sm);color:{};margin:var(--space-2) 0 0;line-height:1.45;text-align:left;">{}</p>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(description)
        )
    } else {
        String::new()
    };

    let mut list_html = String::new();
    if !callouts.is_empty() {
        let mut items = String::new();
        for (idx, c) in callouts.iter().enumerate() {
            let lbl = c.get("label").and_then(|v| v.as_str()).unwrap_or("");
            let d = c.get("description").and_then(|v| v.as_str()).unwrap_or("");
            items.push_str(&format!(
                r#"<div style="margin-bottom:12px;display:flex;gap:var(--space-1);align-items:flex-start;">
                    <span style="display:inline-flex;align-items:center;justify-content:center;width:18px;height:18px;border-radius:50%;background:{};color:white;font-family:{};font-size:10px;font-weight:700;flex-shrink:0;margin-top:2px;">{}</span>
                    <div>
                        <h4 style="font-family:{};font-size:var(--text-sm);font-weight:600;color:{};margin:0 0 2px;">{}</h4>
                        {}
                    </div>
                </div>"#,
                colors.primary, tokens.body_font, idx + 1,
                tokens.body_font, colors.text_primary, escape_html(lbl),
                if !d.is_empty() {
                    format!(r#"<p style="font-family:{};font-size:11px;color:{};margin:0;line-height:1.45;">{}</p>"#, tokens.body_font, colors.text_secondary, escape_html(d))
                } else {
                    String::new()
                }
            ));
        }
        list_html = format!(
            r#"<div style="margin-top:20px;display:flex;flex-direction:column;text-align:left;">{}</div>"#,
            items
        );
    }

    let content = format!(
        r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;justify-content:center;">
            <div style="position:relative;width:100%;height:240px;">
                {}
                {}
            </div>
            {}
            {}
        </div>"#,
        img_html, markers, desc_html, list_html
    );

    let padding_val = if padding.is_empty() {
        "80px var(--space-6) 80px"
    } else {
        padding
    };
    let html = slide_base(&content, tokens, bg_style, false, padding_val, "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": "default",
        "theme": theme,
        "archetype": archetype
    })
}

pub fn image_stat_slide(
    tokens: &DesignTokens,
    image_url: &str,
    stat_value: &str,
    stat_label: &str,
    description: &str,
    layout: &str,
    bg_style: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
    archetype: &str,
    padding: &str,
) -> Value {
    let mut treatment = resolve_current_image_treatment(theme, archetype);
    if treatment.image_mask == "circle"
        || treatment.image_frame == "organic"
        || treatment.image_frame == "circle"
        || treatment.image_frame == "pill"
    {
        treatment.image_mask = "none".to_string();
        treatment.image_frame = "rounded".to_string();
    }
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let img_height = if layout == "image-left" || layout == "image-right" {
        "100%"
    } else {
        "220px"
    };

    let img_html = render_themed_image(
        image_url, tokens, &treatment, "100%", img_height, stat_label, is_dark,
    );

    let val_style = format!(
        "font-family:{};font-size:48px;font-weight:800;color:{};margin:0 0 4px;line-height:1;",
        tokens.heading_font, colors.primary
    );
    let lbl_style = format!(
        "font-family:{};font-size:var(--text-sm);font-weight:600;color:{};margin:0 0 8px;line-height:1.2;text-transform:uppercase;letter-spacing:0.05em;",
        tokens.body_font, colors.text_primary
    );
    let desc_style = format!(
        "font-family:{};font-size:12px;color:{};margin:0;line-height:1.4;",
        tokens.body_font, colors.text_secondary
    );

    let stat_html = format!(
        r#"<div style="display:flex;flex-direction:column;justify-content:center;text-align:left;">
            <span style="{}">{}</span>
            <span style="{}">{}</span>
            {}
        </div>"#,
        val_style,
        escape_html(stat_value),
        lbl_style,
        escape_html(stat_label),
        if !description.is_empty() {
            format!(
                r#"<p style="{}">{}</p>"#,
                desc_style,
                escape_html(description)
            )
        } else {
            String::new()
        }
    );

    let content = match layout {
        "image-right" => {
            format!(
                r#"<div style="display:grid;grid-template-columns:1fr 1fr;gap:24px;width:100%;height:100%;align-items:center;">
                    {}
                    {}
                </div>"#,
                stat_html, img_html
            )
        }
        "image-top" => {
            format!(
                r#"<div style="display:flex;flex-direction:column;gap:var(--space-2);width:100%;height:100%;justify-content:center;">
                    {}
                    {}
                </div>"#,
                img_html, stat_html
            )
        }
        "image-bottom" => {
            format!(
                r#"<div style="display:flex;flex-direction:column;gap:var(--space-2);width:100%;height:100%;justify-content:center;">
                    {}
                    {}
                </div>"#,
                stat_html, img_html
            )
        }
        _ => {
            // image-left
            format!(
                r#"<div style="display:grid;grid-template-columns:1fr 1fr;gap:24px;width:100%;height:100%;align-items:center;">
                    {}
                    {}
                </div>"#,
                img_html, stat_html
            )
        }
    };

    let padding_val = if padding.is_empty() {
        "80px var(--space-6) 80px"
    } else {
        padding
    };
    let html = slide_base(&content, tokens, bg_style, false, padding_val, "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": layout,
        "theme": theme,
        "archetype": archetype
    })
}

pub fn image_gallery_slide(
    tokens: &DesignTokens,
    images: Vec<Value>,
    layout: &str,
    title: &str,
    section_caption: &str,
    bg_style: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
    archetype: &str,
    padding: &str,
) -> Value {
    let mut treatment = resolve_current_image_treatment(theme, archetype);
    treatment.image_mask = "none".to_string();
    if treatment.image_frame == "circle" || treatment.image_frame == "pill" {
        treatment.image_frame = "rounded".to_string();
    }

    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let radius_md = current_component_radius(tokens, "frame");
    let shadow_sm = tokens
        .shadows
        .get("sm")
        .cloned()
        .unwrap_or_else(|| "none".to_string());
    let mut inner_treatment = treatment.clone();
    inner_treatment.image_frame = "sharp".to_string();
    inner_treatment.image_mask = "none".to_string();

    let mut img_cards = Vec::new();
    for img in &images {
        let url = img.get("url").and_then(|v| v.as_str()).unwrap_or("");
        let cap = img.get("caption").and_then(|v| v.as_str()).unwrap_or("");
        let img_html =
            render_themed_image(url, tokens, &inner_treatment, "100%", "100%", cap, is_dark);
        let caption_html = if !cap.is_empty() {
            format!(
                r#"<div style="padding:5px 8px;background:rgba(0,0,0,0.62);position:absolute;top:8px;left:8px;max-width:calc(100% - 16px);z-index:3;color:white;font-family:{};font-size:10px;font-weight:800;text-align:left;border-radius:999px;letter-spacing:0.04em;text-transform:uppercase;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;">
                    {}
                </div>"#,
                tokens.body_font,
                escape_html(cap)
            )
        } else {
            String::new()
        };
        img_cards.push(format!(
            r#"<div style="position:relative;width:100%;height:100%;border-radius:{};overflow:hidden;box-shadow:{};">
                {}
                {}
            </div>"#,
            radius_md, shadow_sm, img_html, caption_html
        ));
    }

    let has_header = !title.is_empty();
    let has_footer = !section_caption.is_empty();
    let any_captions = images.iter().any(|img| {
        img.get("caption")
            .and_then(|v| v.as_str())
            .map(|s| !s.is_empty())
            .unwrap_or(false)
    });

    let grid_height = if layout == "3-grid" && any_captions {
        if has_header || has_footer {
            "132px"
        } else {
            "185px"
        }
    } else {
        if has_header || has_footer {
            "172px"
        } else {
            "240px"
        }
    };

    let grid_html = if layout == "3-grid" {
        let mut three_cards = Vec::new();
        for img in images.iter().take(3) {
            let url = img.get("url").and_then(|v| v.as_str()).unwrap_or("");
            let cap = img.get("caption").and_then(|v| v.as_str()).unwrap_or("");
            let img_html =
                render_themed_image(url, tokens, &inner_treatment, "100%", "100%", cap, is_dark);
            let inner_cap = if !cap.is_empty() {
                format!(
                    r#"<div style="padding:5px 0 0;font-family:{};font-size:10px;font-weight:700;color:{};letter-spacing:0.04em;text-transform:uppercase;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;">{}</div>"#,
                    tokens.body_font,
                    colors.text_secondary,
                    escape_html(cap)
                )
            } else {
                String::new()
            };
            three_cards.push(format!(
                r#"<div style="display:flex;flex-direction:column;width:100%;">
                    <div style="position:relative;width:100%;height:{};border-radius:{};overflow:hidden;box-shadow:{};flex-shrink:0;">
                        {}
                    </div>
                    {}
                </div>"#,
                grid_height, radius_md, shadow_sm, img_html, inner_cap
            ));
        }
        format!(
            r#"<div style="display:grid;grid-template-columns:repeat(3, minmax(0, 1fr));gap:10px;width:100%;">{}</div>"#,
            three_cards.join(" ")
        )
    } else if layout == "4-grid" {
        format!(
            r#"<div style="display:grid;grid-template-columns:1fr 1fr;grid-template-rows:1fr 1fr;gap:var(--space-1);height:{};width:100%;">
                {}
            </div>"#,
            grid_height,
            img_cards
                .iter()
                .take(4)
                .cloned()
                .collect::<Vec<String>>()
                .join("")
        )
    } else if layout == "featured-1-2" && img_cards.len() >= 3 {
        format!(
            r#"<div style="display:grid;grid-template-columns:1.2fr 1fr;gap:var(--space-1);height:{};width:100%;">
                {}
                <div style="display:grid;grid-template-rows:1fr 1fr;gap:var(--space-1);height:100%;">
                    {}
                    {}
                </div>
            </div>"#,
            grid_height, img_cards[0], img_cards[1], img_cards[2]
        )
    } else if layout == "featured-2-1" && img_cards.len() >= 3 {
        format!(
            r#"<div style="display:grid;grid-template-columns:1fr 1.2fr;gap:var(--space-1);height:{};width:100%;">
                <div style="display:grid;grid-template-rows:1fr 1fr;gap:var(--space-1);height:100%;">
                    {}
                    {}
                </div>
                {}
            </div>"#,
            grid_height, img_cards[0], img_cards[1], img_cards[2]
        )
    } else {
        // 2-grid
        format!(
            r#"<div style="display:grid;grid-template-columns:1fr 1fr;gap:var(--space-1);height:{};width:100%;">
                {}
            </div>"#,
            grid_height,
            img_cards
                .iter()
                .take(2)
                .cloned()
                .collect::<Vec<String>>()
                .join("")
        )
    };

    let title_html = if !title.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:{}px;font-weight:800;color:{};margin-bottom:10px;letter-spacing:-0.01em;line-height:1.1;">{}</div>"#,
            tokens.heading_font,
            tokens
                .type_scale
                .get("title")
                .map(|t| t.font_size)
                .unwrap_or(24)
                .min(22),
            colors.text_primary,
            escape_html(title)
        )
    } else {
        String::new()
    };

    let caption_html = if !section_caption.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:{}px;color:{};margin-top:8px;line-height:1.25;">{}</div>"#,
            tokens.body_font,
            tokens
                .type_scale
                .get("caption")
                .map(|t| t.font_size)
                .unwrap_or(12),
            colors.text_secondary,
            escape_html(section_caption)
        )
    } else {
        String::new()
    };

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;justify-content:center;align-items:flex-start;">
            {}
            {}
            {}
        </div>"#,
        title_html, grid_html, caption_html
    );

    let padding_val = if padding.is_empty() {
        "80px var(--space-6) 80px"
    } else {
        padding
    };
    let html = slide_base(&content, tokens, bg_style, false, padding_val, "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": layout,
        "theme": theme,
        "archetype": archetype
    })
}

pub fn image_collage_slide(
    tokens: &DesignTokens,
    images: Vec<Value>,
    style: &str,
    title: &str,
    section_caption: &str,
    bg_style: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
    archetype: &str,
    padding: &str,
) -> Value {
    let mut treatment = resolve_current_image_treatment(theme, archetype);
    treatment.image_mask = "none".to_string();
    if treatment.image_frame == "circle" || treatment.image_frame == "pill" {
        treatment.image_frame = "rounded".to_string();
    }

    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let has_header = !title.is_empty();
    let has_footer = !section_caption.is_empty();
    let collage_height_px = if has_header || has_footer { 238 } else { 320 };

    struct CollageSlot {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        rot: i32,
        z: i32,
    }
    let image_count = images.len().min(4).max(1);
    let slots: Vec<CollageSlot> = if image_count <= 2 {
        let frame_h = collage_height_px - 22;
        vec![
            CollageSlot {
                x: 4,
                y: 10,
                w: 186,
                h: frame_h,
                rot: -2,
                z: 2,
            },
            CollageSlot {
                x: 202,
                y: 34,
                w: 110,
                h: frame_h - 48,
                rot: 3,
                z: 3,
            },
        ]
    } else if image_count == 3 {
        match style {
            "layered" | "editorial_stack" => vec![
                CollageSlot {
                    x: 4,
                    y: 14,
                    w: 184,
                    h: collage_height_px - 34,
                    rot: -2,
                    z: 2,
                },
                CollageSlot {
                    x: 202,
                    y: 10,
                    w: 110,
                    h: 116,
                    rot: 3,
                    z: 3,
                },
                CollageSlot {
                    x: 194,
                    y: 148,
                    w: 118,
                    h: 112,
                    rot: -3,
                    z: 4,
                },
            ],
            "geometric" | "mosaic" => vec![
                CollageSlot {
                    x: 4,
                    y: 8,
                    w: 148,
                    h: collage_height_px - 26,
                    rot: 0,
                    z: 1,
                },
                CollageSlot {
                    x: 164,
                    y: 8,
                    w: 148,
                    h: 124,
                    rot: 0,
                    z: 2,
                },
                CollageSlot {
                    x: 164,
                    y: 146,
                    w: 148,
                    h: collage_height_px - 164,
                    rot: 0,
                    z: 3,
                },
            ],
            "filmstrip" => vec![
                CollageSlot {
                    x: 4,
                    y: 20,
                    w: 96,
                    h: collage_height_px - 54,
                    rot: -2,
                    z: 1,
                },
                CollageSlot {
                    x: 110,
                    y: 8,
                    w: 96,
                    h: collage_height_px - 30,
                    rot: 0,
                    z: 2,
                },
                CollageSlot {
                    x: 216,
                    y: 20,
                    w: 96,
                    h: collage_height_px - 54,
                    rot: 2,
                    z: 3,
                },
            ],
            _ => vec![
                CollageSlot {
                    x: 4,
                    y: 16,
                    w: 154,
                    h: collage_height_px - 42,
                    rot: -3,
                    z: 2,
                },
                CollageSlot {
                    x: 172,
                    y: 8,
                    w: 140,
                    h: 122,
                    rot: 3,
                    z: 3,
                },
                CollageSlot {
                    x: 164,
                    y: 148,
                    w: 148,
                    h: collage_height_px - 166,
                    rot: -2,
                    z: 4,
                },
            ],
        }
    } else {
        match style {
            "geometric" | "mosaic" => vec![
                CollageSlot {
                    x: 4,
                    y: 8,
                    w: 148,
                    h: 124,
                    rot: 0,
                    z: 1,
                },
                CollageSlot {
                    x: 164,
                    y: 8,
                    w: 148,
                    h: 124,
                    rot: 0,
                    z: 2,
                },
                CollageSlot {
                    x: 4,
                    y: 146,
                    w: 148,
                    h: collage_height_px - 164,
                    rot: 0,
                    z: 3,
                },
                CollageSlot {
                    x: 164,
                    y: 146,
                    w: 148,
                    h: collage_height_px - 164,
                    rot: 0,
                    z: 4,
                },
            ],
            "filmstrip" => vec![
                CollageSlot {
                    x: 4,
                    y: 20,
                    w: 72,
                    h: collage_height_px - 54,
                    rot: -2,
                    z: 1,
                },
                CollageSlot {
                    x: 84,
                    y: 8,
                    w: 72,
                    h: collage_height_px - 30,
                    rot: 0,
                    z: 2,
                },
                CollageSlot {
                    x: 164,
                    y: 8,
                    w: 72,
                    h: collage_height_px - 30,
                    rot: 0,
                    z: 3,
                },
                CollageSlot {
                    x: 244,
                    y: 20,
                    w: 68,
                    h: collage_height_px - 54,
                    rot: 2,
                    z: 4,
                },
            ],
            _ => vec![
                CollageSlot {
                    x: 6,
                    y: 16,
                    w: 154,
                    h: 142,
                    rot: -3,
                    z: 2,
                },
                CollageSlot {
                    x: 172,
                    y: 8,
                    w: 138,
                    h: 112,
                    rot: 2,
                    z: 3,
                },
                CollageSlot {
                    x: 20,
                    y: 174,
                    w: 120,
                    h: 86,
                    rot: 2,
                    z: 4,
                },
                CollageSlot {
                    x: 154,
                    y: 140,
                    w: 156,
                    h: 120,
                    rot: -2,
                    z: 5,
                },
            ],
        }
    };

    let shadow_md = tokens
        .shadows
        .get("md")
        .cloned()
        .unwrap_or_else(|| "none".to_string());
    let radius_md = current_component_radius(tokens, "frame");
    let mut inner_treatment = treatment.clone();
    inner_treatment.image_frame = "sharp".to_string();
    inner_treatment.image_mask = "none".to_string();

    let mut img_html = String::new();
    for (idx, img) in images.iter().take(4).enumerate() {
        let url = img.get("url").and_then(|v| v.as_str()).unwrap_or("");
        let slot = &slots[idx % slots.len()];
        let x = slot.x;
        let y = slot.y;
        let w = slot.w;
        let h = slot.h;
        let rot = slot.rot;
        let z = slot.z;

        let themed_img = render_themed_image(
            url,
            tokens,
            &inner_treatment,
            "100%",
            "100%",
            &format!("Collage {}", idx + 1),
            is_dark,
        );

        img_html.push_str(&format!(
            r#"<div style="position:absolute;left:{}px;top:{}px;width:{}px;height:{}px;transform:rotate({}deg);z-index:{};box-shadow:{};border-radius:{};overflow:hidden;border:2px solid {};background:{};padding:3px;box-sizing:border-box;">
                {}
            </div>"#,
            x,
            y,
            w,
            h,
            rot,
            z,
            shadow_md,
            radius_md,
            if is_dark { "rgba(255,255,255,0.82)" } else { "rgba(255,255,255,0.96)" },
            if is_dark { "rgba(255,255,255,0.08)" } else { "rgba(255,255,255,0.94)" },
            themed_img
        ));
    }

    let collage_html = format!(
        r#"<div style="position:relative;width:316px;max-width:100%;height:{}px;margin:0 auto;box-sizing:border-box;">
            <div style="position:absolute;left:38px;top:26px;width:230px;height:210px;border-radius:{};background:{};opacity:{};z-index:0;"></div>
            {}
        </div>"#,
        collage_height_px,
        radius_md,
        colors.primary,
        if is_dark { "0.12" } else { "0.08" },
        img_html
    );

    let title_html = if !title.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:{}px;font-weight:800;color:{};margin-bottom:10px;letter-spacing:-0.01em;line-height:1.1;">{}</div>"#,
            tokens.heading_font,
            tokens
                .type_scale
                .get("title")
                .map(|t| t.font_size)
                .unwrap_or(24)
                .min(22),
            colors.text_primary,
            escape_html(title)
        )
    } else {
        String::new()
    };

    let caption_html = if !section_caption.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:{}px;color:{};margin-top:8px;line-height:1.25;">{}</div>"#,
            tokens.body_font,
            tokens
                .type_scale
                .get("caption")
                .map(|t| t.font_size)
                .unwrap_or(12),
            colors.text_secondary,
            escape_html(section_caption)
        )
    } else {
        String::new()
    };

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;justify-content:center;align-items:flex-start;">
            {}
            {}
            {}
        </div>"#,
        title_html, collage_html, caption_html
    );

    let padding_val = if padding.is_empty() {
        "80px var(--space-6) 80px"
    } else {
        padding
    };
    let html = slide_base(&content, tokens, bg_style, false, padding_val, "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": style,
        "theme": theme,
        "archetype": archetype
    })
}

pub fn image_comparison_slide(
    tokens: &DesignTokens,
    before_image: &str,
    after_image: &str,
    before_label: &str,
    after_label: &str,
    description: &str,
    divider_style: &str,
    bg_style: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
    archetype: &str,
    padding: &str,
) -> Value {
    let mut treatment = resolve_current_image_treatment(theme, archetype);
    treatment.image_mask = "none".to_string();
    treatment.image_frame = "sharp".to_string();

    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let left_img = render_themed_image(
        before_image,
        tokens,
        &treatment,
        "100%",
        "100%",
        before_label,
        is_dark,
    );
    let right_img = render_themed_image(
        after_image,
        tokens,
        &treatment,
        "100%",
        "100%",
        after_label,
        is_dark,
    );

    let lbl_style = format!(
        "background:rgba(0,0,0,0.7);backdrop-filter:blur(4px);-webkit-backdrop-filter:blur(4px);color:white;padding:5px 10px;font-family:{};font-size:10px;font-weight:700;border-radius:4px;position:absolute;top:12px;z-index:3;text-transform:uppercase;letter-spacing:0.06em;",
        tokens.body_font
    );

    let divider_html = if divider_style == "arrow" {
        format!(
            r#"<div style="position:absolute;left:50%;top:50%;transform:translate(-50%,-50%);width:36px;height:36px;border-radius:50%;background:{};color:white;display:flex;align-items:center;justify-content:center;font-size:var(--text-sm);z-index:4;box-shadow:0 4px 16px rgba(0,0,0,0.3);font-weight:bold;">
                ⇄
            </div>"#,
            colors.primary
        )
    } else {
        r#"<div style="position:absolute;left:50%;top:0;bottom:0;width:2px;background:white;z-index:4;box-shadow:0 0 8px rgba(0,0,0,0.4);transform:translateX(-50%);"></div>"#.to_string()
    };

    let desc_html = if !description.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:var(--text-sm);color:{};margin:var(--space-2) 0 0;line-height:1.45;text-align:center;width:100%;">{}</p>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(description)
        )
    } else {
        String::new()
    };

    let radius_lg = current_component_radius(tokens, "frame");
    let shadow_md = tokens
        .shadows
        .get("md")
        .cloned()
        .unwrap_or_else(|| "0 4px 16px rgba(0,0,0,0.15)".to_string());
    let grid_wrapper_style = format!(
        "position:relative;width:100%;height:220px;display:grid;grid-template-columns:1fr 1fr;gap:2px;border-radius:{};overflow:hidden;box-shadow:{};border: 1px solid {}30;background:{}20;",
        radius_lg, shadow_md, colors.border, colors.border
    );

    let content = format!(
        r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;justify-content:center;align-items:center;">
            <div style="{}">
                <div style="position:relative;width:100%;height:100%;">
                    {}
                    <span style="{}left:12px;">{}</span>
                </div>
                <div style="position:relative;width:100%;height:100%;">
                    {}
                    <span style="{}right:12px;">{}</span>
                </div>
                {}
            </div>
            {}
        </div>"#,
        grid_wrapper_style,
        left_img,
        lbl_style,
        escape_html(before_label),
        right_img,
        lbl_style,
        escape_html(after_label),
        divider_html,
        desc_html
    );

    let padding_val = if padding.is_empty() {
        "80px var(--space-6) 80px"
    } else {
        padding
    };
    let html = slide_base(&content, tokens, bg_style, false, padding_val, "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": divider_style,
        "theme": theme,
        "archetype": archetype
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design_system::derive_palette;

    #[test]
    fn test_qr_destination_slide_rendering() {
        let tokens = derive_palette(
            "#0066FF",
            "professional",
            16,
            1.25,
            "warm-editorial",
            "",
            None,
            None,
            None,
        )
        .unwrap();

        // 1. Test full-conversion layout
        let res = qr_destination_slide(
            &tokens,
            "https://example.com/dest",
            "Scan this QR code",
            "Some caption text about this conversion",
            "Scan now",
            "example.com/short",
            "Free Ebook included",
            "full-conversion",
            "dark",
            "",
            0.4,
            "minimal",
            "educator",
            "",
            "",
            "",
            "",
        );
        let html = res["html"].as_str().unwrap();
        assert!(html.contains("data:image/svg+xml;utf8,"));
        assert!(html.contains("Scan this QR code"));
        assert!(html.contains("Some caption text about this conversion"));
        assert!(html.contains("Scan now"));
        // short_url no longer rendered as visible text in QR card (removed per design spec)
        // The URL is still encoded in the QR code image itself
        let _ = &res; // ensure res is still used
        assert!(html.contains("Free Ebook included"));

        // 2. Test minimal variant
        let res_min = qr_destination_slide(
            &tokens,
            "https://example.com/dest",
            "Scan this QR code",
            "Some caption text about this conversion",
            "Scan now",
            "example.com/short",
            "Free Ebook included",
            "minimal",
            "light",
            "",
            0.4,
            "minimal",
            "educator",
            "",
            "",
            "",
            "",
        );
        let html_min = res_min["html"].as_str().unwrap();
        assert!(html_min.contains("data:image/svg+xml;utf8,"));
        assert!(!html_min.contains("Scan this QR code"));
        assert!(!html_min.contains("Some caption text about this conversion"));
        assert!(html_min.contains("Scan now"));
        // short_url text removed from QR card per design spec

        // 3. Test without-heading variant
        let res_no_h = qr_destination_slide(
            &tokens,
            "https://example.com/dest",
            "Scan this QR code",
            "Some caption text about this conversion",
            "Scan now",
            "example.com/short",
            "Free Ebook included",
            "without-heading",
            "light",
            "",
            0.4,
            "minimal",
            "educator",
            "",
            "",
            "",
            "",
        );
        let html_no_h = res_no_h["html"].as_str().unwrap();
        assert!(html_no_h.contains("data:image/svg+xml;utf8,"));
        assert!(!html_no_h.contains("Scan this QR code"));
        assert!(html_no_h.contains("Some caption text about this conversion"));
        assert!(html_no_h.contains("Scan now"));
        // short_url text removed from QR card per design spec

        // 4. Test custom padding, brand logo/name, alternative QR text, and variant filtering
        let res_custom = qr_destination_slide(
            &tokens,
            "https://example.com/dest",
            "Scan this QR code",
            "Some caption text about this conversion",
            "Scan now",
            "example.com/short",
            "Free Ebook included",
            "with-heading",
            "light",
            "",
            0.4,
            "minimal",
            "educator",
            "100px 50px 100px",
            "MyBrand",
            "https://example.com/logo.png",
            "Scan MyBrand QR Code",
        );
        let html_custom = res_custom["html"].as_str().unwrap();
        assert!(html_custom.contains("data:image/svg+xml;utf8,"));
        assert!(html_custom.contains("Scan this QR code"));
        assert!(!html_custom.contains("Some caption text about this conversion"));
        assert!(html_custom.contains("padding:100px 50px 100px;"));
        assert!(html_custom.contains("MyBrand"));
        assert!(html_custom.contains("https://example.com/logo.png"));
        assert!(html_custom.contains("alt=\"Scan MyBrand QR Code\""));

        // 5. Test with-cta variant layout behaves like minimal
        let res_with_cta = qr_destination_slide(
            &tokens,
            "https://example.com/dest",
            "Scan this QR code",
            "Some caption text about this conversion",
            "Scan now",
            "example.com/short",
            "Free Ebook included",
            "with-cta",
            "light",
            "",
            0.4,
            "minimal",
            "educator",
            "",
            "",
            "",
            "",
        );
        let html_with_cta = res_with_cta["html"].as_str().unwrap();
        assert!(html_with_cta.contains("data:image/svg+xml;utf8,"));
        assert!(!html_with_cta.contains("Scan this QR code"));
        assert!(!html_with_cta.contains("Some caption text about this conversion"));
        assert!(html_with_cta.contains("Scan now"));
        // short_url text removed from QR card per design spec
    }

    #[test]
    fn test_split_features_image_layout_uses_balanced_columns() {
        let tokens = derive_palette(
            "#0066FF",
            "professional",
            16,
            1.25,
            "warm-editorial",
            "",
            None,
            None,
            None,
        )
        .unwrap();

        let res = split_features_slide(
            &tokens,
            "Platform Benefits",
            vec![
                json!({"title": "Signal Quality", "description": "Clean analytics for product teams."}),
                json!({"title": "Operational Scale", "description": "Reliable rendering across channels."}),
            ],
            "",
            "https://images.unsplash.com/photo-1460925895917-afdab827c52f",
            "light",
            "default",
            "",
            0.4,
            "minimal",
            "data_analyst",
            "",
        );
        let html = res["html"].as_str().unwrap();
        // True 50/50 split: image fills left column full-height, heading +
        // cards stack in right column. No asymmetric column ratios.
        assert!(
            html.contains("grid-template-columns:1fr 1fr"),
            "split_features image layout should use a 50/50 column split"
        );
        // Image should fill its column (height:100%), not be fixed-height.
        assert!(
            html.contains("height:100%"),
            "image should fill its grid cell at 100% height"
        );
        // Heading should be inside the text column, not floating above.
        assert!(
            html.contains("flex-direction:column;justify-content:center"),
            "right column should be a centered flex column with heading + cards"
        );
    }

    #[test]
    fn test_column_chart_renders_values_and_varied_bar_heights() {
        let tokens = derive_palette(
            "#0066FF",
            "professional",
            16,
            1.25,
            "warm-editorial",
            "",
            None,
            None,
            None,
        )
        .unwrap();

        let res = column_chart_slide(
            &tokens,
            vec![
                json!({"label": "Jan", "value": 1200.0}),
                json!({"label": "Feb", "value": 1850.0}),
                json!({"label": "Mar", "value": 2700.0}),
            ],
            "Monthly Active Users",
            "Growth trajectory",
            "light",
            "minimal",
            "",
            0.4,
        );
        let html = res["html"].as_str().unwrap();
        assert!(html.contains(">1200<"));
        assert!(html.contains(">1850<"));
        assert!(html.contains(">2700<"));
        assert!(html.contains("height:44.4%"));
        assert!(html.contains("height:68.5%"));
        assert!(html.contains("height:100.0%"));
    }

    #[test]
    fn test_dispatch_comparison_bars_accepts_metrics_array() {
        let tokens = derive_palette(
            "#0066FF",
            "professional",
            16,
            1.25,
            "warm-editorial",
            "",
            None,
            None,
            None,
        )
        .unwrap();
        let params = json!({
            "title": "Performance Comparison",
            "metrics": [
                {"name": "Latency", "left_value": 120.0, "right_value": 45.0, "left_label": "Legacy", "right_label": "New"}
            ]
        });

        let res = dispatch_slide(
            "comparison_bars",
            &tokens,
            &params,
            "light",
            "minimal",
            "data_analyst",
        )
        .unwrap();
        let html = res["html"].as_str().unwrap();
        assert!(html.contains("Legacy"));
        assert!(html.contains("New"));
        assert!(html.contains(">120<"));
        assert!(html.contains(">45<"));
        assert!(html.contains("width:72.7%"));
        assert!(html.contains("width:27.3%"));
    }
}
