# Slide-Body Auto-Scaling Audit & Upgrade Plan

> **Status:** Plan — ready for implementation (phased)
> **Goal:** Give every slide type that renders text-driven bodies a density/type auto-scaling implementation (or a hard validator cap where scaling is impossible), so no slide overflows into the header/footer bands regardless of input length.
> **Date:** 2026-08-07

## Background & Calibration Model

- Base composition is fixed **420×525**; `.slide-header` band = 36px, `.slide-footer` band = 40px.
- `overflow_model::SAFE_CONTENT_HEIGHT` = **449px** is the single calibration point shared by the renderer (density scaling) and the validator gates (hard errors) so the two sides cannot drift.
- The renderer's auto-scaling patterns already in use:
  - **Greedy density tiers** — estimate stack height from character mass via `overflow_model::estimate_text_height`/`estimate_wrapped_lines`, then pick the least-compressed tier that fits (`problem_solution_slide`, added 2026-08-07).
  - **space_usage tiers** — ratio of estimated height to SAFE_CONTENT_HEIGHT drives font/padding/gap compression (`pricing_plan_slide`, `checklist_action_plan`).
  - **Type fitting** — `fit_font_size` / `fit_font_size_to_lines` shrink a single dominant text element to fit a column (`hero_slide`, `quote_slide`).
  - **Char-mass scaling** — `total_feat_chars` thresholds compress cards (`split_features_slide`).
- **10px body-text floor** — matches the validator's `tiny_text` warning threshold (sub-10px glyphs are flagged).
- Validator gates today: per-type hard caps exist for `split_features` (3 tiles), `metric_grid` (trend/label 20 chars, trend-must-not-echo-value, mandatory description), `pricing_plan` (4 plans), `process_map` (6 steps), plus generic post-render `text_overflow_tight` (warning) / `text_overflow` (error) / `tiny_text` (warning) estimation.

## Audit Matrix (all 46 active types)

### ✅ Already protected (renderer auto-scales or hard-caps)

| Type | Mechanism |
|---|---|
| `hero` | fit_font_size_to_lines/to_words on headline |
| `quote` | fit_font_size on quote text |
| `split_features` | char-mass tiers + 3-tile validator cap |
| `pricing_plan` | space_usage tiers + 4-plan validator cap |
| `process_map` | wrap-aware density estimate + 6-step validator cap |
| `problem_solution` / `case_study_result` | greedy 3-tier density (2026-08-07) |
| `metric_grid` / `stat_row` | minmax(0,1fr) grid + 20-char caps + description gate |
| `funnel_chart` | bar-height compression by step count (partial — count-based only) |
| `comparison_bars` | label/value font scaling by char length (partial — no stack estimate) |
| `myth_fact` | sparse-text up-scaling (partial — scales UP for short text only, no DOWN protection for long myth/fact) |

### ⚠️ PARTIAL / count-based only (scaling fires on item count, not text mass)

| Type | Gap |
|---|---|
| `timeline` | 5/4/else tiers by **step count** + desc-char trim — long titles/descriptions in a 4-5 step stack can still overflow |
| `faq` | 4/3/else tiers by **question count** — long Q&A pairs in 4-mode overflow |
| `comparison_bars` | fonts scale, but no full-stack height estimate |
| `funnel_chart` | bar height by count; step labels/captions can clip |
| `myth_fact` | no DOWN-scaling for long myth/fact/explanation text |

### ❌ NOT protected (fixed sizes, no estimate — the upgrade targets)

