# Agentic UX, QR Destination, and Aspect Ratio Upgrade Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Upgrade SlideForge so AI agents can choose the right slide type, platform, aspect ratio, and conversion slide reliably, with first-class QR destination slides and aspect-ratio-aware rendering/testing.

**Architecture:** Add a small platform canvas/context layer and pass it through MCP, CLI export, carousel rendering, and tests. Add one reusable `qr_destination` slide type in the existing registry/dispatch pattern, backed by local SVG QR generation and layout variants. Improve agent UX by exposing platform defaults, allowed ratios, context recommendations, and validation warnings through existing MCP/CLI tools.

**Tech Stack:** Rust 2024, rmcp, clap, serde/schemars, existing HTML/CSS renderer, existing validation/test harness. Add one QR dependency only if needed: prefer `qrcode = "0.14"` with SVG rendering.

## Global Constraints

- Preserve existing MCP tool names and keep existing request fields backward compatible.
- `platform` remains the primary preset; `aspect_ratio` is an explicit optional override.
- Platform defaults: Instagram feed carousel `4:5`, LinkedIn carousel/document `4:5`, Instagram/Facebook stories `9:16`, Facebook/Instagram new portrait `3:4`, square explicit `1:1`.
- Required supported aspect ratios: `4:5`, `9:16`, `3:4`, `1:1`.
- QR slide type is one reusable slide family: `qr_destination`, not separate blog/donation/product slide types.
- QR slide required params: `destination_url`, `cta_text`; `heading` is optional but recommended.
- QR slide optional params: `heading`, `caption`, `short_url`, `brand_name`, `brand_logo`, `incentive_text`, `qr_alt_text`, `variant`, `background_image`, `image_opacity`, `padding`.
- QR variants: `theme-bg`, `image-bg`, `minimal`, `with-heading`, `without-heading`, `with-caption`, `with-cta`, `full-conversion`.
- Full-scope testing must keep manual review effective: one slide per slide type, with styling/ratio/platform variations spread across types.
- Do not rewrite existing slide generators unless needed to pass aspect-ratio validation.

---

## File Map

- Modify `Cargo.toml`: add QR code dependency if local QR SVG generation is implemented with a crate.
- Modify `src/platforms.rs`: define supported aspect ratios, default ratio per platform, allowed ratios, and canvas dimensions.
- Modify `src/mcp_server.rs`: store and expose `aspect_ratio`, include platform context in configure/render/export responses, and accept explicit ratio overrides.
- Modify `src/slides.rs`: render carousel dimensions from platform canvas context instead of hard-coded `420x525`.
- Modify `src/export.rs`: keep export pixel size aligned with selected ratio/platform.
- Modify `src/main.rs`: expose ratio in `list-platforms`, `export`, and `test-full-scope`.
- Modify `src/slide_registry.rs`: add `qr_destination` and improve context metadata for conversion use cases.
- Modify `src/components.rs`: add QR SVG rendering and `qr_destination_slide`, then dispatch it.
- Modify `src/validate.rs`: add QR-specific and aspect-ratio-aware checks.
- Modify `test_full_scope_rust.py`: include `qr_destination`, spread aspect ratios across generated cases, and assert coverage.
- Modify `README.md`: document agent workflow, platform/ratio behavior, QR slide params, and full-scope testing.

---

### Task 1: Add Platform Canvas Context

**Files:**
- Modify: `src/platforms.rs`
- Modify: `src/mcp_server.rs`
- Modify: `src/main.rs`
- Test: `src/platforms.rs`

**Interfaces:**
- Produces: `AspectRatioSpec { ratio: String, width: u32, height: u32, format: String }`
- Produces: `resolve_canvas(platform: &str, aspect_ratio: Option<&str>) -> PlatformCanvas`
- Produces: `PlatformSpec.default_aspect_ratio: String`
- Produces: `PlatformSpec.allowed_aspect_ratios: Vec<String>`
- Consumes later: render/export/test code uses `PlatformCanvas.width`, `PlatformCanvas.height`, and `PlatformCanvas.aspect_ratio`.

- [ ] **Step 1: Add failing platform tests**

Add tests in `src/platforms.rs`:

