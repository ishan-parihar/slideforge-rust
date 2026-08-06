# Rendering Engine Audit — blitz vs Servo vs Chrome

> **Date:** 2026-08-06 · **Scope:** style-level and font-level rendering bugs in SlideForge
> **Status:** Root causes confirmed empirically. **Phase 1 (marker-class bleed refactor) + Phase 2 (deterministic font loading) + Phase 3 (validator gate) IMPLEMENTED** (2026-08-06). Phase 4 (Servo) optional/later.
> **Auditor:** Buffy (agent) on behalf of the maintainer.
>
> ### Implementation log — Phase 2 landed (deterministic font loading)
- `src/font_vendor.rs` (new): vendors Google Fonts CSS2 stylesheets into
  inline `data:font/woff2;base64,…` `@font-face` rules so the blitz renderer
  registers every glyph synchronously — zero async per-glyph fallback race.
  Injectable fetchers for hermetic tests; shared reqwest blocking client (one
  TLS session per vendor pass); FNV-1a disk cache under `~/.cache/slideforge/fonts`
  or `$SLIDEFORGE_FONT_CACHE`; graceful fallback to the original `<link>` on any
  fetch failure (exports never fail on fonts).
- `src/export.rs`: `build_standalone_slide_doc` now filters the carousel's font
  links per slide (keeps only the families the slide's `--font-heading` /
  `--font-body` css_vars declare) and vendors those; `render_html_to_png`
  (preview-slide) vendors the full document too. Per-slide subsetting is what
  keeps render time sane (was ≈3× slower when every slide embedded all 10
  pairings).
- `Cargo.toml`: `reqwest` (blocking) + `base64` added as direct deps — both
  already present in the lock as transitive deps, so zero version churn.
- Verified: `cargo test` 156/156 green (8 new tests incl. network-blocked
  fallback + standalone-doc zero-remote-refs regression); cold-cache export of
  a 2-slide typology slice = ~21s, warm = ~5s/slide; PNGs byte-identical
  across runs; cache entries carry only data-URI @font-face with zero
  `fonts.gstatic.com` / `fonts.googleapis.com` references.

### Implementation log — Phase 1 & 3 landed
> - `src/layouts.rs`: `slide_base` / `hero_slide_base` / `slide_base_bleed` now emit `class="sf-bleed-layer"` on their root wrapper div.
> - `src/slides.rs`: the three `:has()` rules (A/B/C below) were replaced with marker-class rules — `.slide--full-bleed .slide-body.sf-body-lift`, `.slide:not(.slide--full-bleed) .slide-body.sf-body-lift`, and `.slide:not(.slide--full-bleed) .slide-body > div.sf-bleed-layer:first-child`. `render_carousel_html` emits `sf-body-lift` on the body exactly when the slide root carries the marker (exact `class="sf-bleed-layer"` match), preserving the old gate semantics for non-conforming roots.
> - `src/validate.rs`: `validate_design` hard-errors with `unsupported_css_selector` when any `<style>` block (CSS comments stripped) contains `:has(` — enforced at compile time and in `validate-design`.
> - Verified: `cargo test` 145/145 green; regenerated `random_styles_carousel.html` has **0** `:has(` and 40/40 slides carry `sf-body-lift`; blitz pixel probe shows the header band is pixel-continuous with the body photo (Δ≈12) — the bleed behind the chrome now applies in the export renderer.

---

## 1. Executive summary

The visual breakage users are reporting (non-transparent header/footer, mixed
fonts where "A looks different in the header", inconsistent layouts, aspect-ratio
seams/"blur box" on non-4:5 canvases) has **two independent root causes**, both
in the renderer layer, not in the slide-type CSS itself:

