---
name: slideforge
description: >-
  Create production-quality carousel slides using the SlideForge CLI/MCP tool.
  Use this skill whenever the user wants to create carousel slides, social media
  posts (Instagram, TikTok, LinkedIn), presentation decks, or any multi-slide
  visual content. Also use when the user mentions "carousel", "slide deck",
  "Instagram post", "TikTok content", "slide generator", or wants to generate
  HTML slides with design validation. SlideForge supports 47 slide types across
  6 categories (Text, Data Viz, Metrics, Story, Image, Conversion), 6 brand
  archetypes, 6 visual themes, and 4 aspect ratios with a build-time validator
  that catches layout, contrast, and composition issues.
---

# SlideForge — Carousel Slide Generator

SlideForge is a Rust-native CLI and MCP tool that generates production-quality HTML carousel slides. It produces validated, export-ready carousels across 47 slide types with a build-time quality gate.

## When to Use This Skill

Use this skill whenever the user wants to:
- Create carousel slides for social media (Instagram, TikTok, LinkedIn, Twitter)
- Generate presentation decks or visual content
- Build multi-slide HTML carousels with images, charts, or data
- Export slides as PNG images
- Validate slide layouts for design quality

## Prerequisites

SlideForge must be installed. Check if it's available:

```bash
slideforge --version
```

If not installed, install with:

```bash
curl -fsSL https://raw.githubusercontent.com/ishan-parihar/slideforge-rust/master/scripts/install-slideforge-rust.sh | bash
```

After install, ensure `~/.local/bin` is in PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

## Architecture Overview

SlideForge works in a 4-step pipeline:

1. **Configure Design** — Generate design tokens (colors, fonts, spacing) from a brand color
2. **Generate Slides** — Create individual slide HTML for each slide type
3. **Render Carousel** — Assemble slides into a full carousel HTML document
4. **Export** — Optionally export to PNG images via headless Chrome

The tool is available as both a **CLI** (18 commands) and an **MCP server** (18 tools) with identical functionality.

## CLI Quick Reference

### Discovery Commands

```bash
slideforge list-slides                 # List all 47 slide types
slideforge list-themes                 # List 6 visual themes
slideforge list-platforms              # List 9 export platforms
slideforge list-archetypes             # List 6 brand archetypes
slideforge slide-info <slide_type>     # Get schema for a specific slide type
slideforge skill-guide                 # Print the full design guide
```

### Generation Commands

```bash
# Step 1: Configure design tokens
slideforge configure-design "#6366F1" --style modern --preset tonal_spot --output tokens.json

# Step 2: Generate a slide (JSON output)
slideforge generate-slide hero \
  --primary-color "#6366F1" \
  --theme editorial \
  --bg-style light \
  --params '{"headline":"Hello World","subheadline":"My first slide"}' \
  --output slide1.json

# Step 3: Render carousel (assemble slides into HTML)
slideforge render-carousel slides.json \
  --tokens-file tokens.json \
  --brand-name "MyBrand" \
  --topic "Product Launch" \
  --output carousel.html

# Step 4: Export to PNGs
slideforge export carousel.html --slides 5 --output-dir ./exports
```

### Image Commands

```bash
# Convert a local image to a data URI for embedding in slides
slideforge embed-image photo.png

# Quick PNG preview of a single slide
slideforge preview-slide slide.html --output preview.png
```

### Validation Commands

```bash
# Validate slide params before rendering
slideforge validate-layout hero --params '{"headline":"Test"}'

# Validate rendered carousel HTML for design issues
slideforge validate-design carousel.html

# Validate a carousel composition against arc structure and constraints
slideforge validate-composition --file composition-request.json
```