```rust
#[test]
fn test_platform_defaults_and_allowed_ratios() {
    let ig = get_platform("instagram_portrait").expect("instagram portrait platform");
    assert_eq!(ig.default_aspect_ratio, "4:5");
    assert!(ig.allowed_aspect_ratios.contains(&"4:5".to_string()));
    assert!(ig.allowed_aspect_ratios.contains(&"3:4".to_string()));
    assert!(ig.allowed_aspect_ratios.contains(&"1:1".to_string()));

    let story = get_platform("instagram_story").expect("instagram story platform");
    assert_eq!(story.default_aspect_ratio, "9:16");
    assert!(story.allowed_aspect_ratios.contains(&"9:16".to_string()));

    let linkedin = get_platform("linkedin_landscape").expect("linkedin platform");
    assert_eq!(linkedin.default_aspect_ratio, "4:5");
    assert!(linkedin.allowed_aspect_ratios.contains(&"4:5".to_string()));
}

#[test]
fn test_resolve_canvas_ratio_override() {
    let canvas = resolve_canvas("instagram_portrait", Some("3:4")).expect("3:4 canvas");
    assert_eq!(canvas.aspect_ratio, "3:4");
    assert_eq!((canvas.width, canvas.height), (1080, 1440));

    let square = resolve_canvas("instagram_portrait", Some("1:1")).expect("1:1 canvas");
    assert_eq!((square.width, square.height), (1080, 1080));
}

#[test]
fn test_resolve_canvas_rejects_invalid_ratio_for_platform() {
    let err = resolve_canvas("instagram_story", Some("16:9")).unwrap_err();
    assert!(err.contains("not allowed"));
}
```

- [ ] **Step 2: Run failing tests**

Run: `cargo test platforms::tests::test_platform_defaults_and_allowed_ratios platforms::tests::test_resolve_canvas_ratio_override platforms::tests::test_resolve_canvas_rejects_invalid_ratio_for_platform`

Expected: FAIL because new fields/functions do not exist.

- [ ] **Step 3: Implement platform context**

In `src/platforms.rs`, update `PlatformSpec` and add `PlatformCanvas`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PlatformSpec {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: String,
    pub default_aspect_ratio: String,
    pub allowed_aspect_ratios: Vec<String>,
    pub format: String,
    pub description: String,
    pub recommended_for: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PlatformCanvas {
    pub platform: String,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: String,
    pub format: String,
}
```

Add helpers:

```rust
pub fn aspect_ratio_dimensions(ratio: &str) -> Option<(u32, u32, String)> {
    match ratio {
        "4:5" => Some((1080, 1350, "portrait".to_string())),
        "9:16" => Some((1080, 1920, "portrait".to_string())),
        "3:4" => Some((1080, 1440, "portrait".to_string())),
        "1:1" => Some((1080, 1080, "square".to_string())),
        "16:9" => Some((1920, 1080, "landscape".to_string())),
        "4:3" => Some((1024, 768, "landscape".to_string())),
        _ => None,
    }
}

pub fn resolve_canvas(platform: &str, aspect_ratio: Option<&str>) -> Result<PlatformCanvas, String> {
    let spec = get_platform(platform).ok_or_else(|| format!("Unknown platform: {platform}"))?;
    let ratio = aspect_ratio
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&spec.default_aspect_ratio);

    if !spec.allowed_aspect_ratios.iter().any(|r| r == ratio) {
        return Err(format!(
            "Aspect ratio '{ratio}' is not allowed for platform '{}'. Allowed: {}",
            spec.name,
            spec.allowed_aspect_ratios.join(", ")
        ));
    }

    let (width, height, format) = aspect_ratio_dimensions(ratio)
        .unwrap_or((spec.width, spec.height, spec.format.clone()));

    Ok(PlatformCanvas {
        platform: spec.name,
        width,
        height,
        aspect_ratio: ratio.to_string(),
        format,
    })
}
```

Use these locked platform defaults:

```rust
instagram_portrait: default 4:5, allowed ["4:5", "3:4", "1:1"]
instagram_square: default 1:1, allowed ["1:1", "4:5"]
instagram_story: default 9:16, allowed ["9:16", "3:4"]
tiktok_vertical: default 9:16, allowed ["9:16"]
linkedin_landscape: default 4:5, allowed ["4:5", "1:1"]
twitter_card: default 1:1, allowed ["1:1", "4:5"]
facebook_post: default 3:4, allowed ["3:4", "4:5", "1:1"]
presentation_16_9: default 16:9, allowed ["16:9"]
presentation_4_3: default 4:3, allowed ["4:3"]
```

- [ ] **Step 4: Surface platform context in CLI and MCP**

In `src/mcp_server.rs`, add `aspect_ratio: String` to `ServerState`, `ConfigureDesignRequest`, `ConfigureDesignResponse`, `RenderCarouselRequest`, and `ExportCarouselSlidesRequest`.

In `configure_design`, resolve:

```rust
let platform = req.platform.clone().unwrap_or_else(|| "instagram_portrait".to_string());
let canvas = platforms::resolve_canvas(&platform, req.aspect_ratio.as_deref())
    .map_err(|e| ErrorData::invalid_request(e, None))?;
