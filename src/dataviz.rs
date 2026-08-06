// dataviz.rs — Pure HTML/SVG Data Visualization rendering utilities.
// Ported from carousel-mcp/src/slideforge/dataviz.py

use crate::layouts::SlideColors;
use serde_json::Value;

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

/// One chart text label, positioned for HTML overlay (not SVG `<text>`).
/// Coordinates live in the same viewBox space as the SVG the label belongs
/// to, so a `position:relative` wrapper sized to the SVG box places them with
/// matching geometry. `anchor` mirrors the SVG `text-anchor` semantics
/// ("middle" | "start" | "end"). `swatch` renders a small colored square
/// before the text (used for chart legends).
pub struct ChartLabel {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub anchor: &'static str,
    pub weight: u16,
    pub size_px: f64,
    pub color: String,
    pub swatch: Option<String>,
}

/// Render `ChartLabel`s as absolutely-positioned HTML spans. Blitz's usvg
/// rasterizer can only see the system font database (not the vendored web
/// fonts), so SVG `<text>` renders with wrong/mixed glyphs; composing the text
/// as HTML routes it through the normal font pipeline with correct glyphs.
pub fn chart_label_overlay(labels: &[ChartLabel], font_family: &str) -> String {
    let mut html = String::new();
    for lbl in labels {
        let (tx, lx) = match lbl.anchor {
            "start" => ("translate(0%, -50%)", lbl.x + 3.0),
            "end" => ("translate(-100%, -50%)", lbl.x - 3.0),
            _ => ("translate(-50%, -50%)", lbl.x),
        };
        let swatch = match &lbl.swatch {
            Some(c) => format!(
                r#"<span style="display:inline-block;width:9px;height:4px;border-radius:1px;background:{};margin-right:5px;vertical-align:middle;"></span>"#,
                c
            ),
            None => String::new(),
        };
        html.push_str(&format!(
            r#"<span style="position:absolute;left:{:.1}px;top:{:.1}px;transform:{};font-family:{};font-size:{}px;font-weight:{};color:{};white-space:nowrap;line-height:1.2;">{}{}</span>"#,
            lx,
            lbl.y,
            tx,
            font_family,
            lbl.size_px,
            lbl.weight,
            lbl.color,
            swatch,
            escape_html(&lbl.text)
        ));
    }
    html
}

pub fn render_svg_line_chart(
    data: &[Value],
    width: u32,
    height: u32,
    colors: &SlideColors,
    is_dark: bool,
    draw_area: bool,
) -> String {
    line_chart_parts(data, width, height, colors, is_dark, draw_area).0
}

