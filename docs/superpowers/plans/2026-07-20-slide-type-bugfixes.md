# Slide-Type Bug Fixes — column_chart, myth_fact, grid_cards

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix three concrete visual bugs visible in `dist/full_scope_carousel.html`: (1) column_chart separator not centered on the bar, (2) myth_fact second card and heading/title issues, (3) grid_cards text/component overflow in dense variants.

**Architecture:** All three bugs live in `src/components.rs`. Each fix is a targeted in-place change to the HTML template strings in the relevant slide function. No new files. Tests added inline with the existing `#[cfg(test)]` module at the bottom of `components.rs`.

**Tech Stack:** Rust, inline HTML strings in `components.rs`, `cargo test` for verification, `cargo build --release` to rebuild the binary, `python3 generate_full_scope_test.py` to regenerate the visual output.

## Global Constraints

- Do NOT replace the 4:5 base composition model (420×525 px).
- Do NOT redesign layouts — only targeted HTML/CSS attribute fixes.
- All fixes must preserve backward-compatibility with existing passing unit tests.
- Run `cargo test` after every task before committing.

---

## Bug Inventory (root-cause analysis)

### Bug 1 — `column_chart`: Column separator not centered

**Root cause:** In the grouped (multi-series) path at line 4558–4570 in `src/components.rs`, the category `<div>` has `border-right: 1px solid …` applied on the *outer* category wrapper with `align-items:center`, but the outer div spans the *full cell width*. The visible separator stripe appears at the right-most edge of the category cell, looking visually off-center relative to the actual bar cluster.

**Fix:** Use `position:relative` on the outer category wrapper and append an absolutely-positioned separator `<div style="position:absolute;right:-4px;top:0;bottom:18px;width:1px;background:rgba(128,128,128,0.18);"></div>` spanning the bar height zone (top 0 to bottom minus label height ~18px).

### Bug 2 — `myth_fact`: Second card issues + title/heading problems

**Root cause:**
1. **Redundant second slide entry:** The test suite in `generate_full_scope_test.py` lines 140–142 includes a second `myth_fact` slide using `LONG_TEXT` with the default `split` layout, which is redundant and causes duplicate myth/fact rendering issues. The requirement calls for removing this second card.
2. **Heading font scale:** The heading inside `myth_fact_slide` (line 5035 & 5069 in `src/components.rs`) calls `heading_block("Myth vs Fact", ..., "headline", ...)`. The `"headline"` scale renders too large inside the card container. Changing it to `"title"` scale scales it appropriately.

### Bug 3 — `grid_cards`: Overflow not contained in dense/list-dense/overflow stress variants

**Root cause:**
1. **Dynamic font scaling:** In `grid_cards_slide` (line 3129–3140), the 4-card 2x2 grid path uses hardcoded font size subtractions (`title_fs - 2` / `caption_fs - 1`) that are insufficient when cards have long descriptions (~180+ chars each). Introducing an `ultra_dense` threshold (`total_chars > 500`) and using density-aware font sizes (`title_fs - 4`, `caption_fs - 3`) prevents content overflow.
2. **Missing `list-dense` variant:** `generate_full_scope_test.py` specifies `variant="list-dense"` (line 117) for a 5-card layout, but `grid_cards_slide` lacked a handler for `list-dense`, causing it to fall back to the 4-card max 2x2 grid and drop the 5th card. Adding a dedicated `list-dense` layout branch renders 4–6 cards as a compact vertical list of horizontal rows.

---

## Task 1 — Fix `column_chart` separator centering

**Files:**
- Modify: `src/components.rs:4557-4570`
- Test: `src/components.rs` → `#[cfg(test)]` module

- [ ] **Step 1: Write a failing test for centered separator**

Add inside `#[cfg(test)]` at the bottom of `src/components.rs`:

