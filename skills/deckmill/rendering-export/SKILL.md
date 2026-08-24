---
name: rendering-export
description: Use when assembling individual slide JSON components into a rendered HTML carousel document and exporting them to PNG images.
---

# Deckmill Rendering & Export Pipeline

This leaf skill guides the final stage of carousel creation: combining slides into a single HTML document and rendering them as PNG images using the embedded Blitz renderer (stylo layout + vello-cpu raster, no browser needed).

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
deckmill render-carousel slides.json \
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
deckmill export carousel.html \
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

## Image Sources (Remote vs Embedded)

- Remote `http(s)` image URLs (e.g. Pexels CDN `https://images.pexels.com/...`) are fetched natively by blitz-net during export — they render without any pre-processing.
- For **offline-deterministic** decks, inline images as `data:image/...;base64,...` URIs first (`stock-image --embed` for stock, `embed-image` for local files). Embedded decks render identically with the network disabled.
- Attribution data returned by `stock-image` (photographer, page URL) should be credited on the closing slide.

## Deterministic Font Loading (Critical)

Deckmill vendors Google Fonts so exported text is **pixel-identical on every run**, with or without network access:

- On the first export, the renderer fetches each Google Fonts CSS2 stylesheet (Chrome user-agent), rewrites every `@font-face` remote `url()` to an inline **`data:font/woff2;base64,…`** URL, and stores the processed stylesheet in the on-disk cache (`$DECKMILL_FONT_CACHE`, default `~/.cache/deckmill/fonts`).
- Cache entries are keyed by the stylesheet URL **plus a cache-version prefix**; bumping the version (e.g. after the latin-subset collapse / weight-range merge fixes) invalidates stale files so old glyph-race payloads are never served verbatim.
- Glyph determinism: only the `latin` subset `@font-face` (covering U+0000–00FF, which includes ASCII digits) is kept per (family, weight, style); latin-ext/cyrillic subsets that silently lack ASCII digits are collapsed away so numbers and letters always shape from the intended face.
- **Offline behavior:** cached faces are reused from disk with no network; an uncached family falls back to a local system face rather than failing the export. `DECKMILL_FONT_CACHE` can point anywhere writable (CI caches, tmpfs).

## Renderer Capability Caveats (Blitz / stylo)

- No Chromium: PNG export runs in-process (stylo layout + vello-cpu raster). The `setup` command is a no-op. There is no headless-chrome subprocess to install or download.
- **No `:has()` selector** — stylo does not implement it. Author custom HTML/CSS with explicit marker classes, never `:has()`. (Deckmill's own bleed/overlay CSS uses marker classes for this reason.)
- CSS grid items keep `min-width:auto` semantics: any unbreakable nowrap child (badges, tags) can inflate a `1fr` column. Use `minmax(0,1fr)` tracks plus `min-width:0; overflow:hidden` on grid items in custom layouts.

## Actionable Constraints & Design Rules

- [ ] **Renderer:** PNG export uses the embedded Blitz renderer — no Chromium install or download is required (the `setup` command is a no-op).
- [ ] **Overlay Matching:** Ensure the `--brand-name` and `--brand-handle` parameters passed to `render-carousel` match the configurations in your design tokens.
- [ ] **Slide Count Count:** Always pass the exact number of slides using the `--slides` parameter during export. Specifying an incorrect slide count will cause rendering errors or blank output pages.
- [ ] **Fonts:** Do not hand-edit font links in carousel HTML. Run `export` once to warm the font cache; reuse `DECKMILL_FONT_CACHE` across CI runs for deterministic output.

---

## Aspect Ratio Fit & Background Bleed Mechanics

To design slides effectively, you must understand how Deckmill scales layouts across different platforms:

1. **Base Composition Canvas:**
   All slide layouts are designed and composed inside a fixed **4:5 aspect ratio coordinate space (420px width × 525px height)**.
2. **Export Fitting (Fit-to-Canvas):**
   When exporting slides to a target preset (like `instagram_story` 9:16 or `instagram_square` 1:1), Deckmill does **not** recompose or stretch the layout dimensions. The core 4:5 content box fits in the center of the target canvas.
3. **Background Bleed:**
   The background colors, gradients, and decorative background shapes (e.g. textures or glow meshes) bleed outward to fill the remainder of the target canvas bounds. 
4. **Overlay Positioning:**
   Persistent header/footer meta elements (brand handle, topic name, progress indicators, page numbers) adjust and pin themselves to the outer margins of the final canvas, rather than the core 4:5 bounding box.
5. **Aesthetic Rule for Designers:**
   Avoid applying absolute positional elements that assume the final resolution height/width inside custom slide HTML/CSS. Design styling, font scaling, and container layout rules must remain relative to the 420x525 base size.

