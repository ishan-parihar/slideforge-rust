use indexmap::IndexMap;
use palette::{FromColor, LinSrgb, Oklch, Srgb};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TypeLevel {
    pub font_size: i32,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub font_weight: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FontPairing {
    pub heading_font: String,
    pub body_font: String,
    pub google_fonts_url: String,
    pub heading_class: String,
    pub body_class: String,
    pub style_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContrastReportItem {
    pub ratio: f32,
    pub passes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DesignTokens {
    pub primary: String,
    pub primary_light: String,
    pub primary_dark: String,
    pub surface_light: String,
    pub surface_dark: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_on_dark: String,
    pub text_on_dark_secondary: String,
    pub border_light: String,
    pub border_dark: String,
    pub accent: String,
    pub secondary: String,
    pub tertiary: String,
    pub gradient: String,
    pub temperature: String,
    pub heading_font: String,
    pub body_font: String,
    pub google_fonts_url: String,
    pub type_scale: IndexMap<String, TypeLevel>,
    pub spacing: IndexMap<i32, i32>,
    pub contrast_report: IndexMap<String, ContrastReportItem>,
    pub shadows: IndexMap<String, String>,
    pub radii: IndexMap<String, String>,
    pub gradients: IndexMap<String, String>,
    pub textures: IndexMap<String, String>,
    pub glass: serde_json::Value,
}

impl DesignTokens {
    pub fn to_css_variables(&self) -> String {
        let mut lines = vec![":root {".to_string()];

        let color_tokens = vec![
            ("primary", &self.primary),
            ("primary-light", &self.primary_light),
            ("primary-dark", &self.primary_dark),
            ("surface-light", &self.surface_light),
            ("surface-dark", &self.surface_dark),
            ("text-primary", &self.text_primary),
            ("text-secondary", &self.text_secondary),
            ("text-on-dark", &self.text_on_dark),
            ("text-on-dark-secondary", &self.text_on_dark_secondary),
            ("border-light", &self.border_light),
            ("border-dark", &self.border_dark),
            ("accent", &self.accent),
            ("secondary", &self.secondary),
            ("tertiary", &self.tertiary),
            ("gradient", &self.gradient),
        ];

        for (name, val) in color_tokens {
            lines.push(format!("  --{}: {};", name, val));
        }

        for (k, v) in &self.shadows {
            lines.push(format!("  --shadow-{}: {};", k, v));
        }
        for (k, v) in &self.radii {
            lines.push(format!("  --radius-{}: {};", k, v));
        }
        for (k, v) in &self.gradients {
            lines.push(format!("  --gradient-{}: {};", k, v));
        }
        for (k, v) in &self.textures {
            lines.push(format!("  --texture-{}: {};", k, v));
        }

        if let Some(glass_obj) = self.glass.as_object() {
            for (key, val) in glass_obj {
                if let Some(val_obj) = val.as_object() {
                    for (prop, prop_val) in val_obj {
                        if let Some(s) = prop_val.as_str() {
                            lines.push(format!("  --glass-{}-{}: {};", key, prop, s));
                        }
                    }
                }
            }
        }

        lines.push(format!("  --font-heading: '{}', serif;", self.heading_font));
        lines.push(format!("  --font-body: '{}', sans-serif;", self.body_font));

        for (level, info) in &self.type_scale {
            lines.push(format!("  --text-{}-size: {}px;", level, info.font_size));
            lines.push(format!("  --text-{}-lh: {};", level, info.line_height));
            lines.push(format!("  --text-{}-ls: {}em;", level, info.letter_spacing));
            lines.push(format!("  --text-{}-weight: {};", level, info.font_weight));
        }

        if let Some(micro) = self.type_scale.get("micro") {
            lines.push(format!("  --text-xs: {}px;", micro.font_size));
        }
        if let Some(caption) = self.type_scale.get("caption") {
            lines.push(format!("  --text-sm: {}px;", caption.font_size));
        }
        if let Some(body) = self.type_scale.get("body") {
            lines.push(format!("  --text-base: {}px;", body.font_size));
        }
        if let Some(title) = self.type_scale.get("title") {
            lines.push(format!("  --text-lg: {}px;", title.font_size));
        }
        if let Some(headline) = self.type_scale.get("headline") {
            lines.push(format!("  --text-xl: {}px;", headline.font_size));
        }
        if let Some(display) = self.type_scale.get("display") {
            lines.push(format!("  --text-2xl: {}px;", display.font_size));
        }

        for (step, pixels) in &self.spacing {
            lines.push(format!("  --space-{}: {}px;", step, pixels));
        }

        lines.push("}".to_string());
        lines.join("\n")
    }
}

// Helper color conversions
pub fn parse_hex(hex_str: &str) -> Result<Srgb<f32>, String> {
    let s = hex_str.trim().trim_start_matches('#');
    if s.len() != 6 {
        return Err(format!("Invalid hex string: {}", hex_str));
    }
    let r = u8::from_str_radix(&s[0..2], 16).map_err(|e| e.to_string())?;
    let g = u8::from_str_radix(&s[2..4], 16).map_err(|e| e.to_string())?;
    let b = u8::from_str_radix(&s[4..6], 16).map_err(|e| e.to_string())?;
    Ok(Srgb::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
    ))
}

pub fn to_hex(color: &Srgb<f32>) -> String {
    let r = (color.red * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = (color.green * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = (color.blue * 255.0).round().clamp(0.0, 255.0) as u8;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn oklch_to_hex(l: f32, c: f32, h_deg: f32) -> String {
    let oklch = Oklch::new(l, c, h_deg);
    let srgb = Srgb::from_color(oklch);
    to_hex(&srgb)
}

pub fn hex_to_oklch(hex_str: &str) -> Result<(f32, f32, f32), String> {
    let srgb = parse_hex(hex_str)?;
    let oklch = Oklch::from_color(srgb);
    let hue_deg: f32 = oklch.hue.into_degrees();
    Ok((oklch.l, oklch.chroma, hue_deg))
}

pub fn relative_luminance(hex_str: &str) -> f32 {
    if let Ok(rgb) = parse_hex(hex_str) {
        let lin: LinSrgb<f32> = LinSrgb::from_color(rgb);
        0.2126 * lin.red + 0.7152 * lin.green + 0.0722 * lin.blue
    } else {
        0.0
    }
}

pub fn contrast_ratio(hex1: &str, hex2: &str) -> f32 {
    let l1 = relative_luminance(hex1);
    let l2 = relative_luminance(hex2);
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    (lighter + 0.05) / (darker + 0.05)
}

pub fn passes_wcag_aa(fg_hex: &str, bg_hex: &str, large_text: bool) -> bool {
    let threshold = if large_text { 3.0 } else { 4.5 };
    contrast_ratio(fg_hex, bg_hex) >= threshold
}

// Clamping functions
pub fn auto_clamp_text(
    base_l: f32,
    c: f32,
    h: f32,
    bg_hex: &str,
    direction: &str,
    target_ratio: f32,
) -> String {
    let mut current_l = base_l;
    let step = 0.01;
    let min_l = 0.05;
    let max_l = 0.97;

    for _ in 0..100 {
        let color_hex = oklch_to_hex(current_l, c, h);
        if contrast_ratio(&color_hex, bg_hex) >= target_ratio {
            return color_hex;
        }
        if direction == "darken" {
            current_l -= step;
            if current_l < min_l {
                break;
            }
        } else {
            current_l += step;
            if current_l > max_l {
                break;
            }
        }
    }
    oklch_to_hex(current_l, c, h)
}

pub fn auto_clamp_border(
    base_l: f32,
    c: f32,
    h: f32,
    bg_hex: &str,
    min_ratio: f32,
    direction: &str,
) -> String {
    auto_clamp_text(base_l, c, h, bg_hex, direction, min_ratio)
}

// Spacing & Font mappings
pub fn generate_spacing_scale() -> IndexMap<i32, i32> {
    let base = 8;
    let mut map = IndexMap::new();
    let steps = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12];
    for step in steps {
        map.insert(step, base * step);
    }
    map
}

pub fn generate_type_scale(base_size: i32, ratio: f32) -> IndexMap<String, TypeLevel> {
    let mut map = IndexMap::new();
    map.insert(
        "display".to_string(),
        TypeLevel {
            font_size: (base_size as f32 * ratio.powi(3)).round() as i32,
            line_height: 1.08,
            letter_spacing: -0.02,
            font_weight: 600,
        },
    );
    map.insert(
        "headline".to_string(),
        TypeLevel {
            font_size: (base_size as f32 * ratio.powi(2)).round() as i32,
            line_height: 1.12,
            letter_spacing: -0.015,
            font_weight: 600,
        },
    );
    map.insert(
        "title".to_string(),
        TypeLevel {
            font_size: (base_size as f32 * ratio.powi(1)).round() as i32,
            line_height: 1.18,
            letter_spacing: -0.01,
            font_weight: 600,
        },
    );
    map.insert(
        "body".to_string(),
        TypeLevel {
            font_size: base_size,
            line_height: 1.55,
            letter_spacing: 0.0,
            font_weight: 400,
        },
    );
    map.insert(
        "caption".to_string(),
        TypeLevel {
            font_size: (base_size as f32 / ratio).round() as i32,
            line_height: 1.40,
            letter_spacing: 0.02,
            font_weight: 600,
        },
    );
    map.insert(
        "micro".to_string(),
        TypeLevel {
            font_size: (base_size as f32 / ratio.powi(2)).round() as i32,
            line_height: 1.35,
            letter_spacing: 0.04,
            font_weight: 500,
        },
    );
    map
}

pub fn get_font_pairing(style: &str) -> FontPairing {
    let style_clean = style.to_lowercase().trim().to_string();

    let (h_font, b_font, h_weights, b_weights, desc, h_class, b_class) = match style_clean.as_str()
    {
        "editorial" => (
            "Playfair Display",
            "DM Sans",
            vec![300, 600],
            vec![400, 500, 600],
            "Editorial / premium feel",
            "serif",
            "sans",
        ),
        "warm" => (
            "Lora",
            "Nunito Sans",
            vec![400, 600],
            vec![400, 500, 600],
            "Warm / approachable",
            "serif",
            "sans",
        ),
        "technical" => (
            "Space Grotesk",
            "Space Grotesk",
            vec![300, 600],
            vec![400, 500],
            "Technical / sharp",
            "sans",
            "sans",
        ),
        "bold" => (
            "Fraunces",
            "Outfit",
            vec![300, 600],
            vec![400, 500, 600],
            "Bold / expressive",
            "serif",
            "sans",
        ),
        "classic" => (
            "Libre Baskerville",
            "Work Sans",
            vec![400, 700],
            vec![400, 500, 600],
            "Classic / trustworthy",
            "serif",
            "sans",
        ),
        "rounded" => (
            "Bricolage Grotesque",
            "Bricolage Grotesque",
            vec![600],
            vec![400, 500],
            "Rounded / friendly",
            "sans",
            "sans",
        ),
        _ => (
            "Plus Jakarta Sans",
            "Plus Jakarta Sans",
            vec![700],
            vec![400, 500, 600],
            "Modern / clean",
            "sans",
            "sans",
        ), // modern
    };

    let mut heading_families = vec![format!(
        "{}:wght@{}",
        h_font,
        h_weights
            .iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )];
    if h_font != b_font {
        heading_families.push(format!(
            "{}:wght@{}",
            b_font,
            b_weights
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ));
    }
    let families = heading_families.join("&family=");
    let url = format!(
        "https://fonts.googleapis.com/css2?family={}&display=swap",
        families
    );

    FontPairing {
        heading_font: h_font.to_string(),
        body_font: b_font.to_string(),
        google_fonts_url: url,
        heading_class: h_class.to_string(),
        body_class: b_class.to_string(),
        style_description: desc.to_string(),
    }
}

pub fn rotate_hue_piecewise(hue: f32, breakpoints: &[f32], rotations: &[f32]) -> f32 {
    for i in 0..breakpoints.len() - 1 {
        if breakpoints[i] <= hue && hue < breakpoints[i + 1] {
            let progress = (hue - breakpoints[i]) / (breakpoints[i + 1] - breakpoints[i]);
            return (hue + rotations[i] + (rotations[i + 1] - rotations[i]) * progress) % 360.0;
        }
    }
    hue
}

pub fn derive_palette(
    primary_hex: &str,
    style: &str,
    type_scale_base: i32,
    type_scale_ratio: f32,
    preset: &str,
    visual_theme: &str,
    overrides: Option<&IndexMap<String, String>>,
    secondary_hex: Option<&str>,
    tertiary_hex: Option<&str>,
) -> Result<DesignTokens, String> {
    derive_palette_with_canvas(
        primary_hex,
        style,
        type_scale_base,
        type_scale_ratio,
        preset,
        visual_theme,
        overrides,
        secondary_hex,
        tertiary_hex,
        1080, // default width
        1350, // default height (4:5 aspect ratio)
    )
}

pub fn derive_palette_with_canvas(
    primary_hex: &str,
    style: &str,
    type_scale_base: i32,
    type_scale_ratio: f32,
    preset: &str,
    visual_theme: &str,
    overrides: Option<&IndexMap<String, String>>,
    secondary_hex: Option<&str>,
    tertiary_hex: Option<&str>,
    canvas_width: u32,
    canvas_height: u32,
) -> Result<DesignTokens, String> {
    let primary = parse_hex(primary_hex)?;
    let mut primary_str = to_hex(&primary);

    let (mut pl, mut pc, mut ph) = hex_to_oklch(&primary_str)?;

    let theme_offset = match visual_theme {
        "editorial" => 0.0,
        "bold" => 30.0,
        "minimal" => -20.0,
        "dark" => 10.0,
        "vibrant" => 60.0,
        "natural" => 150.0,
        _ => 0.0,
    };

    if visual_theme != "" {
        ph = (ph + theme_offset) % 360.0;
        primary_str = oklch_to_hex(pl, pc, ph);
        let res = hex_to_oklch(&primary_str)?;
        pl = res.0;
        pc = res.1;
        ph = res.2;
    }

    // Preset configuration details
    let mut sec_hue_offset = 0.0;
    let mut sec_chroma_scale = 0.44;
    let mut tert_hue_offset = 60.0;
    let mut tert_chroma_scale = 0.67;
    let mut is_mono = false;
    let mut is_vibrant_preset = false;

    match preset {
        "neutral" => {
            sec_chroma_scale = 0.22;
            tert_hue_offset = 0.0;
            tert_chroma_scale = 0.44;
        }
        "monochrome" => {
            is_mono = true;
        }
        "expressive" => {
            sec_hue_offset = 120.0;
            sec_chroma_scale = 0.56;
            tert_hue_offset = 60.0;
            tert_chroma_scale = 0.78;
        }
        "rainbow" => {
            sec_hue_offset = 90.0;
            sec_chroma_scale = 0.89;
            tert_hue_offset = 180.0;
            tert_chroma_scale = 0.89;
        }
        "fruit_salad" => {
            sec_hue_offset = 150.0;
            sec_chroma_scale = 0.67;
            tert_hue_offset = 30.0;
            tert_chroma_scale = 0.56;
        }
        "content" => {
            sec_chroma_scale = 0.33;
            tert_hue_offset = 60.0;
            tert_chroma_scale = 0.56;
        }
        "fidelity" => {
            sec_chroma_scale = 0.22;
            tert_hue_offset = 30.0;
            tert_chroma_scale = 0.33;
        }
        "vibrant" => {
            is_vibrant_preset = true;
        }
        _ => {} // tonal_spot default
    };

    let secondary = if let Some(sec) = secondary_hex {
        parse_hex(sec)?;
        sec.to_string()
    } else {
        if is_mono {
            oklch_to_hex(pl, 0.0, ph)
        } else if is_vibrant_preset {
            let breakpoints = vec![0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0];
            let rotations = vec![18.0, 15.0, 10.0, 12.0, 15.0, 18.0, 15.0, 12.0, 12.0];
            let sh = rotate_hue_piecewise(ph, &breakpoints, &rotations);
            oklch_to_hex(pl, 0.06, sh)
        } else {
            let sh = (ph + sec_hue_offset) % 360.0;
            let sc = pc * sec_chroma_scale;
            oklch_to_hex(pl, sc, sh)
        }
    };

    let tertiary = if let Some(tert) = tertiary_hex {
        parse_hex(tert)?;
        tert.to_string()
    } else {
        if is_mono {
            oklch_to_hex(pl, 0.0, ph)
        } else if is_vibrant_preset {
            let breakpoints = vec![0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0];
            let rotations = vec![35.0, 30.0, 20.0, 25.0, 30.0, 35.0, 30.0, 25.0, 25.0];
            let th = rotate_hue_piecewise(ph, &breakpoints, &rotations);
            oklch_to_hex(pl, 0.09, th)
        } else {
            let th = (ph + tert_hue_offset) % 360.0;
            let tc = pc * tert_chroma_scale;
            oklch_to_hex(pl, tc, th)
        }
    };

    let (_, _, sh) = hex_to_oklch(&secondary)?;
    let surface_light = oklch_to_hex(0.97, 0.015, sh);
    let surface_dark = oklch_to_hex(0.08, 0.02, sh);

    if contrast_ratio(&primary_str, &surface_light) < 4.5 {
        primary_str = auto_clamp_text(pl, pc, ph, &surface_light, "darken", 4.5);
        let res = hex_to_oklch(&primary_str)?;
        pl = res.0;
        pc = res.1;
        ph = res.2;
    }

    let mut primary_light = oklch_to_hex((pl + 0.15).min(0.97), pc, ph);
    if contrast_ratio(&primary_light, &surface_dark) < 4.5 {
        primary_light =
            auto_clamp_text((pl + 0.15).min(0.97), pc, ph, &surface_dark, "lighten", 4.5);
    }

    let mut primary_dark = oklch_to_hex((pl - 0.15).max(0.05), pc, ph);
    if contrast_ratio(&primary_dark, &surface_light) < 4.5 {
        primary_dark =
            auto_clamp_text((pl - 0.15).max(0.05), pc, ph, &surface_light, "darken", 4.5);
    }

    if contrast_ratio(&primary_light, &primary_dark) < 4.5 {
        primary_light =
            auto_clamp_text((pl + 0.15).min(0.97), pc, ph, &primary_dark, "lighten", 4.5);
    }

    let text_primary = oklch_to_hex(0.15, 0.01, ph);

    let text_secondary = auto_clamp_text(0.38, 0.015, ph, &surface_light, "darken", 6.0);
    let text_on_dark = oklch_to_hex(0.95, 0.01, ph);
    let text_on_dark_secondary = auto_clamp_text(0.85, 0.015, ph, &surface_dark, "lighten", 6.0);

    let border_light = auto_clamp_border(0.85, 0.015, ph, &surface_light, 1.5, "darken");
    let border_dark = auto_clamp_border(0.22, 0.03, ph, &surface_dark, 1.5, "lighten");

    let accent = oklch_to_hex(pl, pc, (ph + 180.0) % 360.0);
    let gradient = format!(
        "linear-gradient(165deg, {} 0%, {} 60%, {} 100%)",
        surface_dark, primary_dark, surface_dark
    );
    let temperature = if ph >= 180.0 {
        "warm".to_string()
    } else {
        "cool".to_string()
    };

    // Compile contrast report
    let mut contrast_report = IndexMap::new();
    contrast_report.insert(
        "text_primary on surface_light".to_string(),
        ContrastReportItem {
            ratio: (contrast_ratio(&text_primary, &surface_light) * 100.0).round() / 100.0,
            passes: passes_wcag_aa(&text_primary, &surface_light, false),
        },
    );
    contrast_report.insert(
        "text_secondary on surface_light".to_string(),
        ContrastReportItem {
            ratio: (contrast_ratio(&text_secondary, &surface_light) * 100.0).round() / 100.0,
            passes: passes_wcag_aa(&text_secondary, &surface_light, false),
        },
    );
    contrast_report.insert(
        "text_on_dark on surface_dark".to_string(),
        ContrastReportItem {
            ratio: (contrast_ratio(&text_on_dark, &surface_dark) * 100.0).round() / 100.0,
            passes: passes_wcag_aa(&text_on_dark, &surface_dark, false),
        },
    );
    contrast_report.insert(
        "text_on_dark_secondary on surface_dark".to_string(),
        ContrastReportItem {
            ratio: (contrast_ratio(&text_on_dark_secondary, &surface_dark) * 100.0).round() / 100.0,
            passes: passes_wcag_aa(&text_on_dark_secondary, &surface_dark, false),
        },
    );
    contrast_report.insert(
        "border_light on surface_light".to_string(),
        ContrastReportItem {
            ratio: (contrast_ratio(&border_light, &surface_light) * 100.0).round() / 100.0,
            passes: contrast_ratio(&border_light, &surface_light) >= 1.5,
        },
    );
    contrast_report.insert(
        "border_dark on surface_dark".to_string(),
        ContrastReportItem {
            ratio: (contrast_ratio(&border_dark, &surface_dark) * 100.0).round() / 100.0,
            passes: contrast_ratio(&border_dark, &surface_dark) >= 1.5,
        },
    );

    let fonts = get_font_pairing(style);
    
    // Calculate scaling factor based on canvas dimensions
    // Original system used 420x525 with fixed fonts (32px display, 24px heading, 14px body)
    // Apply linear scaling based on width to maintain component proportions
    let original_width = 420.0;
    let target_width = canvas_width as f32;
    let linear_scale = target_width / original_width;
    
    // Scale type scale and spacing based on canvas size
    let scaled_type_scale_base = (type_scale_base as f32 * linear_scale).round() as i32;
    let type_scale = generate_type_scale(scaled_type_scale_base, type_scale_ratio);
    
    // Scale spacing proportionally
    let base_spacing = generate_spacing_scale();
    let mut spacing = IndexMap::new();
    for (step, pixels) in &base_spacing {
        let scaled_pixels = ((*pixels as f32) * linear_scale).round() as i32;
        spacing.insert(step.clone(), scaled_pixels);
    }

    let mut shadows = IndexMap::new();
    let scale_value = |val: f32| -> i32 {
        (val * linear_scale).round() as i32
    };
    
    // Scale shadow values
    shadows.insert("sm".to_string(), format!("0 {}px 2px rgba(0,0,0,0.05)", scale_value(1.0)));
    shadows.insert(
        "md".to_string(),
        format!("0 {}px {}px -1px rgba(0,0,0,0.08), 0 {}px {}px -2px rgba(0,0,0,0.05)", 
                scale_value(4.0), scale_value(6.0), scale_value(2.0), scale_value(4.0)),
    );
    shadows.insert(
        "lg".to_string(),
        format!("0 {}px {}px -3px rgba(0,0,0,0.08), 0 {}px {}px -4px rgba(0,0,0,0.05)",
                scale_value(10.0), scale_value(15.0), scale_value(4.0), scale_value(6.0)),
    );
    shadows.insert(
        "xl".to_string(),
        format!("0 {}px {}px -5px rgba(0,0,0,0.1), 0 {}px {}px -6px rgba(0,0,0,0.05)",
                scale_value(20.0), scale_value(25.0), scale_value(8.0), scale_value(10.0)),
    );
    shadows.insert(
        "inner".to_string(),
        format!("inset 0 {}px 2px 4px rgba(0,0,0,0.06)", scale_value(2.0)),
    );
    shadows.insert("glow".to_string(), format!("0 0 {}px {}40", scale_value(20.0), primary_str));

    let mut radii = IndexMap::new();
    let scale_radius = |val: f32| -> String {
        if val > 100.0 { // Keep pill radius as is
            format!("{}px", val as i32)
        } else {
            let scaled = (val * linear_scale).round() as i32;
            format!("{}px", scaled.max(1)) // Ensure minimum 1px
        }
    };
    
    radii.insert("sm".to_string(), scale_radius(6.0));
    radii.insert("md".to_string(), scale_radius(10.0));
    radii.insert("lg".to_string(), scale_radius(16.0));
    radii.insert("xl".to_string(), scale_radius(24.0));
    radii.insert("pill".to_string(), "9999px".to_string());

    let mut gradients_map = IndexMap::new();
    gradients_map.insert(
        "subtle".to_string(),
        format!(
            "linear-gradient(135deg, {} 0%, {}08 100%)",
            surface_light, primary_str
        ),
    );
    gradients_map.insert("mesh".to_string(), format!("radial-gradient(at 40% 20%, {}15 0px, transparent 50%), radial-gradient(at 80% 0%, {}10 0px, transparent 50%), radial-gradient(at 0% 50%, {}12 0px, transparent 50%)", primary_str, accent, primary_light));
    gradients_map.insert(
        "hero".to_string(),
        format!(
            "linear-gradient(135deg, {} 0%, {}30 50%, {} 100%)",
            surface_dark, primary_dark, surface_dark
        ),
    );
    gradients_map.insert("noise".to_string(), "url(\"data:image/svg+xml,%3Csvg viewBox='0 0 256 256' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.65' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)' opacity='0.03'/%3E%3C/svg%3E\")".to_string());

    let mut textures = IndexMap::new();
    textures.insert("grain".to_string(), "url(\"data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='g'%3E%3CfeTurbulence type='turbulence' baseFrequency='0.85' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23g)' opacity='0.04'/%3E%3C/svg%3E\")".to_string());

    let glass = serde_json::json!({
        "light": { "blur": "12px", "bg": format!("{}CC", surface_light), "border": format!("1px solid {}40", border_light) },
        "dark": { "blur": "12px", "bg": format!("{}CC", surface_dark), "border": format!("1px solid {}40", border_dark) },
        "frosted": { "blur": "20px", "bg": format!("{}E6", surface_light), "border": format!("1px solid {}30", border_light) }
    });

    let mut tokens = DesignTokens {
        primary: primary_str,
        primary_light,
        primary_dark,
        surface_light,
        surface_dark,
        text_primary,
        text_secondary,
        text_on_dark,
        text_on_dark_secondary,
        border_light,
        border_dark,
        accent,
        secondary,
        tertiary,
        gradient,
        temperature,
        heading_font: fonts.heading_font,
        body_font: fonts.body_font,
        google_fonts_url: fonts.google_fonts_url,
        type_scale,
        spacing,
        contrast_report,
        shadows,
        radii,
        gradients: gradients_map,
        textures,
        glass,
    };

    // Apply overrides if any
    if let Some(over) = overrides {
        for (k, v) in over {
            let normalized = if v.starts_with("#") || v.len() == 6 {
                if let Ok(c) = parse_hex(v) {
                    to_hex(&c)
                } else {
                    v.to_string()
                }
            } else {
                v.to_string()
            };

            match k.as_str() {
                "primary" => tokens.primary = normalized,
                "primary_light" => tokens.primary_light = normalized,
                "primary_dark" => tokens.primary_dark = normalized,
                "surface_light" => tokens.surface_light = normalized,
                "surface_dark" => tokens.surface_dark = normalized,
                "text_primary" => tokens.text_primary = normalized,
                "text_secondary" => tokens.text_secondary = normalized,
                "text_on_dark" => tokens.text_on_dark = normalized,
                "text_on_dark_secondary" => tokens.text_on_dark_secondary = normalized,
                "border_light" => tokens.border_light = normalized,
                "border_dark" => tokens.border_dark = normalized,
                "accent" => tokens.accent = normalized,
                "secondary" => tokens.secondary = normalized,
                "tertiary" => tokens.tertiary = normalized,
                _ => {}
            }
        }
        // Revalidate token overrides
        tokens = revalidate_tokens(tokens)?;
    }

    Ok(tokens)
}

fn revalidate_tokens(mut tokens: DesignTokens) -> Result<DesignTokens, String> {
    let text_bg_pairs = vec![
        ("text_primary", "surface_light", false),
        ("text_secondary", "surface_light", false),
        ("text_on_dark", "surface_dark", false),
        ("text_on_dark_secondary", "surface_dark", false),
        ("primary", "surface_light", false),
        ("primary_dark", "surface_light", false),
        ("primary_light", "surface_dark", false),
        ("primary_light", "primary_dark", false),
    ];

    for (text_field, bg_field, is_large) in text_bg_pairs {
        let fg_hex = match text_field {
            "text_primary" => &tokens.text_primary,
            "text_secondary" => &tokens.text_secondary,
            "text_on_dark" => &tokens.text_on_dark,
            "text_on_dark_secondary" => &tokens.text_on_dark_secondary,
            "primary" => &tokens.primary,
            "primary_dark" => &tokens.primary_dark,
            "primary_light" => &tokens.primary_light,
            _ => &tokens.primary,
        }
        .clone();

        let bg_hex = match bg_field {
            "surface_light" => &tokens.surface_light,
            "surface_dark" => &tokens.surface_dark,
            "primary_dark" => &tokens.primary_dark,
            _ => &tokens.surface_light,
        }
        .clone();

        let threshold = if is_large { 3.0 } else { 4.5 };
        if contrast_ratio(&fg_hex, &bg_hex) < threshold {
            let direction = if bg_hex == tokens.surface_light {
                "darken"
            } else {
                "lighten"
            };
            let (l, c, h) = hex_to_oklch(&fg_hex)?;
            let fixed = auto_clamp_text(l, c, h, &bg_hex, direction, threshold);
            match text_field {
                "text_primary" => tokens.text_primary = fixed,
                "text_secondary" => tokens.text_secondary = fixed,
                "text_on_dark" => tokens.text_on_dark = fixed,
                "text_on_dark_secondary" => tokens.text_on_dark_secondary = fixed,
                "primary" => tokens.primary = fixed,
                "primary_dark" => tokens.primary_dark = fixed,
                "primary_light" => tokens.primary_light = fixed,
                _ => {}
            }
        }
    }
    Ok(tokens)
}

pub fn blend_alpha(fg_hex: &str, alpha: f32, bg_hex: &str) -> String {
    let fg = parse_hex(fg_hex).unwrap_or(Srgb::new(0.0, 0.0, 0.0));
    let bg = parse_hex(bg_hex).unwrap_or(Srgb::new(1.0, 1.0, 1.0));
    let r = fg.red * alpha + bg.red * (1.0 - alpha);
    let g = fg.green * alpha + bg.green * (1.0 - alpha);
    let b = fg.blue * alpha + bg.blue * (1.0 - alpha);
    to_hex(&Srgb::new(r, g, b))
}

pub fn get_contrast_safe_color(
    fg_hex: &str,
    bg_colors: &[String],
    target_ratio: f32,
) -> Result<String, String> {
    let mut resolved_bg_list = Vec::new();
    let hex_re = regex::Regex::new(r"#([0-9a-fA-F]{6,8})").unwrap();

    for bg in bg_colors {
        let bg_clean = bg.trim();
        if let Some(caps) = hex_re.captures(bg_clean) {
            let h = caps.get(1).unwrap().as_str();
            let mut hex_color = format!("#{}", &h[..6].to_uppercase());
            if h.len() == 8 {
                let alpha = u8::from_str_radix(&h[6..8], 16).unwrap_or(255) as f32 / 255.0;
                let fallback_bg = if bg_clean.contains("dark")
                    || bg_clean.contains("gradient")
                    || bg_clean.contains("hero")
                {
                    "#000000"
                } else {
                    "#FFFFFF"
                };
                hex_color = blend_alpha(&hex_color, alpha, fallback_bg);
            }
            resolved_bg_list.push(hex_color);
        } else if bg_clean.starts_with("#") {
            resolved_bg_list.push(bg_clean.to_string());
        }
    }

    if resolved_bg_list.is_empty() {
        resolved_bg_list.push("#FFFFFF".to_string());
    }

    let (l, c, h) = hex_to_oklch(fg_hex)?;

    // Determine direction
    let max_light_contrast = resolved_bg_list
        .iter()
        .map(|bg| contrast_ratio("#FFFFFF", bg))
        .fold(f32::INFINITY, f32::min);
    let max_dark_contrast = resolved_bg_list
        .iter()
        .map(|bg| contrast_ratio("#000000", bg))
        .fold(f32::INFINITY, f32::min);

    let direction = if max_light_contrast > max_dark_contrast {
        "lighten"
    } else {
        "darken"
    };

    let mut current_l = l;
    let step = 0.01;
    let min_l = 0.05;
    let max_l = 0.97;

    for _ in 0..100 {
        let color_hex = oklch_to_hex(current_l, c, h);
        let worst_ratio = resolved_bg_list
            .iter()
            .map(|bg| contrast_ratio(&color_hex, bg))
            .fold(f32::INFINITY, f32::min);

        if worst_ratio >= target_ratio {
            return Ok(color_hex);
        }

        if direction == "darken" {
            current_l -= step;
            if current_l < min_l {
                break;
            }
        } else {
            current_l += step;
            if current_l > max_l {
                break;
            }
        }
    }

    Ok(oklch_to_hex(current_l, c, h))
}
