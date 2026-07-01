# Aspect-Ratio-Aware Composition Scaling

> **Status:** Plan — ready for implementation
> **Goal:** Fix distorted layouts for non-4:5 aspect ratios (9:16, 1:1, 3:4) by introducing a composition-based architecture that preserves the original 4:5 content proportions while extending the background to fill the target canvas.

## Problem Analysis

### Current Behavior (Broken)

The current vectoric scaling renders ALL slide content at a height proportional to the target aspect ratio:

| Aspect Ratio | Target Canvas | Base Canvas | Composition Height | Problem |
|-------------|--------------|-------------|-------------------|---------|
| **4:5** | 1080×1350 | 420×525 | 525px | ✅ Perfect — native ratio |
| **9:16** | 1080×1920 | 420×747 | 747px | ❌ Content stretched 42% vertically — overlays spread apart, text gaps inflated |
| **3:4** | 1080×1440 | 420×560 | 560px | ❌ Content stretched 7% — subtle but visible spacing distortion |
| **1:1** | 1080×1080 | 420×420 | 420px | ❌ Content compressed 20% — elements overflow, text wraps incorrectly |

### Root Cause

In `slides.rs:render_carousel_html()`:
```rust
let base_height = (base_width as f32 / spec.canvas_width as f32 * spec.canvas_height as f32).round() as u32;
```

This computes `base_height` to match the target aspect ratio, causing ALL content (text, cards, overlays, progress bars, spacing) to stretch or compress proportionally. The `slide_base()` function in `layouts.rs` uses `height:100%` which fills the entire slide, distributing flex content across the distorted height.

### Desired Behavior

- **9:16**: Content stays at 4:5 proportions (420×525). Background extends vertically to fill 9:16 (420×747). Overlays and progress bar stay within the 4:5 composition.
- **3:4**: Content stays at 4:5 proportions (420×525). Background extends vertically to fill 3:4 (420×560).
- **1:1**: Content stays at 4:5 proportions (420×525). Background extends horizontally to fill 1:1 (525×525 at base). Composition centered with BG on sides.
- **4:5**: No change — exact fit.

## Architecture

### Core Concept

Introduce a **composition layer** that is always 420×525 (the native 4:5 proportions). The slide element has the full canvas dimensions (varying by aspect ratio) and provides the background bleed. The composition is centered within the slide.

```
┌──────────────────────────────────────────────────┐
│  Slide (canvas dimensions, e.g. 420×747 for 9:16)│
│  Background fills entire slide                    │
│                                                   │
│        ┌── BG bleed (top) ──────────┐             │
│        │                            │             │
│        │  ┌── Composition ────────┐ │             │
│        │  │  420 × 525 (always)   │ │             │
│        │  │                       │ │             │
│        │  │  Content (from        │ │             │
│        │  │  slide_base)          │ │             │
│        │  │                       │ │             │
│        │  │  Overlay (brand/url)  │ │             │
│        │  │  Progress bar         │ │             │
│        │  │  Swipe arrow          │ │             │
│        │  └───────────────────────┘ │             │
│        │                            │             │
│        └── BG bleed (bottom) ───────┘             │
│                                                   │
└──────────────────────────────────────────────────┘
```

### Canvas Dimension Computation

The canvas (slide element) is the smallest rectangle at the target aspect ratio that contains the 4:5 composition:

```rust
const COMP_WIDTH: u32 = 420;   // Native composition width
const COMP_HEIGHT: u32 = 525;  // Native composition height
let native_ratio = COMP_WIDTH as f32 / COMP_HEIGHT as f32; // 0.8

if target_ratio <= native_ratio {
    // Target is TALLER than 4:5 (9:16, 3:4, 4:5)
    // Composition fills canvas width, canvas extends vertically
    canvas_width = COMP_WIDTH;
    canvas_height = (COMP_WIDTH as f32 / target_width * target_height).round();
} else {
    // Target is WIDER than 4:5 (1:1, 16:9)
    // Composition fills canvas height, canvas extends horizontally
    canvas_height = COMP_HEIGHT;
    canvas_width = (COMP_HEIGHT as f32 / target_height * target_width).round();
}
```