```rust
#[test]
fn test_column_chart_grouped_separator_is_not_on_outer_edge() {
    let tokens = derive_palette(
        "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
    ).unwrap();

    let res = column_chart_slide(
        &tokens,
        vec![
            json!({
                "label": "2020",
                "series": [{"name": "Tech", "value": 45}, {"name": "Health", "value": 30}]
            }),
            json!({
                "label": "2022",
                "series": [{"name": "Tech", "value": 52}, {"name": "Health", "value": 35}]
            }),
        ],
        "Employment by Sector",
        "",
        "dark",
        "bold",
        "",
        0.4,
    );
    let html = res["html"].as_str().unwrap();
    assert!(
        html.contains("position:relative") && html.contains("position:absolute"),
        "grouped column separator should use absolute positioning, not outer border-right"
    );
    assert!(
        !html.contains("border-right:1px solid rgba(128,128,128,0.15);margin-right:-0.5px;"),
        "outer category wrapper must not have border-right"
    );
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test test_column_chart_grouped_separator_is_not_on_outer_edge -- --nocapture
```

- [ ] **Step 3: Fix separator in `column_chart_slide` grouped path**

In `src/components.rs` (~line 4557), update category formatting:

```rust
let separator_html = if num_series > 1 && ci < num_categories - 1 {
    r#"<div style="position:absolute;right:-4px;top:0;bottom:18px;width:1px;background:rgba(128,128,128,0.18);"></div>"#
} else {
    ""
};
format!(
    r#"<div style="display:flex;flex-direction:column;align-items:center;flex:1;min-width:0;position:relative;">
        <div style="display:flex;align-items:flex-end;justify-content:center;width:100%;height:104px;gap:{}px;">
            {}
        </div>
        <span style="font-family:{};font-size:10px;color:{};margin-top:6px;text-align:center;max-width:100%;">{}</span>
        {}
    </div>"#,
    gap_px,
    inner_bars,
    tokens.body_font,
    colors.text_secondary,
    escape_html(lbl),
    separator_html
)
```

- [ ] **Step 4: Run tests to verify fix passes**

```bash
cargo test test_column_chart_grouped_separator_is_not_on_outer_edge -- --nocapture
```

- [ ] **Step 5: Commit**

```bash
git add src/components.rs
git commit -m "fix(column_chart): replace outer border-right with centered absolute separator div"
```

---

## Task 2 — Fix `myth_fact`: Remove second slide entry + fix heading scale

**Files:**
- Modify: `generate_full_scope_test.py:140-142`
- Modify: `src/components.rs:5035,5069`
- Test: `src/components.rs` → `#[cfg(test)]` module

- [ ] **Step 1: Write a failing test for myth_fact heading scale**

Add inside `#[cfg(test)]` in `src/components.rs`:

```rust
#[test]
fn test_myth_fact_debunk_uses_title_scale_heading() {
    let tokens = derive_palette(
        "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
    ).unwrap();

    let res = myth_fact_slide(
        &tokens,
        "Breakfast is the most important meal of the day.",
        "Studies show no significant difference between breakfast eaters and skippers.",
        "The breakfast myth was popularized by cereal companies.",
        "light",
        "debunk",
        "editorial",
        "",
        0.4,
    );
    let html = res["html"].as_str().unwrap();
    let title_fs = tokens.type_scale.get("title").unwrap().font_size;
    let headline_fs = tokens.type_scale.get("headline").unwrap().font_size;
    assert!(
        html.contains(&format!("font-size:{}px", title_fs)),
        "debunk heading should use title scale ({}px), not headline scale ({}px)",
        title_fs, headline_fs
    );
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test test_myth_fact_debunk_uses_title_scale_heading -- --nocapture
```

- [ ] **Step 3: Update heading scale in `src/components.rs` and remove duplicate test slide**

In `src/components.rs` (lines 5035 and 5069), change `"headline"` to `"title"` in `heading_block(...)`.
In `generate_full_scope_test.py` (lines 140–142), remove the second `myth_fact` slide dictionary entry.

- [ ] **Step 4: Run tests to verify fix passes**

```bash
cargo test test_myth_fact_debunk_uses_title_scale_heading -- --nocapture
```

- [ ] **Step 5: Commit**

```bash
git add src/components.rs generate_full_scope_test.py
git commit -m "fix(myth_fact): use title-scale heading in both variants; remove duplicate LONG_TEXT slide entry"
```

---

## Task 3 — Fix `grid_cards`: Dynamic scaling + add `list-dense` variant

**Files:**
- Modify: `src/components.rs:2899-2901, 2957-3140`
- Test: `src/components.rs` → `#[cfg(test)]` module

