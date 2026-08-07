---
name: slide-composition
description: Use when choosing, parameters-filling, or structuring any of the 46 slide types. Directs you to text layouts, data visualizations, story flows, or image layouts.
---

# SlideForge Slide Composition Router

This router assists you in selecting and structuring slide content. SlideForge supports **46 active slide types** (a few legacy names are deprecated redirects to their replacements). Descend into the correct sub-skill based on your content requirements.

## Routing Directory

To view templates, mandatory parameters, and specific layout limits, read the relevant child skill path (relative to this suite's root):

### 1. [Text & Layouts](text-layouts/SKILL.md)
- **When to descend:** Your content consists of textual headers, lists, quotes, definitions, text blocks, or timelines.
- **Includes slide types:** `hero`, `quote`, `split_features`, `definition`, `text_block`, `timeline`.

### 2. [Data Visualization](data-viz/SKILL.md)
- **When to descend:** You need to show quantitative metrics, numeric comparisons, tables, charts, progress status, or sparklines.
- **Includes slide types:** `chart`, `scatter_plot`, `gauge`, `radar_chart`, `table`, `funnel_chart`, `metric_grid`, `comparison_bars`, `progress_rings`.

### 3. [Story & Educational Flows](story-flows/SKILL.md)
- **When to descend:** Your slide needs to structure structured learning paths, lists of actions, comparisons, problem/solution matches, myth/fact checks, pricing, or case studies.
- **Includes slide types:** `problem_solution`, `myth_fact`, `case_study_result`, `testimonial_avatar`, `before_after_story`, `logo_cloud`, `pricing_plan`, `faq`, `process_map`.

### 4. [Image Integration](image-integration/SKILL.md)
- **When to descend:** You are embedding photography or graphics into layout columns, collages, comparisons, galleries, or background grids.
- **Includes slide types:** `image_caption`, `image_headline`, `image_quote`, `image_callout`, `image_gallery`, `image_collage`, `image_comparison`.

---

## Slide Selection Navigation Protocol

1. Read the [Slide Types Manifest](../references/slide-types-manifest.md) for a comprehensive list of all 46 slide types, their required parameters, and best-for use cases.
2. Run `list-slides` (CLI) or `list_slide_types` (MCP) to confirm the available list of types.
3. Run `slide-info <type>` (CLI) or `get_slide_type_info` (MCP) to retrieve the JSON schema containing required and optional parameters for a specific slide type.
4. Select the child skill path above that matches your slide category.
5. Compose the slide parameters JSON payload strictly adhering to the child skill's formatting rules.

