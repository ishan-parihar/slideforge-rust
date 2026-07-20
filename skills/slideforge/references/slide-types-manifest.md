# SlideForge 47 Slide Types Manifest

Use this reference to find the exact slide type matching your content requirements. Each slide type belongs to a specific layout family and child composition skill.

---

## 1. Text & Layouts (`text-layouts/SKILL.md`)

These slides focus on structuring copywriting, headings, bullets, quotes, and layouts:

| Slide Type | Required Parameters | Optional Parameters | Best For |
|---|---|---|---|
| `hero` | `headline` | `subheadline`, `background_color`, `text_color`, `badge`, `cta_text` | Opening hooks, section dividers |
| `feature` | `title`, `description` | `icon`, `icon_color`, `badge`, `image_url`, `cta_text`, `variant` | Highlight single feature/benefit |
| `list` | `title`, `items` | `ordered`, `icon`, `icon_color`, `columns`, `show_numbers` | Bullet points, checklists |
| `quote` | `quote` | `author`, `role`, `company`, `avatar_url`, `rating`, `logo_url` | Customer testimonials, bold pull-quotes |
| `cta` | `headline`, `button_text` | `subheadline`, `button_url`, `secondary_text`, `urgency_text` | Final conversion, calls-to-action |
| `callout` | `title`, `text` | `icon`, `icon_color`, `variant` | Warning, note, tips cards |
| `split_features` | `title`, `features` | `variant`, `background_color` | Two column feature listings |
| `grid_cards` | `title`, `cards` | `variant`, `cols` | 3-4 feature cards in a grid |
| `headline_subheadline` | `headline` | `subheadline`, `variant` | Minimal text emphasis |
| `definition` | `term`, `definition` | `context`, `variant` | Educational terms or glossary entries |
| `text_block` | `title`, `body` | `variant` | Simple paragraph content |
| `section_divider` | `title` | `kicker`, `subtitle`, `variant` | Slide deck chapter openers |
| `text_columns` | `title`, `columns` | `variant` | Multi-column text blocks |
| `timeline` | `title`, `steps` | `variant` | Paced step-by-step processes |

---

## 2. Data Visualizations (`data-viz/SKILL.md`)

These slides render charts, gauges, tables, and metric indicators:

| Slide Type | Required Parameters | Optional Parameters | Best For |
|---|---|---|---|
| `chart` | `chart_type` (bar/line/pie), `title`, `data` | `caption`, `variant` | General data visualizations |
| `scatter_plot` | `title`, `data` | `x_label`, `y_label`, `variant` | Correlation charts |
| `gauge` | `value` (0-100), `label` | `title`, `variant` | Radial progress, system loads |
| `radar_chart` | `title`, `data` | `variant` | Multidimensional comparisons (skills, tests) |
| `column_chart` | `title`, `data` | `caption`, `variant` | Vertical bar charts |
| `table` | `title`, `headers`, `rows` | `variant` | Tabular data matrices |
| `metric_sparkline` | `value`, `label`, `data` | `trend`, `context`, `variant` | Financial metrics with trend |
| `funnel_chart` | `title`, `steps` | `variant` | Sales conversions |
| `metric_grid` | `title`, `metrics` | `variant` | 2x2 grid of key performance metrics |
| `comparison_bars` | `title`, `comparison` | `description`, `variant` | Direct comparison between two values |
| `progress_rings` | `title`, `items` | `description`, `variant` | concentric circular progress loops |
| `metric_card` | `value`, `label` | `trend`, `context`, `variant` | Prominent metric card |
| `stat_row` | `title`, `stats` | `variant` | Row of 3 key statistics |

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
| `image_stat` | `image_url`, `stat_value`, `stat_label` | `description`, `layout` | Metric overlays on images |
| `image_gallery` | `images` (2-6 urls) | `layout`, `title`, `section_caption` | Portfolio displays |
| `image_collage` | `images`, `title` | `style`, `section_caption` | Creative collections |
| `image_comparison` | `before_image`, `after_image` | `before_label`, `after_label`, `divider_style` | Before/After graphics |

---

## 5. Conversions & Marketing

| Slide Type | Required Parameters | Optional Parameters | Best For |
|---|---|---|---|
| `qr_destination` | `destination_url`, `cta_text`, `heading` | `caption`, `short_url`, `incentive_text` | Converting attention to traffic (QR code) |
