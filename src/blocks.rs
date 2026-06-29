use std::collections::HashMap;
use crate::design_system::DesignTokens;

pub fn escape_html(input: &str) -> String {
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

fn format_styles(styles: &[(&str, String)]) -> String {
    styles.iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("; ")
}

pub fn gradient_text(text: &str, gradient: &str, extra_styles: &[(&str, String)]) -> String {
    let mut styles = vec![
        ("background".to_string(), gradient.to_string()),
        ("-webkit-background-clip".to_string(), "text".to_string()),
        ("-webkit-text-fill-color".to_string(), "transparent".to_string()),
        ("background-clip".to_string(), "text".to_string()),
    ];
    for (k, v) in extra_styles {
        styles.push((k.to_string(), v.clone()));
    }
    let css_str = styles.iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("; ");
    format!(r#"<span style="{}">{}</span>"#, css_str, text)
}

pub fn text_block(
    content: &str,
    tokens: &DesignTokens,
    variant: &str,
    color: Option<&str>,
    gradient: bool,
    gradient_colors: Option<(&str, &str)>,
    align: &str,
    max_width: Option<&str>,
    margin: &str,
) -> String {
    let scale = tokens.type_scale.get(variant).unwrap_or_else(|| {
        tokens.type_scale.get("body").unwrap()
    });
    
    let font = if variant == "display" || variant == "headline" || variant == "title" {
        &tokens.heading_font
    } else {
        &tokens.body_font
    };
    
    let text_color = color.unwrap_or(&tokens.text_primary);
    
    let mut styles = vec![
        ("font-family", font.clone()),
        ("font-size", format!("{}px", scale.font_size)),
        ("font-weight", scale.font_weight.to_string()),
        ("line-height", scale.line_height.to_string()),
        ("color", text_color.to_string()),
        ("margin", margin.to_string()),
        ("text-align", align.to_string()),
    ];
    
    if scale.letter_spacing != 0.0 {
        styles.push(("letter-spacing", format!("{}em", scale.letter_spacing)));
    }
    if let Some(mw) = max_width {
        styles.push(("max-width", mw.to_string()));
    }
    
    if gradient {
        if let Some((c1, c2)) = gradient_colors {
            let grad_css = format!("linear-gradient(135deg, {} 0%, {} 100%)", c1, c2);
            let mut extra = vec![
                ("text-align", align.to_string()),
                ("margin", margin.to_string()),
            ];
            if let Some(mw) = max_width {
                extra.push(("max-width", mw.to_string()));
            }
            return gradient_text(content, &grad_css, &extra.iter().map(|(k, v)| (*k, v.clone())).collect::<Vec<_>>());
        }
    }
    
    format!(r#"<p style="{}">{}</p>"#, format_styles(&styles.iter().map(|(k, v)| (*k, v.clone())).collect::<Vec<_>>()), escape_html(content))
}

pub fn heading_block(
    content: &str,
    tokens: &DesignTokens,
    variant: &str,
    color: Option<&str>,
    gradient: bool,
    gradient_colors: Option<(&str, &str)>,
    align: &str,
    margin: &str,
    escape: bool,
) -> String {
    let scale = tokens.type_scale.get(variant).unwrap_or_else(|| {
        tokens.type_scale.get("headline").unwrap()
    });
    
    // Remove HTML tags for length calculation
    let mut plain_content = content.to_string();
    if !escape {
        let re = regex::Regex::new(r"<[^>]*>").unwrap();
        plain_content = re.replace_all(content, "").to_string();
    }
    
    let mut font_size = scale.font_size;
    if plain_content.len() > 60 {
        if let Some(title_scale) = tokens.type_scale.get("title") {
            font_size = title_scale.font_size;
        }
    } else if plain_content.len() > 40 && variant == "display" {
        if let Some(headline_scale) = tokens.type_scale.get("headline") {
            font_size = headline_scale.font_size;
        }
    }
    
    let text_color = color.unwrap_or(&tokens.text_primary);
    
    let mut styles = vec![
        ("font-family", tokens.heading_font.clone()),
        ("font-size", format!("{}px", font_size)),
        ("font-weight", scale.font_weight.to_string()),
        ("line-height", scale.line_height.to_string()),
        ("color", text_color.to_string()),
        ("margin", margin.to_string()),
        ("text-align", align.to_string()),
    ];
    
    if scale.letter_spacing != 0.0 {
        styles.push(("letter-spacing", format!("{}em", scale.letter_spacing)));
    }
    
    if gradient {
        if let Some((c1, c2)) = gradient_colors {
            let grad_css = format!("linear-gradient(135deg, {} 0%, {} 100%)", c1, c2);
            let extra = vec![
                ("text-align", align.to_string()),
                ("margin", margin.to_string()),
            ];
            return gradient_text(content, &grad_css, &extra.iter().map(|(k, v)| (*k, v.clone())).collect::<Vec<_>>());
        }
    }
    
    let display_content = if escape { escape_html(content) } else { content.to_string() };
    format!(r#"<h2 style="{}">{}</h2>"#, format_styles(&styles.iter().map(|(k, v)| (*k, v.clone())).collect::<Vec<_>>()), display_content)
}

pub fn icon_block(
    icon: &str,
    tokens: &DesignTokens,
    color: Option<&str>,
    size: &str,
    font_size: &str,
    margin: &str,
) -> String {
    let accent = color.unwrap_or(&tokens.primary);
    format!(
        r#"<div style="width: {}; height: {}; background: {}12; border-radius: {}; display: flex; align-items: center; justify-content: center; font-size: {}; margin: {};">{}</div>"#,
        size, size, accent, tokens.radii.get("md").unwrap_or(&"10px".to_string()), font_size, margin, escape_html(icon)
    )
}

pub fn stat_block(
    value: &str,
    tokens: &DesignTokens,
    color: Option<&str>,
    font_size: &str,
    margin: &str,
) -> String {
    let accent = color.unwrap_or(&tokens.primary);
    format!(
        r#"<div aria-hidden="true" style="font-family: {}; font-size: {}; font-weight: 800; color: {}; opacity: 0.65; line-height: 1; margin: {};">{}</div>"#,
        tokens.heading_font, font_size, accent, margin, escape_html(value)
    )
}

pub fn quote_block(
    text: &str,
    tokens: &DesignTokens,
    color: Option<&str>,
    font_size: Option<&str>,
    margin: &str,
) -> String {
    let accent = color.unwrap_or(&tokens.primary);
    let scale = tokens.type_scale.get("headline").unwrap_or_else(|| {
        tokens.type_scale.get("body").unwrap()
    });
    
    let mut fs = font_size.map(|s| s.to_string()).unwrap_or_else(|| format!("{}px", scale.font_size));
    if font_size.is_none() {
        if text.len() > 180 {
            if let Some(body_scale) = tokens.type_scale.get("body") {
                fs = format!("{}px", body_scale.font_size);
            }
        } else if text.len() > 120 {
            if let Some(title_scale) = tokens.type_scale.get("title") {
                fs = format!("{}px", title_scale.font_size);
            }
        }
    }
    
    format!(
        r#"<blockquote style="font-family: {}; font-size: {}; font-weight: {}; line-height: {}; color: {}; margin: {}; max-width: 90%; letter-spacing: {}em; border-left: 3px solid {}; padding-left: 20px;">{}</blockquote>"#,
        tokens.heading_font, fs, scale.font_weight, scale.line_height, color.unwrap_or(&tokens.text_primary), margin, scale.letter_spacing, accent, escape_html(text)
    )
}

pub fn badge_block(
    text: &str,
    tokens: &DesignTokens,
    color: Option<&str>,
    margin: &str,
) -> String {
    let accent = color.unwrap_or(&tokens.primary);
    format!(
        r#"<div style="display: inline-block; padding: 6px 16px; background: {}15; border: 1px solid {}30; border-radius: {}; font-family: {}; font-size: 13px; font-weight: 600; color: {}; letter-spacing: 0.05em; text-transform: uppercase; margin: {};">{}</div>"#,
        accent, accent, tokens.radii.get("pill").unwrap_or(&"9999px".to_string()), tokens.body_font, accent, margin, escape_html(text)
    )
}

pub fn button_block(
    text: &str,
    url: &str,
    tokens: Option<&DesignTokens>,
    bg_color: Option<&str>,
    text_color: Option<&str>,
    margin: &str,
) -> String {
    let default_bg = "#1a1a2e";
    let bg = bg_color.unwrap_or_else(|| {
        if let Some(t) = tokens { &t.primary_dark } else { default_bg }
    });
    
    let fg = text_color.unwrap_or("#ffffff");
    let font = if let Some(t) = tokens { &t.body_font } else { "system-ui" };
    let _pill_default = "9999px".to_string();
    let radii = if let Some(t) = tokens { t.radii.get("pill").unwrap_or(&_pill_default) } else { "9999px" };
    let shadow_color = if let Some(t) = tokens { &t.primary_dark } else { "#1a1a2e" };
    
    format!(
        r#"<a href="{}" style="display: inline-block; padding: 16px 40px; background: {}; color: {}; font-family: {}; font-size: 16px; font-weight: 600; border-radius: {}; text-decoration: none; box-shadow: 0 4px 14px {}40; letter-spacing: 0.02em; margin: {};">{}</a>"#,
        escape_html(url), bg, fg, font, radii, shadow_color, margin, escape_html(text)
    )
}

pub fn divider_block(
    tokens: &DesignTokens,
    width: &str,
    height: &str,
    margin: &str,
) -> String {
    format!(
        r#"<div style="width: {}; height: {}; background: {}; border-radius: 2px; margin: {};"></div>"#,
        width, height, tokens.primary, margin
    )
}

pub fn avatar_block(
    name: &str,
    tokens: &DesignTokens,
    color: Option<&str>,
    size: &str,
) -> String {
    let accent = color.unwrap_or(&tokens.primary);
    let initial = if !name.is_empty() { name.chars().next().unwrap().to_uppercase().to_string() } else { "?".to_string() };
    format!(
        r#"<div style="width: {}; height: {}; border-radius: 50%; background: {}20; display: flex; align-items: center; justify-content: center; font-size: 14px; font-weight: 700; color: {}; flex-shrink: 0;">{}</div>"#,
        size, size, accent, accent, initial
    )
}

pub fn attribution_block(
    name: &str,
    role: &str,
    tokens: &DesignTokens,
    color: Option<&str>,
    margin: &str,
    alignment: &str,
) -> String {
    let text_color = color.unwrap_or(&tokens.text_primary);
    let role_text = if !role.is_empty() { format!(" · {}", role) } else { "".to_string() };
    let justify = if alignment == "center" { "center" } else { "flex-start" };
    format!(
        r#"<div style="margin:{};display:flex;align-items:center;gap:12px;justify-content:{};">
            {}
            <div>
                <div style="font-family:{};font-size:{}px;font-weight:600;color:{};">{}</div>
                <div style="font-family:{};font-size:{}px;color:{};">{}</div>
            </div>
        </div>"#,
        margin, justify,
        avatar_block(name, tokens, color, "32px"),
        tokens.body_font, tokens.type_scale.get("caption").map(|s| s.font_size).unwrap_or(12), text_color, escape_html(name),
        tokens.body_font, tokens.type_scale.get("micro").map(|s| s.font_size).unwrap_or(10), tokens.text_secondary, role_text
    )
}

/// Returns a bullet marker span (e.g. ✦ or a number).
pub fn dot_marker(index: Option<usize>, color: &str) -> String {
    match index {
        Some(n) => format!(
            r#"<span style="color:{};font-weight:700;margin-right:12px;font-size:14px;">{}</span>"#,
            color, n + 1
        ),
        None => format!(
            r#"<span style="color:{};margin-right:12px;font-size:12px;line-height:1.5;">✦</span>"#,
            color
        ),
    }
}

pub fn list_item_block(
    label: &str,
    sub: &str,
    tokens: Option<&DesignTokens>,
    marker: &str,
    color: Option<&str>,
    font_size: Option<&str>,
    margin: &str,
) -> String {
    let fs = font_size.map(|s| s.to_string()).unwrap_or_else(|| {
        if let Some(t) = tokens { format!("{}px", t.type_scale.get("body").unwrap().font_size) } else { "15px".to_string() }
    });
    
    let text_color = color.unwrap_or_else(|| {
        if let Some(t) = tokens { &t.text_primary } else { "#0a0a0a" }
    });
    
    let sub_color = if let Some(t) = tokens { &t.text_secondary } else { "#666666" };
    let sub_fs = if let Some(t) = tokens { format!("{}px", t.type_scale.get("caption").unwrap().font_size) } else { "12px".to_string() };
    
    let sub_html = if !sub.is_empty() {
        format!(r#"<div style="font-size:{};color:{};margin-top:4px;">{}</div>"#, sub_fs, sub_color, escape_html(sub))
    } else {
        "".to_string()
    };
    
    let font_family = if let Some(t) = tokens { &t.body_font } else { "system-ui" };
    
    format!(
        r#"<div style="display:flex;align-items:flex-start;margin:{};">
            {}
            <div>
                <div style="font-family:{};font-size:{};font-weight:500;color:{};">{}</div>
                {}
            </div>
        </div>"#,
        margin, marker, font_family, fs, text_color, escape_html(label), sub_html
    )
}