| Aspect Ratio | Target | Base Canvas | Composition | Bleed |
|-------------|--------|-------------|-------------|-------|
| 4:5 | 1080×1350 | 420×525 | 420×525 | None |
| 9:16 | 1080×1920 | 420×747 | 420×525 | 111px top + 111px bottom |
| 3:4 | 1080×1440 | 420×560 | 420×525 | 17px top + 18px bottom |
| 1:1 | 1080×1080 | 525×525 | 420×525 | 52px left + 53px right |

### Vectoric Scale Factor

The scale factor maps the base canvas to the target:
- **4:5**: `1080 / 420 = 2.571`
- **9:16**: `1080 / 420 = 2.571` (same — width-based)
- **3:4**: `1080 / 420 = 2.571` (same — width-based)
- **1:1**: `1080 / 525 = 2.057` (different — height-based for wider targets)

## Implementation Steps

### Step 1: Update `render_carousel_html()` canvas computation

**File:** `src/slides.rs`

Replace the current linear canvas computation with the aspect-ratio-aware composition logic:

```rust
// Replace current:
// let base_width: u32 = 420;
// let base_height = (base_width as f32 / spec.canvas_width as f32 * spec.canvas_height as f32).round() as u32;

// With:
const COMP_WIDTH: u32 = 420;
const COMP_HEIGHT: u32 = 525;
let target_w = spec.canvas_width;
let target_h = spec.canvas_height;
let target_ratio = target_w as f32 / target_h as f32;
let native_ratio = COMP_WIDTH as f32 / COMP_HEIGHT as f32;

let (base_w, base_h) = if target_ratio <= native_ratio {
    // Taller target: composition fills width
    (COMP_WIDTH, (COMP_WIDTH as f32 / target_w as f32 * target_h as f32).round() as u32)
} else {
    // Wider target: composition fills height
    ((COMP_HEIGHT as f32 / target_h as f32 * target_w as f32).round() as u32, COMP_HEIGHT)
};

let scale_factor = target_w as f32 / base_w as f32;
```

### Step 2: Add composition CSS variables

**File:** `src/slides.rs` — CSS `:root` block

Add `--composition-width` and `--composition-height` variables (always 420×525):

```css
:root {
  --slide-width: [BASE_WIDTH]px;
  --slide-height: [BASE_HEIGHT]px;
  --composition-width: 420px;
  --composition-height: 525px;
}
```

### Step 3: Add `.slide-composition` CSS class

**File:** `src/slides.rs` — CSS block

Add the composition wrapper class and update `.slide`:

```css
.slide {
  min-width: var(--slide-width);
  height: var(--slide-height);
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.slide-composition {
  width: var(--composition-width);
  height: var(--composition-height);
  position: relative;
  overflow: hidden;
  flex-shrink: 0;
}
```

Remove `display: flex; flex-direction: column;` from `.slide` — it's replaced by centering flex.

Remove `.slide.has-progress { padding-bottom: 0; }` — progress bar is now inside composition.

### Step 4: Restructure slide HTML assembly

**File:** `src/slides.rs` — `render_carousel_html()`

Wrap each slide's content in the composition container. Move overlay, progress, and arrow INSIDE the composition:

**Current:**
```html
<div class="slide slide--dark has-progress" style="">
  {slide.html}
  {overlay_html}
  {progress_html}
  {arrow_html}
</div>
```

**New:**
```html
<div class="slide slide--dark" style="">
  <div class="slide-composition">
    {slide.html}
    {overlay_html}
    {progress_html}
    {arrow_html}
  </div>
</div>
```

This is the key structural change. The `slide.html` (output of `slide_base()`) now lives inside the composition, which is always 420×525. The overlays, progress bar, and swipe arrow are also inside the composition, so they're positioned relative to the 4:5 area.

### Step 5: Update `slide_base()` background handling

**File:** `src/layouts.rs` — `slide_base()`