| Type | Fixed sizes | Overflow trigger |
|---|---|---|
| `big_statement` | 52px heading / 72px stat / 220px watermark | Long heading at 52px → massive wall, no fit (highest risk) |
| `text_block` | 28px title, 14px body | Long body paragraphs (deck slide 14 already warned 9px eyebrow + long body) |
| `definition` | 30px term, 15px def, 13px context | Long definition/context |
| `comment_cta` | 38px headline, 16px body, 18px highlight | Long headline/keyword |
| `before_after_story` | 28px title, var(--text-sm) cards | Long before/after/metric text (3-tile so bounded, but tight) |
| `table` | 11px cells, no row cap | Many rows / long cell text (deck slide 8: `text_overflow_tight` 395/417px) |
| `logo_cloud` | 58px rows, fixed grid | Many logos overflow rows |
| `testimonial_avatar` | 28px quote, fixed avatar | Long quotes |
| `gauge` | fixed SVG + hardcoded caption | Long label/description; hardcoded "Overall system health…" caption ignores param text |
| `radar_chart` | fixed 210px SVG + desc | Long description under chart |
| `scatter_plot` | fixed 185px SVG + hardcoded caption | Hardcoded "Scatter distribution…" caption; long title |
| `progress_rings` | fixed 90px rings, take(3) | Long ring labels/desc |
| `qr_destination` | fixed QR + heading/caption | Long caption/heading/CTA |
| `image_headline` | 32px headline | Long headline clips (has scrim, no font-fit) |
| `image_quote` | 22px quote | Long quote clips |
| `image_callout` | 18px title + pins | Long title/callouts |
| `chart` / `column_chart` | data take(5/8), fixed SVG | Long labels/caption |
| `metric_sparkline` | fixed spark + trend chip | Long trend/label |

### Cross-cutting gaps
- Hardcoded captions that ignore param text: `gauge` ("Overall system health…"), `scatter_plot` ("Scatter distribution…"), `table` fallback caption, `radar_chart` desc slot (param-driven, ok).
- Validator per-type caps do not cover the newly-scaled types (no char caps for `big_statement` heading, `definition` term/def, `text_block` body, `comment_cta` headline, `logo_cloud` count, `table` row count).
- The generic multi-item gate (`process_map` 6, `pricing_plan` 4) should be generalized to `timeline`, `faq`, `logo_cloud`, `table`.

---

## Phase 1 — Fix the fixed-layout text slides (P0, highest risk)

### 1.1 `big_statement_slide` — type fit on the hero element
- **File:** `src/components.rs`
- Apply `overflow_model::fit_font_size_to_lines` (headline: 3 lines max in the 420px−padding column; stat mode: fit the giant number to 1 line via `fit_font_size_to_words`).
- Scale the watermark (220px) down proportionally with the heading size.
- Add char caps to validator: heading **max 90 chars** (hard error), body max 200.

### 1.2 `text_block_slide` — greedy density tiers on body stack
- Reuse the `problem_solution` greedy-3-tier pattern: estimate eyebrow+title+paragraphs (incl. drop-cap line) via `estimate_text_height`, compress title/body/gaps on three tiers with a 10px floor.
- Validator: body char cap (e.g. 420) + eyebrow non-empty recommendation.

### 1.3 `definition_slide` — type fit on definition + context
- `fit_font_size` on the 15px definition text in the bordered column (min 11px); scale term 30px → 24px when definition is long.
- Validator: `definition` max 240 chars, `context` max 160 (hard errors).

### 1.4 `comment_cta_slide` — fit headline + scale highlight
- `fit_font_size_to_lines` on the 38px headline (2 lines max); scale body/highlight from the same space_usage computation.
- Validator: headline max 70 chars.

### 1.5 `before_after_story_slide` — adopt the problem_solution tier pattern
- Same family as `problem_solution` (28px title + two var(--text-sm) cards + metric + description). Port the greedy-3-tier density logic.