state.platform = canvas.platform.clone();
state.aspect_ratio = canvas.aspect_ratio.clone();
```

Return `aspect_ratio` in `ConfigureDesignResponse`.

In `list_platforms`, include:

```rust
"default_aspect_ratio": p.default_aspect_ratio,
"allowed_aspect_ratios": p.allowed_aspect_ratios,
```

In `src/main.rs`, update `ListPlatforms` output to show default and allowed ratios.

- [ ] **Step 5: Verify**

Run: `cargo test platforms`

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/platforms.rs src/mcp_server.rs src/main.rs
git commit -m "feat: add platform aspect ratio context"
```

---

### Task 2: Make Carousel Rendering and Export Ratio-Aware

**Files:**
- Modify: `src/slides.rs`
- Modify: `src/mcp_server.rs`
- Modify: `src/export.rs`
- Modify: `src/main.rs`
- Test: `src/slides.rs`

**Interfaces:**
- Consumes: `platforms::resolve_canvas`.
- Produces: `CarouselSpec.canvas_width: u32`, `canvas_height: u32`, `aspect_ratio: String`, `platform: String`.
- Produces: HTML CSS variables `--slide-width`, `--slide-height`.

- [ ] **Step 1: Add failing rendering tests**

In `src/slides.rs`, add tests:

```rust
#[test]
fn test_render_carousel_uses_canvas_dimensions() {
    let spec = CarouselSpec {
        slides: vec![SlideSpec {
            html: "<div>hello</div>".to_string(),
            background: "light".to_string(),
            variant: "test".to_string(),
            theme: "minimal".to_string(),
            archetype: "educator".to_string(),
        }],
        css_variables: ":root { --primary:#000; }".to_string(),
        google_fonts_url: String::new(),
        heading_font: "Inter".to_string(),
        body_font: "Inter".to_string(),
        brand_name: "Brand".to_string(),
        brand_handle: "@brand".to_string(),
        topic: "Topic".to_string(),
        url: "https://example.com".to_string(),
        hashtags: vec![],
        show_progress: false,
        visual_theme: "minimal".to_string(),
        include_ig_frame: false,
        platform: "instagram_portrait".to_string(),
        aspect_ratio: "3:4".to_string(),
        canvas_width: 360,
        canvas_height: 480,
    };

    let html = render_carousel_html(&spec);
    assert!(html.contains("--slide-width: 360px"));
    assert!(html.contains("--slide-height: 480px"));
    assert!(html.contains("width: var(--slide-width)"));
    assert!(html.contains("height: var(--slide-height)"));
}
```

- [ ] **Step 2: Run failing test**

Run: `cargo test slides::tests::test_render_carousel_uses_canvas_dimensions`

Expected: FAIL because `CarouselSpec` lacks canvas fields.

- [ ] **Step 3: Implement ratio-aware CSS**

In `src/slides.rs`, extend `CarouselSpec`:

```rust
pub platform: String,
pub aspect_ratio: String,
pub canvas_width: u32,
pub canvas_height: u32,
```

Replace hard-coded CSS dimensions with variables:

```css
:root {
  --slide-width: [SLIDE_WIDTH]px;
  --slide-height: [SLIDE_HEIGHT]px;
}
.carousel-viewport { width: var(--slide-width); height: var(--slide-height); overflow: hidden; position: relative; }
.slide { min-width: var(--slide-width); height: var(--slide-height); position: relative; }
.ig-frame { width: var(--slide-width); }
```

In `render_carousel_html`, replace placeholders:

```rust
css_block = css_block
    .replace("[SLIDE_WIDTH]", &spec.canvas_width.to_string())
    .replace("[SLIDE_HEIGHT]", &spec.canvas_height.to_string());
```

Use a preview scale of `360x450`, `360x640`, `360x480`, `420x420` in tests/HTML preview if needed, but export must use actual platform pixels.

- [ ] **Step 4: Wire MCP render/export**

In `render_carousel`, resolve canvas from request or state:

```rust
let platform = req.platform.clone().unwrap_or_else(|| state.platform.clone());
let aspect_ratio = req.aspect_ratio.clone().filter(|s| !s.is_empty()).or_else(|| Some(state.aspect_ratio.clone()));
let canvas = platforms::resolve_canvas(&platform, aspect_ratio.as_deref())
    .map_err(|e| ErrorData::invalid_request(e, None))?;
```

Pass `canvas_width`, `canvas_height`, `platform`, `aspect_ratio` to `CarouselSpec`.

In `export_carousel_slides`, resolve canvas the same way and export at `canvas.width x canvas.height`.

In CLI `Export`, add:

```rust
#[arg(long)]
aspect_ratio: Option<String>,
```

Then call `platforms::resolve_canvas(preset, aspect_ratio.as_deref())`.

- [ ] **Step 5: Verify**

Run:

```bash
cargo test slides platforms
cargo build
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/slides.rs src/mcp_server.rs src/export.rs src/main.rs
git commit -m "feat: render and export aspect-ratio-aware carousels"
```

---

### Task 3: Add `qr_destination` Slide Type

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/slide_registry.rs`
- Modify: `src/components.rs`
- Modify: `src/validate.rs`
- Test: `src/slide_registry.rs`
- Test: `src/validate.rs`

**Interfaces:**
- Produces slide type: `qr_destination`.
- Produces function: `qr_destination_slide(tokens, destination_url, heading, caption, cta_text, short_url, incentive_text, variant, bg_style, background_image, image_opacity, theme, archetype, padding) -> Value`.
- Produces helper: `render_qr_svg_data_uri(destination_url: &str) -> Result<String, String>`.

- [ ] **Step 1: Add dependency**

In `Cargo.toml`, add:

```toml
qrcode = "0.14"
```

- [ ] **Step 2: Add failing registry test**

In `src/slide_registry.rs`, extend `test_registry_has_all_types` with `"qr_destination"`.

Add:

```rust
#[test]
fn test_qr_destination_registry_metadata() {
    let info = get_slide_type_info("qr_destination").expect("qr_destination exists");
    assert_eq!(info["layout_family"], "conversion");
    assert!(info["best_for"].as_array().unwrap().iter().any(|v| v == "conversion"));
    assert!(info["variants"].as_array().unwrap().iter().any(|v| v == "full-conversion"));
}
```

- [ ] **Step 3: Add registry entry**

In `src/slide_registry.rs`, add:

```rust
"qr_destination": {
    "description": "Conversion slide with scannable QR code, heading, caption, CTA, and optional short URL fallback",
    "required_params": ["destination_url", "cta_text"],
    "optional_params": ["heading", "caption", "short_url", "brand_name", "brand_logo", "incentive_text", "qr_alt_text", "variant", "background_image", "image_opacity", "padding"],
    "variants": ["theme-bg", "image-bg", "minimal", "with-heading", "without-heading", "with-caption", "with-cta", "full-conversion"],
    "default_variant": "full-conversion",
    "layout_family": "conversion",
    "best_for": ["conversion", "closing", "off-platform", "blog", "donation", "digital-product", "newsletter", "link-hub"]
}
```

- [ ] **Step 4: Add failing validation tests**

In `src/validate.rs`, add:

```rust
#[test]
fn test_qr_destination_requires_url_and_cta() {
    let params = json!({"heading": "Read the full guide"});
    let r = validate_slide_spec("qr_destination", &params);
    assert!(!r.valid);
    assert!(r.errors.iter().any(|e| e.contains("destination_url")));
    assert!(r.errors.iter().any(|e| e.contains("cta_text")));
}