The `validate-composition` command checks:
- **Pool membership** — every slide_type in the composition must be in one of the arc_structure pools
- **Bg rhythm alternation** — light/dark backgrounds must alternate for visual contrast
- **No consecutive same-type** — same slide_type cannot appear back-to-back (DLD rhythm)
- **Max consecutive dataviz** — limits how many data-viz slides can appear in a row
- **Min/max slide count** — composition must fall within preset's ideal range

Input format (JSON):
```json
{
  "composition": [
    {"slide_type": "hero", "arc": "hook"},
    {"slide_type": "problem_solution", "arc": "evidence"},
    {"slide_type": "cta", "arc": "action"}
  ],
  "arc_structure": {
    "hook": {"types": ["hero"], "pool": [], "count": {"min": 1, "max": 6}},
    "evidence": {"types": [], "pool": ["problem_solution", "grid_cards"], "count": {"min": 4, "max": 6}},
    "action": {"types": ["cta"], "pool": [], "count": {"min": 1, "max": 6}}
  },
  "constraints": {
    "max_consecutive_dataviz": 2,
    "min_slides": 5,
    "max_slides": 12
  }
}
```

## MCP Server

Start the MCP server for AI agent integration:

```bash
slideforge mcp
```

MCP configuration for Claude Desktop, Cursor, etc.:

```json
{
  "mcpServers": {
    "slideforge": {
      "command": "slideforge",
      "args": ["mcp"]
    }
  }
}
```

## Slide Type Catalog

### Text & Layout (16 types)
`hero`, `feature`, `list`, `quote`, `cta`, `comparison`, `stat_row`, `timeline`, `callout`, `split_features`, `grid_cards`, `headline_subheadline`, `definition`, `text_block`, `section_divider`, `text_columns`

### Data Visualization (11 types)
`chart`, `scatter_plot`, `gauge`, `radar_chart`, `column_chart`, `table`, `metric_sparkline`, `funnel_chart`, `metric_grid`, `comparison_bars`, `progress_rings`

### Story (10 types)
`problem_solution`, `myth_fact`, `case_study_result`, `testimonial_avatar`, `before_after_story`, `logo_cloud`, `pricing_plan`, `checklist_action_plan`, `faq`, `process_map`

### Image (8 types)
`image_caption`, `image_headline`, `image_quote`, `image_callout`, `image_stat`, `image_gallery`, `image_collage`, `image_comparison`

### Conversion (1 type)
`qr_destination`

### Metrics (1 type)
`metric_card`

## Key Parameters

### Visual Themes
- `editorial` — Clean, magazine-inspired, sharp edges
- `bold` — High-contrast, dynamic, strong shadows
- `minimal` — Restrained, generous whitespace
- `dark` — Dark-mode-first, glassmorphism
- `vibrant` — Saturated, playful radii
- `natural` — Organic shapes, earthy palette

### Background Styles
- `light` — Light surface
- `dark` — Dark surface
- `gradient` — Gradient background
- `mesh` — Mesh gradient
- `hero` — Hero gradient

### Color Presets
- `tonal_spot` — Tonal variations of primary
- `vibrant` — High-saturation palette
- `neutral` — Muted, neutral palette
- `monochrome` — Single-hue variations
- `expressive` — Bold, expressive palette
- `fidelity` — True-to-brand colors

### Aspect Ratios
- `4:5` — Instagram portrait (default)
- `9:16` — Instagram Story / TikTok
- `3:4` — Vertical
- `1:1` — Square

### Platforms
`instagram_portrait`, `instagram_square`, `instagram_story`, `tiktok_vertical`, `linkedin_landscape`, `twitter_card`, `facebook_post`, `presentation_16_9`, `presentation_4_3`

## Workflow Patterns

### Pattern 1: Single-Slide Quick Preview

For iterating on one slide design:

```bash
# Generate a hero slide
slideforge generate-slide hero \
  --primary-color "#6366F1" \
  --params '{"headline":"Test Headline"}' \
  --output slide.json

# Extract the HTML and preview it
python3 -c "import json; print(json.load(open('slide.json'))['html'])" > slide.html
slideforge preview-slide slide.html --output preview.png
```

