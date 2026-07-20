---
name: text-layouts
description: Use when constructing text-only slides such as hero, feature highlights, lists, timelines, callouts, and calls-to-action (CTA).
---

# SlideForge Text & Layouts

This leaf skill guides the composition of text-focused slides. Text layout slides are the backbone of narrative structure and must remain clean, highly readable, and free of layout overflows.

## Supported Slide Types & Schemas

### 1. `hero` (Hook / Title Slide)
Grabs attention at the start of a carousel.
- **Required Parameters:**
  - `headline` (string) — Main bold hook statement. Max 50 chars.
- **Optional Parameters:**
  - `subheadline` (string) — Supporting copy. Max 80 chars.
  - `badge` (string) — Small upper tag (e.g., "NEW", "CHAPTER 1").
  - `variant` (string) — `"centered"` or `"left-aligned"`.

### 2. `list` (Structured Bullets)
Best for presenting details or points.
- **Required Parameters:**
  - `title` (string) — Title of the list.
  - `items` (array) — Array of objects, each containing:
    - `title` (string, required) — Bullet heading.
    - `description` (string, optional) — Short paragraph below bullet.
- **Limits:** Max 5 list items to prevent vertical overflow.

### 3. `quote` (Testimonial / Highlight Quote)
Emphasizes a key insight or client quote.
- **Required Parameters:**
  - `quote` (string) — The main quote text. Max 150 chars.
- **Optional Parameters:**
  - `author` (string) — Name of the speaker.
  - `role` (string) — Title or company of the speaker.

### 4. `cta` (Call-To-Action / Final Slide)
Converts attention at the end of the carousel.
- **Required Parameters:**
  - `headline` (string) — Conversion headline. Max 50 chars.
  - `button_text` (string) — Action button label (e.g., "Try Free").
- **Optional Parameters:**
  - `button_url` (string) — URL destination link.
  - `subtext` (string) — Micro-copy below button (e.g., "No card needed").

### 5. `timeline` (Chronological Process)
Step-by-step sequential horizontal or vertical flow.
- **Required Parameters:**
  - `title` (string) — Title of the timeline.
  - `steps` (array) — List of step objects:
    - `title` (string, required) — Step name.
    - `description` (string, required) — Step detail.
- **Limits:** Max 4 steps.

---

## Actionable Constraints & Design Rules

- [ ] **No Content Bloat:** Do not exceed character limits. SlideForge enforces hidden overflow; text will clip if it overflows the 420x525 base composition container.
- [ ] **Paragraph Line Breaks:** For headings, use `\n` to manually break lines if a word wraps awkwardly.
- [ ] **Icon Presence:** For the `callout` and `feature` slides, choose short emojis or simple Lucide icon names (e.g., "rocket", "shield", "check").
- [ ] **Text Columns Width:** If using `text_columns`, limit to 2 or 3 columns max. Any more will make text illegibly narrow.

---

## Example Payload

```json
{
  "slide_type": "hero",
  "params": {
    "headline": "Build Beautiful\nSlides with Code",
    "subheadline": "A developer-friendly composition system",
    "badge": "LAUNCH",
    "variant": "centered"
  }
}
```