1. **`src/slides.rs` emits CSS gated on `:has()` — and the embedded blitz/stylo
   engine silently drops every `:has()` rule.** The full-bleed background-bleed
   architecture (which makes header/footer transparent and stretches the image
   layer behind the corner chrome) depends on **exactly three `:has()` selectors**.
   Because stylo 0.19.0 (Servo's fork, in "servo" mode) does not implement
   `:has()`, those three rules never apply → backgrounds never bleed behind the
   chrome bands → the "non-transparent header/footer", band seams, and content
   misplacement bugs. **This is the dominant bug and the fix is in our own
   emitted CSS, not in the engine.**

2. **The Google-Fonts loading path in blitz is racy and per-glyph.** blitz keeps
   a static `fontdb::Database` populated only with **system fonts**, and loads
   `@font-face` web fonts asynchronously via `fetch_font_face()` →
   `FontInfoOverride` → `register_fonts()`. The exporter's resolve loop waits
   only on `net.is_empty()`, which does **not** guarantee font registration
   completed. When the paint happens before all @font-face fetches/registrations
   land, text is shaped with per-character `find_font_for(query, ch)` fallback
   that mixes a web font for one glyph and a system font for another → the
   mixed-alphabet symptoms. On real carousels (10 font `<link>`s, each CSS2
   stylesheet fanning out to many woff2 subsets), this race is the norm, not the
   exception.

**Would Servo fix it? Partially — but not the way you'd hope.**

- Servo and blitz share the **same underlying style engine** (`stylo`, maintained
  by the Servo project). The crates.io `servo` crate (LTS, 2026-04-13) — and the
  `stylo` 0.19.0 blitz embeds — **both lack `:has()`** in servo-mode selectors.
  `:has()` was only enabled in Servo via PR #44902 (May 2026), *after* the
  crates.io LTS cut. So switching to the currently published Servo crate would
  **not** fix bug #1.
- Switching engines would, however, give you better font fallback (Servo's Feb
  2026 release notes explicitly mention "improvements to font fallback") and a
  more complete layout engine — at the cost of heavier memory (software-GL
  rendering context, full WebView/Constellation architecture, tokio required),
  much slower compile times, and a rewrite of `export.rs`.

**Recommendation (decision matrix at §6):** stay on blitz for now. Fix the
`:has()` dependency in `src/slides.rs` (replace with explicit classes — blitz
fully supports class selectors, `:first-of-type`, `:not()`, `:last-child`,
`::before`/`::after`, `calc()`, flexbox, CSS variables — we verified each).
Fix the font race with a deterministic wait for `FontInfoOverride` registration
plus font-preloading. Then re-validate. Revisit Servo only if you need
font-fallback correctness *and* accept the memory/rewrite cost.

---

## 2. Evidence — blitz CSS support matrix (empirical)

Method: minimal HTML documents rendered through `slideforge preview-slide`
(current release binary), pixel-detected whether each rule applied.

| Feature | Result | Notes |
|---|---|---|
| Class / ID / element selectors | ✅ | baseline |
| Attribute selectors `[class]` | ✅ | |
| Adjacent sibling `+` | ✅ | |
| `:not(.x)` | ✅ | |
| `:first-child`, `:last-child` | ✅ | |
| `:first-of-type`, `:last-of-type` | ✅ | (earlier "IGNORED" was a probe bug — rule set red, we looked for blue) |
| `:only-child`, `:nth-child(n)`, `:nth-of-type(n)` | ✅ | |
| `:root` | ✅ | |
| `::before` / `::after` | ✅ | including `content:""` boxes |
| `calc()` incl. `calc(100px - var(--h))`, negative terms, multi-var | ✅ | |
| CSS custom properties (`var()`) | ✅ | incl. nested var composition |
| flexbox + `gap` | ✅ | |
| `:hover`, `:active` | ✅ (parse) | interaction pseudo-classes don't matter for export |
| **`:has(…)`** | ❌ **DROPPED** | every `:has()` rule is discarded wholesale |
| `:not(:has(…))` | ❌ **DROPPED** | same — the whole rule is dropped |
| `:is()` / `:where()` | ⚠️ untested | 0 occurrences in current source |

**Verdict:** the engine is remarkably complete — **except `:has()`.**
Every visual bug attributable to CSS in the exported PNGs can be traced to the
three `:has()` rules in `src/slides.rs` (lines 206, 323, 326).

---

## 3. Root cause #1 — the three `:has()` rules that hold the bleed architecture together

### Where the rules live (`src/slides.rs`)

```css
/* (A) full-bleed: lift body clip so the stretched bg layer can bleed          */
.slide--full-bleed .slide-body:has(> div:first-of-type > .slide-content,
                                    > div:first-of-type > .slide-content--bleed) { … }

/* (B) non-full-bleed: lift body clip for bg continuity behind chrome bands     */
.slide:not(.slide--full-bleed) .slide-body:has(> div:first-of-type > …) { … }

/* (C) non-full-bleed: stretch the bg layer to cover header+footer (comp+36px)  */
.slide:not(.slide--full-bleed) .slide-body > div:first-of-type:has(> .slide-content,
                                     > .slide-content--bleed) { position:absolute!important; … }
```

### Why they exist
The `:has()` gates were added so the bleed/stretch treatment only applies to
slide roots that conform to the `slide_base`/`slide_base_bleed` wrapper contract
(first child of `.slide-body` wraps `.slide-content` / `.slide-content--bleed`).
Non-conforming slide roots keep the old hard clip.

### Why they break under blitz
blitz embeds `stylo` 0.19.0 in **servo mode**. Servo-mode
`NonTSPseudoClass` (verified in the vendored crate:
`stylo-0.19.0/servo/selector_parser.rs`) has **no structural/relative selector
support** — the `:has` machinery lives only in `selectors`' gecko-mode code
paths. When stylo's parser hits `:has(`, the **entire rule** is rejected (it
doesn't "degrade gracefully" per-declaration; the selector list fails). The
result:

| Rule | Intent | Under blitz |
|---|---|---|
| (A) | bleed layer escapes body clip on full-bleed | ❌ dropped → layer clipped at body bounds |
| (B) | bg continuity behind chrome bands (normal slides) | ❌ dropped → chrome bands show the *slide-level* bg, not the bled surface |
| (C) | stretch bg layer to cover header+footer | ❌ dropped → the layer stays 420×449, header/footer bands show a different background |

### Symptom mapping (the bugs you reported)

- **"Header/footer still not fully transparent, especially on image slides"**
  → (B)+(C) dropped. The full-slide injected photo (or the slide_base surface)
  never stretches behind the 36px header / 40px footer bands, so those bands
  render the `.slide`-level background (`--surface-light`, mesh, etc.) instead
  of the slide's own surface/image. It is *not* that the band itself paints an
  opaque background — the CSS says `background: transparent` and blitz honors
  it — it's that **nothing is painted behind the band**, so you see the
  slide-level fill. This is exactly why it's most obvious on image slides.
- **"Aspect-ratio conversion creates a blur box / seam / misplaced content"**
  → (A) dropped on full-bleed canvases (1:1, 3:4, 9:16, 16:9). The stretched
  layer can't escape `.slide-body { overflow: hidden }`, so the bleed is cut at
  the composition edge and the bands show a different surface → visible seam /
  "blur box" (e.g. "04 Maison Verre — Autumn Atelier").
- **"Content pushed down / cropped on non-4:5"** → partial. The
  `.slide-content` re-anchor rules (`.slide:not(.slide--full-bleed) … >
  .slide-content { top: header-h; height: comp-h − header − footer }`) do NOT
  use `:has()` and DO apply. But without (C) the background layer they sit on
  top of is mis-positioned, so the composited result looks wrong.
- **Viewers look fine but exports look broken** → the HTML pools render in a
  real browser (Chrome) which *does* support `:has()`. The exported PNGs go
  through blitz. Same HTML, two different engines → divergence. This is why
  "it looks fine in the viewer" and "the export is broken" coexist.

### The fix (source-level, minimal, no engine change)

Replace the three `:has()` gates with **explicit marker classes** on the wrapper
elements. blitz fully supports class selectors. Two changes:

1. In `src/components.rs`, the wrapper contract already exists as
   `.slide-content` and `.slide-content--bleed`. Add a **bleed-capable marker
   class** at the same time these wrappers are emitted (e.g. add `sf-bleed` to
   the wrapper div, or emit `class="sf-bleed-layer"` on the layer div that
   `slide_base`/`slide_base_bleed` produce).
2. Rewrite the three rules to target the marker directly:

```css
.slide--full-bleed .slide-body > div.sf-bleed-layer { … }          /* replaces (A) */
.slide:not(.slide--full-bleed) .slide-body > div.sf-bleed-layer { … } /* replaces (C) */
```

For (B) — the body-clip lift — gate on the same marker:
`.slide--full-bleed .slide-body:has(> div:first-of-type > .slide-content)`
becomes `.slide--full-bleed .slide-body > div.sf-bleed-layer { overflow: visible !important }`
(put the `overflow` lift on the layer, or keep the lift on `.slide-body` gated
by `.slide-body > div.sf-bleed-layer:first-child` which needs no `:has`).

**Important:** keep `:first-of-type` — it works (verified). The only forbidden
selector in this architecture is `:has()`.

**Validator reinforcement (per AGENTS.md):** add a validator check that scans
the emitted carousel CSS for `:has(` and errors with
"blitz/stylo servo-mode does not implement :has(); replace with marker classes"
so this class of bug is caught at the gate, not in review.

---

## 4. Root cause #2 — Google Fonts race + per-glyph fallback

### The pipeline (verified in vendored blitz 0.3.0-beta.1 source)

1. `FONT_DB: LazyLock<Arc<fontdb::Database>>` — static, **system fonts only**
   (`db.load_system_fonts()`). 20,603 system fonts on this machine.
2. `@font-face` rules from the (fetched) CSS stylesheets are gathered in
   `document.rs` and passed to `net::fetch_font_face()`.
3. Font bytes come back asynchronously (blitz-net provider); blitz registers
   them into the parley query via `FontInfoOverride` (so the CSS-declared family
   name + weight/style are honoured) through `register_fonts(...)`.
4. Text shaping in `font_metrics.rs` resolves fonts **per character**:
   `find_font_for(query, ch)` iterates the query's font stack and returns the
   first font whose charmap covers `ch`. If a web font is registered with the
   right family name, glyphs in its charmap use it; glyphs it doesn't cover (or
   the whole run, if the web font never registered) fall back to the *next
   matching font in the system db*.