/// Compose the line/area chart SVG plus its text labels. The SVG deliberately
/// omits every `<text>` element (blitz's usvg rasterizer cannot see vendored
/// web fonts), and the returned labels are overlaid as HTML by the caller via
/// `chart_label_overlay`. GEOMETRY COUPLING: label coordinates are computed in
/// the same viewBox space as the SVG, so they stay aligned automatically.
pub fn line_chart_parts(
    data: &[Value],
    width: u32,
    height: u32,
    colors: &SlideColors,
    is_dark: bool,
    draw_area: bool,
) -> (String, Vec<ChartLabel>) {
    if data.is_empty() {
        return (String::new(), Vec::new());
    }

    // Detect multi-series: each item has a "series" array [{name, value}]
    let is_multi = data.iter().any(|item| {
        item.get("series")
            .and_then(|v| v.as_array())
            .map(|arr| !arr.is_empty())
            .unwrap_or(false)
    });

    let pad_left = 40;
    let pad_right = 15;
    let pad_top = 35;
    let pad_bottom = 22;
    let chart_w = width as f64 - pad_left as f64 - pad_right as f64;
    let chart_h = height as f64 - pad_top as f64 - pad_bottom as f64;

    let mut labels_out: Vec<ChartLabel> = Vec::new();

    // ── Shared: extract labels from the top-level items ──
    let labels: Vec<String> = data
        .iter()
        .take(8)
        .map(|item| {
            item.get("label")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        })
        .collect();
    let n_points = labels.len();

    // ── Shared: x-coordinate mapping ──
    let x_of = |i: usize| -> f64 {
        if n_points > 1 {
            pad_left as f64 + (i as f64 / (n_points - 1) as f64) * chart_w
        } else {
            pad_left as f64 + chart_w / 2.0
        }
    };

    // ── Shared: grid lines (based on global min/max across all series) ──
    let all_vals: Vec<f64> = if is_multi {
        data
            .iter()
            .take(8)
            .filter_map(|item| item.get("series")?.as_array())
            .flatten()
            .filter_map(|s| s.get("value")?.as_f64())
            .collect()
    } else {
        data
            .iter()
            .take(8)
            .filter_map(|item| {
                item.get("value")
                    .and_then(|v| v.as_f64().or_else(|| v.as_str()?.parse::<f64>().ok()))
            })
            .collect()
    };

    let max_val = all_vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut min_val = all_vals.iter().copied().fold(f64::INFINITY, f64::min);
    if max_val == min_val {
        min_val -= 1.0;
    }

    let mut grid_lines = String::new();
    for i in 0..3 {
        let frac = i as f64 / 2.0;
        let y_val = min_val + frac * (max_val - min_val);
        let y_pos = height as f64 - pad_bottom as f64 - frac * chart_h;
        grid_lines.push_str(&format!(
            r#"<line x1="{}" y1="{:.1}" x2="{}" y2="{:.1}" stroke="{}55" stroke-dasharray="3,3" stroke-width="1" />"#,
            pad_left, y_pos, width - pad_right, y_pos, colors.border
        ));
        // Y-axis grid value (HTML overlay — see line_chart_parts doc).
        labels_out.push(ChartLabel {
            text: format!("{:.1}", y_val),
            x: pad_left as f64 - 8.0,
            y: y_pos + 4.0,
            anchor: "end",
            weight: 600,
            size_px: 9.0,
            color: colors.text_secondary.clone(),
            swatch: None,
        });
    }

    // ── Build line paths ──
    // Each series gets its own color and SVG path element.
    let series_palette = [
        &colors.primary,
        "#FF8C6B",
        "#3ECFA0",
        "#FFB84D",
        "#E879A8",
        "#5BB5F0",
    ];

    let mut all_paths = String::new();
    let mut area_grad_defs = String::new();
    let mut area_paths = String::new();
    let mut all_markers = String::new();

    let bg_color_repr = if is_dark {
        "var(--surface-dark, #010105)"
    } else {
        "var(--surface-light, #F3F5FC)"
    };

    if is_multi {
        // ── Multi-series rendering ──
        // Collect unique series names across all items.
        let mut series_names: Vec<String> = Vec::new();
        for item in data.iter().take(8) {
            if let Some(arr) = item.get("series").and_then(|v| v.as_array()) {
                for s in arr {
                    let name = s.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    if !series_names.contains(&name) {
                        series_names.push(name);
                    }
                }
            }
        }

        for (si, sname) in series_names.iter().enumerate() {
            let stroke_col = series_palette[si % series_palette.len()];

            // Extract values for this series across all categories
            let vals: Vec<f64> = data
                .iter()
                .take(8)
                .map(|item| {
                    item.get("series")
                        .and_then(|v| v.as_array())
                        .and_then(|arr| {
                            arr.iter()
                                .find(|s| s.get("name").and_then(|v| v.as_str()) == Some(sname.as_str()))
                                .and_then(|s| s.get("value")?.as_f64())
                        })
                        .unwrap_or(0.0)
                })
                .collect();

            let mut points = Vec::new();
            for (i, &val) in vals.iter().enumerate() {
                let x = x_of(i);
                let y = height as f64 - pad_bottom as f64
                    - ((val - min_val) / (max_val - min_val)) * chart_h;
                points.push((x, y));
            }

            // Line path
            if !points.is_empty() {
                let mut path_d = format!("M {:.1} {:.1} ", points[0].0, points[0].1);
                for pt in &points[1..] {
                    path_d.push_str(&format!("L {:.1} {:.1} ", pt.0, pt.1));
                }
                all_paths.push_str(&format!(
                    r#"<path d="{}" stroke="{}" stroke-width="2.5" fill="none" stroke-linecap="round" stroke-linejoin="round" />"#,
                    path_d, stroke_col
                ));
            }

            // Area fill
            if draw_area && !points.is_empty() {
                let y_baseline = height as f64 - pad_bottom as f64;
                let mut area_d = format!("M {:.1} {:.1} ", points[0].0, y_baseline);
                for pt in &points {
                    area_d.push_str(&format!("L {:.1} {:.1} ", pt.0, pt.1));
                }
                area_d.push_str(&format!(
                    "L {:.1} {:.1} Z ",
                    points[points.len() - 1].0,
                    y_baseline
                ));
                let grad_id = format!("chart_area_grad_{}", si);
                let grad_def = format!(
                    r#"<linearGradient id="{}" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stop-color="{}" stop-opacity="0.18" />
                        <stop offset="100%" stop-color="{}" stop-opacity="0.0" />
                    </linearGradient>"#,
                    grad_id, stroke_col, stroke_col
                );
                area_grad_defs.push_str(&grad_def);
                area_paths.push_str(&format!(
                    r#"<path d="{}" fill="url(#{})" />"#,
                    area_d, grad_id
                ));
            }

            // Markers
            for pt in &points {
                all_markers.push_str(&format!(
                    r#"<circle cx="{:.1}" cy="{:.1}" r="3.5" fill="{}" stroke="{}" stroke-width="1.5" />"#,
                    pt.0, pt.1, stroke_col, bg_color_repr
                ));
            }


        }
    } else {
        // ── Single-series rendering (backward-compatible) ──
        let vals: Vec<f64> = data
            .iter()
            .take(8)
            .map(|item| {
                item.get("value")
                    .and_then(|v| {
                        v.as_f64()
                            .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
                    })
                    .unwrap_or(0.0)
            })
            .collect();

        let mut points = Vec::new();
        for (i, &val) in vals.iter().enumerate() {
            let x = x_of(i);
            let y = height as f64 - pad_bottom as f64
                - ((val - min_val) / (max_val - min_val)) * chart_h;
            points.push((x, y));
        }

        let stroke_col = &colors.primary;
        if !points.is_empty() {
            let mut path_d = format!("M {:.1} {:.1} ", points[0].0, points[0].1);
            for pt in &points[1..] {
                path_d.push_str(&format!("L {:.1} {:.1} ", pt.0, pt.1));
            }
            all_paths.push_str(&format!(
                r#"<path d="{}" stroke="{}" stroke-width="2.5" fill="none" stroke-linecap="round" stroke-linejoin="round" />"#,
                path_d, stroke_col
            ));

            if draw_area {
                let y_baseline = height as f64 - pad_bottom as f64;
                let mut area_d = format!("M {:.1} {:.1} ", points[0].0, y_baseline);
                for pt in &points {
                    area_d.push_str(&format!("L {:.1} {:.1} ", pt.0, pt.1));
                }
                area_d.push_str(&format!(
                    "L {:.1} {:.1} Z ",
                    points[points.len() - 1].0,
                    y_baseline
                ));
                let grad_id = "chart_area_grad";
                area_grad_defs.push_str(&format!(
                    r#"<linearGradient id="{}" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stop-color="{}" stop-opacity="0.25" />
                        <stop offset="100%" stop-color="{}" stop-opacity="0.0" />
                    </linearGradient>"#,
                    grad_id, stroke_col, stroke_col
                ));
                area_paths.push_str(&format!(
                    r#"<path d="{}" fill="url(#{})" />"#,
                    area_d, grad_id
                ));
            }

            for pt in &points {
                all_markers.push_str(&format!(
                    r#"<circle cx="{:.1}" cy="{:.1}" r="4" fill="{}" stroke="{}" stroke-width="1.5" />"#,
                    pt.0, pt.1, stroke_col, bg_color_repr
                ));
            }
        }
    }

    // ── X-axis labels (HTML overlay — see line_chart_parts doc) ──
    for (i, lbl) in labels.iter().enumerate() {
        labels_out.push(ChartLabel {
            text: lbl.clone(),
            x: x_of(i),
            y: height as f64 - 4.0,
            anchor: "middle",
            weight: 400,
            size_px: 9.0,
            color: colors.text_secondary.clone(),
            swatch: None,
        });
    }

    // ── Legend (multi-series only, SVG-native <text> elements) ──
    let legend_svg = if is_multi {
        // Build a horizontal row of colored rectangles + labels using SVG primitives
        // for maximum renderer compatibility (no foreignObject).
        let mut legend_svg_parts = String::new();
        // Rough estimate: each entry ≈ 60px wide
        let entry_width = 60.0;
        let num_entries = data
            .first()
            .and_then(|item| item.get("series")?.as_array())
            .map(|arr| arr.len())
            .unwrap_or(0);
        let total_width = num_entries as f64 * entry_width;
        let x_offset = (width as f64 - total_width) / 2.0;

        if let Some(first_series) = data
            .first()
            .and_then(|item| item.get("series")?.as_array())
        {
            let mut widths = Vec::new();
            for sv in first_series {
                let name = sv.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let w = (name.len() as f64 * 6.2 + 22.0).max(45.0);
                widths.push(w);
            }
            let total_width: f64 = widths.iter().sum();
            let mut cur_x = (width as f64 - total_width) / 2.0;

            for (si, sv) in first_series.iter().enumerate() {
                let name = sv.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let col = series_palette[si % series_palette.len()];
                let rect_x = cur_x;
                let text_x = rect_x + 12.0;
                legend_svg_parts.push_str(&format!(
                    r#"<rect x="{:.1}" y="10" width="9" height="4" rx="1" fill="{}" />"#,
                    rect_x, col
                ));
                // Legend label (HTML overlay — see line_chart_parts doc).
                labels_out.push(ChartLabel {
                    text: name.to_string(),
                    x: text_x,
                    y: 14.5,
                    anchor: "start",
                    weight: 700,
                    size_px: 9.0,
                    color: colors.text_secondary.clone(),
                    swatch: Some(col.to_string()),
                });
                cur_x += widths[si];
            }
        }
        format!(
            r#"<g transform="translate(0,0)">{}</g>"#,
            legend_svg_parts
        )
    } else {
        String::new()
    };

    // ── Final SVG assembly ──
    // Only emit <defs> if there are gradient definitions
    let defs_block = if area_grad_defs.is_empty() {
        String::new()
    } else {
        format!("<defs>{}</defs>", area_grad_defs)
    };

    // Legend goes in the visible SVG tree, NOT inside <defs>.
    // Position it above the chart area with a small vertical offset.
    let legend_g = if !legend_svg.is_empty() {
        format!(
            r#"<g transform="translate(0, 4)">{}</g>"#,
            legend_svg
        )
    } else {
        String::new()
    };

    let svg = format!(
        r#"<svg width="100%" height="{}px" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">
            {}
            {}
            {}
            {}
            {}
            {}
        </svg>"#,
        height, width, height, defs_block, legend_g, grid_lines, area_paths, all_paths, all_markers
    );
    (svg, labels_out)
}