The background from `slide_base()` now only covers the composition area (420×525). The `.slide` element's CSS class (`.slide--dark`, `.slide--light`, etc.) already sets `background-color` on the slide element, which provides the bleed background for the extended canvas area.

No changes needed to `slide_base()` itself — the background, noise, shapes, and content all stay within the composition via `width:100%;height:100%`. The `.slide` CSS class provides the bleed background automatically.

### Step 6: Update JS carousel navigation

**File:** `src/slides.rs` — JS block

The JS `translateX` uses `[SLIDE_WIDTH]` which is replaced with `base_width`. In the new approach, `base_width` is `base_w` (which is 525 for 1:1, 420 for others):

```rust
.replace("[SLIDE_WIDTH]", &base_w.to_string())
```

This ensures the carousel track slides by the correct canvas width, not the composition width.

### Step 7: Update token generation to always use 4:5

**Files:** `src/main.rs`, `src/mcp_server.rs`

Token generation should ALWAYS use 420×525 (the native 4:5 dimensions), regardless of target aspect ratio. The tokens (spacing, font sizes, shadows, radii) are designed for the 4:5 composition and should not change.

**`src/main.rs` — `run_full_scope_test()`:**
```rust
// Replace:
// let base_width = 420;
// let base_height = (base_width as f32 / canvas.width as f32 * canvas.height as f32).round() as u32;

// With:
let base_width = 420;
let base_height = 525; // Always 4:5 for token generation
```

**`src/mcp_server.rs` — `configure_design()`:**
```rust
// Replace:
// let base_width = 420;
// let base_height = (base_width as f32 / canvas.width as f32 * canvas.height as f32).round() as u32;

// With:
let base_width = 420;
let base_height = 525; // Always 4:5 for token generation
```

**`src/mcp_server.rs` — `get_tokens()`:** Already uses `resolve_render_canvas()` which returns base dimensions. Update to always return 420×525 for the base:

```rust
// In resolve_render_canvas(), always use 4:5 base:
let base_width: u32 = 420;
let base_height: u32 = 525; // Always 4:5
```

### Step 8: Update IG frame overhead calculation

**File:** `src/slides.rs`

The IG overhead is currently hardcoded to 150px. This should remain the same since the IG frame elements (header ~56px + footer ~90px) don't change with aspect ratio.

No changes needed.

### Step 9: Update unit tests

**File:** `src/slides.rs` — `mod tests`

Update the existing test and add new tests for each aspect ratio:

```rust
#[test]
fn test_render_carousel_4_5_native() {
    // 4:5 → exact fit, no bleed
    let spec = CarouselSpec { canvas_width: 1080, canvas_height: 1350, .. };
    let html = render_carousel_html(&spec);
    assert!(html.contains("--slide-width: 420px"));
    assert!(html.contains("--slide-height: 525px"));
    assert!(html.contains("--composition-width: 420px"));
    assert!(html.contains("--composition-height: 525px"));
    assert!(html.contains("slide-composition"));
}

#[test]
fn test_render_carousel_9_16_taller() {
    // 9:16 → composition centered vertically, BG extends top/bottom
    let spec = CarouselSpec { canvas_width: 1080, canvas_height: 1920, .. };
    let html = render_carousel_html(&spec);
    assert!(html.contains("--slide-width: 420px"));
    assert!(html.contains("--slide-height: 747px"));
    assert!(html.contains("--composition-width: 420px"));
    assert!(html.contains("--composition-height: 525px"));
}

#[test]
fn test_render_carousel_1_1_wider() {
    // 1:1 → composition centered horizontally, BG extends sides
    let spec = CarouselSpec { canvas_width: 1080, canvas_height: 1080, .. };
    let html = render_carousel_html(&spec);
    assert!(html.contains("--slide-width: 525px"));
    assert!(html.contains("--slide-height: 525px"));
    assert!(html.contains("--composition-width: 420px"));
    assert!(html.contains("--composition-height: 525px"));
}

#[test]
fn test_render_carousel_3_4_slightly_taller() {
    // 3:4 → composition centered vertically, small BG extension
    let spec = CarouselSpec { canvas_width: 1080, canvas_height: 1440, .. };
    let html = render_carousel_html(&spec);
    assert!(html.contains("--slide-width: 420px"));
    assert!(html.contains("--slide-height: 560px"));
    assert!(html.contains("--composition-width: 420px"));
    assert!(html.contains("--composition-height: 525px"));
}
```

