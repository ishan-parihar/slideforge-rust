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

    // Core CSS
    let mut css_block = r#"
*, *::before, *::after { margin: 0; padding: 0; box-sizing: border-box; }

:root {
  --slide-width: [SLIDE_WIDTH]px;
  --slide-height: [SLIDE_HEIGHT]px;
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
  display: flex; flex-direction: column;
  overflow: hidden;
}
.slide.has-progress { padding-bottom: 0; }

.slide--light, .slide--mesh { background-color: var(--surface-light, #F3F5FC); color: var(--text-primary, #0A0B0F); }
.slide--dark { background-color: var(--surface-dark, #010105); color: var(--text-on-dark, #ECEEF5); }
.slide--gradient { background: var(--gradient, linear-gradient(165deg, #3F34BD, #6366F1, #8D97FF)); background-color: var(--surface-dark, #010105); color: var(--text-on-dark, #ECEEF5); }
.slide--mesh { background: var(--gradient-mesh, radial-gradient(at 40% 20%, #6366F115 0px, transparent 50%)); color: var(--text-primary, #0A0B0F); }
.slide--hero { background: var(--gradient-hero, linear-gradient(135deg, #010105 0%, #6366F130 50%, #010105 100%)); background-color: var(--surface-dark, #010105); color: var(--text-on-dark, #ECEEF5); }

.slide--dark, .slide--gradient, .slide--hero {
  --text-primary: var(--text-on-dark, #ECEEF5);
  --text-secondary: var(--text-on-dark-secondary, #B9BDC9);
  --border-light: var(--border-dark, #2A2D3D);
  --surface-light: var(--surface-dark, #010105);
}

.slide-layout {
  position: relative; z-index: 2; display: flex; flex-direction: column;
  height: 100%; padding: 80px 52px 90px; width: 100%;
}
.slide-layout--center { justify-content: center; }

.grid-2 { display: flex; gap: 24px; width: 100%; }
.grid-2 > * { flex: 1; min-width: 0; }

.serif { font-family: var(--font-heading, serif); }
.sans { font-family: var(--font-body, sans-serif); }

.display-text {
  font-size: 32px; font-weight: 700; line-height: 1.1; letter-spacing: -0.02em;
  margin-bottom: 16px; text-wrap: balance;
}
.heading-text {
  font-size: 24px; font-weight: 600; line-height: 1.2; letter-spacing: -0.01em;
  margin-bottom: 16px; text-wrap: balance;
}
.body-text {
  font-size: 14px; font-weight: 400; line-height: 1.5;
  text-wrap: pretty; overflow-wrap: break-word; word-break: break-word;
}
.caption-text {
  font-size: 11px; font-weight: 500; line-height: 1.4; letter-spacing: 0.02em;
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
  padding: 20px; border-radius: var(--radius-lg, 12px);
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
  display: inline-block; padding: 6px 16px; font-size: 13px; font-weight: 600;
  border-radius: var(--radius-pill, 9999px); text-transform: uppercase;
  letter-spacing: 0.05em; margin-bottom: 24px;
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
  padding: 12px 28px; font-size: 14px; font-weight: 600;
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
  width: 56px; height: 56px; border-radius: var(--radius-md, 8px);
  display: flex; align-items: center; justify-content: center;
  font-size: 28px; margin-bottom: 24px;
}
.slide--light .icon-box, .slide--mesh .icon-box { background: var(--primary)12; }
.slide--dark .icon-box, .slide--gradient .icon-box, .slide--hero .icon-box { background: rgba(255,255,255,0.12); }

.timeline { display: flex; flex-direction: column; gap: 16px; }
.timeline-item { display: flex; gap: 16px; align-items: flex-start; }
.timeline-number {
  font-size: 26px; font-weight: 300; color: var(--primary);
  min-width: 36px; line-height: 1;
}
.timeline-content { flex: 1; }

.callout {
  display: flex; gap: 16px; align-items: flex-start; padding: 16px;
  border-radius: var(--radius-md, 8px); border: 1px solid transparent;
}
.callout--warning {
  background: rgba(255, 165, 0, 0.1); border-color: var(--status-warning, #FFA500);
}
.callout--info {
  background: rgba(0, 120, 255, 0.1); border-color: var(--status-info, #0078FF);
}

.breadcrumb-progress {
  position: absolute; bottom: 42px; left: 28px; right: 28px;
  display: flex; align-items: center; gap: 5px;
  z-index: 50;
}
.breadcrumb-chip {
  height: 4px; flex: 1; border-radius: 2px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}
.slide--light .breadcrumb-chip, .slide--mesh .breadcrumb-chip { background: rgba(0,0,0,0.12); }
.slide--dark .breadcrumb-chip, .slide--gradient .breadcrumb-chip, .slide--hero .breadcrumb-chip { background: rgba(255,255,255,0.2); }
.slide--light .breadcrumb-chip.completed, .slide--mesh .breadcrumb-chip.completed { background: var(--primary, #7C3AED); opacity: 0.55; }
.slide--dark .breadcrumb-chip.completed, .slide--gradient .breadcrumb-chip.completed, .slide--hero .breadcrumb-chip.completed { background: rgba(255,255,255,0.65); }
.breadcrumb-chip.active {
  height: 5px; flex: 2.2;
}
.slide--light .breadcrumb-chip.active, .slide--mesh .breadcrumb-chip.active { background: var(--primary, #7C3AED); opacity: 1; }
.slide--dark .breadcrumb-chip.active, .slide--gradient .breadcrumb-chip.active, .slide--hero .breadcrumb-chip.active { background: #ffffff; opacity: 1; }

.swipe-arrow {
  position: absolute; top: 0; right: 0; bottom: 0; width: 48px;
  display: flex; align-items: center; justify-content: center; pointer-events: none;
  z-index: 50;
}
.slide--light .swipe-arrow, .slide--mesh .swipe-arrow { background: linear-gradient(to right, transparent, rgba(0,0,0,0.06)); }
.slide--dark .swipe-arrow, .slide--gradient .swipe-arrow, .slide--hero .swipe-arrow { background: linear-gradient(to right, transparent, rgba(255,255,255,0.08)); }
.swipe-arrow svg { stroke-width: 2; stroke-linecap: round; stroke-linejoin: round; }
.slide--light .swipe-arrow svg, .slide--mesh .swipe-arrow svg { stroke: var(--text-primary, #0B0A0F); opacity: 0.4; }
.slide--dark .swipe-arrow svg, .slide--gradient .swipe-arrow svg, .slide--hero .swipe-arrow svg { stroke: var(--text-on-dark, #EEEDF5); opacity: 0.4; }

.slide__overlay { position: absolute; inset: 0; pointer-events: none; z-index: 45; padding: 24px 28px; display: flex; flex-direction: column; justify-content: space-between; }
.slide__overlay-top { display: flex; justify-content: space-between; align-items: flex-start; }
.slide__overlay-bottom { display: flex; justify-content: space-between; align-items: flex-end; }
.overlay__brand { font-family: var(--heading); font-size: 10.5px; font-weight: 700; letter-spacing: 0.1em; text-transform: uppercase; }
.slide--light .overlay__brand, .slide--mesh .overlay__brand { opacity: 0.85; }
.slide--dark .overlay__brand, .slide--gradient .overlay__brand, .slide--hero .overlay__brand { color: var(--text-on-dark, #EEEDF5); opacity: 0.85; }
.overlay__topic { font-family: var(--body); font-size: 10.5px; font-weight: 500; text-align: right; max-width: 40%; }
.slide--light .overlay__topic, .slide--mesh .overlay__topic { opacity: 0.8; }
.slide--dark .overlay__topic, .slide--gradient .overlay__topic, .slide--hero .overlay__topic { color: var(--text-on-dark, #EEEDF5); opacity: 0.8; }
.overlay__url { font-family: var(--body); font-size: 9.5px; letter-spacing: 0.02em; }
.slide--light .overlay__url, .slide--mesh .overlay__url { opacity: 0.75; }
.slide--dark .overlay__url, .slide--gradient .overlay__url, .slide--hero .overlay__url { color: var(--text-on-dark, #EEEDF5); opacity: 0.75; }
.overlay__hashtags { font-family: var(--body); font-size: 9.5px; text-align: right; max-width: 40%; line-height: 1.3; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.slide--light .overlay__hashtags, .slide--mesh .overlay__hashtags { opacity: 0.75; }
.slide--dark .overlay__hashtags, .slide--gradient .overlay__hashtags, .slide--hero .overlay__hashtags { color: var(--text-on-dark, #EEEDF5); opacity: 0.75; }
"#.replace("[CSS_VARS]", &spec.css_variables)
        .replace("[SLIDE_WIDTH]", &spec.canvas_width.to_string())
        .replace("[SLIDE_HEIGHT]", &spec.canvas_height.to_string());

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
        // When theme="dark" with bg_style="light", the effective surface is visually dark
        // but CSS would apply light theme vars. Fix by using dark CSS class instead.
        let theme_is_dark = spec.visual_theme == "dark";
        let bg_is_light = slide.background == "light";

        let mut bg_class = if theme_is_dark && bg_is_light {
            // Dark theme with light bg → use dark CSS class for correct theme vars
            "slide--dark".to_string()
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
            let top_left = if !spec.brand_name.is_empty() {
                format!(
                    r#"<span class="overlay__brand">{}</span>"#,
                    escape_html(&spec.brand_name)
                )
            } else {
                String::new()
            };
            let top_right = if !spec.topic.is_empty() {
                format!(
                    r#"<span class="overlay__topic">{}</span>"#,
                    escape_html(&spec.topic)
                )
            } else {
                String::new()
            };
            let bottom_left = if !spec.url.is_empty() {
                format!(
                    r#"<span class="overlay__url">{}</span>"#,
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
                    r#"<span class="overlay__hashtags">{}</span>"#,
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

        slides_html.push_str(&format!(
            r#"<div class="slide {}{}"{}>
{}
{}
{}
{}
</div>
"#,
            bg_class, progress_class, bg_style, slide.html, overlay_html, progress_html, arrow_html
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
"#.replace("[TOTAL]", &total.to_string()).replace("[SLIDE_WIDTH]", &spec.canvas_width.to_string());
    }

    let font_link = if !spec.google_fonts_url.is_empty() {
        format!(
            r#"<link href="{}" rel="stylesheet">"#,
            spec.google_fonts_url
        )
    } else {
        String::new()
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
{}
<style>
{}
</style>
</head>
<body>

{}
<div class="carousel-viewport">
<div class="carousel-track" id="carouselTrack">
{}
</div>
</div>
{}

{}
</body>
</html>"#,
        font_link, css_block, ig_header, slides_html, ig_footer, js_block
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
                font-size: 11.5px !important;
                font-weight: 700 !important;
                letter-spacing: 0.02em !important;
                opacity: 0.9 !important;
            }
            .overlay__topic, .overlay__url, .overlay__hashtags {
                font-family: var(--font-body), sans-serif !important;
                font-style: normal !important;
                text-transform: uppercase !important;
                font-size: 9px !important;
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
                font-size: 11px !important;
                letter-spacing: 0.15em !important;
                opacity: 0.95 !important;
            }
            .overlay__topic, .overlay__url, .overlay__hashtags {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 700 !important;
                text-transform: uppercase !important;
                font-size: 9.5px !important;
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
                font-size: 10px !important;
                letter-spacing: 0.1em !important;
                text-transform: uppercase !important;
                opacity: 0.85 !important;
            }
            .overlay__topic, .overlay__url, .overlay__hashtags {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 500 !important;
                font-size: 9.5px !important;
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
                font-size: 10.5px !important;
                letter-spacing: 0.12em !important;
                text-transform: uppercase !important;
                opacity: 0.9 !important;
            }
            .overlay__topic, .overlay__url, .overlay__hashtags {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 600 !important;
                font-size: 9px !important;
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
                font-size: 11px !important;
                color: var(--primary) !important;
                letter-spacing: 0.08em !important;
                text-transform: uppercase !important;
                opacity: 0.95 !important;
            }
            .overlay__topic, .overlay__url, .overlay__hashtags {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 600 !important;
                font-size: 9.5px !important;
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
                font-size: 11.5px !important;
                letter-spacing: 0.03em !important;
                opacity: 0.9 !important;
            }
            .overlay__topic, .overlay__url, .overlay__hashtags {
                font-family: var(--font-body), sans-serif !important;
                font-weight: 600 !important;
                font-size: 9px !important;
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
        assert!(html.contains("--slide-width: 360px"));
        assert!(html.contains("--slide-height: 480px"));
        assert!(html.contains("width: var(--slide-width)"));
        assert!(html.contains("height: var(--slide-height)"));
    }
}
