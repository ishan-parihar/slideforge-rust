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
    /// Which vertical zone the overlaid text occupies ("top" | "center" |
    /// "bottom"). The scrim gradient's dark end anchors to this zone so white
    /// text stays legible on bright/high-key images. Defaults to "bottom".
    #[serde(default)]
    pub overlay_anchor: String,
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
            image_frame: "sharp".to_string(),
            image_overlay: "none".to_string(),
            overlay_anchor: "bottom".to_string(),
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
            image_filter: "none".to_string(),
            image_position: "full-bleed".to_string(),
            image_frame: "sharp".to_string(),
            image_overlay: "none".to_string(),
            overlay_anchor: "bottom".to_string(),
            image_mix_blend: "normal".to_string(),
            image_mask: "fade-bottom".to_string(),
            image_animation: "none".to_string(),
            image_content: "photography".to_string(),
            image_opacity: None,
        }
    }
    pub fn bold_preset() -> Self {
        Self {
            image_filter: "none".to_string(),
            image_position: "full-bleed".to_string(),
            image_frame: "sharp".to_string(),
            image_overlay: "none".to_string(),
            overlay_anchor: "bottom".to_string(),
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
            overlay_anchor: "bottom".to_string(),
            image_mix_blend: "normal".to_string(),
            image_mask: "none".to_string(),
            image_animation: "none".to_string(),
            image_content: "photography".to_string(),
            image_opacity: None,
        }
    }
    pub fn dark_preset() -> Self {
        Self {
            image_filter: "none".to_string(),
            image_position: "full-bleed".to_string(),
            image_frame: "sharp".to_string(),
            image_overlay: "none".to_string(),
            overlay_anchor: "bottom".to_string(),
            image_mix_blend: "normal".to_string(),
            image_mask: "none".to_string(),
            image_animation: "subtle-zoom".to_string(),
            image_content: "photography".to_string(),
            image_opacity: None,
        }
    }
    pub fn vibrant_preset() -> Self {
        Self {
            image_filter: "none".to_string(),
            image_position: "full-bleed".to_string(),
            image_frame: "sharp".to_string(),
            image_overlay: "none".to_string(),
            overlay_anchor: "bottom".to_string(),
            image_mix_blend: "normal".to_string(),
            image_mask: "none".to_string(),
            image_animation: "ken-burns".to_string(),
            image_content: "abstract".to_string(),
            image_opacity: None,
        }
    }
    pub fn natural_preset() -> Self {
        Self {
            image_filter: "none".to_string(),
            image_position: "full-bleed".to_string(),
            image_frame: "sharp".to_string(),
            image_overlay: "none".to_string(),
            overlay_anchor: "bottom".to_string(),
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
    if fr == "rounded" || fr == "squircle" {
        frame_css = "border-radius: var(--radius-sm);".to_string();
    } else if fr == "pill" || fr == "circle" || fr == "organic" {
        frame_css = "border-radius: 9999px;".to_string();
    } else if fr == "polaroid" {
        frame_css = format!("border: var(--space-2) solid white; box-shadow: var(--shadow-md); border-radius: var(--radius-sm);");
    } else {
        frame_css = "border-radius: 0;".to_string();
    }

    // Overlay mapping — position-aware scrim: the dark end of the gradient
    // always sits under the text zone (top/center/bottom), anchored by
    // `treatment.overlay_anchor`. Minimum darkening inside the text band is
    // ~0.45 so bright/high-key photos never wash out white text; the rest of
    // the frame stays light to preserve the photo. Previously the gradient was
    // fixed bottom-heavy (0.15 top), so top/center text floated over the
    // weakest scrim zone.
    let mut overlay_html = String::new();
    match treatment.image_overlay.as_str() {
        "none" => {}
        _ => {
            if is_dark {
                overlay_html = match treatment.overlay_anchor.as_str() {
                    "top" => r#"<div style="position:absolute;inset:0;background:linear-gradient(to bottom, rgba(0,0,0,0.55), rgba(0,0,0,0.28) 55%, rgba(0,0,0,0.12));z-index:2;"></div>"#.to_string(),
                    "center" => r#"<div style="position:absolute;inset:0;background:radial-gradient(ellipse 78% 62% at 50% 50%, rgba(0,0,0,0.55), rgba(0,0,0,0.20) 72%, rgba(0,0,0,0.10));z-index:2;"></div>"#.to_string(),
                    _ => r#"<div style="position:absolute;inset:0;background:linear-gradient(to bottom, rgba(0,0,0,0.12), rgba(0,0,0,0.30) 55%, rgba(0,0,0,0.72));z-index:2;"></div>"#.to_string(),
                };
            } else {
                overlay_html = r#"<div style="position:absolute;inset:0;background:linear-gradient(to bottom, rgba(255,255,255,0.10), rgba(255,255,255,0.55));z-index:2;"></div>"#.to_string();
            }
        }
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

/// Returns empty tags — glass container removed per design directive.
fn get_glass_container(_tokens: &DesignTokens, _is_dark: bool) -> (String, String) {
    (String::new(), String::new())
}

/// Card styling tuple: (card_bg, card_border, card_blur) for dark/light contexts.
fn card_styles(tokens: &DesignTokens, is_dark: bool) -> (String, String, String) {
    if is_dark {
        (
            "rgba(255,255,255,0.04)".to_string(),
            "1px solid rgba(255,255,255,0.08)".to_string(),
            String::new(),
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

/// Current slide theme (set by `dispatch_slide`). Used by `layouts::slide_base`
/// so the painted surface matches `get_slide_colors` exactly — otherwise a
/// theme="dark" slide on a light bg_style gets white text on a light surface.
pub fn current_theme() -> String {
    CURRENT_THEME.with(|t| t.borrow().clone())
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

fn image_filter_css(_filter: &str, is_dark: bool) -> &'static str {
    if is_dark {
        "filter: contrast(1.06) brightness(1.06);"
    } else {
        "filter: contrast(1.02) brightness(1.02);"
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
                "gradient" => r#"<div style="position:absolute;inset:0;background:linear-gradient(to bottom, rgba(0,0,0,0.25), rgba(0,0,0,0.72));z-index:1;"></div>"#.to_string(),
                "solid" => r#"<div style="position:absolute;inset:0;background:rgba(0,0,0,0.48);z-index:1;"></div>"#.to_string(),
                "vignette" => r#"<div style="position:absolute;inset:0;background:radial-gradient(circle, transparent 40%, rgba(0,0,0,0.65) 100%);z-index:1;"></div>"#.to_string(),
                _ => r#"<div style="position:absolute;inset:0;background:linear-gradient(to bottom, rgba(0,0,0,0.20), rgba(0,0,0,0.60));z-index:1;"></div>"#.to_string(),
            }
        } else {
            match ov {
                "gradient" => r#"<div style="position:absolute;inset:0;background:linear-gradient(to bottom, rgba(255,255,255,0.15), rgba(255,255,255,0.65));z-index:1;"></div>"#.to_string(),
                "solid" => r#"<div style="position:absolute;inset:0;background:rgba(255,255,255,0.50);z-index:1;"></div>"#.to_string(),
                "vignette" => r#"<div style="position:absolute;inset:0;background:radial-gradient(circle, transparent 50%, rgba(0,0,0,0.25) 100%);z-index:1;"></div>"#.to_string(),
                _ => r#"<div style="position:absolute;inset:0;background:linear-gradient(to bottom, rgba(255,255,255,0.10), rgba(255,255,255,0.50));z-index:1;"></div>"#.to_string(),
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
/// - `chapter`: section-divider visual (kicker, title, accent bar, subtitle).
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
    tagline: &str,
    metric_value: &str,
    metric_label: &str,
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

    // Automatic type-fit (shared overflow model): the display tier (113px) turns
    // a long hero headline into a wall that overflows by ~230px. Cap the headline
    // to 2 wrapped lines in the ~320px hero column via the same model the
    // validator gate uses, so fitted-at-source == accepted-by-gate.
    let display_tier = tokens
        .type_scale
        .get("display")
        .map(|t| t.font_size as f32)
        .unwrap_or(40.0);
    // Split variant left column ≈ 164px (content 420−96 padding, gap 24, ratio
    // 1.2fr:1fr). Centered/chapter variants use the full ~324px column. Compute
    // per-variant so words never get chopped mid-character.
    let split_title_size = crate::overflow_model::fit_font_size_to_lines(
        headline,
        164.0,
        3,
        1.05,
        22.0,
        display_tier,
    )
    .min(crate::overflow_model::fit_font_size_to_words(
        headline, 164.0, 22.0, display_tier, 2.0,
    )) as i32;
    let hero_title_size = crate::overflow_model::fit_font_size_to_lines(
        headline,
        320.0,
        2,
        1.05,
        30.0,
        display_tier,
    ) as i32;

    // Animated right-pointing arrow indicator for carousel progression
    let arrow_html = r#"<div style="position:absolute;right:var(--space-6);bottom:var(--space-6);z-index:10;"><svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" style="color:var(--text-secondary, #888);opacity:0.6;animation:arrow-pulse 2s ease-in-out infinite;"><path d="M5 12h14M12 5l7 7-7 7"/></svg></div><style>@keyframes arrow-pulse { 0%, 100% { transform: translateX(0); opacity: 0.4; } 50% { transform: translateX(6px); opacity: 0.8; } }</style>"#;

    let html = if effective_variant == "split" {
        // split variant: two-column hero — learn from chapter's raw HTML approach
        let kicker_html = if !tagline.is_empty() {
            format!(
                r#"<div style="font-family:{};font-size:11px;font-weight:700;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:12px;">{}</div>"#,
                tokens.body_font, colors.primary, escape_html(tagline)
            )
        } else if !badge.is_empty() {
            format!(
                r#"<div style="font-family:{};font-size:11px;font-weight:700;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:12px;">{}</div>"#,
                tokens.body_font, colors.primary, escape_html(badge)
            )
        } else {
            String::new()
        };            let headline_html = format!(
                r#"<h1 style="font-family:{};font-size:{}px;font-weight:900;color:{};line-height:1.08;margin:0 0 8px;overflow-wrap:break-word;">{}</h1>"#,
                tokens.heading_font, split_title_size, colors.text_primary, escape_html(headline)
            );
        let sub_html = if !subheadline.is_empty() {
            format!(
                r#"<p style="font-family:{};font-size:15px;color:{};line-height:1.5;margin:0;max-width:320px;">{}</p>"#,
                tokens.body_font, colors.text_secondary, escape_html(subheadline)
            )
        } else {
            String::new()
        };
        let left_content = format!("{}{}{}", kicker_html, headline_html, sub_html);
        let right_visual = {
            // Always use a real image for the split variant's right side.
            // When no background_image is provided, a stock Unsplash image is
            // used as a mandatory fallback. The split variant MUST have a real
            // image to avoid an empty right column.
            let img_url = if !background_image.is_empty() {
                background_image.to_string()
            } else {
                // Mandatory stock image fallback for split variant
                "https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=500&fit=crop&auto=format".to_string()
            };
            let safe_bg = img_url.replace('\\', "\\\\").replace('\'', "\\'");
            format!(
                r#"<div style="position:relative;width:100%;height:var(--space-28);border-radius:var(--radius-md);overflow:hidden;box-shadow:var(--shadow-lg);background-image:url('{}');background-size:cover;background-position:center;opacity:{};"></div>"#,
                safe_bg, image_opacity
            )
        };
        // Add arrow indicator to right visual area
        let right_visual_with_arrow = format!("{}{}", right_visual, arrow_html);
        split_layout(
            &left_content,
            &right_visual_with_arrow,
            tokens,
            bg_style,
            "var(--space-3)",
            "1.2fr 1fr",
            true,
        )
    } else if effective_variant == "chapter" {
        // chapter variant: section-divider visual — plain kicker, accent bar, subtitle
        let kicker_html = if !badge.is_empty() {
            format!(
                r#"<div style="font-family:{};font-size:12px;font-weight:800;color:{};letter-spacing:0.08em;text-transform:uppercase;margin-bottom:18px;">{}</div>"#,
                tokens.body_font, colors.primary, escape_html(badge)
            )
        } else {
            String::new()
        };
        let headline_html = format!(
            r#"<h1 style="font-family:{};font-size:{}px;font-weight:900;color:{};line-height:1.05;margin:0;max-width:320px;">{}</h1>"#,
            tokens.heading_font, hero_title_size, colors.text_primary, escape_html(headline)
        );
        let accent_bar = format!(
            r#"<div style="width:76px;height:4px;background:{};border-radius:{};margin:var(--space-3) 0;"></div>"#,
            colors.primary,
            current_component_radius(tokens, "chip")
        );
        let sub_html = if !subheadline.is_empty() {
            format!(
                r#"<p style="font-family:{};font-size:15px;color:{};line-height:1.45;margin:0;max-width:300px;">{}</p>"#,
                tokens.body_font, colors.text_secondary, escape_html(subheadline)
            )
        } else {
            String::new()
        };
        let content = format!(
            r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;justify-content:center;">{}{}{}{}{}</div>"#,
            kicker_html, headline_html, accent_bar, sub_html, arrow_html
        );
        slide_base(&content, tokens, bg_style, true, "16px 52px", "center")
    } else if effective_variant == "centered" {
        // centered variant: editorial magazine-style hero — learn from chapter variant
        // Use tagline as kicker (like chapter's badge), no glass container badge
        let kicker_html = if !tagline.is_empty() {
            format!(
                r#"<div style="font-family:{};font-size:11px;font-weight:700;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:18px;">{}</div>"#,
                tokens.body_font, colors.primary, escape_html(tagline)
            )
        } else if !badge.is_empty() {
            format!(
                r#"<div style="font-family:{};font-size:11px;font-weight:700;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:18px;">{}</div>"#,
                tokens.body_font, colors.primary, escape_html(badge)
            )
        } else {
            String::new()
        };
        let headline_html = format!(
            r#"<h1 style="font-family:{};font-size:{}px;font-weight:900;color:{};line-height:1.05;margin:0;max-width:320px;">{}</h1>"#,
            tokens.heading_font, hero_title_size, colors.text_primary, escape_html(headline)
        );
        let sub_html = if !subheadline.is_empty() {
            format!(
                r#"<p style="font-family:{};font-size:15px;color:{};line-height:1.45;margin:0;max-width:320px;">{}</p>"#,
                tokens.body_font, colors.text_secondary, escape_html(subheadline)
            )
        } else {
            String::new()
        };
        // Chapter-style accent bar
        let accent = format!(
            r#"<div style="width:76px;height:4px;background:{};border-radius:{};margin:var(--space-3) 0;"></div>"#,
            colors.primary,
            current_component_radius(tokens, "chip")
        );
        let content = format!(
            r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;justify-content:center;align-items:center;text-align:center;">{}{}{}{}{}</div>"#,
            kicker_html, headline_html, accent, sub_html, arrow_html
        );
        slide_base(&content, tokens, bg_style, true, "16px 52px", "center")
    } else {
        // left-aligned default (kept for backwards compat)
        let align = "left";
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
        let content = format!("{}{}{}{}{}", gc, badge_html, headline_html, sub_html, gx);
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

    // Automatic type-fit (shared overflow model): size the quote so it fits the
    // glass card's inner height. The display tier (113px) turns a 40-char quote
    // into a ~400px wall; fit_font_size picks the largest size that fits.
    // Geometry: composition 420 − 2×44 slide padding − 2×30 glass padding = 272px
    // column; 405px available − glass pad (2×26) − 64px quote mark − ~90px
    // divider+attribution. Budget and width live in overflow_model.rs (single
    // calibration point shared with the validator gate — see QUOTE_* consts).
    let quote_column_width = crate::overflow_model::QUOTE_COLUMN_WIDTH;
    let quote_max_height = crate::overflow_model::QUOTE_TEXT_BUDGET;
    let quote_line_height = 1.25;
    let max_quote_font = tokens.type_scale.get("display").unwrap().font_size as f32;
    let quote_font_size = format!(
        "{}px",
        crate::overflow_model::fit_font_size(
            quote,
            quote_column_width,
            quote_max_height,
            quote_line_height,
            26.0,
            max_quote_font,
        ) as i32
    );

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
            // Contrast-safe accent: `colors.primary` is derived to hold ≥4.5:1
            // (light) / ≥5.5:1 (dark) against the slide surface, so the mark
            // stays visible on glass + image backdrops. (Was a ~12-19% alpha
            // tint of `tokens.primary` that vanished against the surface.)
            let quote_mark_color = colors.primary.clone();
            let decorative_quote = format!(
                r#"<div style="font-family:Georgia,serif;font-size:48px;line-height:1;color:{};margin-bottom:-4px;user-select:none;" aria-hidden="true">❝</div>"#,
                quote_mark_color
            );
            let q = format!(
                r#"<blockquote style="font-family:{};font-size:{};font-weight:{};line-height:1.2;color:{};margin:0;max-width:100%;border-left:4px solid {};padding-left:24px;text-wrap:balance;font-style:italic;">{}</blockquote>"#,
                tokens.heading_font,
                quote_font_size,
                headline_fw,
                colors.text_primary,
                tokens.primary,
                escape_html(quote)
            );
            let accent_and_attr = if !author.is_empty() {
                let attr = attribution_block(
                    author,
                    role,
                    tokens,
                    Some(&colors.text_primary),
                    "20px 0 0",
                    "left",
                );
                format!(
                    r#"<div style="width:40px;height:2px;background:{};margin:20px 0 12px;border-radius:1px;"></div>{}"#,
                    tokens.primary,
                    attr
                )
            } else {
                String::new()
            };
            let content = format!(
                r#"{}{}{}{}{}"#,
                glass_open, decorative_quote, q, accent_and_attr, glass_close
            );
            slide_base(
                &content,
                tokens,
                bg_style,
                false,
                "16px 44px 20px",
                "center",
            )
        }
        "attribution-below" => {
            // Contrast-safe accent: `colors.primary` is derived to hold ≥4.5:1
            // (light) / ≥5.5:1 (dark) against the slide surface, so the mark
            // stays visible on glass + image backdrops. (Was a ~12-19% alpha
            // tint of `tokens.primary` that vanished against the surface.)
            let quote_mark_color = colors.primary.clone();
            let decorative_quote = format!(
                r#"<div style="font-family:Georgia,serif;font-size:64px;line-height:1;color:{};text-align:center;margin-bottom:-8px;user-select:none;" aria-hidden="true">❝</div>"#,
                quote_mark_color
            );
            let q = format!(
                r#"<blockquote style="font-family:{};font-size:{};font-weight:{};line-height:1.25;color:{};margin:0;text-align:center;text-wrap:balance;font-style:italic;">{}</blockquote>"#,
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
                r#"{}{}{}<div style="width:40px;height:2px;background:{};margin:20px auto 16px;border-radius:1px;"></div><div style="text-align:center;">{}</div>{}"#,
                glass_open, decorative_quote, q, tokens.primary, attr, glass_close
            );
            slide_base(
                &content,
                tokens,
                bg_style,
                false,
                "16px 44px 20px",
                "center",
            )
        }
        _ => {
            // centered (default) — editorial style with decorative quote mark
            // Contrast-safe accent: `colors.primary` is derived to hold ≥4.5:1
            // (light) / ≥5.5:1 (dark) against the slide surface, so the mark
            // stays visible on glass + image backdrops. (Was a ~12-19% alpha
            // tint of `tokens.primary` that vanished against the surface.)
            let quote_mark_color = colors.primary.clone();
            let decorative_quote = format!(
                r#"<div style="font-family:Georgia,serif;font-size:64px;line-height:1;color:{};text-align:center;margin-bottom:-8px;user-select:none;" aria-hidden="true">❝</div>"#,
                quote_mark_color
            );
            let q = format!(
                r#"<blockquote style="font-family:{};font-size:{};font-weight:{};line-height:1.25;color:{};margin:0;text-align:center;text-wrap:balance;font-style:italic;">{}</blockquote>"#,
                tokens.heading_font,
                quote_font_size,
                headline_fw,
                colors.text_primary,
                escape_html(quote)
            );
            let accent_and_attr = if !author.is_empty() {
                let attr = attribution_block(
                    author,
                    role,
                    tokens,
                    Some(&colors.text_primary),
                    "0",
                    "center",
                );
                format!(
                    r#"<div style="width:40px;height:2px;background:{};margin:20px auto 16px;border-radius:1px;"></div>{}"#,
                    tokens.primary,
                    attr
                )
            } else {
                String::new()
            };
            let content = format!(
                r#"{}{}{}<div style="margin-top:12px;text-align:center;">{}</div>{}"#,
                glass_open, decorative_quote, q, accent_and_attr, glass_close
            );
            slide_base(
                &content,
                tokens,
                bg_style,
                false,
                "16px 44px 20px",
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

    // ponytail: dynamic scaling — learn from grid_cards dense variant (lines 2716-2738).
    // Compute character mass to scale padding + font sizes so tiles never overflow.
    // Tiles are capped at 3: the right column (or grid stack) can carry at most
    // three feature cards inside the 449px body; a 4th card overflows the banded
    // layout. Configs with more than 3 features are rejected by the validator
    // (see validate_slide_spec split_features gate) so content is never dropped
    // silently — the renderer here simply never renders a 4th tile.
    let rendered_features: Vec<&Value> = features.iter().take(3).collect();
    let max_feat_title_len = rendered_features
        .iter()
        .map(|f| f.get("title").and_then(|v| v.as_str()).unwrap_or("").len())
        .max()
        .unwrap_or(0);
    let max_feat_desc_len = rendered_features
        .iter()
        .map(|f| f.get("description").and_then(|v| v.as_str()).unwrap_or("").len())
        .max()
        .unwrap_or(0);
    let total_feat_chars: usize = rendered_features.iter().map(|f| {
        let t = f.get("title").and_then(|v| v.as_str()).unwrap_or("").len();
        let d = f.get("description").and_then(|v| v.as_str()).unwrap_or("").len();
        t + d
    }).sum();

    // ponytail: heading margin scales down for dense content
    let heading_margin_bottom = if total_feat_chars > 240 { "6px" } else { "12px" };
    let heading = heading_block(
        title,
        tokens,
        "headline",
        Some(&colors.text_primary),
        false,
        None,
        "left",
        &format!("0 0 {}", heading_margin_bottom),
        true,
    );

    // Scale tiers matching grid_cards dense thresholds
    let (card_padding, title_size, desc_size, card_gap) = if image_feature_layout {
        if total_feat_chars > 240 || max_feat_desc_len > 60 {
            ("6px", body_fs.saturating_sub(4), caption_fs.saturating_sub(2), "0px")
        } else if total_feat_chars > 160 || max_feat_desc_len > 40 {
            ("8px", body_fs.saturating_sub(3), caption_fs.saturating_sub(1), "0px")
        } else {
            ("10px", body_fs.saturating_sub(2), caption_fs.saturating_sub(1), "0px")
        }
    } else {
        if total_feat_chars > 240 || max_feat_desc_len > 60 {
            ("var(--space-1)", body_fs.saturating_sub(1), caption_fs.saturating_sub(1), "4px")
        } else {
            ("var(--space-1)", body_fs, caption_fs, "var(--space-1)")
        }
    };
    let card_margin = if image_feature_layout { "0" } else { "0 0 12px" };

    let mut feature_cards = Vec::new();
    for feat in rendered_features.iter() {
        let t = feat.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let d = feat
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        feature_cards.push(format!(
            r#"<div style="background:{};border:1px solid {};border-radius:{};box-shadow:{};padding:{};display:flex;gap:{};align-items:flex-start;margin:{};box-sizing:border-box;min-width:0;">
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
        // ponytail: dynamic outer gap — scale column gap for dense content
        let outer_gap = if total_feat_chars > 240 { "12px" } else { "20px" };
        let heading_gap = if total_feat_chars > 240 { "8px" } else { "14px" };
        let card_stack_gap = if total_feat_chars > 240 { "6px" } else { "10px" };
        format!(
            r#"<div style="display:grid;grid-template-columns:1fr 1fr;gap:{};width:100%;height:100%;align-items:stretch;overflow:hidden;">
                <div style="min-width:0;min-height:0;overflow:hidden;display:flex;align-items:stretch;">{}</div>
                <div style="min-width:0;min-height:0;overflow:hidden;display:flex;flex-direction:column;justify-content:center;gap:{};">
                    {}
                    <div style="display:flex;flex-direction:column;gap:{};overflow:hidden;">{}</div>
                </div>
            </div>"#,
            outer_gap, left_visual, heading_gap, heading, card_stack_gap, features_html
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
            "16px 36px 20px"
        } else {
            "16px var(--space-6) 20px"
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
// 14. text_block_slide
// ─────────────────────────────────────────────────────────────────────────────

/// Editorial article-style text block with eyebrow, weight-900 title,
/// accent line, drop-cap first paragraph, and optional subtitle.
/// `body` is a single string; newlines produce separate paragraphs.
pub fn text_block_slide(
    tokens: &DesignTokens,
    title: &str,
    body: &str,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
    subtitle: &str,
    text_align: &str,
    max_width_val: &str,
    variant: &str,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let body_fs = 14i32;

    // Variant-driven alignment and width
    let effective_variant = if variant.is_empty() { "left" } else { variant };
    let align = if text_align.is_empty() {
        match effective_variant {
            "centered" => "center",
            _ => "left",
        }
    } else {
        text_align
    };
    let mw = if max_width_val.is_empty() {
        match effective_variant {
            "narrow" => "280px",
            "wide" => "380px",
            "centered" => "340px",
            _ => "340px",
        }
    } else {
        max_width_val
    };

    // Eyebrow: small uppercase label derived from title first-word or subtitle
    let eyebrow_text = if !subtitle.is_empty() {
        subtitle
    } else if !title.is_empty() {
        // Use first 3 words as eyebrow hint
        let words: Vec<&str> = title.split_whitespace().take(3).collect();
        &words.join(" ")
    } else {
        ""
    };
    let eyebrow_html = if !eyebrow_text.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:9px;font-weight:700;color:{};text-transform:uppercase;letter-spacing:0.12em;margin-bottom:10px;">{}</div>"#,
            tokens.body_font,
            colors.primary,
            escape_html(eyebrow_text)
        )
    } else {
        String::new()
    };

    // Title: weight-900, 28px — the dominant visual element
    let title_html = if !title.is_empty() {
        format!(
            r#"<h2 style="font-family:{};font-size:28px;font-weight:900;color:{};margin:0 0 6px;line-height:1.15;letter-spacing:-0.02em;">{}</h2>"#,
            tokens.heading_font,
            colors.text_primary,
            escape_html(title)
        )
    } else {
        String::new()
    };

    // Accent line between title and body
    let accent_html = format!(
        r#"<div style="width:40px;height:2px;background:{};margin:12px 0 16px;opacity:0.35;"></div>"#,
        colors.text_primary
    );

    // Split body on newlines into paragraphs; first non-empty gets drop-cap
    let mut body_html = String::new();
    let mut first_done = false;
    for para in body.split('\n') {
        let p = para.trim();
        if !p.is_empty() {
            if !first_done {
                // Drop-cap first paragraph: first letter oversized
                let chars: Vec<char> = p.chars().collect();
                if chars.len() > 1 {
                    let first_char = chars[0];
                    let rest: String = chars[1..].iter().collect();
                    body_html.push_str(&format!(
                        r#"<p style="font-family:{};font-size:{}px;color:{};margin:0 0 14px;line-height:1.6;text-align:{};"><span style="font-size:36px;font-weight:900;color:{};float:left;line-height:0.85;margin:2px 8px 0 0;font-family:{};">{}</span>{}</p>"#,
                        tokens.body_font, body_fs, colors.text_secondary, align,
                        colors.primary, tokens.heading_font,
                        escape_html(&first_char.to_string()),
                        escape_html(&rest)
                    ));
                } else {
                    body_html.push_str(&format!(
                        r#"<p style="font-family:{};font-size:{}px;color:{};margin:0 0 14px;line-height:1.6;text-align:{};">{}</p>"#,
                        tokens.body_font, body_fs, colors.text_secondary, align,
                        escape_html(p)
                    ));
                }
                first_done = true;
            } else {
                body_html.push_str(&format!(
                    r#"<p style="font-family:{};font-size:{}px;color:{};margin:0 0 14px;line-height:1.6;text-align:{};">{}</p>"#,
                    tokens.body_font, body_fs, colors.text_secondary, align,
                    escape_html(p)
                ));
            }
        }
    }

    let content = format!(
        r#"<div style="max-width:{};margin:0 auto;text-align:{};width:100%;box-sizing:border-box;display:flex;flex-direction:column;justify-content:center;height:100%;overflow:hidden;padding-bottom:var(--space-2);">
            {}
            {}
            {}
            <div>{}</div>
        </div>"#,
        mw, align,
        eyebrow_html,
        title_html,
        accent_html,
        body_html
    );

    let padding_val = match effective_variant {
        "centered" => "16px 48px 20px",
        "narrow" => "16px 56px 20px",
        "wide" => "16px 36px 20px",
        _ => "16px 48px 20px",
    };
    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        padding_val,
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
        "16px var(--space-6) 20px",
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
    } else if chart_type == "bar_vertical" || chart_type == "column" {
        // Vertical column chart — single-series or grouped via `series` field.
        // Replaces the retired `column_chart_slide`. Detects `series: [{name, value}]`
        // entries for grouped/multi-series layout; falls back to flat single-series.
        let series_colors = [
            "#767CFF", "#FF8C6B", "#3ECFA0", "#FFB84D", "#E879A8", "#5BB5F0",
        ];

        let is_grouped = data.iter().any(|item| {
            item.get("series")
                .and_then(|v| v.as_array())
                .map(|arr| !arr.is_empty())
                .unwrap_or(false)
        });

        if is_grouped {
            // Multi-series grouped columns (replaces column_chart grouped path).
            let global_max: f64 = data
                .iter()
                .filter_map(|item| item.get("series")?.as_array())
                .flatten()
                .filter_map(|s| s.get("value")?.as_f64())
                .fold(0.0f64, f64::max)
                .max(1.0);

            let num_series = data
                .first()
                .and_then(|item| item.get("series")?.as_array())
                .map(|arr| arr.len())
                .unwrap_or(1);
            let bar_inner_pct = (70.0 / num_series as f64).max(20.0);
            let gap_px = if num_series > 2 { 0 } else { 2 };

            let categories: String = data
                .iter()
                .enumerate()
                .map(|(ci, item)| {
                    let lbl = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                    let series = item
                        .get("series")
                        .and_then(|v| v.as_array())
                        .cloned()
                        .unwrap_or_default();

                    let inner_bars: String = series
                        .iter()
                        .enumerate()
                        .map(|(si, sv)| {
                            let val = sv.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let pct = (val / global_max) * 100.0;
                            let col = series_colors[si % series_colors.len()];
                            let val_display = if val >= 1000.0 {
                                format!("{:.0}", val)
                            } else if val == val.floor() {
                                format!("{:.0}", val)
                            } else {
                                format!("{:.1}", val)
                            };
                            format!(
                                r#"<div style="display:flex;flex-direction:column;align-items:center;flex:1;min-width:0;gap:2px;">
                                    <div style="font-family:{};font-size:8px;font-weight:800;color:{};line-height:1;text-align:center;">{}</div>
                                    <div style="width:100%;height:104px;display:flex;align-items:flex-end;justify-content:center;">
                                        <div style="width:{:.0}%;height:{:.1}%;min-height:4px;background:{};border-radius:3px 3px 0 0;"></div>
                                    </div>
                                </div>"#,
                                tokens.body_font,
                                colors.text_primary,
                                val_display,
                                bar_inner_pct,
                                pct,
                                col
                            )
                        })
                        .collect();

                    let separator_html = if num_series > 1 && ci < data.len() - 1 {
                        r#"<div style="position:absolute;right:-8px;transform:translateX(50%);top:0;bottom:18px;width:1px;background:rgba(128,128,128,0.22);"></div>"#
                    } else {
                        ""
                    };
                    format!(
                        r#"<div style="display:flex;flex-direction:column;align-items:center;flex:1;min-width:0;position:relative;">
                            <div style="display:flex;align-items:flex-end;justify-content:center;width:100%;height:104px;gap:{}px;">{}</div>
                            <span style="font-family:{};font-size:10px;color:{};margin-top:6px;text-align:center;max-width:100%;">{}</span>
                            {}
                        </div>"#,
                        gap_px,
                        inner_bars,
                        tokens.body_font,
                        colors.text_secondary,
                        escape_html(lbl),
                        separator_html
                    )
                })
                .collect();

            let legend_items: String = data
                .first()
                .and_then(|item| item.get("series")?.as_array())
                .cloned()
                .unwrap_or_default()
                .iter()
                .enumerate()
                .map(|(si, sv)| {
                    let name = sv.get("name").and_then(|v| v.as_str()).unwrap_or("");
                    let col = series_colors[si % series_colors.len()];
                    format!(
                        r#"<div style="display:flex;align-items:center;gap:4px;">
                            <div style="width:8px;height:8px;border-radius:2px;background:{};flex-shrink:0;"></div>
                            <span style="font-family:{};font-size:8px;color:{};">{}</span>
                        </div>"#,
                        col, tokens.body_font, colors.text_secondary, escape_html(name)
                    )
                })
                .collect();

            let legend_html = if !legend_items.is_empty() {
                format!(
                    r#"<div style="display:flex;justify-content:center;gap:12px;margin-top:6px;">{}</div>"#,
                    legend_items
                )
            } else {
                String::new()
            };

            chart_html = format!(
                r#"<div style="display:flex;gap:var(--space-3);width:100%;height:142px;margin-top:16px;overflow:hidden;">{}</div>{}"#,
                categories, legend_html
            );
        } else {
            // Single-series flat columns.
            let vals: Vec<f64> = data
                .iter()
                .map(|item| {
                    item.get("value")
                        .and_then(|v| {
                            v.as_f64()
                                .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
                        })
                        .unwrap_or(0.0)
                })
                .collect();
            let max_val = vals.iter().copied().fold(0.0, f64::max).max(1.0);

            let bars: String = data
                .iter()
                .zip(vals.iter())
                .map(|(item, val)| {
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
                            <span style="font-family:{};font-size:10px;color:{};margin-top:6px;text-align:center;max-width:100%;">{}</span>
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
                })
                .collect();

            chart_html = format!(
                r#"<div style="display:flex;gap:var(--space-3);width:100%;height:142px;margin-top:16px;overflow:hidden;">{}</div>"#,
                bars
            );
        }
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
        chart_html = render_svg_scatter_plot(&data, 320, 130, &colors, "", "");
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
        "16px var(--space-6) 20px",
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
    x_label: &str,
    y_label: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let title_html = heading_block(
        title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 12px", true,
    );
    let svg = render_svg_scatter_plot(&data, 320, 185, &colors, x_label, y_label);
    let radius = current_component_radius(tokens, "card");
    let chart_bg = if colors.is_dark {
        "rgba(255,255,255,0.05)"
    } else {
        "rgba(255,255,255,0.92)"
    };
    let chart_border = format!("1px solid {}", colors.border);
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:12px;">
            {}
            <div style="width:100%;height:195px;border-radius:{};overflow:hidden;background:{};border:{};padding:8px 10px;box-sizing:border-box;display:flex;align-items:center;justify-content:center;">{}</div>
            <p style="font-family:{};font-size:10.5px;color:{};margin:0;line-height:1.4;opacity:0.85;">Scatter distribution illustrating the linear relationship between character mass and compile latency under concurrency tests.</p>
        </div>"#,
        title_html, radius, chart_bg, chart_border, svg, tokens.body_font, colors.text_secondary
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
        title, tokens, "headline", Some(&colors.text_primary), false, None, "center", "0 0 12px", true,
    );
    let svg = render_svg_gauge_chart(value, 100.0, "%", &colors);
    let radius = current_component_radius(tokens, "card");
    let card_bg = if colors.is_dark { "rgba(255,255,255,0.05)" } else { "rgba(255,255,255,0.92)" };
    let subtext = if !label.is_empty() { label.to_string() } else { "Optimal Range".to_string() };

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;align-items:center;gap:12px;">
            {}
            <div style="width:100%;background:{};border:1px solid {};border-radius:{};padding:20px 16px 16px;box-sizing:border-box;display:flex;flex-direction:column;align-items:center;gap:10px;">
                <div style="width:100%;max-width:240px;height:120px;margin:0 auto;display:flex;justify-content:center;">{}</div>
                <div style="font-family:{};font-size:10px;font-weight:900;color:#10B981;background:#10B98118;padding:3px 10px;border-radius:999px;letter-spacing:0.06em;">✓ STATUS: {}</div>
            </div>
            <p style="font-family:{};font-size:10.5px;color:{};margin:4px 0 0;text-align:center;line-height:1.4;opacity:0.85;">Overall system health and efficiency score calculated across 100+ stress-test assertions.</p>
        </div>"#,
        title_html,
        card_bg,
        colors.border,
        radius,
        svg,
        tokens.heading_font,
        escape_html(&subtext.to_uppercase()),
        tokens.body_font,
        colors.text_secondary
    );
    let html = slide_base(&content, tokens, bg_style, false, "16px 44px", "center");
    let html = inject_background_image(html, bg_img, img_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "default", "theme": theme})
}

fn radar_chart_slide(
    tokens: &DesignTokens,
    data: Vec<Value>,
    title: &str,
    description: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let title_html = heading_block(
        title, tokens, "title", None, true, None, "left", "0 0 10px", false,
    );
    let svg = render_svg_radar_chart(&data, 320, 210, &colors);
    let desc_html = if !description.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:11px;color:{};margin:8px 0 0;line-height:1.4;text-align:center;max-width:320px;opacity:0.85;">{}</p>"#,
            tokens.body_font, colors.text_secondary, escape_html(description)
        )
    } else {
        String::new()
    };
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;align-items:center;">{}<div style="width:100%;max-width:320px;height:210px;margin:4px auto 0;">{}</div>{}</div>"#,
        title_html, svg, desc_html
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
                         <span style="font-family:{4};font-size:16px;font-weight:700;color:{5};">{9:.0}%</span>
                    </div>
                </div>
                <span style="font-family:{6};font-size:11px;font-weight:600;color:{7};margin-top:12px;text-align:center;text-transform:uppercase;letter-spacing:0.04em;">{8}</span>
            </div>"#,
            ring_color, deg, track_color, inner_bg, tokens.heading_font, colors.text_primary, tokens.body_font, colors.text_secondary, escape_html(lbl), val
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

    let (l_lbl, l_val, r_lbl, r_val) = if let (Some(ea), Some(va)) = (comparison.get("entity_a"), comparison.get("value_a")) {
        let eb = comparison.get("entity_b").and_then(|v| v.as_str()).unwrap_or("Entity B").to_string();
        let vb = comparison.get("value_b").and_then(|v| v.as_f64()).unwrap_or(0.0);
        (ea.as_str().unwrap_or("Entity A").to_string(), va.as_f64().unwrap_or(0.0), eb, vb)
    } else {
        let left = comparison.get("left").cloned().unwrap_or(json!({}));
        let right = comparison.get("right").cloned().unwrap_or(json!({}));
        let ll = left.get("label").and_then(|v| v.as_str()).unwrap_or("Entity A").to_string();
        let lv = left.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let rl = right.get("label").and_then(|v| v.as_str()).unwrap_or("Entity B").to_string();
        let rv = right.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
        (ll, lv, rl, rv)
    };

    let total = (l_val + r_val).max(1.0);
    let l_pct = (l_val / total) * 100.0;
    let r_pct = (r_val / total) * 100.0;

    let l_color = &colors.primary;
    let r_color = &colors.text_secondary;

    let default_unit = comparison.get("metric").and_then(|v| v.as_str()).unwrap_or("");
    let l_unit = comparison.get("left").and_then(|l| l.get("unit")).and_then(|v| v.as_str()).unwrap_or(default_unit);
    let r_unit = comparison.get("right").and_then(|r| r.get("unit")).and_then(|v| v.as_str()).unwrap_or(default_unit);
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

    let l_val_str = format!("{:.0}{}{}", l_val, l_space, escape_html(l_unit));
    let r_val_str = format!("{:.0}{}{}", r_val, r_space, escape_html(r_unit));

    let lbl_fs = if l_lbl.len().max(r_lbl.len()) > 20 { 9 } else if l_lbl.len().max(r_lbl.len()) > 12 { 10 } else { 11 };
    let max_val_len = l_val_str.len().max(r_val_str.len());
    let val_fs = if max_val_len > 12 { 15 } else if max_val_len > 8 { 18 } else if max_val_len > 5 { 20 } else { 24 };

    let metric_name = comparison.get("metric").and_then(|v| v.as_str()).unwrap_or("");
    let metric_tag = if !metric_name.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:11px;font-weight:800;color:{};text-transform:uppercase;letter-spacing:0.08em;margin-bottom:6px;text-align:center;width:100%;">{}</div>"#,
            tokens.body_font, colors.primary, escape_html(metric_name)
        )
    } else {
        String::new()
    };

    let bar_html = format!(
        r#"<div style="width:100%;margin-top:20px;">
            {}
            <div style="display:flex;justify-content:space-between;align-items:flex-end;margin-bottom:8px;font-family:{};font-size:{}px;color:{};font-weight:700;line-height:1.2;">
                <span style="text-align:left;max-width:48%;overflow-wrap:break-word;">{}</span>
                <span style="text-align:right;max-width:48%;overflow-wrap:break-word;">{}</span>
            </div>
            <div style="width:100%;height:16px;background:{}30;border-radius:8px;overflow:hidden;display:flex;">
                <div style="width:{:.1}%;height:100%;background:{};border-radius:8px 0 0 8px;"></div>
                <div style="width:{:.1}%;height:100%;background:{};border-radius:0 8px 8px 0;"></div>
            </div>
            <div style="display:flex;justify-content:space-between;align-items:center;margin-top:10px;font-family:{};font-size:{}px;font-weight:800;letter-spacing:-0.01em;line-height:1.2;">
                <span style="color:{};text-align:left;white-space:nowrap;flex-shrink:0;">{}</span>
                <span style="color:{};text-align:right;white-space:nowrap;flex-shrink:0;">{}</span>
            </div>
        </div>"#,
        metric_tag,
        tokens.body_font,
        lbl_fs,
        colors.text_primary,
        escape_html(&l_lbl),
        escape_html(&r_lbl),
        colors.border,
        l_pct,
        l_color,
        r_pct,
        r_color,
        tokens.heading_font,
        val_fs,
        l_color,
        l_val_str,
        r_color,
        r_val_str
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
        title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 12px", true,
    );

    let radius = current_component_radius(tokens, "card");
    let card_bg = if is_dark {
        "rgba(255,255,255,0.05)"
    } else {
        "rgba(255,255,255,0.92)"
    };
    let card_border = format!("1px solid {}", colors.border);

    let grid_html: String = metrics.iter().take(4).map(|item| {
        let val = item.get("value").and_then(|v| v.as_str()).unwrap_or("");
        let lbl = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
        let trend = item.get("trend").and_then(|v| v.as_str()).unwrap_or("");

        // Progress-bar fill is driven ONLY by explicit per-tile config that
        // mirrors what the metric demonstrates — never inferred abstractly:
        //   1. `progress` — fraction 0..1 or percent 0..100 (number or "72%")
        //   2. `current` + `total` — current/total as a fraction
        // A tile with no explicit progress config renders NO bar. There is no
        // numeric-`value` fallback (e.g. value "4.2x" must not become 4.2%)
        // and no silent 100% default — an abstract fill is worse than none.
        let bar_fill: Option<f64> = item
            .get("progress")
            .and_then(|v| v.as_f64())
            .or_else(|| {
                item.get("progress").and_then(|v| v.as_str()).and_then(|s| {
                    s.trim().trim_end_matches('%').parse::<f64>().ok()
                })
            })
            .map(|p| if p <= 1.0 { p * 100.0 } else { p })
            .or_else(|| {
                let c = item
                    .get("current")
                    .and_then(|v| v.as_f64())
                    .or_else(|| item.get("current").and_then(|v| v.as_str()).and_then(|s| s.trim().parse::<f64>().ok()));
                let t = item
                    .get("total")
                    .and_then(|v| v.as_f64())
                    .or_else(|| item.get("total").and_then(|v| v.as_str()).and_then(|s| s.trim().parse::<f64>().ok()));
                match (c, t) {
                    (Some(c), Some(t)) if t > 0.0 => Some((c / t * 100.0).clamp(0.0, 100.0)),
                    _ => None,
                }
            });
        let bar_pct = bar_fill.map(|p| p.clamp(0.0, 100.0));
        let bar_html = match bar_pct {
            Some(pct) => format!(
                r#"<div style="width:100%;height:3px;background:{}20;border-radius:999px;margin-top:2px;overflow:hidden;">
                    <div style="width:{:.0}%;height:100%;background:{};border-radius:999px;"></div>
                </div>"#,
                colors.primary, pct, colors.primary
            ),
            None => String::new(),
        };

        let trend_color = if trend.contains('+') || trend.to_lowercase().contains("up") { "#10B981" } else { "#EF4444" };
        let trend_badge = if !trend.is_empty() {
            format!(r#"<span style="font-size:10px;font-weight:900;color:{};background:{}18;padding:2px 6px;border-radius:4px;margin-left:auto;">{}</span>"#, trend_color, trend_color, escape_html(trend))
        } else {
            String::new()
        };

        format!(
            r#"<div style="background:{};border:{};border-radius:{};padding:16px 14px;box-sizing:border-box;display:flex;flex-direction:column;gap:6px;position:relative;">
                <div style="display:flex;align-items:center;justify-content:space-between;width:100%;">
                    <span style="font-family:{};font-size:10px;font-weight:800;color:{};text-transform:uppercase;letter-spacing:0.06em;">{}</span>
                    {}
                </div>
                <div style="font-family:{};font-size:30px;font-weight:900;color:{};line-height:1;">{}</div>
                {}
            </div>"#,
            card_bg, card_border, radius,
            tokens.body_font, colors.text_secondary, escape_html(lbl),
            trend_badge,
            tokens.heading_font, colors.primary, escape_html(val),
            bar_html
        )
    }).collect();

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:12px;">
            {}
            <div style="display:grid;grid-template-columns:1fr 1fr;gap:12px;width:100%;">{}</div>
            <p style="font-family:{};font-size:10.5px;color:{};margin:4px 0 0;line-height:1.4;opacity:0.85;">Real-time performance telemetry metrics sampled across production worker threads.</p>
        </div>"#,
        heading, grid_html, tokens.body_font, colors.text_secondary
    );
    let html = hero_layout(&content, tokens, bg_style, false, "left");
    let html = inject_background_image(html, bg_img, img_opacity, is_dark);
    json!({"html": html, "background": bg_style, "variant": "default", "theme": theme})
}

fn funnel_chart_slide(
    tokens: &DesignTokens,
    steps: Vec<Value>,
    title: &str,
    description: &str,
    bg_style: &str,
    theme: &str,
    bg_img: &str,
    img_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let heading = heading_block(title, tokens, "title", None, true, None, "left", "0", false);

    let mut vals = Vec::new();
    let max_items = steps.len().min(5);
    for item in steps.iter().take(max_items) {
        let v_num = item.get("value").and_then(|v| {
            v.as_f64().or_else(|| {
                v.as_str().and_then(|s| {
                    s.replace(",", "").replace("K", "000").replace("M", "000000").replace("$", "").parse::<f64>().ok()
                })
            })
        }).unwrap_or(0.0);
        vals.push(v_num);
    }
    let top_val = vals.first().copied().unwrap_or(1.0).max(1.0);

    let num_steps = steps.len().min(5);
    // Density scaling (shared overflow model): 5 rows × 34px bars + arrows + title
    // exceeded the 405px safe height by ~22px. Shrink bar height for dense funnels.
    let bar_height = match num_steps {
        5 => 26,
        4 => 30,
        _ => 34,
    };
    let funnel_html: String = steps.iter().take(5).enumerate().map(|(i, item)| {
        let lbl = item.get("label").or_else(|| item.get("title")).and_then(|v| v.as_str()).unwrap_or("");
        let current_val = vals.get(i).copied().unwrap_or(0.0);
        let val_display = if let Some(s) = item.get("value").and_then(|v| v.as_str()) {
            s.to_string()
        } else if current_val >= 1000.0 {
            format!("{:.0}", current_val)
        } else {
            format!("{:.0}", current_val)
        };
        let width_pct = (15.0 + (current_val / top_val) * 85.0).clamp(15.0, 100.0) as u32;
        let opacity_pct = 1.0 - (i as f64 * 0.15);

            let arrow = if i < num_steps - 1 {
                format!(r#"<div style="text-align:center;font-size:10px;color:{};margin:1px 0;font-weight:bold;">↓</div>"#, colors.text_secondary)
        } else {
            String::new()
        };

        if width_pct < 40 {
            format!(
                r#"<div style="position:relative;width:100%;margin:0 auto;display:flex;align-items:center;gap:var(--space-1);">
                    <span style="font-family:{};font-size:10px;font-weight:700;color:{};text-transform:uppercase;letter-spacing:0.04em;flex:1;text-align:right;">{}</span>
                    <div style="width:{}%;background:{};opacity:{:.2};border-radius:6px;height:{}px;min-width:12px;flex-shrink:0;"></div>
                    <strong style="font-family:{};font-size:12px;color:{};flex:1;">{}</strong>
                </div>{}"#,
                tokens.body_font, colors.text_primary, escape_html(lbl),
                width_pct, colors.primary, opacity_pct, bar_height,
                tokens.body_font, colors.text_primary, escape_html(&val_display),
                arrow
            )
        } else {
            format!(
                r#"<div style="width:{}%;background:{};opacity:{:.2};border-radius:6px;padding:6px 14px;box-sizing:border-box;margin:0 auto;display:flex;justify-content:space-between;align-items:center;">
                    <span style="font-family:{};font-size:10px;font-weight:700;color:{};text-transform:uppercase;letter-spacing:0.04em;">{}</span>
                    <strong style="font-family:{};font-size:12px;color:{};">{}</strong>
                </div>{}"#,
                width_pct, colors.primary, opacity_pct,
                tokens.body_font, colors.button_text, escape_html(lbl),
                tokens.body_font, colors.button_text, escape_html(&val_display),
                arrow
            )
        }
    }).collect();

    let desc_html = if !description.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:11px;color:{};margin:12px 0 0;line-height:1.4;text-align:center;width:100%;opacity:0.85;">{}</p>"#,
            tokens.body_font, colors.text_secondary, escape_html(description)
        )
    } else {
        String::new()
    };

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;justify-content:center;">
            {}
            <div style="width:100%;margin-top:14px;box-sizing:border-box;">{}</div>
            {}
        </div>"#,
        heading, funnel_html, desc_html
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
    let heading = heading_block(title, tokens, "headline", None, true, None, "left", "0 0 12px", false);
    let radius = current_component_radius(tokens, "card");
    let card_bg = if is_dark { "rgba(255,255,255,0.05)" } else { "rgba(255,255,255,0.92)" };

    let header_cells: Vec<String> = headers.iter().map(|h| {
        let text = h.as_str().unwrap_or("");
        format!("<th style=\"padding:10px 14px;text-align:left;font-family:{};font-size:10.5px;font-weight:900;color:{};background:{};border-bottom:1px solid {};text-transform:uppercase;letter-spacing:0.06em;\">{}</th>", tokens.heading_font, colors.text_primary, colors.primary.clone() + "18", colors.border, escape_html(text))
    }).collect();

    let body_rows: String = rows.iter().enumerate().map(|(idx, row)| {
        let cells: Vec<String> = row.as_array().map(|arr| {
            arr.iter().enumerate().map(|(c_idx, cell)| {
                let text = cell.as_str().unwrap_or("");
                // Consistent cell styling: first column is the row label
                // (semibold), every other column is secondary text. NO
                // auto-badge heuristic — a naive contains('x')||contains('%')
                // check made cells like "60%" and "Global equity index" (the
                // 'x' in "index") green pills while neighbors stayed plain,
                // producing an inconsistent tag-like vs plain mix.
                let cell_html = if c_idx == 0 {
                    format!(r#"<span style="font-family:{};font-weight:800;color:{};">{}</span>"#, tokens.heading_font, colors.text_primary, escape_html(text))
                } else {
                    format!(r#"<span style="font-family:{};color:{};">{}</span>"#, tokens.body_font, colors.text_secondary, escape_html(text))
                };
                let bg = if idx % 2 == 0 { "transparent" } else { "rgba(255,255,255,0.02)" };
                format!("<td style=\"padding:9px 14px;font-size:11px;background:{};border-bottom:1px solid {}18;\">{}</td>", bg, colors.border, cell_html)
            }).collect()
        }).unwrap_or_default();
        format!("<tr>{}</tr>", cells.join(""))
    }).collect();

    let caption = if !_caption.is_empty() { _caption.to_string() } else { "Benchmark metrics evaluated under 1,000 carousel thread iterations.".to_string() };

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:12px;">
            {}
            <div style="width:100%;background:{};border:1px solid {};border-radius:{};overflow:hidden;box-sizing:border-box;">
                <table style="width:100%;border-collapse:collapse;">
                    <thead><tr>{}</tr></thead>
                    <tbody>{}</tbody>
                </table>
            </div>
            <p style="font-family:{};font-size:10.5px;color:{};margin:0;line-height:1.4;opacity:0.85;">{}</p>
        </div>"#,
        heading, card_bg, colors.border, radius, header_cells.join(""), body_rows, tokens.body_font, colors.text_secondary, escape_html(&caption)
    );
    let html = hero_layout(&content, tokens, bg_style, false, "left");
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

    let card_bg = if is_dark {
        "rgba(255,255,255,0.05)"
    } else {
        "rgba(255,255,255,0.92)"
    };
    let border_color = format!("1px solid {}", colors.border);
    let radius = current_component_radius(tokens, "card");

    let trend_color = if trend.contains('↓') || trend.contains('-') {
        "#EF4444"
    } else {
        "#10B981"
    };
    let trend_html = if !trend.is_empty() {
        format!(
            r#"<span style="font-family:{};font-size:11px;font-weight:800;color:{};background:{}18;padding:3px 10px;border-radius:999px;display:inline-block;margin-top:6px;">{}</span>"#,
            tokens.body_font,
            trend_color,
            trend_color,
            escape_html(trend)
        )
    } else {
        String::new()
    };

    let spark_points: Vec<String> = spark_values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let val = v.as_f64().unwrap_or(0.0);
            let x = 10.0 + (i as f64 / (spark_values.len() as f64 - 1.0).max(1.0)) * 260.0;
            let y = 35.0 - (val / 100.0) * 28.0;
            format!("{:.1},{:.1}", x, y)
        })
        .collect();
    let spark_html = if spark_points.len() > 1 {
        format!(
            r#"<svg width="280" height="42" viewBox="0 0 280 42"><polyline points="{}" fill="none" stroke="{}" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/></svg>"#,
            spark_points.join(" "),
            colors.primary
        )
    } else {
        String::new()
    };

    let card_html = format!(
        r#"<div style="background:{};border:{};border-radius:{};padding:22px 20px 18px;text-align:center;width:100%;box-sizing:border-box;display:flex;flex-direction:column;align-items:center;">
            <span style="font-family:{};font-size:9.5px;font-weight:900;letter-spacing:0.12em;text-transform:uppercase;color:{};background:{}18;padding:2px 8px;border-radius:4px;margin-bottom:8px;">TELEMETRY MONITOR</span>
            <span style="font-family:{};font-size:52px;font-weight:900;color:{};margin:0;line-height:1;">{}</span>
            <h3 style="font-family:{};font-size:13.5px;font-weight:800;color:{};margin:6px 0 2px;line-height:1.2;">{}</h3>
            {}
            <div style="margin:10px auto 4px;display:flex;justify-content:center;">{}</div>
        </div>"#,
        card_bg,
        border_color,
        radius,
        tokens.heading_font,
        colors.primary,
        colors.primary,
        tokens.heading_font,
        colors.text_primary,
        escape_html(value),
        tokens.heading_font,
        colors.text_secondary,
        escape_html(label),
        trend_html,
        spark_html
    );

    let ctx_text = if !_context.is_empty() {
        _context.to_string()
    } else {
        "Continuous 90-day production telemetry monitor tracking engine uptime.".to_string()
    };

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:12px;align-items:center;">
            {}
            <p style="font-family:{};font-size:10.5px;color:{};margin:0;line-height:1.4;text-align:center;opacity:0.85;">{}</p>
        </div>"#,
        card_html, tokens.body_font, colors.text_secondary, escape_html(&ctx_text)
    );
    let html = hero_layout(&content, tokens, bg_style, false, "center");
    let html = inject_background_image(html, bg_img, img_opacity, is_dark);
    json!({"html": html, "background": bg_style, "variant": "default", "theme": theme})
}

// column_chart_slide removed: vertical bar rendering now lives inside chart_slide
// via chart_type="bar_vertical". The legacy dispatch slot still routes "column_chart"
// JSON through chart_slide for backwards compatibility.


pub fn problem_solution_slide(
    tokens: &DesignTokens,
    title: &str,
    problem: &str,
    solution: &str,
    proof_points: Vec<Value>,
    description: &str,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let radius = current_component_radius(tokens, "card");
    // Card text colors: on light slides, cards use a light card_bg, so their
    // text must be dark (using token text_primary/secondary). On dark slides,
    // cards use dark card_bg, so text must be light (using text_on_dark).
    let card_label_color = if colors.is_dark {
        colors.text_primary.clone()
    } else {
        tokens.text_primary.clone()
    };
    let card_body_color = if colors.is_dark {
        colors.text_secondary.clone()
    } else {
        tokens.text_secondary.clone()
    };
    let card_bg = if colors.is_dark {
        "rgba(255,255,255,0.10)"
    } else {
        "rgba(255,255,255,0.92)"
    };
    let card_title_color = if colors.is_dark {
        colors.text_primary.clone()
    } else {
        tokens.text_primary.clone()
    };

    let proof_grid_html = if proof_points.is_empty() {
        String::new()
    } else {
        let items_text: Vec<String> = proof_points
            .iter()
            .take(4)
            .map(|item| {
                // ponytail: plain-string items (no object keys) render as-is
                if item.is_string() {
                    return escape_html(&item.as_str().unwrap_or(""));
                }
                let t = simple_text(item, &["title", "label"]);
                let d = simple_text(item, &["description", "body"]);
                if !d.is_empty() {
                    format!("<strong>{}</strong>: {}", escape_html(&t), escape_html(&d))
                } else {
                    escape_html(&t)
                }
            })
            .collect();

        let combined_desc = items_text.join(" &nbsp;•&nbsp; ");

        format!(
            r#"<div style="background:{};border:1px solid {};border-radius:{};padding:12px 16px;width:100%;box-sizing:border-box;">
                <div style="font-family:{};font-size:10px;font-weight:800;color:{};letter-spacing:0.08em;text-transform:uppercase;margin-bottom:4px;">KEY IMPACT & PROOF</div>
                <div style="font-family:{};font-size:11px;color:{};line-height:1.45;">{}</div>
            </div>"#,
            card_bg, colors.border, radius,
            tokens.body_font, colors.primary,
            tokens.body_font, card_body_color, combined_desc
        )
    };
    let desc_html = if !description.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:var(--text-sm);color:{};line-height:1.45;margin:0;">{}</p>"#,
            tokens.body_font, card_body_color, escape_html(description)
        )
    } else {
        String::new()
    };
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:18px;">
            <h2 style="font-family:{};font-size:28px;font-weight:900;color:{};margin:0;line-height:1.08;">{}</h2>
            <div style="display:grid;grid-template-columns:1fr 1fr;gap:14px;">
                <div style="border-radius:{};padding:18px;background:{};border:1px solid {};border-left:3px solid {};"><div style="font-family:{};font-size:11px;font-weight:800;color:{};margin-bottom:8px;">PROBLEM</div><p style="font-family:{};font-size:var(--text-sm);color:{};line-height:1.45;margin:0;">{}</p></div>
                <div style="border-radius:{};padding:18px;background:{};border:1px solid {};border-left:3px solid {};"><div style="font-family:{};font-size:11px;font-weight:800;color:{};margin-bottom:8px;">SOLUTION</div><p style="font-family:{};font-size:var(--text-sm);color:{};line-height:1.45;margin:0;">{}</p></div>
            </div>
            {}
            {}
        </div>"#,
        tokens.heading_font,
        card_title_color,
        escape_html(title),
        radius,
        card_bg,
        colors.border,
        colors.primary,
        tokens.body_font,
        card_label_color,
        tokens.body_font,
        card_body_color,
        escape_html(problem),
        radius,
        card_bg,
        colors.border,
        colors.primary,
        tokens.body_font,
        card_label_color,
        tokens.body_font,
        card_body_color,
        escape_html(solution),
        proof_grid_html,
        desc_html
    );
    let html = slide_base(&content, tokens, bg_style, false, "16px 44px", "center");
    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "problem_solution", "theme": theme})
}

