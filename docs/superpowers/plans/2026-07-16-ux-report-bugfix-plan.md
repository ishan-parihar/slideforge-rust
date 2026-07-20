# SlideForge UX Report Bugfix & Polish Plan

> Source: `/home/ishanp/Downloads/Yukigram/slideforge_ux_report.md`
> Status: Draft — awaiting approval before execution

---

## Executive Summary

The UX report identifies SlideForge as a **v0.8 product with v1.0-quality output**. The rendering engine is production-grade, but the CLI/MCP interface has sharp edges that break agentic workflows. This plan addresses every issue from the report, organized by severity and grouped into implementation phases that respect the AGENTS.md constraint: targeted repairs inside existing systems, not redesigns.

---

## Issue Inventory (mapped to code)

### CRITICAL — Breaks core workflows

| # | Issue | Root Cause | Files |
|---|-------|-----------|-------|
| C1 | **MCP stateless**: every `generate_slide` → `render_carousel` call spawns a fresh server; design tokens lost between calls | `run_server()` creates `Server::new()` per invocation; no session persistence | `mcp_server.rs` (ServerState, run_server) |
| C2 | **`render-carousel` CLI `--slides` flag**: report says positional arg required but code shows `slides_file: String` is positional — this is actually correct by design, but the MCP `render_carousel` tool accepts inline `slides: Vec<Value>` while CLI requires a file. Inconsistency causes agent confusion | MCP `RenderCarouselRequest` has `slides: Vec<Value>`, CLI `RenderCarousel` has `slides_file: String` | `mcp_server.rs` (RenderCarouselRequest), `main.rs` (Commands::RenderCarousel) |
| C3 | **`export` hard-fails without Chromium**: no fallback, no helpful error | `export_slides()` calls `Browser::new()` directly; error propagates as opaque Chrome error | `export.rs` (export_slides, render_html_to_png), `main.rs` (Commands::Export) |

### HIGH — Degrades agentic accuracy

| # | Issue | Root Cause | Files |
|---|-------|-----------|-------|
| H1 | **No `--help` per slide type**: agent can't discover param schema without calling MCP tool | `slide-info` command exists but `generate-slide` doesn't suggest it; no inline param examples | `main.rs` (Commands::GenerateSlide) |
| H2 | **Validation is fire-and-forget**: `validate_slide_spec` runs but errors are only embedded in output JSON; `cli_generate_slide` always renders regardless; exit code always 0 | `cli_generate_slide` line 582-606: validates, attaches to output, but never checks `validation.errors` to abort or set exit code | `main.rs` (cli_generate_slide:582-606), `mcp_server.rs` (generate_slide:702-711) |
| H3 | **MCP `generate_slide` same fire-and-forget**: validation errors embedded in response but `dispatch_slide` still called with missing params → renders empty HTML | `mcp_server.rs` generate_slide:702-721: validates, then calls `dispatch_slide` unconditionally | `mcp_server.rs` (generate_slide:702-721) |

### MEDIUM — Reduces UX quality

| # | Issue | Root Cause | Files |
|---|-------|-----------|-------|
| M1 | **Design token override opaque**: can't override a single token without regenerating full set | No `--override-token` flag; `derive_palette_with_canvas` is all-or-nothing | `main.rs`, `design_system.rs` |
| M2 | **Error messages leak Rust types**: `reqwest::Error`, `headless_chrome::...` in MCP responses | `.map_err(|e| e.to_string())` propagates internal error types | `export.rs`, `mcp_server.rs` |

### LOW — Polish

| # | Issue | Root Cause | Files |
|---|-------|-----------|-------|
| L1 | **Carousel HTML not self-contained**: Google Fonts `<link>` requires network | Hardcoded `google_fonts_url` in CarouselSpec | `slides.rs` (render_carousel_html) |
| L2 | **No param schema examples in registry**: registry has `required_params`/`optional_params` but no example values | `slide_registry.rs` has no `examples` field | `slide_registry.rs` |

---

## Implementation Phases

### Phase 1: Silent-failure kill (H2 + H3) — 1-2 hours

**Goal:** Stop rendering slides when required params are missing. Surface validation errors as real failures, not JSON decoration.

**1a. `cli_generate_slide` — abort on validation errors** (`main.rs:582-606`)

Current code:
```rust
let validation = validate::validate_slide_spec(&slide_type, &params_json);
// ... continues to dispatch_slide regardless
```

Fix:
```rust
let validation = validate::validate_slide_spec(&slide_type, &params_json);
if !validation.errors.is_empty() {
    // Print validation result as structured JSON and exit non-zero
    let response = serde_json::json!({
        "success": false,
        "validation": {
            "errors": validation.errors,
            "warnings": validation.warnings,
        },
        "hint": "Call 'slide-info <slide_type>' to see required params."
    });
    eprintln!("{}", serde_json::to_string_pretty(&response)?);
    std::process::exit(1);
}
```