### Why it breaks on real carousels

- The carousel emits up to **10 Google Fonts `<link>` stylesheets**. Each
  `fonts.googleapis.com/css2` response contains many `@font-face` rules (one per
  weight **and** per unicode-range subset: latin, latin-ext, cyrillic, greek,
  vietnamese …). Each becomes a separate fetch → dozens of woff2 round-trips.
- The exporter resolve loop (`render_document_to_png`) polls
  `net.is_empty() || rounds > 500` (≈5 s max) then calls `document.resolve(0.0)`
  once more and paints. `net.is_empty()` reflects **in-flight fetches**, not
  whether `FontInfoOverride` registration was applied to the shaping query.
  With 10 stylesheets × many subsets, a tail of font events routinely lands
  after the last poll → final paint uses system fallback for the affected
  families/weights.
- Because fallback is **per character**, a partially-loaded family renders some
  glyphs in the web font and others in a system serif/sans → *"A looks
  different in the header text"*.

### Evidence

- A minimal single-family Google Fonts test (`Bangers`, one `<link>`, one
  weight) rendered **identically** (glyph width 705 px in both blitz and
  Chrome) — proving the fetch+register path works when there's no race.
- The real carousel header glyph-width vectors differ between engines
  (blitz `[16,8,9,18,…]` vs Chrome `[18,5,4,19,…]`) — proving the *multi-family,
  multi-weight, subset-fan-out* case falls back to different fonts.