fn simple_text(v: &Value, keys: &[&str]) -> String {
    if let Some(s) = v.as_str() {
        return s.to_string();
    }
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
            r#"<div style="width:{}px;height:{}px;display:flex;align-items:center;justify-content:center;flex-shrink:0;"><img src="{}" alt="{}" style="max-width:{}px;max-height:{}px;width:auto;height:auto;object-fit:contain;display:block;"></div>"#,
            size,
            size,
            source,
            escape_html(fallback),
            (size as f32 * 1.6) as i32,
            size
        );
    }
    let icon_text = if !icon.is_empty() { icon } else { fallback };
    let bg = if colors.is_dark { "rgba(255,255,255,0.10)" } else { "rgba(0,0,0,0.06)" };
    format!(
        r#"<div style="width:{}px;height:{}px;border-radius:50%;background:{};display:flex;align-items:center;justify-content:center;font-size:{}px;font-weight:700;color:{};flex-shrink:0;">{}</div>"#,
        size,
        size,
        bg,
        (size as f32 * 0.45) as i32,
        colors.text_primary,
        escape_html(icon_text)
    )
}

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
    let radius = current_component_radius(tokens, "card");
    let card_bg = if is_dark { "rgba(255,255,255,0.05)" } else { "rgba(255,255,255,0.92)" };
    let border = format!("1px solid {}", colors.border);

    // Density scaling (shared overflow model): 4-5 items at fixed sizes stack
    // ~203px too tall under the new 41px-body type tiers. Scale item fonts,
    // padding, and gaps by step count so the stack fits the 405px safe height.
    let step_count = steps.len();
    // item_phase_size now sizes the number inside the 32px circular badge — it
    // needs to be legible there (12-13px), unlike the old 8.5px text chip.
    let (item_title_size, item_desc_size, item_pad, item_gap, item_phase_size) = match step_count {
        5 => (12.0, 10.0, 8, 3, 12.0),
        4 => (12.5, 10.5, 10, 4, 12.5),
        _ => (13.0, 11.0, 12, 6, 13.0),
    };
    let step_desc_text: usize = steps
        .iter()
        .map(|s| {
            s.get("description")
                .and_then(|v| v.as_str())
                .map(|d| d.len())
                .unwrap_or(0)
        })
        .sum::<usize>();
    // Long descriptions push the item body into extra lines: trim desc font further.
    let item_desc_size = if step_desc_text > 180 { item_desc_size - 1.0 } else { item_desc_size };

    let items_html: String = steps.iter().enumerate().map(|(idx, step)| {
        // Data keys mirror process_map: label/title/number for the step name and
        // description/caption for the body. Every timeline harness (stress decks,
        // audit viewer) uses label+description — reading only `title` produced
        // empty step names and left the PHASE chip as the sole differentiator.
        let step_title = simple_text(step, &["label", "title", "number"]);
        let step_desc = simple_text(step, &["description", "caption"]);
        let num_str = format!("{:02}", idx + 1);
        // process_map-style tile: circular number badge + bold title + muted
        // description. The badge gives each tile a strong visual anchor and the
        // title/description type hierarchy (800 vs 400 weight, larger vs smaller)
        // reads clearly — unlike the old tiny PHASE chip + same-size text.
        format!(
            r#"<div style="min-width:0;background:{};border:{};border-radius:{};padding:{}px 12px;box-sizing:border-box;display:flex;align-items:center;gap:12px;">
                <div style="width:32px;height:32px;border-radius:50%;background:{};color:{};display:flex;align-items:center;justify-content:center;font-family:{};font-size:{}px;font-weight:900;flex-shrink:0;">{}</div>
                <div style="flex:1;min-width:0;">
                    <div style="font-family:{};font-size:{}px;font-weight:800;color:{};line-height:1.2;">{}</div>
                    <div style="font-family:{};font-size:{}px;color:{};line-height:1.4;margin-top:2px;overflow-wrap:break-word;">{}</div>
                </div>
            </div>"#,
            card_bg, border, radius, item_pad,
            colors.primary, colors.button_text, tokens.heading_font, item_phase_size, num_str,
            tokens.heading_font, item_title_size, colors.text_primary, escape_html(&step_title),
            tokens.body_font, item_desc_size, colors.text_secondary, escape_html(&step_desc)
        )
    }).collect();

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:{}px;">
            {}
            <div style="display:flex;flex-direction:column;gap:{}px;width:100%;border-left:3px solid {};padding-left:14px;box-sizing:border-box;">{}</div>
        </div>"#,
        item_gap, heading, item_gap, colors.primary, items_html
    );
    let html = hero_layout(&content, tokens, bg_style, false, "left");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({"html": html, "background": bg_style, "variant": variant, "theme": theme})
}

