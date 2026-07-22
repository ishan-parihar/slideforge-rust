# Design Spec: Pool-Based Preset Composition (Approach C)

**Date:** 2026-07-22
**Status:** Draft — pending user review
**Scope:** Preset catalog schema v5, AI-agent composition workflow, validation layer

---

## 1. Problem Statement

The current preset catalog (v4.0.0) defines **locked slide sequences** — every carousel produced from a preset follows the exact same slide-type order regardless of the content. The AI agent has zero compositional freedom: it receives a fixed sequence and fills in the blanks.

This causes:

- **17 of 45 registered slide-types are never used in any preset** (case_study_result, column_chart, comparison, faq, funnel_chart, gauge, image_collage, image_comparison, image_stat, progress_rings, quote, radar_chart, scatter_plot, split_features, stat_row, table, text_columns)
- **20 of 28 presets have zero data visualization slides** despite having dataviz types available
- **Presets produce visually homogeneous carousels** — every proof_stacking carousel looks structurally identical regardless of whether the content is a tech startup pitch or a political campaign
- **The AI agent cannot adapt composition to content depth** — a 3-slide proof vs a 7-slide proof uses the same locked sequence, just with fewer repeatable iterations

## 2. Design Goal

Presets should define **constraints and a pool of available slide-types**, not a locked sequence. The AI agent composes the carousel design *before* filling content, selecting from the pool based on the content's needs while respecting the preset's emotional arc and rhythm rules.

### Workflow (target state):

```
1. Thematic Configuration
   → User defines: topic, audience, emotional_arc, persuasion_techniques, tone

2. Preset Selection
   → System matches emotional_arc to preset(s)
   → Preset provides: hero_type, cta_type, arc_structure, SLIDE_POOL, constraints

3. Agent Composition (NEW)
   → Agent reads arc_structure + pool + constraints
   → Agent selects slide-types from pool for each arc position
   → Agent outputs: designed carousel spec (slide-types + slot assignments)
   → Validation: DLD rhythm, no consecutive duplicates, min/max slide counts

4. Content Fill
   → Agent fills params_template for each selected slide
```

## 3. Approach C: Locked Skeleton + Flexible Body

### 3.1 Arc Structure

Every preset defines an `arc_structure` with four arc positions:

| Arc Position | Role | Locking | Description |
|---|---|---|---|
| `hook` | Opening | **Locked** — hero/CTA types fixed by preset | First 1-2 slides. Sets emotional tone. |
| `evidence` | Body | **Flexible** — agent picks from pool | 2-6 middle slides. Core content. Agent composes from pool. |
| `proof` | Reinforcement | **Semi-flexible** — agent picks from subset | 1-2 slides. Validates evidence. Subset of pool. |
| `action` | Closing | **Locked** — CTA/QR types fixed by preset | Final slide. Always CTA or QR. |

The hook and CTA are the emotional bookends — they define the preset's identity and stay locked. The body (evidence + proof) is where compositional variety matters — the agent picks slide-types from the pool based on content depth, data availability, and audience.

### 3.2 Pool Definition

Each arc position defines a `pool` of allowed slide-types:

```json
{
  "arc_structure": {
    "hook": {
      "types": ["hero", "image_headline"],
      "count": {"min": 1, "max": 2}
    },
    "evidence": {
      "pool": [
        "column_chart", "comparison_bars", "metric_grid", "progress_rings",
        "radar_chart", "gauge", "scatter_plot", "funnel_chart", "stat_row",
        "table", "chart", "problem_solution", "before_after_story",
        "case_study_result", "process_map", "timeline", "myth_fact",
        "split_features", "grid_cards", "image_callout", "image_stat"
      ],
      "count": {"min": 2, "max": 6}
    },
    "proof": {
      "pool": [
        "testimonial_avatar", "logo_cloud", "case_study_result",
        "before_after_story", "comparison_bars", "stat_row"
      ],
      "count": {"min": 1, "max": 2}
    },
    "action": {
      "types": ["cta", "qr_destination"],
      "count": {"min": 1, "max": 1}
    }
  }
}
```

**Key distinction:**
- `types` (locked positions): exactly these types, no substitution
- `pool` (flexible positions): agent selects from this set

### 3.3 Constraints

