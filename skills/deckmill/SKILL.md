---
name: deckmill
description: Use when generating professional social media carousels or presentation slides using the Deckmill CLI/MCP tool. Routes to design settings, content composition, rendering/export pipelines, and validation guides.
---

# Deckmill Designer — Meta-Skill Suite

Deckmill is a Rust-native CLI and MCP system that generates pixel-perfect carousel slides. It derives colors and styling from perceptual color science (OKLCH) and outputs HTML compiled and exported to PNGs using the embedded Blitz renderer (stylo layout + vello-cpu raster, no browser needed). Google Fonts are vendored deterministically (data-URI woff2 + on-disk cache) so exports are identical regardless of network state.

This root router navigates you to the specialized sub-skills for using Deckmill in the most effective manner. Deckmill exposes **46 active slide types** across 5 layout families.

---

## Navigation Protocol

Only this root router is always in context. To use this skill without bloat:
- **JUMP** when you know what you need: go directly to a child leaf.
- **WALK** when you don't: descend router by router to find the correct parameters.
- **RE-ROUTE** on task shift: return here to choose a different path (e.g. going from slide generation to rendering/exporting).

### Skill Map Directory

Descend into the child skill matching your current step (relative to this suite's root):

1. **[Design System Settings](design-system/SKILL.md)**
   - *Use when:* Starting a session, setting brand colors, selecting visual themes, archetypes, color presets, or typology bundles.
2. **[Slide Content Composition Router](slide-composition/SKILL.md)**
   - *Use when:* Choosing slide types and formatting parameters (text layouts, data visualizations, story flows, image slides).
3. **[Rendering & Export Pipeline](rendering-export/SKILL.md)**
   - *Use when:* Assembling individual slide components into an HTML carousel document and rendering to high-res PNGs.
4. **[Validation & Layout Fixing](validation-fixing/SKILL.md)**
   - *Use when:* Auditing slide parameters and visual layouts for overflows, line-clipping, hard-cap violations, and contrast.

---

## Infrastructure Notes (Blitz Renderer Era)

- **No Chromium:** PNG export runs fully in-process (stylo layout + vello-cpu). The `setup` command is a no-op. No browser download or headless-chrome dependency exists.
- **Deterministic fonts:** Google Fonts stylesheets are fetched once, rewritten to inline `data:font/woff2` URLs, and cached under `$DECKMILL_FONT_CACHE` (default `~/.cache/deckmill/fonts`). Text never falls back per-glyph due to network races. On a fully offline machine, cached faces are reused; uncached styles degrade to local fallbacks rather than failing the export.
- **Selector caveat:** the stylo engine does **not** support the `:has()` CSS selector. Author custom HTML/CSS with explicit marker classes instead of relying on `:has()`.
- **AXI-compliant CLI:** errors print to stdout with stable exit codes (usage=2, validation=1); `deckmill skill-guide --check` is the CI gate that keeps the committed SKILL.md in sync with the live command surface.
- **Session hook:** a `SessionStart` hook (merged via `deckmill session-hook --merge`) prints a compact dashboard (recent decks, validator health) so agents get ambient context.

---

## Actionable Guidelines & Checklists

- [ ] **First Action:** Always configure the design system tokens at the beginning of a slide generation session.
- [ ] **Sequence:** Follow the sequence: Configure Design → Compose Slides → Assemble & Render → Validate Layout & Design → Export PNGs.
- [ ] **Aesthetics Rule:** Never mix visual themes in a single carousel. Keep brand continuity.
- [ ] **Hard Caps:** Prefer validator-enforced hard caps over silent truncation — the renderer never writes `…` into metrics; over-cap inputs are hard errors.
