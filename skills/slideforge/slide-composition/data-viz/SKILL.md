---
name: data-viz
description: Use when constructing data visualization slides such as charts, tables, gauges, sparklines, metric grids, and progress rings.
---

# SlideForge Data Visualizations & Metrics

This leaf skill guides the composition of charts, tables, and metric indicators. Data visualizations are rendered dynamically inside the SlideForge engine. Ensuring correct data structures is crucial to avoid rendering crashes.

## Supported Slide Types & Schemas

### 1. `chart` (Data Graphs)
Renders standard bar or line charts.
- **Required Parameters:**
  - `chart_type` (string) — `"bar"`, `"line"`, or `"pie"`.
  - `title` (string) — Title of the chart.
  - `data` (array) — Data points list:
    - `label` (string, required) — X-axis or slice label.
    - `value` (number, required) — Y-axis value.
- **Limits:** Max 5 data points.

### 2. `gauge` (Radial Progress)
Displays progress toward a single percentage goal.
- **Required Parameters:**
  - `value` (number) — Current progress value between `0` and `100`.
  - `label` (string) — Target label (e.g., "CPU", "Goal Met").
- **Optional Parameters:**
  - `title` (string) — Slide title.

### 3. `metric_sparkline` (Metric + Trend Line)
Shows a single metric with a mini trend chart.
- **Required Parameters:**
  - `value` (string) — Large prominent metric value (e.g., "$45K").
  - `label` (string) — Label for the metric.
  - `data` (array of numbers) — List of historical numbers for the sparkline chart.
- **Optional Parameters:**
  - `trend` (string) — Trend offset percentage (e.g., "+12%").
  - `context` (string) — Text explanation.

### 4. `progress_rings` (Multi-Rings)
Renders concentric progress circles for multiple metrics.
- **Required Parameters:**
  - `title` (string) — Slide title.
  - `items` (array) — List of progress items:
    - `label` (string, required) — Metric name.
    - `value` (number, required) — Value between `0` and `100`.
    - `color` (string, optional) — Custom hex color.

### 5. `table` (Data Table)
Formats tabular columns and rows cleanly.
- **Required Parameters:**
  - `title` (string) — Title.
  - `headers` (array of strings) — List of column headers.
  - `rows` (array of arrays of strings) — List of row values.
- **Limits:** Max 4 columns and 5 rows to prevent layout overflow.

---

## Actionable Constraints & Design Rules

- [ ] **Data Array Boundaries:** Do not flood charts with large datasets. Limit line/bar datasets to 5 elements. Oversized datasets will overlap axis labels.
- [ ] **Valid Percentages:** Ensure gauge and progress ring values are strictly between `0` and `100`.
- [ ] **Metric Value Contrast:** Keep values bold and labels light.
- [ ] **Data Consistency:** In comparison bars, ensure both the left and right values use the same units for logical readability.

---

## Example Payload

```json
{
  "slide_type": "chart",
  "params": {
    "chart_type": "bar",
    "title": "Revenue Growth 2026",
    "data": [
      {"label": "Q1", "value": 40},
      {"label": "Q2", "value": 65},
      {"label": "Q3", "value": 90}
    ]
  }
}
```
