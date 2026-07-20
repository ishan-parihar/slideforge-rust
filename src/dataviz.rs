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

pub fn render_svg_line_chart(
    data: &[Value],
    width: u32,
    height: u32,
    colors: &SlideColors,
    is_dark: bool,
    draw_area: bool,
) -> String {
    if data.is_empty() {
        return String::new();
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
        grid_lines.push_str(&format!(
            r#"<text x="{}" y="{:.1}" font-size="9px" fill="{}" text-anchor="end" font-weight="600">{:.1}</text>"#,
            pad_left - 8, y_pos + 4.0, colors.text_secondary, y_val
        ));
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

    // ── X-axis labels ──
    let mut labels_svg = String::new();
    for (i, lbl) in labels.iter().enumerate() {
        let x = x_of(i);
        labels_svg.push_str(&format!(
            r#"<text x="{:.1}" y="{}" font-size="9px" fill="{}" text-anchor="middle">{}</text>"#,
            x,
            height - 4,
            colors.text_secondary,
            escape_html(lbl)
        ));
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
                    r#"<rect x="{:.1}" y="10" width="9" height="4" rx="1" fill="{}" /><text x="{:.1}" y="14.5" font-size="9px" font-weight="700" fill="{}" font-family="sans-serif">{}</text>"#,
                    rect_x, col, text_x, colors.text_secondary, escape_html(name)
                ));
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

    format!(
        r#"<svg width="100%" height="{}px" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">
            {}
            {}
            {}
            {}
            {}
            {}
            {}
        </svg>"#,
        height, width, height, defs_block, legend_g, grid_lines, area_paths, all_paths, all_markers, labels_svg
    )
}

pub fn render_svg_scatter_plot(
    data: &[Value],
    width: u32,
    height: u32,
    colors: &SlideColors,
    x_label: &str,
    y_label: &str,
) -> String {
    if data.is_empty() {
        return String::new();
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
            .unwrap_or("")
            .to_string();

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

    let pad_left = 38;
    let pad_right = 38;
    let pad_top = 22;
    let pad_bottom = 28;

    let chart_w = width as f64 - pad_left as f64 - pad_right as f64;
    let chart_h = height as f64 - pad_top as f64 - pad_bottom as f64;

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
        grid_lines.push_str(&format!(
            r#"<text x="{}" y="{:.1}" font-size="8px" fill="{}" text-anchor="end">{:.1}</text>"#,
            pad_left - 6,
            y_pos + 3.0,
            colors.text_secondary,
            y_val
        ));
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
        grid_lines.push_str(&format!(
            r#"<text x="{:.1}" y="{}" font-size="8px" fill="{}" text-anchor="middle">{:.0}</text>"#,
            x_pos,
            height as f64 - pad_bottom as f64 + 12.0,
            colors.text_secondary,
            x_val
        ));
    }

    let primary_color = &colors.primary;
    let mut points_svg = String::new();
    let mut path_d = String::new();

    for i in 0..x_vals.len() {
        let x_pos = pad_left as f64 + ((x_vals[i] - min_x) / (max_x - min_x)) * chart_w;
        let y_pos =
            height as f64 - pad_bottom as f64 - ((y_vals[i] - min_y) / (max_y - min_y)) * chart_h;

        if i == 0 {
            path_d.push_str(&format!("M {:.1} {:.1}", x_pos, y_pos));
        } else {
            path_d.push_str(&format!(" L {:.1} {:.1}", x_pos, y_pos));
        }

        let mut r = 5.0;
        if max_size > 0.0 {
            r = 5.0 + (sizes[i] / max_size) * 10.0;
        }

        points_svg.push_str(&format!(
            r#"<g>
                <circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="{}" fill-opacity="0.8" stroke="white" stroke-width="1.5" />
                <text x="{:.1}" y="{:.1}" font-size="8px" fill="{}" text-anchor="middle" font-weight="700">{}</text>
            </g>"#,
            x_pos, y_pos, r, primary_color,
            x_pos, y_pos - r - 4.0, colors.text_primary, escape_html(&labels[i])
        ));
    }

    let trendline = format!(
        r#"<path d="{}" fill="none" stroke="{}" stroke-width="2" stroke-dasharray="4,4" opacity="0.6" />"#,
        path_d, primary_color
    );

    let x_axis_title = if !x_label.is_empty() {
        format!(
            r#"<text x="{}" y="{}" font-size="9px" font-weight="700" fill="{}" text-anchor="middle" letter-spacing="0.05em">{}</text>"#,
            pad_left as f64 + chart_w / 2.0,
            height - 2,
            colors.text_secondary,
            escape_html(x_label)
        )
    } else {
        String::new()
    };

    let y_axis_title = if !y_label.is_empty() {
        format!(
            r#"<text x="{}" y="12" font-size="9px" font-weight="700" fill="{}" text-anchor="start" letter-spacing="0.05em">{}</text>"#,
            pad_left,
            colors.text_secondary,
            escape_html(y_label)
        )
    } else {
        String::new()
    };

    format!(
        r#"<svg width="100%" height="{}px" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">
            {}
            {}
            {}
            {}
            {}
        </svg>"#,
        height, width, height, grid_lines, trendline, points_svg, x_axis_title, y_axis_title
    )
}