## Files to Modify

| File | Changes |
|------|---------|
| `src/slides.rs` | Canvas computation, CSS variables, `.slide-composition` class, HTML assembly, JS `[SLIDE_WIDTH]` |
| `src/layouts.rs` | No changes needed — `slide_base()` output stays inside composition |
| `src/main.rs` | Token generation always uses 420×525 |
| `src/mcp_server.rs` | Token generation always uses 420×525, `resolve_render_canvas` returns 420×525 base |
| `src/platforms.rs` | `resolve_render_canvas()` base dimensions always 420×525 |

## Files NOT Modified

| File | Reason |
|------|--------|
| `src/design_system.rs` | Token generation unchanged (already uses 420×525 defaults) |
| `src/components.rs` | `inject_background_image()` still finds `position:relative;width:100%;height:100%` inside composition — works as-is |
| `src/blocks.rs` | HTML block helpers unchanged — they generate content that goes inside `slide_base()` |
| `src/effects.rs` | Background effects, noise, shapes unchanged — they stay within the composition |
| `src/export.rs` | Screenshot dimensions come from `resolve_canvas()` which returns target dimensions — unchanged |

## Edge Cases

### 1. Background Images
`inject_background_image()` in `components.rs` injects images inside the first `position:relative;width:100%;height:100%` div found in the HTML. This is inside `slide_base()` output, which is inside the composition. Background images will only cover the 4:5 composition area, not the bleed. The bleed shows the solid background color from the `.slide` CSS class. **Acceptable for Phase 1.** Phase 2 could extend background images to the full canvas.

### 2. Split Layouts
`split_layout()` uses a grid inside `slide_base()`. The grid fills the composition (420×525). Works correctly — the two columns are proportionally correct within 4:5.

### 3. QR Destination Slide
The QR slide has a two-column grid (`grid-template-columns: 1.2fr 1fr`). Inside the 4:5 composition, this works correctly. The QR code and text are proportionally sized.

### 4. Charts and Data Visualizations
Charts render inside `slide_base()` content. They fill the composition area. Works correctly.

### 5. High Scale Factors (16:9 landscape)
For 16:9 (1920×1080): target_ratio = 1.778 > 0.8 → wider target.
- canvas_height = 525, canvas_width = 525 * 16/9 = 933
- Composition: 420×525 centered in 933×525
- Scale factor: 1920/933 = 2.058
- Content is narrow (420px) within wide canvas (933px) — lots of BG on sides. **Acceptable.**

### 6. IG Frame with Non-4:5
The IG frame elements (header, dots, actions, caption) are OUTSIDE the carousel viewport. They use `width: var(--slide-width)` which is the canvas width. For 1:1, the IG frame is 525px wide (base) → 1080px after scaling. Correct for a 1:1 Instagram post.

### 7. Carousel Track Scrolling
The JS `translateX` uses `base_w` (canvas width). For 1:1, each slide is 525px wide (base). The track scrolls by 525px per slide. The viewport is 525px wide. Correct.

## Verification

1. **Unit tests**: `cargo test` — all existing + new tests pass
2. **Full-scope test**: `cargo run -- test-full-scope` — all 24 carousels generate without errors
3. **Visual spot-check**: Open generated HTML files for each aspect ratio:
   - 4:5: Content fills canvas exactly (no change from current)
   - 9:16: Content centered vertically, BG extends top/bottom, overlays within composition
   - 1:1: Content centered horizontally, BG extends sides, no overflow
   - 3:4: Content centered vertically, small BG extension
4. **Specific checks**:
   - Text is readable and not stretched/compressed
   - Progress bar is at the bottom of the composition, not the canvas
   - Overlays (brand, topic, URL, hashtags) are at the corners of the composition
   - Cards and grids maintain proper proportions
   - Swipe arrow is within the composition