**1b. MCP `generate_slide` — block on validation errors** (`mcp_server.rs:702-721`)

Current code validates but calls `dispatch_slide` unconditionally. Fix: return validation error as `ErrorData::invalid_request` when errors exist, before calling `dispatch_slide`.

```rust
let validation = validate::validate_slide_spec(&slide_type, &params);
if !validation.errors.is_empty() {
    return Err(ErrorData::invalid_request(
        format!(
            "Missing required params for '{}': {}. Call get_slide_type_info for schema.",
            slide_type,
            validation.errors.join("; ")
        ),
        None,
    ));
}
// Only proceed to dispatch_slide when validation passes
```

**1c. Add `--dry-run` flag to `generate-slide`** (optional, per report recommendation)

Add `--dry-run` to `Commands::GenerateSlide` that runs validation + exits with result without rendering. Useful for agentic pre-flight checks.

**Verification:**
- `cargo test` passes
- `slideforge generate-slide checklist_action_plan --params '{}'` exits non-zero with error listing `title` and `items` as missing
- MCP `generate_slide` with missing params returns `ErrorData` instead of empty HTML

---

### Phase 2: Export hardening (C3) — 30 min

**Goal:** `export` and `preview-slide` fail gracefully when Chromium is missing.

**2a. Detect Chromium at startup** (`export.rs`)

Wrap `Browser::new(ops)` in a helper that catches the specific error and returns a clear message:

```rust
pub fn ensure_chrome_available() -> Result<(), String> {
    let ops = LaunchOptions::default_builder()
        .headless(true)
        .build()
        .map_err(|e| format!("Failed to configure headless browser: {}", e))?;
    Browser::new(ops).map_err(|_| {
        "Chromium/Chrome is not installed. Install it:\n  \
         - Ubuntu/Debian: sudo apt install chromium-browser\n  \
         - macOS: brew install --cask chromium\n  \
         - Or set CHROME_PATH env var to your Chrome binary.".to_string()
    })?;
    Ok(())
}
```

**2b. Check before export** (`main.rs:219-244`, `main.rs:444-476`)

Add `ensure_chrome_available()?` before calling `export_slides` or `render_html_to_png`. This replaces the opaque Chrome error with actionable guidance.

**2c. MCP `preview_slide` and `export_carousel_slides`** (`mcp_server.rs`)

Same pattern: check before launching Chrome, return `ErrorData::invalid_request` with install instructions.

**Verification:**
- On a machine without Chrome: `slideforge export ...` prints install instructions and exits non-zero
- With Chrome: works as before

---

### Phase 3: MCP statelessness fix (C1) — 1-2 hours

**Goal:** Design tokens persist across MCP tool calls within a session.

**Current architecture:** `Server::new()` creates `ServerState` with default values. Each MCP connection creates a new `Server`. There's no session ID or persistence mechanism.

**Approach: File-based session state** (not a redesign, per AGENTS.md)

**3a. Add session state file path** (`mcp_server.rs`)

Add `state_file: PathBuf` to `ServerState`. On `configure_design`, write tokens to `{state_file}`. On `Server::new()`, attempt to load from `{state_file}` if it exists.

```rust
pub struct ServerState {
    // ... existing fields ...
    state_file: PathBuf,
}
```

**3b. State load on startup** (`mcp_server.rs:303-308`)

```rust
pub fn new() -> Self {
    let state_file = dirs::home_dir()
        .unwrap_or_default()
        .join(".slideforge")
        .join("session_state.json");
    let state = if state_file.exists() {
        fs::read_to_string(&state_file)
            .ok()
            .and_then(|s| serde_json::from_str::<SessionState>(&s).ok())
            .unwrap_or_default()
    } else {
        SessionState::default()
    };
    Self { state: Mutex::new(state), state_file }
}
```

**3c. State save on mutation** (`mcp_server.rs`)

After `configure_design` and any state-mutating tool call, write state to disk:

```rust
fn save_state(state: &SessionState, path: &Path) {
    if let Some(parent) = path.parent() { let _ = fs::create_dir_all(parent); }
    let _ = fs::write(path, serde_json::to_string_pretty(state).unwrap_or_default());
}
```

**3d. CLI `render-carousel` parity** (`main.rs`)

Already file-based via `--tokens-file`. No change needed.

**Trade-off acknowledged:** File-based persistence means two concurrent MCP servers could race. This is acceptable for the single-agent use case. Per AGENTS.md: "Targeted repair inside existing systems."

**Verification:**
- `configure_design` → `generate_slide` → `render_carousel` in MCP: tokens persist across calls
- State file exists at `~/.slideforge/session_state.json`
- `slideforge configure-design` → `slideforge render-carousel`: works via `--tokens-file`

---

### Phase 4: Error message polish (M2) — 30 min

**Goal:** MCP error responses show human-readable messages, not Rust type paths.

