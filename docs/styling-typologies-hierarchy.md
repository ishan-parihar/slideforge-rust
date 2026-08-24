# Deckmill Styling Hierarchy — Formal Specification

> Status: **PLAN — final review pending. No source edits.**
> Supersedes the brainstorm section of `docs/styling-architecture-v2.md`.
> Decision (confirmed): curated bundles are the primary agent path; the raw axis space
> remains reachable via `--override` and via free operator composition.

---

## 1. The model: anchors, not walls

The system is an **open axis space**. A typology is a *named anchor* — one fully-pinned point
in that space. A variant is a *formal operator transform* applied to the anchor, never a
mood-word pick. The 3 shipped variants per typology are the same 3 canonical operators for
every typology; operators compose freely, so the space is open, not a 10×3 table.

```
AXIS SPACE (open)                      TYPOLOGY (anchor)        VARIANT (transform)
font(11) × hue(f32) × family(7)  ──▶   pins all 12 axes    ──▶  operator ∘ anchor
× preset(9) × bg(5) × surface(6)       "editorial"               polarity | energy | material
× tier(3) × radius(3) × decoration(3)                             (compose freely)
× weight(3) × case(3) × tracking(3)
≈ 36,450 look points                  10 curated anchors          ∞ composed points
```

---

## 2. Axis vocabulary (formal, closed enums with concrete mappings)

### 2.1 `font_pairing` — 11 values
Existing 7 (verified in `get_font_pairing`, design_system.rs:325) + 4 new:

| id | heading | body | heading weights | body weights |
|---|---|---|---|---|
| `editorial` | Playfair Display | DM Sans | 300,600 | 400,500,600 |
| `warm` | Lora | Nunito Sans | 400,600 | 400,500,600 |
| `technical` | Space Grotesk | Space Grotesk | 300,600 | 400,500 |
| `bold` | Fraunces | Outfit | 300,600 | 400,500,600 |
| `classic` | Libre Baskerville | Work Sans | 400,700 | 400,500,600 |
| `rounded` | Bricolage Grotesque | Bricolage Grotesque | 600 | 400,500 |
| `modern` | Plus Jakarta Sans | Plus Jakarta Sans | 700 | 400,500,600 |
| `luxury` ⭐ | Cormorant Garamond | DM Sans | 400,600 | 400,500,600 |
| `vintage` ⭐ | DM Serif Display | Space Grotesk | 400 | 400,500 |
| `data` ⭐ | Space Grotesk | IBM Plex Mono | 400,600 | 400,500 |
| `nightlife` ⭐ | Plus Jakarta Sans | Playfair Display italic | 700 | 400,500,600 |

⭐ = new Google Font (appends to existing `fonts.googleapis.com/css2` URL builder — no new mechanism).

### 2.2 `hue_bias` — f32 degrees ∈ [−180, +180]
Added to primary hue *before* family math. Named themes today (editorial 0°, bold +30°,
minimal −20°, dark +10°, vibrant +60°, natural +150°) become the 6 nearest named anchors;
any float is legal.

### 2.3 `family` (color-scheme) — 7 values, concrete sec/tert math
Replaces/augments the preset table's fixed offsets (design_system.rs:512). `neutral` is
byte-identical to today → zero regression.

| family | sec offset | sec chroma | tert offset | tert chroma | structural |
|---|---|---|---|---|---|
| `neutral` | 0 | 0.44 | +60 | 0.67 | — |
| `analogous` | +25 | 0.44 | +50 | 0.55 | — |
| `complementary` | +180 | 0.44 | +160 | 0.44 | — |
| `triadic` | +120 | 0.44 | +240 | 0.44 | — |
| `split-complement` | +150 | 0.44 | −30 | 0.50 | — |
| `monochrome` | — | — | — | — | chroma→0 all; single accent = primary |
| `duotone` | — | — | — | — | surface = tint of primary; 2-color system |

### 2.4 `preset` — 9 existing values (tonal_spot, neutral, monochrome, expressive, rainbow,
fruit_salad, content, fidelity, vibrant). Kept for back-compat; typology picks one default.