- `font_metrics.rs`'s per-char `find_font_for(query, ch)` + `charmap.map(ch)`
  is the exact mechanism producing mixed-font output.

### The fix (deterministic, no engine change)

1. **Pre-load fonts before resolve:** in `export.rs`, fetch each Google Fonts
   stylesheet ourselves (or accept a set of woff2 URLs), download the woff2
   bytes, and register them with the document **before** the resolve loop via
   the same `FontInfoOverride` API blitz uses internally — then wait until
   `register_fonts` has been acknowledged, not merely until `net.is_empty()`.
2. **Or, simpler and deterministic:** vendor the font files. At generation time,
   resolve the Google Fonts CSS2 URL to concrete woff2 URLs (single latin subset
   is sufficient for our content), download once, and emit them as `data:` URIs
   or local `file://` fonts in the standalone slide documents. This removes the
   network from the render path entirely — the exporter never races.
3. Add an **export-time warning**: if a slide's HTML references a family that
   the exporter could not register, surface it in the CLI/MCP output (validators
   already run at compile time; extend `validate-design` to cross-check the
   compiled slide's `--heading`/`--body` families against the registered set).

---

## 5. Servo as an alternative — the honest assessment

### Facts (from servo.org blog + crates.io + docs.rs, Aug 2026)

- Servo is on crates.io (LTS cut **2026-04-13**). API: `ServoBuilder`, `Servo`,
  `WebView`, `SoftwareRenderingContext::new(size)` →
  `RenderingContext::read_to_image(rect) -> Option<ImageBuffer>`.
- `SoftwareRenderingContext` is **software OpenGL** ("generally bad
  performance, but consistent") — headless-capable, matches our needs.
- Servo **needs tokio + its full Constellation/WebView/embedder machinery**;
  `SoftwareRenderingContext` is `!Send`/`!Sync` (like blitz's `HtmlDocument`).
- **`:has()` was enabled in Servo only on 2026-05-13 (PR #44902)** — one month
  *after* the crates.io LTS. The published servo crate at latest crates.io
  version still lacks it (until the next release) — exactly like blitz.
- Feb 2026: "improvements to font fallback" landed in Servo's tree (post-LTS).

### What Servo would fix vs not

| Concern | Servo (current crates.io) | Servo (git master) |
|---|---|---|
| `:has()` rules working | ❌ (same gap as blitz) | ✅ (May 2026) |
| Font fallback correctness | ✅ better than blitz | ✅ best |
| Web font @font-face loading | ✅ full browser engine | ✅ |
| CSS completeness (calc, flex, etc.) | ✅ | ✅ |
| Memory footprint | ⚠️ heavier than blitz (full browser engine + software GL) | ⚠️ heavier |
| Compile time | ⚠️ very heavy (Constellation/WebView/stylo + all of Servo) | ⚠️ heaviest |
| `export.rs` rewrite | required (new API shape) | required |

### Conclusion on the engine question

- If the goal is "fix the current visual bugs": **staying on blitz + removing
  `:has()` from our CSS + deterministic font loading fixes everything we can
  attribute**, with a ~1-day change and no dependency churn. Servo's crates.io
  LTS wouldn't even fix the `:has()` rules.
- If the goal is long-term font correctness on arbitrary Google Fonts content
  (diacritics, exotic subsets, variable fonts) without hand-rolling font
  preloading: **Servo git master** is the right engine, but budget for a
  renderer rewrite and a binary/memory footprint increase. Revisit when the
  next servo crates.io release includes `:has()` + the Feb-2026 font-fallback
  work.

**Do not switch engines to fix `:has()` — that's a regression in the wrong
direction (bigger cost, same bug). Fix the CSS.**

---

## 6. Decision matrix — renderer options

| Option | Fixes `:has()` bugs | Font quality | Mem/disk | Effort | Verdict |
|---|---|---|---|---|---|
| **blitz 0.3.0-beta.1 + fix our CSS (marker classes)** | ✅ | ⚠️ needs preload fix | ✅ low (current) | Low (~1d) | **Choose this now** |
| blitz + vendored local fonts (data: URI) | ✅ | ✅ deterministic | ✅ | Low–Med | Add on top of the above |
| **servo crate (crates.io LTS)** | ❌ same gap | ✅ | ⚠️ heavy | High (rewrite) | ✗ not now |
| servo git master | ✅ | ✅✅ | ⚠️⚠️ heaviest | High + new deps | Revisit after next crates.io release |
| headless Chrome (previous) | ✅ | ✅✅ | ❌ huge (playwright/chromium) | reverted earlier | ✗ (we already left it) |

---

## 7. Refactor plan (phased)

### Phase 1 — unblock the bleed architecture (highest impact, ~1 day)

1. `src/components.rs`: emit a marker class on the wrapper produced by
   `slide_base` / `slide_base_bleed` (e.g. `sf-bleed-layer` on the root layer
   div, or `sf-bleed` on `.slide-content`/`.slide-content--bleed`).
2. `src/slides.rs`: rewrite rules (A)/(B)/(C) to target the marker classes —
   **no `:has()` anywhere**; keep `:first-of-type` (works).
3. Rebuild, re-export `typology_carousel.html` + `random_styles_carousel.html` +
   the 10 stress decks, and diff the band pixels: header band (y≈2%) must equal
   the body surface on image slides (transparent chrome now shows the bled
   layer), and no seam on 1:1/3:4/9:16.
4. Regenerate viewers (`typology_viewer.html`, `random_styles_viewer.html`,
   `stress_test_master.html`) and eyeball the same slides.

### Phase 2 — deterministic font loading (~1 day)

1. `export.rs`: replace the `net.is_empty()` wait with a wait that also tracks
   font registration (poll the document for
   pending `FontInfoOverride` applications, or pre-register woff2 data before
   resolve).
2. If polling the document is not exposed, **vendor the fonts**: resolve each
   Google Fonts CSS2 URL at generation time, download the latin-subset woff2,
   inline as `data:font/woff2;base64,…` in the standalone slide document, and
   register directly. Deterministic, offline, no race.
3. Add a `--no-font-race-check` style guard or a regression test: export the
   typology carousel with network blocked (`file://` only) and assert the header
   glyph widths match the fonts-available run.

### Phase 3 — validator gate (AGENTS.md contract) (~half day)

1. `src/validate.rs`: add `check_no_unsupported_selectors(html)` — scan compiled
   carousel CSS for `:has(` and error (hard fail at compile time and in
   `validate-design`).
2. Extend `validate-design` to cross-check that every `--heading`/`--body`
   family in a compiled slide resolves to a registered font in the carousel's
   font URLs (catches the font race at the gate too).
3. Add the two checks to the tests (`cargo test` stays 140+ green).

### Phase 4 — (optional, later) Servo

- Wait for the next servo crates.io release that includes `:has()` (PR #44902,
  May 2026) + the Feb-2026 font-fallback improvements.
- Prototype in a feature branch: `export.rs` behind a `--renderer servo` flag
  using `ServoBuilder` + `SoftwareRenderingContext::read_to_image`, run the same
  pixel-diff harness. Only adopt if memory/compile budget allows.

### Verification harness (keep it)

The selector-matrix probe used for this audit is reusable and should be
committed as a dev script (`scripts/renderer_css_probe.py`): it renders a
minimal document per CSS feature and prints SUPPORTED/DROPPED. Run it after any
blitz/stylo upgrade to catch regressions in the same class.

---

## 8. Evidence files / artifacts

- Probe scripts: `/tmp/sel*.py`, `/tmp/has_test*.html` (selector matrix),
  `/tmp/font_ab.py`, `/tmp/font_width.py`, `/tmp/glyph_diff.py`,
  `/tmp/glyph2.sh` (blitz-vs-Chrome font A/B).
- Vendored sources inspected:
  - `blitz-dom-0.3.0-beta.1/src/util.rs` (static FONT_DB = system fonts)
  - `blitz-dom-0.3.0-beta.1/src/net.rs` (`fetch_font_face`, FontFaceOverrides)
  - `blitz-dom-0.3.0-beta.1/src/document.rs` (`register_fonts` w/ overrides)
  - `blitz-dom-0.3.0-beta.1/src/font_metrics.rs` (per-char `find_font_for`)
  - `stylo-0.19.0/servo/selector_parser.rs` (no structural pseudo-classes in
    servo-mode NonTSPseudoClass)
  - `selectors-0.39.0` (relative-selector machinery exists but unused in servo mode)
- Web: servo.org blog (crates.io LTS 2026-04-13; Feb-2026 font-fallback;
  `:has` PR #44902 2026-05-13); docs.rs `servo` crate API.
