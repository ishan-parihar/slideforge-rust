use crate::archetypes;
use crate::components;
use crate::design_system;
use crate::export;
use crate::platforms;
use crate::slide_registry;
use crate::slides::{self, CarouselSpec, SlideSpec};
use crate::validate;
use indexmap::IndexMap;
use rmcp::{
    ErrorData, ServerHandler,
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::{Json, Parameters},
    model::ServerInfo,
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// ── Session state ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct ServerState {
    pub primary_color: String,
    pub font_style: String,
    pub type_scale_base: i32,
    pub type_scale_ratio: f32,
    pub preset: String,
    pub css_variables: String,
    pub google_fonts_url: String,
    pub heading_font: String,
    pub body_font: String,
    pub brand_name: String,
    pub brand_handle: String,
    pub visual_theme: String,
    pub layout_theme: String,
    pub effect_theme: String,
    pub topic: String,
    pub url: String,
    pub hashtags: Vec<String>,
    pub show_progress: bool,
    pub archetype: String,
    pub platform: String,
    pub aspect_ratio: String,
    pub bg_style: String,
    pub validated: bool,
}

fn platform_context_json(platform: &str, aspect_ratio: &str) -> serde_json::Value {
    match platforms::resolve_canvas(platform, Some(aspect_ratio)) {
        Ok(canvas) => serde_json::json!({
            "platform": canvas.platform,
            "aspect_ratio": canvas.aspect_ratio,
            "width": canvas.width,
            "height": canvas.height,
            "format": canvas.format,
            "agent_guidance": "Use qr_destination for off-platform actions because Instagram, Facebook, TikTok, and LinkedIn carousel images do not support clickable slide areas."
        }),
        Err(_) => serde_json::json!({
            "platform": platform,
            "aspect_ratio": aspect_ratio,
            "agent_guidance": "Resolve platform context with list_platforms before generating slides."
        }),
    }
}

