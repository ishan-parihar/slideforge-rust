---
name: text-layouts
description: Use when constructing text-only slides such as hero, quotes, split feature grids, definitions, text blocks, section dividers, and timelines.
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

### 2. `split_features` (Feature Beat / Icon Grid)
Highlights a single feature beat OR multiple feature cards (icon-grid variant).
- **Required Parameters:**
  - `title` (string) — Section title.
  - `features` (array) — Each feature contains:
    - `title` (string, required) — Feature heading.
    - `description` (string, optional) — Short paragraph.
    - `icon` (string, optional) — Emoji or Lucide icon name.
- **Limits:** Max 6 features; icon-grid variant maxes at 4 columns.

### 3. `quote` (Testimonial / Highlight Quote)
Emphasizes a key insight or client quote.
- **Required Parameters:**
  - `quote` (string) — The main quote text. Max 150 chars.
- **Optional Parameters:**
  - `author` (string) — Name of the speaker.
  - `role` (string) — Title or company of the speaker.

### 4. `timeline` (Chronological Process)
Step-by-step sequential horizontal or vertical flow.
- **Required Parameters:**
  - `title` (string) — Title of the timeline.
  - `steps` (array) — List of step objects:
    - `title` (string, required) — Step name.
    - `description` (string, required) — Step detail.
- **Limits:** Max 6 steps.

### 5. `definition` (Term Glossary)
Educational terms or glossary entries.
- **Required Parameters:**
  - `term` (string) — Term being defined.
  - `definition` (string) — Definition body.
- **Optional Parameters:**
  - `context` (string) — Example or contextual note.
  - `phonetic` (string) — Pronunciation guide.

### 6. `text_block` (Paragraph Content)
Simple paragraph content.
- **Required Parameters:**
  - `title` (string) — Title of the block.
  - `body` (string) — Body text.
- **Optional Parameters:**
  - `variant` (string) — Visual variant.

### 7. `section_divider` (Chapter Opener)
Slide deck chapter openers.
- **Required Parameters:**
  - `title` (string) — Chapter title.
- **Optional Parameters:**
  - `kicker` (string) — Small upper tag.
  - `subtitle` (string) — Subtitle.

---

## Actionable Constraints & Design Rules

- [ ] **No Content Bloat:** Do not exceed character limits. The validator surfaces overflow as a compile-time error rather than silently clipping text.
- [ ] **Paragraph Line Breaks:** For headings, use `\n` to manually break lines if a word wraps awkwardly.
- [ ] **Icon Presence:** For `split_features` slides, choose short emojis or simple Lucide icon names (e.g., "rocket", "shield", "check").

---

## Example Payload

```json
{
  "slide_type": "split_features",
  "params": {
    "title": "Built for production",
    "features": [
      {"icon": "bolt", "title": "Fast", "description": "<10ms per slide"},
      {"icon": "shield", "title": "Validated", "description": "Compile-time overflow check"}
    ]
  }
}
```
