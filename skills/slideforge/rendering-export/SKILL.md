---
name: rendering-export
description: Use when assembling individual slide JSON components into a rendered HTML carousel document and exporting them to PNG images.
---

# SlideForge Rendering & Export Pipeline

This leaf skill guides the final stage of carousel creation: combining slides into a single HTML document and rendering them as PNG images using headless Chromium.

## The Assembly & Render Pipeline

To produce a multi-slide carousel, you must follow this exact 3-step compilation sequence:

### Step 1: Generate Individual Slide Files
Call `generate-slide` (CLI) or `generate_slide` (MCP) for each slide in your deck, saving the output of each to a separate JSON file (e.g., `s1.json`, `s2.json`, `s3.json`).

### Step 2: Combine into a JSON Array
Concatenate the individual slide JSON outputs into a single JSON array file.
```bash
echo "[$(cat s1.json),$(cat s2.json),$(cat s3.json)]" > slides.json
```

### Step 3: Render the Carousel
Compile the slides array into the final HTML presentation document using `render-carousel` (CLI) or `render_carousel` (MCP). Pass in the session tokens to ensure all styles apply.

```bash
slideforge-rust render-carousel slides.json \
  --tokens-file tokens.json \
  --brand-name "Acme Corp" \
  --brand-handle "@acmecorp" \
  --topic "Productivity" \
  --output carousel.html
```

---

## Exporting to PNGs

To convert the compiled HTML document into individual image files for posting, run the `export` command (CLI) or `export_carousel_slides` (MCP).

```bash
slideforge-rust export carousel.html \
  --output-dir ./exports \
  --slides 3 \
  --preset instagram_portrait
```

### Platform Resolutions Reference
Choose the correct platform preset for exporting. Height and width are strictly governed by the preset:

| Preset | Aspect Ratio | Dimensions | Usage |
|---|---|---|---|
| `instagram_portrait` | 4:5 | 1080 × 1350 | Recommended Feed Carousel |
| `instagram_square` | 1:1 | 1080 × 1080 | Square feed posts |
| `instagram_story` | 9:16 | 1080 × 1920 | Stories and Reels slide shows |
| `tiktok_vertical` | 9:16 | 1080 × 1920 | TikTok image collections |
| `linkedin_landscape` | ~1.9:1 | 1200 × 627 | LinkedIn document slide attachments |
| `twitter_card` | 16:9 | 1200 × 675 | Twitter inline images |
| `presentation_16_9` | 16:9 | 1920 × 1080 | Widescreen slide decks |

---

## Actionable Constraints & Design Rules

- [ ] **Chromium Check:** Ensure Chromium is installed. In offline or CI/CD environments, run `slideforge-rust setup` first to download Chromium locally.
- [ ] **Overlay Matching:** Ensure the `--brand-name` and `--brand-handle` parameters passed to `render-carousel` match the configurations in your design tokens.
- [ ] **Slide Count Count:** Always pass the exact number of slides using the `--slides` parameter during export. Specifying an incorrect slide count will cause rendering errors or blank output pages.

---

## Aspect Ratio Fit & Background Bleed Mechanics

To design slides effectively, you must understand how SlideForge scales layouts across different platforms:

1. **Base Composition Canvas:**
   All slide layouts are designed and composed inside a fixed **4:5 aspect ratio coordinate space (420px width × 525px height)**.
2. **Export Fitting (Fit-to-Canvas):**
   When exporting slides to a target preset (like `instagram_story` 9:16 or `instagram_square` 1:1), SlideForge does **not** recompose or stretch the layout dimensions. The core 4:5 content box fits in the center of the target canvas.
3. **Background Bleed:**
   The background colors, gradients, and decorative background shapes (e.g. textures or glow meshes) bleed outward to fill the remainder of the target canvas bounds. 
4. **Overlay Positioning:**
   Persistent header/footer meta elements (brand handle, topic name, progress indicators, page numbers) adjust and pin themselves to the outer margins of the final canvas, rather than the core 4:5 bounding box.
5. **Aesthetic Rule for Designers:**
   Avoid applying absolute positional elements that assume the final resolution height/width inside custom slide HTML/CSS. Design styling, font scaling, and container layout rules must remain relative to the 420x525 base size.

