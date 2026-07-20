---
name: story-flows
description: Use when constructing narrative and educational slide types like problem-solution, myth-fact, process maps, pricing plans, checklist action plans, and case studies.
---

# SlideForge Story & Educational Flows

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
Challenges standard assumptions to educate the user.
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
  - `plans` (array) — Array of up to 3 plan objects:
    - `name` (string, required) — Plan tier name (e.g., "Starter").
    - `price` (string, required) — Price tag (e.g., "$49/mo").
    - `description` (string, required) — Short description of the tier.
    - `icon` (string, optional) — E.g., "S", "∞".

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
- [ ] **Plan Count Limit:** Do not specify more than 3 plans in `pricing_plan`. Exceeding 3 plans results in horizontal grid breaking.

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
