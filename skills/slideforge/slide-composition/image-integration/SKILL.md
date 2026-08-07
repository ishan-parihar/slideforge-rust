---
name: image-integration
description: Use when integrating photography, illustrations, or graphics into slide templates like image_caption, image_headline, galleries, and comparisons.
---

# SlideForge Image Integration

This leaf skill guides the inclusion and rendering of images inside carousels. SlideForge compiles to HTML and rasterizes via the embedded Blitz renderer (stylo layout + vello-cpu), so image loading and layouts require strict rules.

## Local Image Data URI Conversion (Critical)

The Blitz renderer cannot resolve relative file paths across system contexts.
- **Rule:** Never reference local file paths (like `./assets/photo.png`) in your image parameters.
- **Workflow:** You must convert the local image into a Base64 Data URI first using `embed-image` (CLI) or `embed_local_image` (MCP). Then use that Data URI in your parameter payload.

### CLI Conversion Example
```bash
slideforge-rust embed-image ./photo.png
# Output will print a JSON string containing "data_uri": "data:image/png;base64,..."
```

---

## Stock Photography via Pexels (Recommended Default)

For decks without a provided image set, use the `stock-image` command (CLI) or `stock_image` tool (MCP) to source photography automatically — **this is the default image strategy for image slides** unless the brief provides art direction.

### CLI
```bash
# Search — returns results as JSON with src variants + attribution
slideforge-rust stock-image "dark portrait" --orientation portrait --count 3

# Embed — inlines the TOP result as a data: URI (count is ignored with --embed)
slideforge-rust stock-image "dark portrait" --orientation portrait --embed
```

- **Key:** read from `PEXELS_API_KEY` env (free at pexels.com/api, ~200 req/hr). Missing key → clean error pointing to the signup page; no half-failure.
- **`--embed`:** downloads the top result and inlines it as a base64 `data:` URI — fully offline-deterministic decks (no network needed at render/export). `--embed` always returns exactly one image; call it once per image slot.
- **Without `--embed`:** prints Pexels CDN `https://images.pexels.com/...` URLs — blitz fetches `http(s)` image sub-resources natively, so remote URLs render in exports too.
- **Orientation:** `portrait` (default, 4:5/instagram), `landscape` (16:9/linkedin), `square` (1:1). `best_url()` maps orientation → correct src crop.
- **Attribution:** the returned `photographer` and `page_url` should be credited on the closing slide per Pexels license.

### MCP
`stock_image` accepts `query`, `orientation`, `count`, `embed` — identical semantics. Response mirrors the CLI JSON: `photo {id, alt, photographer, page_url, portrait, landscape, large2x, original}`, plus `url` (or `data_uri` when embedded).

### Golden Rule
Never hardcode a bare Pexels API key into slide params, scripts, or the deck JSON. Always resolve through `PEXELS_API_KEY` at call time. Keep attribution data (photographer/page_url) available for the closing slide.

---

## Supported Slide Types & Schemas

### 1. `image_caption` (Image with Context)
Shows an image with descriptive text.
- **Required Parameters:**
  - `image_url` (string) — Public URL or Base64 Data URI.
  - `caption` (string) — Small title.
- **Optional Parameters:**
  - `description` (string) — Elaboration body.
  - `layout` (string) — `"image-top"`, `"image-bottom"`, `"image-left"`, or `"image-right"`.

### 2. `image_headline` (Poster Style)
A full-width poster background image with large heading overlay.
- **Required Parameters:**
  - `image_url` (string) — Public URL or Base64 Data URI.
  - `headline` (string) — Large overlay text. Max 50 chars.
- **Optional Parameters:**
  - `subheadline` (string) — Smaller text.
  - `overlay_position` (string) — `"top"`, `"center"`, or `"bottom"`.

### 3. `image_comparison` (Side-by-side Images)
Contrasts two screenshots or photos (e.g. before vs after).
- **Required Parameters:**
  - `before_image` (string) — URL/Data URI.
  - `after_image` (string) — URL/Data URI.
- **Optional Parameters:**
  - `before_label` (string) — Defaults to "Before".
  - `after_label` (string) — Defaults to "After".
  - `divider_style` (string) — `"solid"`, `"dashed"`, or `"arrow"`.

### 4. `image_quote` (Quote over Image)
Quote text overlaid on a background image.
- **Required Parameters:**
  - `image_url` (string) — Public URL or Base64 Data URI.
  - `quote` (string) — Quote text.
- **Optional Parameters:**
  - `author`, `role` (string) — Attribution.

### 5. `image_callout` (Image with Hot-Spot Callouts)
Image with labeled callout pins.
- **Required Parameters:**
  - `image_url` (string) — Public URL or Base64 Data URI.
  - `callouts` (array) — Each callout: `{label, x, y}`.
- **Optional Parameters:**
  - `description` (string) — Description body.
  - `variant` (string) — Visual variant.

### 6. `image_gallery` (Image Grid)
Grid of 2-6 images.
- **Required Parameters:**
  - `images` (array) — Array of `{url}` or URL strings (2-6 entries).
- **Optional Parameters:**
  - `layout` (string) — Grid layout (e.g. `"4-grid"`, `"2-grid"`).
  - `title` (string) — Gallery title.
  - `section_caption` (string) — Caption block.

### 7. `image_collage` (Creative Collection)
Free-form scattered collection of images.
- **Required Parameters:**
  - `images` (array) — Array of `{url}` (2-6 entries).
  - `title` (string) — Collage title.
- **Optional Parameters:**
  - `style` (string) — Visual style variant.
  - `section_caption` (string) — Caption block.

---

## Styling Configurations

You can pass these style overrides via CLI `--override` or slide metadata:
- **Filters:** `none`, `grayscale`, `sepia`, `duotone-warm`, `duotone-cool`, `high-contrast`, `soft`, `vintage`.
- **Overlays:** `none`, `gradient`, `solid`, `duotone`, `vignette`, `tint`.
- **Frames:** `sharp` (0px), `rounded` (8px), `squircle` (16px), `organic` (asymmetric border radius).

---

## Actionable Constraints & Design Rules

- [ ] **Image Pre-compression:** High-resolution images (>5MB) slow down rendering significantly. Ensure Base64 source files are compressed before conversion.
- [ ] **Valid URL Scheme:** `image_url` must start with `http://`, `https://`, or `data:image/`.
- [ ] **Data URI Sanitization:** When generating payload files, verify that the data URI string has no extra spaces or newline breaks inside the JSON object.

---

## Example Payload

```json
{
  "slide_type": "image_headline",
  "params": {
    "image_url": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=",
    "headline": "Unlock Your Performance",
    "subheadline": "The journey starts here.",
    "overlay_position": "bottom"
  }
}
```