pub fn definition_slide(
    tokens: &DesignTokens,
    term: &str,
    definition: &str,
    phonetic: &str,
    context: &str,
    bg_style: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let radius = current_component_radius(tokens, "card");

    let card_bg = if is_dark { "rgba(255,255,255,0.05)" } else { "rgba(255,255,255,0.92)" };
    let border = format!("1px solid {}", colors.border);

    let category_html = format!(
        r#"<span style="font-family:{};font-size:9.5px;font-weight:900;letter-spacing:0.12em;text-transform:uppercase;color:{};background:{}18;padding:2px 8px;border-radius:4px;display:inline-block;margin-bottom:8px;">CORE DEFINITION</span>"#,
        tokens.heading_font,
        colors.primary,
        colors.primary
    );

    let term_html = format!(
        r#"<h2 style="font-family:{};font-size:28px;font-weight:900;color:{};margin:0 0 4px;line-height:1.1;">{}</h2>"#,
        tokens.heading_font,
        colors.text_primary,
        escape_html(term)
    );

    let phonetic_html = if !phonetic.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:11px;font-style:italic;color:{};margin-bottom:12px;opacity:0.8;">{}</div>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(phonetic)
        )
    } else {
        String::new()
    };

    let def_html = format!(
        r#"<div style="border-left:3px solid {};padding-left:14px;margin:12px 0 14px;">
            <p style="font-family:{};font-size:13px;font-weight:500;color:{};margin:0;line-height:1.5;">{}</p>
        </div>"#,
        colors.primary,
        tokens.body_font,
        colors.text_primary,
        escape_html(definition)
    );

    let ctx_html = if !context.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:10.5px;color:{};margin:0;line-height:1.4;opacity:0.85;">{}</p>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(context)
        )
    } else {
        String::new()
    };

    let content = format!(
        r#"<div style="width:100%;background:{};border:{};border-radius:{};padding:22px 20px;box-sizing:border-box;display:flex;flex-direction:column;gap:4px;">
            {}
            {}
            {}
            {}
            {}
        </div>"#,
        card_bg, border, radius,
        category_html, term_html, phonetic_html, def_html, ctx_html
    );

    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        "16px 44px",
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
    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    let radius_md = current_component_radius(tokens, "card");
    let (card_bg, card_border, _) = card_styles(tokens, is_dark);
    let shadow_sm = tokens.shadows.get("sm").cloned().unwrap_or_else(|| "none".to_string());

    let effective_variant = variant;

    let myth_len = myth.len();
    let fact_len = fact.len();
    let dynamic_fs = if myth_len < 40 && fact_len < 40 {
        body_fs + 4
    } else if myth_len > 120 || fact_len > 120 {
        body_fs - 2
    } else {
        body_fs
    };

    let dynamic_padding = if myth_len < 40 && fact_len < 40 {
        "16px 20px"
    } else if myth_len > 120 || fact_len > 120 {
        "12px 14px"
    } else {
        "14px 18px"
    };

    let heading = heading_block(
        "Myth vs Fact",
        tokens,
        "title",
        Some(&colors.text_primary),
        false,
        None,
        "left",
        "0 0 12px",
        true,
    );

    let content = match effective_variant {
        "debunk" => {
            let myth_html = format!(
                r#"<div style="background:{};border:{};border-radius:{};padding:{};margin-bottom:12px;box-shadow:{};position:relative;flex-shrink:0;min-width:0;">
                    <div style="font-family:{};font-size:{}px;font-weight:600;color:{};text-decoration:line-through;text-decoration-color:{};text-decoration-thickness:2px;opacity:0.6;line-height:1.35;overflow-wrap:break-word;">{}</div>
                    <div style="position:absolute;top:50%;left:50%;transform:translate(-50%,-50%) rotate(-6deg);font-family:{};font-size:10px;font-weight:800;color:{};letter-spacing:0.12em;text-transform:uppercase;background:{};padding:4px 12px;border-radius:20px;box-shadow:0 2px 6px rgba(0,0,0,0.12);">MYTH</div>
                </div>"#,
                card_bg, card_border, radius_md, dynamic_padding, shadow_sm,
                tokens.body_font, dynamic_fs, colors.text_secondary, colors.primary,
                escape_html(myth),
                tokens.heading_font, colors.button_text, colors.primary,
            );
            let fact_html = format!(
                r#"<div style="background:{};border-left:4px solid {};border:{};border-left-width:4px;border-radius:{};padding:{};box-shadow:{};flex-shrink:0;min-width:0;">
                    <div style="font-family:{};font-size:10px;font-weight:800;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:4px;">FACT</div>
                    <div style="font-family:{};font-size:{}px;font-weight:600;color:{};line-height:1.35;overflow-wrap:break-word;">{}</div>
                </div>"#,
                card_bg, colors.primary, card_border, radius_md, dynamic_padding, shadow_sm,
                tokens.heading_font, colors.primary,
                tokens.body_font, dynamic_fs, colors.text_primary, escape_html(fact),
            );
            let explanation_html = if !explanation.is_empty() {
                format!(
                    r#"<div style="font-family:{};font-size:{}px;color:{};margin-top:14px;line-height:1.45;overflow-wrap:break-word;">{}</div>"#,
                    tokens.body_font, caption_fs, colors.text_secondary, escape_html(explanation)
                )
            } else {
                String::new()
            };
            // ONE centered group: heading (tight 12px bottom margin) + cards +
            // explanation. Centering the whole group (not the cards alone in a
            // tall middle region) kills the dead gap between the heading and the
            // cards that both the old layouts produced. The myth/fact cards
            // center against each other via align-items on the row.
            format!(
                r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;justify-content:center;overflow:hidden;">
                    {}
                    <div style="display:flex;flex-direction:column;width:100%;min-height:0;flex:0 1 auto;justify-content:center;overflow:hidden;">{}{}{}</div>
                </div>"#,
                heading, myth_html, fact_html, explanation_html
            )
        }
        _ => {
            // split (default) — myth and fact side by side
            let myth_html = format!(
                r#"<div style="flex:1;min-width:0;flex-shrink:1;">
                    <div style="font-family:{};font-size:10px;font-weight:800;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:8px;">MYTH</div>
                    <div style="background:{};border:{};border-radius:{};padding:{};box-shadow:{};box-sizing:border-box;">
                        <div style="font-family:{};font-size:{}px;font-weight:500;color:{};line-height:1.4;text-decoration:line-through;text-decoration-color:{};text-decoration-thickness:1.5px;opacity:0.6;overflow-wrap:break-word;">{}</div>
                    </div>
                </div>"#,
                tokens.heading_font, colors.text_secondary,
                card_bg, card_border, radius_md, dynamic_padding, shadow_sm,
                tokens.body_font, dynamic_fs, colors.text_secondary, colors.primary,
                escape_html(myth),
            );
            let fact_html = format!(
                r#"<div style="flex:1;min-width:0;flex-shrink:1;">
                    <div style="font-family:{};font-size:10px;font-weight:800;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:8px;">FACT</div>
                    <div style="background:{};border:{};border-left-width:4px;border-left-color:{};border-radius:{};padding:{};box-shadow:{};box-sizing:border-box;">
                        <div style="font-family:{};font-size:{}px;font-weight:600;color:{};line-height:1.4;overflow-wrap:break-word;">{}</div>
                    </div>
                </div>"#,
                tokens.heading_font, colors.primary,
                card_bg, card_border, colors.primary, radius_md, dynamic_padding, shadow_sm,
                tokens.body_font, dynamic_fs, colors.text_primary, escape_html(fact),
            );
            let explanation_html = if !explanation.is_empty() {
                format!(
                    r#"<div style="font-family:{};font-size:{}px;color:{};margin-top:14px;line-height:1.45;text-align:center;">{}</div>"#,
                    tokens.body_font, caption_fs, colors.text_secondary, escape_html(explanation)
                )
            } else {
                String::new()
            };
            // Split (default): one centered group — heading + myth/fact row +
            // explanation, tight internal gaps. See debunk branch comment for
            // why centering the WHOLE group (not the cards alone) matters.
            format!(
                r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;justify-content:center;overflow:hidden;">
                    {}
                    <div style="display:flex;gap:14px;width:100%;min-height:0;flex:0 1 auto;align-items:center;overflow:hidden;">{}{}</div>
                    {}
                </div>"#,
                heading, myth_html, fact_html, explanation_html
            )
        }
    };

    let html = slide_base(&content, tokens, bg_style, false, "20px 44px 20px", "center");
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
    
    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    
    // Calculate item count and total content for aggressive scaling
    let item_count = items.len();
    let total_content_len: usize = items.iter().take(6).map(|item| {
        let label = if item.is_string() {
            item.as_str().unwrap_or("").to_string()
        } else {
            simple_text(item, &["label", "title", "task", "step", "description", "text"])
        };
        label.len()
    }).sum();
    
    // Calculate total content and item count for dynamic scaling
    let item_count = items.len();
    let total_content_len: usize = items.iter().map(|item| {
        let label = if item.is_string() {
            item.as_str().unwrap_or("").to_string()
        } else {
            simple_text(item, &["label", "title", "task", "step", "description", "text"])
        };
        label.len()
    }).sum();
    
    // Calculate actual content requirements
    // Shared banded-chrome model: body region = composition − 36px header band
    // − 40px footer band. Single calibration point in overflow_model.rs so the
    // renderer's density scaling and the validator gate can never drift.
    const SAFE_CONTENT_HEIGHT: f32 = crate::overflow_model::SAFE_CONTENT_HEIGHT;
    
    // Estimate required height: title + items + gaps.
    // Empirical card heights derived from rendered geometry (per directive #1774):
    //  - 6+ items: ~56px each (24px number + 14px text + 12+12 padding + 4px flex)
    //  - 4-5 items: ~62px each (larger numbers + text + padding)
    //  - 1-3 items: ~70px each (largest layout, more breathing room)
    // Empirical gaps: 8px (rendered).
    let title_height = 30.0;
    let item_height_estimate = if item_count >= 6 {
        56.0
    } else if item_count >= 4 {
        62.0
    } else {
        70.0
    };
    let gap_estimate = if item_count >= 6 { 8.0 } else if item_count >= 4 { 8.0 } else { 10.0 };
    let estimated_content_height = title_height + (item_count as f32 * item_height_estimate) + ((item_count - 1) as f32 * gap_estimate);
    
    // Calculate required padding to fit within safe content height
    let total_padding_needed = SAFE_CONTENT_HEIGHT - estimated_content_height;
    let content_padding = if total_padding_needed < 40.0 {
        "16px var(--space-6) 16px" // Very aggressive
    } else if total_padding_needed < 60.0 {
        "16px var(--space-6) 20px" // Aggressive
    } else if total_padding_needed < 80.0 {
        "16px var(--space-6) 20px" // Moderate
    } else {
        "16px 44px" // Standard
    };
    
    // Scale fonts based on how tight the fit is
    let space_usage = estimated_content_height / SAFE_CONTENT_HEIGHT; // 0.0 to 1.0+
    let base_item_fs = caption_fs + 1;
    let base_num_fs = 12;
    
    let (item_fs, num_fs, card_padding, gap, heading_fs) = if space_usage > 0.85 {
        // Very tight fit - aggressive scaling
        ((base_item_fs as f32 * 0.75) as i32, 9, "6px 10px", 4, body_fs - 1)
    } else if space_usage > 0.75 {
        // Tight fit - moderate scaling
        ((base_item_fs as f32 * 0.85) as i32, 10, "8px 12px", 6, body_fs)
    } else {
        // Normal fit - standard sizing
        (base_item_fs, base_num_fs, "12px 14px", 8, body_fs + 1)
    };
    
    let rows = items
        .iter()
        .take(6)
        .enumerate()
        .map(|(idx, item)| {
            let label = if item.is_string() {
                item.as_str().unwrap_or("").to_string()
            } else {
                simple_text(item, &["label", "title", "task", "step", "description", "text"])
            };
            format!(
                r#"<div style="display:flex;gap:var(--space-1);align-items:flex-start;background:{};border:1px solid {};border-radius:{};padding:{};">
                    <div style="width:24px;height:24px;border-radius:50%;background:{};color:white;display:flex;align-items:center;justify-content:center;font-family:{};font-size:{}px;font-weight:800;flex-shrink:0;">{}</div>
                    <div style="font-family:{};font-size:{}px;font-weight:700;color:{};line-height:1.45;">{}</div>
                </div>"#,
                card_bg,
                colors.border,
                radius,
                card_padding,
                colors.primary,
                tokens.body_font,
                num_fs,
                idx + 1,
                tokens.body_font,
                item_fs,
                colors.text_primary,
                escape_html(&label)
            )
        })
        .collect::<Vec<_>>()
        .join("");
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:18px;"><h2 style="font-family:{};font-size:{}px;font-weight:900;color:{};margin:0;">{}</h2><div style="display:flex;flex-direction:column;gap:{}px;">{}</div></div>"#,
        tokens.heading_font,
        heading_fs,
        colors.text_primary,
        escape_html(title),
        gap,
        rows
    );
    let html = slide_base(&content, tokens, bg_style, false, content_padding, "center");
    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);
    json!({"html": html, "background": bg_style, "variant": "checklist_action_plan", "theme": theme})
}

