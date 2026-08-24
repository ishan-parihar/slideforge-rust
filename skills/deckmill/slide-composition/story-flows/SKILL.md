---
name: story-flows
description: Use when constructing narrative and educational slide types like problem-solution, myth-fact, process maps, pricing plans, checklist action plans, and case studies.
---

# Deckmill Story & Educational Flows

This leaf skill guides the composition of structural story layouts. These slide types are designed to drive engagement, educate the reader, and construct a logical narrative flow leading to a call-to-action.

## Supported Slide Types & Schemas

### 1. `problem_solution` (Friction & Resolution)
Clearly contrasts a pain point with a solution.
- **Required Parameters:**
  - `problem` (string) — Description of the pain point. Max 100 chars.
  - `solution` (string) — How your product/method resolves it. Max 100 chars.
- **Optional Parameters:**
  - `title` (string) — Slide heading.
  - `proof_points` (array) — Bullet points backing up the solution:
    - `title` (string, required) — Point header.
    - `description` (string, optional) — Point context.

### 2. `myth_fact` (Myth Debunker)
Challenges standard assumptions to educate the user. **Auto-scales both ways:** sparse text (<40 chars) scales UP (+4px, roomier padding) while long text steps down through density tiers against a full stack estimate (split and debunk variants) — the old crude `>120 chars → −2px` heuristic is gone.
- **Required Parameters:**
  - `myth` (string) — The common misconception. Max 100 chars.
  - `fact` (string) — The actual truth. Max 100 chars.
- **Optional Parameters:**
  - `explanation` (string) — Short elaboration.

### 3. `case_study_result` (Success Story)
Details client outcomes.
- **Required Parameters:**
  - `client` (string) — Name of client or company.
  - `challenge` (string) — Initial state challenge.
  - `solution` (string) — Solution implemented.
  - `results` (array) — Result metrics:
    - `icon` (string) — E.g., "↗" or "✦".
    - `title` (string, required) — Big metric text (e.g. "3.1x", "42%").
    - `description` (string, required) — Metric explanation.

### 4. `pricing_plan` (Offer Stack)
Compares pricing plans side by side.
- **Required Parameters:**
  - `title` (string) — Title of pricing slide.
  - `plans` (array) — Array of **2–4** plan objects:
    - `name` (string, required) — Plan tier name (e.g., "Starter").
    - `price` (string, required) — Price tag (e.g., "$49/mo").
    - `description` (string, required) — Short description of the tier.
    - `icon` (string, optional) — E.g., "S", "∞".
- **Composition Rule:** 2 and 4 plans fill the row evenly; **3 plans render with the third tile centered horizontally** (no dangling asymmetry). Choose 2/3/4 deliberately based on the number of tiers you actually offer — 3 reads as "balanced trio", 2 as "this vs that", 4 as "full ladder".

### 5. `before_after_story` (Transformation)
Shows a clear transformation with a supporting metric.
- **Required Parameters:**
  - `before` (string) — Raw initial state text.
  - `after` (string) — Optimized final state text.
- **Optional Parameters:**
  - `title` (string) — Slide header.
  - `metric` (string) — Stat line confirming the change.

---

## Actionable Constraints & Design Rules

- [ ] **Clean Contrast:** For `myth_fact`, ensure the myth reads as negative (grayer or crossed out, automatically styled) and the fact pops clearly.
- [ ] **Proof Point Limits:** In `problem_solution`, limit proof points to 2. Adding more will cause vertical wrapping issues.
- [ ] **Plan Count Limit:** `pricing_plan` accepts 2–4 plans. Below 2 or above 4 is a validator error. With 3 plans the third tile centers automatically.

---

## Example Payload

```json
{
  "slide_type": "myth_fact",
  "params": {
    "myth": "More dashboards create better decisions.",
    "fact": "Fewer signals with clearer ownership drive progress.",
    "explanation": "Extra clutter dilutes attention."
  }
}
```
