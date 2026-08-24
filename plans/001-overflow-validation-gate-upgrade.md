# 001 — Overflow validation gate upgrade + automatic component scaling

**Status:** ✅ EXECUTED (A–F complete, Aug 4 2026) · **Commit stamped:** `7d58014` · **Priority:** P0
**Execution estimate:** L (several days across phases; A–C are a day)

---

## 1. Why this matters (the user's request)

The typology sample space (`dist/typology_viewer.html`, 210 slides) has font-size
and text-overflow bugs. The overflow validator must be able to catch them, at
**two gates**:

1. **Compile-time** — every slide rendered by `generate-slide` (CLI + MCP) must be
   design-validated before it is emitted, so all slides are always aesthetic.
2. **Runtime post-generation** — `validate-design <carousel.html>` must be able to
   flag overflowing slides, and any deck passing the gate must have zero errors.

Then, with a *shared* overflow model between the renderer and the validator,
**automatic component scaling** can be implemented so slides fit at the source.

---

## 2. Audit findings (evidence)

### Finding 1 — CRITICAL: the runtime gate is a silent no-op on this carousel

`src/validate.rs:1896` `validate_design` splits the HTML into slides with:

```rust
let slide_start_re = Regex::new(r#"<div\s+class="([^"]*)" "#).unwrap();
```

This requires `class=` to be the **first** attribute after `<div`. Since the
per-slide `id="slide-{idx}"` attribute was added to the renderer
(`src/slides.rs` render loop emits `<div id="slide-0" class="slide slide--light">`),
the regex matches **zero** slide divs. Consequence:

- `slides.len() == 0` → `slide_count = slides.len().max(1) == 1`
- the per-slide loop body never executes → **0 issues, `passed: true`**

Verified empirically:
`validate-design dist/typology_carousel.html` → `{"passed": true, "slide_count": 1, "error_count": 0}`.
A Python prototype confirms: old regex → 0 slides; fixed regex → 210 slides.

**Impact:** every carousel rendered since the `id` attribute landed is unvalidated.
This is the primary bug to fix (Phase A).

### Finding 2 — HIGH: no general text-height-vs-available-height estimator

Even with the regex fixed, `validate_design` has no check that estimates whether a
slide's text stack fits its available height. Existing related checks are narrow:

- `grid_container_text_len` char budget (2500) — only for `display:grid` containers (`src/validate.rs` ~2470)
- descender clipping with `overflow:hidden` + tight line-height (~2520, ~2610)
- one-word-per-line / text-constriction / component constriction (width-only)
- inline component boxes vs 420×525 via `rect_overflows_slide_body`

None of these catch a 113px 3-line blockquote overflowing by 400px. (Phase B adds it.)

### Finding 3 — HIGH: the 93 actual overflow bugs (measured)

Headless-Chromium measurement of the current carousel (tool: `measure_overflow.py`
in repo root — reusable for Phase F verification): **93 of 210 slides overflow**
(content taller than the 420×525 composition, clipped by the composition).

Breakdown by slide type (from `dist/typology_test/compiled_slides.json`):

| slide type | overflowing | root cause |
|---|---|---|
| `quote` | 30 | `quote_slide` (`src/components.rs:1019–1035`) sizes text by **character count only**: `<60 chars → display tier (113px)`. The harness quote (40 chars) renders at 113px in a ~320px column → 3 lines ≈ 401px overflow. |
| `timeline` | 30 | dynamic scaling at `src/components.rs:3300–3380` uses hardcoded per-item heights (56–70px) and font tiers calibrated for the **old 16px-base type scale**; the new typology tiers (body 41px, caption 29px) make real item heights ~2× the estimate. |
| `hero` | 18 | `hero_slide` (`src/components.rs:774`) renders at full display/headline tier sizes with no length-aware clamp. |
| `funnel_chart` | 15 | step bars + title at tier sizes exceed available height. |

Measured worst cases: `slide 169` (nature quote) content overflow **401px**;
`slide 0` (hero) estimated 845px vs ~305px available (prototype estimator).

### Finding 4 — MEDIUM: no compile-time rendered-HTML gate

`cli_generate_slide` (`src/main.rs:1051–1067`) and MCP `generate_slide`
(`src/mcp_server.rs:963–1048`) run `validate::validate_slide_spec` (param
presence only) before `dispatch_slide`, but never validate the **rendered HTML**.
The building block already exists: `validate_layout` (`src/validate.rs:269`)
routes `rendered_html` → `validate_design` and merges errors/warnings into a
`ValidationResult` — it is just not invoked in the generation flow (only exposed
as an MCP tool). (Phase C wires it in.)

