# Deckmill Styling Architecture v2 — Audit + Thematic Expansion Plan

> Status: **PLAN — pending approval per directive #1651**. No source edits have been made.
> Compiled from verified source reads: `src/design_system.rs`, `src/layouts.rs`, `src/effects.rs`, `src/blocks.rs`, `src/archetypes.rs`, `src/mcp_server.rs`, `src/main.rs`.

---

## 1. Current state of the theming system

The system is a **multi-axis token generator**: one primary hex color flows through
OKLCH color math into a full `DesignTokens` set, which per-slide renderers consume
via `SlideColors`. There are **6 independent styling axes**, plus archetypes which
are a *compositional* axis (not a color axis):

| Axis | Values | Where resolved | Exposed on CLI | Exposed on MCP |
|---|---|---|---|---|
| **visual_theme** (hue rotation) | 6: `editorial`(0°) `bold`(30°) `minimal`(−20°) `dark`(10°) `vibrant`(60°) `natural`(150°) | `design_system.rs:485` | ❌ (only via `--style` misroute) | ✅ `configure_design` |
| **font_style** (pairing) | 7: `editorial` `warm` `technical` `bold` `classic` `rounded` `modern` | `get_font_pairing()` | ✅ `--style` | ✅ `font_style` |
| **preset** (secondary/tertiary chroma+hue) | 9: `tonal_spot` `vibrant` `neutral` `monochrome` `expressive` `fidelity` `rainbow` `fruit_salad` `content` | `derive_palette` preset match | ✅ `--preset` | ✅ `preset` |
| **type_scale** (base × ratioⁿ) | base=16, ratio=1.25 (hardcoded in CLI; MCP allows override) | `derive_palette` | ❌ | ✅ |
| **bg_style** (surface) | 5: `light` `dark` `gradient` `mesh` `hero` | `effects.rs` slide_background | ✅ `--bg-style` | ✅ `bg_style` |
| **primary color** | any hex | OKLCH parse | ✅ | ✅ |

### How tokens flow (operationalization)

```
primary_hex + visual_theme + preset
        │  OKLCH: rotate hue (theme_offset), offset secondary/tertiary hue (preset), clamp chroma
        ▼
   DesignTokens (16 core tokens + type_scale + fonts + gradients)
        │  to_css_variables() → CSS vars for the full deck
        ▼
   get_slide_colors(tokens, is_dark)   [layouts.rs]
        │  contrast arbiter: clamps text/primary/button colors to WCAG AA
        ▼
   SlideColors { text_primary, text_secondary, primary, button_bg, button_text, border, is_dark }
        │  consumed by every slide function in components.rs
        ▼
   slide_base(420×525) → slide HTML → validator → PNG
```

### The archetype layer (compositional, not color)

6 archetypes (`archetypes.rs`): `educator`, `thought_leader`, `startup_pitch`,
`brand_storyteller`, `data_analyst`, `creator`. Each carries:
- `primary_theme` (light/dark/warm/vibrant) — **declarative only**
- `default_bg_style` — **decorative strings** like `clean_white`, `dark_gradient`,
  `vibrant_dark`, `warm_gradient`, `structured_light`, `neon_gradient` — **these do
  NOT map to the 5 real bg_styles**; nothing consumes them downstream
- `slide_presets` — per-slide-type composition settings: alignment, variant, glass,
  decorations, padding, justify, headline_gradient, accent_usage — **these ARE consumed** by dispatch

---

## 2. Operational gaps (verified)

1. **Theme forces font pairing — the biggest variance killer.**
   `mcp_server.rs:546-555` maps `visual_theme → font_style` lossily:
   `minimal→modern`, `dark→technical`, `vibrant→rounded`, `natural→warm`.
   You cannot express "editorial serif + vibrant hue" or "rounded font + minimal
   theme" — the theme hard-couples to a font family. The two axes are mathematically
   independent (hue rotation vs font selection) but the API treats them as one.

2. **CLI configure-design is a stub.** `main.rs:303-315` exposes only
   `--primary --style --preset --output`, hardcodes `visual_theme=""`,
   `type_scale_base=16`, `type_scale_ratio=1.25`. So an agent driving the CLI gets
   **1/6 of the theme space** the MCP surface offers, and cannot pass
   `visual_theme` / `type_scale` / `bg_style` at all.

3. **Archetype `default_bg_style` is dead config.** `clean_white`, `dark_gradient`,
   `vibrant_dark`, `warm_gradient`, `structured_light`, `neon_gradient` are not real
   bg_styles. If an agent sets `archetype=thought_leader` expecting its declared dark
   gradient surface, the renderer ignores it (dispatch reads `bg_style` param, not
   archetype metadata).

