<div align="center">

# SlideForge Rust

**High-performance slide carousel generator** — CLI + MCP server for creating professional Instagram/LinkedIn/TikTok carousels as HTML → PNG.

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![MCP](https://img.shields.io/badge/MCP-1.8.0-purple.svg)](https://modelcontextprotocol.io)

</div>

---

## ✨ Features

- **47 slide types** across 6 categories: Text & Layout, Data Viz, Metrics, Story, Image, Conversion
- **MCP server** — integrate with Claude, Cursor, or any MCP client for AI-driven slide generation
- **CLI** — scriptable, CI-friendly commands for batch generation
- **Design system** — tokens, themes, archetypes, Google Fonts, CSS variables
- **Export pipeline** — HTML → PNG via headless Chromium (1080×1350, 1080×1080, 1080×1920, 1200×628, etc.)
- **Validation** — pre-flight param checks, layout overflow detection, contrast auditing
- **Session persistence** — `configure_design` tokens survive MCP restarts (`~/.slideforge/session_state.json`)

---

## 🚀 Quick Start

### Install (pre-built binary)

```bash
# Download latest release from GitHub Releases
# Or build from source:
cargo install --git https://github.com/your-org/slideforge-rust
```

### CLI Usage

```bash
# Generate a single slide
slideforge-rust generate-slide hero \
  --primary-color '#4F46E5' \
  --params '{"headline":"Ship slides in minutes","subheadline":"AI-powered carousels"}' \
  --override accent='#00FF88'

# Export carousel to PNGs
slideforge-rust export ./carousel.html \
  --output-dir ./exports \
  --slides 4 \
  --preset instagram_portrait

# List all slide types
slideforge-rust list-slides

# Get schema for a slide type
slideforge-rust slide-info hero
```

### MCP Server

```bash
# Start MCP server (stdio transport)
slideforge-rust mcp
```

Configure in your MCP client (Claude Desktop, Cursor, etc.):

```json
{
  "mcpServers": {
    "slideforge": {
      "command": "slideforge-rust",
      "args": ["mcp"]
    }
  }
}
```

**MCP Tools:**
- `configure_design` — set brand color, theme, archetype, platform (persists to disk)
- `generate_slide` — create one slide (validates required params, blocks on missing)
- `render_carousel` — assemble slides into full HTML carousel
- `export_carousel_slides` — render carousel to PNG directory
- `preview_slide` — quick single-slide PNG preview
- `get_slide_type_info` — discover required/optional params + example payload
- `validate_layout` / `validate_design` — audit HTML for overflow, contrast, clipping

---

## 🎨 Slide Types (47 total)

| Category | Types |
|----------|-------|
| **Text & Layout** | hero, feature, list, quote, cta, comparison, stat_row, timeline, callout, split_features, grid_cards, headline_subheadline, definition, text_block, section_divider, text_columns |
| **Data Viz** | chart, scatter_plot, gauge, radar_chart, column_chart, table, metric_sparkline, funnel_chart, metric_grid, comparison_bars, progress_rings |
| **Metrics** | metric_card, stat_row |
| **Story** | problem_solution, myth_fact, case_study_result, testimonial_avatar, before_after_story, logo_cloud, pricing_plan, checklist_action_plan, faq, process_map |
| **Image** | image_caption, image_headline, image_quote, image_callout, image_stat, image_gallery, image_collage, image_comparison |
| **Conversion** | qr_destination |

Each type exposes `required_params`, `optional_params`, `variants`, and an `example` payload via `get_slide_type_info`.

---

## 🔧 Configuration

### Design Tokens (via `configure_design`)

```json
{
  "primary_color": "#4F46E5",
  "visual_theme": "bold",        // editorial, bold, minimal, dark, vibrant, natural
  "preset": "vibrant",           // tonal_spot, vibrant, neutral, monochrome, expressive, fidelity, rainbow, fruit_salad, content
  "archetype": "startup_pitch",  // educator, thought_leader, startup_pitch, brand_storyteller, data_analyst, creator
  "platform": "instagram_portrait",
  "brand_name": "Acme Inc",
  "brand_handle": "@acme"
}
```

Tokens persist to `~/.slideforge/session_state.json` and survive MCP restarts.

### Token Override (CLI only)

```bash
slideforge-rust generate-slide hero \
  --primary-color '#FF5500' \
  --params '{"headline":"Override test"}' \
  --override accent='#00FF88' \
  --override secondary='#222244'
```

Unknown keys warn with typo suggestions (`typo` → `Did you mean 'accent'?`).

---

## 📦 Export Pipeline

```
generate-slide(s) → render-carousel → export
```

| Preset | Aspect | Dimensions | Use Case |
|--------|--------|------------|----------|
| `instagram_portrait` | 4:5 | 1080×1350 | Feed carousels |
| `instagram_square` | 1:1 | 1080×1080 | Feed posts |
| `instagram_story` | 9:16 | 1080×1920 | Stories/Reels |
| `tiktok_vertical` | 9:16 | 1080×1920 | TikTok |
| `linkedin_landscape` | 1.91:1 | 1200×628 | LinkedIn docs |
| `twitter_card` | 16:9 | 1200×675 | Twitter/X |
| `presentation_16_9` | 16:9 | 1920×1080 | Slides |
| `presentation_4_3` | 4:3 | 1024×768 | Slides |

**PNG geometry fix (v0.2.0+):** All presets now render at exact target dimensions (no more 143px height deficit).

---

## 🧪 Validation

```bash
# Validate slide params before rendering
slideforge-rust validate-layout --slide-type hero --params '{"headline":"Test"}'

# Audit rendered HTML for design issues
slideforge-rust validate-design ./carousel.html
```

Checks: overflow, contrast, descender clipping, squished components, distorted images, progress ring thickness, text column width.

---

## 💾 Memory Profile

| Component | RSS (idle) | RSS (export) |
|-----------|------------|--------------|
| MCP server | **~8.5 MB** | — |
| CLI (generate-slide) | **~4 MB** | — |
| Export (Chromium) | — | **~350–800 MB** (one-shot per export) |

Chromium subprocess is spawned per `export`/`preview` call and torn down immediately. No persistent browser pool.

---

## 🏗️ Build from Source

```bash
# Standard (glibc)
cargo build --release

# Musl static (for Alpine/scratch containers)
cargo build --release --target x86_64-unknown-linux-musl
```

Requires: Rust 1.75+, `clang`/`lld` for musl target.

---

## 📄 License

MIT — see [LICENSE](LICENSE).
---

## Agent Integration (AXI §7)

SlideForge ships an installable AI agent skill that provides ambient context at session start — showing slide types, design tokens, and contextual help hints.

### Install the Skill

```bash
# Via npx (recommended)
npx skills add ishan-parihar/slideforge-rust --skill slideforge

# Or download manually
curl -fsSL https://raw.githubusercontent.com/ishan-parihar/slideforge-rust/master/SKILL.md \
  -o ~/.agents/skills/slideforge/SKILL.md
```

### Session Hook (Claude Code)

Add to `~/.claude/settings.json` or project `.claude/settings.json`:

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "slideforge"
          }
        ]
      }
    ]
  }
}
```

At session start, SlideForge prints a compact dashboard:

```
bin: ~/.local/bin/slideforge
description: Instagram/LinkedIn/TikTok carousel generator — 47 slide types, 8 platform presets

slides[47]{type,description}:
  hero,Opening hook with headline and subheadline
  ...

design_tokens:
  primary_color: #4F46E5
  theme: bold
  archetype: startup_pitch

help[4]:
  Run `slideforge list-slides` to see all 47 slide types
  Run `slideforge generate-slide hero --params '{...}'` to create a slide
  Run `slideforge render-carousel slides.json --tokens-file tokens.json` to render
  Run `slideforge export carousel.html --output-dir ./exports` to export PNGs
```

### Session Hook (Codex)

Add to `~/.codex/hooks.json` or project `.codex/hooks.json`:

```json
{
  "SessionStart": "slideforge"
}
```

### Session Hook (OpenCode)

Create `~/.config/opencode/plugins/slideforge.ts`:

```typescript
export default {
  name: "slideforge",
  onSessionStart: async () => {
    const { execSync } = require("child_process");
    return execSync("slideforge").toString();
  },
};
```