### Finding 5 — LOW: `validate_design` on a bare slide fragment finds 0 slides

Same regex issue in a second form: compile-time validation will pass the slide's
inner HTML fragment (no `.slide` class div) to `validate_design`, which would again
find 0 slides and skip per-slide checks. `validate_design` needs a fallback mode:
if no `.slide` divs are found, treat the whole input as a single slide (this also
matches today's `slide_count.max(1)` semantics — it just must actually *validate*
that one slide).

---

## 3. Repo conventions to follow

- Rust 2021; regex-based HTML inspection everywhere in `src/validate.rs` (no HTML
  parser dependency — do not add one). Regexes are compiled inside the function.
- `DesignIssue { slide: usize, r#type, severity, detail, message, suggestion }`;
  `ValidationReport { passed, issues, slide_count, error_count, warning_count, info_count }`.
  `passed = error_count == 0`. Severities: `"error" | "warning" | "info"`.
- Tests live in `#[cfg(test)] mod tests` at the bottom of each file, using
  `assert!(report ...)` / `assert_eq!` patterns (see `src/validate.rs:749+`).
- Dynamic scaling style precedent: `split_features` dense variant
  (`src/components.rs:1338–1370`) and the timeline/process_map estimators
  (`src/components.rs:3300`, `3462`) — text-mass → scale-factor/padding tiers.
- Per `AGENTS.md`: validator and renderer must **share** the overflow model; keep
  diffs minimal and targeted; add validator coverage for each bug class.

---

## 4. Phased implementation

> **Execution log (this session):** all phases shipped. Final evidence:
> - `cargo test` **112/112** (10 new tests incl. `slide_split_failed` guard)
> - `measure_overflow.py`: **93 → 0 / 210** overflowing slides (browser-measured)
> - `validate-design dist/typology_carousel.html`: `passed: true, slide_count: 210, errors: 0`
> - Compile gate negative test: long quote **rejected** (`exit=1`, `text_overflow`); short quote passes
> - Harness `generate_typology_test.py` runs `validate-design` post-render and fails the build on errors
>
> **Two calibration fixes found during execution** (beyond the plan):
> 1. `estimate_slide_text_height` fell back to `DEFAULT_COLUMN_WIDTH` (332px) for
>    blockquotes, but quotes live in a ~268px glass column (`--space-4` = 32px
>    padding each side) — the wider fallback let borderline wall-of-text quotes
>    through the gate. Fixed: blockquote fallback width = 272px.
> 2. The text-sum alone missed ~180px of fixed quote chrome (quote mark, divider,
>    attribution, glass padding). Fixed: `blockquote_chrome = 180.0` added once.
>    After both, the ~200-char adversarial quote is correctly rejected while all
>    210 legit slides still pass (no false positives).

### Phase A — Restore the runtime gate (CRITICAL, do first)

**A1. Fix the slide-splitting regex** in `src/validate.rs` `validate_design` (~line 1900):

```rust
// BEFORE (broken): requires class= to be the first attribute.
let slide_start_re = Regex::new(r#"<div\s+class="([^"]*)" "#).unwrap();
// AFTER (attribute-order agnostic; still requires the exact "slide" token).
let slide_start_re = Regex::new(r#"<div\b[^>]*?\bclass="([^"]*)" "#).unwrap();
```

The existing filter `class_attr.split_whitespace().any(|class| class == "slide")`
already excludes `slide-content`, `slide-composition`, `slide__overlay`, etc.
The trailing space in the pattern is now optional-but-harmless (`" ` keeps it).

**A2. Single-fragment fallback.** After the split loop:

```rust
let slide_count = slides.len().max(1);
```
becomes (keep `slide_count` the same; only change the loop source):

```rust
// No `.slide` div found → treat the whole input as one slide fragment
// (compile-time validation passes bare slide HTML to validate_design).
let slides: Vec<&str> = if slides.is_empty() { vec![html] } else { slides };
```

Place this **after** the `slide_count` computation so the count stays
`slides.len().max(1)` while the loop validates the fragment.

**A3. Regression tests** in `src/validate.rs` `mod tests`:
- `test_validate_design_splits_id_first_slide_divs` — build a 3-slide HTML string
  with `<div id="slide-0" class="slide slide--light">…` and assert
  `report.slide_count == 3` and that a known issue on slide 2 reports `slide: 2`.
- `test_validate_design_bare_fragment_validates_as_one_slide` — pass a slide
  fragment (no `.slide` div) containing a `tiny_text` offender and assert it is
  flagged (proves the fallback actually validates).