4. **layout_theme / effect_theme are stored but unused.** `ConfigureDesignRequest`
   accepts them; `state.layout_theme="asymmetric"`, `state.effect_theme="glass"` are
   persisted — nothing downstream consumes either field. Dead API surface.

5. **All 6 themes are hue rotations of ONE color.** There is no notion of
   *palette family* (analogous/complementary/triadic), no duotone/gradient-color
   family, no light-vs-dark *theme pair* (a light theme and a dark theme are the same
   hue rotated differently — not two surfaces of one brand).

6. **Presets only shape secondary/tertiary.** The 9 presets offset hue of
   secondary/tertiary and scale chroma, but never restructure the *relationship*
   (e.g. a true monochrome still lets primary drift; a duotone is absent).

---

## 3. Brainstorm — the ideal theming permutation space

### 3a. The full Cartesian product (what "variance" should mean)

An agent should be able to pick **independently**:

```
theme_family × font_pairing × surface × palette_mode × type_scale × primary
```

with **no hidden coupling**. That product, at current inventory:
6 themes × 7 fonts × 5 bg × 9 presets × 3 type scales × ∞ colors = **~5670+ distinct looks**
— but only if the coupling in gap #1 is broken.

### 3b. New theming dimensions worth adding (ranked by leverage)

| Dimension | Proposal | Why it adds real variance |
|---|---|---|
| **theme_family** (new) | `neutral` (default) · `analogous` (±30°) · `complementary` (180°) · `triadic` (120°) · `split-complement` (150°/−30°) · `mono` (chroma→0) | Palette *relationships*, not just hue nudges. This is the single biggest visual differentiator — analogous feels calm, complementary pops, triadic is playful |
| **duotone / gradient pairing** | pick a second color or `auto` (derives from hue family); used by hero/gradient/mesh bg + image overlays | Kills the "single accent on gray" monotony; gives editorial color-blocking |
| **theme pair (light+dark)** | `--theme-pair light:dark` — one brand, two surfaces | Carousels currently alternate bg but colors drift; a true pair keeps brand identity across the DLD rhythm |
| **surface treatment** | `flat` · `glass` (existing 3) · `outline` · `gradient-fill` — separate axis from bg_style | bg_style is the *canvas*; surface is the *card chrome*. Today glass is per-archotype boolean; should be a first-class axis |
| **type scale presets** | `compact` (1.15) · `standard` (1.25) · `airy` (1.4) | Typographic voice, cheap to implement (already formula-driven) |
| **shape language** | `sharp` · `rounded` · `organic` (border-radius family) | Currently radius is hardcoded per-component; a token-driven radius scale would unify it |
| **motion/energy** | `calm` · `standard` · `high` (decor density, blur strength, gradient intensity) | Low-cost amplitude knob for mesh/gradient/shape decoration |

### 3c. Thematic *typologies* (curated bundles — the "styles" users actually mean)

Instead of 5670 raw combos, expose **curated thematic presets** that pin the full
matrix to a coherent identity. Proposal — 10 typologies spanning the spectrum:

| # | Typology | Font | Theme family | Surface | bg_style | Mood |
|---|---|---|---|---|---|---|
| 1 | **Editorial** (exists) | Playfair+DM Sans | neutral | glass | light | premium journalism |
| 2 | **Startup** (exists) | Plus Jakarta | neutral | glass | gradient | clean SaaS |
| 3 | **Technical** (exists) | Space Grotesk | mono | flat+outline | dark | engineering |
| 4 | **Brutalist** (new) | Fraunces + mono | complementary | outline, no radius | dark | loud, raw |
| 5 | **Luxury** (new) | Cormorant Garamond | analogous | flat, thin rules | light | high-end |
| 6 | **Playful** (new) | rounded family | triadic | glass, big radius | mesh | friendly |
| 7 | **Vintage** (new) | serif display | mono warm | flat, grain | light | retro editorial |
| 8 | **Data** (new) | mono + grotesque | split-complement | flat | dark | analytics dashboard |
| 9 | **Nature** (new) | Lora | analogous (green bias) | flat+glass | mesh | organic calm |
| 10 | **Nightlife** (new) | rounded + script accent | complementary | glass + glow | hero | neon night |

Each typology = one `StylingBundle` struct pinning: font_pairing, theme_family,
hue_bias (optional), surface, bg_style, radius_scale, decoration_level, type_scale.
This is the "ideal permutation" answer: **curated bundles for humans, full matrix for
power users.**