### Pattern 2: Multi-Slide Carousel

For a full carousel:

```bash
# 1. Configure design
slideforge configure-design "#6366F1" --output tokens.json

# 2. Generate each slide (save to individual JSON files)
slideforge generate-slide hero --tokens-file tokens.json \
  --params '{"headline":"Opening Hook"}' --output s1.json
slideforge generate-slide feature --tokens-file tokens.json \
  --params '{"title":"Key Feature","description":"Why it matters"}' --output s2.json
slideforge generate-slide cta --tokens-file tokens.json \
  --params '{"headline":"Get Started","button_text":"Try Now"}' --output s3.json

# 3. Combine into a slides array
echo "[$(cat s1.json),$(cat s2.json),$(cat s3.json)]" > slides.json

# 4. Render the carousel
slideforge render-carousel slides.json \
  --tokens-file tokens.json \
  --brand-name "MyBrand" \
  --output carousel.html

# 5. Export to PNGs
slideforge export carousel.html --slides 3 --output-dir ./exports
```

### Pattern 3: Image-Heavy Slides

For slides with local images:

```bash
# Convert local image to data URI
DATA_URI=$(slideforge embed-image photo.png | python3 -c "import json,sys; print(json.load(sys.stdin)['data_uri'])")

# Use the data URI in an image slide
slideforge generate-slide image_headline \
  --primary-color "#6366F1" \
  --params "{\"image_url\":\"$DATA_URI\",\"headline\":\"My Photo\",\"subheadline\":\"Caption here\"}" \
  --output slide.json
```

### Pattern 4: Validate Before Export

Always validate before exporting:

```bash
# Validate params
slideforge validate-layout hero --params '{"headline":"Test"}'

# Validate rendered HTML
slideforge validate-design carousel.html
```

The validator catches: layout overflow, contrast issues, image visibility, progress-slider spacing, full-bleed centering, and more. Fix all errors before exporting.

### Pattern 5: Campaign Presets

For pre-built campaign carousels with emotional arc templates, use the preset system:

```bash
# Generate all 28 campaign presets
python3 generate_preset_slides.py

# Generate specific presets
python3 generate_preset_slides.py --only announcement,authority_introduction

# Validate compositions without generating
python3 generate_preset_slides.py --composition-mode validate

# Check type diversity across generated carousels
python3 generate_preset_slides.py --test-diversity

# List available presets
python3 generate_preset_slides.py --list
```

The preset system provides:
- **28 campaign presets** across 7 categories (announcement, authority, origin, aspiration, credibility, proof, process, product, evidence, contrast, exposure, skill, deep_dive, principle, technique, wisdom, legacy, paradigm, transformation, event, urgency, collective, outrage, story, emotional, guided, nostalgia, underdog)
- **Pool-based composition** — each preset declares `arc_structure` with hook/evidence/action pools
- **Repeatable blocks** — `repeat_count` lets the AI agent decide iteration count within bounds
- **DLD rhythm enforcement** — no consecutive same-type slides, alternating bg_style
- **Content-fill fallback** — `fill_params()` provides demo values when `preset_content()` doesn't have an entry for a slot

The generation script reads from `docs/presets/campaign-presets.json` and writes carousels to `dist/presets/*.html`.

## Common Params by Slide Type

### hero
```json
{
  "headline": "Your headline here",
  "subheadline": "Supporting text",
  "badge": "NEW",
  "variant": "centered"
}
```

### feature
```json
{
  "title": "Feature name",
  "description": "What it does",
  "icon": "rocket",
  "variant": "icon-top"
}
```

### image_headline
```json
{
  "image_url": "https://example.com/photo.jpg",
  "headline": "Overlay text",
  "subheadline": "Sub text",
  "overlay_position": "bottom"
}
```

