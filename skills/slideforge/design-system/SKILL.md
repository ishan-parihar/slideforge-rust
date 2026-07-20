---
name: design-system
description: Use when configuring visual styling, themes, brand archetypes, color palettes, and presets for SlideForge slides.
---

# SlideForge Design System Configuration

Configure design tokens and session brand settings before generating any slides. This ensures consistent color palettes, typography, spacing, and styling across the entire carousel.

## Core Principles

1. **Brand-First Customization:** Never generate slides using raw default configurations. Always derive a cohesive design token set from the brand's primary hex color.
2. **Visual Continuity:** Ensure the same visual theme, archetype, and design tokens are applied to every slide in a session/carousel. Mixing themes (e.g., mixing `bold` with `minimal`) results in jarring design inconsistencies.

---

## Configuration Workflow

Whether using the CLI (`slideforge-rust configure-design`) or the MCP tool (`configure_design`), you must define the design system tokens at the start of a session.

### 1. Primary Brand Color
- Must be a valid 6-digit hex code starting with `#` (e.g., `#6366F1`).
- Avoid web-safe defaults (like `#FF0000` or `#0000FF`). Use sophisticated, tailored colors.
- Use `recommend_color_scheme` (MCP) or `recommend-colors` (CLI) to explore alternative schemes before finalizing.

### 2. Select a Visual Theme
The theme controls the borders, shadows, textures, and typography. Choose exactly one:

| Theme | Aesthetic Style | Best For | Font Scale / Radii |
|---|---|---|---|
| `editorial` | Magazine-inspired, sharp borders, structured text | Authoritative/Education | Serif headers, sharp corners (0px) |
| `bold` | High contrast, deep gradients, heavy shadows | Thought leadership, promotions | Heavy sans, blocky elements |
| `minimal` | Generous whitespace, no decoration, restrained accents | Data analysis, corporate | Clean sans, small padding |
| `dark` | Neon-adjacent colors, glassmorphism surfaces | Tech, startup pitches, crypto | Sans-serif, glossy glass cards |
| `vibrant` | Playful pill shapes, high-saturation gradients | Creator-focused content, lifestyle | Large rounded corners, bubble elements |
| `natural` | Warm organic shapes, earthy colors | Storytellers, wellness, eco brands | Handcrafted accents, soft curves |

### 3. Select a Brand Archetype
Archetypes dictate typography scales, alignments, and layout structures:

- `educator` — Learning-first, clean layouts, left-aligned text, minimal decorations.
- `thought_leader` — Professional authority, big bold statements, gradient headlines.
- `startup_pitch` — Vibrant, high contrast, dark backgrounds by default, high energy.
- `brand_storyteller` — Warm palette, imagery-forward, emotional tone.
- `data_analyst` — Structured grids, stat cards, clean tables, neutral colors.
- `creator` — Saturated colors, bold text, trendy gradients, playful shapes.

### 4. Select a Color Palette Preset
Controls how secondary, accent, and background colors are mapped from the primary color:
- `tonal_spot` (Default) — Tonal variations of the primary color. Classic and safe.
- `vibrant` — Dynamic, highly saturated secondary/accent colors.
- `neutral` — Restrained, muted color variations.
- `monochrome` — Single hue variations (shades of the primary color).
- `expressive` — High contrast, unexpected complementary colors.
- `fidelity` — Keeps secondary and accent colors as close to the original input color as possible.

---

## Actionable Guidelines & Checklists

- [ ] **Hex Format Valid:** Ensure all input colors are strictly formatted as `#RRGGBB`.
- [ ] **Consistent Archetype-Theme Pairs:** Ensure the archetype matches the visual theme (e.g., `startup_pitch` archetype works best with `dark` or `vibrant` themes; `educator` works best with `editorial` or `minimal`).
- [ ] **Single Config Point:** Do not call configure design multiple times with different colors in a single session. Create a file `tokens.json` to reuse tokens across commands.

---

## Professional Designer's Aesthetic Laws

To ensure every slide carousel generated feels premium, elegant, and modern, you must strictly enforce these six aesthetic rules:

### Law 1: The Rule of One Dominant Idea (Breathability)
* Never crowd multiple distinct points on a single slide.
* Limit paragraphs to 3 lines maximum.
* If a bulleted list or card grid has more than 4 items, break it across two sequential slides (e.g., Slide A: "Top 3 Tools...", Slide B: "2 More Tools...").

### Law 2: Perceptual Reading Hierarchy
* The human eye must naturally scan content in this sequence:
  1. **Visual Accent:** Card borders, icons, progress bars, or numbers (high saturation accent colors).
  2. **Headline:** Major hook (display scale, heavy font-weight, primary brand color or crisp white/black).
  3. **Body/Supporting copy:** Explanations (body scale, medium font-weight, secondary/tertiary colors).
  4. **Brand Meta-details:** Handles, URLs, slide numbers (micro scale, lowest hierarchy, secondary text).

### Law 3: Contrast and Color Balance
* Never use saturated primaries (like `#00FFCC` or `#FF3366`) for long body paragraphs; it causes severe eye strain. Use them exclusively for titles, numbers, bullet icons, and borders.
* On light backgrounds, body text must be deep slate/charcoal (e.g. `#1E293B`), never pure black (`#000000`).
* On dark backgrounds, card containers must use subtle translucent glass fill (e.g., `rgba(255,255,255,0.06)`) with a thin semi-transparent border (e.g. `rgba(255,255,255,0.08)`) instead of solid colored card fills.

### Law 4: Typography and Archetype Alignment
Always match the slide's content domain with its visual archetype:
* **Technical/Code/Data:** Use `educator` or `data_analyst` archetype + `minimal` or `editorial` theme. Keeps fonts clean (Space Grotesk) and structural grids clear.
* **Storytelling/Case Studies:** Use `brand_storyteller` + `natural` theme. Soft warm colors, serif details, and higher margins.
* **Viral Marketing/Launches:** Use `startup_pitch` or `creator` + `dark` or `vibrant` theme. Glowing accents, rounded shapes, and deep high-contrast gradients.

### Law 5: Whitespace Preservation
* Keep margins breathing. Enforce a minimum of 40px internal margin (use tokens `var(--space-5)` or `var(--space-6)`) around layout components.
* If text runs close to the header/footer metadata, shorten the text or split the slide. Text clipping at borders is a severe failure.


---

## Example Usage

### CLI
```bash
# Recommend color combinations
slideforge-rust recommend-colors "#4F46E5" --style modern --num-schemes 3

# Configure the session tokens
slideforge-rust configure-design "#4F46E5" --style modern --preset tonal_spot --output tokens.json
```

### MCP Payload
```json
{
  "primary_color": "#4F46E5",
  "visual_theme": "bold",
  "preset": "tonal_spot",
  "archetype": "startup_pitch",
  "brand_name": "Acme Corp",
  "brand_handle": "@acmecorp"
}
```
