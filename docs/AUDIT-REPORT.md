# SlideForge Rust Port: Audit Report & Refactor Plan

## Executive Summary

The Rust port (`slideforge-rust`) has **structural parity** with the Python implementation but suffers from **CSS-level divergence** that causes visual bugs. The core issue is that CSS variable ordering, padding values, and theme-aware color determination differ between implementations. This report catalogs all identified parity gaps and provides an actionable refactor plan.

---

## Part 1: Architecture Comparison

### Module Mapping (Python → Rust)

| Python Module | Rust Module | Status | Notes |
|---|---|---|---|
| `design_system.py` (1481 LOC) | `design_system.rs` (767 LOC) | ✅ Ported | Color math, OKLCH, WCAG contrast, palette derivation |
| `components.py` (3118 LOC) | `components.rs` (2065 LOC) | ✅ Ported | Slide generators (hero, feature, list, etc.) |
| `layouts.py` (322 LOC) | `layouts.rs` (291 LOC) | ✅ Ported | Layout primitives, slide_base, centered_layout |
| `blocks.py` (369 LOC) | `blocks.rs` (382 LOC) | ✅ Ported | Text blocks, badges, buttons, icons |
| `effects.py` (185 LOC) | `effects.rs` (167 LOC) | ✅ Ported | Glass, floating shapes, noise overlays |
| `archetypes.py` (840 LOC) | `archetypes.rs` (222 LOC) | ⚠️ Partial | Registry exists but unused in test flow |
| `slide_registry.py` (624 LOC) | `slide_registry.rs` (251 LOC) | ⚠️ Partial | Metadata registry exists |
| `dataviz.py` (473 LOC) | `dataviz.rs` (401 LOC) | ✅ Ported | SVG charts |
| `platforms.py` (135 LOC) | `platforms.rs` (150 LOC) | ✅ Ported | Platform presets |
| `validate/design_validator.py` (2148 LOC) | `validate.rs` (244 LOC) | ❌ **Major Gap** | Python has full HTML validation; Rust only has spec validation |
| `server.py` (899 LOC) | `mcp_server.rs` (716 LOC) | ✅ Ported | MCP server |
| `slides/renderer.py` (710 LOC) | `slides.rs` (709 LOC) | ✅ Ported | Carousel HTML rendering |
| `images/` (359 LOC) | N/A | ❌ **Missing** | Image search/providers not ported |
| `export/playwright_export.py` (178 LOC) | `export.rs` (102 LOC) | ⚠️ Partial | Export exists but simplified |
| N/A | `test_full_scope_rust.py` | ✅ Added | Test harness for Rust |

### Missing Modules (Not Ported)

1. **`validate/design_validator.py`** — Full HTML validation (contrast, safe zones, text density, overflow detection)
2. **`images/`** — Pexels/Unsplash image search and pattern application
3. **`themes/presets.py`** — Theme preset registry (partially inlined in `slides.rs`)
4. **`block_renderer.py`** — Block-to-HTML rendering (merged into `blocks.rs`)

---

## Part 2: CSS Parity Gaps (Root Cause of Visual Bugs)

### 2.1 CSS Variable Ordering

**Issue**: Rust uses `HashMap<String, String>` for design tokens, which has **non-deterministic iteration order**. Python uses ordered dicts or explicit ordering.

**Impact**: CSS variables like `--shadow-sm`, `--shadow-md`, `--text-micro-size`, `--space-1` appear in random order between renders. While CSS specificity should handle this, some edge cases with `var()` fallbacks may behave differently.

**Evidence** (carousel_10_brand_storyteller diff):
```css
/* Python order */
--shadow-inner: inset 0 2px 4px rgba(0,0,0,0.06);
--shadow-lg: 0 10px 15px -3px rgba(0,0,0,0.08);
--shadow-md: 0 4px 6px -1px rgba(0,0,0,0.08);
--shadow-sm: 0 1px 2px rgba(0,0,0,0.05);

/* Rust order (random HashMap iteration) */
--shadow-sm: 0 1px 2px rgba(0,0,0,0.05);
--shadow-md: 0 4px 6px -1px rgba(0,0,0,0.08);
--shadow-inner: inset 0 2px 4px rgba(0,0,0,0.06);
--shadow-lg: 0 10px 15px -3px rgba(0,0,0,0.08);
```

