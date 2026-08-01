# Slide Type Audit — Overflow Bugs + Redundancy Analysis

**Date:** 2026-07-30
**Source:** `dist/remaining_types_carousel.html` (regenerated from latest binary)
**Authoritative references:** `src/components.rs`, `src/slide_registry.rs`, `docs/presets/campaign-presets.json`, `skills/slideforge/references/slide-types-manifest.md`

---

## Part A — Four Overflow / Rendering Bugs

### A1. "New Slide Type Checklist" (slide 12, `checklist_action_plan`)

**Status:** Overflows — scaling does NOT trigger.

**Source path:** `checklist_action_plan_slide()` at `src/components.rs:5045`

**Root cause (dead thresholds):**

The dynamic-scaling formula at `src/components.rs:5094-5102` underestimates real content height so badly that even with 6 items it falls into the "normal fit" branch:

```
item_height_estimate = 38.0  (for item_count >= 6)
gap_estimate         = 4.0
estimated_content_height = 30 + 6*38 + 5*4 = 278
SAFE_CONTENT_HEIGHT       = 405
space_usage = 278/405      = 0.687   →  falls into NORMAL branch
```

But in the rendered HTML (`dist/remaining_types_carousel.html:870`), the slide-content uses `padding:72px 44px` (= normal tier), so 6 items at 11-12px font each render at ~70px tall — that's 420px just for items, plus title and gaps. Total content easily exceeds 405px safe height.

**Real heights** (observed in PNG export): 6 items × ~52px = ~312px of items + ~70px of header/chrome → ~382px just for content, leaving only ~25px for chrome (title + page padding) → overflow.

**Why the formula lies:** the `item_height_estimate` of 38.0 was meant to be "11px text + 8px padding + 9px number + 10px gap", but that's the *minimum* of each item, not the rendered height. With `display:flex;align-items:flex-start;` and `line-height:1.45` on the label, real items render at 50-55px tall when the label wraps or has more than one line. The "Free vs Pro" labels in slide 12 are 60-80 char task descriptions — they DO wrap.

**Fix proposal:**

Raise `item_height_estimate` to match real rendered heights, OR change the threshold so any item_count >= 6 unconditionally enters the aggressive branch:

```rust
// Option A (calibration): match real height
let item_height_estimate = if item_count >= 6 { 55.0 }     // observed: 50-55px
                           else if item_count >= 4 { 60.0 }
                           else { 65.0 };

// Option B (threshold): force aggressive scaling when item_count >= 6
if item_count >= 6 { space_usage = 0.9; }  // synthetic bump into aggressive tier
```

**Plus:** also delete the dead `total_content_len` blocks at lines 5066-5074 and 5077-5085 (computed but never used).

---

### A2. "Carousel Generation Pipeline" (slide 9, `process_map`)

**Status:** Currently renders with `padding:16px` (aggressive scaling DID trigger — this is the GOOD case). User may be referring to the prior bad version, but I want to confirm before assuming.

**Source path:** `process_map_slide()` at `src/components.rs:5566`

**Confirmation needed:** Re-inspect the PNG `dist/remaining_types_exports/slide_9.png`. If it actually fits, this slide can move to "no fix needed" status. If it still overflows, the same calibration fix from A1 applies.

---

### A3. "SlideForge vs Manual" (slide 14, `comparison` cards variant)

**Status:** Renders, but with `padding:80px 48px` — scaling branch NOT triggered (falls into "normal fit"). Does it actually overflow? Need to inspect.

**Source path:** `comparison_slide()` at `src/components.rs:1680`, cards branch at `src/components.rs:1790-1824`

**Formula check (4 rows, cards variant):**

```
row_height_estimate = 55.0   (for row_count >= 4)
gap_estimate        = 12.0
estimated_content_height = 37 + 4*55 + 3*12 = 293
SAFE_CONTENT_HEIGHT       = 405
space_usage = 293/405      = 0.724   →  normal branch (not aggressive)
```