**Verification (A):** `cargo test` green; `validate-design dist/typology_carousel.html`
now reports `slide_count: 210` (issues expected to be non-zero once Phase B lands;
before Phase B, existing per-slide checks like competing_ctas/tiny_text may fire).

### Phase B — General text-overflow estimator (shared model)

**B1.** Add helper functions to `src/validate.rs` (near `numeric_style_value`,
~line 1569) — keep the regex approach:

- `resolve_font_size(style: &str, css_vars: &HashMap<&str, String>) -> f32`
  — parse `font-size: Npx` inline; if absent or `var(--text-*-size)`, resolve from
  the carousel `:root` token block **and** the per-slide `css_vars` block (both are
  already in the HTML; parse `--text-{level}-size: Npx` pairs from the `<style>` blocks).
- `estimate_wrapped_lines(text: &str, font_size: f32, width: f32) -> usize`
  — `chars_per_line = max(1, floor(width / (font_size * 0.55)))`; lines =
  explicit `\n` count + ceil(visible_len / chars_per_line).
- `estimate_text_height(html: &str, ...) -> f32` — walk text elements
  (`p|h1-6|blockquote|span|li` plus `div` that directly contains text with no
  nested text element), sum `lines * line_height_px + margin` (8px default).

**B2.** New per-slide check in the main loop (after the existing checks, ~line 2590):

```rust
// SAFE_CONTENT_HEIGHT = 525 - 60 (header) - 60 (footer) = 405
// subtract .slide-content top+bottom padding (parse `padding:` like the existing
// multi-item check at ~line 1995, default 60px each).
let available = SAFE_CONTENT_HEIGHT - padding_top - padding_bottom;
let est = estimate_text_height(slide_html, ...);
if est > available {
    issues.push(DesignIssue { r#type: "text_overflow", severity: "error", ... });
} else if est > available * 0.92 {
    issues.push(DesignIssue { r#type: "text_overflow_tight", severity: "warning", ... });
}
```

Calibration rule (per AGENTS.md "shared model"): the threshold math must match the
renderer's component estimators. Start with the prototype constants
(`0.55` avg-char-width factor, 8px block margins) and adjust against the
measurement harness (Phase F) until the validator's flagged set ≈ the browser's
measured overflow set (± a few tight-warning cases).

**B3. Tests** (fixtures inline, no external files):
- `test_validate_design_flags_113px_quote_overflow` — the exact harness quote
  markup (`<blockquote style="font-family:...;font-size:113px;...">Design is the silent
  language of trust.</blockquote>` in a 320px column) → `text_overflow` error.
- `test_validate_design_passes_short_headline` — a short headline that fits → no error.
- `test_validate_design_resolves_css_var_font_sizes` — element using
  `font-size:var(--text-display-size)` inside a slide whose `css_vars` block sets
  `--text-display-size: 62px` → estimator uses 62px.

**Verification (B):** `cargo test` green. On the regenerated carousel,
`validate-design` must now flag most of the 93 measured slides (compare counts with
`measure_overflow.py` output; expect 85–95 flagged including tight warnings).

### Phase C — Compile-time gate (CLI + MCP)

**C1.** In `cli_generate_slide` (`src/main.rs`, after `dispatch_slide` at ~line 1067,
before enrichment): run

```rust
let validation = validate::validate_layout(&slide_type, &params_json, Some(&result["html"].as_str().unwrap_or("")), aspect_ratio.as_deref());
```

Merge `validation.errors` into the response JSON under `validation.errors` (the
existing block already emits `validation.warnings`), and **fail the generation**
(`std::process::exit(1)` with the error list printed) when
`!validation.errors.is_empty()`, mirroring the existing pre-flight spec gate just above it.

**C2.** Same wiring in MCP `generate_slide` (`src/mcp_server.rs:1048+`): after
`dispatch_slide`, call `validate_layout(...)`, and on errors return
`ErrorData::invalid_request` with the humanized messages (pattern already used for
the myth/fact overflow block at `src/mcp_server.rs:1025-1050`).

**C3. Tests:** `src/main.rs` / `src/mcp_server.rs` — generate a slide whose params
force overflow (long quote) and assert generation fails / MCP returns an error;
generate a clean slide and assert it passes.

**Verification (C):** `cargo test`; `generate-slide quote --params '{"quote":"<90 chars...>"}'`
(through the typology path used by the harness) fails with a `text_overflow` error;
a short quote succeeds.

### Phase D — Runtime post-generation gate + harness

**D1.** `validate-design` CLI (`src/main.rs:641`) and MCP tool (`src/mcp_server.rs:1462`)
need no code change beyond A+B — they already call `validate_design`. Confirm
`validate-design dist/typology_carousel.html` reports `slide_count: 210` and lists
per-slide `text_overflow` errors.