### qr_destination
```json
{
  "destination_url": "https://example.com",
  "cta_text": "Scan to read",
  "heading": "Read the full article",
  "caption": "A practical guide",
  "short_url": "example.com/guide"
}
```

### chart
```json
{
  "chart_type": "bar",
  "title": "Revenue Growth",
  "data": [
    {"label": "Q1", "value": 40},
    {"label": "Q2", "value": 65},
    {"label": "Q3", "value": 90}
  ]
}
```

### split_features
```json
{
  "title": "Performance",
  "image_url": "https://example.com/photo.jpg",
  "features": [
    {"icon": "bolt", "title": "Fast", "description": "Sub-100ms"},
    {"icon": "shield", "title": "Secure", "description": "Bank-grade"}
  ]
}
```

## Tips for Best Results

1. **Always configure design first** — The `configure-design` command generates tokens that ensure consistent colors, fonts, and spacing across all slides.

2. **Use `--tokens-file` for multi-slide carousels** — This ensures all slides share the same design system.

3. **Validate before export** — Run `validate-design` on the final carousel HTML to catch layout issues before PNG export.

4. **Use data URIs for local images** — The `embed-image` command converts local files to base64 data URIs that work reliably in exported HTML.

5. **Preview individual slides** — Use `preview-slide` to iterate on a single slide before rendering the full carousel.

6. **Check the design guide** — Run `slideforge skill-guide` for the full documentation including best practices and example workflows.

7. **Error messages list valid values** — If you pass an invalid enum (theme, bg_style, preset, platform), the error message lists all valid values.

8. **The `generate-slide` response includes validation** — Check the `validation` field in the JSON output for warnings about missing required params.

## Troubleshooting

### "command not found: slideforge"
The binary isn't in PATH. Fix with:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

### "binary verification failed" during install
The download may have failed silently. Try downloading manually:
```bash
curl -fsSL -o ~/.local/bin/slideforge "https://github.com/ishan-parihar/slideforge-rust/releases/download/v0.2.0/slideforge-x86_64-linux-gnu"
chmod +x ~/.local/bin/slideforge
```

### Chrome/Chromium not found during export
PNG export requires headless Chrome. Install it:
```bash
# Ubuntu/Debian
sudo apt install chromium-browser

# Or Google Chrome
sudo apt install google-chrome-stable
```

### Slides look off-center on 9:16
This was fixed in v0.2.0. Ensure you're using the latest version:
```bash
slideforge --version
```

## MCP Tool Reference

All 19 CLI commands have matching MCP tools with identical parameters:

| MCP Tool | Equivalent CLI | Purpose |
|----------|---------------|---------|
| `configure_design` | `configure-design` | Generate design tokens |
| `generate_slide` | `generate-slide` | Generate a single slide |
| `render_carousel` | `render-carousel` | Assemble slides into carousel |
| `export_carousel_slides` | `export` | Export to PNGs |
| `list_slide_types` | `list-slides` | List 47 slide types |
| `get_slide_type_info` | `slide-info` | Get slide schema |
| `validate_layout` | `validate-layout` | Validate params |
| `validate_design` | `validate-design` | Validate HTML |
| `validate_composition` | `validate-composition` | Validate arc structure + constraints |
| `embed_local_image` | `embed-image` | Local image → data URI |
| `preview_slide` | `preview-slide` | Single-slide PNG preview |
| `load_carousel_skill` | `skill-guide` | Load design guide |
| `list_themes` | `list-themes` | List visual themes |
| `list_platforms` | `list-platforms` | List export platforms |
| `list_archetypes` | `list-archetypes` | List brand archetypes |
| `recommend_color_scheme` | `recommend-colors` | Recommend palettes |
| `get_slide_types_for_context` | `slide-types-for-context` | Context-based recommendations |
| `validate_and_fix` | `validate-layout` | Validate + auto-fix |
| `design_system` | `configure-design` | Stateless token derivation |