### 2.5 `bg` — 5 values: `light` `dark` `gradient` `mesh` `hero` (existing).
### 2.6 `surface` — 6 values: `flat` `glass-light` `glass-dark` `frosted` `outline`
`gradient-fill`. Maps to card fill/blur/border/shadow. Existing archetype glass flags map
1:1 to glass-light/glass-dark.
### 2.7 `type_tier` — 3 values → ratio: `compact` 1.15 · `standard` 1.25 · `airy` 1.40.
### 2.8 `radius` — 3 values → (sm/md/lg): `sharp` (2,4,8) · `rounded` (8,12,16) · `organic` (12,20,28).
### 2.9 `decoration` — 3 values → amplitude multiplier: `calm` 0.5 · `standard` 1.0 · `high` 1.5
(scales float-shape density, blur strength, gradient intensity, noise opacity).
### 2.10 `weight` — 3 values → (heading/body): `low` (400,500) · `normal` (500,700) · `high` (700,900).
### 2.11 `case` — 3 values: `sentence` · `upper-headlines` · `all-caps` (heading text-transform).
### 2.12 `tracking` — 3 values: `tight` −0.5% · `normal` 0 · `wide` +2% (heading letter-spacing).

---

## 3. Typology anchors — full 12-axis pin, no mood words

| id | font | hue | family | preset | bg | surface | tier | radius | decor | weight | case | tracking |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `editorial` | editorial | 0 | neutral | tonal_spot | light | glass-light | airy | sharp | calm | normal | sentence | normal |
| `startup` | modern | 0 | neutral | tonal_spot | gradient | glass-light | standard | rounded | standard | normal | upper-headlines | tight |
| `technical` | technical | −10 | split-complement | content | dark | flat | compact | sharp | calm | normal | upper-headlines | tight |
| `brutalist` | bold | +30 | complementary | vibrant | dark | outline | compact | sharp | high | high | all-caps | wide |
| `luxury` | luxury | +15 | analogous | neutral | light | flat | airy | sharp | calm | low | upper-headlines | wide |
| `playful` | rounded | +60 | triadic | fruit_salad | mesh | glass-light | standard | organic | high | normal | sentence | normal |
| `vintage` | vintage | −20 | monochrome | neutral | light | flat | standard | organic | calm | normal | sentence | normal |
| `data` | data | +10 | split-complement | content | dark | flat | compact | sharp | calm | normal | upper-headlines | tight |
| `nature` | warm | +150 | analogous | content | mesh | glass-light | airy | organic | calm | normal | sentence | normal |
| `nightlife` | nightlife | +60 | complementary | vibrant | hero | glass-dark | standard | rounded | high | normal | upper-headlines | tight |

Every anchor is a **complete** tuple — no unspecified axes anywhere.

---

## 4. Variants — formal operator transforms (the answer to "are they random?")

**No.** There is exactly one rule, applied uniformly: a variant is `operator ∘ anchor`,
where each operator is a defined axis-transform. The 3 shipped variants per typology are
the 3 canonical operators; they compose, so any agent can build arbitrary depth.

### Operator definitions (formal)

| op | transforms | applied to axes |
|---|---|---|
| `polarity` | bg light↔dark (gradient/mesh/hero → dark; dark → light); surface glass-light↔glass-dark, flat/outline unchanged; preset → neutral; decoration −1 step; case/tracking/weight unchanged | bg, surface, preset, decoration |
| `energy` | family intensity +1 (neutral→analogous→complementary→triadic; split-complement→complementary; monochrome/duotone unchanged); decoration +1 step; preset → vibrant (when available) | family, decoration, preset |
| `material` | surface step +1 (flat→glass-light→glass-dark→frosted→outline→gradient-fill); radius +1 step (sharp→rounded→organic) | surface, radius |

### Canonical variant set (same for every typology)

| variant | definition | example: `editorial` (startup in parens) |
|---|---|---|
| `default` | anchor as-is | light / glass-light / neutral / calm (gradient / glass-light / neutral / standard) |
| `polarity` | `polarity ∘ anchor` | **dark** / **glass-dark** / neutral / **calm→calm** (dark / glass-dark / neutral / calm) |
| `energy` | `energy ∘ anchor` | light / glass-light / **analogous** / **standard** (gradient / glass-light / **analogous** / **high**) |

Composition example: `material ∘ polarity ∘ editorial` = dark / outline / neutral / calm / sharp
(fully specified by the operator algebra — nothing is left to taste).

---

## 5. Resolution algorithm (the single entry point)

