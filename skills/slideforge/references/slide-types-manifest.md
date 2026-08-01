# SlideForge 36 Slide Types Manifest

Use this reference to find the exact slide type matching your content requirements. Each slide type belongs to a specific layout family and child composition skill.

> **Note (2026-07-30 purge):** The following slide types have been retired from the registry because they were visually redundant with stronger alternatives in the same family. Use the replacement column when composing.
>
> | Retired | Use Instead |
> |---|---|
> | `feature` | `split_features` (single-feature beat) |
> | `list` | `checklist_action_plan` (action list) |
> | `callout` | `split_features` (educational contrast) |
> | `grid_cards` | `split_features` (2 col) or `case_study_result` (results grid) |
> | `text_columns` | `split_features` (2 col) |
> | `image_stat` | `image_callout` (image+stat combo) |
> | `cta` | `qr_destination` (the canonical closing CTA slide) |
> | `stat_row` | `metric_grid` |
> | `column_chart` | `chart` with `chart_type="bar"` + `variant="vertical"` |
> | `comparison` | `before_after_story` (A vs B) |

---
## 1. Text & Layouts (`text-layouts/SKILL.md`)

These slides focus on structuring copywriting, headings, bullets, quotes, and layouts:

| Slide Type | Required Parameters | Optional Parameters | Best For |
|---|---|---|---|
| `hero` | `headline` | `subheadline`, `background_color`, `text_color`, `badge`, `cta_text` | Opening hooks, section dividers |
| `quote` | `quote` | `author`, `role`, `company`, `avatar_url`, `rating`, `logo_url` | Customer testimonials, bold pull-quotes |
| `split_features` | `title`, `features` | `variant`, `background_color` | Two column feature listings, icon grids |
| `definition` | `term`, `definition` | `context`, `variant` | Educational terms or glossary entries |
| `text_block` | `title`, `body` | `variant` | Simple paragraph content |
| `section_divider` | `title` | `kicker`, `subtitle`, `variant` | Slide deck chapter openers |
| `timeline` | `title`, `steps` | `variant` | Paced step-by-step processes |

---
## 2. Data Visualizations (`data-viz/SKILL.md`)

These slides render charts, gauges, tables, and metric indicators:

| Slide Type | Required Parameters | Optional Parameters | Best For |
|---|---|---|---|
| `chart` | `chart_type` (bar/line/pie), `title`, `data` | `caption`, `variant` (vertical for vertical bars) | General data visualizations |
| `scatter_plot` | `title`, `data` | `x_label`, `y_label`, `variant` | Correlation charts |
| `gauge` | `value` (0-100), `label` | `title`, `variant` | Radial progress, system loads |
| `radar_chart` | `title`, `data` | `variant` | Multidimensional comparisons (skills, tests) |
| `table` | `title`, `headers`, `rows` | `variant` | Tabular data matrices |
| `funnel_chart` | `title`, `steps` | `variant` | Sales conversions |
| `metric_grid` | `title`, `metrics` | `variant` | 2x2 grid of key performance metrics |
| `comparison_bars` | `title`, `comparison` | `description`, `variant` | Direct comparison between two values |
| `progress_rings` | `title`, `items` | `description`, `variant` | concentric circular progress loops |

---
## 3. Story & Educational Flows (`story-flows/SKILL.md`)

These slides guide narrative storytelling, objections, and planning:

| Slide Type | Required Parameters | Optional Parameters | Best For |
|---|---|---|---|
| `problem_solution` | `problem`, `solution` | `title`, `proof_points`, `variant` | Core pitch slides |
| `myth_fact` | `myth`, `fact` | `explanation`, `variant` | Educational debunking |
| `case_study_result` | `client`, `challenge`, `solution`, `results` | `variant` | Validating authority and proof |
| `testimonial_avatar` | `quote`, `author` | `role`, `avatar_url`, `variant` | Detailed customer reviews |
| `before_after_story` | `before`, `after` | `title`, `metric`, `variant` | Transformation showcases |
| `logo_cloud` | `title`, `logos` | `variant` | Social proof, trust boards |
| `pricing_plan` | `title`, `plans` | `variant` | Offer details and cost choices |
| `checklist_action_plan` | `title`, `items` | `variant` | Process step checklist |
| `faq` | `title`, `questions` | `variant` | Handing objections |
| `process_map` | `title`, `steps` | `variant` | Operating flows |

---
## 4. Image Integration (`image-integration/SKILL.md`)

These slides handle graphics, illustrations, and local Base64 image embeddings:

| Slide Type | Required Parameters | Optional Parameters | Best For |
|---|---|---|---|
| `image_caption` | `image_url`, `caption` | `description`, `layout` | Captioned photos |
| `image_headline` | `image_url`, `headline` | `subheadline`, `overlay_position` | Large background poster headings |
| `image_quote` | `image_url`, `quote` | `author`, `role`, `variant` | Quotes overlaying a background image |
| `image_callout` | `image_url`, `callouts` | `description`, `variant` | Product layouts with hot-spot callouts |
| `image_gallery` | `images` (2-6 urls) | `layout`, `title`, `section_caption` | Portfolio displays |
| `image_collage` | `images`, `title` | `style`, `section_caption` | Creative collections |
| `image_comparison` | `before_image`, `after_image` | `before_label`, `after_label`, `divider_style` | Before/After graphics |

---
## 5. Conversions & Marketing

| Slide Type | Required Parameters | Optional Parameters | Best For |
|---|---|---|---|
| `qr_destination` | `destination_url`, `cta_text`, `heading` | `caption`, `short_url`, `incentive_text` | Converting attention to traffic (QR code) |
