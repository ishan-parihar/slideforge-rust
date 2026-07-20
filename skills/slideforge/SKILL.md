---
name: slideforge
description: Use when generating professional social media carousels or presentation slides using the SlideForge CLI/MCP tool. Routes to design settings, content composition, rendering/export pipelines, and validation guides.
---

# SlideForge Designer — Meta-Skill Suite

SlideForge is a Rust-native CLI and MCP system that generates pixel-perfect carousel slides. It derives colors and styling from perceptual color science (OKLCH) and outputs HTML compiled and exported to PNGs using a headless Chromium browser.

This root router navigates you to the specialized sub-skills for using SlideForge in the most effective manner.

---

## Navigation Protocol

Only this root router is always in context. To use this skill without bloat:
- **JUMP** when you know what you need: go directly to a child leaf.
- **WALK** when you don't: descend router by router to find the correct parameters.
- **RE-ROUTE** on task shift: return here to choose a different path (e.g. going from slide generation to rendering/exporting).

### Skill Map Directory

Descend into the child skill matching your current step:

1. **[Design System Settings](file:///home/ishanp/.agents/skills/slideforge/design-system/SKILL.md)**
   - *Use when:* Starting a session, setting brand colors, selecting visual themes, archetypes, and color presets.
2. **[Slide Content Composition Router](file:///home/ishanp/.agents/skills/slideforge/slide-composition/SKILL.md)**
   - *Use when:* Choosing slide types and formatting parameters (text layouts, data visualizations, story flows, image slides).
3. **[Rendering & Export Pipeline](file:///home/ishanp/.agents/skills/slideforge/rendering-export/SKILL.md)**
   - *Use when:* Assembling individual slide components into an HTML carousel document and rendering to high-res PNGs.
4. **[Validation & Layout Fixing](file:///home/ishanp/.agents/skills/slideforge/validation-fixing/SKILL.md)**
   - *Use when:* Auditing slide parameters and visual layouts for overflows, line-clipping, and contrast.

---

## Actionable Guidelines & Checklists

- [ ] **First Action:** Always configure the design system tokens at the beginning of a slide generation session.
- [ ] **Sequence:** Follow the sequence: Configure Design → Compose Slides → Assemble & Render → Validate Layout & Design → Export PNGs.
- [ ] **Aesthetics Rule:** Never mix visual themes in a single carousel. Keep brand continuity.
