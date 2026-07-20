use crate::design_system::DesignTokens;
use std::collections::HashMap;

pub fn glass_surface(
    tokens: &DesignTokens,
    variant: &str,
    border_radius: &str,
) -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Default glass properties if the key doesn't exist
    let (bg, blur, border) = match variant {
        "dark" => ("#010105CC", "12px", "1px solid #2A2D3D40"),
        "frosted" => ("#F2F4FFE6", "20px", "1px solid #C4C7D130"),
        _ => ("#F2F4FFCC", "12px", "1px solid #C4C7D140"), // light
    };

    let mut glass_bg = bg.to_string();
    let mut glass_blur = blur.to_string();
    let mut glass_border = border.to_string();

    if let Some(glass_obj) = tokens.glass.as_object() {
        if let Some(v) = glass_obj.get(variant) {
            if let Some(v_obj) = v.as_object() {
                if let Some(b) = v_obj.get("bg").and_then(|x| x.as_str()) {
                    glass_bg = b.to_string();
                }
                if let Some(bl) = v_obj.get("blur").and_then(|x| x.as_str()) {
                    glass_blur = bl.to_string();
                }
                if let Some(bo) = v_obj.get("border").and_then(|x| x.as_str()) {
                    glass_border = bo.to_string();
                }
            }
        }
    }

    map.insert("background".to_string(), glass_bg);
    map.insert(
        "backdrop-filter".to_string(),
        format!("blur({})", glass_blur),
    );
    map.insert(
        "-webkit-backdrop-filter".to_string(),
        format!("blur({})", glass_blur),
    );
    map.insert("border".to_string(), glass_border);
    map.insert("border-radius".to_string(), border_radius.to_string());
    map
}

pub fn noise_overlay(opacity: f32) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("position".to_string(), "absolute".to_string());
    map.insert("inset".to_string(), "0".to_string());
    map.insert("background".to_string(), "var(--texture-grain)".to_string());
    map.insert("opacity".to_string(), opacity.to_string());
    map.insert("pointer-events".to_string(), "none".to_string());
    map.insert("z-index".to_string(), "1".to_string());
    map
}

pub fn mesh_gradient(variant: &str, custom_colors: Option<&[String]>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let bg = if let Some(c) = custom_colors {
        if c.len() >= 3 {
            format!(
                "radial-gradient(at 40% 20%, {}20 0px, transparent 50%), radial-gradient(at 80% 0%, {}18 0px, transparent 50%), radial-gradient(at 0% 50%, {}15 0px, transparent 50%), linear-gradient(180deg, {}08 0%, transparent 100%)",
                c[0], c[1], c[2], c[0]
            )
        } else {
            format!("var(--gradient-{})", variant)
        }
    } else {
        format!("var(--gradient-{})", variant)
    };
    map.insert("background".to_string(), bg);
    map
}

#[allow(dead_code)]
pub fn decorative_grid(cell_size: i32, color: &str, opacity: f32) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let op_hex = format!("{:02X}", (opacity * 255.0).round().clamp(0.0, 255.0) as u8);
    map.insert("position".to_string(), "absolute".to_string());
    map.insert("inset".to_string(), "0".to_string());
    map.insert(
        "background-image".to_string(),
        format!(
            "linear-gradient({}{} 1px, transparent 1px), linear-gradient(90deg, {}{} 1px, transparent 1px)",
            color, op_hex, color, op_hex
        ),
    );
    map.insert(
        "background-size".to_string(),
        format!("{}px {}px", cell_size, cell_size),
    );
    map.insert("pointer-events".to_string(), "none".to_string());
    map
}

#[allow(dead_code)]
pub fn inner_shadow_card(
    tokens: &DesignTokens,
    bg_color: Option<&str>,
    border_radius: &str,
) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let bg = bg_color.unwrap_or(&tokens.surface_light);
    map.insert("background".to_string(), bg.to_string());
    map.insert(
        "box-shadow".to_string(),
        "var(--shadow-lg), var(--shadow-inner)".to_string(),
    );
    map.insert("border-radius".to_string(), border_radius.to_string());
    map
}

pub fn floating_shape(
    shape_type: &str,
    size: i32,
    color: &str,
    x: &str,
    y: &str,
    rotation: i32,
    opacity: f32,
) -> HashMap<String, String> {
    let mut styles = HashMap::new();
    styles.insert("position".to_string(), "absolute".to_string());
    styles.insert("width".to_string(), format!("{}px", size));
    styles.insert("height".to_string(), format!("{}px", size));
    styles.insert("background".to_string(), color.to_string());
    styles.insert("opacity".to_string(), opacity.to_string());
    styles.insert("left".to_string(), x.to_string());
    styles.insert("top".to_string(), y.to_string());
    styles.insert("pointer-events".to_string(), "none".to_string());

    match shape_type {
        "circle" => {
            styles.insert("border-radius".to_string(), "50%".to_string());
        }
        "square" => {
            styles.insert("border-radius".to_string(), "4px".to_string());
        }
        "diamond" => {
            styles.insert("border-radius".to_string(), "4px".to_string());
            styles.insert(
                "transform".to_string(),
                format!("rotate({}deg)", 45 + rotation),
            );
        }
        "pill" => {
            styles.insert("border-radius".to_string(), "9999px".to_string());
        }
        "ring" => {
            styles.insert("border-radius".to_string(), "50%".to_string());
            styles.insert("background".to_string(), "transparent".to_string());
            styles.insert("border".to_string(), format!("3px solid {}", color));
        }
        _ => {}
    }
    styles
}

pub fn slide_background(
    _tokens: &DesignTokens,
    bg_style: &str,
    accent_colors: Option<&[String]>,
) -> String {
    match bg_style {
        "dark" => "var(--surface-dark)".to_string(),
        "mesh" => {
            let mg = mesh_gradient("mesh", accent_colors);
            mg.get("background")
                .cloned()
                .unwrap_or_else(|| "var(--gradient-mesh)".to_string())
        }
        "hero" => "var(--gradient-hero)".to_string(),
        "gradient" => "var(--gradient)".to_string(),
        _ => "var(--surface-light)".to_string(),
    }
}
