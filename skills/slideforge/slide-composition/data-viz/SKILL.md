---
name: data-viz
description: Use when constructing data visualization slides such as charts, tables, gauges, metric grids, and progress rings.
---

# SlideForge Data Visualizations & Metrics

This leaf skill guides the composition of charts, tables, and metric indicators. Data visualizations are rendered dynamically inside the SlideForge engine. Ensuring correct data structures is crucial to avoid rendering crashes.

## Supported Slide Types & Schemas

### 1. `chart` (Data Graphs)
Renders standard bar, line, or pie charts.
- **Required Parameters:**
  - `chart_type` (string) — `"bar"`, `"line"`, or `"pie"`.
  - `title` (string) — Title of the chart.
  - `data` (array) — Data points list:
    - `label` (string, required) — X-axis or slice label.
    - `value` (number, required) — Y-axis value.
- **Optional Parameters:**
  - `variant` (string) — For `chart_type="bar"`, set to `"vertical"` to render a vertical/column chart instead of horizontal bars.
- **Limits:** Max 5 data points.

### 2. `gauge` (Radial Progress)
Displays progress toward a single percentage goal.
- **Required Parameters:**
  - `value` (number) — Current progress value between `0` and `100`.
  - `label` (string) — Target label (e.g., "CPU", "Goal Met").
- **Optional Parameters:**
  - `title` (string) — Slide title.

### 3. `progress_rings` (Multi-Rings)
Renders concentric progress circles for multiple metrics.
- **Required Parameters:**
  - `title` (string) — Slide title.
  - `items` (array) — List of progress items:
    - `label` (string, required) — Metric name.
    - `value` (number, required) — Value between `0` and `100`.
    - `color` (string, optional) — Custom hex color.

### 4. `table` (Data Table)
Formats tabular columns and rows cleanly.
- **Required Parameters:**
  - `title` (string) — Title.
  - `headers` (array of strings) — List of column headers.
  - `rows` (array of arrays of strings) — List of row values.
- **Limits:** Max 4 columns and 5 rows to prevent layout overflow.

### 5. `metric_grid` (2×2 Key Metrics Grid)
Renders a 2x2 grid of prominent metrics. Tiles never overflow: grid tracks use `minmax(0,1fr)` and every tile is a shrinkable grid item.
- **Required Parameters:**
  - `title` (string) — Slide title.
  - `metrics` (array) — 2-4 metric objects:
    - `value` (string, required) — The headline number/string.
    - `label` (string, required) — Metric label. **Max 20 chars** — hard error above.
    - `trend` (string, optional) — Trend badge text (e.g. `"+12%"`). **Max 20 chars** — hard error above. Renders on its **own dedicated line** under the value, never competing with the label.
    - `progress` (number, optional) — 0–100. Mirrors the actual metric the tile demonstrates (e.g. 42 for a 42% metric). If omitted, the progress bar hides; the bar exists to show what the metric *means*, not as an abstract filler.
- **Hard Caps:** the validator emits a **hard error** (not a warning, not `…` truncation) when any `trend` or `label` exceeds 20 characters. The renderer never writes ellipsis into metrics — fix the copy instead of relying on clipping.

### 6. `funnel_chart` (Sales Funnel)
Visualises sequential conversion steps.
- **Required Parameters:**
  - `title` (string) — Slide title.
  - `steps` (array) — Each step contains:
    - `label` (string, required) — Step name.
    - `value` (number, required) — Step value.

### 7. `scatter_plot` (Correlation Chart)
Plots X/Y correlations.
- **Required Parameters:**
  - `title` (string) — Slide title.
  - `data` (array) — Each point contains:
    - `x` (number, required)
    - `y` (number, required)
    - `label` (string, optional)
- **Optional Parameters:**
  - `x_label`, `y_label` (string) — Axis labels.

### 8. `radar_chart` (Multidimensional Comparison)
Renders a radar/spider chart across multiple axes.
- **Required Parameters:**
  - `title` (string) — Slide title.
  - `data` (array) — Axes + values per series:
    - `axis` (string, required)
    - `value` (number, required)

### 9. `comparison_bars` (Side-by-Side Value Bars)
Shows two values side-by-side as a paired bar comparison.
- **Required Parameters:**
  - `title` (string) — Slide title.
  - `comparison` (object):
    - `left_value`, `right_value` (number) — Pair of values.
    - `left_label`, `right_label` (string) — Pair of labels.

---

## Actionable Constraints & Design Rules

- [ ] **Data Array Boundaries:** Do not flood charts with large datasets. Limit line/bar datasets to 5 elements. Oversized datasets will overlap axis labels.
- [ ] **Valid Percentages:** Ensure gauge, progress ring, and `metric_grid` `progress` values are strictly between `0` and `100`.
- [ ] **Metric Value Contrast:** Keep values bold and labels light.
- [ ] **Data Consistency:** In comparison bars, ensure both the left and right values use the same units for logical readability.
- [ ] **Metric Caps:** Never exceed 20 chars for `metric_grid` `trend` or `label` — the validator will fail generation with a hard error.

---

## Example Payload

```json
{
  "slide_type": "chart",
  "params": {
    "chart_type": "bar",
    "variant": "vertical",
    "title": "Revenue Growth 2026",
    "data": [
      {"label": "Q1", "value": 40},
      {"label": "Q2", "value": 65},
      {"label": "Q3", "value": 90}
    ]
  }
}
```
