# Cognitive Kernel - Intent Mapping

## Deliverable
A hierarchical meta-skill suite under `/home/ishanp/.agents/skills/slideforge` for using the SlideForge CLI/MCP tool to create professional, aesthetically beautiful slides. The meta-skill must follow the `meta-skill-creator` architecture (recursive router-leaf structure) and be validated by the registry script.

## Constraints & Requirements
1. **Design Quality (WOW factor):** Enable any agent (even a "stupid" one) to act as a professional designer and produce gorgeous carousels. Must establish clear design principles (OKLCH, themes, archetypes, background styles, color presets).
2. **Hierarchy:** Must not be a single flat file. Decompose into logical routers and leaves.
3. **Registry & Map Validation:** Must include a registry script (adapted from `scripts/registry.py`) and generate the `_map.md` and validation outputs.
4. **No Nuance / Ambiguity:** Instructions must be clear, actionable, pass/fail checklists, no generic "consider/be careful" guidelines.
5. **Latest Binary Exploration:** We must build and explore the actual capabilities of the compiled binary (`slideforge-rust`) to ensure all options are correct and documented.

## Scope Fences
- **In-Scope:** Creating the skill folder structure, individual `SKILL.md` router and leaf files, supporting frameworks/reference documents, copying/running registry/validation script, testing.
- **Out-of-Scope:** Fixing bugs in the Rust binary itself (unless blocking execution, but our focus is creating the skill guide).