**Fix**: Replace `HashMap` with `IndexMap` or use `Vec<(String, String)>` for ordered token storage.

### 2.2 Bottom Padding Difference

**Issue**: Rust uses `padding: 80px 52px 80px` while Python uses `padding: 80px 52px 90px`.

**Impact**: Content is 10px closer to the bottom in Rust, causing overlap with the progress bar and overlays.

**Evidence** (carousel_14_thought_leader diff):
```
Python: height: 100%; padding: 80px 52px 90px; width: 100%;
Rust:   height: 100%; padding: 80px 52px 80px; width: 100%;
```

**Fix**: Update `slides.rs` line 73 from `80px` to `90px` for bottom padding.

### 2.3 Progress Bar Position

**Issue**: Rust positions the progress bar at `bottom: 52px` while Python uses `bottom: 42px`.

**Impact**: Progress bar is 10px higher in Rust, creating uneven spacing.

**Evidence**:
```
Python: position: absolute; bottom: 42px; left: 28px; right: 28px;
Rust:   position: absolute; bottom: 52px; left: 28px; right: 28px;
```

**Fix**: Update `slides.rs` line 189 from `bottom: 52px` to `bottom: 42px`.

### 2.4 Progress Bar Z-Index

**Issue**: Rust uses `z-index: 25` while Python uses `z-index: 50`.

**Impact**: Progress bar may be hidden behind overlay elements in Rust.

**Evidence**:
```
Python: z-index: 50;
Rust:   z-index: 25;
```

**Fix**: Update `slides.rs` line 191 from `z-index: 25` to `z-index: 50`.

### 2.5 Progress Bar Chip Dimensions

**Issue**: Rust uses smaller chip dimensions than Python.

**Impact**: Progress indicators are less visible in Rust.

**Evidence**:
```
Python: height: 4px; flex: 1; border-radius: 2px;
Rust:   height: 3px; flex: 1; border-radius: 1.5px;

Python: height: 5px; flex: 2.2;  (active)
Rust:   height: 4px; flex: 1.8;  (active)
```

**Fix**: Update chip dimensions in `slides.rs` to match Python values.

### 2.6 Progress Bar Chip Opacity

**Issue**: Rust uses lower opacity for completed/active chips.

**Impact**: Progress indicators are less visible in Rust.

**Evidence**:
```
Python completed: opacity: 0.55;
Rust completed:   opacity: 0.4;

Python active: opacity: 1;
Rust active:   opacity: 0.9;
```

**Fix**: Update opacity values in `slides.rs` to match Python.

### 2.7 Swipe Arrow Z-Index

**Issue**: Rust uses `z-index: 25` for swipe arrow while Python uses `z-index: 50`.

**Impact**: Swipe arrow may be hidden behind overlays.

**Evidence**:
```
Python: z-index: 50;
Rust:   z-index: 25;
```

**Fix**: Update `slides.rs` line 265 from `z-index: 25` to `z-index: 50`.

### 2.8 Slide Overlay Z-Index

**Issue**: Rust uses `z-index: 30` for slide overlays while Python uses `z-index: 45`.

**Impact**: Overlays may not appear above content properly.

**Evidence**:
```
Python: .slide__overlay { ... z-index: 45; ... }
Rust:   .slide__overlay { ... z-index: 30; ... }
```

**Fix**: Update `slides.rs` line 273 from `z-index: 30` to `z-index: 45`.

---

## Part 3: Theme-Aware Color Determination Bug

### 3.1 The "Dark" Theme Override Bug

**Issue**: When `theme="dark"` and `bg_style="light"`, the CSS overrides `--surface-light` to `#1A1A2E` (a dark color), but `is_dark_bg("light")` returns `false`, causing light-colored text to render on a dark background.

**Root Cause**: Both Python and Rust have this same logic issue, but it manifests differently:

