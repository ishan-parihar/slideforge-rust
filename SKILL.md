---
name: slideforge
description: >
  Generate professional social media carousels or presentation slides using the
  SlideForge CLI/MCP tool. Routes to design settings, content composition,
  rendering/export pipelines, and validation guides.
---

# SlideForge Skill

Generate Instagram/LinkedIn/TikTok carousels as HTML → PNG with AI-grade design systems.

<!-- Static skill — regenerate from CLI: slideforge skill-guide -->
<!-- Install: npx skills add <owner/slideforge-rust> --skill slideforge -->
<!-- CI check: diff <(slideforge skill-guide) SKILL.md && exit 1 -->
<!-- Install: npx skills add <owner/slideforge-rust> --skill slideforge -->

## Quick Start

```bash
# Generate a single slide
slideforge generate-slide hero --primary-color '#4F46E5' \
  --params '{"headline":"Hello","subheadline":"World"}'

# Render a full carousel
slideforge render-carousel slides.json --tokens-file tokens.json --output carousel.html

# Export to PNGs
slideforge export carousel.html --output-dir ./exports --slides 4 --preset instagram_portrait
```

## Key Commands

| Command | Description |
|---------|-------------|
| `slideforge list-slides` | Show all slide types |
| `slideforge list-platforms` | Show export presets |
| `slideforge configure-design <hex>` | Generate design tokens |
| `slideforge generate-slide <type>` | Create a slide |
| `slideforge render-carousel <file>` | Assemble slides into HTML |
| `slideforge export <html>` | Export to PNG directory |
| `slideforge skill-guide` | Full design documentation |
| `slideforge mcp` | Start MCP server for AI agents |

## Slide Types

hero, feature, list, quote, cta, comparison, stat_row, timeline, callout, split_features, grid_cards, headline_subheadline, definition, text_block, chart, column_chart, metric_card, metric_grid, funnel_chart, progress_rings, comparison_bars, table, qr_destination, and more. Run `slideforge list-slides` for the full catalog.

## Design System

1. Run `slideforge configure-design '#4F46E5'` to generate tokens
2. Pass `--tokens-file tokens.json` to `generate-slide` for consistent branding
3. Use `--override accent=#FF5500` to tweak individual colors

## Platforms

instagram_portrait (1080x1350), instagram_square (1080x1080), instagram_story (1080x1920), tiktok_vertical (1080x1920), linkedin_landscape (1200x628), twitter_card (1200x675)