pub fn case_study_result_slide(
    tokens: &DesignTokens,
    client: &str,
    challenge: &str,
    solution: &str,
    results: Vec<Value>,
    description: &str,
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
        description,
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
    // Supported tile counts: 1, 2, 3 or 4. 3 plans render as a 2-column grid
    // with the 3rd tile centered below (no compositional asymmetry); 4 plans
    // render as a balanced 2×2 grid. Configs with >4 plans are rejected by the
    // validator gate (see validate_slide_spec pricing_plan check).
    let plan_count = plans.len().min(4).max(1);
    
    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    
    // Calculate total content and plan count for aggressive scaling
    let total_content_len: usize = plans.iter().take(plan_count).map(|plan| {
        let name = simple_text(plan, &["name", "title"]);
        let features_arr = plan.get("features").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let features_len: usize = features_arr.iter().map(|f| f.as_str().unwrap_or("").len()).sum();
        name.len() + features_len
    }).sum();
    
    // Calculate actual content requirements
    // Shared banded-chrome model: body region = composition − 36px header band
    // − 40px footer band. Single calibration point in overflow_model.rs so the
    // renderer's density scaling and the validator gate can never drift.
    const SAFE_CONTENT_HEIGHT: f32 = crate::overflow_model::SAFE_CONTENT_HEIGHT;
    
    // Estimate required height: title + plans + gaps
    let title_height = 30.0; // 15px font + 15px margin
    let plan_height_estimate = if plan_count >= 4 {
        110.0 // 2×2 grid — compact cards: 10px name + 12px price + 8px header + 8px x 3 features + 7px footer + 12px padding + 14px gap
    } else if plan_count == 3 {
        120.0 // 12px name + 14px price + 10px header + 9px x 4 features + 8px footer + 16px padding + 20px gap
    } else if plan_count == 2 {
        140.0 // 14px name + 16px price + 12px header + 11px x 4 features + 10px footer + 20px padding + 24px gap
    } else {
        160.0 // 16px name + 18px price + 14px header + 12px x 4 features + 12px footer + 24px padding + 28px gap
    };
    let gap_estimate = if plan_count >= 4 { 10.0 } else if plan_count == 3 { 12.0 } else if plan_count == 2 { 16.0 } else { 20.0 };
    let estimated_content_height = title_height + (plan_count as f32 * plan_height_estimate) + ((plan_count - 1) as f32 * gap_estimate);
    
    // Calculate required padding to fit within safe content height
    let total_padding_needed = SAFE_CONTENT_HEIGHT - estimated_content_height;
    let content_padding = if total_padding_needed < 40.0 {
        "16px var(--space-6) 16px" // Very aggressive
    } else if total_padding_needed < 60.0 {
        "16px var(--space-6) 20px" // Aggressive
    } else if total_padding_needed < 80.0 {
        "16px var(--space-6) 20px" // Moderate
    } else {
        "16px 44px" // Standard
    };
    
    // Scale fonts based on how tight the fit is
    let space_usage = estimated_content_height / SAFE_CONTENT_HEIGHT; // 0.0 to 1.0+
    let base_price_fs = body_fs + 2;
    let base_name_fs = caption_fs + 2;
    let base_feature_fs = caption_fs + 2;
    let base_button_fs = caption_fs + 2;
    let base_heading_fs = body_fs + 1;
    
    let (price_fs, name_fs, feature_fs, card_padding, button_fs, heading_fs) = if space_usage > 0.85 {
        // Very tight fit - aggressive scaling
        ((base_price_fs as f32 * 0.75) as i32, (base_name_fs as f32 * 0.75) as i32, (base_feature_fs as f32 * 0.7) as i32, "8px 10px 6px", (base_button_fs as f32 * 0.75) as i32, (base_heading_fs as f32 * 0.85) as i32)
    } else if space_usage > 0.75 {
        // Tight fit - moderate scaling
        ((base_price_fs as f32 * 0.85) as i32, (base_name_fs as f32 * 0.85) as i32, (base_feature_fs as f32 * 0.85) as i32, "10px 12px 8px", (base_button_fs as f32 * 0.85) as i32, (base_heading_fs as f32 * 0.9) as i32)
    } else {
        // Normal fit - standard sizing
        (base_price_fs, base_name_fs, base_feature_fs, "14px 16px 12px", base_button_fs, base_heading_fs)
    };

    let cards: Vec<String> = plans
        .iter()
        .take(plan_count)
        .enumerate()
        .map(|(idx, plan)| {
            let name = simple_text(plan, &["name", "title"]);
            let price = simple_text(plan, &["price", "value"]);
            let period = simple_text(plan, &["period", "cycle"]);
            let is_featured = plan.get("featured").and_then(|v| v.as_bool()).unwrap_or(idx == 1 || (idx == 0 && plan_count == 1));

            let features_arr = plan
                .get("features")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            let mut features_html = String::new();
            for f in features_arr.iter().take(4) {
                let text = f.as_str().unwrap_or("");
                if !text.is_empty() {
                    features_html.push_str(&format!(
                        r#"<li style="display:flex;align-items:center;gap:6px;font-family:{};font-size:{}px;color:{};line-height:1.3;margin-bottom:4px;">
                            <span style="color:{};font-weight:900;font-size:{}px;">✓</span> {}
                        </li>"#,
                        tokens.body_font, feature_fs, colors.text_primary,
                        colors.primary, feature_fs + 1,
                        escape_html(text)
                    ));
                }
            }
            if !features_html.is_empty() {
                features_html = format!(r#"<ul style="list-style:none;padding:0;margin:10px 0 12px;">{}</ul>"#, features_html);
            }

            let (card_bg, card_border, badge_html, shadow) = if is_featured {
                (
                    if colors.is_dark { "rgba(99, 102, 241, 0.12)" } else { "rgba(255, 255, 255, 0.98)" },
                    format!("2px solid {}", colors.primary),
                    format!(
                        r#"<div style="position:absolute;top:-10px;right:12px;background:{};color:{};font-family:{};font-size:8.5px;font-weight:900;padding:2px 8px;border-radius:999px;letter-spacing:0.06em;text-transform:uppercase;">POPULAR</div>"#,
                        colors.primary, colors.button_text, tokens.heading_font
                    ),
                    "0 8px 24px rgba(0,0,0,0.15)".to_string(),
                )
            } else {
                (
                    if colors.is_dark { "rgba(255,255,255,0.05)" } else { "rgba(255,255,255,0.85)" },
                    format!("1px solid {}", colors.border),
                    String::new(),
                    "none".to_string(),
                )
            };

            let cta_text = if is_featured { "Upgrade Now" } else { "Get Started" };

            format!(
                r#"<div style="min-width:0;background:{};border:{};border-radius:{};padding:{};box-sizing:border-box;display:flex;flex-direction:column;justify-content:space-between;position:relative;box-shadow:{};">
                    {}
                    <div>
                        <div style="font-family:{};font-size:{}px;font-weight:800;color:{};letter-spacing:0.08em;text-transform:uppercase;margin-bottom:4px;">{}</div>
                        <div style="display:flex;align-items:baseline;gap:3px;margin-bottom:4px;">
                            <span style="font-family:{};font-size:{}px;font-weight:900;color:{};line-height:1;">{}</span>
                            <span style="font-family:{};font-size:{}px;color:{};">{}</span>
                        </div>
                        {}
                    </div>
                    <div style="margin-top:10px;padding:6px 0;background:{};color:{};border-radius:{};text-align:center;font-family:{};font-size:{}px;font-weight:800;">{}</div>
                </div>"#,
                card_bg,
                card_border,
                radius,
                card_padding,
                shadow,
                badge_html,
                tokens.heading_font,
                name_fs,
                colors.text_secondary,
                escape_html(&name),
                tokens.heading_font,
                price_fs,
                colors.text_primary,
                escape_html(&price),
                tokens.body_font,
                caption_fs,
                colors.text_secondary,
                escape_html(&period),
                features_html,
                if is_featured { &colors.primary } else { &colors.border },
                if is_featured { &colors.button_text } else { &colors.text_primary },
                current_component_radius(tokens, "button"),
                tokens.heading_font,
                button_fs,
                cta_text
            )
        })
        .collect();

    // Grid composition:
    //   1 plan  → single column, one row
    //   2 plans → two columns, one row
    //   3 plans → two columns; the 3rd tile is CENTERED below the first pair
    //             (grid-column 1/-1 with a half-column max-width) so the
    //             composition stays balanced instead of leaving a left-aligned
    //             orphan tile.
    //   4 plans → balanced 2×2 grid.
    let plan_grid = if plan_count == 3 {
        format!(
            r#"<div style="display:grid;grid-template-columns:repeat(2, minmax(0, 1fr));gap:12px;width:100%;min-width:0;">
                {}{}
                <div style="grid-column:1 / -1;display:flex;justify-content:center;min-width:0;">
                    <div style="width:100%;max-width:calc(50% - 6px);min-width:0;">{}</div>
                </div>
            </div>"#,
            cards[0],
            cards[1],
            cards[2]
        )
    } else {
        let grid_cols = if plan_count == 1 { 1 } else { 2 };
        format!(
            r#"<div style="display:grid;grid-template-columns:repeat({}, minmax(0, 1fr));gap:12px;width:100%;min-width:0;">{}</div>"#,
            grid_cols,
            cards.join("")
        )
    };

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:16px;min-width:0;">
            <h2 style="font-family:{};font-size:{}px;font-weight:900;color:{};margin:0;line-height:1.1;">{}</h2>
            {}
        </div>"#,
        tokens.heading_font,
        heading_fs,
        colors.text_primary,
        escape_html(title),
        plan_grid
    );
    let html = slide_base(
        &content,
        tokens,
        bg_style,
        false,
        content_padding,
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
    let html = slide_base(&content, tokens, bg_style, false, "16px 44px", "center");
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
    let html = slide_base(&content, tokens, bg_style, false, "16px 44px", "center");
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
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let heading = heading_block(title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 10px", true);
    let radius = current_component_radius(tokens, "card");
    let card_bg = if is_dark { "rgba(255,255,255,0.05)" } else { "rgba(255,255,255,0.92)" };
    let border = format!("1px solid {}", colors.border);

    // Hardcode MAX_FAQ_ITEMS = 4 to guarantee zero vertical overflow
    let capped_items: Vec<&Value> = questions.iter().take(4).collect();
    let count = capped_items.len();

    let (padding_css, q_size_px, a_size_px, gap_px) = match count {
        4 => ("8px 12px", 11.5, 10.5, 6),
        3 => ("10px 14px", 12.5, 11.0, 8),
        _ => ("12px 14px", 13.0, 11.5, 10),
    };

    let cards_html: String = capped_items.iter().enumerate().map(|(idx, q)| {
        let question_text = simple_text(q, &["question", "q", "title"]);
        let answer_text = simple_text(q, &["answer", "a", "description"]);
        format!(
            r#"<div style="background:{};border:{};border-radius:{};padding:{};box-sizing:border-box;display:flex;flex-direction:column;gap:3px;">
                <div style="display:flex;align-items:center;gap:6px;">
                    <span style="font-family:{};font-size:9.5px;font-weight:900;color:{};background:{}18;padding:1px 5px;border-radius:4px;flex-shrink:0;">Q{}</span>
                    <h3 style="font-family:{};font-size:{}px;font-weight:800;color:{};margin:0;line-height:1.2;">{}</h3>
                </div>
                <p style="font-family:{};font-size:{}px;color:{};margin:0;line-height:1.4;">{}</p>
            </div>"#,
            card_bg, border, radius, padding_css,
            tokens.heading_font, colors.primary, colors.primary, idx + 1,
            tokens.heading_font, q_size_px, colors.text_primary, escape_html(&question_text),
            tokens.body_font, a_size_px, colors.text_secondary, escape_html(&answer_text)
        )
    }).collect();

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:{}px;">
            {}
            <div style="display:flex;flex-direction:column;gap:{}px;width:100%;">{}</div>
        </div>"#,
        gap_px, heading, gap_px, cards_html
    );
    let html = hero_layout(&content, tokens, bg_style, false, "left");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({"html": html, "background": bg_style, "variant": "faq", "theme": theme})
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
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    
    // Calculate total content and step count for dynamic scaling
    let step_count = steps.len();
    let total_content_len: usize = steps.iter().map(|step| {
        let step_title = simple_text(step, &["label", "title", "number"]);
        let step_desc = simple_text(step, &["description", "caption"]);
        step_title.len() + step_desc.len()
    }).sum();
    
    // Calculate actual content requirements
    // Shared banded-chrome model: body region = composition − 36px header band
    // − 40px footer band. Single calibration point in overflow_model.rs so the
    // renderer's density scaling and the validator gate can never drift.
    const SAFE_CONTENT_HEIGHT: f32 = crate::overflow_model::SAFE_CONTENT_HEIGHT;
    
    // Estimate required height: title + items + gaps
    let title_height = 37.0; // 25px font + 12px margin
    let item_height_estimate = if step_count >= 6 {
        55.0 // 17px title + 14px desc + 14px padding + 10px gap
    } else if step_count >= 4 {
        62.0 // 18px title + 16px desc + 18px padding + 10px gap
    } else {
        70.0 // 20px title + 18px desc + 22px padding + 10px gap
    };
    let gap_estimate = if step_count >= 6 { 10.0 } else if step_count >= 4 { 12.0 } else { 14.0 };
    let estimated_content_height = title_height + (step_count as f32 * item_height_estimate) + ((step_count - 1) as f32 * gap_estimate);
    
    // Calculate required padding to fit within safe content height
    let total_padding_needed = SAFE_CONTENT_HEIGHT - estimated_content_height;
    let content_padding = if total_padding_needed < 40.0 {
        "16px var(--space-6) 16px" // Very aggressive
    } else if total_padding_needed < 60.0 {
        "16px var(--space-6) 20px" // Aggressive
    } else if total_padding_needed < 80.0 {
        "16px var(--space-6) 20px" // Moderate
    } else {
        "16px var(--space-6) 20px" // Standard
    };
    
    // Scale fonts based on how tight the fit is
    let space_usage = estimated_content_height / SAFE_CONTENT_HEIGHT; // 0.0 to 1.0+
    let base_title_fs = body_fs + 1;
    let base_desc_fs = caption_fs + 1;
    let base_num_fs = 13;
    
    let (title_fs, desc_fs, num_fs, card_padding, gap) = if space_usage > 0.85 {
        // Very tight fit - aggressive scaling
        ((base_title_fs as f32 * 0.75) as i32, (base_desc_fs as f32 * 0.75) as i32, 10, "8px 10px 6px", 4)
    } else if space_usage > 0.75 {
        // Tight fit - moderate scaling
        ((base_title_fs as f32 * 0.85) as i32, (base_desc_fs as f32 * 0.85) as i32, 11, "10px 12px 8px", 6)
    } else {
        // Normal fit - standard sizing
        (base_title_fs, base_desc_fs, base_num_fs, "14px 14px 12px", 10)
    };
    
    let heading = heading_block(title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 12px", true);
    let radius = current_component_radius(tokens, "card");
    let card_bg = if is_dark { "rgba(255,255,255,0.05)" } else { "rgba(255,255,255,0.92)" };
    let border = format!("1px solid {}", colors.border);

    let rows: String = steps.iter().enumerate().map(|(idx, step)| {
        let step_title = simple_text(step, &["label", "title", "number"]);
        let step_desc = simple_text(step, &["description", "caption"]);
        let num_str = format!("0{}", idx + 1);
        format!(
            r#"<div style="min-width:0;background:{};border:{};border-radius:{};padding:{};box-sizing:border-box;display:flex;align-items:center;gap:12px;">
                <div style="width:34px;height:34px;border-radius:50%;background:{};color:{};display:flex;align-items:center;justify-content:center;font-family:{};font-size:{}px;font-weight:900;flex-shrink:0;">{}</div>
                <div style="flex:1;min-width:0;">
                    <div style="font-family:{};font-size:{}px;font-weight:800;color:{};margin-bottom:2px;">{}</div>
                    <div style="font-family:{};font-size:{}px;color:{};line-height:1.4;overflow-wrap:break-word;">{}</div>
                </div>
            </div>"#,
            card_bg, border, radius, card_padding,
            colors.primary, colors.button_text, tokens.heading_font, num_fs, num_str,
            tokens.heading_font, title_fs, colors.text_primary, escape_html(&step_title),
            tokens.body_font, desc_fs, colors.text_secondary, escape_html(&step_desc)
        )
    }).collect();

    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:{}px;">
            {}
            <div style="display:flex;flex-direction:column;gap:{}px;width:100%;">{}</div>
            <p style="font-family:{};font-size:10.5px;color:{};margin:4px 0 0;line-height:1.4;opacity:0.85;">Automated 3-step compilation pipeline converting raw JSON specifications into production-ready carousel assets.</p>
        </div>"#,
        gap, heading, gap, rows, tokens.body_font, colors.text_secondary
    );
    let html = slide_base(&content, tokens, bg_style, false, content_padding, "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({"html": html, "background": bg_style, "variant": "process_map", "theme": theme})
}

pub fn before_after_story_slide(
    tokens: &DesignTokens,
    title: &str,
    before: &str,
    after: &str,
    metric: &str,
    metric_label: &str,
    description: &str,
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
    } else if !metric_label.is_empty() {
        // Full result/impact tile when metric_label is provided
        format!(
            r#"<div style="margin-top:14px;border-radius:{};background:{};border:1px solid {};padding:var(--space-2) 16px;">
                <div style="font-family:{};font-size:11px;font-weight:900;color:{};letter-spacing:0.06em;margin-bottom:6px;">{}</div>
                <div style="font-family:{};font-size:var(--text-lg);font-weight:800;color:{};line-height:1.2;">{}</div>
            </div>"#,
            radius,
            card_bg,
            colors.border,
            tokens.heading_font,
            colors.primary,
            escape_html(metric_label),
            tokens.body_font,
            colors.text_primary,
            escape_html(metric)
        )
    } else {
        // Legacy metric badge fallback
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
    let desc_html = if !description.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:var(--text-sm);color:{};line-height:1.45;margin:0;">{}</p>"#,
            tokens.body_font, colors.text_secondary, escape_html(description)
        )
    } else {
        String::new()
    };
    // Card text colors: on light slides, cards use a light card_bg, so their
    // text must be dark (using token text_primary/secondary). On dark slides,
    // cards use dark card_bg, so text must be light (using text_on_dark).
    let card_label_color = if colors.is_dark {
        colors.text_primary.clone()
    } else {
        tokens.text_primary.clone()
    };
    let card_body_color = if colors.is_dark {
        colors.text_secondary.clone()
    } else {
        tokens.text_secondary.clone()
    };
    let content = format!(
        r#"<div style="width:100%;display:flex;flex-direction:column;gap:18px;">
            <h2 style="font-family:{};font-size:28px;font-weight:900;color:{};margin:0;line-height:1.08;">{}</h2>
            <div style="display:grid;grid-template-columns:1fr auto 1fr;gap:var(--space-1);align-items:stretch;">
                <div style="border-radius:{};padding:16px;background:{};border:1px solid {};box-sizing:border-box;">
                    <div style="font-family:{};font-size:11px;font-weight:900;color:{};margin-bottom:8px;letter-spacing:0.06em;">BEFORE</div>
                    <p style="font-family:{};font-size:var(--text-sm);color:{};line-height:1.45;margin:0;">{}</p>
                </div>
                <div style="display:flex;align-items:center;justify-content:center;color:{};font-family:{};font-size:22px;font-weight:900;">→</div>
                <div style="border-radius:{};padding:16px;background:{};border:1px solid {};box-sizing:border-box;">
                    <div style="font-family:{};font-size:11px;font-weight:900;color:{};margin-bottom:8px;letter-spacing:0.06em;">AFTER</div>
                    <p style="font-family:{};font-size:var(--text-sm);color:{};line-height:1.45;margin:0;">{}</p>
                </div>
            </div>
            {}
            {}
        </div>"#,
        tokens.heading_font,
        card_label_color,
        escape_html(title),
        radius,
        card_bg,
        colors.border,
        tokens.body_font,
        colors.primary,
        tokens.body_font,
        card_body_color,
        escape_html(before),
        colors.primary,
        tokens.heading_font,
        radius,
        card_bg,
        colors.border,
        tokens.body_font,
        colors.primary,
        tokens.body_font,
        card_body_color,
        escape_html(after),
        metric_html,
        desc_html
    );
    let html = slide_base(&content, tokens, bg_style, false, "16px 44px", "center");
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

    let url_badge_html = if !short_url.is_empty() {
        format!(
            r#"<div style="margin-top:8px;font-family:{};font-size:10px;font-weight:700;color:#1F2937;background:rgba(0,0,0,0.04);border:1px solid rgba(0,0,0,0.1);border-radius:12px;padding:4px 10px;text-align:center;letter-spacing:0.02em;box-sizing:border-box;">🔗 {}</div>"#,
            tokens.body_font,
            escape_html(short_url)
        )
    } else {
        String::new()
    };

    let cta_html = if !cta_text.is_empty() {
        format!(
            r#"<div style="margin-top:10px;background:{};color:{};font-family:{};font-size:13px;font-weight:800;padding:10px 24px;border-radius:20px;box-shadow:0 4px 12px rgba(0,0,0,0.12);text-align:center;letter-spacing:-0.01em;display:inline-block;">{}</div>"#,
            colors.primary,
            colors.button_text,
            tokens.heading_font,
            escape_html(cta_text)
        )
    } else {
        String::new()
    };

    let qr_card_html = format!(
        r#"<div style="background:#FFFFFF;border:1px solid rgba(0,0,0,0.1);border-radius:16px;padding:16px;display:flex;flex-direction:column;align-items:center;box-shadow:0 16px 36px rgba(0,0,0,0.18);box-sizing:border-box;margin:10px auto;">
            <img src="{}" alt="{}" style="width:160px;height:160px;display:block;" />
            {}
        </div>"#,
        qr_src,
        escape_html(effective_alt),
        url_badge_html
    );

    let include_heading = !matches!(effective_variant, "minimal" | "without-heading" | "with-caption" | "with-cta");
    let include_caption = !matches!(effective_variant, "minimal" | "without-caption" | "with-heading" | "with-cta");
    let include_incentive = !matches!(effective_variant, "minimal");

    let heading_html = if include_heading && !heading.is_empty() {
        format!(
            r#"<h2 style="font-family:{};font-size:22px;font-weight:900;color:{};margin:0 0 4px;line-height:1.2;text-align:center;letter-spacing:-0.01em;">{}</h2>"#,
            tokens.heading_font,
            colors.text_primary,
            escape_html(heading)
        )
    } else {
        String::new()
    };

    let caption_html = if include_caption && !caption.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:11px;line-height:1.4;color:{};margin:8px 0 0;text-align:center;max-width:320px;opacity:0.85;">{}</p>"#,
            tokens.body_font,
            colors.text_secondary,
            escape_html(caption)
        )
    } else {
        String::new()
    };

    let incentive_html = if include_incentive && !incentive_text.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:11px;font-weight:700;color:{};background:{};border:1px solid {};border-radius:20px;padding:4px 14px;text-align:center;margin-top:8px;">🎁 {}</div>"#,
            tokens.body_font,
            colors.text_primary,
            if is_dark { "rgba(255,255,255,0.06)" } else { "rgba(0,0,0,0.035)" },
            colors.border,
            escape_html(incentive_text)
        )
    } else {
        String::new()
    };

    let content = format!(
        r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;align-items:center;justify-content:center;box-sizing:border-box;">
            {}
            {}
            {}
            {}
            {}
            {}
        </div>"#,
        brand_html,
        heading_html,
        qr_card_html,
        cta_html,
        caption_html,
        incentive_html
    );

    let layout_padding = if !padding.is_empty() { padding } else { "16px 36px" };
    let html = slide_base(&content, tokens, bg_style, false, layout_padding, "center");

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