**D2.** Make the harness fail on validation errors: in `generate_typology_test.py`,
after `render_carousel(...)`, run
`validate-design <carousel>` and `sys.exit(1)` with a summary when
`error_count > 0` (print the first ~20 issues). This is the "any slide passing
through the gate post-generation must pass" guarantee.

**D3. Tests:** none needed (harness is Python); verify by running the harness and
observing it exits non-zero while slides overflow (expected until Phase E lands).

### Phase E — Automatic component scaling (fix the 93 slides at the source)

**E1. Shared estimator helper.** Extract the height estimator used by the validator
(Phase B) into a shared, dependency-free module (e.g. `src/overflow_model.rs` with
`pub fn estimate_text_height(...)` and `pub fn fit_font_size(text, width, max_lines,
line_height) -> f32`) and use it from **both** `src/validate.rs` and
`src/components.rs`. This satisfies the AGENTS.md "renderer and validator share the
model" rule and keeps the two sides from drifting.

**E2. `quote_slide`** (`src/components.rs:1019`): replace the char-count bucketing
with a width-aware fit: compute `fit_font_size(quote, column_width, max_lines=3,
line_height=1.2)` clamped to the tier scale (min 28px, max display tier). The
harness quote then lands near 57–62px instead of 113px.

**E3. `hero_slide`** (`src/components.rs:774`): clamp headline/display sizes by
length (e.g. >28 chars → one tier down) using the same helper.

**E4. `timeline` / `process_map` / `funnel_chart`** (`src/components.rs:3300`,
`3462`, and the funnel bars): recalibrate the hardcoded per-item height estimates
for the new tier scale — make them *derive* from `tokens.type_scale` (body/caption
sizes) instead of the old constants (56–70px items), and keep the aggressive/tight/
normal scaling tiers. Done criteria: the harness's 3-item timeline and 4-step
funnel render inside 405px.

**E5.** Keep existing sparse-text balancing (myth_fact) untouched.

**Tests (E):** unit tests per component asserting generated HTML contains
`font-size` values ≤ the pre-fix values for long content, and that
`validate_design(html)` on the generated fragment reports no `text_overflow`.

### Phase F — Full verification sweep

1. `cargo test` → all green (existing ~102 + new).
2. `cargo build --release`; copy to `~/.cargo/bin/deckmill` and
   `dist/deckmill-x86_64-linux-gnu` (pattern from the previous session).
3. `python3 generate_typology_test.py` → must now **fail** (gate) while slides
   overflow; then after E, must pass.
4. `python3 measure_overflow.py` → overflowing count must drop from 93 to ≤ ~5
   (tight-warning tolerance) after Phase E.
5. `validate-design dist/typology_carousel.html` → `passed: true, slide_count: 210`.
6. Viewer + carousel 200 on `localhost:8765`.

---

## 5. Hard boundaries

- **In scope:** `src/validate.rs`, `src/components.rs` (quote/hero/timeline/
  process_map/funnel_chart scaling), `src/main.rs` + `src/mcp_server.rs` (gate
  wiring), `generate_typology_test.py` (gate), new `src/overflow_model.rs`.
- **Out of scope:** the 4:5 base-composition model, the renderer's canvas scaling,
  adding an HTML-parser dependency, redesigning typography tokens, regenerating
  large output sets as a substitute for root-cause fixes (AGENTS.md).
- Do **not** touch `src/slides.rs` render loop or `src/styling.rs` axes unless a
  Phase-E fix proves impossible at the component layer (stop and report first).

## 6. Escape hatches

- If the fixed regex splits incorrectly on any real deck (e.g. slides containing
  `<div class="slide">` inside their content), STOP and report the failing HTML
  rather than weakening the token check.
- If the estimator disagrees badly with `measure_overflow.py` (validator flags >
  1.2× or < 0.7× the measured set), STOP and re-calibrate constants — do not ship
  a validator that cries wolf or a gate that misses.
- If a Phase-E component fix cascades (e.g. timeline needs its layout changed, not
  just fonts), STOP and report; do not rewrite the component.

## 7. Maintenance notes

- The regex split + `css_vars` resolution are coupled to the per-slide `<style>`
  blocks emitted by `render_carousel_html` (`src/slides.rs`). Any future change to
  the slide markup (id/class order) must re-run the Phase A test.
- The shared `overflow_model` is the single calibration point for both renderer and
  validator; future typology tier changes must update it (and re-run Phase F).
- `measure_overflow.py` and `prototype_validator_fix.py` are scratch tools at the
  repo root; move them under `scripts/` (or delete `prototype_validator_fix.py`)
  once Phase F is green.
