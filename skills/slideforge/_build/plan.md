# Build Plan: SlideForge Meta-Skill Suite

This plan details the capability map, tree architecture, design conventions, and build status checklist for the SlideForge meta-skill suite.

---

## 1. Capability Map (Spec)

Operations that the meta-skill must teach agents to perform:
- **C1: Initialize Brand Design System:** Derive color palettes, typography, spacing, and styles from a brand hex color.
- **C2: Set Archetype & Visual Theme:** Customize carousel appearance using brand archetypes (educator, thought_leader, etc.) and themes (editorial, bold, etc.).
- **C3: Recommend Alternate Color Schemes:** Explore and recommend color schemes for slides.
- **C4: Context-Aware Slide Type Selection:** Choose slide types that match the content structure and message intent.
- **C5: Query Slide Schema & Parameters:** Discover required and optional parameters for any of the 47 slide types.
- **C6: Generate Slide Content (Text & Metrics):** Structure text parameters, headings, lists, badges, quotes, and callouts correctly.
- **C7: Generate Data Visualizations:** Render charts, gauges, tables, sparklines, progress rings, and comparison bars with valid data shapes.
- **C8: Structure Story & Educational Flows:** Layout problem-solution, myth-fact, process map, before-after, and case study slides.
- **C9: Integrate Images:** Convert local images to base64 data URIs and embed them into collage, gallery, comparison, or caption layouts.
- **C10: Assemble Full Carousel HTML:** Merge multiple slide JSON outputs into a cohesive HTML document with brand footers, topic headers, and progress bars.
- **C11: Run Layout and Pre-flight Audits:** Run pre-rendering validation (`validate-layout`) to ensure required parameters are present.
- **C12: Run Design and Overflow Audits:** Run post-rendering design audits (`validate-design`) to catch text clipping, contrast errors, and layout squishing.
- **C13: Export Carousel to High-Res PNGs:** Screenshot the carousel HTML into exact resolution slides (Instagram, LinkedIn, TikTok).
- **C14: Manage Chromium Setup:** Install/setup the Chromium subprocess for CI/CD or offline environments.

---

## 2. Tree Architecture

- **slideforge/SKILL.md** (Root Router)
  - **design-system/SKILL.md** (Leaf) — Handles C1, C2, C3. Sets up design parameters.
  - **slide-composition/** (Router) — Routes to content slide builders.
    - **text-layouts/SKILL.md** (Leaf) — Handles C5, C6 (standard text slides).
    - **data-viz/SKILL.md** (Leaf) — Handles C5, C7 (metrics, charts, and visualizations).
    - **story-flows/SKILL.md** (Leaf) — Handles C5, C8 (educational story structures).
    - **image-integration/SKILL.md** (Leaf) — Handles C5, C9 (local image processing and layouts).
  - **rendering-export/SKILL.md** (Leaf) — Handles C10, C13, C14 (carousel generation and image rendering).
  - **validation-fixing/SKILL.md** (Leaf) — Handles C11, C12 (pre-flight checks, layout fixing, and design auditing).

---

## 3. Conventions Block

Every leaf skill in this tree must adhere to the following rules:
1. **Tool-Agnostic Terminology:** The skill should provide patterns that work equally well via CLI (`slideforge-rust ...`) or MCP tools (`configure_design`, `generate_slide`). Focus on the *semantic intent* of each step.
2. **Actionable Checklists:** Avoid vague descriptions like "consider using list slides". Instead, use rigid guidelines: "Use a `list` slide type when you have 2–5 bullet points. Do not exceed 5 items to prevent content overflow."
3. **No Nuance Clauses:** Keep rules absolute. If there is an exception, write it as a concrete conditional: "If the slide is dark, the rendering engine automatically applies glassmorphism styles."
4. **Wow-Factor Aesthetics:** Always advise generating high-contrast, structured slides. Hero slide first, Call-to-action (CTA) slide last. Do not use plain templates; use distinct archetypes and themes.

---

## 4. Build Status Checklist

- [ ] Write `design-system/SKILL.md` (Phase 3)
- [ ] Write `slide-composition/SKILL.md` (Router) (Phase 4)
- [ ] Write `slide-composition/text-layouts/SKILL.md` (Phase 3)
- [ ] Write `slide-composition/data-viz/SKILL.md` (Phase 3)
- [ ] Write `slide-composition/story-flows/SKILL.md` (Phase 3)
- [ ] Write `slide-composition/image-integration/SKILL.md` (Phase 3)
- [ ] Write `rendering-export/SKILL.md` (Phase 3)
- [ ] Write `validation-fixing/SKILL.md` (Phase 3)
- [ ] Write root `SKILL.md` router (Phase 4)
- [ ] Implement and run `registry.py` (Phase 5)
- [ ] Test routing and leaves (Phase 6)
