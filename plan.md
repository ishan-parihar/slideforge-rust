# Implementation Plan: Pool-Based Preset Composition

**Spec:** `docs/superpowers/specs/2026-07-22-pool-based-preset-composition-design.md`

## Design Summary

Add pool-based composition mode to the preset system. Presets gain `arc_structure` + `constraints` fields alongside the existing `slides` array (backwards-compatible). A new `validate_composition` tool lets AI agents validate their carousel designs before content fill.

---

## Tasks

### 1. Add `validate_composition()` to validate.rs
- Depends on: none
- Files: `src/validate.rs`
- What: Add `CompositionSlide` struct, `CompositionRequest`, `validate_composition()` function
- Must NOT: modify existing `validate_slide_spec` or `validate_and_fix_slide`
- Verify: `cargo test` passes, new tests for valid/invalid compositions

### 2. Expose `validate_composition` as MCP tool
- Depends on: 1
- Files: `src/mcp_server.rs`
- What: Add `ValidateCompositionRequest` struct, `#[tool]` method on `SlideForgeServer`
- Must NOT: change existing tool signatures
- Verify: MCP server compiles, tool appears in tool list

### 3. Add `validate-composition` CLI command
- Depends on: 1
- Files: `src/main.rs`
- What: Add `ValidateComposition` variant to `Commands` enum, wire to `validate::validate_composition`
- Must NOT: change existing CLI subcommand behavior
- Verify: `cargo build --release` succeeds, `slideforge validate-composition --help` works

### 4. Add `arc_structure` + `constraints` to preset catalog
- Depends on: none (parallel with 1-3)
- Files: `docs/presets/campaign-presets.json`
- What: Add `arc_structure` and `constraints` fields to all 28 presets. Keep existing `slides` for backwards compat.
- Must NOT: remove existing `slides` arrays, change preset IDs
- Verify: JSON is valid, all 28 presets have `arc_structure` and `constraints`

### 5. Update `generate_preset_slides.py` for composition mode
- Depends on: 1, 4
- Files: `generate_preset_slides.py`
- What: Add `generate_composition(preset)` function that reads `arc_structure`, generates a sample composition, validates via `validate_composition`, then renders. Add `--mode composition` flag.
- Must NOT: break legacy `slides` mode
- Verify: `python3 generate_preset_slides.py --preset proof_stacking --mode composition` generates valid HTML

### 6. Convert 5 pilot presets to arc_structure
- Depends on: 4
- Files: `docs/presets/campaign-presets.json`
- What: proof_stacking, evidence_argument, deep_dive, collective_mobilization, nostalgia_engine
- Must NOT: change slide content or preset behavior
- Verify: JSON valid, presets generate correct HTML

### 7. Convert remaining 23 presets
- Depends on: 6
- Files: `docs/presets/campaign-presets.json`
- What: Convert all remaining presets to arc_structure
- Verify: All 28 presets generate correctly

### 8. Full verification chain
- Depends on: 1, 2, 3, 4, 5, 7
- What: `cargo test` → `cargo build --release` → copy binary → `python3 generate_preset_slides.py` → inspect dist/presets/ HTML
- Verify: Zero per-card/per-wrapper overflow:hidden, all 28 carousels render

---

## References

- Spec: `docs/superpowers/specs/2026-07-22-pool-based-preset-composition-design.md`
- Validate: `src/validate.rs:49` (validate_slide_spec), `src/validate.rs:286` (validate_and_fix_slide)
- MCP tools: `src/mcp_server.rs:1247` (validate_layout), `src/mcp_server.rs:1271` (validate_and_fix)
- CLI: `src/main.rs:66` (Commands enum), `src/main.rs:476` (ValidateLayout)
- Presets: `docs/presets/campaign-presets.json`
- Generation: `generate_preset_slides.py:75` (generate_slide), `generate_preset_slides.py:106` (render_carousel)
