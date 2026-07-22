# Dataviz Slide Utilization Gap — Fix Plan

**Date:** 2026-07-23
**Status:** Investigation complete, ready for implementation
**Scope:** Fix content-entry-order mismatches in `generate_preset_slides.py` so all 45 registered slide types render correctly

---

## Problem Statement

The SlideForge binary registers 45 slide types, but the `generate_preset_slides.py` script only successfully renders a subset of them. The audit found:

- **301 catalog slots** across 28 presets
- **236 rendered slides** (78% render rate)
- **65 missing slides** (22% gap)
- **15+ slide types completely unused** in rendered output: `comparison`, `faq`, `funnel_chart`, `image_collage`, `image_comparison`, `image_stat`, `pricing_plan`, `progress_rings`, `quote`, `radar_chart`, `scatter_plot`, `split_features`, `table`, `text_columns`

The user observation: "We have a lot of various kinds of dataviz slides, but why the fuck are we only utilizing this very boring single-tile metrics-slides?"

---

## Root Cause

The `preset_content()` function in `generate_preset_slides.py` has content entries in the **WRONG ORDER** relative to the catalog's expanded slide sequence. The consumption loop (lines 1009-1048) takes entries sequentially — one per expanded slot position. When the order doesn't match, each slide gets the wrong content, and the validator rejects it for missing required params.

### Example: `aspiration_ladder`

**Catalog expanded sequence (7 slots):**
```
[0] image_headline
[1] metric_grid
[2] image_headline
[3] process_map
[4] testimonial_avatar
[5] progress_rings
[6] qr_destination
```

**Content entries (7 entries, wrong order):**
```
[0] headline, image_url, subheadline    → matches image_headline ✓
[1] headline, image_url, subheadline    → should be title, metrics ✗
[2] quote, author, role, avatar_url     → should be headline, image_url, subheadline ✗
[3] title, metrics                       → should be title, steps ✗
[4] title, steps                         → should be quote, author, role, avatar_url ✗
[5] title, rings                         → should be title, items ✗ (param name mismatch)
[6] destination_url, cta_text, ...      → matches qr_destination ✓
```

**Result:** Only 2 of 7 slides render. The other 5 fail validation because the content entry at that position doesn't have the required params for the catalog slide type at that position.

### Secondary Issue: Param Name Mismatches

Even when content entries exist for the right slide type, some use wrong param names:
- `progress_rings` expects `items` but content uses `rings`
- `comparison` expects `columns, rows` but content may use different keys
- `image_comparison` expects `before_image, after_image` but content may use `before, after`

---

## Affected Presets

18 presets have unused types in their catalog that fail to render:

| Preset | Unused Types |
|--------|-------------|
| aspiration_ladder | progress_rings |
| proof_stacking | progress_rings |
| product_showcase | pricing_plan, image_collage |
| evidence_argument | table |
| contrast_demonstration | image_comparison, comparison |
| skill_transfer | faq |
| deep_dive | split_features |
| principle_education | faq |
| technique_mastery | radar_chart |
| wisdom_transmission | text_columns |
| legacy_preservation | quote |
| paradigm_shift | scatter_plot |
| event_mobilization | image_collage |
| urgency_mobilization | image_stat |
| collective_mobilization | image_stat |
| emotional_engineering | funnel_chart |
| nostalgia_engine | quote |

---

## Fix Strategy

### Approach: Reorder Content Entries to Match Catalog

For each preset, reorder the content entries in `preset_content()` to match the catalog's expanded slide sequence. Fix any param name mismatches.

**Why this approach:**
- The catalog is the source of truth for slide composition
- The content entries are the source of truth for content
- The consumption loop is correct — it takes one entry per slot
- The bug is in the content entry ordering, not the loop logic

### Alternative Approaches Considered

1. **Rewrite `preset_content()` from scratch** — Too disruptive, loses existing content quality
2. **Add a mapping layer** — Adds complexity without solving the root cause
3. **Make `fill_params()` smarter** — Doesn't fix the order issue, just masks it

---

## Tasks

### Task 1: Audit All 28 Presets for Content-Order Mismatches

**Depends on:** none
**Files:** `generate_preset_slides.py`
**What:** For each preset, compare catalog expanded sequence vs content entry keys. Identify all mismatches.
**Must NOT:** Modify any files yet — this is read-only audit.
**Verify:** Run audit script, output mismatch count per preset.