/// CTA slide types for visual carousels — structurally distinct persuasion architectures.
///
/// Each type embodies a different way to drive conversion:
/// - big_statement: Brand confidence via massive typography
/// Big statement CTA: single dominant focal element.
/// Variant "default": massive centered text (movie poster style).
/// Variant "stat": giant number/label dominates visual field.
pub fn big_statement_slide(
    tokens: &DesignTokens,
    heading: &str,
    body: &str,
    stat_value: &str,
    stat_label: &str,
    cta_text: &str,
    url: &str,
    bg_style: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_stat = !stat_value.is_empty();

    // Hero element: stat mode (giant number) or text mode (massive heading)
    let hero_html = if is_stat {
        // Stat mode — giant number + label
        let stat_num = format!(
            r#"<div style="font-family:{};font-size:72px;font-weight:900;color:{};line-height:1.0;letter-spacing:-0.04em;text-align:center;">{}</div>"#,
            tokens.heading_font, colors.primary, escape_html(stat_value)
        );
        let label = if !stat_label.is_empty() {
            format!(
                r#"<div style="font-family:{};font-size:11px;font-weight:700;color:{};text-transform:uppercase;letter-spacing:0.12em;margin-top:8px;text-align:center;">{}</div>"#,
                tokens.body_font, colors.text_secondary, escape_html(stat_label)
            )
        } else { String::new() };
        format!("{}{}", stat_num, label)
    } else {
        // Text mode — watermark + heading
        let watermark = if !heading.is_empty() {
            let ch = heading.chars().next().unwrap_or('A');
            format!(
                r#"<div style="position:absolute;top:50%;left:50%;transform:translate(-50%,-55%);font-family:{};font-size:220px;font-weight:900;color:{};opacity:0.04;pointer-events:none;user-select:none;line-height:1;">{}</div>"#,
                tokens.heading_font, colors.text_primary, escape_html(&ch.to_string())
            )
        } else { String::new() };
        let h = if !heading.is_empty() {
            format!(
                r#"<h2 style="font-family:{};font-size:52px;font-weight:900;color:{};margin:0;line-height:1.0;letter-spacing:-0.04em;text-align:center;z-index:2;">{}</h2>"#,
                tokens.heading_font, colors.text_primary, escape_html(heading)
            )
        } else { String::new() };
        format!("{}{}", watermark, h)
    };

    // Accent line — between hero and body
    let accent_html = if is_stat && !heading.is_empty() {
        format!(
            r#"<div style="width:60px;height:2px;background:{};opacity:0.4;margin:18px auto;"></div>"#,
            colors.text_primary
        )
    } else if !is_stat && !heading.is_empty() && !body.is_empty() {
        format!(
            r#"<div style="width:60px;height:2px;background:{};opacity:0.4;margin:18px auto;z-index:2;"></div>"#,
            colors.text_primary
        )
    } else { String::new() };

    // Body text — only in text mode; in stat mode, heading serves as supporting text
    let body_html = if is_stat && !heading.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:14px;line-height:1.5;color:{};text-align:center;max-width:280px;margin:8px auto 0;">{}</div>"#,
            tokens.body_font, colors.text_secondary, escape_html(heading)
        )
    } else if !is_stat && !body.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:15px;font-style:italic;line-height:1.6;color:{};margin:10px auto 0;max-width:360px;text-align:center;z-index:2;">{}</p>"#,
            tokens.body_font, colors.text_secondary, escape_html(body)
        )
    } else { String::new() };

    // Action pill — in-flow, pushed to bottom by flex spacer
    let action_html = if !cta_text.is_empty() {
        let pill_bg = if colors.is_dark { "rgba(255,255,255,0.12)" } else { "rgba(0,0,0,0.08)" };
        format!(
            r#"<div style="text-align:center;z-index:2;margin-top:24px;">
                <span style="display:inline-block;font-family:{};font-size:11px;font-weight:800;color:{};text-transform:uppercase;letter-spacing:0.12em;background:{};padding:12px 28px;border-radius:100px;">{}</span>
            </div>"#,
            tokens.body_font, colors.text_primary, pill_bg, escape_html(cta_text)
        )
    } else { String::new() };

    let content = format!(
        r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;align-items:center;justify-content:center;box-sizing:border-box;padding:0 40px;position:relative;overflow:hidden;">
            <div style="text-align:center;display:flex;flex-direction:column;align-items:center;">
                {}
                {}
                {}
                {}
            </div>
        </div>"#,
        hero_html, accent_html, body_html, action_html
    );

    let variant = if is_stat { "stat" } else { "default" };
    let html = slide_base(&content, tokens, bg_style, false, "0", "center");
    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": variant,
        "theme": theme
    })
}