```json
{
  "constraints": {
    "no_consecutive_same_type": true,
    "bg_rhythm": "alternating_dark_light",
    "max_slides": 12,
    "min_slides": 5,
    "max_consecutive_dataviz": 2,
    "require_narrative_after_dataviz": true
  }
}
```

| Constraint | Rule | Purpose |
|---|---|---|
| `no_consecutive_same_type` | Same slide_type cannot appear twice in a row | Visual variety |
| `bg_rhythm` | Alternate dark/light bg_style | DLD rhythm principle |
| `max_slides` | Total carousel cannot exceed this | Layout bounds |
| `min_slides` | Minimum slides required | Content depth |
| `max_consecutive_dataviz` | Max 2 data slides in a row | Prevent chart fatigue |
| `require_narrative_after_dataviz` | A narrative slide must follow 2+ dataviz slides | Pacing |

### 3.4 Example: proof_stacking Preset v5

**Before (v4.0.0) — locked sequence:**
```json
{
  "slides": [
    {"slide_type": "section_divider", "slot": "opening_hook"},
    {"type": "repeatable", "repeat_count": {"min": 1, "max": 3},
     "unit_slides": [
       {"slide_type": "problem_solution"},
       {"slide_type": "before_after_story"},
       {"slide_type": "case_study_result"},
       {"slide_type": "testimonial_avatar"}
     ]},
    {"slide_type": "qr_destination", "slot": "closing_cta"}
  ]
}
```

**After (v5) — pool-based composition:**
```json
{
  "arc_structure": {
    "hook": {
      "types": ["hero", "section_divider", "headline_subheadline"],
      "count": {"min": 1, "max": 1}
    },
    "evidence": {
      "pool": [
        "case_study_result", "before_after_story", "problem_solution",
        "column_chart", "comparison_bars", "stat_row", "table",
        "metric_grid", "scatter_plot", "image_callout", "image_stat"
      ],
      "count": {"min": 2, "max": 5}
    },
    "proof": {
      "pool": [
        "testimonial_avatar", "logo_cloud", "case_study_result",
        "progress_rings", "gauge"
      ],
      "count": {"min": 1, "max": 2}
    },
    "action": {
      "types": ["cta", "qr_destination"],
      "count": {"min": 1, "max": 1}
    }
  },
  "constraints": {
    "no_consecutive_same_type": true,
    "bg_rhythm": "alternating_dark_light",
    "max_slides": 9,
    "min_slides": 5,
    "max_consecutive_dataviz": 2,
    "require_narrative_after_dataviz": true
  }
}
```

The AI agent now composes:
```
hook: section_divider
evidence: [column_chart, case_study_result, comparison_bars, testimonial_avatar]
proof: [stat_row]
action: cta
```

Or for a data-heavy content:
```
hook: hero
evidence: [scatter_plot, column_chart, table, case_study_result, before_after_story]
proof: [progress_rings]
action: qr_destination
```

Same preset, different compositions, both valid.

## 4. Schema Changes

### 4.1 Preset Schema (v5)

```json
{
  "id": "string (required)",
  "name": "string (required)",
  "category": "string (required)",
  "description": "string (required)",
  "version": "5.0.0",
  "ideal_slide_count": "string (range, e.g. '5-9')",
  "rhythm": "string (human-readable arc description)",
  "emotional_arc": "string (required)",
  "persuasion_techniques": ["string (required)"],
  "arc_structure": {
    "hook": {
      "types": ["string (locked slide-types)"],
      "count": {"min": "int", "max": "int"}
    },
    "evidence": {
      "pool": ["string (flexible slide-types)"],
      "count": {"min": "int", "max": "int"}
    },
    "proof": {
      "pool": ["string (flexible slide-types)"],
      "count": {"min": "int", "max": "int"}
    },
    "action": {
      "types": ["string (locked slide-types)"],
      "count": {"min": 1, "max": 1}
    }
  },
  "constraints": {
    "no_consecutive_same_type": "bool (default: true)",
    "bg_rhythm": "string (default: 'alternating_dark_light')",
    "max_slides": "int",
    "min_slides": "int",
    "max_consecutive_dataviz": "int (default: 2)",
    "require_narrative_after_dataviz": "bool (default: true)"
  },
  "slides": ["(DEPRECATED — kept for backwards compat, ignored when arc_structure present)"]
}
```

### 4.2 Backwards Compatibility

