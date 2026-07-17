# SlideForge Agent Notes

## Project Direction

SlideForge is a 4:5-first slide composition system. The existing layouts are mostly functional and should be preserved. Do not redesign the renderer around full-canvas, per-aspect-ratio layout recomposition unless the user explicitly asks for that larger redesign.

The working model is:

- Compose slides inside the established 420x525 base composition.
- Export to target aspect ratios by fitting that composition into the target canvas.
- Let backgrounds bleed to the target canvas when needed.
- Keep content, components, typography, spacing, and card composition governed by the existing base-layout system.

## Bug-Fix Strategy

When fixing visual bugs, prefer targeted repairs inside existing slide types, layout helpers, validators, and data adapters. The expected scope is to fix the remaining broken 25 percent of cases without destabilizing the 75 percent that already works.

Do not solve isolated visual issues by introducing a new global scaling model, rewriting all hardcoded dimensions, or making every component responsive to every final export aspect ratio. That path creates a much larger second-order debugging problem and is not the chosen trajectory.

Acceptable fixes:

- Correct a specific slide type that overflows or constricts content.
- Improve an existing layout primitive while preserving its contract.
- Add validator checks for real failure classes such as clipped text, bad margins, overlap, constricted columns, or invalid dimensions.
- Fix data-shape adapters where test/full-scope inputs do not match renderer expectations.
- Add variants to an existing slide type when variety is needed and the old variants remain compatible.
- Clip decorative effects at the final slide bounds while keeping background bleed behavior intact.

Avoid unless explicitly requested:

- Replacing the 4:5 base composition model.
- Scaling all content to the final output canvas as a new layout architecture.
- Redesigning slide styling, typography, spacing, and composition from the ground up.
- Broad aesthetic rewrites while investigating named bugs.
- Regenerating large output sets as a substitute for fixing root causes in source.

## Aspect Ratio Decision

Aspect-ratio support should preserve the original content composition. For 1:1, 9:16, 3:4, and similar exports, the final canvas may differ from the 4:5 composition, but slide content should not be recomposed as if each target ratio is a separate design system.

Backgrounds may bleed outward to fill the target aspect ratio. Edge effects such as glows and shadows must not appear as stale 4:5 artifacts inside the final canvas. Prefer clipping or relocating decorative effects over changing the whole content layout model.

## Validator Expectations

Validators are part of the product contract. When a visual bug is found, add validator coverage where practical so the same class of issue is caught before manual review.

Validator work should be specific and actionable:

- Flag descender/text clipping risks when tight line-height and hidden overflow can cut off characters.
- Flag card or component overflow relative to the slide body.
- Flag constricted text columns that collapse into unreadable wrapping.
- Flag chart/data visualizations that hide values or produce misleadingly identical marks.
- Keep warnings and errors tied to concrete fixes, not generic aesthetic preferences.

## Working Protocol

Before changing layout behavior:

1. Identify the slide type, layout helper, style primitive, or data adapter causing the bug.
2. Compare against nearby working slide types.
3. Add a focused failing test or validator case for the bug class.
4. Patch the smallest shared source of the issue.
5. Run unit tests and the relevant full-scope harness.
6. Review generated HTML only as evidence, not as the primary fix.

If a fix starts requiring a full-system scaling rewrite, stop and reassess. That is a sign the approach has drifted from the intended project direction.

## R&D Protocol & Design Guidelines

To maintain visual excellence and code robustness, all agents must adhere to the following core engineering rules:

### 1. Visual Density & Adaptive Spacing
- **Dynamic Layout Scaling:** Components that render grids or multiple items (e.g., `grid_cards`) must calculate the total text character mass of all items at runtime. Scale down paddings, margins, gaps, and font sizes proportionally (dense vs standard modes) to prevent body viewports from overflowing the fixed `420x525` composition constraints.
- **Sparse Text Balancing:** Simple text layouts (e.g., `myth_fact`) must dynamically scale text up (e.g., $+5\text{px}$) and expand card paddings if the content is short ($<40$ characters) to balance free-space with fill-space.

### 2. Collection Parsers & Input Robustness
- **Fallback for Raw Strings:** Multi-item collection components (e.g., `list`) must support both structured JSON objects `[{"title": "..."}]` and raw strings `["..."]`. Always write fallbacks to detect raw string array elements and format them directly to prevent rendering blank lines.

### 3. Deck-Level Marketing Constraints
- **Single CTA Rule:** A slide deck/carousel must contain exactly one Call-To-Action (CTA) slide, positioned as the final closing slide. 
- **Validator Checks:** Keep `validate_design` updated to audit the compiled deck HTML for:
  - *Competing CTAs:* Flag warnings if multiple slides contain buttons (`class="btn"`) or QR codes.
  - *Non-interactive buttons:* Flag warnings if a slide contains a styled web button without a QR code on image-export platforms (Instagram/TikTok), suggesting `qr_destination` or "Link in Bio" framing instead.

### 4. Data Visualization Efficacy
- **Multi-Series Coexistence:** High-competence visual charts (grouped column charts, line graphs) must support both 1D data arrays and nested series datasets with complete backwards compatibility.
- **Pure-Render Implementation:** Build visualizations natively without external charting libraries:
  - *Grouped Columns:* Render side-by-side flex divs inside X-axis category columns.
  - *SVG Lines:* Overlay multiple `<path>` elements mapped to theme colors, paired with an SVG-rendered legend block.

### 5. Testing & Verification Commands
Always run the following commands to verify edits before committing:
- **Run Unit Tests:** `cargo test`
- **Recompile Release Binary:** `cargo build --release`
- **Regenerate Test Carousels:** Run the local automation script `python3 generate_gender_studies_carousel.py` and inspect exported PNG sizes under `dist/gender_studies_exports/`.