#[test]
fn test_qr_destination_warns_without_heading_or_caption() {
    let params = json!({
        "destination_url": "https://example.com/guide",
        "cta_text": "Scan to read"
    });
    let r = validate_slide_spec("qr_destination", &params);
    assert!(r.valid);
    assert!(r.warnings.iter().any(|w| w.contains("heading")));
}
```

Implement warning in `validate_slide_spec`:

```rust
if slide_type == "qr_destination" {
    let has_heading = params.get("heading").and_then(|v| v.as_str()).map(|s| !s.trim().is_empty()).unwrap_or(false);
    let has_caption = params.get("caption").and_then(|v| v.as_str()).map(|s| !s.trim().is_empty()).unwrap_or(false);
    if !has_heading && !has_caption {
        result.add_warning("qr_destination should include heading or caption so users know why to scan.");
    }
}
```

- [ ] **Step 5: Implement QR rendering**

In `src/components.rs`, import:

```rust
use qrcode::QrCode;
use qrcode::render::svg;
```

Add helper near image helpers:

```rust
fn render_qr_svg_data_uri(destination_url: &str) -> Result<String, String> {
    let code = QrCode::new(destination_url.as_bytes())
        .map_err(|e| format!("Failed to generate QR code: {e}"))?;
    let svg = code.render::<svg::Color>()
        .min_dimensions(256, 256)
        .dark_color(svg::Color("#0B0A0F"))
        .light_color(svg::Color("#FFFFFF"))
        .build();
    let encoded = svg
        .replace('#', "%23")
        .replace('<', "%3C")
        .replace('>', "%3E")
        .replace('"', "'");
    Ok(format!("data:image/svg+xml;utf8,{encoded}"))
}
```

Add `qr_destination_slide` using existing blocks and style helpers. Layout rules:

- QR block must be visually dominant and at least `184px` in preview HTML.
- QR image always sits on a white quiet-zone surface.
- CTA text must be adjacent to QR, not hidden in the IG caption.
- `short_url` renders below QR as fallback.
- `image-bg` uses `background_image`; other variants use theme/background only.
- `without-heading` omits heading but keeps CTA and QR.
- `minimal` uses centered QR + CTA + short URL.
- `full-conversion` uses heading, caption, incentive line, QR, CTA, and short URL in a balanced two-zone layout.

Implementation skeleton:

```rust
pub fn qr_destination_slide(
    tokens: &DesignTokens,
    destination_url: &str,
    heading: &str,
    caption: &str,
    cta_text: &str,
    short_url: &str,
    incentive_text: &str,
    variant: &str,
    bg_style: &str,
    background_image: &str,
    image_opacity: f32,
    theme: &str,
    archetype: &str,
    padding: &str,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let effective_variant = if variant.is_empty() { "full-conversion" } else { variant };
    let qr_src = render_qr_svg_data_uri(destination_url).unwrap_or_default();
    let radius = current_component_radius(tokens, "card");
    let qr_size = if matches!(effective_variant, "minimal" | "without-heading") { "208px" } else { "188px" };

    let qr_html = format!(
        r#"<div style="background:#FFFFFF;border:1px solid rgba(0,0,0,0.08);border-radius:{};padding:16px;display:inline-flex;flex-direction:column;align-items:center;gap:10px;box-shadow:0 14px 34px rgba(0,0,0,0.14);">
            <img src="{}" alt="{}" style="width:{};height:{};display:block;" />
            {}
        </div>"#,
        radius,
        qr_src,
        escape_html(if cta_text.is_empty() { "Scan QR code" } else { cta_text }),
        qr_size,
        qr_size,
        if !short_url.is_empty() {
            format!(r#"<div style="font-family:{};font-size:11px;font-weight:700;color:#0B0A0F;max-width:{};overflow-wrap:anywhere;text-align:center;">{}</div>"#, tokens.body_font, qr_size, escape_html(short_url))
        } else {
            String::new()
        }
    );

    // Build text column and layout here using existing heading_block/text_block.
    // Keep all text outside the QR white surface except short_url.
}
```

Return JSON with `"variant": effective_variant`.

- [ ] **Step 6: Dispatch slide**

In `dispatch_slide`, add:

```rust
"qr_destination" => Ok(qr_destination_slide(
    tokens,
    &s("destination_url").if_empty(&s("url")),
    &s("heading").if_empty(&s("headline")),
    &s("caption").if_empty(&s("description")),
    &s("cta_text").if_empty(&s("button_text").if_empty("Scan to open")),
    &s("short_url"),
    &s("incentive_text"),
    &s("variant").if_empty("full-conversion"),
    bg_style,
    &bg_img,
    img_opacity,
    theme,
    _archetype,
    &s("padding"),
)),
```

- [ ] **Step 7: Verify**

Run:

```bash
cargo test slide_registry validate
cargo build
```

Expected: PASS.

- [ ] **Step 8: Commit**

```bash
git add Cargo.toml Cargo.lock src/slide_registry.rs src/components.rs src/validate.rs
git commit -m "feat: add qr destination slide type"
```

---

### Task 4: Improve Agentic Tool UX and Context Recommendations

**Files:**
- Modify: `src/mcp_server.rs`
- Modify: `src/slide_registry.rs`
- Modify: `src/validate.rs`
- Modify: `README.md`
- Test: `src/slide_registry.rs`

**Interfaces:**
- Produces context recommendation for `"conversion"` and `"off-platform"`.
- Produces MCP responses that include `platform_context`.
- Produces docs that tell agents the proper workflow.

- [ ] **Step 1: Add context tests**

In `src/slide_registry.rs`, add:

```rust
#[test]
fn test_get_slide_types_for_context_conversion_includes_qr() {
    let types = get_slide_types_for_context("conversion");
    assert!(types.contains(&"qr_destination".to_string()));
    assert!(types.contains(&"cta".to_string()));
}

#[test]
fn test_get_slide_types_for_context_off_platform_prefers_qr() {
    let types = get_slide_types_for_context("off-platform");
    assert_eq!(types.first().map(|s| s.as_str()), Some("qr_destination"));
}
```

- [ ] **Step 2: Make recommendations agent-useful**

Update `get_slide_types_for_context` so:

- exact context matches remain supported;
- for `"conversion"`, return `["qr_destination", "cta", "pricing_plan"]` first when present;
- for `"off-platform"`, return `["qr_destination", "cta"]`;
- for unknown contexts, keep current behavior of returning an empty list.

- [ ] **Step 3: Add `platform_context` to MCP responses**

Add a helper in `src/mcp_server.rs`:

```rust
fn platform_context_json(platform: &str, aspect_ratio: &str) -> serde_json::Value {
    match platforms::resolve_canvas(platform, Some(aspect_ratio)) {
        Ok(canvas) => serde_json::json!({
            "platform": canvas.platform,
            "aspect_ratio": canvas.aspect_ratio,
            "width": canvas.width,
            "height": canvas.height,
            "format": canvas.format,
            "agent_guidance": "Use qr_destination for off-platform actions because Instagram, Facebook, TikTok, and LinkedIn carousel images do not support clickable slide areas."
        }),
        Err(_) => serde_json::json!({
            "platform": platform,
            "aspect_ratio": aspect_ratio,
            "agent_guidance": "Resolve platform context with list_platforms before generating slides."
        }),
    }
}
```

Include `platform_context` in:

- `configure_design` response
- `render_carousel` response
- `list_platforms` response per platform
- `get_slide_type_info("qr_destination")` can remain registry-only unless easy to add examples.

- [ ] **Step 4: Update validation guidance**

In `validate_slide_spec`, for `qr_destination`:

- warn when `short_url` is absent;
- warn when `cta_text` is longer than 34 characters;
- warn when `destination_url` does not start with `http://` or `https://`.

Exact warnings:

```rust
"qr_destination should include short_url as a manual fallback for users who cannot scan."
"qr_destination cta_text should be 34 characters or fewer for slide readability."
"qr_destination destination_url should be an absolute http(s) URL."
```

- [ ] **Step 5: Document the AI agent workflow**

In `README.md`, add a section `Agent Workflow`:

```markdown
1. Call `list_platforms` and choose `platform` plus `aspect_ratio`.
2. Call `configure_design` with `platform`, `aspect_ratio`, brand, topic, URL, and hashtags.
3. Call `get_slide_types_for_context` for each narrative job.
4. Use `qr_destination` for off-platform actions such as blog posts, donations, digital products, newsletters, or link hubs.
5. Call `validate_layout` before generation and `validate_design` after rendering.
6. Export with the same `platform` and `aspect_ratio` used during render.
```

- [ ] **Step 6: Verify**

Run:

```bash
cargo test slide_registry validate
cargo build
```

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src/mcp_server.rs src/slide_registry.rs src/validate.rs README.md
git commit -m "feat: improve agent slide creation workflow"
```

---

### Task 5: Expand Full-Scope Tests Across Slide Types and Ratios

**Files:**
- Modify: `test_full_scope_rust.py`
- Modify: `src/main.rs`
- Test: `test_full_scope_rust.py`

**Interfaces:**
- Consumes: `aspect_ratio` field in MCP `configure_design`, `render_carousel`, and `export_carousel_slides`.
- Produces test coverage over `4:5`, `9:16`, `3:4`, `1:1`.

- [ ] **Step 1: Add QR fixture**

In `test_full_scope_rust.py`, add:

```python
"qr_destination": [
    {
        "destination_url": "https://nexusai.io/blog/agentic-slide-workflows",
        "heading": "Read the full workflow",
        "caption": "A practical guide to turning carousel attention into owned traffic.",
        "cta_text": "Scan to read",
        "short_url": "nexusai.io/guide",
        "incentive_text": "Includes templates and examples.",
        "variant": "full-conversion",
    }
],
```

Add `qr_destination` to the generated type set.

- [ ] **Step 2: Add ratio spread**

In `test_full_scope_rust.py`, add:

```python
ASPECT_RATIOS = ["4:5", "9:16", "3:4", "1:1"]
PLATFORM_RATIO_OVERRIDES = {
    "instagram_portrait": ["4:5", "3:4", "1:1"],
    "instagram_square": ["1:1", "4:5"],
    "instagram_story": ["9:16", "3:4"],
    "tiktok_vertical": ["9:16"],
    "linkedin_landscape": ["4:5", "1:1"],
}
```

When building each config:

```python
allowed = PLATFORM_RATIO_OVERRIDES.get(platform, ["4:5"])
aspect_ratio = allowed[i % len(allowed)]
```

Pass `aspect_ratio` to `configure_design` and `render_carousel`.

- [ ] **Step 3: Add coverage assertions**

Track `ratio_coverage` and print:

```python
results["ratio_coverage"].add(aspect_ratio)
```

After run:

```python
missing_ratios = set(["4:5", "9:16", "3:4", "1:1"]) - results["ratio_coverage"]
if missing_ratios:
    results["failed"] += 1
    results["errors"].append(f"Missing aspect ratios: {', '.join(sorted(missing_ratios))}")
```

- [ ] **Step 4: Update Rust native `test-full-scope`**

In `src/main.rs`, add `qr_destination` to native slide type coverage and use `platforms::resolve_canvas` in the same spread pattern.

Print platform and aspect ratio:

```rust
println!(
    "[{}/{}] Testing {}, platform: {}, ratio: {}, theme: {}",
    carousel_id, total, archetype, platform, aspect_ratio, theme
);
```

- [ ] **Step 5: Verify**

Run:

```bash
cargo build --release
python3 test_full_scope_rust.py
./target/release/slideforge-rust test-full-scope --output-dir /tmp/slideforge-ratio-scope
```

Expected:

- Python test reports all slide types covered, including `qr_destination`.
- Ratio coverage includes `4:5`, `9:16`, `3:4`, `1:1`.
- Native full-scope succeeds with no failed carousels.

- [ ] **Step 6: Commit**

```bash
git add test_full_scope_rust.py src/main.rs
git commit -m "test: expand full scope across aspect ratios"
```

---

### Task 6: Final Verification, Release, and VPS Refresh

**Files:**
- Modify: `README.md` if final command output reveals doc drift.
- Build artifact: `dist/slideforge-x86_64-unknown-linux-musl`

**Interfaces:**
- Produces updated binary with QR/ratiο support.
- Produces Git tag/release after all tests pass.

- [ ] **Step 1: Run local quality gate**

Run:

```bash
cargo fmt --check
cargo test
cargo build --release
cargo build --release --target x86_64-unknown-linux-musl
python3 test_full_scope_rust.py
```

Expected: all pass.

- [ ] **Step 2: Refresh dist artifact**

Run:

```bash
mkdir -p dist
cp target/x86_64-unknown-linux-musl/release/slideforge-rust dist/slideforge-x86_64-unknown-linux-musl
chmod +x dist/slideforge-x86_64-unknown-linux-musl
```

- [ ] **Step 3: Smoke-test MCP**

Run an MCP `tools/list` smoke test using the existing pattern from prior work. Expected tools still include:

```text
configure_design
generate_slide
render_carousel
export_carousel_slides
validate_layout
validate_design
list_platforms
get_slide_types_for_context
```

Also call:

```json
{"name":"get_slide_type_info","arguments":{"slide_type":"qr_destination"}}
```

Expected: returns QR metadata.

- [ ] **Step 4: Commit final docs/artifact**

```bash
git add README.md dist/slideforge-x86_64-unknown-linux-musl
git commit -m "chore: release qr and aspect ratio upgrade"
```

- [ ] **Step 5: Push and release**

Use the authenticated local `gh` setup:

```bash
git push origin master
git tag v0.2.0
git push origin v0.2.0
gh release create v0.2.0 dist/slideforge-x86_64-unknown-linux-musl --title "slideforge-rust v0.2.0" --notes "Adds QR destination slides, platform aspect-ratio context, and expanded full-scope coverage."
```

- [ ] **Step 6: Install on VPS and restart Hermes**

Use the existing VPS helper:

```bash
scp dist/slideforge-x86_64-unknown-linux-musl nerd@racknerd:/home/nerd/.local/bin/slideforge
/home/ishanp/ssh-racknerd.sh 'chmod +x /home/nerd/.local/bin/slideforge && systemctl --user daemon-reload && systemctl --user restart hermes-gateway && systemctl --user is-active hermes-gateway'
```

Expected: `active`.

- [ ] **Step 7: VPS verification**

Run:

```bash
/home/ishanp/ssh-racknerd.sh '/home/nerd/.local/bin/slideforge list-platforms'
/home/ishanp/ssh-racknerd.sh '/home/nerd/.local/bin/slideforge test-full-scope --output-dir /tmp/slideforge-vps-full-scope'
```

Expected:

- `list-platforms` shows default and allowed aspect ratios.
- Full-scope completes with `Total Failed: 0`.

---

## Acceptance Criteria

- `list_platforms` exposes default ratio, allowed ratios, dimensions, recommended usage, and agent guidance.
- `configure_design`, `render_carousel`, and `export_carousel_slides` accept and preserve `aspect_ratio`.
- Rendered HTML no longer hard-codes `420x525`; it uses canvas dimensions derived from platform/ratio.
- `qr_destination` appears in `list_slide_types` and `get_slide_type_info`.
- `get_slide_types_for_context("off-platform")` recommends `qr_destination` first.
- QR slides render with a scannable white quiet-zone QR, clear CTA, optional short URL, and aesthetic variants.
- Full-scope tests cover every slide type once and spread `4:5`, `9:16`, `3:4`, and `1:1`.
- Local and VPS CLI/MCP smoke tests pass.

## Execution Notes

- Keep each task committed separately.
- Do not refactor all slide layouts at once; fix only ratio-related breakage discovered by tests.
- If `qrcode` dependency fails to build on musl, replace it with a small internal QR SVG generator only as a last resort. The preferred path is the crate because QR encoding correctness matters.
- If old HTML fixtures become noisy, regenerate only the Rust full-scope output directory after tests pass.