If a preset has `arc_structure`, the agent uses pool-based composition. If it only has `slides`, the agent uses the legacy locked sequence. This allows incremental migration — presets can be converted one at a time.

### 4.3 Slide-Type Vocabulary (existing, reused)

The `_meta.slide_type_vocabulary` already categorizes all 45 types by role. The pool definitions reference these categories:

- `opening` → hook pool candidates
- `narrative` → evidence/proof pool candidates
- `data` → evidence pool candidates
- `social_proof` → proof pool candidates
- `visual` → evidence pool candidates (image-heavy content)
- `feature` → evidence pool candidates (product showcases)
- `closing` → action pool candidates

### 4.4 Dataviz Category Enforcement

The `constraints` block includes dataviz-specific rules:

- `max_consecutive_dataviz`: prevents chart fatigue (default: 2)
- `require_narrative_after_dataviz`: forces a narrative slide after 2+ dataviz slides

These constraints reference the `_meta.slide_type_vocabulary.data` list to classify which types are dataviz.

## 5. Agent Composition Protocol

### 5.1 Input to Agent

```json
{
  "preset": "proof_stacking",
  "topic": "Series A fundraise for climate-tech startup",
  "audience": "institutional investors",
  "content_depth": "5 case studies available",
  "data_available": true,
  "tone": "authoritative"
}
```

### 5.2 Agent Decision Process

1. **Read arc_structure** from preset
2. **Filter pool** by content characteristics:
   - If `data_available == true` → include dataviz types in evidence pool
   - If `data_available == false` → exclude dataviz, use narrative types
   - If `content_depth == "high"` → use max count for evidence arc
   - If `content_depth == "low"` → use min count for evidence arc
3. **Select slide-types** for each arc position from the filtered pool
4. **Apply constraints**: DLD rhythm, no consecutive duplicates, dataviz pacing
5. **Output designed spec**:
   ```json
   {
     "composition": [
       {"arc": "hook", "slide_type": "hero", "bg_style": "dark"},
       {"arc": "evidence", "slide_type": "column_chart", "bg_style": "light"},
       {"arc": "evidence", "slide_type": "case_study_result", "bg_style": "dark"},
       {"arc": "evidence", "slide_type": "comparison_bars", "bg_style": "light"},
       {"arc": "proof", "slide_type": "stat_row", "bg_style": "dark"},
       {"arc": "action", "slide_type": "cta", "bg_style": "light"}
     ]
   }
   ```

### 5.3 Validation Gate

Before content fill, the composition is validated against:

1. **Arc position counts**: each arc has min/max satisfied
2. **Pool membership**: every selected type is in the arc's pool
3. **DLD rhythm**: no consecutive same bg_style
4. **No consecutive same type**: slide_type doesn't repeat
5. **Dataviz pacing**: max 2 consecutive dataviz, narrative follows
6. **Total slide count**: within min/max bounds

Validation failures produce actionable errors:
```
ERROR: evidence arc has 1 slide, minimum is 2
ERROR: 'column_chart' appears consecutively at positions 3 and 4
ERROR: 3 consecutive dataviz slides at positions 2-4, requires narrative slide after position 4
```

## 6. Implementation Scope

### 6.1 Catalog Migration (v4 → v5)

Convert all 28 presets from locked `slides` array to `arc_structure` + `constraints`:

| Category | Presets | Conversion Complexity |
|---|---|---|
| hook_and_introduction | announcement, authority_introduction, origin_story, product_showcase | Low — simple arc |
| credibility_and_proof | proof_stacking, credibility_cascade, process_transparency | Medium — repeatable blocks → pool |
| evidence_and_argument | evidence_argument, contrast_demonstration, deep_dive | Medium — dataviz-heavy pools |
| knowledge_and_transfer | skill_transfer, principle_education, technique_mastery, wisdom_transmission | Medium — educational arcs |
| perspective_and_transformation | paradigm_shift, transformation_arc, legacy_preservation | Low — narrative-heavy |
| mobilization_and_action | urgency_mobilization, collective_mobilization, event_mobilization | Low — action-focused |
| narrative_and_emotion | story_journey, emotional_engineering, guided_experience, nostalgia_engine, outrage_catalyst, exposure_reveal, aspiration_ladder, underdog_comeback | High — diverse emotional arcs |

### 6.2 Validation Layer Updates