```python
# Python (layouts.py:57)
is_dark = bg_style in ("dark", "hero", "gradient") or _is_visually_dark_surface(effective_surf)
```

```rust
// Rust (layouts.rs:59)
let is_dark = is_dark_bg(bg_style) || is_visually_dark_surface(&eff_surf);
```

Both implementations DO check `is_visually_dark_surface()`, but the **effective surface calculation** differs:

- **Python**: `effective_surf = theme_surface_overrides(theme, "light")` → `#1A1A2E` (dark)
- **Rust**: Same logic, BUT the `get_slide_colors()` function uses the **wrong base colors** when `is_dark=true` but `bg_style="light"` because `is_dark_bg()` is used in `slide_base()` to determine CSS classes.

**The CSS Class Problem**:
- Python CSS: `.slide--light { background-color: var(--surface-light); color: var(--text-primary); }`
- When `theme="dark"`, CSS overrides `--surface-light: #1A1A2E` but `.slide--light` class still applies `color: var(--text-primary)` (dark text)
- The Rust component correctly generates light text colors via `get_slide_colors()`, but the CSS class `.slide--light` overrides them with dark text colors

**Impact**: White text on dark background when `theme="dark"` with `bg_style="light"`.

**Fix**: The CSS needs to respect the visual darkness, not just the `bg_style` class. Options:
1. Use `slide--dark` class when `is_visually_dark_surface()` is true, regardless of `bg_style`
2. Add CSS overrides for `.slide--light` when `--surface-light` is visually dark
3. Apply inline styles with higher specificity

### 3.2 Text Coloring in carousel_10_brand_storyteller

**Specific Bug**: Slide 2 shows white text on what appears to be a light background because:
1. `bg_style="gradient"` → `is_dark_bg()` returns `true`
2. `get_slide_colors()` returns light text colors (`#F5ECEA`)
3. CSS class `.slide--gradient` applies dark background
4. BUT the visual appearance depends on `--gradient` CSS variable which uses `var(--surface-dark)` as base

The text colors ARE correct for dark backgrounds. The visual issue may be that the **gradient** is rendering differently due to CSS variable ordering or missing gradient definitions.

---

## Part 4: Layout Bugs

### 4.1 carousel_1_educator slide-4 (Callout Layout)

**Issue**: Layout is not aesthetically designed; progress slider not visible.

**Root Cause**: The callout component generates a nested structure with:
- Outer container: `position:relative;width:100%;height:100%;display:flex;align-items:center;justify-content:center;`
- Inner callout: `background: #010105CC; border-radius: 16px; border-left: 6px solid #5B5CE7;`

The problem is that the callout is **too large** relative to the slide, and the progress bar may be hidden behind the callout's z-index.

**Fix**: Reduce callout padding and ensure progress bar z-index (50) > callout z-index (2).

### 4.2 carousel_14_thought_leader slide-2 (Overflow)

**Issue**: Content overflows to the right.

**Root Cause**: The CTA component uses `text-align: center` inside a flex container with `padding: 80px 64px 80px`. The inner content (`position:relative;width:100%;height:100%;display:flex;align-items:center;justify-content:center;`) doesn't account for the 64px side padding.

**The Layout Structure**:
```html
<div style="position:relative;z-index:10;padding:80px 64px 80px;display:flex;...">
  <div style="position:relative;width:100%;height:100%;display:flex;align-items:center;justify-content:center;...">
    <!-- CTA content here - width:100% but parent has 64px padding -->
  </div>
</div>
```

The inner `width:100%` is relative to the parent's content box (which is `100% - 128px`), but the glass container and button may exceed this width.

**Fix**: 
1. Add `max-width: 100%` and `overflow: hidden` to inner containers
2. Reduce padding on slides with centered content
3. Use `box-sizing: border-box` consistently

### 4.3 carousel_10_brand_storyteller slide-4 (Component Sizing)

**Issue**: Component size not adjusted based on content; margins/paddings not scaling correctly.

**Root Cause**: The slide layout uses fixed padding (`80px 52px 80px`) regardless of content size. Components like `grid_cards` with 3 cards need more vertical space than `callout` with a single paragraph.