```python
# Pseudocode for audit
for preset in catalog['presets']:
    catalog_seq = expand_slides(preset['slides'])  # with repeatable at max
    content_keys = get_content_keys(preset_content(preset))
    for i, (slide_type, keys) in enumerate(zip(catalog_seq, content_keys)):
        required = slide_info(slide_type).required_params
        missing = required - keys
        if missing:
            report(preset, i, slide_type, missing)
```

### Task 2: Fix Content Entry Order for All 28 Presets

**Depends on:** Task 1
**Files:** `generate_preset_slides.py`
**What:** Reorder content entries to match catalog sequence. Fix param name mismatches (`rings` → `items`, etc.).
**Must NOT:** Change the catalog slide sequence — only the content entries.
**Verify:** Run audit script again, expect 0 mismatches.

### Task 3: Fix Param Name Mismatches

**Depends on:** Task 2
**Files:** `generate_preset_slides.py`
**What:** For each content entry, ensure param names match the slide type's required params. Common fixes:
- `progress_rings`: `rings` → `items`
- `comparison`: use `columns, rows` instead of generic keys
- `image_comparison`: `before, after` → `before_image, after_image`
- `image_stat`: ensure `image_url, stat_value, stat_label` present
- `faq`: `items` → `questions`
- `table`: ensure `headers, rows` present

**Verify:** Run `slideforge generate-slide` for each fixed type, expect success.

### Task 4: Regenerate All 28 Presets and Verify Render Rate

**Depends on:** Task 3
**Files:** `dist/presets/*.html`, `dist/presets/tokens/*.json`
**What:** Run full verification chain:
1. `cargo test` — expect 94/94 pass
2. `cargo build --release` — expect success
3. Copy binary to `dist/slideforge-x86_64-linux-gnu`
4. `python3 generate_preset_slides.py` — expect 28/28 generate
5. Count rendered slides — expect ~301 (was 236)
6. Verify all 45 types appear in rendered output

**Verify:** Render rate ≥ 95% (was 78%). All 45 types used at least once.

### Task 5: Update SKILL.md with Dataviz Type Coverage

**Depends on:** Task 4
**Files:** `skill/slideforge/SKILL.md`
**What:** Document that all 45 slide types are now utilized. Add note about content-entry-order requirement.
**Verify:** SKILL.md mentions all 45 types and the content-order requirement.

---

## Verification Chain

Per AGENTS.md directive #1505/#1537/#1538:

```bash
# 1. Unit tests
cargo test

# 2. Release build
cargo build --release

# 3. Copy binary
cp target/release/slideforge-rust dist/slideforge-x86_64-linux-gnu

# 4. Regenerate all presets
python3 generate_preset_slides.py

# 5. Verify render rate
python3 -c "
import json, os
from collections import Counter
types = Counter()
for f in os.listdir('dist/presets/tokens'):
    if f.endswith('.json') and not f.startswith('_'):
        data = json.load(open(f'dist/presets/tokens/{f}'))
        items = data if isinstance(data, list) else data.get('slides', [])
        for item in items:
            t = item.get('slide_type') or item.get('type')
            if t: types[t] += 1
print(f'Rendered: {sum(types.values())} slides')
print(f'Types used: {len(types)}')
print(f'Type coverage: {sorted(types.keys())}')
"

# 6. Verify no overflow:hidden regressions
grep -c 'overflow:hidden' dist/presets/*.html
```

---

## Success Criteria

1. **All 28 presets generate successfully** (no validation failures)
2. **Render rate ≥ 95%** (was 78%)
3. **All 45 slide types appear** in rendered output (was 30)
4. **Zero content-entry-order mismatches** in audit
5. **Zero param-name mismatches** in audit
6. **All unit tests pass** (94/94)
7. **No overflow:hidden regressions** in live HTML

---

## Risk Assessment

**Low risk:**
- Reordering content entries doesn't change the catalog
- Param name fixes are localized to specific entries
- Verification chain catches regressions

**Medium risk:**
- Some presets may need content rewritten if existing entries don't fit the catalog sequence
- Param name mismatches may require looking up slide-info for each type

**Mitigation:**
- Run audit before and after each change
- Use `--only` flag to test one preset at a time
- Full verification chain after all changes