/// Comment-to-DM CTA slide.
/// Left-aligned headline + sub-headline + CTA instruction with highlighted keyword.
fn comment_cta_slide(
    tokens: &DesignTokens,
    heading: &str,
    sub_heading: &str,
    cta_text: &str,
    keyword: &str,
    bg_style: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);

    // Headline
    let headline_html = if !heading.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:38px;font-weight:900;color:{};line-height:1.05;letter-spacing:-0.03em;text-align:left;font-style:italic;">{}</div>"#,
            tokens.heading_font, colors.text_primary, escape_html(heading)
        )
    } else { String::new() };

    // Sub-headline (italic)
    let sub_html = if !sub_heading.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:16px;font-style:italic;line-height:1.5;color:{};text-align:left;margin-top:16px;">{}</div>"#,
            tokens.body_font, colors.text_secondary, escape_html(sub_heading)
        )
    } else { String::new() };

    // CTA instruction with highlighted keyword
    let cta_html = if !cta_text.is_empty() {
        let rendered = if !keyword.is_empty() {
            let kw_esc = escape_html(keyword);
            let kw_hl = format!(
                r#"<span style="color:{};font-weight:900;text-decoration:underline;text-underline-offset:3px;">{}</span>"#,
                colors.primary, kw_esc
            );
            escape_html(cta_text).replace(&kw_esc, &kw_hl)
        } else {
            escape_html(cta_text)
        };
        format!(
            r#"<div style="font-family:{};font-size:18px;font-weight:700;color:{};text-align:left;margin-top:24px;line-height:1.5;">{}</div>"#,
            tokens.body_font, colors.text_primary, rendered
        )
    } else { String::new() };

    let content = format!(
        r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;box-sizing:border-box;padding:44px 44px 36px;position:relative;overflow:hidden;">
        <div style="flex:1;display:flex;align-items:center;justify-content:center;">
            <div style="text-align:left;display:flex;flex-direction:column;align-items:flex-start;">
                {}{}{}
            </div>
        </div></div>"#,
        headline_html, sub_html, cta_html
    );

    let html = slide_base(&content, tokens, bg_style, false, "0", "center");
    let html = inject_background_image(html, background_image, image_opacity, colors.is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": "default",
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
    // Build a tokens copy.
    let tokens_owned = tokens.clone();
    let tokens = &tokens_owned;

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
    // Extract text from either a plain string or an object with label/description/title/body keys.
    // For objects, concatenates label+description (or title+body) to capture the full content.
    let s_or_text = |key: &str| {
        p.get(key).map(|v| {
            if let Some(s) = v.as_str() {
                return s.to_string();
            }
            let primary = ["label", "title"]
                .iter()
                .find_map(|k| v.get(*k).and_then(|x| x.as_str()))
                .unwrap_or("");
            let secondary = ["description", "body"]
                .iter()
                .find_map(|k| v.get(*k).and_then(|x| x.as_str()))
                .unwrap_or("");
            if !secondary.is_empty() && !primary.is_empty() {
                format!("{}: {}", primary, secondary)
            } else if !secondary.is_empty() {
                secondary.to_string()
            } else {
                primary.to_string()
            }
        }).unwrap_or_default()
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
            &s("variant").if_empty("centered"),
            theme,
            &bg_img,
            img_opacity,
            &s("tagline"),
            &s("metric_value"),
            &s("metric_label"),
        )),
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
        "comparison" => {
            // comparison removed (all 4 variants: cards, vs-split, feature-matrix, table).
            // Redirect legacy callers to before_after_story which is the closest semantic
            // equivalent (A vs B framing). First row of `rows` becomes (before, after);
            // remaining row content goes into a metric/label fallback.
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
            let before = rows
                .first()
                .and_then(|r| r.first().cloned())
                .unwrap_or_default();
            let after = rows
                .first()
                .and_then(|r| r.get(1).cloned())
                .unwrap_or_default();
            let metric = s("metric");
            let metric_label = s("metric_label");
            let _ = (columns, rows); // consumed for shape
            Ok(before_after_story_slide(
                tokens,
                &s("title"),
                &before,
                &after,
                &metric,
                &metric_label,
                &s("description"),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "stat_row" => {
            // stat_row removed (folded into metric_grid which has the same N-stat grid
            // semantics). Route legacy callers to metric_grid so existing JSON keeps working.
            let mut params = p.clone();
            if let Some(obj) = params.as_object_mut() {
                // Rename `stats` → `metrics` if needed; metric_grid expects metrics: [{value,label,trend?}]
                if !obj.contains_key("metrics") {
                    if let Some(stats) = obj.remove("stats") {
                        obj.insert("metrics".to_string(), stats);
                    }
                }
            }
            let metrics = params
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
            Err(format!(
                "grid_cards slide type has been removed. Use split_features for 2-column feature lists or chart_slide with stat_row data for N-stat grids. Cards input was: {} items",
                cards.len()
            ))
        }
        "definition" => Ok(definition_slide(
            tokens,
            &s("term"),
            &s("definition"),
            &s("phonetic"),
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
            &s("subtitle"),
            &s("text_align"),
            &s("max_width"),
            &s("variant"),
        )),
        // metric_card removed — use metric_grid, comparison_bars, gauge, or progress_rings
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
                &s("description").if_empty(&s("caption")),
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
        "gauge" => {
            // Try numeric first, then parse string values like "72"
            let gauge_val = p.get("value")
                .and_then(|v| v.as_f64())
                .or_else(|| p.get("value").and_then(|v| v.as_str()).and_then(|s| s.parse::<f64>().ok()))
                .unwrap_or(0.0);
            Ok(gauge_slide(
                tokens,
                gauge_val,
                &s("label"),
                &s("title"),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
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
                &s("description").if_empty(&s("caption")),
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
            Err(format!(
                "text_columns slide type has been removed. Use split_features for 2-column body content or quote_slide for parallel quotes. Columns input was: {} items",
                columns.len()
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
                &s("description").if_empty(&s("caption")),
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
        "metric_sparkline" => Ok(metric_card_slide(
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
        "column_chart" => {
            // column_chart removed (merged into chart_slide chart_type="bar_vertical")
            // Route to chart_slide for backwards compatibility with existing callers.
            let data = p
                .get("data")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            Ok(chart_slide(
                tokens,
                "bar_vertical",
                data,
                &s("title"),
                &s("description").if_empty(&s("caption")),
                bg_style,
                theme,
                &bg_img,
                img_opacity,
            ))
        }
        "section_divider" => {
            // section_divider REMOVED (2026-08): redundant with hero (chapter
            // variant), which renders the same kicker + accent bar + title +
            // subtitle layout. Redirect legacy callers to hero chapter.
            Ok(hero_slide(
                tokens,
                &s("title").if_empty(&s("headline")),
                &s("subtitle").if_empty(&s("subheadline")),
                &s("kicker").if_empty(&s("label")),
                bg_style,
                true,
                "chapter",
                theme,
                &bg_img,
                img_opacity,
                "",
                "",
                "",
            ))
        }
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
                &s("description"),
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
                &s("description"),
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
            &s_or_text("before"),
            &s_or_text("after"),
            &s("metric"),
            &s("metric_label"),
            &s("description"),
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
        "image_stat" => Err(
            "image_stat slide type has been removed. Use image_callout for image+text combos, image_caption for image+caption, or metric_grid for stat-heavy slides without image."
                .to_string(),
        ),
        "feature" => Err(
            "feature slide type has been removed. Use split_features for single-benefit slides or grid_cards — wait, grid_cards was also removed. Use split_features for single-feature or case_study_result for narrative beats."
                .to_string(),
        ),
        "list" => Err(
            "list slide type has been removed. Use checklist_action_plan for action lists or quote_slide for quoted step sequences."
                .to_string(),
        ),
        "cta" => Err(
            "cta slide type has been removed. Use qr_destination for the closing slide with QR code (the only CTA slide allowed by deck-level marketing constraints)."
                .to_string(),
        ),
        "callout" => Err(
            "callout slide type has been removed. Use myth_fact for 'myth vs fact' callouts or image_callout for image-anchored callout boxes."
                .to_string(),
        ),
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
            &s("title"),
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
        "big_statement" => Ok(big_statement_slide(
            tokens,
            &s("heading").if_empty(&s("title")),
            &s("body").if_empty(&s("description")),
            &s("stat_value").if_empty(&s("value")),
            &s("stat_label").if_empty(&s("label")),
            &s("cta_text").if_empty(&s("button_text")),
            &s("url").if_empty(&s("destination_url")),
            bg_style,
            &bg_img,
            img_opacity,
            theme,
        )),
        "comment_cta" => Ok(comment_cta_slide(
            tokens,
            &s("heading").if_empty(&s("title")),
            &s("sub_heading").if_empty(&s("subtitle")),
            &s("cta_text").if_empty(&s("action_text")),
            &s("keyword"),
            bg_style,
            &bg_img,
            img_opacity,
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
    treatment.image_mask = "none".to_string();
    if treatment.image_frame != "sharp" {
        treatment.image_frame = "rounded".to_string();
    }
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;

    let img_height = if layout == "image-left" || layout == "image-right" {
        "100%"
    } else {
        "225px"
    };

    let img_html = render_themed_image(
        image_url, tokens, &treatment, "100%", img_height, caption, is_dark,
    );

    let heading_html = if !caption.is_empty() {
        heading_block(caption, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 8px", true)
    } else {
        String::new()
    };

    let desc_style = format!(
        "font-family:{};font-size:12px;color:{};margin:0;line-height:1.45;opacity:0.9;",
        tokens.body_font, colors.text_secondary
    );

    let text_html = format!(
        r#"<div style="display:flex;flex-direction:column;justify-content:center;">
            {}
            {}
        </div>"#,
        heading_html,
        if !description.is_empty() {
            format!(r#"<p style="{}">{}</p>"#, desc_style, escape_html(description))
        } else {
            String::new()
        }
    );

    let content = match layout {
        "image-bottom" => {
            format!(
                r#"<div style="display:flex;flex-direction:column;gap:12px;width:100%;height:100%;justify-content:center;">
                    {}
                    {}
                </div>"#,
                text_html, img_html
            )
        }
        "image-left" => {
            format!(
                r#"<div style="display:grid;grid-template-columns:1.2fr 1fr;gap:16px;width:100%;height:100%;align-items:center;">
                    {}
                    {}
                </div>"#,
                img_html, text_html
            )
        }
        "image-right" => {
            format!(
                r#"<div style="display:grid;grid-template-columns:1fr 1.2fr;gap:16px;width:100%;height:100%;align-items:center;">
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
            overlay_treatment.image_overlay = "gradient".to_string();
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
                    <div style="position:absolute;bottom:0;left:0;right:0;padding:24px 28px 76px 28px;z-index:3;color:white;">
                        <h3 style="font-family:{};font-size:24px;font-weight:800;color:white;margin:0 0 8px;line-height:1.2;text-shadow:0 2px 8px rgba(0,0,0,0.7);">{}</h3>
                        {}
                    </div>
                </div>"#,
                img_full,
                tokens.heading_font,
                escape_html(caption),
                if !description.is_empty() {
                    format!(
                        r#"<p style="font-family:{};font-size:12px;color:rgba(255,255,255,0.95);margin:0;line-height:1.45;text-shadow:0 2px 6px rgba(0,0,0,0.6);">{}</p>"#,
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
                r#"<div style="display:flex;flex-direction:column;gap:12px;width:100%;height:100%;justify-content:center;">
                    {}
                    {}
                </div>"#,
                img_html, text_html
            )
        }
    };

    let padding_val = if padding.is_empty() {
        "16px var(--space-6) 20px"
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
    treatment.image_frame = "sharp".to_string();
    treatment.image_mask = "none".to_string();
    treatment.image_overlay = "gradient".to_string();
    // Anchor the scrim under the text zone: top/center/bottom text placement
    // each get a gradient whose dark end tracks them (bright-image safety).
    treatment.overlay_anchor = match overlay_position {
        "top" => "top",
        "center" => "center",
        _ => "bottom",
    }
    .to_string();

    let img_html = render_themed_image(
        image_url, tokens, &treatment, "100%", "100%", headline, true,
    );

    let v_align = match overlay_position {
        "center" => "center",
        "top" => "flex-start",
        _ => "flex-end",
    };

    // Bottom-anchored overlay clearance: the full-bleed image layer bleeds
    // behind the transparent footer band, so the text overlay must carry extra
    // bottom padding to keep headline/subheadline above the footer chrome
    // (40px band + 8px progress strip). Measured overlap on a bottom-aligned
    // image_headline: subheadline bottom sat ~16px inside the footer band.
    let overlay_padding = match overlay_position {
        "top" => "76px 28px 60px",
        "center" => "60px 28px",
        _ => "60px 28px calc(96px + var(--chrome-footer-h, 40px))",
    };

    // Layered text-shadow stack: crisp 1px edge hold + mid offset + soft halo
    // keeps glyphs readable even where the scrim is deliberately light.
    let headline_style = format!(
        "font-family:{};font-size:32px;font-weight:800;color:white;margin:0;line-height:1.15;letter-spacing:-0.02em;text-shadow:0 1px 2px rgba(0,0,0,0.85),0 2px 6px rgba(0,0,0,0.55),0 4px 18px rgba(0,0,0,0.45);",
        tokens.heading_font
    );
    let sub_style = format!(
        "font-family:{};font-size:13.5px;color:rgba(255,255,255,0.95);margin:10px 0 0;line-height:1.45;text-shadow:0 1px 2px rgba(0,0,0,0.8),0 2px 8px rgba(0,0,0,0.5);",
        tokens.body_font
    );

    let content = format!(
        r#"<div style="position:relative;width:100%;height:100%;">
            {}
            <div style="position:absolute;inset:0;padding:{};display:flex;flex-direction:column;justify-content:{};z-index:3;">
                <h2 style="{}">{}</h2>
                {}
            </div>
        </div>        "#,
        img_html,
        overlay_padding,
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
    treatment.image_overlay = "gradient".to_string();
    // Centered multi-line quote -> center-weighted scrim (radial), so bright
    // image regions behind and between quote lines stay darkened.
    treatment.overlay_anchor = "center".to_string();

    // image_quote always renders white text over the full-bleed photo, so the
    // slide is always treated as dark (scrim on) — even if a light bg_style is
    // passed — otherwise a light bg drops the overlay (render_themed_image
    // clears it for light) and leaves white-on-white text.
    let is_dark = true;

    let img_html = render_themed_image(image_url, tokens, &treatment, "100%", "100%", quote, is_dark);

    let content = format!(
        r#"<div style="position:relative;width:100%;height:100%;display:flex;align-items:center;justify-content:center;">
            {}
            <div style="position:absolute;inset:0;padding:60px 28px;display:flex;flex-direction:column;justify-content:center;align-items:center;text-align:center;z-index:3;">
                <div style="font-size:36px;color:white;line-height:1;margin-bottom:8px;font-weight:bold;opacity:0.85;text-shadow:0 1px 2px rgba(0,0,0,0.85),0 2px 8px rgba(0,0,0,0.6);">“</div>
                <p style="font-family:{};font-size:22px;font-style:italic;font-weight:600;color:white;margin:0 0 16px;line-height:1.4;max-width:380px;text-shadow:0 1px 2px rgba(0,0,0,0.9),0 2px 6px rgba(0,0,0,0.55),0 4px 16px rgba(0,0,0,0.4);">{}</p>
                {}
                {}
            </div>
        </div>"#,
        img_html,
        tokens.heading_font,
        escape_html(quote),
        if !author.is_empty() {
            format!(r#"<p style="font-family:{};font-size:12px;font-weight:800;color:white;margin:0;text-transform:uppercase;letter-spacing:0.08em;text-shadow:0 1px 2px rgba(0,0,0,0.85),0 2px 6px rgba(0,0,0,0.5);">{}</p>"#, tokens.body_font, escape_html(author))
        } else {
            String::new()
        },
        if !role.is_empty() {
            format!(r#"<p style="font-family:{};font-size:11px;color:rgba(255,255,255,0.95);margin:4px 0 0;text-shadow:0 1px 2px rgba(0,0,0,0.8),0 2px 6px rgba(0,0,0,0.45);">{}</p>"#, tokens.body_font, escape_html(role))
        } else {
            String::new()
        }
    );

    let html = slide_base_bleed(&content, tokens, bg_style, false, "0", "stretch");
    json!({
        "html": html,
        "background": bg_style,
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
    treatment.image_mask = "none".to_string();
    treatment.image_frame = "rounded".to_string();
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

    // ponytail: callout markers removed for clean image display; add back if numbered annotation is needed
    let markers = String::new();

    let desc_html = if !description.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:18px;font-weight:800;color:{};line-height:1.2;letter-spacing:-0.01em;margin:16px 0 0;">{}</div>"#,
            tokens.heading_font, colors.text_primary, escape_html(description)
        )
    } else {
        String::new()
    };

    let mut list_html = String::new();
    if !callouts.is_empty() {
        let mut items = String::new();
        for c in &callouts {
            let lbl = c.get("label").and_then(|v| v.as_str()).unwrap_or("");
            let d = c.get("description").and_then(|v| v.as_str()).unwrap_or("");
            if !lbl.is_empty() {
                items.push_str(&format!(
                    r#"<div style="display:flex;align-items:flex-start;gap:8px;font-family:{};font-size:12px;color:{};line-height:1.5;">
                        <span style="display:inline-block;width:6px;height:6px;min-width:6px;border-radius:50%;background:{};margin-top:5px;"></span>
                        <span><strong style="color:{};font-weight:700;">{}</strong>{}</span>
                    </div>"#,
                    tokens.body_font, colors.text_secondary,
                    colors.primary,
                    colors.text_primary, escape_html(lbl),
                    if !d.is_empty() { format!(" — {}", escape_html(d)) } else { String::new() }
                ));
            }
        }
        if !items.is_empty() {
            list_html = format!(
                r#"<div style="margin-top:12px;display:flex;flex-direction:column;gap:6px;width:100%;">{}</div>"#,
                items
            );
        }
    }

    let content = format!(
        r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;justify-content:center;align-items:flex-start;">
            <div style="position:relative;width:100%;height:230px;border-radius:var(--radius-md);overflow:hidden;box-shadow:0 6px 20px rgba(0,0,0,0.15);border:1px solid {}30;">
                {}
            </div>
            {}
            {}
        </div>"#,
        colors.border, img_html, desc_html, list_html
    );

    let padding_val = if padding.is_empty() {
        "16px var(--space-6) 20px"
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
        let (url, cap) = if let Some(s) = img.as_str() {
            (s, "")
        } else {
            let u = img.get("url").and_then(|v| v.as_str()).unwrap_or("");
            let c = img.get("caption").and_then(|v| v.as_str()).unwrap_or("");
            (u, c)
        };
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
            r#"<div style="position:relative;width:100%;height:100%;overflow:hidden;">
                {}
                {}
            </div>"#,
            img_html, caption_html
        ));
    }

    let has_header = !title.is_empty();
    let has_footer = !section_caption.is_empty();

    let eff_layout = if layout.is_empty() || layout == "auto" || layout == "2-grid" {
        if img_cards.len() >= 4 {
            "4-grid"
        } else if img_cards.len() == 3 {
            "3-grid"
        } else {
            "2-grid"
        }
    } else {
        layout
    };

    let grid_height = match eff_layout {
        "4-grid" | "6-grid" => if has_header && has_footer { "210px" } else if has_header || has_footer { "235px" } else { "275px" },
        "3-grid" => if has_header && has_footer { "190px" } else if has_header || has_footer { "220px" } else { "260px" },
        _ => if has_header && has_footer { "220px" } else if has_header || has_footer { "245px" } else { "280px" },
    };

    let grid_html = match eff_layout {
        "3-grid" => {
            let mut three_cards = Vec::new();
            for img in images.iter().take(3) {
                let url = img.get("url").and_then(|v| v.as_str()).unwrap_or("");
                let cap = img.get("caption").and_then(|v| v.as_str()).unwrap_or("");
                let img_html =
                    render_themed_image(url, tokens, &inner_treatment, "100%", "100%", cap, is_dark);
                let inner_cap = if !cap.is_empty() {
                    format!(
                        r#"<div style="padding:4px 0 0;font-family:{};font-size:9.5px;font-weight:700;color:{};letter-spacing:0.04em;text-transform:uppercase;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;">{}</div>"#,
                        tokens.body_font,
                        colors.text_secondary,
                        escape_html(cap)
                    )
                } else {
                    String::new()
                };
                three_cards.push(format!(
                    r#"<div style="display:flex;flex-direction:column;width:100%;">
                        <div style="position:relative;width:100%;height:{};overflow:hidden;flex-shrink:0;">
                            {}
                        </div>
                        {}
                    </div>"#,
                    grid_height, img_html, inner_cap
                ));
            }
            format!(
                r#"<div style="display:grid;grid-template-columns:repeat(3, minmax(0, 1fr));gap:8px;width:100%;">{}</div>"#,
                three_cards.join(" ")
            )
        }
        "4-grid" => {
            format!(
                r#"<div style="display:grid;grid-template-columns:1fr 1fr;grid-template-rows:1fr 1fr;gap:6px;height:{};width:100%;">
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
        }
        "6-grid" => {
            format!(
                r#"<div style="display:grid;grid-template-columns:repeat(3, 1fr);grid-template-rows:1fr 1fr;gap:6px;height:{};width:100%;">
                    {}
                </div>"#,
                grid_height,
                img_cards
                    .iter()
                    .take(6)
                    .cloned()
                    .collect::<Vec<String>>()
                    .join("")
            )
        }
        "featured-1-2" if img_cards.len() >= 3 => {
            format!(
                r#"<div style="display:grid;grid-template-columns:1.2fr 1fr;gap:6px;height:{};width:100%;">
                    {}
                    <div style="display:grid;grid-template-rows:1fr 1fr;gap:6px;height:100%;">
                        {}
                        {}
                    </div>
                </div>"#,
                grid_height, img_cards[0], img_cards[1], img_cards[2]
            )
        }
        "featured-2-1" if img_cards.len() >= 3 => {
            format!(
                r#"<div style="display:grid;grid-template-columns:1fr 1.2fr;gap:6px;height:{};width:100%;">
                    <div style="display:grid;grid-template-rows:1fr 1fr;gap:6px;height:100%;">
                        {}
                        {}
                    </div>
                    {}
                </div>"#,
                grid_height, img_cards[0], img_cards[1], img_cards[2]
            )
        }
        _ => {
            // 2-grid
            format!(
                r#"<div style="display:grid;grid-template-columns:1fr 1fr;gap:8px;height:{};width:100%;">
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
        }
    };

    let title_html = if !title.is_empty() {
        heading_block(title, tokens, "headline", Some(&colors.text_primary), false, None, "left", "0 0 10px", true)
    } else {
        String::new()
    };

    let caption_html = if !section_caption.is_empty() {
        format!(
            r#"<div style="font-family:{};font-size:11.5px;color:{};margin-top:10px;line-height:1.35;width:100%;">{}</div>"#,
            tokens.body_font,
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
        "16px var(--space-6) 20px"
    } else {
        padding
    };
    let html = slide_base(&content, tokens, bg_style, false, padding_val, "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": eff_layout,
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
                // Layout verified non-overlapping after rotation via bbox math:
                //   F1 bbox: x=6.7..157.3, y=8.3..143.7
                //   F2 bbox: x=168.1..309.9, y=5.6..120.4
                //   F3 bbox: x=14.5..139.5, y=155.9..246.1
                //   F4 bbox: x=162.3..313.7, y=147.4..252.6
                // Validator previously caught F1∩F4 overlap at the original hardcoded values
                // (F1 x=6..160 vs F4 x=154..310 → 6px x-overlap, larger after rotation).
                CollageSlot {
                    x: 10,
                    y: 12,
                    w: 144,
                    h: 128,
                    rot: -3,
                    z: 2,
                },
                CollageSlot {
                    x: 170,
                    y: 8,
                    w: 138,
                    h: 110,
                    rot: 2,
                    z: 3,
                },
                CollageSlot {
                    x: 16,
                    y: 158,
                    w: 122,
                    h: 86,
                    rot: 2,
                    z: 4,
                },
                CollageSlot {
                    x: 164,
                    y: 150,
                    w: 148,
                    h: 100,
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
        let url = if let Some(s) = img.as_str() {
            s
        } else {
            img.get("url").and_then(|v| v.as_str()).unwrap_or("")
        };
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
            {}
        </div>"#,
        collage_height_px,
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
        "16px var(--space-6) 20px"
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
    title: &str,
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

    let title_html = if !title.is_empty() {
        heading_block(title, tokens, "headline", Some(&colors.text_primary), false, None, "center", "0 0 8px", true)
    } else {
        String::new()
    };

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
        "background:rgba(0,0,0,0.7);backdrop-filter:blur(4px);-webkit-backdrop-filter:blur(4px);color:white;padding:4px 8px;font-family:{};font-size:9.5px;font-weight:700;border-radius:4px;position:absolute;top:10px;z-index:3;text-transform:uppercase;letter-spacing:0.06em;",
        tokens.body_font
    );

    let divider_html = if divider_style == "arrow" {
        format!(
            r#"<div style="position:absolute;left:50%;top:50%;transform:translate(-50%,-50%);width:32px;height:32px;border-radius:50%;background:{};color:white;display:flex;align-items:center;justify-content:center;font-size:var(--text-sm);z-index:4;box-shadow:0 4px 16px rgba(0,0,0,0.3);font-weight:bold;">
                ⇄
            </div>"#,
            colors.primary
        )
    } else {
        r#"<div style="position:absolute;left:50%;top:0;bottom:0;width:2px;background:white;z-index:4;box-shadow:0 0 8px rgba(0,0,0,0.4);transform:translateX(-50%);"></div>"#.to_string()
    };

    let desc_html = if !description.is_empty() {
        format!(
            r#"<p style="font-family:{};font-size:11px;color:{};margin:4px 0 0;line-height:1.4;text-align:center;width:100%;opacity:0.85;">{}</p>"#,
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
    let grid_height = if !title.is_empty() { "215px" } else { "240px" };
    let grid_wrapper_style = format!(
        "position:relative;width:100%;height:{};display:grid;grid-template-columns:1fr 1fr;gap:2px;border-radius:{};overflow:hidden;box-shadow:{};border: 1px solid {}30;background:{}20;",
        grid_height, radius_lg, shadow_md, colors.border, colors.border
    );

    let content = format!(
        r#"<div style="width:100%;height:100%;display:flex;flex-direction:column;justify-content:center;align-items:center;gap:6px;">
            {}
            <div style="{}">
                <div style="position:relative;width:100%;height:100%;">
                    {}
                    <span style="{}left:10px;">{}</span>
                </div>
                <div style="position:relative;width:100%;height:100%;">
                    {}
                    <span style="{}right:10px;">{}</span>
                </div>
                {}
            </div>
            {}
        </div>"#,
        title_html,
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
        "16px var(--space-6) 20px"
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
            "16px 50px 20px",
            "MyBrand",
            "https://example.com/logo.png",
            "Scan MyBrand QR Code",
        );
        let html_custom = res_custom["html"].as_str().unwrap();
        assert!(html_custom.contains("data:image/svg+xml;utf8,"));
        assert!(html_custom.contains("Scan this QR code"));
        assert!(!html_custom.contains("Some caption text about this conversion"));
        assert!(html_custom.contains("padding:16px 50px 20px;"));
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

    #[test]
    fn test_myth_fact_debunk_uses_title_scale_heading() {
        let tokens = derive_palette(
            "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
        ).unwrap();

        let res = myth_fact_slide(
            &tokens,
            "Breakfast is the most important meal of the day.",
            "Studies show no significant difference between breakfast eaters and skippers.",
            "The breakfast myth was popularized by cereal companies.",
            "light",
            "debunk",
            "editorial",
            "",
            0.4,
        );
        let html = res["html"].as_str().unwrap();
        let title_fs = tokens.type_scale.get("title").unwrap().font_size;
        let headline_fs = tokens.type_scale.get("headline").unwrap().font_size;
        assert!(
            html.contains(&format!("font-size: {}px", title_fs)),
            "debunk heading should use title scale ({}px), not headline scale ({}px)",
            title_fs, headline_fs
        );
    }

    #[test]
    fn test_grid_cards_removed_dispatch_errors() {
        // grid_cards was retired 2026-07-30. The dispatch must surface a
        // clear "removed" error rather than rendering an empty slide.
        let tokens = derive_palette(
            "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
        ).unwrap();
        let params = json!({
            "title": "Should not render",
            "cards": [
                {"icon": "🔍", "title": "Card 1", "description": "Card 1 description"},
            ]
        });
        let err = dispatch_slide("grid_cards", &tokens, &params, "light", "editorial", "educator")
            .expect_err("grid_cards must error after 2026-07-30 purge");
        assert!(
            err.to_string().contains("removed"),
            "error must mention 'removed': {}",
            err
        );
    }

    #[test]
    fn test_split_features_caps_tiles_at_three() {
        // split_features absorbed the grid_cards list-dense visual contract —
        // a multi-row list of icon+title+description beats. The banded body can
        // carry AT MOST three feature cards (a 4th tile overflows into the
        // chrome bands). The renderer caps rendered tiles at 3 and the validator
        // rejects configs with more than 3 features (see validate_slide_spec).
        let tokens = derive_palette(
            "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
        ).unwrap();

        let params = json!({
            "title": "Research Methodology",
            "variant": "list-dense",
            "features": [
                {"icon": "🔍", "title": "Literature Review", "description": "200+ papers analyzed"},
                {"icon": "📋", "title": "Survey Design", "description": "2400 participants"},
                {"icon": "🧪", "title": "Controlled Trials", "description": "Double-blind experiments"},
                {"icon": "📈", "title": "Statistical Modeling", "description": "Bayesian inference"},
                {"icon": "✅", "title": "Peer Review", "description": "External validation"},
            ]
        });
        let res = dispatch_slide("split_features", &tokens, &params, "light", "editorial", "educator")
            .expect("split_features should accept list-dense features");
        let html = res["html"].as_str().unwrap();
        // Only the first 3 tiles render; the 4th/5th are never emitted.
        assert!(html.contains("Literature Review"), "feature 1 title missing");
        assert!(html.contains("Survey Design"), "feature 2 title missing");
        assert!(html.contains("Controlled Trials"), "feature 3 title missing");
        assert!(!html.contains("Statistical Modeling"), "4th tile must NOT render");
        assert!(!html.contains("Peer Review"), "5th tile must NOT render");

        // The validator gate rejects >3 features as a hard error.
        let vr = crate::validate::validate_slide_spec("split_features", &params);
        assert!(!vr.valid, "config with 5 features must be rejected");
        assert!(vr.errors.iter().any(|e| e.contains("maximum of 3")));
    }

    #[test]
    fn test_myth_fact_no_glass_container_wrapper() {
        let tokens = derive_palette(
            "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
        ).unwrap();

        let res = myth_fact_slide(
            &tokens,
            "Skipping breakfast makes you gain weight.",
            "Meta-analyses show no direct causal link.",
            "Popularized by commercial cereal marketing.",
            "light",
            "debunk",
            "editorial",
            "",
            0.4,
        );
        let html = res["html"].as_str().unwrap();
        assert!(
            !html.contains("backdrop-filter:blur"),
            "myth_fact slide should not use glass container blur wrapper"
        );
    }

    #[test]
    fn test_metric_grid_progress_bars_data_driven() {
        let tokens = derive_palette(
            "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
        )
        .unwrap();
        // Tile 1 uses current/total, tile 2 uses progress fraction, tile 3 uses
        // progress percent, tile 4 has NO explicit progress config (no bar).
        let metrics = json!([
            {"value": "12/50", "label": "Compiled", "current": 12, "total": 50},
            {"value": "3/4", "label": "Adopted", "progress": 0.25},
            {"value": "88", "label": "Speed", "progress": "88%"},
            {"value": "47", "label": "Types"}
        ]);
        let res = metric_grid_slide(
            &tokens,
            metrics.as_array().unwrap().clone(),
            "Pipeline",
            "dark",
            "editorial",
            "",
            0.4,
        );
        let html = res["html"].as_str().unwrap();
        // The static 75% fill must be gone.
        assert!(
            !html.contains("width:75%;height:100%;background"),
            "metric_grid must not use the old static 75% bar fill"
        );
        // current/total 12/50 = 24%
        assert!(html.contains("width:24%;height:100%;background"), "expected 24% fill from current/total");
        // progress 0.25 = 25%
        assert!(html.contains("width:25%;height:100%;background"), "expected 25% fill from progress fraction");
        // progress "88%" = 88%
        assert!(html.contains("width:88%;height:100%;background"), "expected 88% fill from progress percent");
        // A numeric value alone must NOT imply a bar (no abstract fallback).
        assert!(
            !html.contains("width:47%;height:100%;background"),
            "numeric value must not become an abstract bar fill"
        );
        // The redundant percentage counter is gone from the frontend.
        assert!(
            !html.contains(">47%<")
                && !html.contains("font-size:9.5px;font-weight:800;color:"),
            "the % counter next to the value must be removed"
        );
    }

    #[test]
    fn test_quote_mark_uses_contrast_safe_primary() {
        // The decorative mark must use the contrast-safe `colors.primary`
        // (solid, >=4.5:1 on light / >=5.5:1 on dark) instead of a faint
        // alpha-suffixed tint of `tokens.primary` that disappears on glass.
        let tokens = derive_palette(
            "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
        )
        .unwrap();
        let res = quote_slide(
            &tokens,
            "The best interface is the one you forget.",
            "Ada",
            "Design Lead",
            "light",
            "default",
            "editorial",
            "",
            0.0,
        );
        let html = res["html"].as_str().unwrap();
        assert!(html.contains('\u{275d}'), "quote mark must render");
        // Extract the color declaration that precedes the mark glyph.
        let mark_color = html
            .find('\u{275d}')
            .and_then(|i| html[..i].rfind("color:"))
            .map(|c| &html[c..html.find('\u{275d}').unwrap()])
            .unwrap_or("");
        assert!(
            !mark_color.contains("#0066FF"),
            "mark must not use raw tokens.primary: {}",
            mark_color
        );
        assert!(
            mark_color.trim_end_matches('"').len() >= 15,
            "mark color must be a full hex (no alpha suffix), got: {}",
            mark_color
        );
    }

    #[test]
    fn test_image_headline_scrim_tracks_text_position() {
        // The overlay scrim must be position-aware so bright images never wash
        // out white text: top -> top-heavy gradient, center -> radial scrim,
        // bottom -> bottom-heavy gradient (with a raised mid-scrim).
        let tokens = derive_palette(
            "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
        )
        .unwrap();
        let url = "https://example.com/photo.jpg";
        for (pos, expect_frag) in [
            ("top", "linear-gradient(to bottom, rgba(0,0,0,0.55), rgba(0,0,0,0.28) 55%, rgba(0,0,0,0.12))"),
            ("center", "radial-gradient(ellipse 78% 62% at 50% 50%, rgba(0,0,0,0.55), rgba(0,0,0,0.20) 72%, rgba(0,0,0,0.10))"),
            ("bottom", "linear-gradient(to bottom, rgba(0,0,0,0.12), rgba(0,0,0,0.30) 55%, rgba(0,0,0,0.72))"),
        ] {
            let res = image_headline_slide(
                &tokens, url, "Built for carousels", "Sub line", pos, "dark", "", 0.0,
                "editorial", "", "",
            );
            let html = res["html"].as_str().unwrap();
            assert!(
                html.contains(expect_frag),
                "{}: expected position-aware scrim {}",
                pos,
                expect_frag
            );
            // Layered text-shadow on the headline (crisp edge + halo).
            assert!(
                html.contains("0 1px 2px rgba(0,0,0,0.85),0 2px 6px"),
                "{}: headline must use layered text-shadow",
                pos
            );
        }
    }

    #[test]
    fn test_image_quote_center_scrim_and_shadows() {
        // Centered multi-line quote -> center-weighted radial scrim + layered
        // shadows on the quote and attribution.
        let tokens = derive_palette(
            "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
        )
        .unwrap();
        let res = image_quote_slide(
            &tokens,
            "https://example.com/photo.jpg",
            "Composition is a constraint problem.",
            "System",
            "Design",
            "dark",
            "",
            0.0,
            "editorial",
            "",
            "",
        );
        let html = res["html"].as_str().unwrap();
        assert!(
            html.contains("radial-gradient(ellipse 78% 62% at 50% 50%, rgba(0,0,0,0.55)"),
            "image_quote must use center-weighted radial scrim"
        );
        assert!(
            html.contains("0 1px 2px rgba(0,0,0,0.9),0 2px 6px rgba(0,0,0,0.55),0 4px 16px"),
            "quote text must use layered text-shadow"
        );
    }

    #[test]
    fn test_pricing_plan_three_tiles_center_last() {
        let tokens = derive_palette(
            "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
        )
        .unwrap();
        let plans = json!([
            {"name": "CLI", "price": "Free", "features": ["A", "B"]},
            {"name": "Pro", "price": "$29/mo", "features": ["A", "B"], "featured": true},
            {"name": "Team", "price": "$99/mo", "features": ["A", "B"]}
        ]);
        let res = pricing_plan_slide(
            &tokens, "Plans", plans.as_array().unwrap().clone(), "dark", "editorial", "", 0.4,
        );
        let html = res["html"].as_str().unwrap();
        // The 3rd tile must be wrapped in a centered grid-span container.
        assert!(
            html.contains("grid-column:1 / -1") && html.contains("justify-content:center"),
            "3-plan pricing must center the 3rd tile (found centered span wrapper)"
        );
    }

    #[test]
    fn test_pricing_plan_supports_two_and_four_tiles() {
        let tokens = derive_palette(
            "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
        )
        .unwrap();
        let plans2 = json!([
            {"name": "CLI", "price": "Free", "features": ["A"]},
            {"name": "Pro", "price": "$29/mo", "features": ["A"]}
        ]);
        let res2 = pricing_plan_slide(
            &tokens, "Plans", plans2.as_array().unwrap().clone(), "dark", "editorial", "", 0.4,
        );
        let html2 = res2["html"].as_str().unwrap();
        // 2 plans → plain 2-column grid, NO centered-span wrapper.
        assert!(!html2.contains("grid-column:1 / -1"), "2-plan grid must not center-wrap");
        assert_eq!(html2.matches("Get Started").count() + html2.matches("Upgrade Now").count(), 2);

        let plans4 = json!([
            {"name": "A", "price": "1", "features": ["a"]},
            {"name": "B", "price": "2", "features": ["a"]},
            {"name": "C", "price": "3", "features": ["a"]},
            {"name": "D", "price": "4", "features": ["a"]}
        ]);
        let res4 = pricing_plan_slide(
            &tokens, "Plans", plans4.as_array().unwrap().clone(), "dark", "editorial", "", 0.4,
        );
        let html4 = res4["html"].as_str().unwrap();
        // 4 plans → balanced 2×2 grid, no centered-span wrapper.
        assert!(!html4.contains("grid-column:1 / -1"), "4-plan grid must not center-wrap");
        assert_eq!(html4.matches("Get Started").count() + html4.matches("Upgrade Now").count(), 4);
    }

    #[test]
    fn test_timeline_tiles_read_label_and_use_badge_composition() {
        let tokens = derive_palette(
            "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
        ).unwrap();
        let steps = json!([
            {"label": "v1", "description": "Initial release"},
            {"label": "v2", "description": "Pool composition"},
            {"label": "v3", "description": "Validation suite"},
            {"label": "v4", "description": "27 presets + audit"}
        ]);
        let res = timeline_slide(
            &tokens,
            "Release history",
            steps.as_array().unwrap().clone(),
            "dark",
            "vertical",
            "editorial",
            "",
            0.4,
        );
        let html = res["html"].as_str().unwrap();
        // Step labels (label key, not title) must render as tile headings.
        assert!(html.contains("v1"), "timeline must read step `label` into the title");
        assert!(html.contains("v4"), "timeline must render all step labels");
        // process_map-style circular number badge (32px circle + number).
        assert!(html.contains("border-radius:50%"), "tile must use a circular number badge");
        assert!(html.contains("01"), "badge must show a zero-padded step number");
        // Old tiny PHASE text chip must be gone.
        assert!(!html.contains("PHASE 0"), "old PHASE text chip removed");
        // Type hierarchy: title (800 weight) is visually distinct from desc.
        assert!(html.contains("font-weight:800"), "tile title carries bold weight");
    }
}