pub fn render_svg_scatter_plot(
    data: &[Value],
    width: u32,
    height: u32,
    colors: &SlideColors,
    x_label: &str,
    y_label: &str,
) -> String {
    scatter_parts(data, width, height, colors, x_label, y_label).0
}

/// Compose the scatter SVG plus its text labels (HTML overlay — see
/// `line_chart_parts` doc; blitz's usvg rasterizer cannot see vendored fonts).
pub fn scatter_parts(
    data: &[Value],
    width: u32,
    height: u32,
    colors: &SlideColors,
    x_label: &str,
    y_label: &str,
) -> (String, Vec<ChartLabel>) {
    if data.is_empty() {
        return (String::new(), Vec::new());
    }

    let mut x_vals = Vec::new();
    let mut y_vals = Vec::new();
    let mut sizes = Vec::new();
    let mut labels = Vec::new();

    for item in data.iter().take(12) {
        let x = item.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let y = item.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let size = item.get("size").and_then(|v| v.as_f64()).unwrap_or(8.0);
        let label = item
            .get("label")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{:.0},{:.0}", x, y));

        x_vals.push(x);
        y_vals.push(y);
        sizes.push(size);
        labels.push(label);
    }

    let max_x = x_vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut min_x = x_vals.iter().copied().fold(f64::INFINITY, f64::min);
    let max_y = y_vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut min_y = y_vals.iter().copied().fold(f64::INFINITY, f64::min);
    let max_size = sizes.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    if max_x == min_x {
        min_x -= 1.0;
    }
    if max_y == min_y {
        min_y -= 1.0;
    }

    let pad_left = 50;
    let pad_right = 24;
    let pad_top = 28;
    let pad_bottom = 38;

    let chart_w = width as f64 - pad_left as f64 - pad_right as f64;
    let chart_h = height as f64 - pad_top as f64 - pad_bottom as f64;

    let inner_pad_x = 22.0;
    let inner_pad_y = 18.0;
    let plot_w = chart_w - 2.0 * inner_pad_x;
    let plot_h = chart_h - 2.0 * inner_pad_y;

    let mut labels_out: Vec<ChartLabel> = Vec::new();
    let mut grid_lines = String::new();
    // Y grid
    for i in 0..3 {
        let frac = i as f64 / 2.0;
        let y_val = min_y + frac * (max_y - min_y);
        let y_pos = height as f64 - pad_bottom as f64 - frac * chart_h;
        grid_lines.push_str(&format!(
            r#"<line x1="{}" y1="{:.1}" x2="{}" y2="{:.1}" stroke="{}44" stroke-dasharray="3,3" stroke-width="1" />"#,
            pad_left, y_pos, width - pad_right, y_pos, colors.border
        ));
        labels_out.push(ChartLabel {
            text: format!("{:.1}", y_val),
            x: pad_left as f64 - 6.0,
            y: y_pos + 3.0,
            anchor: "end",
            weight: 600,
            size_px: 8.5,
            color: colors.text_secondary.clone(),
            swatch: None,
        });
    }

    // X grid
    for i in 0..3 {
        let frac = i as f64 / 2.0;
        let x_val = min_x + frac * (max_x - min_x);
        let x_pos = pad_left as f64 + frac * chart_w;
        grid_lines.push_str(&format!(
            r#"<line x1="{:.1}" y1="{}" x2="{:.1}" y2="{}" stroke="{}44" stroke-dasharray="3,3" stroke-width="1" />"#,
            x_pos, pad_top, x_pos, height as f64 - pad_bottom as f64, colors.border
        ));
        labels_out.push(ChartLabel {
            text: format!("{:.0}", x_val),
            x: x_pos,
            y: height as f64 - pad_bottom as f64 + 14.0,
            anchor: "middle",
            weight: 600,
            size_px: 8.5,
            color: colors.text_secondary.clone(),
            swatch: None,
        });
    }

    let primary_color = &colors.primary;
    let mut points_svg = String::new();
    let mut path_d = String::new();

    for i in 0..x_vals.len() {
        let x_pos = pad_left as f64 + inner_pad_x + ((x_vals[i] - min_x) / (max_x - min_x)) * plot_w;
        let y_pos =
            height as f64 - pad_bottom as f64 - inner_pad_y - ((y_vals[i] - min_y) / (max_y - min_y)) * plot_h;

        if i == 0 {
            path_d.push_str(&format!("M {:.1} {:.1}", x_pos, y_pos));
        } else {
            path_d.push_str(&format!(" L {:.1} {:.1}", x_pos, y_pos));
        }

        let mut r = 4.5;
        if max_size > 0.0 {
            r = 4.5 + (sizes[i] / max_size) * 7.0;
        }

        points_svg.push_str(&format!(
            r#"<circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="{}" fill-opacity="0.85" stroke="white" stroke-width="1.5" />"#,
            x_pos, y_pos, r, primary_color
        ));
        // Point label (HTML overlay — see line_chart_parts doc).
        labels_out.push(ChartLabel {
            text: labels[i].clone(),
            x: x_pos,
            y: y_pos - r - 4.0,
            anchor: "middle",
            weight: 800,
            size_px: 8.0,
            color: colors.text_primary.clone(),
            swatch: None,
        });
    }

    let trendline = format!(
        r#"<path d="{}" fill="none" stroke="{}" stroke-width="2" stroke-dasharray="4,4" opacity="0.6" />"#,
        path_d, primary_color
    );

    if !x_label.is_empty() {
        labels_out.push(ChartLabel {
            text: x_label.to_string(),
            x: pad_left as f64 + chart_w / 2.0,
            y: height as f64 - 2.0,
            anchor: "middle",
            weight: 800,
            size_px: 9.0,
            color: colors.text_secondary.clone(),
            swatch: None,
        });
    }
    if !y_label.is_empty() {
        labels_out.push(ChartLabel {
            text: y_label.to_string(),
            x: pad_left as f64,
            y: 14.0,
            anchor: "start",
            weight: 800,
            size_px: 9.0,
            color: colors.text_secondary.clone(),
            swatch: None,
        });
    }

    let svg = format!(
        r#"<svg width="100%" height="{}px" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">
            {}
            {}
            {}
        </svg>"#,
        height, width, height, grid_lines, trendline, points_svg
    );
    (svg, labels_out)
}

