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

## Part 2: Completed Data Visualization Upgrades

### Task 5: Extend Schemas and Slide Registry
Enable validation support for multi-series objects inside required and optional parameters definitions.

**Files:** `src/slide_registry.rs`

- [x] **Step 1: Write validation tests for multi-series parameters**
  Added unit tests verifying `column_chart` accepts both flat arrays and nested `series` arrays.
- [x] **Step 2: Run tests to verify failure**
  Verified registry correctly rejected malformed nested structures before fix.
- [x] **Step 3: Modify slide_registry.rs rules**
  Updated `column_chart` description, `best_for` tags, and added `example` with nested series format.
- [x] **Step 4: Run tests to verify success**
  All 75 tests pass including 2 new registry schema tests.
- [x] **Step 5: Commit**
  Committed with Tasks 6-7.

---

### Task 6: Implement Grouped Column Chart Layout
Update `column_chart_slide` to inspect the data format and render side-by-side grouped bars for multi-series elements.

**Files:** `src/components.rs`

- [x] **Step 1: Write test case for grouped column rendering**
  Added `test_column_chart_grouped_series` and `test_column_chart_single_series_backward_compatible`.
- [x] **Step 2: Run tests to verify failure**
  Verified grouped series format was not handled before fix.
- [x] **Step 3: Implement multi-series parser in column_chart_slide**
  Detects `series` array, computes global max, renders grouped side-by-side bars with 6-color palette and HTML legend.
- [x] **Step 4: Run tests to verify success**
  All 75 tests pass including 2 new grouped column tests.
- [x] **Step 5: Commit**
  Committed with Tasks 5 & 7.

---

### Task 7: Implement SVG Multi-Path Line Renderer
Extend `render_svg_line_chart` in `dataviz.rs` to iterate over multiple data paths and overlay them inside the SVG canvas.

**Files:** `src/dataviz.rs`

- [x] **Step 1: Write multi-path SVG validation test**
  Verified SVG output contains multiple `<path>` elements with different stroke colors.
- [x] **Step 2: Run tests to verify failure**
  Verified single-path rendering was the only mode before fix.
- [x] **Step 3: Modify render_svg_line_chart function**
  Multi-series detection, separate `<path>` per series, global min/max normalization, SVG-native legend (`<rect>`+`<text>`), gradient defs properly split from area paths.
- [x] **Step 4: Run tests to verify success**
  All 75 tests pass.
- [x] **Step 5: Commit**
  Committed with Tasks 5 & 6.