**Fix**: Implement dynamic padding based on slide type:
- `hero`, `cta`, `callout`: `padding: 80px 52px 90px`
- `grid_cards`, `split_features`: `padding: 60px 40px 70px` (smaller to fit more content)
- `stat_row`, `timeline`: `padding: 70px 48px 80px`

### 4.4 carousel_11_data_analyst slide-3 & carousel_12_creator slide-2

**Issue**: Layout is ugly/not aesthetically designed.

**Root Cause**: These slides likely use `stat_row` or `chart` components with hardcoded dimensions that don't scale well.

**Fix**: Review component implementations for:
1. Proper use of `flex: 1` for equal distribution
2. Consistent spacing between elements
3. Responsive sizing based on content count

### 4.5 carousel_16_brand_storyteller slide-1 (Top Overflow)

**Issue**: Content overflows to the top.

**Root Cause**: The hero slide uses `justify-content: center` which centers content vertically. If the content is tall, it may overflow the top.

**Fix**: 
1. Use `justify-content: flex-start` with appropriate top padding
2. Or use `overflow: hidden` on the slide container
3. Or reduce content size for hero slides

---

## Part 5: Missing Features

### 5.1 Design Validator (Major Gap)

Python has a full `design_validator.py` (2148 LOC) that:
- Validates contrast ratios for all text/background pairs
- Checks safe zone clearance (progress bar, overlays)
- Validates text density (max 20% coverage)
- Detects overflow issues
- Provides auto-fix capabilities

Rust only has `validate.rs` (244 LOC) which validates slide specs against the registry but NOT the rendered HTML.

**Impact**: No automated way to detect visual bugs in Rust.

**Recommendation**: Port `design_validator.py` to Rust as a post-render validation step.

### 5.2 Image Search/Providers

Python has `images/providers.py` and `images/patterns.py` for Pexels/Unsplash integration. This is not ported to Rust.

**Impact**: No image search capability in Rust MCP server.

### 5.3 Theme Preset Registry

Python has `themes/presets.py` with per-slide-type theme configurations. Rust inlines these in `slides.rs` `get_theme_css_overrides()`.

**Impact**: Less maintainable; changes require editing `slides.rs` directly.

---

## Part 6: Refactor Plan

### Phase 1: Critical CSS Parity (1-2 hours)

**Goal**: Fix all CSS-level divergence that causes visual bugs.

| Task | File | Change | Priority |
|---|---|---|---|
| Fix bottom padding | `slides.rs:73` | `80px` → `90px` | P0 |
| Fix progress bar position | `slides.rs:189` | `bottom: 52px` → `bottom: 42px` | P0 |
| Fix progress bar z-index | `slides.rs:191` | `z-index: 25` → `z-index: 50` | P0 |
| Fix swipe arrow z-index | `slides.rs:265` | `z-index: 25` → `z-index: 50` | P0 |
| Fix overlay z-index | `slides.rs:273` | `z-index: 30` → `z-index: 45` | P0 |
| Fix chip dimensions | `slides.rs:249,257` | Match Python values | P1 |
| Fix chip opacity | `slides.rs:252-260` | Match Python values | P1 |

### Phase 2: Token Ordering (30 minutes)

**Goal**: Ensure CSS variables are output in deterministic order.

| Task | File | Change | Priority |
|---|---|---|---|
| Replace HashMap with IndexMap | `design_system.rs` | Use `indexmap::IndexMap` for all token maps | P1 |

### Phase 3: Theme-Aware Colors (1 hour)

**Goal**: Fix the "dark" theme override bug.

| Task | File | Change | Priority |
|---|---|---|---|
| Fix is_dark determination | `layouts.rs:59` | Use visual darkness, not just bg_style | P0 |
| Add CSS overrides for dark theme | `slides.rs` | Override `.slide--light` colors when surface is dark | P0 |

### Phase 4: Layout Fixes (2-3 hours)

**Goal**: Fix overflow and aesthetic issues.

