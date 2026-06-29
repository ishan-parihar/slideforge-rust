use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

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
}

pub fn render_carousel_html(spec: &CarouselSpec) -> String {
    let slide_width = 420;
    let slide_height = 525;
    
    // Core CSS definitions
    let theme_overrides = get_theme_css_overrides(&spec.visual_theme);
    
    let mut slides_html = String::new();
    for (i, slide) in spec.slides.iter().enumerate() {
        let bg_class = match slide.background.as_str() {
            "dark" => "slide--dark",
            "gradient" => "slide--gradient",
            "mesh" => "slide--mesh",
            "hero" => "slide--hero",
            _ => "slide--light",
        };
        
        let progress_class = if spec.show_progress { "has-progress" } else { "" };
        
        // Progress bar indicators
        let mut progress_bar = String::new();
        if spec.show_progress && spec.slides.len() > 1 {
            let mut steps = String::new();
            for j in 0..spec.slides.len() {
                let active = if j <= i { "active" } else { "" };
                steps.push_str(&format!(
                    r#"<div class="progress-step {}" style="flex: 1; height: 3px; background: rgba(128,128,128,0.2); margin: 0 2px; border-radius: 2px;">
                        <div class="progress-step-fill" style="width: {}; height: 100%; background: var(--primary); transition: width 0.3s ease;"></div>
                    </div>"#,
                    active,
                    if j < i { "100%" } else if j == i { "100%" } else { "0%" }
                ));
            }
            progress_bar = format!(
                r#"<div class="progress-bar-container" style="position: absolute; bottom: 12px; left: 40px; right: 40px; display: flex; align-items: center; z-index: 10;">
                    {}
                </div>"#,
                steps
            );
        }
        
        // Overlay indicators
        let mut header_overlay = String::new();
        if !spec.brand_name.is_empty() {
            header_overlay = format!(
                r#"<div class="overlay__header" style="position: absolute; top: 16px; left: 40px; right: 40px; display: flex; justify-content: space-between; align-items: center; z-index: 10; pointer-events: none;">
                    <span class="overlay__brand" style="color: var(--text-primary); font-size: 11px; font-weight: 700;">{}</span>
                    <span class="overlay__topic" style="color: var(--text-secondary); font-size: 9px; font-weight: 500; opacity: 0.8;">{}</span>
                </div>"#,
                escape_html(&spec.brand_name),
                escape_html(&spec.topic)
            );
        }
        
        let mut footer_overlay = String::new();
        if !spec.brand_handle.is_empty() || !spec.url.is_empty() {
            let handle_text = if !spec.brand_handle.is_empty() { &spec.brand_handle } else { &spec.url };
            let hashtags_text = spec.hashtags.join(" ");
            
            footer_overlay = format!(
                r#"<div class="overlay__footer" style="position: absolute; bottom: 24px; left: 40px; right: 40px; display: flex; justify-content: space-between; align-items: center; z-index: 10; pointer-events: none; font-size: 9px;">
                    <span class="overlay__url" style="color: var(--text-secondary); font-weight: 600;">{}</span>
                    <span class="overlay__hashtags" style="color: var(--primary); font-weight: 600;">{}</span>
                </div>"#,
                escape_html(handle_text),
                escape_html(&hashtags_text)
            );
        }
        
        slides_html.push_str(&format!(
            r#"<div class="slide {} {}">
                {}
                {}
                {}
                {}
            </div>"#,
            bg_class, progress_class,
            slide.html,
            header_overlay,
            progress_bar,
            footer_overlay
        ));
    }
    
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{}</title>
    <link rel="stylesheet" href="{}">
    <style>
        *, *::before, *::after {{ margin: 0; padding: 0; box-sizing: border-box; }}
        {}
        body {{
            display: flex; justify-content: center; align-items: center;
            min-height: 100vh; background: #f0f0f0;
            font-family: var(--font-body, system-ui, sans-serif);
        }}
        .carousel-viewport {{ width: {}px; height: {}px; overflow: hidden; position: relative; }}
        .carousel-track {{ display: flex; height: 100%; transition: transform 0.35s ease; will-change: transform; }}
        .slide {{
            min-width: {}px; height: {}px; position: relative;
            display: flex; flex-direction: column; overflow: hidden;
        }}
        .slide-layout {{
            position: relative; z-index: 2; display: flex; flex-direction: column;
            height: 100%; padding: 80px 48px 80px; width: 100%; justify-content: center;
        }}
        .grid-2 {{ display: flex; gap: 24px; width: 100%; }}
        .grid-2 > * {{ flex: 1; min-width: 0; }}
        .serif {{ font-family: var(--font-heading, serif); }}
        .sans {{ font-family: var(--font-body, sans-serif); }}
        .display-text {{ font-size: 32px; font-weight: 700; line-height: 1.1; letter-spacing: -0.02em; margin-bottom: 16px; }}
        .heading-text {{ font-size: 24px; font-weight: 600; line-height: 1.2; letter-spacing: -0.01em; margin-bottom: 16px; }}
        .body-text {{ font-size: 14px; font-weight: 400; line-height: 1.55; }}
        .caption-text {{ font-size: 11px; font-weight: 500; line-height: 1.4; letter-spacing: 0.02em; }}
        .text-gradient {{
            background: linear-gradient(135deg, var(--text-primary) 0%, var(--primary) 100%);
            -webkit-background-clip: text; -webkit-text-fill-color: transparent;
        }}
        .slide--dark, .slide--gradient, .slide--hero {{
            --text-primary: var(--text-on-dark, #ECEEF5);
            --text-secondary: var(--text-on-dark-secondary, #B9BDC9);
            --border-light: var(--border-dark, #2A2D3D);
            --surface-light: var(--surface-dark, #010105);
        }}
        .card {{
            padding: 20px; border-radius: var(--radius-lg, 12px);
            background: var(--surface-light); border: 1px solid var(--border-light);
        }}
        .glass {{
            background: rgba(255,255,255,0.04); backdrop-filter: blur(12px);
            border: 1px solid rgba(255,255,255,0.08);
        }}
        .slide--light .glass {{ background: rgba(255,255,255,0.7); border: 1px solid rgba(0,0,0,0.06); }}
        {}
    </style>
</head>
<body>
    <div class="carousel-viewport">
        <div class="carousel-track">
            {}
        </div>
    </div>
</body>
</html>"#,
        escape_html(&spec.topic),
        spec.google_fonts_url,
        spec.css_variables,
        slide_width, slide_height,
        slide_width, slide_height,
        theme_overrides,
        slides_html
    )
}

fn get_theme_css_overrides(theme: &str) -> &'static str {
    match theme {
        "editorial" => r#"
            :root {
                --font-heading: 'Playfair Display', Georgia, serif;
                --font-body: 'Inter', system-ui, sans-serif;
                --radius-lg: 8px;
                --radius-md: 6px;
            }
            .slide--light { --surface-light: #F8F6F3; }
            .slide--dark, .slide--hero { --surface-dark: #0A0A0F; }
        "#,
        "bold" => r#"
            :root {
                --font-heading: 'Inter', system-ui, sans-serif;
                --font-body: 'Inter', system-ui, sans-serif;
                --radius-lg: 4px;
                --radius-md: 4px;
            }
            .slide--light { --surface-light: #F0F0F5; }
            .slide--dark, .slide--hero { --surface-dark: #050508; }
        "#,
        "minimal" => r#"
            :root {
                --font-heading: 'Inter', system-ui, sans-serif;
                --font-body: 'Inter', system-ui, sans-serif;
                --radius-lg: 2px;
                --radius-md: 2px;
            }
            .slide--light { --surface-light: #FFFFFF; }
            .slide--dark, .slide--hero { --surface-dark: #111111; }
        "#,
        "dark" => r#"
            :root {
                --font-heading: 'Inter', system-ui, sans-serif;
                --font-body: 'Inter', system-ui, sans-serif;
                --radius-lg: 12px;
                --radius-md: 10px;
            }
            .slide--light { --surface-light: #1A1A2E; }
            .slide--dark, .slide--hero { --surface-dark: #0D0D14; }
        "#,
        "vibrant" => r#"
            :root {
                --font-heading: 'Inter', system-ui, sans-serif;
                --font-body: 'Inter', system-ui, sans-serif;
                --radius-lg: 16px;
                --radius-md: 12px;
            }
            .slide--light { --surface-light: #F5F3FF; }
            .slide--dark, .slide--hero { --surface-dark: #0A0515; }
        "#,
        "natural" => r#"
            :root {
                --font-heading: 'Playfair Display', Georgia, serif;
                --font-body: 'Inter', system-ui, sans-serif;
                --radius-lg: 20px;
                --radius-md: 16px;
            }
            .slide--light { --surface-light: #FAF8F5; }
            .slide--dark, .slide--hero { --surface-dark: #0F0D0A; }
        "#,
        _ => ""
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
