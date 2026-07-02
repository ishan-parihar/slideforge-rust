use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SlideSpec {
    pub html: String,
    pub background: String, // light, dark, gradient, mesh, hero
    pub variant: String,
    pub theme: String,
    pub archetype: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CarouselSpec {
    pub slides: Vec<SlideSpec>,
    pub css_variables: String,
    pub google_fonts_url: String,
    pub heading_font: String,
    pub body_font: String,
    pub brand_name: String,
    pub brand_handle: String,
    pub topic: String,
    pub url: String,
    pub hashtags: Vec<String>,
    pub show_progress: bool,
    pub visual_theme: String,
    pub include_ig_frame: bool,
    pub platform: String,
    pub aspect_ratio: String,
    pub canvas_width: u32,
    pub canvas_height: u32,
}

pub fn render_carousel_html(spec: &CarouselSpec) -> String {
    let total = spec.slides.len();

    // Composition is always 4:5 (420×525). Canvas extends to fill target aspect ratio.
    // For taller targets (9:16, 3:4): composition fills width, BG bleeds top/bottom.
    // For wider targets (1:1, 16:9): composition fills height, BG bleeds sides.
    const COMP_WIDTH: u32 = 420;
    const COMP_HEIGHT: u32 = 525;
    let target_w = spec.canvas_width;
    let target_h = spec.canvas_height;
    let target_ratio = target_w as f32 / target_h as f32;
    let native_ratio = COMP_WIDTH as f32 / COMP_HEIGHT as f32; // 0.8

    let (base_w, base_h) = if target_ratio <= native_ratio {
        // Taller target: composition fills canvas width, canvas extends vertically
        (
            COMP_WIDTH,
            (COMP_WIDTH as f32 / target_w as f32 * target_h as f32).round() as u32,
        )
    } else {
        // Wider target: composition fills canvas height, canvas extends horizontally
        (
            (COMP_HEIGHT as f32 / target_h as f32 * target_w as f32).round() as u32,
            COMP_HEIGHT,
        )
    };

    let scale_factor = target_w as f32 / base_w as f32;
    let sf = format!("{:.6}", scale_factor);

    // IG frame overhead at base scale (header ~56px + footer ~90px = ~146px, rounded to 150)
    // This prevents clipping of the IG header and footer elements below the viewport
    let ig_overhead: u32 = if spec.include_ig_frame { 150 } else { 0 };
    let total_base_height = base_h + ig_overhead;
    let total_target_height = (total_base_height as f32 * scale_factor).round() as u32;

    // Core CSS
    let mut css_block = r#"
*, *::before, *::after { margin: 0; padding: 0; box-sizing: border-box; }

:root {
  --slide-width: [BASE_WIDTH]px;
  --slide-height: [BASE_HEIGHT]px;
  --composition-width: 420px;
  --composition-height: 525px;
}

[CSS_VARS]

body {
  display: flex; justify-content: center; align-items: center;
  min-height: 100vh; background: #f0f0f0;
  font-family: var(--font-body, system-ui, sans-serif);
}

.carousel-viewport { width: var(--slide-width); height: var(--slide-height); overflow: hidden; position: relative; }
.carousel-track { display: flex; height: 100%; transition: transform 0.35s cubic-bezier(0.25, 0.1, 0.25, 1); will-change: transform; }

.slide {
  min-width: var(--slide-width); height: var(--slide-height); position: relative;
  display: flex; align-items: center; justify-content: center;
  overflow: hidden;
}
.slide--full-bleed { overflow: hidden; }
.slide-composition {
  width: var(--composition-width); height: var(--composition-height);
  position: relative; overflow: hidden; flex-shrink: 0;
}
/* Full-bleed: allow composition background to bleed beyond 420×525 bounds.
   The inner div (gradient + noise + shapes) is stretched to fill the slide,
   while content stays at designed 420×525 composition dimensions.
   The .slide element's overflow:hidden clips at the final slide boundary. */
.slide--full-bleed .slide-composition {
  overflow: visible;
}
.slide--full-bleed .slide-composition > div:first-of-type {
  position: absolute !important;
  top: calc((var(--composition-height) - var(--slide-height)) / 2) !important;
  left: calc((var(--composition-width) - var(--slide-width)) / 2) !important;
  width: var(--slide-width) !important;
  height: var(--slide-height) !important;
  overflow: hidden !important;
}
/* Content constrainer: reposition content wrapper to original composition
   bounds, vertically CENTERED within the canvas. For 9:16 (747px tall canvas
   with 525px composition), this places content at top:111px instead of top:0,
   eliminating the "content pushed to upper area" bug. For 1:1 and 4:5, the
   calc resolves to 0 (canvas == composition height). */
.slide--full-bleed .slide-content {
  position: absolute !important;
  z-index: 10 !important;
  top: calc((var(--slide-height) - var(--composition-height)) / 2) !important;
  left: calc((var(--slide-width) - var(--composition-width)) / 2) !important;
  width: var(--composition-width) !important;
  height: var(--composition-height) !important;
}
/* Light/mesh slides need the mesh gradient on .slide too */
.slide--full-bleed.slide--light,
.slide--full-bleed.slide--mesh {
  background: var(--gradient-mesh, none), var(--surface-light, #F3F5FC);
}

.slide--light, .slide--mesh { background-color: var(--surface-light, #F3F5FC); color: var(--text-primary, #0A0B0F); }
.slide--dark { background-color: var(--surface-dark, #010105); color: var(--text-on-dark, #ECEEF5); }
.slide--gradient { background: var(--gradient, linear-gradient(165deg, #3F34BD, #6366F1, #8D97FF)); background-color: var(--surface-dark, #010105); color: var(--text-on-dark, #ECEEF5); }
.slide--mesh { background: var(--gradient-mesh, radial-gradient(at 40% 20%, #6366F115 0px, transparent 50%)); color: var(--text-primary, #0A0B0F); }
.slide--hero { background: var(--gradient-hero, linear-gradient(135deg, #010105 0%, #6366F130 50%, #010105 100%)); background-color: var(--surface-dark, #010105); color: var(--text-on-dark, #ECEEF5); }

.slide--dark, .slide--gradient, .slide--hero {
  --text-primary: var(--text-on-dark, #ECEEF5) !important;
  --text-secondary: var(--text-on-dark-secondary, #B9BDC9) !important;
  --border-light: var(--border-dark, #2A2D3D) !important;
  --surface-light: var(--surface-dark, #010105) !important;
  color: var(--text-on-dark, #ECEEF5) !important;
}

.slide-layout {
  position: relative; z-index: 2; display: flex; flex-direction: column;
  height: 100%; padding: var(--space-10) var(--space-6) var(--space-12); width: 100%;
}
.slide-layout--center { justify-content: center; }

.grid-2 { display: flex; gap: var(--space-3); width: 100%; }
.grid-2 > * { flex: 1; min-width: 0; }

.serif { font-family: var(--font-heading, serif); }
.sans { font-family: var(--font-body, sans-serif); }

.display-text {
  font-size: var(--text-2xl-size, 32px); font-weight: var(--text-2xl-weight, 700); 
  line-height: var(--text-2xl-lh, 1.1); letter-spacing: var(--text-2xl-ls, -0.02em);
  margin-bottom: var(--space-2, 16px); text-wrap: balance;
}
.heading-text {
  font-size: var(--text-xl-size, 24px); font-weight: var(--text-xl-weight, 600); 
  line-height: var(--text-xl-lh, 1.2); letter-spacing: var(--text-xl-ls, -0.01em);
  margin-bottom: var(--space-2, 16px); text-wrap: balance;
}
.body-text {
  font-size: var(--text-base-size, 14px); font-weight: var(--text-base-weight, 400); 
  line-height: var(--text-base-lh, 1.5);
  text-wrap: pretty; overflow-wrap: break-word; word-break: break-word;
}
.caption-text {
  font-size: var(--text-sm-size, 11px); font-weight: var(--text-sm-weight, 500); 
  line-height: var(--text-sm-lh, 1.4); letter-spacing: var(--text-sm-ls, 0.02em);
  text-wrap: pretty; overflow-wrap: break-word;
}

.text-gradient {
  background: linear-gradient(135deg, var(--text-primary) 0%, var(--primary) 100%);
  -webkit-background-clip: text; -webkit-text-fill-color: transparent;
}
.slide--dark .text-gradient, .slide--gradient .text-gradient, .slide--hero .text-gradient {
  background: linear-gradient(135deg, #FFFFFF 0%, var(--primary-light) 100%);
  -webkit-background-clip: text; -webkit-text-fill-color: transparent;
}

.card {
  padding: var(--space-2, 20px); border-radius: var(--radius-lg, 12px);
  background: var(--surface-light, #F3F5FC);
  border: 1px solid var(--border-light, #C4C7D1);
  box-shadow: var(--shadow-sm);
  overflow-wrap: break-word; word-break: break-word;
}
.slide--dark .card, .slide--gradient .card, .slide--hero .card {
  background: rgba(255,255,255,0.04);
  border: 1px solid var(--border-dark, #2A2D3D);
  box-shadow: var(--shadow-md);
}

.glass {
  background: rgba(255,255,255,0.04);
  backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px);
  border: 1px solid rgba(255,255,255,0.08);
  box-shadow: var(--shadow-lg);
}
.slide--light .glass, .slide--mesh .glass {
  background: rgba(255,255,255,0.7);
  border: 1px solid rgba(0,0,0,0.06);
}

.badge {
  display: inline-block; padding: var(--space-1, 6px) var(--space-2, 16px); font-size: var(--text-sm-size, 13px); font-weight: 600;
  border-radius: var(--radius-pill, 9999px); text-transform: uppercase;
  letter-spacing: 0.05em; margin-bottom: var(--space-3, 24px);
}
.slide--light .badge, .slide--mesh .badge {
  background: var(--primary)15; color: var(--primary);
  border: 1px solid var(--primary)30;
}
.slide--dark .badge, .slide--gradient .badge, .slide--hero .badge {
  background: var(--text-on-dark)15; color: var(--text-on-dark);
  border: 1px solid var(--text-on-dark)30;
}

.btn {
  display: inline-flex; align-items: center; justify-content: center;
  padding: var(--space-2, 12px) var(--space-4, 28px); font-size: var(--text-base-size, 14px); font-weight: 600;
  border-radius: var(--radius-pill, 9999px); text-decoration: none;
  box-shadow: var(--shadow-md); transition: transform 0.2s;
}
.btn:active { transform: scale(0.97); }
.slide--light .btn, .slide--mesh .btn {
  background: var(--primary); color: #FFFFFF;
}
.slide--dark .btn, .slide--gradient .btn, .slide--hero .btn {
  background: var(--surface-light); color: var(--primary-dark);
}

.icon-box {
  width: var(--space-7, 56px); height: var(--space-7, 56px); border-radius: var(--radius-md, 8px);
  display: flex; align-items: center; justify-content: center;
  font-size: var(--text-xl-size, 28px); margin-bottom: var(--space-3, 24px);
}
.slide--light .icon-box, .slide--mesh .icon-box { background: var(--primary)12; }
.slide--dark .icon-box, .slide--gradient .icon-box, .slide--hero .icon-box { background: rgba(255,255,255,0.12); }

.timeline { display: flex; flex-direction: column; gap: var(--space-2, 16px); }
.timeline-item { display: flex; gap: var(--space-2, 16px); align-items: flex-start; }
.timeline-number {
  font-size: var(--text-xl-size, 26px); font-weight: 300; color: var(--primary);
  min-width: var(--space-4, 36px); line-height: 1;
}
.timeline-content { flex: 1; }

.callout {
  display: flex; gap: var(--space-2, 16px); align-items: flex-start; padding: var(--space-2, 16px);
  border-radius: var(--radius-md, 8px); border: 1px solid transparent;
}
.callout--warning {
  background: rgba(255, 165, 0, 0.1); border-color: var(--status-warning, #FFA500);
}
.callout--info {
  background: rgba(0, 120, 255, 0.1); border-color: var(--status-info, #0078FF);
}

.breadcrumb-progress {
  position: absolute; bottom: var(--space-1, 8px); left: var(--space-3, 28px); right: var(--space-3, 28px);
  display: flex; align-items: center; gap: 6px;
  z-index: 50;
}
.breadcrumb-chip {
  height: 2px; flex: 1; border-radius: 999px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}
.slide--light .breadcrumb-chip, .slide--mesh .breadcrumb-chip { background: rgba(0,0,0,0.12); }
.slide--dark .breadcrumb-chip, .slide--gradient .breadcrumb-chip, .slide--hero .breadcrumb-chip { background: rgba(255,255,255,0.2); }
.slide--light .breadcrumb-chip.completed, .slide--mesh .breadcrumb-chip.completed { background: var(--primary, #7C3AED); opacity: 0.55; }
.slide--dark .breadcrumb-chip.completed, .slide--gradient .breadcrumb-chip.completed, .slide--hero .breadcrumb-chip.completed { background: rgba(255,255,255,0.65); }
.breadcrumb-chip.active {
  height: 3px; flex: 2.2;
}
.slide--light .breadcrumb-chip.active, .slide--mesh .breadcrumb-chip.active { background: var(--primary, #7C3AED); opacity: 1; }
.slide--dark .breadcrumb-chip.active, .slide--gradient .breadcrumb-chip.active, .slide--hero .breadcrumb-chip.active { background: #ffffff; opacity: 1; }

.swipe-arrow {
  position: absolute; top: 0; right: 0; bottom: 0; width: var(--space-6, 48px);
  display: flex; align-items: center; justify-content: center; pointer-events: none;
  z-index: 50;
}
.slide--light .swipe-arrow, .slide--mesh .swipe-arrow { background: linear-gradient(to right, transparent, rgba(0,0,0,0.06)); }
.slide--dark .swipe-arrow, .slide--gradient .swipe-arrow, .slide--hero .swipe-arrow { background: linear-gradient(to right, transparent, rgba(255,255,255,0.08)); }
.swipe-arrow svg { stroke-width: 2; stroke-linecap: round; stroke-linejoin: round; }
.slide--light .swipe-arrow svg, .slide--mesh .swipe-arrow svg { stroke: var(--text-primary, #0B0A0F); opacity: 0.4; }
.slide--dark .swipe-arrow svg, .slide--gradient .swipe-arrow svg, .slide--hero .swipe-arrow svg { stroke: var(--text-on-dark, #EEEDF5); opacity: 0.4; }

.slide__overlay { position: absolute; inset: 0; pointer-events: none; z-index: 45; padding: var(--space-3, 24px) var(--space-3, 28px); display: flex; flex-direction: column; justify-content: space-between; }
.slide__overlay-top { display: flex; justify-content: space-between; align-items: flex-start; }
.slide__overlay-bottom { display: flex; justify-content: space-between; align-items: flex-end; }
.overlay__brand { font-family: var(--heading); font-size: 12px; font-weight: 700; letter-spacing: 0.08em; text-transform: uppercase; }
.slide--light .overlay__brand, .slide--mesh .overlay__brand { opacity: 0.85; }
.slide--dark .overlay__brand, .slide--gradient .overlay__brand, .slide--hero .overlay__brand { color: var(--text-on-dark, #EEEDF5); opacity: 0.85; }
.overlay__topic { font-family: var(--body); font-size: 11.5px; font-weight: 600; text-align: right; max-width: 42%; }
.slide--light .overlay__topic, .slide--mesh .overlay__topic { opacity: 0.8; }
.slide--dark .overlay__topic, .slide--gradient .overlay__topic, .slide--hero .overlay__topic { color: var(--text-on-dark, #EEEDF5); opacity: 0.8; }
.overlay__url { font-family: var(--body); font-size: 11.5px; letter-spacing: 0.01em; font-weight: 600; }
.slide--light .overlay__url, .slide--mesh .overlay__url { opacity: 0.75; }
.slide--dark .overlay__url, .slide--gradient .overlay__url, .slide--hero .overlay__url { color: var(--text-on-dark, #EEEDF5); opacity: 0.75; }
.overlay__hashtags { font-family: var(--body); font-size: 11.5px; font-weight: 600; text-align: right; max-width: 42%; line-height: 1.3; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.slide--light .overlay__hashtags, .slide--mesh .overlay__hashtags { opacity: 0.75; }
.slide--dark .overlay__hashtags, .slide--gradient .overlay__hashtags, .slide--hero .overlay__hashtags { color: var(--text-on-dark, #EEEDF5); opacity: 0.75; }
"#.replace("[CSS_VARS]", &spec.css_variables)
        .replace("[BASE_WIDTH]", &base_w.to_string())
        .replace("[BASE_HEIGHT]", &base_h.to_string());

    let theme_overrides = get_theme_css_overrides(&spec.visual_theme);
    css_block.push_str(theme_overrides);

    if spec.include_ig_frame {
        css_block.push_str(r#"
.ig-frame {
  width: var(--slide-width); background: #fff; border-radius: 0;
  box-shadow: 0 2px 16px rgba(0,0,0,0.10); overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}
.ig-header { display: flex; align-items: center; gap: 10px; padding: 12px 14px; border-bottom: 1px solid #efefef; }
.ig-avatar {
  width: 32px; height: 32px; border-radius: 50%; background: var(--primary, #0B0A0F);
  display: flex; align-items: center; justify-content: center;
  color: #fff; font-size: 14px; font-weight: 700;
}
.ig-handle { font-size: 13px; font-weight: 600; color: #262626; }
.ig-subtitle { font-size: 11px; color: #8e8e8e; }
.ig-dots { display: flex; justify-content: center; gap: 4px; padding: 8px 0; }
.ig-dot { width: 6px; height: 6px; border-radius: 50%; background: #d8d8d8; transition: background 0.2s; }
.ig-dot.active { background: #262626; }
.ig-actions { display: flex; gap: 14px; padding: 8px 14px; }
.ig-actions svg { width: 22px; height: 22px; stroke: #262626; fill: none; stroke-width: 1.8; cursor: pointer; }
.ig-actions svg:hover { opacity: 0.6; }
.ig-actions svg.bookmark { margin-left: auto; }
.ig-caption { padding: 0 14px 12px; font-size: 12px; line-height: 1.45; color: #262626; }
.ig-caption strong { font-weight: 600; }
.ig-caption .timestamp { color: #8e8e8e; font-size: 11px; margin-top: 4px; display: block; }
"#);
    }

    let mut slides_html = String::new();
    let valid_bg_types = vec!["light", "dark", "gradient", "mesh", "hero"];

    for (idx, slide) in spec.slides.iter().enumerate() {
        let is_last = idx == total - 1;
        let show_progress_slide = spec.show_progress;
        let show_arrow = !is_last;

        // Detect if theme "dark" forces a visually dark surface
        // When theme="dark", ALL slides must use dark CSS class regardless of bg_style
        // to ensure correct text-on-dark colors via CSS variable overrides.
        let theme_is_dark = spec.visual_theme == "dark";

        let mut bg_class = if theme_is_dark {
            // Dark theme forces dark CSS class on ALL slides for correct theme vars
            match slide.background.as_str() {
                "dark" | "gradient" | "hero" => format!("slide--{}", slide.background),
                _ => "slide--dark".to_string(),
            }
        } else if valid_bg_types.contains(&slide.background.as_str()) {
            format!("slide--{}", slide.background)
        } else {
            String::new()
        };

        if slide.html.contains("background-image") {
            if bg_class.is_empty() {
                bg_class = "has-bg-image".to_string();
            } else {
                bg_class.push_str(" has-bg-image");
            }
        }

        let mut bg_style = String::new();
        // Check if custom hex color
        if bg_class.is_empty() && slide.background.starts_with('#') && slide.background.len() == 7 {
            bg_style = format!(r#" style="background-color: {};""#, slide.background);
            if slide.html.contains("background-image") {
                bg_class = "has-bg-image".to_string();
            }
        }

        let progress_class = if show_progress_slide {
            " has-progress"
        } else {
            ""
        };

        // Top/bottom overlay
        let mut overlay_html = String::new();
        if !spec.brand_name.is_empty()
            || !spec.topic.is_empty()
            || !spec.url.is_empty()
            || !spec.hashtags.is_empty()
        {
            // When the slide has a background image, add inline text-shadow
            // to overlay spans so the validator recognizes the contrast
            // backing and the text remains legible over photographic backgrounds.
            let has_bg_img = slide.html.contains("background-image");
            let shadow_attr = if has_bg_img {
                r#" style="text-shadow: 0 1px 3px rgba(0,0,0,0.45), 0 0 1px rgba(0,0,0,0.3);""#
            } else {
                ""
            };
            let top_left = if !spec.brand_name.is_empty() {
                format!(
                    r#"<span class="overlay__brand"{}>{}</span>"#,
                    shadow_attr,
                    escape_html(&spec.brand_name)
                )
            } else {
                String::new()
            };
            let top_right = if !spec.topic.is_empty() {
                format!(
                    r#"<span class="overlay__topic"{}>{}</span>"#,
                    shadow_attr,
                    escape_html(&spec.topic)
                )
            } else {
                String::new()
            };
            let bottom_left = if !spec.url.is_empty() {
                format!(
                    r#"<span class="overlay__url"{}>{}</span>"#,
                    shadow_attr,
                    escape_html(&spec.url)
                )
            } else {
                String::new()
            };

            let mut clean_tags = vec![];
            let mut char_count = 0;
            for h in spec.hashtags.iter().take(2) {
                let tag = format!("#{}", h.trim_start_matches('#'));
                if char_count + tag.len() + (if clean_tags.is_empty() { 0 } else { 1 }) <= 24 {
                    clean_tags.push(tag.clone());
                    char_count += tag.len() + (if clean_tags.len() > 1 { 1 } else { 0 });
                } else {
                    break;
                }
            }
            let hashtags_str = clean_tags.join(" ");
            let bottom_right = if !hashtags_str.is_empty() {
                format!(
                    r#"<span class="overlay__hashtags"{}>{}</span>"#,
                    shadow_attr,
                    escape_html(&hashtags_str)
                )
            } else {
                String::new()
            };

            overlay_html = format!(
                r#"  <div class="slide__overlay">
    <div class="slide__overlay-top">
      {}
      {}
    </div>
    <div class="slide__overlay-bottom">
      {}
      {}
    </div>
  </div>"#,
                top_left, top_right, bottom_left, bottom_right
            );
        }

        // Slide progress chips
        let mut progress_html = String::new();
        if show_progress_slide {
            let mut chips = vec![];
            for i in 0..total {
                if i < idx {
                    chips.push("completed");
                } else if i == idx {
                    chips.push("active");
                } else {
                    chips.push("");
                }
            }
            let chip_html: String = chips
                .into_iter()
                .map(|state| format!(r#"<div class="breadcrumb-chip {}"></div>"#, state))
                .collect();

            progress_html = format!(
                r#"  <div class="breadcrumb-progress">
    {}
  </div>"#,
                chip_html
            );
        }

        // Swipe Arrow
        let mut arrow_html = String::new();
        if show_arrow {
            arrow_html = r#"  <div class="swipe-arrow">
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
      <path d="M9 6L15 12L9 18"/>
    </svg>
  </div>"#
                .to_string();
        }

        // Full-bleed: when canvas differs from 4:5 composition, the composition
        // expands via CSS to fill the entire slide, so slide_base() backgrounds,
        // noise, and shapes naturally extend to the full canvas.
        let is_full_bleed = base_w != COMP_WIDTH || base_h != COMP_HEIGHT;
        let full_bleed_class = if is_full_bleed {
            " slide--full-bleed"
        } else {
            ""
        };

        // Overlay, progress, and arrow are placed as direct children of .slide
        // (not .slide-composition) so they anchor to the full canvas for
        // full-bleed aspect ratios. For 4:5 (canvas == composition), this is
        // a no-op since .slide and .slide-composition share the same bounds.
        slides_html.push_str(&format!(
            r#"<div class="slide {}{}"{}><div class="slide-composition">
{}
</div>{}
{}
{}</div>
"#,
            bg_class,
            full_bleed_class,
            bg_style,
            slide.html,
            overlay_html,
            progress_html,
            arrow_html
        ));
    }

    let dots = (0..total)
        .map(|i| {
            format!(
                r#"<div class="ig-dot{}"></div>"#,
                if i == 0 { " active" } else { "" }
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let avatar_letter = spec
        .brand_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());

    let mut ig_header = String::new();
    let mut ig_footer = String::new();
    let mut js_block = String::new();

    if spec.include_ig_frame {
        ig_header = format!(
            r#"<div class="ig-frame">
  <div class="ig-header">
    <div class="ig-avatar">{}</div>
    <div>
      <div class="ig-handle">{}</div>
      <div class="ig-subtitle">Carousel</div>
    </div>
  </div>"#,
            avatar_letter, spec.brand_handle
        );

        ig_footer = format!(
            r#"  <div class="ig-dots" id="igDots">{}</div>
  <div class="ig-actions">
    <svg viewBox="0 0 24 24"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/></svg>
    <svg viewBox="0 0 24 24"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>
    <svg viewBox="0 0 24 24"><line x1="22" y1="2" x2="11" y2="13"/><polygon points="22 2 15 22 11 13 2 9 22 2"/></svg>
    <svg class="bookmark" viewBox="0 0 24 24"><path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"/></svg>
  </div>
  <div class="ig-caption">
    <strong>{}</strong> Swipe through for the full breakdown
    <span class="timestamp">2 hours ago</span>
  </div>
</div>"#,
            dots, spec.brand_handle
        );

        js_block = r#"
<script>
(function() {
  const track = document.getElementById('carouselTrack');
  const dots = document.querySelectorAll('.ig-dot');
  let current = 0;
  let startX = 0;
  let isDragging = false;
  const total = [TOTAL];

  function goTo(index) {
    current = Math.max(0, Math.min(index, total - 1));
    track.style.transition = 'transform 0.35s cubic-bezier(0.25, 0.1, 0.25, 1)';
    track.style.transform = `translateX(${-current * [SLIDE_WIDTH]}px)`;
    if (dots.length) dots.forEach((d, i) => d.classList.toggle('active', i === current));
  }

  track.addEventListener('mousedown', (e) => { isDragging = true; startX = e.clientX; track.style.transition = 'none'; });
  track.addEventListener('touchstart', (e) => { isDragging = true; startX = e.touches[0].clientX; track.style.transition = 'none'; }, { passive: true });

  function endDrag(x) {
    if (!isDragging) return;
    isDragging = false;
    const diff = startX - x;
    if (Math.abs(diff) > 50) goTo(current + (diff > 0 ? 1 : -1));
    else goTo(current);
  }

  track.addEventListener('mouseup', (e) => endDrag(e.clientX));
  track.addEventListener('touchend', (e) => endDrag(e.changedTouches[0].clientX));
  track.addEventListener('mouseleave', (e) => { if (isDragging) endDrag(e.clientX); });

  document.addEventListener('keydown', (e) => {
    if (e.key === 'ArrowRight') goTo(current + 1);
    if (e.key === 'ArrowLeft') goTo(current - 1);
  });
})();
</script>
"#.replace("[TOTAL]", &total.to_string()).replace("[SLIDE_WIDTH]", &base_w.to_string());
    }

    let font_link = if !spec.google_fonts_url.is_empty() {
        format!(
            r#"<link href="{}" rel="stylesheet">"#,
            spec.google_fonts_url
        )
    } else {
        String::new()
    };

    let carousel_html = format!(
        r#"{header}
<div class="carousel-viewport">
<div class="carousel-track" id="carouselTrack">
{slides}
</div>
</div>
{footer}"#,
        header = ig_header,
        slides = slides_html,
        footer = ig_footer,
    );

    // Wrap in vectoric scale container
    // outer container uses total_target_height to include IG frame elements
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
{font}
<style>
{style}
</style>
</head>
<body>

<div style="width:{target_w}px;height:{target_h_ig}px;overflow:visible;position:relative;margin:0 auto;">
  <div style="transform:scale({sf});transform-origin:top left;width:{base_w}px;height:{total_base_h}px;">
    {carousel}
  </div>
</div>

{js}
</body>
</html>"#,
        font = font_link,
        style = css_block,
        target_w = spec.canvas_width,
        target_h_ig = total_target_height,
        sf = sf,
        base_w = base_w,
        total_base_h = total_base_height,
        carousel = carousel_html,
        js = js_block,
    )
}

fn get_theme_css_overrides(theme: &str) -> &'static str {
    match theme {
        "editorial" => {
            r#"
            :root {
                --font-heading: 'Playfair Display', Georgia, serif;
                --font-body: 'Inter', system-ui, sans-serif;
                --radius-lg: 8px;
                --radius-md: 6px;
            }
            .slide--light, .slide--mesh { --surface-light: #F8F6F3; }
            .slide--dark, .slide--hero { --surface-dark: #0A0A0F; }
            .overlay__brand {
                font-family: var(--font-heading), Georgia, serif !important;
                font-style: normal !important;
                text-transform: none !important;
                font-size: 12.5px !important;
                font-weight: 700 !important;
                letter-spacing: 0.02em !important;
                opacity: 0.9 !important;
            }
            .overlay__topic, .overlay__url, .overlay__hashtags {
                font-family: var(--font-body), sans-serif !important;
                font-style: normal !important;
                text-transform: uppercase !important;
                font-size: 11.5px !important;
                font-weight: 600 !important;
                letter-spacing: 0.12em !important;
                opacity: 0.75 !important;
            }
        "#
        }
        "bold" => {
            r#"
            :root {
                --font-heading: 'Inter', system-ui, sans-serif;
                --font-body: 'Inter', system-ui, sans-serif;
                --radius-lg: 4px;
                --radius-md: 4px;
                --shadow-sm: 0 2px 8px rgba(0,0,0,0.15);
                --shadow-md: 0 4px 16px rgba(0,0,0,0.2);
            }
            .slide--light, .slide--mesh { --surface-light: #F0F0F5; }
            .slide--dark, .slide--hero { --surface-dark: #050508; }
            .overlay__brand {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 900 !important;
                text-transform: uppercase !important;
                font-size: 12px !important;
                letter-spacing: 0.15em !important;
                opacity: 0.95 !important;
            }
            .overlay__topic, .overlay__url, .overlay__hashtags {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 700 !important;
                text-transform: uppercase !important;
                font-size: 11.5px !important;
                letter-spacing: 0.12em !important;
                opacity: 0.8 !important;
            }
        "#
        }
        "minimal" => {
            r#"
            :root {
                --font-heading: 'Inter', system-ui, sans-serif;
                --font-body: 'Inter', system-ui, sans-serif;
                --radius-lg: 2px;
                --radius-md: 2px;
                --shadow-sm: none;
                --shadow-md: none;
            }
            .slide--light, .slide--mesh { --surface-light: #FFFFFF; }
            .slide--dark, .slide--hero { --surface-dark: #111111; }
            .overlay__brand {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 600 !important;
                font-size: 12px !important;
                letter-spacing: 0.1em !important;
                text-transform: uppercase !important;
                opacity: 0.85 !important;
            }
            .overlay__topic, .overlay__url, .overlay__hashtags {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 500 !important;
                font-size: 11.5px !important;
                letter-spacing: 0.08em !important;
                text-transform: lowercase !important;
                opacity: 0.75 !important;
            }
        "#
        }
        "dark" => {
            r#"
            :root {
                --font-heading: 'Inter', system-ui, sans-serif;
                --font-body: 'Inter', system-ui, sans-serif;
                --radius-lg: 12px;
                --radius-md: 10px;
            }
            .slide--light, .slide--mesh { --surface-light: #1A1A2E; }
            .slide--dark, .slide--hero { --surface-dark: #0D0D14; }
            .slide--light .overlay__brand,
            .slide--light .overlay__topic,
            .slide--light .overlay__url,
            .slide--light .overlay__hashtags,
            .slide--mesh .overlay__brand,
            .slide--mesh .overlay__topic,
            .slide--mesh .overlay__url,
            .slide--mesh .overlay__hashtags { color: var(--text-on-dark, #EEEDF5) !important; }
            .overlay__brand {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 800 !important;
                font-size: 12px !important;
                letter-spacing: 0.12em !important;
                text-transform: uppercase !important;
                opacity: 0.9 !important;
            }
            .overlay__topic, .overlay__url, .overlay__hashtags {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 600 !important;
                font-size: 11.5px !important;
                letter-spacing: 0.1em !important;
                text-transform: uppercase !important;
                opacity: 0.8 !important;
            }
        "#
        }
        "vibrant" => {
            r#"
            :root {
                --font-heading: 'Inter', system-ui, sans-serif;
                --font-body: 'Inter', system-ui, sans-serif;
                --radius-lg: 16px;
                --radius-md: 12px;
            }
            .slide--light, .slide--mesh { --surface-light: #F5F3FF; }
            .slide--dark, .slide--hero { --surface-dark: #0A0515; }
            .overlay__brand {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 800 !important;
                font-size: 12px !important;
                color: var(--primary) !important;
                letter-spacing: 0.08em !important;
                text-transform: uppercase !important;
                opacity: 0.95 !important;
            }
            .overlay__topic, .overlay__url, .overlay__hashtags {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 600 !important;
                font-size: 11.5px !important;
                letter-spacing: 0.08em !important;
                text-transform: uppercase !important;
                opacity: 0.85 !important;
            }
        "#
        }
        "natural" => {
            r#"
            :root {
                --font-heading: 'Playfair Display', Georgia, serif;
                --font-body: 'Inter', system-ui, sans-serif;
                --radius-lg: 20px;
                --radius-md: 16px;
            }
            .slide--light, .slide--mesh { --surface-light: #FAF8F5; }
            .slide--dark, .slide--hero { --surface-dark: #0F0D0A; }
            .overlay__brand {
                font-family: var(--font-heading), Georgia, serif !important;
                font-style: italic !important;
                font-weight: 600 !important;
                font-size: 12.5px !important;
                letter-spacing: 0.03em !important;
                opacity: 0.9 !important;
            }
            .overlay__topic, .overlay__url, .overlay__hashtags {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 600 !important;
                font-size: 11.5px !important;
                letter-spacing: 0.1em !important;
                text-transform: uppercase !important;
                opacity: 0.75 !important;
            }
        "#
        }
        _ => "",
    }
}

fn escape_html(input: &str) -> String {
    let mut s = String::new();
    for c in input.chars() {
        match c {
            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),
            '&' => s.push_str("&amp;"),
            '"' => s.push_str("&quot;"),
            '\'' => s.push_str("&#x27;"),
            _ => s.push(c),
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_carousel_uses_canvas_dimensions() {
        let spec = CarouselSpec {
            slides: vec![SlideSpec {
                html: "<div>hello</div>".to_string(),
                background: "light".to_string(),
                variant: "test".to_string(),
                theme: "minimal".to_string(),
                archetype: "educator".to_string(),
            }],
            css_variables: ":root { --primary:#000; }".to_string(),
            google_fonts_url: String::new(),
            heading_font: "Inter".to_string(),
            body_font: "Inter".to_string(),
            brand_name: "Brand".to_string(),
            brand_handle: "@brand".to_string(),
            topic: "Topic".to_string(),
            url: "https://example.com".to_string(),
            hashtags: vec![],
            show_progress: false,
            visual_theme: "minimal".to_string(),
            include_ig_frame: false,
            platform: "instagram_portrait".to_string(),
            aspect_ratio: "3:4".to_string(),
            canvas_width: 360,
            canvas_height: 480,
        };

        let html = render_carousel_html(&spec);
        assert!(html.contains("--slide-width: 420px"));
        assert!(html.contains("--slide-height: 560px")); // 3:4 → taller, canvas extends vertically
        assert!(html.contains("--composition-width: 420px"));
        assert!(html.contains("--composition-height: 525px"));
        assert!(html.contains("slide-composition"));
        assert!(html.contains("width: var(--slide-width)"));
        assert!(html.contains("height: var(--slide-height)"));
        assert!(html.contains("transform:scale(0.857143)"));
        assert!(html.contains("width:360px;height:480px")); // outer container
        // Full-bleed: 3:4 differs from 4:5, so full-bleed class present
        assert!(html.contains("slide--full-bleed"));
    }

    #[test]
    fn test_render_carousel_4_5_native_no_full_bleed() {
        // 4:5 → exact fit, no full-bleed needed
        let spec = CarouselSpec {
            slides: vec![SlideSpec {
                html: "<div>hello</div>".to_string(),
                background: "dark".to_string(),
                variant: "test".to_string(),
                theme: "minimal".to_string(),
                archetype: "educator".to_string(),
            }],
            css_variables: ":root { --primary:#000; }".to_string(),
            google_fonts_url: String::new(),
            heading_font: "Inter".to_string(),
            body_font: "Inter".to_string(),
            brand_name: "Brand".to_string(),
            brand_handle: "@brand".to_string(),
            topic: "Topic".to_string(),
            url: "https://example.com".to_string(),
            hashtags: vec![],
            show_progress: false,
            visual_theme: "minimal".to_string(),
            include_ig_frame: false,
            platform: "instagram_portrait".to_string(),
            aspect_ratio: "4:5".to_string(),
            canvas_width: 1080,
            canvas_height: 1350,
        };

        let html = render_carousel_html(&spec);
        assert!(html.contains("--slide-width: 420px"));
        assert!(html.contains("--slide-height: 525px")); // 4:5 → exact native fit
        assert!(html.contains("--composition-width: 420px"));
        assert!(html.contains("--composition-height: 525px"));
        // No slide-level noise div for 4:5 (noise only appears as class on .slide elements)
        assert!(!html.contains(r#"<div style="position:absolute;inset:0;background:var(--texture-grain);opacity:0.04;pointer-events:none;z-index:1;"></div>"#));
        assert!(html.contains("transform:scale(2.571429)"));
        assert!(html.contains("width:1080px;height:1350px"));
    }

    #[test]
    fn test_render_carousel_9_16_full_bleed() {
        // 9:16 → much taller, full-bleed with BG bleed top/bottom
        let spec = CarouselSpec {
            slides: vec![SlideSpec {
                html: "<div>hello</div>".to_string(),
                background: "gradient".to_string(),
                variant: "test".to_string(),
                theme: "minimal".to_string(),
                archetype: "educator".to_string(),
            }],
            css_variables: ":root { --primary:#000; }".to_string(),
            google_fonts_url: String::new(),
            heading_font: "Inter".to_string(),
            body_font: "Inter".to_string(),
            brand_name: "Brand".to_string(),
            brand_handle: "@brand".to_string(),
            topic: "Topic".to_string(),
            url: "https://example.com".to_string(),
            hashtags: vec![],
            show_progress: false,
            visual_theme: "minimal".to_string(),
            include_ig_frame: false,
            platform: "instagram_reels".to_string(),
            aspect_ratio: "9:16".to_string(),
            canvas_width: 1080,
            canvas_height: 1920,
        };

        let html = render_carousel_html(&spec);
        assert!(html.contains("--slide-width: 420px"));
        assert!(html.contains("--slide-height: 747px")); // 9:16 → canvas extends vertically
        assert!(html.contains("--composition-width: 420px"));
        assert!(html.contains("--composition-height: 525px"));
        assert!(html.contains("slide--full-bleed")); // full-bleed for 9:16
    }

    #[test]
    fn test_render_carousel_1_1_full_bleed_wider() {
        // 1:1 → wider, full-bleed with BG bleed sides
        let spec = CarouselSpec {
            slides: vec![SlideSpec {
                html: "<div>hello</div>".to_string(),
                background: "light".to_string(),
                variant: "test".to_string(),
                theme: "minimal".to_string(),
                archetype: "educator".to_string(),
            }],
            css_variables: ":root { --primary:#000; }".to_string(),
            google_fonts_url: String::new(),
            heading_font: "Inter".to_string(),
            body_font: "Inter".to_string(),
            brand_name: "Brand".to_string(),
            brand_handle: "@brand".to_string(),
            topic: "Topic".to_string(),
            url: "https://example.com".to_string(),
            hashtags: vec![],
            show_progress: false,
            visual_theme: "minimal".to_string(),
            include_ig_frame: false,
            platform: "instagram_square".to_string(),
            aspect_ratio: "1:1".to_string(),
            canvas_width: 1080,
            canvas_height: 1080,
        };

        let html = render_carousel_html(&spec);
        assert!(html.contains("--slide-width: 525px")); // 1:1 → wider canvas
        assert!(html.contains("--slide-height: 525px"));
        assert!(html.contains("--composition-width: 420px"));
        assert!(html.contains("--composition-height: 525px"));
        assert!(html.contains("slide--full-bleed")); // full-bleed for 1:1
        assert!(html.contains(".slide--full-bleed { overflow: hidden; }"));
        // Full-bleed composition allows overflow:visible so backgrounds bleed
        assert!(html.contains("overflow: visible;"));
        assert!(!html.contains("Allow glow/shadow effects to extend beyond composition bounds"));
        // Full-bleed: content constrainer CSS is present
        assert!(html.contains("slide-content"));
    }

    #[test]
    fn test_render_carousel_overlay_and_progress_are_readable_base_size() {
        let spec = CarouselSpec {
            slides: vec![SlideSpec {
                html: "<div>hello</div>".to_string(),
                background: "light".to_string(),
                variant: "test".to_string(),
                theme: "minimal".to_string(),
                archetype: "educator".to_string(),
            }],
            css_variables: ":root { --primary:#000; }".to_string(),
            google_fonts_url: String::new(),
            heading_font: "Inter".to_string(),
            body_font: "Inter".to_string(),
            brand_name: "Brand".to_string(),
            brand_handle: "@brand".to_string(),
            topic: "Topic".to_string(),
            url: "https://example.com".to_string(),
            hashtags: vec!["slideforge".to_string()],
            show_progress: true,
            visual_theme: "minimal".to_string(),
            include_ig_frame: false,
            platform: "instagram_portrait".to_string(),
            aspect_ratio: "4:5".to_string(),
            canvas_width: 1080,
            canvas_height: 1350,
        };

        let html = render_carousel_html(&spec);
        assert!(html.contains(".overlay__brand { font-family: var(--heading); font-size: 12px;"));
        assert!(html.contains(".overlay__url { font-family: var(--body); font-size: 11.5px;"));
        assert!(html.contains(".breadcrumb-chip {\n  height: 2px;"));
        assert!(html.contains(".breadcrumb-chip.active {\n  height: 3px;"));
        assert!(!html.contains("font-size: 9px !important"));
        assert!(!html.contains("font-size: 9.5px !important"));
    }
}