---

## 4. Formalization — proposed architecture changes

### 4a. Break the theme→font coupling (gap #1, highest priority)

- `visual_theme` stops implying `font_style`. In `configure_design`, resolve
  `style` from `font_style` when provided; only fall back to the current mapping when
  `font_style` is absent (keep backward compat — the mapping stays as the *default*,
  no longer the *only* path).
- Add `theme_family` as its own param (default `neutral`) driving the new
  hue-relationship math in `derive_palette` (secondary/tertiary offsets become
  family-derived, presets become optional refinements on top).

### 4b. Complete the CLI surface (gap #2)

Extend `configure-design` with: `--visual-theme`, `--font-style` (decoupled),
`--theme-family`, `--type-scale-base`, `--type-scale-ratio`, `--bg-style`,
`--surface`, `--typology` (curated bundle selector). CLI and MCP expose identical
dimensions thereafter.

### 4c. Wire archetype bg + dead fields (gaps #3–4)

- Map archetype `default_bg_style` to real bg_styles
  (`clean_white→light`, `dark_gradient→gradient`, `vibrant_dark→dark`,
  `warm_gradient→gradient`, `structured_light→light`, `neon_gradient→hero`), and make
  dispatch honor archetype bg when slide doesn't override it.
- Either consume `layout_theme`/`effect_theme` (map to surface treatment +
  layout spacing) or drop them from the request schema — no dead API surface.

### 4d. Validator additions (per AGENTS.md contract)

- `validate_design`: if `--typology` used, assert rendered slide matches the
  typology's surface/radius expectations (surface token present, radius family
  applied) — catches renderers that bypass tokens (the recurring bug class).
- New `theme_family` check: secondary/tertiary contrast vs primary + surface must
  still pass the existing AA arbiter after family offsets.

### 4e. StylingBundle registry

New `src/styling.rs` with the 10 typology bundles + a `resolve_styling(bundle_name)`
that returns the full axis pin set — consumed by both CLI and MCP. This is the
single source of truth the random-styles harness will sweep next.

---

## 5. What an agent configures today (the answer to the user's question)

**Before any slide generation**, the agent calls `configure_design` (MCP) or
`configure-design` (CLI), then `generate_slide` per slide. Today it can set:

```
MCP:  primary_color, font_style, visual_theme, preset, type_scale_base/ratio,
      bg_style, archetype, platform, aspect_ratio, layout_theme (unused),
      effect_theme (unused), brand_name/handle/topic/url/hashtags
CLI:  primary, style, preset, bg_style, archetype, platform, aspect_ratio,
      tokens_file, override KEY=VALUE
```

The agent effectively chooses **1 primary color, 1 of 7 fonts, 1 of 6 themes, 1 of 9
presets, 1 of 5 bg styles, 1 of 6 archetypes** — but the theme/font coupling (gap #1)
collapses that to **~6 effective "theme packages"** unless it passes `font_style`
explicitly on MCP. The intended v2 upgrade: decoupled axes + theme_family + curated
typologies, so the same inventory yields ~10 curated identities or the full matrix.

---

## 6. Suggested implementation order

1. **P0 — decouple theme/font** (`mcp_server.rs` + `main.rs`), keep mapping as fallback. → unlocks the whole matrix
2. **P0 — theme_family math** in `derive_palette` (family → secondary/tertiary offsets), default `neutral` = today's behavior (no regression)
3. **P1 — `src/styling.rs`** with 10 curated typologies (bundles)
4. **P1 — CLI completion** (`--visual-theme --font-style --theme-family --type-scale-* --bg-style --surface --typology`)
5. **P1 — archetype bg mapping + consume layout/effect_theme or drop**
6. **P2 — validator checks** (typology compliance, family contrast)
7. **P2 — random-styles harness sweep** over the full matrix incl. typologies

Each step keeps the #2097 verification chain: cargo test → release build → dual
binary copy → regen → live HTML inspect.

---

## 7. Open questions for the user

1. **Curated typologies vs raw matrix?** Ship both (bundles as presets, full matrix
   as power mode)? Or one only?
2. **New Google Fonts**: typologies 5/7/10 want Cormorant Garamond, vintage serif
   display, script accents — OK to add font-families to the Google Fonts URL set?
3. **theme_family naming**: `neutral/analogous/complementary/triadic/split-complement/mono` — good, or a different vocabulary?
4. **Scope of this pass**: implement P0+P1 in one arc, or P0 only first and review
   the typology catalog before building `styling.rs`?