```rust
fn resolve_styling(anchor: &str,
                   ops: &[VariantOp],            // polarity | energy | material, any order, any count
                   family_override: Option<Family>,
                   overrides: &[(&str, &str)])   // raw axis values, LAST precedence
    -> AxisSet {                                 // all 12 axes, fully pinned

    let mut s = TYPOLOGY_ANCHORS[anchor];        // complete tuple (section 3)
    for op in ops { s = apply_op(s, op); }       // section 4 operators
    if let Some(f) = family_override { s.family = f; }
    for (axis, val) in overrides { s.set(axis, val); }
    s
}
```

Precedence: `override > family_override > operator sequence > anchor > system default`.

`AxisSet → DesignTokens` mapping is one deterministic function (all enums already map to
concrete CSS/token values in section 2), so CLI and MCP produce identical output.

---

## 6. Agent-facing surface (CLI == MCP)

```
deckmill configure-design --primary <hex>
    --typology <editorial|startup|technical|brutalist|luxury|playful|vintage|data|nature|nightlife>
    --variant <default|polarity|energy>            (repeatable: any operator seq)
    --color-scheme <neutral|analogous|complementary|triadic|split-complement|monochrome|duotone>
    --surface <flat|glass-light|glass-dark|frosted|outline|gradient-fill>
    --type-tier <compact|standard|airy>
    --radius <sharp|rounded|organic>
    --decoration <calm|standard|high>
    --weight <low|normal|high>
    --case <sentence|upper-headlines|all-caps>
    --tracking <tight|normal|wide>
    --preset <tonal_spot|neutral|monochrome|expressive|rainbow|fruit_salad|content|fidelity|vibrant>
    --bg-style <light|dark|gradient|mesh|hero>
    --override <AXIS=VALUE>...                     (raw matrix; same axis names)
```

MCP `ConfigureDesignRequest` carries the same fields; `resolve_styling` is shared, so parity
is by construction.

---

## 7. Deployment phases (each ends with the #2097 chain)

| Phase | Scope | Files | Verify |
|---|---|---|---|
| **P0a** | Decouple theme↔font: explicit `font_style` wins; theme→font mapping becomes fallback only | mcp_server.rs, main.rs | existing carousels unchanged |
| **P0b** | `family` enum + offset math in derive_palette; `neutral` = today's exact behavior | design_system.rs | 40/40 regen byte-identical |
| **P1a** | `src/styling.rs`: enums + 12-axis AxisSet + 10 anchors + 3 operators + resolve_styling | styling.rs (new) | unit tests: every anchor complete, operators total, no dup ids |
| **P1b** | CLI flags (section 6) on configure-design + generate-slide | main.rs | CLI parity with MCP |
| **P1c** | MCP request fields + state (same surface) | mcp_server.rs | parity test |
| **P2a** | Archetype `default_bg_style` (clean_white/dark_gradient/vibrant_dark/warm_gradient/structured_light/neon_gradient — all dead today) mapped to real bg values; honored when slide doesn't override | archetypes.rs, dispatch | dead-config gap closed |
| **P2b** | `layout_theme`/`effect_theme` consumed (map to surface + spacing) or dropped | mcp_server.rs | no dead API |
| **P3** | 4 new fonts into URL builder + pairing table | design_system.rs | fonts load in PNG |
| **P4** | Validator: family contrast vs AA arbiter, surface/radius tokens present when pinned | validate.rs | new validator tests |
| **P5** | Harness: `generate_typology_test.py` — 10 anchors × 40 types, `--variant` + `--color-scheme` + cross-axis randomize **within** anchor | new script | `dist/typology_viewer.html` grouped by anchor, 400 slides, fingerprint per card |

---

## 8. Gates (final, before code)

1. **11 font pairings** (7 existing + luxury/vintage/data/nightlife) — approved?
2. **7 families** with the sec/tert table above — approved? (monochrome/duotone marked structural — OK?)
3. **Operator model for variants** (polarity/energy/material, composable) — this replaces the
   vague "3 variants" with a formal algebra. Confirm this is the variant mechanism you want.
4. **Anchor table (section 3)** — any axis pins you want changed before I lock them?
5. **`--override` + free operator composition** as the only raw paths — confirmed?
6. **Phase order P0→P5 in one arc**, or review gate after P1 (viewer on 10 anchors before P2–P5)?
