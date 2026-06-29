# SlideForge Design Guide

SlideForge is a Rust-native carousel generation engine and MCP server that produces pixel-perfect social media carousels using a headless Chrome renderer and a full design system derived from perceptual color science (OKLCH).

---

## Architecture

```
configure_design → generate_slide(s) → render_carousel → export_carousel_slides
```

1. **`configure_design`** — Derives a full design token set (palette, typography, spacing, effects) from a single hex brand color using Material Color Utilities / OKLCH math. Saves all session context (brand name, handle, platform, archetype, theme).

2. **`generate_slide`** — Generates self-contained HTML for a single slide using the configured design tokens and a slide type dispatcher.

3. **`render_carousel`** — Assembles individual slide HTML into a full multi-slide HTML document with Google Fonts, CSS variables, progress indicators, and brand footer.

4. **`export_carousel_slides`** — Uses headless Chrome (via `headless_chrome` crate) to render and screenshot each slide as a PNG at the target platform resolution.

---

## Slide Types

| Type | Description | Required Params |
|------|-------------|-----------------|
| `hero` | Hook slide with bold headline | `headline` |
| `feature` | Single feature with icon + title + text | `title`, `description` |
| `list` | Bulleted or numbered list | `title`, `items` |
| `quote` | Testimonial with attribution | `quote` |
| `cta` | Call-to-action with button | `headline`, `button_text` |
| `comparison` | Side-by-side comparison | `title`, `left_label`, `left_items`, `right_label`, `right_items` |
| `stat_row` | Grid of key statistics | `title`, `stats` |
| `timeline` | Step-by-step process | `title`, `steps` |
| `callout` | Highlighted callout card | `title`, `text` |
| `split_features` | Two-column feature list | `title`, `features` |
| `grid_cards` | Grid of icon+title+desc cards | `title`, `cards` |
| `headline_subheadline` | Large headline + body | `headline`, `subheadline` |
| `definition` | Term definition style | `term`, `definition` |
| `text_block` | Title + paragraph body | `title`, `body` |

---

## Visual Themes

| Theme | Style |
|-------|-------|
| `editorial` | Magazine-inspired, textured, sharp |
| `bold` | High-contrast, heavy shadows, gradients |
| `minimal` | Clean whitespace, no decorations |
| `dark` | Moody dark-mode with glass effects |
| `vibrant` | Energetic, gradient-heavy, pill shapes |
| `natural` | Warm, organic, earthy tones |

---

## Background Styles

- `light` — Light surface with subtle mesh gradient
- `dark` — Dark surface with floating shapes
- `hero` — Dark gradient from primary to dark
- `gradient` — Colorful linear gradient

---

## Platforms

| Preset | Size | Use |
|--------|------|-----|
| `instagram_portrait` | 1080×1350 | Instagram carousel (recommended) |
| `instagram_square` | 1080×1080 | Instagram square posts |
| `instagram_story` | 1080×1920 | Instagram/TikTok Stories |
| `tiktok_vertical` | 1080×1920 | TikTok vertical slides |
| `linkedin_landscape` | 1200×627 | LinkedIn document posts |
| `twitter_card` | 1200×675 | Twitter/X card images |
| `facebook_post` | 1200×630 | Facebook posts |
| `presentation_16_9` | 1920×1080 | Presentation slides |
| `presentation_4_3` | 1024×768 | Classic presentation slides |

---

## Archetypes

| Name | Description | Theme |
|------|-------------|-------|
| `educator` | Learning-focused, clean | editorial |
| `thought_leader` | Professional authority, bold | bold |
| `startup_pitch` | Vibrant, high-energy | vibrant/dark |
| `brand_storyteller` | Emotional, imagery-forward | natural |
| `data_analyst` | Data-heavy, structured grids | minimal |
| `creator` | Trendy, dynamic, bold text | vibrant |

---

## Best Practices

- Always call `configure_design` first before generating slides
- Use `list_slide_types` to discover available slide types
- Use `validate_layout` before rendering to catch missing params
- For dark slides (`dark`, `hero`, `gradient`), glass effects are automatically applied
- Use `recommend_color_scheme` to explore palette variations before committing
- The `archetype` setting controls default variants and styling for all slides in the session
- For Instagram carousels, 6-8 slides is optimal. Always end with a CTA slide.

---

## Example Workflow

```json
// 1. Configure design
{ "tool": "configure_design", "primary_color": "#4F46E5", "visual_theme": "bold", "platform": "instagram_portrait", "archetype": "startup_pitch" }

// 2. Generate slides
{ "tool": "generate_slide", "slide_type": "hero", "params": { "headline": "10x Your Growth", "badge": "New" } }
{ "tool": "generate_slide", "slide_type": "stat_row", "params": { "title": "The Numbers", "stats": [{"value": "10×", "label": "ROI"}, {"value": "98%", "label": "Satisfaction"}] } }
{ "tool": "generate_slide", "slide_type": "cta", "params": { "headline": "Start Today", "button_text": "Get Access" } }

// 3. Render carousel
{ "tool": "render_carousel", "slides": [...], "output_path": "carousel.html" }

// 4. Export PNGs
{ "tool": "export_carousel_slides", "html_path": "carousel.html", "output_dir": "./slides", "total_slides": 6 }
```