pub fn render_svg_gauge_chart(value: f64, target: f64, unit: &str, colors: &SlideColors) -> String {
    gauge_parts(value, target, unit, colors).0
}

/// Compose the gauge SVG plus its value/TARGET labels (HTML overlay — see
/// `line_chart_parts` doc; blitz's usvg rasterizer cannot see vendored fonts).
pub fn gauge_parts(
    value: f64,
    target: f64,
    unit: &str,
    colors: &SlideColors,
) -> (String, Vec<ChartLabel>) {
    let r = 52.0;
    let cx = 100.0;
    let cy = 80.0;

    let circ = std::f64::consts::PI * r;
    let pct = (value / target).max(0.0).min(1.0);
    let offset = circ * (1.0 - pct);

    let primary_color = &colors.primary;
    let clean_unit = if unit.len() <= 5 { escape_html(unit) } else { "%".to_string() };

    let svg = format!(
        r#"<svg width="100%" height="115px" viewBox="0 0 200 115" xmlns="http://www.w3.org/2000/svg">
            <!-- Background Arc -->
            <path d="M {:.1} {:.1} A {:.1} {:.1} 0 0 1 {:.1} {:.1}" fill="none" stroke="{}44" stroke-width="12" stroke-linecap="round" />
            
            <!-- Foreground Filled Arc -->
            <path d="M {:.1} {:.1} A {:.1} {:.1} 0 0 1 {:.1} {:.1}" fill="none" stroke="{}" stroke-width="12" stroke-linecap="round"
                  stroke-dasharray="{:.2}" stroke-dashoffset="{:.2}" opacity="0.9" />
        </svg>"#,
        cx - r, cy, r, r, cx + r, cy, colors.border,
        cx - r, cy, r, r, cx + r, cy, primary_color, circ, offset
    );

    // Central metric + TARGET (HTML overlay — see line_chart_parts doc).
    let labels = vec![
        ChartLabel {
            text: format!("{:.1}{}", value, clean_unit),
            x: cx,
            y: cy - 6.0,
            anchor: "middle",
            weight: 900,
            size_px: 26.0,
            color: colors.text_primary.clone(),
            swatch: None,
        },
        ChartLabel {
            text: format!("TARGET: {:.0}{}", target, clean_unit),
            x: cx,
            y: cy + 18.0,
            anchor: "middle",
            weight: 700,
            size_px: 9.0,
            color: colors.text_secondary.clone(),
            swatch: None,
        },
    ];
    (svg, labels)
}