Same calibration problem as A1. Real rendered card height with `padding:12px 16px` and 14px title + 17px values is ~75-85px, not 55px. With 4 rows that's ~340px just for cards + ~60px title + ~160px top/bottom padding → overflow.

**Fix proposal:** Same calibration fix as A1.

---

### A4. "Free vs Pro" (slide 13, `comparison` table variant) — REDUNDANCY

**Status:** User flagged this as "not appearing correctly" — it's a comparison-as-grid-table, not a real table.

**Root cause:** `comparison_slide` has 4 variants (`cards`, `vs-split`, `feature-matrix`, `table` default). The `table` variant at `src/components.rs:1912-1954` is a CSS-grid that LOOKS like a comparison chart, with bold column headers and highlight columns. This visually duplicates the dedicated `table_slide` (`src/components.rs:4226`) which uses a real `<table>` element with smaller fonts and stripe rows.

**Two clean options:**

| Option | What changes | Trade-off |
|---|---|---|
| **A. Remove `table` variant from `comparison`** | `comparison` only renders `cards`/`vs-split`/`feature-matrix`. Anyone wanting the flat-grid comparison look must use `table` slide type instead. | Loses the "highlight column" + bigger fonts capability on a small comparison. Forces conceptual clarity: comparison = A vs B framing, table = data matrix. |
| **B. Rename variant to `matrix`** | Keep the visual but rename to `feature-matrix`-like intent (the user reading the slide sees "Pro" highlighted, the renderer draws a matrix). | Naming confusion: comparison has `feature-matrix` already. |
| **C. Keep variant, just apply scaling** | Same fix as A3 (apply scaling). Visual unchanged. | User's "redundancy" complaint stays unaddressed. |

**Recommendation:** Option A. The `table` slide type is the proper way to render a 2-column comparison matrix when you want row labels + values; the `comparison` slide type should be reserved for genuine A-vs-B framing (cards, vs-split).

---

## Part B — Full Slide Type Redundancy Audit (45 catalog entries)

### Category 1: STRICT SUBSETS (a slide type does only what another already does — should be removed)