- [ ] **Step 1: Write failing tests for ultra-dense grid and list-dense variant**

```rust
#[test]
fn test_grid_cards_ultra_dense_uses_smallest_font_sizes() {
    let tokens = derive_palette(
        "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
    ).unwrap();

    let cards = vec![
        json!({"icon": "📊", "title": "Advanced Analytics Engine", "description": "Real-time data processing, predictive modeling, interactive dashboards that help organizations make data-driven decisions with unprecedented accuracy."}),
        json!({"icon": "🔒", "title": "Enterprise Security Suite", "description": "Military-grade encryption, multi-factor authentication, role-based access control, audit logging, compliance monitoring for evolving threats."}),
        json!({"icon": "🤝", "title": "Collaborative Workspace", "description": "Real-time document editing, video conferencing, project management, team communication channels for distributed teams."}),
        json!({"icon": "⚡", "title": "API Gateway", "description": "RESTful and GraphQL APIs, webhook management, third-party service connectors, rate limiting, comprehensive developer docs."}),
    ];
    let params = json!({"title": "Detailed Platform Features", "cards": cards});
    let res = dispatch_slide("grid_cards", &tokens, &params, "dark", "bold", "data_analyst").unwrap();
    let html = res["html"].as_str().unwrap();
    let default_title_fs = tokens.type_scale.get("title").unwrap().font_size;
    assert!(
        !html.contains(&format!("font-size:{}px;font-weight:600", default_title_fs)),
        "ultra-dense 4-card grid should NOT use default title font-size ({}px)",
        default_title_fs
    );
}

#[test]
fn test_grid_cards_list_dense_variant_renders_five_cards() {
    let tokens = derive_palette(
        "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
    ).unwrap();

    let params = json!({
        "title": "Research Methodology",
        "variant": "list-dense",
        "cards": [
            {"icon": "🔍", "title": "Literature Review", "description": "200+ papers analyzed"},
            {"icon": "📋", "title": "Survey Design", "description": "2400 participants"},
            {"icon": "🧪", "title": "Controlled Trials", "description": "Double-blind experiments"},
            {"icon": "📈", "title": "Statistical Modeling", "description": "Bayesian inference"},
            {"icon": "✅", "title": "Peer Review", "description": "External validation"},
        ]
    });
    let res = dispatch_slide("grid_cards", &tokens, &params, "light", "editorial", "educator").unwrap();
    let html = res["html"].as_str().unwrap();
    assert!(html.contains("Literature Review"));
    assert!(html.contains("Peer Review"));
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cargo test test_grid_cards_ultra_dense_uses_smallest_font_sizes test_grid_cards_list_dense_variant_renders_five_cards -- --nocapture
```

- [ ] **Step 3: Implement `ultra_dense` threshold and `list-dense` variant**

In `src/components.rs`:
1. Add `let ultra_dense = total_chars > 500;` alongside `dense` and `very_dense`.
2. Update default 4-card grid scaling to check `ultra_dense`.
3. Add `else if effective_variant == "list-dense"` handler rendering up to 6 cards in a compact vertical list column.

- [ ] **Step 4: Run tests to verify fix passes**

```bash
cargo test test_grid_cards_ultra_dense_uses_smallest_font_sizes test_grid_cards_list_dense_variant_renders_five_cards -- --nocapture
```

- [ ] **Step 5: Commit**

```bash
git add src/components.rs
git commit -m "fix(grid_cards): add ultra_dense threshold; density-aware 4-card font scaling; add list-dense variant"
```

---

## Task 4 — Rebuild binary + regenerate output + verify

- [ ] **Step 1: Run full test suite**

```bash
cargo test
```

- [ ] **Step 2: Rebuild release binary and copy to dist**

```bash
cargo build --release && cp target/release/slideforge-rust dist/slideforge-x86_64-linux-gnu
```

- [ ] **Step 3: Regenerate test carousel**

```bash
python3 generate_full_scope_test.py
```

- [ ] **Step 4: Commit updated test output**

```bash
git add dist/full_scope_carousel.html
git commit -m "chore: regenerate full_scope_carousel after slide-type bugfixes"
```
