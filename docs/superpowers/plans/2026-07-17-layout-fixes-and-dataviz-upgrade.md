# SlideForge Layout Fixes & Data Visualization Upgrade Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement visual layout corrections (font scaling, raw-string bullet point fallbacks, single-CTA deck validation rules) and upgrade SlideForge to support multi-dimensional/grouped series charts.

**Architecture:** Modify `grid_cards_slide` and `myth_fact_slide` to dynamically adjust font sizes and padding based on content mass. Upgrade `list_slide` to support raw string fallbacks, and introduce a deck-level CTA validator. Implement grouped flexbox columns in `column_chart_slide` and multi-path SVG rendering in `render_svg_line_chart`.

**Tech Stack:** Rust (serde_json, SVG rendering, HTML/CSS generation)

---

## Part 1: Completed Layout Corrections & Refactors

### Task 1: Slide 9 Grid Card Overflow
* **Files Modified:** `src/components.rs:3054-3065`
* [x] **Step 1:** Calculate the sum of characters across all card descriptions inside `grid_cards_slide`.
* [x] **Step 2:** Scale down paddings, gaps, and font sizes dynamically if total description character count exceeds `240` or `350` characters.
* [x] **Step 3:** Recompile and verify that long description cards fit within the `365px` slide body bounds without cutoffs.

### Task 2: Slide 12 Myth vs Fact Text Scaling
* **Files Modified:** `src/components.rs:4732-4803`
* [x] **Step 1:** Calculate character lengths for `myth` and `fact`.
* [x] **Step 2:** If both are short ($<40$ characters), scale up the font size by **$+5\text{px}$** and increase padding to `var(--space-4)` to eliminate excessive white space and balance the composition.
* [x] **Step 3:** Validate that short texts occupy card containers aesthetically.

### Task 3: Slide 13 List Bullet Points
* **Files Modified:** `src/components.rs:1290-1300`, `src/components.rs:1338-1347`, `src/components.rs:1389-1398`, `generate_gender_studies_carousel.py`
* [x] **Step 1:** Convert raw strings into objects `[{"title": "..."}]` inside `generate_gender_studies_carousel.py`.
* [x] **Step 2:** Implement a string-check fallback inside `list_slide`'s card, grid, and bullet arms so that raw string parameters are printed directly rather than rendering blank list lines.
* [x] **Step 3:** Run `cargo test` to verify that both formats parse cleanly.

### Task 4: Carousel Validator & Single CTA Enforcement
* **Files Modified:** `src/validate.rs:228-235`, `src/validate.rs:1228-1265`, `generate_gender_studies_carousel.py`
* [x] **Step 1:** Add a competing CTA warning in `validate_design` that flags carousels containing multiple slides with buttons (`class="btn"`) or QR codes.
* [x] **Step 2:** Add a warning in `validate_slide_spec` for `cta` slides alerting that web buttons are non-interactive on image-based social media platforms.
* [x] **Step 3:** Replace Slide 19 (`cta`) with `process_map` in the generator script to ensure exactly one CTA (`qr_destination`) exists at the end of the deck.

---

## Part 2: Planned Data Visualization Upgrades

### Task 5: Extend Schemas and Slide Registry
Enable validation support for multi-series objects inside required and optional parameters definitions.

**Files:**
- Modify: `src/slide_registry.rs:430-460`
- Test: `src/slide_registry.rs`

- [ ] **Step 1: Write validation tests for multi-series parameters**
  Add unit tests in `src/slide_registry.rs` checking that `column_chart` successfully accepts both 1D arrays `[{"label": "1970", "value": 58}]` and nested series arrays:
  ```json
  [
    {
      "label": "1970",
      "series": [
        {"name": "Men", "value": 58},
        {"name": "Women", "value": 42}
      ]
    }
  ]
  ```
- [ ] **Step 2: Run tests to verify failure**
  Run `cargo test` and verify that the registry validation rejects the new nested series structure.
- [ ] **Step 3: Modify slide_registry.rs rules**
  Update `get_slide_type_info` or schema validators in `slide_registry.rs` to allow the `series` array parameter.
- [ ] **Step 4: Run tests to verify success**
  Run `cargo test` to ensure validations pass.
- [ ] **Step 5: Commit**
  Commit registry changes.

---

### Task 6: Implement Grouped Column Chart Layout
Update `column_chart_slide` to inspect the data format and render side-by-side grouped bars for multi-series elements.

**Files:**
- Modify: `src/components.rs:4382-4450`
- Test: `tests/components/column_chart.rs`

- [ ] **Step 1: Write test case for grouped column rendering**
  Write a test checking that the returned HTML contains multiple column bar elements (two distinct styled divs representing Men and Women series) inside a single horizontal category block.
- [ ] **Step 2: Run tests to verify failure**
  Run `cargo test` and verify the test fails or panics on the nested format.
- [ ] **Step 3: Implement multi-series parser in column_chart_slide**
  Parse data item objects. If `series` array is present:
  - Find the global maximum value across all series to scale the heights.
  - Draw a flex container for each category label containing multiple color-coded vertical bars with custom heights.
- [ ] **Step 4: Run tests to verify success**
  Run `cargo test` and check that the columns are output correctly.
- [ ] **Step 5: Commit**
  Commit components changes.

---

### Task 7: Implement SVG Multi-Path Line Renderer
Extend `render_svg_line_chart` in `dataviz.rs` to iterate over multiple data paths and overlay them inside the SVG canvas.

**Files:**
- Modify: `src/dataviz.rs:22-177`
- Test: `tests/dataviz/line_chart.rs`

- [ ] **Step 1: Write multi-path SVG validation test**
  Verify that the SVG output contains multiple `<path>` elements with different stroke colors.
- [ ] **Step 2: Run tests to verify failure**
  Run `cargo test` to ensure it fails on the multi-series schema.
- [ ] **Step 3: Modify render_svg_line_chart function**
  Loop through each series, generating separate coordinates, line paths, area fills, and Y-axis scaling factors.
- [ ] **Step 4: Run tests to verify success**
  Run `cargo test` to confirm compilation and path validation.
- [ ] **Step 5: Commit**
  Commit visualization upgrades.