| Slide Type | Status | Reason |
|---|---|---|
| ~~`metric_card`~~ | **ALREADY REMOVED** (memory #1632) | Strict subset of `metric_grid`, `comparison_bars`, `gauge`, `progress_rings`. Render fn kept as alias for `metric_sparkline`. |
| ~~`headline_subheadline`~~ | **ALREADY REMOVED** (memory #1659) | Strict subset of `hero`. |
| **`metric_sparkline`** | **REDUNDANT** | Renders one big number + label + trend pill + tiny inline SVG sparkline. This is `metric_card_slide` (still in source as alias). The dedicated `metric_card` was removed because it's a subset — `metric_sparkline` shares the same subset problem PLUS the inline SVG is illegible at 280×42px (anti-design pattern per anti-slop). Either remove `metric_sparkline` from the registry and the alias from `metric_card_slide`, OR keep `metric_sparkline` as a *variant* of `metric_grid` (`metric_grid` with `variant: "sparkline"`). |

### Category 2: OVERLAPPING VISUALS (different slide types produce similar output — consolidate to a variant)

| Slide Type A | Slide Type B | Overlap | Recommendation |
|---|---|---|---|
| `comparison` (`table` variant) | `table` | Both render tabular comparison data. | **Merge:** drop `comparison` `table` variant; redirect callers to `table` slide type. (See A4.) |
| `comparison` (`feature-matrix` variant) | `table` | Both render feature × entity matrix. | **Keep both** — `feature-matrix` is highlight-column-focused with checkmarks, `table` is neutral data matrix. Document the distinction in SKILL.md. |
| `comparison` (`cards` variant) | `before_after_story` | Both can show A vs B as paired cards. | **Keep both** — `comparison` cards is multi-row attribute comparison; `before_after_story` is narrative transformation with before/after label structure. |
| `comparison_bars` | `column_chart` (2-series) | Both render two-value side-by-side bars. | **Keep both** — `comparison_bars` is entity-vs-entity on a single metric, `column_chart` is category-axis multi-bar. Different visual intent. |
| `stat_row` | `metric_grid` | Both render grid of N key stats with values+labels. | **Investigate further.** `stat_row` accepts arbitrary `stats: [{value, label, sub?}]`, `metric_grid` accepts `metrics: [{value, label, trend?}]`. If `stat_row` adds nothing `metric_grid` lacks, merge. Quick check needed. |
| `list` (numbered variant) | `checklist_action_plan` | Both render numbered items. | **Keep both** — `list` is generic content (can be unordered bullets), `checklist_action_plan` is specifically an action plan with check-circles and a different framing. Different intent. |
| `grid_cards` | `feature` (gallery variant) | Both render N cards in a grid. | **Investigate.** `grid_cards` has rich per-card `{title, body, icon?}`; `feature` slide renders one card per slide. Likely fine if `grid_cards` is multi-card and `feature` is single-card, but the naming overlap is confusing — verify the dispatch. |
| `image_gallery` | `image_collage` | Both render multiple images in a layout. | **Keep both** — `gallery` is grid/flex layout (clean, organized), `collage` is rotated/layered (decorative, editorial). Different intent. |
| `comparison_bars` | `metric_grid` (2×1) | Both can show two metrics side by side. | **Keep both** — `comparison_bars` is entity-vs-entity on a *single* metric, `metric_grid` is N independent metrics. |

### Category 3: NEAR-DUPLICATE FAMILIES (different slide types but same data shape and visual)

| Slide Type A | Slide Type B | Data shape | Visual | Recommendation |
|---|---|---|---|---|
| `gauge` | `progress_rings` | `{value, label}` vs `{items: [{label, value}]}` | Radial semicircle vs concentric rings | **Keep both** — fundamentally different visual metaphor. |
| `chart` (bar/line/pie) | `column_chart` | `chart_type: "bar"` overlaps with `column_chart` | Both bar charts | **Investigate.** `chart` covers bar/line/pie but `column_chart` is just vertical bars. Either `chart` covers everything (drop `column_chart` since chart with chart_type=bar is identical) OR `chart` should drop bar and only do line/pie. Currently they're both in the registry. **Strong recommendation: drop `column_chart`, route callers to `chart` with `chart_type: "bar"`.** |

### Category 4: SOFT-OVERLAP (functionally similar but visually distinct — keep, but document the distinction)

| Slide Type | Near-neighbor | Distinction |
|---|---|---|
| `hero` | `section_divider`, `cta` | `hero` = visual opener; `section_divider` = chapter break; `cta` = closing conversion. All three render a big title + sub, but the role in deck arc differs. |
| `quote` | `testimonial_avatar` | `quote` = abstract pull-quote; `testimonial_avatar` = attributed review with avatar. Different credibility framing. |
| `case_study_result` | `before_after_story` | `case_study` = client/challenge/solution/results (multi-field); `before_after` = two-state transformation (compact). |
| `timeline` | `process_map` | Both render ordered steps. `timeline` = horizontal/vertical dated events; `process_map` = functional operating flow. Different metaphor. |
| `myth_fact` | `problem_solution` | `myth_fact` = debunking pattern (debunk variant stacked); `problem_solution` = problem → solution → proof points. Different rhetoric. |
| `split_features` | `grid_cards` | `split_features` = 2-column feature list with image; `grid_cards` = N-tile grid with rich cards. Different density. |
| `callout` | `feature` | Both can render single icon+text card. `callout` is inline note/warning; `feature` is benefit highlight. |
| `logo_cloud` | `testimonial_avatar` | Both are trust/social proof. `logo_cloud` = brand marks; `testimonial_avatar` = personal quote. |
| `image_callout` | `image_caption` | Both = image + text overlay. `callout` has multiple hot-spot labels; `caption` has one caption. |
| `image_stat` | `metric_grid` | Both = big stat. `image_stat` has bg image; `metric_grid` doesn't. |
| `faq` | `list` | Both can be Q&A. `faq` is collapsible accordion pattern; `list` is flat. |
| `image_comparison` | `before_after_story` | Both = before/after. `image_comparison` = two photos side by side; `before_after_story` = text narrative. |
| `qr_destination` | `cta` | Both = conversion. `qr_destination` is QR-specific with `destination_url`; `cta` is button-text. |
| `pricing_plan` | `comparison_bars` | Both involve pricing/comparison. `pricing_plan` is structured plan cards with features; `comparison_bars` is one metric compared. |
| `pricing_plan` | `comparison` (cards variant) | Both show plan/feature cards. `pricing_plan` has tiered structure with "popular" badge; `comparison` is flatter. **Investigate further** — there may be more redundancy here. |

---

## Summary: Recommended Removals / Consolidations

**REMOVE (slide types to delete from registry + dispatch + presets + SKILL.md + manifest):**

1. `comparison` `table` variant — redirect to `table` slide type. (Or: rename variant to make intent clear, but `table` is the cleaner choice.)
2. `column_chart` — redirect to `chart` with `chart_type: "bar"`. (If `chart` doesn't already cover this, extend it.)
3. `metric_sparkline` — remove from registry; either drop the slide type entirely or fold into `metric_grid` as a `variant: "sparkline"` option. (Per AGENTS.md #1651, surface before editing.)

**INVESTIGATE (decide after looking):**

4. `stat_row` vs `metric_grid` — likely merge; both are N-stat grids.
5. `grid_cards` vs `feature` (gallery variant) — clarify if `feature` is single-card only, no overlap.
6. `pricing_plan` vs `comparison` (cards) — confirm pricing_plan adds tier-specific structure (popular badge, prices, features list) vs comparison's flat A-vs-B.

**NO ACTION (true redundancy already removed, soft overlaps are intentional):**

- `metric_card`, `headline_subheadline` — already removed in earlier refactors.

---

## Recommended Action Plan (Sequenced)

### Phase 1 — Fix the four overflow bugs (immediate, contained)

1. **A1** — Fix `checklist_action_plan_slide` threshold/calibration in `src/components.rs:5094-5102`. Also delete dead code at lines 5066-5074 and 5077-5085.
2. **A3** — Fix `comparison_slide` cards variant threshold/calibration in `src/components.rs:1741-1748`.
3. **A2** — Re-verify PNG; if overflow persists, apply same fix to `process_map_slide`.
4. **A4** — Decide on `comparison` `table` variant: remove (Option A) or keep with scaling (Option C).

After each fix:
- `cargo test` → `cargo build --release` → copy binary to `dist/slideforge-x86_64-linux-gnu`
- `python3 generate_remaining_types_test.py`
- Inspect `dist/remaining_types_exports/slide_9.png, slide_12.png, slide_13.png, slide_14.png` for visible fit

### Phase 2 — Apply redundancy decisions (after user approval)

For each REDUNDANT removal: remove from registry, dispatch, presets, MCP server, SKILL.md, manifest per the discipline in AGENTS.md #1651/#1658.

For each INVESTIGATE item: read source for both slide types, produce a one-paragraph recommendation, surface for approval.

### Phase 3 — Regen + verify

`python3 generate_preset_slides.py`, `python3 generate_remaining_types_test.py`, `dist/slideforge-x86_64-linux-gnu validate-design` across all carousels.

---

## Decision Required

User, please choose:

1. **For A4** (`comparison` `table` variant):
   - (a) Remove the variant entirely (redirect to `table` slide type)
   - (b) Keep variant, just fix scaling
   - (c) Other

2. **For Phase 2 removals** — which of the 3 REDUNDANT items do you want me to remove?
   - [ ] `comparison` `table` variant (if not chosen in step 1)
   - [ ] `column_chart`
   - [ ] `metric_sparkline`

3. **For Phase 2 investigations** — which of the 3 INVESTIGATE items do you want me to investigate before deciding?
   - [ ] `stat_row` vs `metric_grid`
   - [ ] `grid_cards` vs `feature` (gallery variant)
   - [ ] `pricing_plan` vs `comparison` (cards)

I will not modify any source until you've chosen.
