# Vectoric Carousel Scaling Implementation Plan

> **Status:** Plan — ready for implementation
> **Goal:** Replace the linear_scale variable-refactor approach with CSS `transform: scale()` vectoric scaling that takes the 420px-baseline HTML output and scales it to any target canvas size.

## Core Concept

Instead of refactoring every hardcoded pixel value in the CSS/HTML to use proportional variables (error-prone, exhausting), we:

1. **Generate all slide HTML at 420px base width** (the original, well-tested system)
2. **Then scale the entire rendered carousel** to the target canvas size using CSS `transform: scale()` with `transform-origin: top left`
3. **The scale factor** is simply `target_width / 420.0` — a single number

This preserves every pixel-perfect relationship from the original system without touching a single hardcoded value.

## Why This Works

| Problem | Variable Refactor Approach | Vectoric Scaling Approach |
|---------|---------------------------|--------------------------|
| Overlay text too small at 1080px | Refactor every `10.5px` → CSS var | Scales automatically with transform |
| Progress bar too thin | Refactor `4px` → `calc(var(--space-1) * 0.5)` | Scales automatically with transform |
| IG frame elements tiny | Refactor `32px`, `13px`, etc. → calc/var | Scales automatically with transform |
| Glass blur wrong | Refactor `blur(12px)` → var | Scales automatically with transform |
| Number of changes | 100+ touch points across 7 source files | ~5 touch points in 2 source files |

## Architecture

```
┌─────────────────────────────────────────────┐
│  Outer Container (target_width × target_height) │
│  overflow: hidden                              │
│                                                 │
│  ┌─────────────────────────────────────────┐   │
│  │  Scaled Container                       │   │
│  │  transform: scale(S)                    │   │
│  │  transform-origin: top left             │   │
│  │  width: 420px                           │   │
│  │  height: [base_height]px                │   │
│  │                                         │   │
│  │  ┌──── IG Frame ───────────────────┐   │   │
│  │  │  ┌── Carousel Viewport ──────┐   │   │   │
│  │  │  │  Slide 1 | Slide 2 | ... │   │   │   │
│  │  │  └──────────────────────────┘   │   │   │
│  │  │  ┌── IG Footer ─────────────┐   │   │   │
│  │  │  │  dots, actions, caption  │   │   │   │
│  │  │  └──────────────────────────┘   │   │   │
│  │  └─────────────────────────────────┘   │   │
│  └─────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

## Aspect Ratio Handling

The base rendering always uses **420px width**. The height is computed from the aspect ratio:

| Aspect Ratio | Base Width | Base Height | Target (e.g.) | Scale Factor |
|-------------|-----------|-------------|---------------|-------------|
| 4:5 | 420 | 525 | 1080×1350 | 2.571 |
| 1:1 | 420 | 420 | 1080×1080 | 2.571 |
| 9:16 | 420 | 747 | 1080×1920 | 2.571 |
| 3:4 | 420 | 560 | 1080×1440 | 2.571 |
| 16:9 | 420 | 236 | 1920×1080 | 4.571 |

**The scale factor is always `target_width / 420.0`.** The outer container is `target_width × target_height` and the inner scaled container is `420 × base_height`.

## Implementation Steps

### Step 1: Add `PlatformCanvas` base-render fields

**File:** `src/platforms.rs`

Add a function `resolve_render_canvas()` that returns both:
- **`base_dimensions`**: width=420, height=computed from aspect ratio
- **`target_dimensions`**: the full canvas width/height

```rust
pub struct RenderCanvas {
    pub base_width: u32,
    pub base_height: u32,
    pub target_width: u32,
    pub target_height: u32,
    pub scale_factor: f32,
    pub aspect_ratio: String,
    pub platform: String,
}

pub fn resolve_render_canvas(platform: &str, aspect_ratio: Option<&str>) -> Result<RenderCanvas, String> {
    let canvas = resolve_canvas(platform, aspect_ratio)?;
    let base_width = 420;
    let base_height = (base_width as f32 / canvas.width as f32 * canvas.height as f32).round() as u32;
    let scale_factor = canvas.width as f32 / base_width as f32;

    Ok(RenderCanvas {
        base_width,
        base_height,
        target_width: canvas.width,
        target_height: canvas.height,
        scale_factor,
        aspect_ratio: canvas.aspect_ratio,
        platform: canvas.platform,
    })
}
```

### Step 2: Update `CarouselSpec` to include base dimensions

**File:** `src/slides.rs`

Add fields:
```rust
pub struct CarouselSpec {
    // ... existing fields ...
    pub base_width: u32,     // 420
    pub base_height: u32,    // computed from aspect ratio
    pub target_width: u32,   // final output width
    pub target_height: u32,  // final output height
}
```

### Step 3: Generate CSS at base dimensions

**File:** `src/slides.rs` — `render_carousel_html()`

Generate the CSS with `--slide-width: [base_width]px` and `--slide-height: [base_height]px` instead of the target dimensions:

```rust
// Replace the current
.replace("[SLIDE_WIDTH]", &spec.canvas_width.to_string())
.replace("[SLIDE_HEIGHT]", &spec.canvas_height.to_string())