- `validate.rs`: add `validate_composition()` that checks arc structure, pool membership, DLD rhythm, dataviz pacing
- `mcp_server.rs`: expose `validate_composition` as MCP tool for AI agents to pre-validate before content fill
- `main.rs`: CLI `validate` subcommand accepts composition JSON

### 6.3 AI-Agent Skill Updates

- `slide-composition/SKILL.md`: update composition protocol to reference arc_structure + pool
- `data-viz/SKILL.md`: add routing guidance for when to pick dataviz types from pool
- `campaign-presets/SKILL.md` (if exists): update preset selection to include composition step

### 6.4 Generation Script Updates

- `generate_preset_slides.py`: support both legacy `slides` and new `arc_structure` modes
- `generate_full_scope_test.py`: add composition-mode test cases

## 7. Testing Strategy

### 7.1 Unit Tests

- `validate_composition()` with valid compositions → pass
- `validate_composition()` with pool violations → actionable error
- `validate_composition()` with DLD rhythm violations → actionable error
- `validate_composition()` with dataviz pacing violations → actionable error
- Backwards compat: legacy `slides` presets still work

### 7.2 Integration Tests

- Generate 28 preset carousels using pool-based composition
- Verify all generated carousels pass DLD rhythm
- Verify no preset generates a carousel with 0 dataviz when data is available
- Verify all 45 slide-types appear in at least one preset's pool

### 7.3 Visual Verification

- Regenerate all 28 preset carousels
- Inspect for structural variety (different presets produce different slide-type combinations)
- Inspect for dataviz presence in data-heavy presets
- Inspect for zero per-card/per-wrapper overflow:hidden (directive #1303)

## 8. Migration Plan

1. **Phase 1**: Add `arc_structure` + `constraints` fields to catalog schema (backwards-compatible)
2. **Phase 2**: Implement `validate_composition()` in validator
3. **Phase 3**: Convert 5 pilot presets (proof_stacking, evidence_argument, deep_dive, collective_mobilization, nostalgia_engine) — one per complexity tier
4. **Phase 4**: Verify pilot presets generate correctly
5. **Phase 5**: Convert remaining 23 presets
6. **Phase 6**: Full verification chain (cargo test → build → regen → inspect)

## 9. Open Questions

1. **Repeatable blocks in pool mode**: Should repeatable blocks be a composition option (agent can insert repeatable blocks from a sub-pool), or does pool-based composition replace repeatable blocks entirely?
   - *Recommendation*: Keep repeatable blocks as an option within the evidence arc — the agent can choose between single slides or repeatable blocks based on content depth.

2. **Params_template in pool mode**: Each slide-type has a `params_template` in the current catalog. In pool mode, the agent selects the slide-type first, then fills the template. Should the template be inline in the composition output, or referenced by type name?
   - *Recommendation*: Reference by type name. The agent looks up the template from the registry when filling content.

3. **Preset category routing**: The current 7 categories map to emotional intent. Should the pool definitions be per-preset or per-category (shared pools within a category)?
   - *Recommendation*: Per-preset. Categories guide selection; pools guide composition. Different presets in the same category should have different pools (e.g. `proof_stacking` needs case study types, `credibility_cascade` needs social proof types).

---

## Appendix A: Slide-Type Classification for Pool Routing

| Category | Types | Pool Role |
|---|---|---|
| Opening | hero, image_headline, section_divider, headline_subheadline | hook |
| Narrative | definition, text_block, problem_solution, myth_fact, before_after_story, timeline, process_map, list, checklist_action_plan | evidence, proof |
| Data Viz | chart, column_chart, scatter_plot, gauge, radar_chart, progress_rings, comparison_bars, metric_grid, funnel_chart, stat_row, table | evidence |
| Social Proof | testimonial_avatar, logo_cloud, case_study_result, faq | proof |
| Visual | image_caption, image_headline, image_quote, image_callout, image_gallery, image_collage, image_comparison, image_stat | evidence |
| Feature | feature, grid_cards, split_features, list, checklist_action_plan | evidence |
| Closing | cta, qr_destination, pricing_plan | action |

## Appendix B: Full Preset Pool Definitions (v5 Target)

*(To be populated during Phase 3-5 migration)*

Each preset gets a complete `arc_structure` + `constraints` block. The pilot presets will establish the pattern; remaining presets follow the same template.