| Task | File | Change | Priority |
|---|---|---|---|
| Fix CTA overflow | `components.rs` | Add max-width and overflow handling | P1 |
| Fix callout sizing | `components.rs` | Reduce padding, ensure z-index hierarchy | P1 |
| Fix hero top overflow | `components.rs` | Use flex-start with padding or overflow:hidden | P1 |
| Add dynamic padding | `layouts.rs` | Adjust padding based on slide type | P2 |

### Phase 5: Design Validator Port (4-6 hours)

**Goal**: Port Python's design validation to Rust.

| Task | File | Change | Priority |
|---|---|---|---|
| Port HTML parser | New `validate_html.rs` | Parse rendered HTML, extract styles | P2 |
| Port contrast checker | `validate_html.rs` | Validate text/bg contrast ratios | P2 |
| Port safe zone checker | `validate_html.rs` | Validate progress bar/overlay clearance | P2 |
| Port text density checker | `validate_html.rs` | Validate max 20% text coverage | P2 |
| Add auto-fix capabilities | `validate_html.rs` | Auto-adjust colors, spacing | P3 |

### Phase 6: Missing Features (Optional)

| Task | File | Change | Priority |
|---|---|---|---|
| Port image providers | New `images.rs` | Pexels/Unsplash integration | P3 |
| Port theme presets | `themes/presets.rs` | Per-slide-type theme configs | P3 |
| Port brand kits | `brand_kits.rs` | Save/load brand profiles | P3 |

---

## Part 7: Testing Strategy

### Current Test Coverage

- `test_full_scope_rust.py` generates 23 carousels with random configurations
- Tests cover all slide types, archetypes, themes, and platforms
- No automated visual regression testing

### Recommended Additions

1. **CSS Parity Test**: Generate same carousel with Python and Rust, diff CSS output
2. **Visual Regression Test**: Screenshot comparison between Python and Rust outputs
3. **Contrast Validator Test**: Run design validator on all generated carousels
4. **Overflow Test**: Check for content overflow in all slide types

---

## Part 8: Priority Summary

### P0 (Critical - Fix Immediately)
1. Bottom padding: `80px` → `90px`
2. Progress bar position: `bottom: 52px` → `bottom: 42px`
3. Progress bar z-index: `25` → `50`
4. Swipe arrow z-index: `25` → `50`
5. Overlay z-index: `30` → `45`
6. Theme-aware color determination (dark theme override)

### P1 (High - Fix This Week)
1. Token ordering (HashMap → IndexMap)
2. Progress bar chip dimensions and opacity
3. CTA overflow fix
4. Callout sizing fix

### P2 (Medium - Fix This Month)
1. Dynamic padding based on slide type
2. Design validator port
3. Hero top overflow fix

### P3 (Low - Future)
1. Image providers
2. Theme presets registry
3. Brand kits
4. Auto-fix capabilities

---

## Appendix: File-Level Changes Required

### `slides.rs`
- Line 73: Change `80px` to `90px` (bottom padding)
- Line 189: Change `bottom: 52px` to `bottom: 42px`
- Line 191: Change `z-index: 25` to `z-index: 50`
- Line 249: Change `height: 3px; border-radius: 1.5px` to `height: 4px; border-radius: 2px`
- Line 252-255: Update chip background colors and opacity
- Line 257: Change `height: 4px; flex: 1.8` to `height: 5px; flex: 2.2`
- Line 260: Change `opacity: 0.9` to `opacity: 1`
- Line 265: Change `z-index: 25` to `z-index: 50`
- Line 273: Change `z-index: 30` to `z-index: 45`

### `design_system.rs`
- Replace `HashMap<String, String>` with `IndexMap<String, String>` for ordered output
- Add `indexmap` dependency to `Cargo.toml`

### `layouts.rs`
- Line 59: Verify `is_dark` determination includes theme override check
- Add dynamic padding logic based on slide type

### `components.rs`
- Add `max-width: 100%` and `overflow: hidden` to inner containers
- Reduce padding for content-heavy slides
- Fix z-index hierarchy for callouts

### `Cargo.toml`
- Add `indexmap` dependency for ordered HashMaps