pub fn render_svg_gauge_chart(value: f64, target: f64, unit: &str, colors: &SlideColors) -> String {
    let r = 52.0;
    let cx = 100.0;
    let cy = 80.0;

    let circ = std::f64::consts::PI * r;
    let pct = (value / target).max(0.0).min(1.0);
    let offset = circ * (1.0 - pct);

    let primary_color = &colors.primary;
    let text_color = &colors.text_primary;
    let clean_unit = if unit.len() <= 5 { escape_html(unit) } else { "%".to_string() };

    format!(
        r#"<svg width="100%" height="115px" viewBox="0 0 200 115" xmlns="http://www.w3.org/2000/svg">
            <!-- Background Arc -->
            <path d="M {:.1} {:.1} A {:.1} {:.1} 0 0 1 {:.1} {:.1}" fill="none" stroke="{}44" stroke-width="12" stroke-linecap="round" />
            
            <!-- Foreground Filled Arc -->
            <path d="M {:.1} {:.1} A {:.1} {:.1} 0 0 1 {:.1} {:.1}" fill="none" stroke="{}" stroke-width="12" stroke-linecap="round"
                  stroke-dasharray="{:.2}" stroke-dashoffset="{:.2}" opacity="0.9" />
                  
            <!-- Central Metric Value -->
            <text x="{:.1}" y="{:.1}" font-size="26px" fill="{}" font-weight="900" text-anchor="middle">{:.1}{}</text>
            <text x="{:.1}" y="{:.1}" font-size="9px" font-weight="700" fill="{}" text-anchor="middle" letter-spacing="0.04em">TARGET: {:.0}{}</text>
        </svg>"#,
        cx - r, cy, r, r, cx + r, cy, colors.border,
        cx - r, cy, r, r, cx + r, cy, primary_color, circ, offset,
        cx, cy - 6.0, text_color, value, clean_unit,
        cx, cy + 18.0, colors.text_secondary, target, clean_unit
    )
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
    for (i, &angle) in angles.iter().enumerate() {
        let rx_max = cx + max_r * angle.cos();
        let ry_max = cy + max_r * angle.sin();
        axis_svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{:.1}" y2="{:.1}" stroke="{}33" stroke-width="1" />"#,
            cx, cy, rx_max, ry_max, colors.border
        ));

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

        let display_label = format!("{} ({:.0})", labels[i], values[i]);
        axis_svg.push_str(&format!(
            r#"<text x="{:.1}" y="{:.1}" font-size="10px" font-weight="700" fill="{}" text-anchor="{}">{}</text>"#,
            lbl_x,
            lbl_y + 3.0,
            colors.text_primary,
            anchor,
            escape_html(&display_label)
        ));
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
        let svg = render_svg_line_chart(&data, 300, 150, &colors, false, false);

        // Multi-series should produce at least 2 path elements
        let path_count = svg.matches("<path").count();
        assert!(
            path_count >= 2,
            "multi-series should produce >=2 <path> elements, got {}",
            path_count
        );

        // Should have a legend with series names
        assert!(svg.contains("Men"), "legend should include Men series name");
        assert!(svg.contains("Women"), "legend should include Women series name");

        // Should NOT be in <defs> (legend must be visible)
        // The legend <g> should appear after the <defs> block
        let defs_end = svg.find("</defs>").unwrap_or(0);
        let men_pos = svg.find("Men").unwrap_or(0);
        assert!(
            men_pos > defs_end,
            "legend (Men) should appear after </defs>, not inside it"
        );
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
