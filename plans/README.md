# SlideForge — Improvement Plans

Written against commit `7d58014` (worktree also carries the uncommitted per-slide
typology-variance feature from the previous session — see `git status`).

## Index

| # | Plan | Priority | Depends on | Status |
|---|------|----------|-----------|--------|
| 001 | Overflow validation gate upgrade + automatic component scaling | P0 | — | READY |

## Recommended execution order

1. **001** — single plan, phased (A→F). Phases A–C must land before D–F:
   A. Restore the runtime gate (slide-splitting regex fix) — *critical, unblocks everything*.
   B. General text-overflow estimator in `validate.rs` (shared model).
   C. Compile-time gate in `generate-slide` (CLI + MCP).
   D. Runtime post-generation gate + harness integration.
   E. Automatic component scaling (fixes the 93 overflowing slides at the source).
   F. Full verification sweep + evidence.

## Status legend

- `READY` — written and vetted, ready to execute.
- `IN PROGRESS` — an executor is working it.
- `DONE` — done criteria verified.
- `BLOCKED` / `STALE` — see the plan.