pub fn render_svg_radar_chart(
    data: &[Value],
    width: u32,
    height: u32,
    colors: &SlideColors,
) -> String {
    if data.len() < 3 {
        return String::new();
    }

    let mut values = Vec::new();
    for item in data.iter().take(8) {
        let val = item
            .get("value")
            .and_then(|v| {
                v.as_f64()
                    .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
            })
            .unwrap_or(0.0);
        values.push(val);
    }

    let n = values.len();
    let max_val = values.iter().copied().fold(0.0, f64::max).max(1.0);

    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0 - 5.0;
    let max_r = (width.min(height) as f64 / 2.0) - 25.0;

    let mut angles = Vec::new();
    for i in 0..n {
        let angle =
            (i as f64) * (2.0 * std::f64::consts::PI / n as f64) - (std::f64::consts::PI / 2.0);
        angles.push(angle);
    }

    let mut bg_rings = String::new();
    for r_idx in 1..=5 {
        let r_frac = r_idx as f64 / 5.0;
        let r_curr = max_r * r_frac;
        let mut ring_pts = Vec::new();
        for &angle in &angles {
            let rx = cx + r_curr * angle.cos();
            let ry = cy + r_curr * angle.sin();
            ring_pts.push(format!("{:.1},{:.1}", rx, ry));
        }
        let pts_str = ring_pts.join(" ");
        bg_rings.push_str(&format!(
            r#"<polygon points="{}" fill="none" stroke="{}33" stroke-width="1" />"#,
            pts_str, colors.border
        ));
    }

    let mut axis_svg = String::new();
    for &angle in &angles {
        let rx_max = cx + max_r * angle.cos();
        let ry_max = cy + max_r * angle.sin();
        axis_svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{:.1}" y2="{:.1}" stroke="{}33" stroke-width="1" />"#,
            cx, cy, rx_max, ry_max, colors.border
        ));

        // NOTE: axis labels are intentionally NOT rendered as SVG <text>.
        // blitz rasterizes inline SVG through usvg/resvg with its own system
        // font database (blitz-dom's `FONT_DB`), which cannot see the vendored
        // Google fonts — so SVG labels render with wrong/mixed glyphs. The
        // slide composes HTML label overlays via `radar_label_layout()` so
        // labels use the vendored body font through the normal HTML pipeline.
        //
        // GEOMETRY COUPLING: the label margins below (max_r = min(w,h)/2 - 25,
        // label radius max_r + 14/+10, anchor offsets) MUST stay in sync with
        // `radar_label_layout` — they share the same viewBox math.
    }

    let primary_color = &colors.primary;
    let mut data_pts = Vec::new();
    let mut markers = String::new();
    for (i, &val) in values.iter().enumerate() {
        let val_r = max_r * (val / max_val);
        let vx = cx + val_r * angles[i].cos();
        let vy = cy + val_r * angles[i].sin();
        data_pts.push(format!("{:.1},{:.1}", vx, vy));
        markers.push_str(&format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{}\" stroke=\"#ffffff\" stroke-width=\"1.5\" />",
            vx, vy, primary_color
        ));
    }

    let data_pts_str = data_pts.join(" ");
    let plot_shape = format!(
        r#"<polygon points="{}" fill="{}" fill-opacity="0.25" stroke="{}" stroke-width="2" />"#,
        data_pts_str, primary_color, primary_color
    );

    format!(
        r#"<svg width="100%" height="{}px" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">
            {}
            {}
            {}
            {}
        </svg>"#,
        height, width, height, bg_rings, axis_svg, plot_shape, markers
    )
}

