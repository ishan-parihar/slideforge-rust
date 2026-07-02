<div align="center">

# SlideForge Rust

**A Rust-native carousel slide generator, CLI, and MCP server for AI agents.**

Generate production-quality HTML carousels across 47 slide types, 6 archetypes, 6 themes, and 4 aspect ratios — with a build-time validator that catches layout, contrast, and composition issues before export.

[![Release](https://img.shields.io/github/v/release/ishan-parihar/slideforge-rust?color=blue)](https://github.com/ishan-parihar/slideforge-rust/releases)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Slide Types](https://img.shields.io/badge/slide%20types-47-blue)](#slide-types)
[![MCP](https://img.shields.io/badge/MCP-18%20tools-purple)](#mcp-tools)

</div>

---

## Quick Start

### Install

```bash
curl -fsSL https://raw.githubusercontent.com/ishan-parihar/slideforge-rust/master/scripts/install-slideforge-rust.sh | bash
```

This downloads the pre-built binary to `~/.local/bin/slideforge` and verifies it works.

<details>
<summary>Other install options</summary>

```bash
# Install to a specific directory
curl -fsSL https://raw.githubusercontent.com/ishan-parihar/slideforge-rust/master/scripts/install-slideforge-rust.sh | bash -s -- --bin-dir /usr/local/bin

# Install a specific version
curl -fsSL https://raw.githubusercontent.com/ishan-parihar/slideforge-rust/master/scripts/install-slideforge-rust.sh | bash -s -- --version v0.1.0

# Install from a local binary
./scripts/install-slideforge-rust.sh --local ./dist/slideforge-x86_64-linux-gnu
```
</details>

### Generate Your First Carousel

```bash
# 1. Configure design tokens
slideforge configure-design "#6366F1" --style modern --preset tonal_spot --output tokens.json

# 2. Generate a hero slide
slideforge generate-slide hero \
  --primary-color "#6366F1" \
  --theme editorial \
  --params '{"headline":"Introducing SlideForge","subheadline":"Carousel slides for AI agents"}' \
  --output slide1.json

# 3. Render the carousel
slideforge render-carousel slide1.json --tokens-file tokens.json --output carousel.html

# 4. Export to PNGs
slideforge export carousel.html --output-dir ./exports --slides 1
```

### Use as MCP Server

Add to your MCP client config (Claude Desktop, Cursor, etc.):

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

Then ask your AI agent: *"Create a 5-slide carousel about AI productivity tools using SlideForge."*

---

## Features

### 47 Slide Types

| Category | Slide Types |
|----------|------------|
| **Text & Layout** | `hero`, `feature`, `list`, `quote`, `cta`, `comparison`, `stat_row`, `timeline`, `callout`, `split_features`, `grid_cards`, `headline_subheadline`, `definition`, `text_block`, `section_divider`, `text_columns` |
| **Data Viz** | `chart`, `scatter_plot`, `gauge`, `radar_chart`, `column_chart`, `table`, `metric_sparkline`, `funnel_chart`, `metric_grid`, `comparison_bars`, `progress_rings` |
| **Metrics** | `metric_card`, `stat_row` |
| **Story** | `problem_solution`, `myth_fact`, `case_study_result`, `testimonial_avatar`, `before_after_story`, `logo_cloud`, `pricing_plan`, `checklist_action_plan`, `faq`, `process_map` |
| **Image** | `image_caption`, `image_headline`, `image_quote`, `image_callout`, `image_stat`, `image_gallery`, `image_collage`, `image_comparison` |
| **Conversion** | `qr_destination` |

### 18 MCP Tools ( matched 1:1 by 18 CLI commands)

| MCP Tool | CLI Command | Description |
|----------|-------------|-------------|
| `configure_design` | `configure-design` | Generate design tokens from a brand color |
| `design_system` | `configure-design` | Stateless token derivation |
| `generate_slide` | `generate-slide` | Generate HTML for a single slide |
| `render_carousel` | `render-carousel` | Assemble slides into a carousel HTML doc |
| `export_carousel_slides` | `export` | Export carousel HTML to PNG images |
| `list_slide_types` | `list-slides` | List all 47 slide types with schemas |
| `get_slide_type_info` | `slide-info` | Get detailed schema for a slide type |
| `get_slide_types_for_context` | `slide-types-for-context` | Get recommended types for a context |
| `list_platforms` | `list-platforms` | List export platforms (Instagram, TikTok, etc.) |
| `list_archetypes` | `list-archetypes` | List brand archetypes |
| `list_themes` | `list-themes` | List visual themes |
| `validate_layout` | `validate-layout` | Validate slide params before rendering |
| `validate_and_fix` | `validate-layout` | Validate and auto-fix missing params |
| `validate_design` | `validate-design` | Validate carousel HTML for design issues |
| `recommend_color_scheme` | `recommend-colors` | Recommend color schemes from a brand color |
| `embed_local_image` | `embed-image` | Convert local image to base64 data URI |
| `preview_slide` | `preview-slide` | Render single slide to PNG preview |
| `load_carousel_skill` | `skill-guide` | Load the design guide documentation |

### Build-Time Validator

The validator is the primary quality gate — it catches layout, contrast, and composition issues at build time so every slide is production-ready:

- **Layout**: component overlap, frame overflow, grid orphan cells, full-bleed centering
- **Contrast**: WCAG AA 4.5:1 text contrast, image-background text legibility
- **Images**: opacity, aspect distortion, caption overlay collisions, full-bleed fill
- **Typography**: tiny text, one-word-per-line risk, descender clipping, text constriction
- **Aspect ratios**: progress-slider spacing, overlay collision, bg-image mask bands

### Multi-Platform Export

| Platform | Aspect Ratios |
|----------|--------------|
| `instagram_portrait` | 4:5, 3:4, 1:1 |
| `instagram_square` | 1:1, 4:5 |
| `instagram_story` | 9:16, 3:4 |
| `tiktok_vertical` | 9:16 |
| `linkedin_landscape` | 4:5, 1:1 |
| `twitter_card` | 1:1, 16:9 |
| `facebook_post` | 4:5, 1:1 |
| `presentation_16_9` | 16:9 |
| `presentation_4_3` | 4:3 |

---

## CLI Usage

```bash
slideforge --help                    # See all 18 commands
slideforge list-slides               # List 47 slide types
slideforge list-themes               # List 6 visual themes
slideforge slide-info hero           # Get hero slide schema
slideforge generate-slide hero --primary-color "#6366F1" --params '{"headline":"Hello"}'
slideforge render-carousel slides.json --tokens-file tokens.json --output carousel.html
slideforge validate-design carousel.html  # Validate for design issues
slideforge embed-image photo.png     # Convert local image to data URI
slideforge preview-slide slide.html  # Quick PNG preview of one slide
slideforge export carousel.html --slides 5 --output-dir ./exports
```

## MCP Usage

Start the MCP server:

```bash
slideforge mcp
```

Without a subcommand, `slideforge` also starts the MCP server over stdio.

### MCP Tools

All 18 CLI commands have matching MCP tools with identical parameter schemas. AI agents can:
- Discover slide types via `list_slide_types` and `get_slide_type_info`
- Generate slides via `generate_slide` (with pre-flight validation in the response)
- Validate via `validate_layout` (params) and `validate_design` (HTML)
- Embed local images via `embed_local_image` (PNG/JPEG/GIF/WebP/SVG → data URI)
- Preview single slides via `preview_slide` (quick PNG without full export)
- Load the design guide via `load_carousel_skill`

---

## Build from Source

### Prerequisites

- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Chrome/Chromium (for PNG export via headless browser)

### Build

```bash
git clone https://github.com/ishan-parihar/slideforge-rust.git
cd slideforge-rust
cargo build --release
```

The binary will be at `target/release/slideforge-rust`.

### Build Static Binary (musl)

For a statically-linked binary (portable across Linux distros):

```bash
rustup target add x86_64-unknown-linux-musl
# Requires musl-tools: sudo apt install musl-tools
cargo build --release --target x86_64-unknown-linux-musl
```

### Run Tests

```bash
cargo test                    # 60 unit tests
cargo fmt --check             # Format check
python3 test_full_scope_rust.py  # Generate 47 test carousels
```

---

## Architecture

```
src/
├── main.rs            — CLI entry point (18 subcommands)
├── mcp_server.rs      — MCP server (18 tools, stdio transport)
├── components.rs      — 47 slide type generators (7,700+ lines)
├── slides.rs          — Carousel assembly + CSS + overlay chrome
├── layouts.rs         — Layout primitives (slide_base, hero, split, grid, etc.)
├── validate.rs        — Build-time validator (20+ checks)
├── design_system.rs   — OKLCH color science + typography + tokens
├── slide_registry.rs  — 47 slide type schemas
├── platforms.rs       — 9 export platforms + aspect ratio resolution
├── archetypes.rs      — 6 brand archetypes with per-slide-type presets
├── blocks.rs          — Reusable HTML blocks (badges, buttons, cards)
├── dataviz.rs         — Chart rendering (bar, line, radar, gauge, etc.)
├── effects.rs         — Glassmorphism, noise, gradient effects
└── export.rs          — Headless Chrome PNG export
```

---

## Quality Gates

Run before release:

```bash
cargo fmt --check
cargo test
cargo build --release
python3 test_full_scope_rust.py
```

The full-scope HTML fixtures are written to `test-drafts/full-scope-test-output-rust/`.

### Validator Results

All 47 generated slides pass `validate_design` with **0 errors and 0 warnings**:

```bash
slideforge validate-design test-drafts/full-scope-test-output-rust/carousel_*.html
```

---

## Release

Create a GitHub release:

```bash
cargo build --release
strip target/release/slideforge-rust
cp target/release/slideforge-rust dist/slideforge-x86_64-linux-gnu

git tag v0.2.0
git push origin feat/vectoric-scaling --tags
# Upload dist/slideforge-x86_64-linux-gnu to the GitHub release
```

---

## License

MIT

## Links

- [Releases](https://github.com/ishan-parihar/slideforge-rust/releases)
- [Design Guide](DESIGN-GUIDE.md)
- [Install Script](scripts/install-slideforge-rust.sh)
