---
name: image-integration
description: Use when integrating photography, illustrations, or graphics into slide templates like image_caption, image_headline, galleries, and comparisons.
---

# SlideForge Image Integration

This leaf skill guides the inclusion and rendering of images inside carousels. Because SlideForge compiles to HTML and is screenshot via headless Chromium, image loading and layouts require strict rules.

## Local Image Data URI Conversion (Critical)

Headless Chrome cannot consistently resolve relative file paths in all system contexts.
- **Rule:** Never reference local file paths (like `./assets/photo.png`) in your image parameters.
- **Workflow:** You must convert the local image into a Base64 Data URI first using `embed-image` (CLI) or `embed_local_image` (MCP). Then use that Data URI in your parameter payload.

### CLI Conversion Example
```bash
slideforge-rust embed-image ./photo.png
# Output will print a JSON string containing "data_uri": "data:image/png;base64,..."
```

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

---

## Styling Configurations

You can pass these style overrides via CLI `--override` or slide metadata:
- **Filters:** `none`, `grayscale`, `sepia`, `duotone-warm`, `duotone-cool`, `high-contrast`, `soft`, `vintage`.
- **Overlays:** `none`, `gradient`, `solid`, `duotone`, `vignette`, `tint`.
- **Frames:** `sharp` (0px), `rounded` (8px), `squircle` (16px), `organic` (asymmetric border radius).

---

## Actionable Constraints & Design Rules

- [ ] **Image Pre-compression:** High-resolution images (>5MB) slow down headless Chrome rendering significantly. Ensure Base64 source files are compressed before conversion.
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