// ── Request / Response types ──────────────────────────────────────────────────

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ConfigureDesignRequest {
    pub primary_color: String,
    pub font_style: Option<String>,
    pub type_scale_base: Option<i32>,
    pub type_scale_ratio: Option<f32>,
    pub preset: Option<String>,
    pub brand_name: Option<String>,
    pub brand_handle: Option<String>,
    pub visual_theme: Option<String>,
    pub layout_theme: Option<String>,
    pub effect_theme: Option<String>,
    pub topic: Option<String>,
    pub url: Option<String>,
    pub hashtags: Option<Vec<String>>,
    pub show_progress: Option<bool>,
    pub archetype: Option<String>,
    pub platform: Option<String>,
    pub aspect_ratio: Option<String>,
    pub bg_style: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ConfigureDesignResponse {
    pub status: String,
    pub primary_color: String,
    pub font_style: String,
    pub preset: String,
    pub visual_theme: String,
    pub layout_theme: String,
    pub effect_theme: String,
    pub topic: String,
    pub archetype: String,
    pub platform: String,
    pub aspect_ratio: String,
    pub contrast_report: IndexMap<String, design_system::ContrastReportItem>,
    pub message: String,
    pub platform_context: serde_json::Value,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DesignSystemRequest {
    pub primary_color: String,
    pub font_style: Option<String>,
    pub type_scale_base: Option<i32>,
    pub type_scale_ratio: Option<f32>,
    pub overrides: Option<IndexMap<String, String>>,
    pub canvas_width: Option<u32>,
    pub canvas_height: Option<u32>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GenerateSlideRequest {
    /// Slide type: hero, feature, list, quote, cta, comparison, stat_row, timeline,
    /// callout, split_features, grid_cards, headline_subheadline, definition, text_block
    pub slide_type: String,
    pub primary_color: Option<String>,
    pub font_style: Option<String>,
    pub preset: Option<String>,
    pub bg_style: Option<String>,
    pub theme: Option<String>,
    pub archetype: Option<String>,
    pub platform: Option<String>,
    pub aspect_ratio: Option<String>,
    /// Slide-type-specific parameters (headline, items, stats, etc.)
    pub params: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RenderCarouselRequest {
    pub slides: Vec<SlideSpec>,
    pub css_variables: Option<String>,
    pub google_fonts_url: Option<String>,
    pub heading_font: Option<String>,
    pub body_font: Option<String>,
    pub brand_name: Option<String>,
    pub brand_handle: Option<String>,
    pub include_ig_frame: Option<bool>,
    pub output_path: Option<String>,
    pub topic: Option<String>,
    pub url: Option<String>,
    pub hashtags: Option<Vec<String>>,
    pub show_progress: Option<bool>,
    pub platform: Option<String>,
    pub aspect_ratio: Option<String>,
}
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SkillGuideResponse {
    pub content: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RenderCarouselResponse {
    pub html: String,
    pub output_path: Option<String>,
    pub total_slides: usize,
    pub platform_context: serde_json::Value,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ExportCarouselSlidesRequest {
    pub html_path: String,
    pub output_dir: String,
    pub total_slides: usize,
    /// Platform name (e.g. "instagram_portrait", "tiktok_vertical"). Call
    /// list_platforms to see all valid values. Note: this field was
    /// previously named "preset" which was misleading — it has always been
    /// used as the platform name.
    pub platform: Option<String>,
    pub aspect_ratio: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ExportResponse {
    pub exported_slides: Vec<String>,
    pub dimensions: String,
    pub preset: String,
    pub total_exported: usize,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ValidateLayoutRequest {
    pub slide_type: String,
    pub params: serde_json::Value,
    pub html: Option<String>,
    pub aspect_ratio: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ValidateLayoutResponse {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ValidateAndFixRequest {
    pub slide_type: String,
    pub params: serde_json::Value,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ValidateAndFixResponse {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub fixes: Vec<String>,
    pub params: serde_json::Value,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ValidateDesignRequest {
    pub html: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RecommendColorSchemeRequest {
    pub primary_color: String,
    pub style: Option<String>,
    pub num_schemes: Option<u8>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct EmbedLocalImageRequest {
    /// Absolute or relative path to the local image file. Supported: PNG, JPEG,
    /// GIF, WebP, SVG.
    pub file_path: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PreviewSlideRequest {
    /// The slide HTML (from generate_slide's response `html` field).
    pub html: String,
    /// Where to save the preview PNG. Defaults to /tmp/slideforge-preview.png.
    pub output_path: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SlideTypeContextRequest {
    pub context: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SlideTypeInfoRequest {
    pub slide_type: String,
}

// ── RawJson: rmcp-safe wrapper for serde_json::Value ─────────────────────────

/// Wrapper around `serde_json::Value` that implements `JsonSchema` with a generic
/// object schema. rmcp requires all tool output schemas to have `type: "object"` at
/// the root — bare `serde_json::Value` produces a permissive schema without a type
/// field, which causes initialization panics.
#[derive(Debug, Clone)]
pub struct RawJson(pub serde_json::Value);

impl Serialize for RawJson {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RawJson {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        serde_json::Value::deserialize(deserializer).map(RawJson)
    }
}

impl JsonSchema for RawJson {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "RawJson".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        "RawJson".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        serde_json::from_value(serde_json::json!({
            "type": "object",
            "additionalProperties": true
        }))
        .unwrap()
    }
}

// ── Server ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Server {
    pub tool_router: ToolRouter<Self>,
    pub state: Arc<Mutex<ServerState>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            state: Arc::new(Mutex::new(ServerState::default())),
        }
    }

    fn get_tokens(&self) -> Result<design_system::DesignTokens, ErrorData> {
        let state = self.state.lock().unwrap();
        if state.primary_color.is_empty() {
            return Err(ErrorData::invalid_request(
                "No primary_color configured. Call configure_design first.",
                None,
            ));
        }
        let style = if state.font_style.is_empty() {
            "modern".to_string()
        } else {
            state.font_style.clone()
        };
        let preset = if state.preset.is_empty() {
            "tonal_spot".to_string()
        } else {
            state.preset.clone()
        };
        let base = state.type_scale_base.max(12);
        let ratio = state.type_scale_ratio;
        let visual_theme = state.visual_theme.clone();
        let primary = state.primary_color.clone();
        let platform = state.platform.clone();
        let aspect_ratio = state.aspect_ratio.clone();
        drop(state);

        let render = platforms::resolve_render_canvas(&platform, Some(&aspect_ratio))
            .unwrap_or_else(|_| platforms::RenderCanvas {
                platform: platform.clone(),
                base_width: 420,
                base_height: 525,
                target_width: 1080,
                target_height: 1350,
                aspect_ratio: aspect_ratio.clone(),
                format: "portrait".to_string(),
                scale_factor: 1080.0 / 420.0,
            });

        design_system::derive_palette_with_canvas(
            &primary,
            &style,
            base,
            ratio,
            &preset,
            &visual_theme,
            None,
            None,
            None,
            render.base_width,
            render.base_height,
        )
        .map_err(|e| ErrorData::internal_error(e, None))
    }
}

#[tool_router(router = tool_router)]
impl Server {
    /// Load the SlideForge design guide and skill documentation. Returns the
    /// full DESIGN-GUIDE.md content which includes slide type catalogs, design
    /// principles, example workflows, and best practices for AI agents.
    #[tool(
        name = "load_carousel_skill",
        description = "Load the SlideForge design guide and skill documentation. Returns the full DESIGN-GUIDE.md content which includes slide type catalogs, design principles, example workflows, and best practices for AI agents creating carousels."
    )]
    pub async fn load_carousel_skill(&self) -> Result<Json<SkillGuideResponse>, ErrorData> {
        Ok(Json(SkillGuideResponse {
            content: include_str!("../DESIGN-GUIDE.md").to_string(),
        }))
    }

    // ── configure_design ──────────────────────────────────────────────────────

    /// Configure design tokens and session brand settings. Call this first.
    #[tool(
        name = "configure_design",
        description = "Configure design tokens and session brand settings. Must be called before generating slides."
    )]
    pub async fn configure_design(
        &self,
        Parameters(req): Parameters<ConfigureDesignRequest>,
    ) -> Result<Json<ConfigureDesignResponse>, ErrorData> {
        let visual_theme = req
            .visual_theme
            .clone()
            .unwrap_or_else(|| "editorial".to_string());
        // Validate visual_theme against allowed values
        let valid_themes = ["editorial", "bold", "minimal", "dark", "vibrant", "natural"];
        if !valid_themes.contains(&visual_theme.as_str()) {
            let msg = format!(
                "Invalid visual_theme '{}'. Valid values: {}",
                visual_theme,
                valid_themes.join(", ")
            );
            return Err(ErrorData::invalid_request(msg, None));
        }
        let style = match visual_theme.as_str() {
            "editorial" => "editorial",
            "bold" => "bold",
            "minimal" => "modern",
            "dark" => "technical",
            "vibrant" => "rounded",
            "natural" => "warm",
            _ => req.font_style.as_deref().unwrap_or("modern"),
        }
        .to_string();
        let preset = req
            .preset
            .clone()
            .unwrap_or_else(|| "tonal_spot".to_string());
        // Validate preset against allowed values
        let valid_presets = [
            "tonal_spot",
            "vibrant",
            "neutral",
            "monochrome",
            "expressive",
            "fidelity",
            "rainbow",
            "fruit_salad",
            "content",
        ];
        if !valid_presets.contains(&preset.as_str()) {
            let msg = format!(
                "Invalid preset '{}'. Valid values: {}",
                preset,
                valid_presets.join(", ")
            );
            return Err(ErrorData::invalid_request(msg, None));
        }
        let type_scale_base = req.type_scale_base.unwrap_or(16);
        let type_scale_ratio = req.type_scale_ratio.unwrap_or(1.25);

        // Resolve canvas for scaling
        let platform = req
            .platform
            .clone()
            .unwrap_or_else(|| "instagram_portrait".to_string());
        let canvas = platforms::resolve_canvas(&platform, req.aspect_ratio.as_deref())
            .map_err(|e| ErrorData::invalid_request(e, None))?;
        // Token generation always uses 4:5 composition (420×525)
        let base_width = 420;
        let base_height = 525;

        let tokens = design_system::derive_palette_with_canvas(
            &req.primary_color,
            &style,
            type_scale_base,
            type_scale_ratio,
            &preset,
            &visual_theme,
            None,
            None,
            None,
            base_width,
            base_height,
        )
        .map_err(|e| ErrorData::internal_error(e, None))?;

        let mut state = self.state.lock().unwrap();
        state.primary_color = req.primary_color.clone();
        state.font_style = style.clone();
        state.type_scale_base = type_scale_base;
        state.type_scale_ratio = type_scale_ratio;
        state.preset = preset.clone();
        state.css_variables = tokens.to_css_variables();
        state.google_fonts_url = tokens.google_fonts_url.clone();
        state.heading_font = tokens.heading_font.clone();
        state.body_font = tokens.body_font.clone();
        state.brand_name = req.brand_name.clone().unwrap_or_default();
        state.brand_handle = req.brand_handle.clone().unwrap_or_default();
        state.visual_theme = visual_theme.clone();
        state.layout_theme = req
            .layout_theme
            .clone()
            .unwrap_or_else(|| "asymmetric".to_string());
        state.effect_theme = req
            .effect_theme
            .clone()
            .unwrap_or_else(|| "glass".to_string());
        state.topic = req.topic.clone().unwrap_or_default();
        state.url = req.url.clone().unwrap_or_default();
        state.hashtags = req.hashtags.clone().unwrap_or_default();
        state.show_progress = req.show_progress.unwrap_or(true);
        state.archetype = req.archetype.clone().unwrap_or_default();
        state.platform = canvas.platform.clone();
        state.aspect_ratio = canvas.aspect_ratio.clone();
        state.bg_style = req.bg_style.clone().unwrap_or_else(|| "light".to_string());
        state.validated = true;

        let layout_theme = state.layout_theme.clone();
        let effect_theme = state.effect_theme.clone();
        let topic = state.topic.clone();
        let archetype = state.archetype.clone();
        let platform = state.platform.clone();
        let aspect_ratio = state.aspect_ratio.clone();

        Ok(Json(ConfigureDesignResponse {
            status: "configured".to_string(),
            primary_color: req.primary_color,
            font_style: style,
            preset,
            visual_theme,
            layout_theme,
            effect_theme,
            topic,
            archetype,
            platform: platform.clone(),
            aspect_ratio: aspect_ratio.clone(),
            contrast_report: tokens.contrast_report,
            message: "Design system configured. All subsequent calls will use this configuration."
                .to_string(),
            platform_context: platform_context_json(&platform, &aspect_ratio),
        }))
    }

    // ── design_system ─────────────────────────────────────────────────────────

    /// Derive a full design token set from a brand color (one-shot, stateless).
    #[tool(
        name = "design_system",
        description = "Derive design tokens (palette, typography, spacing, effects) from a brand color. Stateless; does not update session."
    )]
    pub async fn design_system(
        &self,
        Parameters(req): Parameters<DesignSystemRequest>,
    ) -> Result<Json<design_system::DesignTokens>, ErrorData> {
        let style = req
            .font_style
            .clone()
            .unwrap_or_else(|| "modern".to_string());
        let base = req.type_scale_base.unwrap_or(16);
        let ratio = req.type_scale_ratio.unwrap_or(1.25);

        // Use 420 base canvas for vectoric scaling
        let canvas_width = req.canvas_width.unwrap_or(420);
        let canvas_height = req.canvas_height.unwrap_or(525);

        let tokens = design_system::derive_palette_with_canvas(
            &req.primary_color,
            &style,
            base,
            ratio,
            "tonal_spot",
            "",
            req.overrides.as_ref(),
            None,
            None,
            canvas_width,
            canvas_height,
        )
        .map_err(|e| ErrorData::internal_error(e, None))?;

        Ok(Json(tokens))
    }

    // ── generate_slide ────────────────────────────────────────────────────────

    /// Generate HTML for a single slide using the configured session design.
    #[tool(
        name = "generate_slide",
        description = "Generate HTML for a single slide. Supports 47 slide types across 6 categories: (1) Text & Layout: hero, feature, list, quote, cta, comparison, stat_row, timeline, callout, split_features, grid_cards, headline_subheadline, definition, text_block, section_divider, text_columns. (2) Data Viz: chart, scatter_plot, gauge, radar_chart, column_chart, table, metric_sparkline, funnel_chart, metric_grid, comparison_bars, progress_rings. (3) Metrics: metric_card, stat_row. (4) Story: problem_solution, myth_fact, case_study_result, testimonial_avatar, before_after_story, logo_cloud, pricing_plan, checklist_action_plan, faq, process_map. (5) Image: image_caption, image_headline, image_quote, image_callout, image_stat, image_gallery, image_collage, image_comparison. (6) Conversion: qr_destination. Call list_slide_types for full details (required params, optional params, variants) and get_slide_type_info for a specific type's schema. Image URLs must be supplied by the caller in the image_url or background_image param — use embed_local_image to convert a local file to a data URI."
    )]
    pub async fn generate_slide(
        &self,
        Parameters(req): Parameters<GenerateSlideRequest>,
    ) -> Result<Json<RawJson>, ErrorData> {
        let state = self.state.lock().unwrap();

        let primary = req
            .primary_color
            .as_deref()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .unwrap_or_else(|| state.primary_color.clone());

        if primary.is_empty() {
            return Err(ErrorData::invalid_request(
                "No primary_color. Call configure_design first or pass primary_color.",
                None,
            ));
        }

        let theme = req
            .theme
            .clone()
            .or_else(|| Some(state.visual_theme.clone()))
            .unwrap_or_else(|| "editorial".to_string());
        let theme = if theme.is_empty() {
            "editorial".to_string()
        } else {
            theme
        };

        let style = match theme.as_str() {
            "editorial" => "editorial",
            "bold" => "bold",
            "minimal" => "modern",
            "dark" => "technical",
            "vibrant" => "rounded",
            "natural" => "warm",
            _ => req
                .font_style
                .as_deref()
                .unwrap_or(if state.font_style.is_empty() {
                    "modern"
                } else {
                    &state.font_style
                }),
        }
        .to_string();
        let preset = req
            .preset
            .clone()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| {
                if state.preset.is_empty() {
                    "tonal_spot".to_string()
                } else {
                    state.preset.clone()
                }
            });

        let bg_style = req
            .bg_style
            .clone()
            .or_else(|| Some(state.bg_style.clone()))
            .unwrap_or_else(|| "light".to_string());
        let bg_style = if bg_style.is_empty() {
            "light".to_string()
        } else {
            bg_style
        };
        // Validate bg_style against allowed values
        let valid_bg_styles = ["light", "dark", "gradient", "mesh", "hero"];
        if !valid_bg_styles.contains(&bg_style.as_str()) {
            let msg = format!(
                "Invalid bg_style '{}'. Valid values: {}",
                bg_style,
                valid_bg_styles.join(", ")
            );
            return Err(ErrorData::invalid_request(msg, None));
        }

        let archetype = req
            .archetype
            .clone()
            .or_else(|| Some(state.archetype.clone()))
            .unwrap_or_default();

        let base = state.type_scale_base.max(12);
        let ratio = state.type_scale_ratio;

        // Resolve canvas for scaling
        let platform = req
            .platform
            .clone()
            .or_else(|| Some(state.platform.clone()))
            .unwrap_or_else(|| state.platform.clone());
        let platform = if platform.is_empty() {
            "instagram_portrait".to_string()
        } else {
            platform
        };

        let aspect_ratio = req
            .aspect_ratio
            .clone()
            .or_else(|| Some(state.aspect_ratio.clone()))
            .filter(|s| !s.is_empty());
        let render = platforms::resolve_render_canvas(&platform, aspect_ratio.as_deref())
            .map_err(|e| ErrorData::invalid_request(e, None))?;

        drop(state);

        let tokens = design_system::derive_palette_with_canvas(
            &primary,
            &style,
            base,
            ratio,
            &preset,
            &theme,
            None,
            None,
            None,
            render.base_width,
            render.base_height,
        )
        .map_err(|e| ErrorData::internal_error(e, None))?;

        let params = req.params.clone().unwrap_or(serde_json::json!({}));
        let slide_type = req.slide_type.to_lowercase().replace('-', "_");

        // Pre-flight validation — block if required params are missing
        let validation = validate::validate_slide_spec(&slide_type, &params);

        if !validation.errors.is_empty() {
            return Err(ErrorData::invalid_request(
                format!(
                    "Missing required params for '{}': {}. Call get_slide_type_info for schema.",
                    slide_type,
                    validation.errors.join("; ")
                ),
                None,
            ));
        }

        let warnings = if !validation.warnings.is_empty() {
            Some(serde_json::json!({
                "warnings": validation.warnings,
            }))
        } else {
            None
        };

        let result = components::dispatch_slide(
            &slide_type,
            &tokens,
            &params,
            &bg_style,
            &theme,
            &archetype,
        )
        .map_err(|e| ErrorData::invalid_request(e, None))?;

        // Enrich the response with slide_type echo + validation warnings
        let mut enriched = result;
        if let Some(obj) = enriched.as_object_mut() {
            obj.insert("slide_type".to_string(), serde_json::json!(slide_type));
            if let Some(w) = &warnings {
                obj.insert("validation".to_string(), w.clone());
            }
        }

        Ok(Json(RawJson(enriched)))
    }

    // ── render_carousel ───────────────────────────────────────────────────────

    /// Assemble slide HTML into a full carousel HTML document.
    #[tool(
        name = "render_carousel",
        description = "Assemble slide HTML objects into a full carousel HTML document with CSS, fonts, and brand footer."
    )]
    pub async fn render_carousel(
        &self,
        Parameters(req): Parameters<RenderCarouselRequest>,
    ) -> Result<Json<RenderCarouselResponse>, ErrorData> {
        let state = self.state.lock().unwrap();

        let css_vars = req
            .css_variables
            .clone()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| state.css_variables.clone());

        if css_vars.is_empty() {
            return Err(ErrorData::invalid_request(
                "No css_variables. Call configure_design first or pass css_variables.",
                None,
            ));
        }

        let platform = req
            .platform
            .clone()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| state.platform.clone());
        let platform = if platform.is_empty() {
            "instagram_portrait".to_string()
        } else {
            platform
        };

        let aspect_ratio = req
            .aspect_ratio
            .clone()
            .filter(|s| !s.is_empty())
            .or_else(|| Some(state.aspect_ratio.clone()))
            .filter(|s| !s.is_empty());
        let canvas = platforms::resolve_canvas(&platform, aspect_ratio.as_deref())
            .map_err(|e| ErrorData::invalid_request(e, None))?;

        let spec = CarouselSpec {
            slides: req.slides,
            css_variables: css_vars,
            google_fonts_url: req
                .google_fonts_url
                .clone()
                .unwrap_or_else(|| state.google_fonts_url.clone()),
            heading_font: req
                .heading_font
                .clone()
                .unwrap_or_else(|| state.heading_font.clone()),
            body_font: req
                .body_font
                .clone()
                .unwrap_or_else(|| state.body_font.clone()),
            brand_name: req
                .brand_name
                .clone()
                .unwrap_or_else(|| state.brand_name.clone()),
            brand_handle: req
                .brand_handle
                .clone()
                .unwrap_or_else(|| state.brand_handle.clone()),
            topic: req.topic.clone().unwrap_or_else(|| state.topic.clone()),
            url: req.url.clone().unwrap_or_else(|| state.url.clone()),
            hashtags: req
                .hashtags
                .clone()
                .unwrap_or_else(|| state.hashtags.clone()),
            show_progress: req.show_progress.unwrap_or(state.show_progress),
            visual_theme: state.visual_theme.clone(),
            include_ig_frame: req.include_ig_frame.unwrap_or(true),
            platform: canvas.platform.clone(),
            aspect_ratio: canvas.aspect_ratio.clone(),
            canvas_width: canvas.width,
            canvas_height: canvas.height,
        };
        drop(state);

        let html = slides::render_carousel_html(&spec);
        let total = spec.slides.len();
        let output_path = req.output_path.clone();

        if let Some(ref path) = output_path {
            std::fs::write(path, &html).map_err(|e| {
                ErrorData::internal_error(format!("Failed to write HTML: {}", e), None)
            })?;
        }

        Ok(Json(RenderCarouselResponse {
            html,
            output_path,
            total_slides: total,
            platform_context: platform_context_json(&spec.platform, &spec.aspect_ratio),
        }))
    }

    // ── export_carousel_slides ────────────────────────────────────────────────

    /// Export carousel HTML to per-slide PNG images via headless Chrome.
    #[tool(
        name = "export_carousel_slides",
        description = "Render carousel HTML to PNG images using headless Chrome. Returns file paths to each exported slide."
    )]
    pub async fn export_carousel_slides(
        &self,
        Parameters(req): Parameters<ExportCarouselSlidesRequest>,
    ) -> Result<Json<ExportResponse>, ErrorData> {
        let (state_platform, state_aspect_ratio) = {
            let state = self.state.lock().unwrap();
            (state.platform.clone(), state.aspect_ratio.clone())
        };

        let platform = req
            .platform
            .clone()
            .filter(|s| !s.is_empty())
            .unwrap_or(state_platform);
        let platform = if platform.is_empty() {
            "instagram_portrait".to_string()
        } else {
            platform
        };

        let aspect_ratio = req
            .aspect_ratio
            .clone()
            .filter(|s| !s.is_empty())
            .or_else(|| Some(state_aspect_ratio))
            .filter(|s| !s.is_empty());
        let canvas = platforms::resolve_canvas(&platform, aspect_ratio.as_deref())
            .map_err(|e| ErrorData::invalid_request(e, None))?;

        let paths = export::export_slides(
            &req.html_path,
            &req.output_dir,
            req.total_slides,
            canvas.width,
            canvas.height,
        )
        .await
        .map_err(|e| ErrorData::internal_error(e, None))?;

        let total = paths.len();
        let dimensions = format!("{}×{}", canvas.width, canvas.height);
        Ok(Json(ExportResponse {
            exported_slides: paths,
            dimensions,
            preset: canvas.platform,
            total_exported: total,
        }))
    }

    // ── list_slide_types ──────────────────────────────────────────────────────

    /// List all available slide types with metadata.
    #[tool(
        name = "list_slide_types",
        description = "List all available slide types with descriptions, required params, optional params, and variants."
    )]
    pub async fn list_slide_types(&self) -> Result<Json<RawJson>, ErrorData> {
        Ok(Json(RawJson(slide_registry::get_registry())))
    }

    // ── get_slide_type_info ───────────────────────────────────────────────────

    /// Get detailed info for a specific slide type.
    #[tool(
        name = "get_slide_type_info",
        description = "Get detailed info for one slide type: required params, optional params, variants, examples."
    )]
    pub async fn get_slide_type_info(
        &self,
        Parameters(req): Parameters<SlideTypeInfoRequest>,
    ) -> Result<Json<RawJson>, ErrorData> {
        match slide_registry::get_slide_type_info(&req.slide_type) {
            Some(info) => Ok(Json(RawJson(info))),
            None => Err(ErrorData::invalid_request(
                format!("Unknown slide type: '{}'", req.slide_type),
                None,
            )),
        }
    }

    // ── get_slide_types_for_context ───────────────────────────────────────────

    /// Recommend slide types for a given content context.
    #[tool(
        name = "get_slide_types_for_context",
        description = "Get recommended slide types for a context: opening, closing, data, social-proof, features, process."
    )]
    pub async fn get_slide_types_for_context(
        &self,
        Parameters(req): Parameters<SlideTypeContextRequest>,
    ) -> Result<Json<RawJson>, ErrorData> {
        let types = slide_registry::get_slide_types_for_context(&req.context);
        Ok(Json(RawJson(serde_json::json!({
            "context": req.context,
            "recommended_slide_types": types,
        }))))
    }

    // ── list_platforms ────────────────────────────────────────────────────────

    /// List all available export platform presets with dimensions.
    #[tool(
        name = "list_platforms",
        description = "List all export platforms (Instagram, TikTok, LinkedIn, etc.) with pixel dimensions and aspect ratios."
    )]
    pub async fn list_platforms(&self) -> Result<Json<RawJson>, ErrorData> {
        let all = platforms::all_platforms();
        let list: Vec<serde_json::Value> = all
            .iter()
            .map(|p| {
                serde_json::json!({
                    "name": p.name,
                    "width": p.width,
                    "height": p.height,
                    "aspect_ratio": p.aspect_ratio,
                    "default_aspect_ratio": p.default_aspect_ratio,
                    "allowed_aspect_ratios": p.allowed_aspect_ratios,
                    "format": p.format,
                    "description": p.description,
                    "recommended_for": p.recommended_for,
                    "platform_context": platform_context_json(&p.name, &p.default_aspect_ratio),
                })
            })
            .collect();
        Ok(Json(RawJson(serde_json::json!({ "platforms": list }))))
    }

    // ── list_archetypes ───────────────────────────────────────────────────────

    /// List all available brand archetypes.
    #[tool(
        name = "list_archetypes",
        description = "List all brand archetypes (educator, thought_leader, startup_pitch, creator, etc.) with their default styling."
    )]
    pub async fn list_archetypes(&self) -> Result<Json<RawJson>, ErrorData> {
        let all = archetypes::all_archetypes();
        let list: Vec<serde_json::Value> = all
            .iter()
            .map(|a| {
                serde_json::json!({
                    "name": a.name,
                    "description": a.description,
                    "primary_theme": a.primary_theme,
                    "default_bg_style": a.default_bg_style,
                })
            })
            .collect();
        Ok(Json(RawJson(serde_json::json!({ "archetypes": list }))))
    }

    // ── validate_layout ───────────────────────────────────────────────────────

    /// Validate slide params before generating HTML.
    #[tool(
        name = "validate_layout",
        description = "Validate slide parameters before rendering. Returns errors for missing required fields and warnings for common issues."
    )]
    pub async fn validate_layout(
        &self,
        Parameters(req): Parameters<ValidateLayoutRequest>,
    ) -> Result<Json<ValidateLayoutResponse>, ErrorData> {
        let result = validate::validate_layout(
            &req.slide_type,
            &req.params,
            req.html.as_deref(),
            req.aspect_ratio.as_deref(),
        );
        Ok(Json(ValidateLayoutResponse {
            valid: result.valid,
            errors: result.errors,
            warnings: result.warnings,
        }))
    }

    // ── validate_and_fix ──────────────────────────────────────────────────────

    /// Validate slide params and auto-fix common issues.
    #[tool(
        name = "validate_and_fix",
        description = "Validate slide params and auto-fix missing optional fields with sensible defaults. Returns fixed params."
    )]
    pub async fn validate_and_fix(
        &self,
        Parameters(req): Parameters<ValidateAndFixRequest>,
    ) -> Result<Json<ValidateAndFixResponse>, ErrorData> {
        let mut params = req.params;
        let result = validate::validate_and_fix_slide(&req.slide_type, &mut params);
        Ok(Json(ValidateAndFixResponse {
            valid: result.valid,
            errors: result.errors,
            warnings: result.warnings,
            fixes: result.fixes,
            params,
        }))
    }
    #[tool(
        name = "validate_design",
        description = "Validate carousel HTML for design, contrast, accessibility, and overflow issues."
    )]
    pub async fn validate_design(
        &self,
        Parameters(req): Parameters<ValidateDesignRequest>,
    ) -> Result<Json<RawJson>, ErrorData> {
        let report = validate::validate_design(&req.html);
        Ok(Json(RawJson(
            serde_json::to_value(report).unwrap_or(serde_json::json!({})),
        )))
    }

    // ── list_themes ───────────────────────────────────────────────────────────

    /// List all available visual themes.
    #[tool(
        name = "list_themes",
        description = "List all available visual themes (editorial, bold, minimal, dark, vibrant, natural) with style descriptions."
    )]
    pub async fn list_themes(&self) -> Result<Json<RawJson>, ErrorData> {
        Ok(Json(RawJson(serde_json::json!({
            "themes": [
                {
                    "name": "editorial",
                    "description": "Clean, magazine-inspired layout with sharp edges and textured surfaces",
                    "best_for": ["professional", "thought-leadership", "B2B", "media"]
                },
                {
                    "name": "bold",
                    "description": "High-contrast, dynamic layout with strong shadows and gradient surfaces",
                    "best_for": ["product launches", "bold brands", "attention-grabbing"]
                },
                {
                    "name": "minimal",
                    "description": "Clean whitespace-heavy design with no decorations or shadows",
                    "best_for": ["luxury brands", "simplicity-focused", "portfolios", "design studios"]
                },
                {
                    "name": "dark",
                    "description": "Moody dark-mode with glass effects and colored glows",
                    "best_for": ["tech startups", "SaaS", "developer tools", "dark aesthetics"]
                },
                {
                    "name": "vibrant",
                    "description": "Energetic gradient-heavy layout with pill shapes and bold color",
                    "best_for": ["consumer apps", "fun brands", "creators", "gaming"]
                },
                {
                    "name": "natural",
                    "description": "Warm organic shapes, vintage filters, earthy tones",
                    "best_for": ["wellness", "sustainability", "lifestyle brands", "food"]
                }
            ]
        }))))
    }

    // ── recommend_color_scheme ────────────────────────────────────────────────

    /// Generate multiple harmonious color scheme options from a primary color.
    #[tool(
        name = "recommend_color_scheme",
        description = "Generate harmonious color scheme options from a primary brand color using OKLCH perceptual color science."
    )]
    pub async fn recommend_color_scheme(
        &self,
        Parameters(req): Parameters<RecommendColorSchemeRequest>,
    ) -> Result<Json<RawJson>, ErrorData> {
        let style = req.style.as_deref().unwrap_or("modern");
        let num = req.num_schemes.unwrap_or(4).min(6) as usize;
        let presets = [
            "tonal_spot",
            "vibrant",
            "neutral",
            "monochrome",
            "expressive",
            "fidelity",
        ];

        let mut schemes = Vec::new();
        for &p in presets.iter().take(num) {
            if let Ok(tokens) = design_system::derive_palette_with_canvas(
                &req.primary_color,
                style,
                16,
                1.25,
                p,
                "",
                None,
                None,
                None,
                420, // 420 base width
                525, // 525 base height (4:5 at 420)
            ) {
                schemes.push(serde_json::json!({
                    "preset": p,
                    "primary": tokens.primary,
                    "primary_light": tokens.primary_light,
                    "primary_dark": tokens.primary_dark,
                    "accent": tokens.accent,
                    "secondary": tokens.secondary,
                    "tertiary": tokens.tertiary,
                    "temperature": tokens.temperature,
                    "gradient": tokens.gradient,
                }));
            }
        }

        Ok(Json(RawJson(serde_json::json!({
            "primary_color": req.primary_color,
            "schemes": schemes,
            "tip": "Use configure_design with the preferred preset to lock it in for the session."
        }))))
    }

    // ── embed_local_image ────────────────────────────────────────────────────

    /// Convert a local image file to a base64 data URI that can be passed as
    /// `image_url` or `background_image` to generate_slide. Supports PNG, JPEG,
    /// GIF, WebP, and SVG.
    #[tool(
        name = "embed_local_image",
        description = "Convert a local image file to a base64 data URI for use as image_url or background_image in generate_slide. Supports PNG, JPEG, GIF, WebP, and SVG. Returns a data:image/...;base64,... string that can be passed directly to any image parameter. This is the recommended way to use local images — no file:// scheme or external hosting needed."
    )]
    pub async fn embed_local_image(
        &self,
        Parameters(req): Parameters<EmbedLocalImageRequest>,
    ) -> Result<Json<RawJson>, ErrorData> {
        use std::fs;
        use std::path::Path;

        let path = req.file_path.trim();
        if path.is_empty() {
            return Err(ErrorData::invalid_request("file_path is required.", None));
        }

        let p = Path::new(path);
        if !p.exists() {
            let msg = format!("File not found: {}", path);
            return Err(ErrorData::invalid_request(msg, None));
        }

        // Determine MIME type from extension
        let ext = p
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();
        let mime = match ext.as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "webp" => "image/webp",
            "svg" => "image/svg+xml",
            _ => {
                let msg = format!(
                    "Unsupported image extension '{}'. Supported: png, jpg/jpeg, gif, webp, svg.",
                    ext
                );
                return Err(ErrorData::invalid_request(msg, None));
            }
        };

        // Read file and base64-encode
        let bytes = match fs::read(p) {
            Ok(b) => b,
            Err(e) => {
                return Err(ErrorData::internal_error(
                    format!("Failed to read file '{}': {}", path, e),
                    None,
                ));
            }
        };

        // Check size — warn if > 2MB (data URIs inflate HTML size)
        let size_kb = bytes.len() / 1024;
        let warning = if size_kb > 2048 {
            Some(format!(
                "Image is {}KB — large data URIs inflate HTML size and may slow export. Consider resizing to <500KB before embedding.",
                size_kb
            ))
        } else if size_kb > 500 {
            Some(format!(
                "Image is {}KB — consider resizing to <500KB for optimal export performance.",
                size_kb
            ))
        } else {
            None
        };

        let b64 = base64_encode(&bytes);
        let data_uri = if mime == "image/svg+xml" {
            // For SVG, use utf8 encoding (smaller, preserves text)
            let svg_text = String::from_utf8_lossy(&bytes);
            let encoded = svg_text
                .replace('#', "%23")
                .replace('<', "%3C")
                .replace('>', "%3E")
                .replace('"', "'");
            format!("data:image/svg+xml;utf8,{}", encoded)
        } else {
            format!("data:{};base64,{}", mime, b64)
        };

        Ok(Json(RawJson(serde_json::json!({
            "data_uri": data_uri,
            "mime_type": mime,
            "size_bytes": bytes.len(),
            "size_kb": size_kb,
            "warning": warning,
            "usage": "Pass this data_uri string as image_url or background_image in generate_slide."
        }))))
    }

    // ── preview_slide ────────────────────────────────────────────────────────

    /// Render a single slide's HTML to a PNG for quick preview without
    /// exporting the full carousel. Uses headless Chrome.
    #[tool(
        name = "preview_slide",
        description = "Render a single slide's HTML to a PNG file for quick visual preview. Faster than export_carousel_slides for iterating on one slide. Pass the html (from generate_slide) and an output_path; returns the PNG file path."
    )]
    pub async fn preview_slide(
        &self,
        Parameters(req): Parameters<PreviewSlideRequest>,
    ) -> Result<Json<RawJson>, ErrorData> {
        use std::fs;

        if req.html.is_empty() {
            return Err(ErrorData::invalid_request(
                "html is required. Call generate_slide first and pass its html field.",
                None,
            ));
        }

        let output_path = match &req.output_path {
            Some(p) if !p.is_empty() => p.clone(),
            _ => "/tmp/slideforge-preview.png".to_string(),
        };

        // Wrap the slide HTML in a minimal full HTML document for Chrome
        let full_html = format!(
            r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><style>
body {{ margin:0; padding:0; background:#f0f0f0; display:flex; justify-content:center; align-items:center; min-height:100vh; }}
</style></head><body>{}</body></html>"#,
            req.html
        );

        // Write to a temp file
        let temp_html = "/tmp/slideforge-preview.html";
        if let Err(e) = fs::write(temp_html, &full_html) {
            return Err(ErrorData::internal_error(
                format!("Failed to write temp HTML: {}", e),
                None,
            ));
        }

        // Render via headless Chrome
        match export::render_html_to_png(temp_html, &output_path, 1.0) {
            Ok(_) => Ok(Json(RawJson(serde_json::json!({
                "png_path": output_path,
                "message": format!("Preview saved to {}", output_path)
            })))),
            Err(e) => Err(ErrorData::internal_error(
                format!(
                    "Chrome render failed: {}. Ensure Chrome/Chromium is installed.",
                    e
                ),
                None,
            )),
        }
    }
}

// ── Base64 encoder (no external dependency) ──────────────────────────────────

fn base64_encode(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        let b = match chunk.len() {
            3 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32),
            2 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8),
            1 => (chunk[0] as u32) << 16,
            _ => 0,
        };
        result.push(CHARS[((b >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((b >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((b >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(b & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

// ── ServerHandler impl ────────────────────────────────────────────────────────

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            rmcp::model::ServerCapabilities::builder()
                .enable_logging()
                .build(),
        )
    }
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new();
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    // rmcp 1.8 returns RunningService — must .waiting() to keep the event loop alive.
    // Dropping the RunningService cancels the server via DropGuard.
    let running = rmcp::serve_server(server, transport).await?;
    running.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slides::SlideSpec;

    #[tokio::test]
    async fn test_render_carousel_empty_aspect_ratio_and_platform_fallback() {
        let server = Server::new();

        // Configure design to setup state
        let config_req = ConfigureDesignRequest {
            primary_color: "#4f46e5".to_string(),
            font_style: None,
            type_scale_base: None,
            type_scale_ratio: None,
            preset: None,
            brand_name: Some("TestBrand".to_string()),
            brand_handle: Some("@testbrand".to_string()),
            visual_theme: None,
            layout_theme: None,
            effect_theme: None,
            topic: None,
            url: None,
            hashtags: None,
            show_progress: None,
            archetype: None,
            platform: None,
            aspect_ratio: None,
            bg_style: None,
        };

        let _ = server
            .configure_design(Parameters(config_req))
            .await
            .unwrap();

        // 1. If state has empty platform/aspect_ratio (which it does by default or since we didn't specify them in configure_design),
        // let's verify that render_carousel falls back to "instagram_portrait" and None aspect ratio, and resolves successfully.
        let render_req = RenderCarouselRequest {
            slides: vec![SlideSpec {
                html: "<div class=\"slide\">Hello</div>".to_string(),
                background: "light".to_string(),
                variant: "hero".to_string(),
                theme: "modern".to_string(),
                archetype: "educator".to_string(),
            }],
            css_variables: Some(":root { --primary: #4f46e5; }".to_string()),
            google_fonts_url: None,
            heading_font: None,
            body_font: None,
            brand_name: None,
            brand_handle: None,
            include_ig_frame: Some(false),
            output_path: None,
            topic: None,
            url: None,
            hashtags: None,
            show_progress: None,
            platform: Some("".to_string()), // Empty string, should fallback to instagram_portrait
            aspect_ratio: Some("".to_string()), // Empty string, should filter to None
        };

        let res = server
            .render_carousel(Parameters(render_req))
            .await
            .unwrap();
        // Since we fell back to instagram_portrait (which is 1080x1350, so 4:5), let's make sure it rendered successfully.
        assert_eq!(res.0.total_slides, 1);
    }

    #[tokio::test]
    async fn test_export_carousel_slides_empty_aspect_ratio_and_preset_fallback() {
        let server = Server::new();

        // Setup state platform to empty to test default fallback
        {
            let mut state = server.state.lock().unwrap();
            state.platform = "".to_string();
            state.aspect_ratio = "".to_string();
        }

        let export_req = ExportCarouselSlidesRequest {
            html_path: "nonexistent.html".to_string(),
            output_dir: "./nonexistent_dir".to_string(),
            total_slides: 1,
            platform: Some("".to_string()), // empty platform -> instagram_portrait
            aspect_ratio: Some("".to_string()), // empty aspect_ratio -> None
        };

        let res = server.export_carousel_slides(Parameters(export_req)).await;
        // It should either return an error about file not found (internal error) or similar, but NOT invalid request.
        match res {
            Err(e) => {
                // If it got past platforms::resolve_canvas, it will try to export, which fails with internal error.
                // If platforms::resolve_canvas failed, it would return invalid_request about unknown platform or aspect ratio.
                assert!(!e.message.contains("Unknown platform preset"));
                assert!(!e.message.contains("not allowed"));
            }
            _ => {}
        }
    }
}