### 1.6 `table_slide` — row cap + density
- Add a row-count cap in the validator (max 6 rows hard error, matching `process_map`'s 6-step gate pattern).
- Renderer: compress cell padding/font (11px → 10px floor) and header band when rows ≥ 5.

### 1.7 `logo_cloud_slide` — row compression
- Compress row height (58px → 46px) and gaps when logo count ≥ 6; validator cap at 10 logos.

### 1.8 `testimonial_avatar_slide` — fit quote
- `fit_font_size` on the 28px quote in the 332px column (min 16px).

---

## Phase 2 — Count-based → wrap-aware (P1)

### 2.1 `timeline_slide`
- Replace step-count tiers with the **wrap-aware estimate** (port `process_map`'s `estimate_wrapped_lines` per-title/per-desc at planned font sizes, then pick the least-compressed tier). Keep the 5-step cap.
- Validator: per-step title 24 chars / desc 90 chars hard caps.

### 2.2 `faq_slide`
- Estimate Q+A heights with `estimate_wrapped_lines` per pair at the 4-mode sizes; engage 3-mode/2-mode tiers only when the estimate exceeds SAFE_CONTENT_HEIGHT (currently tiers key off count alone).
- Validator: question 80 chars / answer 160 chars hard caps; 4-item cap (already `take(4)`).

### 2.3 `myth_fact_slide`
- Add a DOWN-scaling tier for long text (currently only scales up for `<40` chars): wrap-aware estimate on myth/fact/explanation, compress font + card padding when over the body budget.
- Validator: myth/fact max 180 chars, explanation max 160.

### 2.4 `gauge` / `scatter_plot` / `radar_chart` / `progress_rings` / `funnel_chart` / `chart` / `column_chart` / `metric_sparkline`
- Remove **hardcoded captions** (`gauge`, `scatter_plot`, `table` fallback) → make them param-driven or empty when absent (matches `metric_grid`'s "no hardcoded placeholder" precedent).
- Add description slots that fit within the fixed SVG height: wrap-aware desc cap, `fit_font_size` when desc present.
- Validator: description char caps per type (e.g. gauge 90, radar 120).

---

## Phase 3 — Image-overlay slides (P2)

### 3.1 `image_headline` / `image_quote` / `image_callout`
- `fit_font_size` on headline/quote within the overlay scrim zone (respect `overlay_anchor`), min sizes 22px/16px/14px.
- Validator: headline 60 chars, quote 140 chars, callout title 30 chars.

### 3.2 `qr_destination`
- Wrap-aware fit on caption + heading; validator caps (heading 60, caption 90, cta_text 30).

---

## Phase 4 — Validator parity + regression harness

### 4.1 Generalize the multi-item gate
- Extend the `multi_item_indicators` array to `timeline` (6), `faq` (4), `logo_cloud` (10), `table` (6 rows) so over-cap configs fail before render.

### 4.2 Per-type char caps
- Add hard-error char caps for every Phase-1/2 type (listed per task) using the same `leading_number`/`MAX_*_CHARS` pattern as `metric_grid`.

### 4.3 Regression harness
- Extend `generate_ellipsis_deck.py` (or a new `generate_overflow_test.py`) with deliberately **max-length** configs for each upgraded type; assert `errors=0` after regeneration and pixel-probe the lowest content row ≥ 40px above the footer band for every slide.

### 4.4 Skill-leaf sync
- Update `skills/slideforge/slide-composition/text-layouts/SKILL.md`, `data-viz/SKILL.md`, and `story-flows/SKILL.md` with the new caps so agents never author over-cap configs.

---

## Testing Strategy
- `cargo test` (existing 183 + new validator cap tests per type).
- `cargo build --release` → copy to `dist/` + `~/.cargo/bin`.
- Regenerate the overflow deck with max-length configs; assert 0 errors.
- Pixel-probe every PNG: content must end ≥40px above the footer band (the `problem_solution` slide 13 baseline: 194px margin).
- `skill-guide --check` + `skills/slideforge` registry `--check` after skill-leaf updates.

## Risks / Notes
- Do NOT introduce a global scaling rewrite — every change is scoped to its slide type (AGENTS.md direction).
- Keep the 10px floor; the aggressive tier compresses padding/gaps, never glyphs below 10px.
- The `myth_fact` up-scaling behavior (sparse balancing) must remain intact when adding down-scaling.