// With
.replace("[SLIDE_WIDTH]", &spec.base_width.to_string())
.replace("[SLIDE_HEIGHT]", &spec.base_height.to_string())
```

### Step 4: Wrap output in scale container

**File:** `src/slides.rs` — `render_carousel_html()`

Wrap the entire carousel output (IG frame + carousel viewport + IG footer) in a scale container, and wrap that in the outer-sized container:

```html
<div style="width: [target_width]px; height: [target_height]px; overflow: hidden; position: relative; background: #f0f0f0;">
  <div style="transform: scale([scale_factor]); transform-origin: top left; width: [base_width]px; height: [base_height]px;">
    IG_FRAME_HEADER
    <div class="carousel-viewport">
      <div class="carousel-track" id="carouselTrack">
        SLIDES
      </div>
    </div>
    IG_FRAME_FOOTER
  </div>
</div>
```

Note: The `scale_factor` must be precise enough to avoid sub-pixel artifacts. Use `format!("{:.6}", scale_factor)`.

### Step 5: Update JS to use base slide width

**File:** `src/slides.rs`

The carousel JavaScript uses `translateX(${-current * [SLIDE_WIDTH]}px)` — this should use the **base_width** (420px) because the transform handles the scaling:

```rust
// Replace
.replace("[SLIDE_WIDTH]", &spec.canvas_width.to_string())

// With
.replace("[SLIDE_WIDTH]", &spec.base_width.to_string())
```

### Step 6: Update `derive_palette_with_canvas` calls

**Files:** `src/mcp_server.rs`, `src/main.rs`, any callers

All calls to `derive_palette_with_canvas` should pass `420` as the canvas width (the base width). The height should be `420 / aspect_ratio_width * aspect_ratio_height`.

For `run_full_scope_test` in `main.rs`:
```rust
// Use base dimensions for token generation
let base_width = 420;
let base_height = (base_width as f32 / canvas.width as f32 * canvas.height as f32).round() as u32;

let tokens = design_system::derive_palette_with_canvas(
    color, "modern", 16, 1.25, "tonal_spot", theme,
    None, None, None,
    base_width, base_height,
)?;
```

### Step 7: Update `run_full_scope_test` to emit correct output

**File:** `src/main.rs`

Update the CarouselSpec to use both base and target dimensions, and generate HTML filenames that reflect the actual output canvas:

```rust
let spec = slides::CarouselSpec {
    // ... existing fields ...
    base_width,
    base_height,
    target_width: canvas.width,
    target_height: canvas.height,
    // canvas_width and canvas_height stay as target dimensions
    // for the carousel metadata / filename
};
```

### Step 8: Build and test

Run:
```bash
cargo build
cargo test
python3 test_full_scope_rust.py
```

Verify:
- Generated HTML files have the correct outer container dimensions
- The carousel viewport inside is 420px wide
- Slides render proportionally correct when viewed
- All unit tests pass

## Files to Modify

| File | Changes |
|------|---------|
| `src/platforms.rs` | Add `resolve_render_canvas()` and `RenderCanvas` struct |
| `src/slides.rs` | Add base/target dims to `CarouselSpec`, scale wrapper in `render_carousel_html()` |
| `src/main.rs` | Update `run_full_scope_test` to use base dimensions for tokens and rendering |
| `src/mcp_server.rs` | Update `derive_palette_with_canvas` calls to pass 420 base width |

## Files NOT Modified

| File | Reason |
|------|--------|
| `src/design_system.rs` | Already clean at baseline; no linear_scale refactors |
| `src/blocks.rs` | Already correct at 420px; scales via transform |
| `src/components.rs` | Already correct at 420px; scales via transform |
| `src/layouts.rs` | Already correct at 420px; scales via transform |
| `src/effects.rs` | Already correct at 420px; scales via transform |

## Edge Cases

1. **High scale factors** (e.g., 1920/420 = 4.571): CSS `transform: scale()` handles this fine. The content is sharp because we're scaling up an already-crisp 420px render.

2. **Sub-pixel rendering**: At extreme scales, some anti-aliasing artifacts may appear. A `transform: scale()` with `transform-origin: top left` avoids the worst of these. Setting `image-rendering: auto` on the container helps.

3. **Responsive viewport**: The outer container has fixed pixel dimensions. For responsive viewing, `max-width: 100%` can be added to the outer container.

4. **Touch events**: The JS handles drag/swipe using clientX coordinates which work correctly inside a scaled container because CSS transforms don't affect touch event coordinates.

5. **IG frame aspect**: The IG frame (header, actions, caption) is generated at 420px base width and scales with everything else. Its internal proportions (avatar 32px, fonts 13px, etc.) remain correct relative to the slide content.

## Verification

1. **Unit tests**: `cargo test` — all existing tests pass
2. **Integration test**: `python3 test_full_scope_rust.py` — all 47 carousels generate without errors
3. **Visual spot-check**: Open a generated HTML file and verify:
   - Outer container matches target dimensions
   - Content inside is scaled up proportionally
   - Text is readable at scale
   - Progress bar, overlay text, and IG frame are proportional