**4a. Wrap Chrome errors** (`export.rs`)

Replace `.map_err(|e| e.to_string())` on Chrome calls with:
```rust
.map_err(|e| format!("Chromium render failed: {}", e))?;
```

**4b. Wrap design system errors** (`mcp_server.rs:684-697`)

Already handled via `ErrorData::internal_error`. Verify no raw type leaks.

**4c. Wrap dispatch_slide errors** (`mcp_server.rs:713-721`)

`dispatch_slide` returns `Result<Value, String>` — the String is already human-readable. Verify it doesn't contain Rust debug format.

**Verification:**
- Trigger each error path, confirm output is actionable text

---

### Phase 5: Param schema discoverability (H1 + L2) — 1 hour

**Goal:** Agents can discover required params without calling MCP tools.

**5a. Add param examples to registry** (`slide_registry.rs`)

Add `"example"` field to each slide type in `get_registry()`:

```rust
"hero": {
    "description": "Hook slide with bold headline",
    "required_params": ["headline"],
    "optional_params": ["subheadline", ...],
    "example": {"headline": "10x Your Content Output", "subheadline": "With AI-powered workflows"},
    ...
}
```

**5b. Enhance `slide-info` CLI output** (`main.rs:309-325`)

Print examples when available:
```
slideforge slide-info hero
# → shows required_params, optional_params, AND example JSON
```

**5c. Add hint to validation error messages** (Phase 1c already does this)

The validation error message includes: `Call 'slide-info <slide_type>' to see required params.`

**5d. Add param schema to `generate-slide --help`** (optional)

Append a note: `Use 'slideforge slide-info <type>' to see required/optional params with examples.`

**Verification:**
- `slideforge slide-info checklist_action_plan` shows example params
- `slideforge slide-info myth_fact` shows `{"myth": "...", "fact": "..."}`
- Validation errors reference the help command

---

### Phase 6: Token override (M1) — 30 min (optional, low priority)

**Goal:** Allow single-token overrides without full regeneration.

**6a. Add `--override` flag to `configure-design`** (`main.rs`)

```rust
/// Override individual tokens (e.g. --override accent=#FF5500)
#[arg(long, action = clap::ArgAction::Append)]
overrides: Vec<String>,
```

**6b. Apply overrides after generation** (`main.rs`)

After `derive_palette`, parse `key=value` pairs and overwrite in the tokens JSON before writing.

**Verification:**
- `slideforge configure-design #4F46E5 --override accent=#FF5500` → tokens file has custom accent

---

## Implementation Order

```
Phase 1 (Critical) ─→ Phase 2 (Critical) ─→ Phase 3 (Critical)
         │                      │                      │
         ▼                      ▼                      ▼
   H2+H3: Validation       C3: Export          C1: MCP state
   blocks bad renders      Chrome guard         file persistence
         │
         ▼
Phase 4 (Medium) ─→ Phase 5 (High) ─→ Phase 6 (Optional)
         │                  │
         ▼                  ▼
   M2: Error msgs      H1+L2: Param schema
   no Rust types       discoverability
```

**Total estimated time: 4-5 hours for Phases 1-5. Phase 6 is optional.**

---

## What This Plan Does NOT Do (per AGENTS.md)

- ❌ Replace the 4:5 base composition model
- ❌ Redesign the MCP session architecture around WebSockets or databases
- ❌ Add a new global scaling model
- ❌ Rewrite all hardcoded dimensions
- ❌ Change slide styling, typography, spacing, or composition
- ❌ Add Docker containerization

## What This Plan DOES Do

- ✅ Stops silent rendering of slides with missing params
- ✅ Makes `export` fail gracefully without Chromium
- ✅ Persists design tokens across MCP tool calls
- ✅ Cleans up error messages for agentic consumption
- ✅ Makes param schemas discoverable via CLI + registry examples
- ✅ Adds optional single-token override capability

---

## Files Modified

| File | Phases | Changes |
|------|--------|---------|
| `src/main.rs` | 1, 2, 5, 6 | Validation abort, Chrome guard, dry-run flag, override flag, help hints |
| `src/mcp_server.rs` | 1, 3, 4 | Validation blocking, state persistence, error message cleanup |
| `src/export.rs` | 2, 4 | Chrome availability check, error message cleanup |
| `src/slide_registry.rs` | 5 | Add `example` fields to all slide types |
| `Cargo.toml` | 3 | Add `dirs` crate for home dir resolution (if not already available) |

---

## Test Strategy

After each phase:
1. `cargo test` — all existing tests pass
2. `cargo build` — clean build
3. Manual verification per phase's verification section
4. Run `slideforge test-full-scope` — all 24 carousels render without errors
5. MCP tool call sequence: `configure_design` → `generate_slide` (with missing params) → confirm error → `generate_slide` (with correct params) → confirm success → `render_carousel` → confirm tokens persist