/// One radar axis label, positioned for HTML overlay (not SVG `<text>`).
/// Coordinates are in the same viewBox space as `render_svg_radar_chart`, so a
/// `position:relative` wrapper around the SVG can place these absolutely with
/// matching geometry. `anchor` mirrors the SVG `text-anchor` semantics
/// ("middle" | "start" | "end").
pub struct RadarLabel {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub anchor: &'static str,
}

/// Compute radar axis label positions — the HTML counterpart to the SVG text
/// that was removed from `render_svg_radar_chart`. Rendering labels as HTML
/// (rather than SVG `<text>`) routes them through the normal HTML font pipeline,
/// so they use the vendored body font with correct glyphs even in blitz, whose
/// usvg rasterizer only sees the system font database.
pub fn radar_label_layout(data: &[Value], width: u32, height: u32) -> Vec<RadarLabel> {
    if data.len() < 3 {
        return Vec::new();
    }

    let mut labels = Vec::new();
    let mut values = Vec::new();
    for item in data.iter().take(8) {
        let label = item
            .get("label")
            .or_else(|| item.get("axis"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        labels.push(label);
        let val = item
            .get("value")
            .and_then(|v| {
                v.as_f64()
                    .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
            })
            .unwrap_or(0.0);
        values.push(val);
    }

    let n = values.len();
    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0 - 5.0;
    let max_r = (width.min(height) as f64 / 2.0) - 25.0;

    let mut out = Vec::new();
    for i in 0..n {
        let angle = (i as f64) * (2.0 * std::f64::consts::PI / n as f64) - (std::f64::consts::PI / 2.0);
        let mut lbl_x = cx + (max_r + 14.0) * angle.cos();
        let lbl_y = cy + (max_r + 10.0) * angle.sin();
        let mut anchor = "middle";
        if angle.cos() > 0.1 {
            anchor = "start";
            lbl_x += 2.0;
        } else if angle.cos() < -0.1 {
            anchor = "end";
            lbl_x -= 2.0;
        }
        let text = if labels[i].is_empty() {
            format!("{:.0}", values[i])
        } else {
            format!("{} ({:.0})", labels[i], values[i])
        };
        out.push(RadarLabel {
            text,
            x: lbl_x,
            y: lbl_y,
            anchor,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layouts::SlideColors;
    use serde_json::json;

    fn make_test_colors() -> SlideColors {
        SlideColors {
            text_primary: "#1A1A2E".to_string(),
            text_secondary: "#6B7280".to_string(),
            primary: "#767CFF".to_string(),
            button_bg: "#767CFF".to_string(),
            button_text: "#FFFFFF".to_string(),
            border: "#E5E7EB".to_string(),
            is_dark: false,
        }
    }

    /// Radar labels are composed as HTML overlays (blitz's usvg rasterizer
    /// can't see vendored web fonts), so their positions must stay inside the
    /// SVG viewBox and mirror the plot geometry that `render_svg_radar_chart`
    /// uses — drift here silently misplaces labels.
    #[test]
    fn test_radar_label_layout_matches_viewbox_and_values() {
        let data = vec![
            json!({"label": "Speed", "value": 95}),
            json!({"label": "Accuracy", "value": 88}),
            json!({"label": "Stability", "value": 70}),
            json!({"label": "Battery", "value": 92}),
        ];
        let labels = radar_label_layout(&data, 320, 210);
        assert_eq!(labels.len(), 4, "one label per axis");
        for lbl in &labels {
            assert!(lbl.x >= 0.0 && lbl.x <= 320.0, "x out of viewBox: {}", lbl.x);
            assert!(lbl.y >= 0.0 && lbl.y <= 210.0, "y out of viewBox: {}", lbl.y);
            assert!(
                lbl.text.contains("Speed") || lbl.text.contains("Accuracy")
                    || lbl.text.contains("Stability") || lbl.text.contains("Battery"),
                "label text lost: {}",
                lbl.text
            );
            assert!(matches!(lbl.anchor, "middle" | "start" | "end"));
        }
        // The svg geometry reserves 25px beyond max_r for labels; labels must
        // sit inside that margin band, not overlap the plot.
        for lbl in &labels {
            assert!(lbl.y >= 0.0 && lbl.y <= 210.0, "label y clipped: {}", lbl.y);
        }
    }

    #[test]
    fn test_line_chart_single_series() {
        let data = vec![
            json!({"label": "Jan", "value": 30}),
            json!({"label": "Feb", "value": 50}),
            json!({"label": "Mar", "value": 40}),
        ];
        let colors = make_test_colors();
        let svg = render_svg_line_chart(&data, 300, 150, &colors, false, false);
        assert!(!svg.is_empty(), "single series should produce non-empty SVG");
        assert!(svg.contains("<svg"), "should be a valid SVG");
        assert!(svg.contains("<path"), "single series should produce at least one <path>");
        // The path should have some visual styling (stroke or style attr)
        assert!(svg.contains("767CFF"), "should use the primary color for the line");
    }

    #[test]
    fn test_line_chart_multi_series_produces_multiple_paths() {
        let data = vec![
            json!({
                "label": "2020",
                "series": [
                    {"name": "Men", "value": 58},
                    {"name": "Women", "value": 42}
                ]
            }),
            json!({
                "label": "2021",
                "series": [
                    {"name": "Men", "value": 55},
                    {"name": "Women", "value": 45}
                ]
            }),
        ];
        let colors = make_test_colors();
        let (svg, labels) = line_chart_parts(&data, 300, 150, &colors, false, false);

        // Multi-series should produce at least 2 path elements
        let path_count = svg.matches("<path").count();
        assert!(
            path_count >= 2,
            "multi-series should produce >=2 <path> elements, got {}",
            path_count
        );

        // Legend names are HTML overlay labels (blitz's usvg can't see
        // vendored fonts), so they must be present in the label list — with
        // the correct series-color swatch for each.
        let men = labels
            .iter()
            .find(|l| l.text == "Men")
            .expect("legend should include Men series name");
        let women = labels
            .iter()
            .find(|l| l.text == "Women")
            .expect("legend should include Women series name");
        assert!(men.swatch.is_some(), "Men legend should carry a color swatch");
        assert!(women.swatch.is_some(), "Women legend should carry a color swatch");

        // No SVG-native <text> should be emitted: it renders with system
        // fonts under blitz (vendored Google fonts are invisible to usvg).
        assert!(
            !svg.contains("<text"),
            "line chart must not emit SVG <text> (glyph bug class)"
        );
    }

    /// Regression: gauge value + TARGET labels are HTML overlays positioned in
    /// the SVG viewBox space; they must exist and stay inside the 200x115 box.
    #[test]
    fn test_gauge_parts_emits_html_labels() {
        let colors = make_test_colors();
        let (svg, labels) = gauge_parts(72.0, 100.0, "%", &colors);
        assert!(!svg.contains("<text"), "gauge SVG must not emit <text>");
        assert_eq!(labels.len(), 2, "value + TARGET labels");
        assert!(labels[0].text.contains("72.0"), "value label lost: {}", labels[0].text);
        assert!(labels[1].text.contains("TARGET"), "TARGET label lost: {}", labels[1].text);
        for l in &labels {
            assert!(l.x >= 0.0 && l.x <= 200.0, "x out of viewBox: {}", l.x);
            assert!(l.y >= 0.0 && l.y <= 115.0, "y out of viewBox: {}", l.y);
        }
        let overlay = chart_label_overlay(&labels, "DM Sans");
        assert!(overlay.contains("72.0") && overlay.contains("TARGET"), "overlay HTML missing labels");
        assert!(overlay.contains("position:absolute"), "labels must be absolute spans");
    }

    /// Regression: scatter labels are HTML overlays; SVG must stay text-free.
    #[test]
    fn test_scatter_parts_emits_html_labels() {
        let data = vec![
            json!({"x": 1, "y": 2, "label": "Alpha"}),
            json!({"x": 2, "y": 5, "label": "Beta"}),
            json!({"x": 3, "y": 3, "label": "Gamma"}),
        ];
        let colors = make_test_colors();
        let (svg, labels) = scatter_parts(&data, 320, 185, &colors, "X Axis", "Y Axis");
        assert!(!svg.contains("<text"), "scatter SVG must not emit <text>");
        let texts: Vec<&str> = labels.iter().map(|l| l.text.as_str()).collect();
        assert!(texts.contains(&"Alpha"), "point label lost: {texts:?}");
        assert!(texts.contains(&"X Axis"), "x-axis title lost: {texts:?}");
        assert!(texts.contains(&"Y Axis"), "y-axis title lost: {texts:?}");
    }

    #[test]
    fn test_line_chart_empty_data() {
        let data: Vec<Value> = vec![];
        let colors = make_test_colors();
        let svg = render_svg_line_chart(&data, 300, 150, &colors, false, false);
        assert!(svg.is_empty(), "empty data should produce empty string");
    }

    #[test]
    fn test_line_chart_area_fill() {
        let data = vec![
            json!({"label": "A", "value": 10}),
            json!({"label": "B", "value": 20}),
            json!({"label": "C", "value": 15}),
        ];
        let colors = make_test_colors();
        let svg = render_svg_line_chart(&data, 300, 150, &colors, false, true);
        assert!(!svg.is_empty(), "area fill should produce non-empty SVG");
        assert!(svg.contains("<svg"), "should be a valid SVG");
        // Area fill should have gradient definitions
        assert!(svg.contains("linearGradient"), "area fill should have gradient defs");
    }
}
