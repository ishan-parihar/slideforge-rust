# Full-Scope Slide-Types Upgrade & Edge-Case Validation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Clean up `myth_fact` layout (remove glass container distortion), guarantee `grid_cards` container containment & dynamic text scaling with validator integration, and upgrade `generate_full_scope_test.py` to cover complex multi-column charts and edge-case testing for all slide types.

**Architecture:**
1. `src/components.rs`: Clean up `myth_fact_slide` by removing `get_glass_container` wrapper and streamlining `debunk` and `split` layouts. Enhance `grid_cards_slide` dynamic font-scaling logic and box constraints across all grid variants (`2-col`, `3-col`, `4-col`, `masonry`, `compact`, `list-dense`).
2. `src/validate.rs`: Add a validator check `validate_grid_cards_density` that flags when `grid_cards` character mass exceeds safe limits or missing titles/descriptions.
3. `generate_full_scope_test.py`: Upgrade test script to cover complex multi-column charts (grouped 3-series data across 4+ categories) and expand test suite to generate complex edge-case slides for all slide types.

**Tech Stack:** Rust, `cargo test`, `cargo build --release`, `python3 generate_full_scope_test.py`.

## Global Constraints

- Do NOT replace the 4:5 base composition model (420×525 px).
- Do NOT rewrite global styling; apply focused repairs in `src/components.rs` and `src/validate.rs`.
- Push commits to `origin/master` after each completed task iteration.
- Run `cargo test` after every task to verify zero regressions.

---

## Task 1 — Clean up `myth_fact_slide` (Remove glass container distortion)

**Files:**
- Modify: `src/components.rs:4980-5080`
- Test: `src/components.rs` → `#[cfg(test)]` module

**Details:**
Currently `myth_fact_slide` uses `let (gc, gx) = get_glass_container(tokens, is_dark);` which wraps the entire slide in a nested glass container element that distorts layout, fonts, and borders.
- Remove `gc` and `gx`.
- Structure `myth_fact_slide` cleanly using `slide_base` with direct card blocks for Myth and Fact.
- Standardize card styling using standard `card_styles` (solid surface background with subtle borders) without heavy glass blur overlays.

- [ ] **Step 1: Write a failing test checking myth_fact HTML has no glass-container class or style**

```rust
#[test]
fn test_myth_fact_no_glass_container_wrapper() {
    let tokens = derive_palette(
        "#0066FF", "professional", 16, 1.25, "warm-editorial", "", None, None, None,
    ).unwrap();

    let res = myth_fact_slide(
        &tokens,
        "Skipping breakfast makes you gain weight.",
        "Meta-analyses show no direct causal link.",
        "Popularized by commercial cereal marketing.",
        "light",
        "debunk",
        "editorial",
        "",
        0.4,
    );
    let html = res["html"].as_str().unwrap();
    assert!(
        !html.contains("backdrop-filter:blur"),
        "myth_fact slide should not use glass container blur wrapper"
    );
}
```

- [ ] **Step 2: Run test to verify failure**

```bash
cargo test test_myth_fact_no_glass_container_wrapper -- --nocapture
```

- [ ] **Step 3: Update `myth_fact_slide` in `src/components.rs`**

Remove `get_glass_container` call and update template formatting:

```rust
pub fn myth_fact_slide(
    tokens: &DesignTokens,
    myth: &str,
    fact: &str,
    explanation: &str,
    bg_style: &str,
    variant: &str,
    theme: &str,
    background_image: &str,
    image_opacity: f32,
) -> Value {
    let colors = get_slide_colors(tokens, bg_style, theme);
    let is_dark = colors.is_dark;
    let body_fs = tokens.type_scale.get("body").unwrap().font_size;
    let caption_fs = tokens.type_scale.get("caption").unwrap().font_size;
    let radius_md = current_component_radius(tokens, "card");
    let (card_bg, card_border, _) = card_styles(tokens, is_dark);
    let shadow_sm = tokens.shadows.get("sm").cloned().unwrap_or_else(|| "none".to_string());

    let myth_len = myth.len();
    let fact_len = fact.len();
    let dynamic_fs = if myth_len < 40 && fact_len < 40 {
        body_fs + 3
    } else if myth_len > 120 || fact_len > 120 {
        body_fs - 2
    } else {
        body_fs
    };

    let dynamic_padding = if myth_len < 40 && fact_len < 40 {
        "16px 20px"
    } else {
        "12px 16px"
    };

    let heading = heading_block("Myth vs Fact", tokens, "title", Some(&colors.text_primary), false, None, "left", "0 0 16px", true);

    let content = match variant {
        "debunk" => {
            let myth_html = format!(
                r#"<div style="background:{};border:{};border-radius:{};padding:{};margin-bottom:12px;box-shadow:{};position:relative;">
                    <div style="font-family:{};font-size:{}px;font-weight:600;color:{};text-decoration:line-through;text-decoration-color:{};text-decoration-thickness:2px;opacity:0.65;line-height:1.35;">{}</div>
                    <div style="position:absolute;top:50%;left:50%;transform:translate(-50%,-50%) rotate(-6deg);font-family:{};font-size:10px;font-weight:800;color:#FFFFFF;letter-spacing:0.12em;text-transform:uppercase;background:{};padding:4px 14px;border-radius:20px;box-shadow:0 2px 6px rgba(0,0,0,0.15);">MYTH</div>
                </div>"#,
                card_bg, card_border, radius_md, dynamic_padding, shadow_sm,
                tokens.body_font, dynamic_fs, colors.text_secondary, colors.primary,
                escape_html(myth),
                tokens.heading_font, colors.primary,
            );
            let fact_html = format!(
                r#"<div style="background:{};border-left:4px solid {};border-radius:{};padding:{};box-shadow:{};">
                    <div style="font-family:{};font-size:10px;font-weight:800;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:4px;">FACT</div>
                    <div style="font-family:{};font-size:{}px;font-weight:600;color:{};line-height:1.35;">{}</div>
                </div>"#,
                card_bg, colors.primary, radius_md, dynamic_padding, shadow_sm,
                tokens.heading_font, colors.primary,
                tokens.body_font, dynamic_fs, colors.text_primary, escape_html(fact),
            );
            let explanation_html = if !explanation.is_empty() {
                format!(
                    r#"<div style="font-family:{};font-size:{}px;color:{};margin-top:14px;line-height:1.45;">{}</div>"#,
                    tokens.body_font, caption_fs, colors.text_secondary, escape_html(explanation)
                )
            } else {
                String::new()
            };
            format!(r#"<div style="width:100%;">{}<div style="display:flex;flex-direction:column;">{}{}{}</div></div>"#, heading, myth_html, fact_html, explanation_html)
        }
        _ => {
            // split (default) — side-by-side
            let myth_html = format!(
                r#"<div style="flex:1;min-width:0;">
                    <div style="font-family:{};font-size:10px;font-weight:800;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:6px;">MYTH</div>
                    <div style="background:{};border:{};border-radius:{};padding:{};box-shadow:{};height:100%;box-sizing:border-box;">
                        <div style="font-family:{};font-size:{}px;font-weight:500;color:{};line-height:1.4;text-decoration:line-through;opacity:0.7;">{}</div>
                    </div>
                </div>"#,
                tokens.heading_font, colors.text_secondary,
                card_bg, card_border, radius_md, dynamic_padding, shadow_sm,
                tokens.body_font, dynamic_fs, colors.text_secondary, escape_html(myth),
            );
            let fact_html = format!(
                r#"<div style="flex:1;min-width:0;">
                    <div style="font-family:{};font-size:10px;font-weight:800;color:{};letter-spacing:0.1em;text-transform:uppercase;margin-bottom:6px;">FACT</div>
                    <div style="background:{};border-left:4px solid {};border-radius:{};padding:{};box-shadow:{};height:100%;box-sizing:border-box;">
                        <div style="font-family:{};font-size:{}px;font-weight:600;color:{};line-height:1.4;">{}</div>
                    </div>
                </div>"#,
                tokens.heading_font, colors.primary,
                card_bg, colors.primary, radius_md, dynamic_padding, shadow_sm,
                tokens.body_font, dynamic_fs, colors.text_primary, escape_html(fact),
            );
            let explanation_html = if !explanation.is_empty() {
                format!(
                    r#"<div style="font-family:{};font-size:{}px;color:{};margin-top:14px;line-height:1.45;text-align:center;">{}</div>"#,
                    tokens.body_font, caption_fs, colors.text_secondary, escape_html(explanation)
                )
            } else {
                String::new()
            };
            format!(r#"<div style="width:100%;">{}<div style="display:flex;gap:12px;margin-top:8px;">{}{}</div>{}</div>"#, heading, myth_html, fact_html, explanation_html)
        }
    };

    let html = slide_base(&content, tokens, bg_style, false, "80px 48px", "center");
    let html = inject_background_image(html, background_image, image_opacity, is_dark);
    json!({
        "html": html,
        "background": bg_style,
        "variant": variant,
        "theme": theme
    })
}
```

- [ ] **Step 4: Run tests to verify pass**

```bash
cargo test test_myth_fact_no_glass_container_wrapper -- --nocapture
```

- [ ] **Step 5: Commit & Push Task 1**

```bash
git add src/components.rs
git commit -m "fix(myth_fact): remove glass container wrapper to eliminate distortion and clean up layout"
git push origin master
```

---

## Task 2 — Enhance `grid_cards` container containment & validator integration

**Files:**
- Modify: `src/components.rs:2880-3150`
- Modify: `src/validate.rs`
- Test: `src/components.rs`, `src/validate.rs`

**Details:**
1. In `grid_cards_slide`, ensure grid/flex containers have explicit `box-sizing: border-box`, `max-width: 100%`, `overflow: hidden` on outer wrapper to prevent card containers from overflowing slide padding bounds.
2. In `render_single_card`, add fine-grained font scaling for title and caption based on card count and text mass.
3. In `src/validate.rs`, add validator rules to check `grid_cards` slides:
   - Warn if total text mass > 800 chars in `grid_cards` (high overflow risk).
   - Check rendered HTML for uncontained wide grids.

- [ ] **Step 1: Write failing test in `src/validate.rs` for `grid_cards` text mass audit**

```rust
#[test]
fn test_validate_grid_cards_flags_extreme_text_mass() {
    let slide = json!({
        "slide_type": "grid_cards",
        "params": {
            "title": "Extreme Density Test",
            "cards": [
                {"title": "Card 1", "description": "a".repeat(300)},
                {"title": "Card 2", "description": "b".repeat(300)},
                {"title": "Card 3", "description": "c".repeat(300)},
            ]
        }
    });
    let report = validate_design(&slide, "editorial");
    assert!(
        report.warnings.iter().any(|w| w.contains("extreme character mass")),
        "validator should warn when grid_cards total character mass exceeds threshold"
    );
}
```

- [ ] **Step 2: Run test to verify failure**

```bash
cargo test test_validate_grid_cards_flags_extreme_text_mass -- --nocapture
```

- [ ] **Step 3: Add validator check in `src/validate.rs`**

Inside `validate_design` function in `src/validate.rs`:

```rust
if slide_type == "grid_cards" {
    if let Some(cards) = params.get("cards").and_then(|v| v.as_array()) {
        let total_chars: usize = cards.iter().map(|c| {
            let t = c.get("title").and_then(|v| v.as_str()).unwrap_or("").len();
            let d = c.get("description").and_then(|v| v.as_str()).unwrap_or("").len();
            t + d
        }).sum();

        if total_chars > 700 {
            warnings.push(format!(
                "grid_cards has extreme character mass ({} chars across {} cards), risk of vertical clipping",
                total_chars, cards.len()
            ));
        }
    }
}
```

- [ ] **Step 4: Update `grid_cards_slide` in `src/components.rs` for strict box containment**

Ensure all grid wrapper containers specify `box-sizing: border-box`, `width: 100%`, `max-width: 100%`, `min-height: 0` on card items so flex/grid items shrink gracefully inside `slide_base`.

- [ ] **Step 5: Run tests to verify pass**

```bash
cargo test test_validate_grid_cards_flags_extreme_text_mass -- --nocapture
```

- [ ] **Step 6: Commit & Push Task 2**

```bash
git add src/components.rs src/validate.rs
git commit -m "feat(grid_cards): enforce strict box containment and add validator character-mass overflow checks"
git push origin master
```

---

## Task 3 — Upgrade `generate_full_scope_test.py` to test all 47 slide types & complex multi-column charts

**Files:**
- Modify: `generate_full_scope_test.py`

**Details:**
- Expand `generate_full_scope_test.py` to include:
  1. Complex multi-column charts with 4 categories and 3 series per category (Tech, Health, Finance).
  2. Test cases for ALL slide types across all 6 categories:
     - Hero, Feature, List, Quote, CTA, Comparison, Stat Row, Timeline, Callout, Split Features, Grid Cards, Headline Subheadline, Definition, Text Block, Section Divider, Text Columns.
     - Chart (Line, Area), Scatter Plot, Gauge, Radar Chart, Column Chart (Flat & Multi-Series Grouped), Table, Metric Sparkline, Funnel Chart, Metric Grid, Comparison Bars, Progress Rings.
     - Metric Card, Stat Row.
     - Problem Solution, Myth Fact, Case Study Result, Testimonial Avatar, Before After Story, Logo Cloud, Pricing Plan, Checklist Action Plan, FAQ, Process Map.
     - Image Caption, Image Headline, Image Quote, Image Callout, Image Stat, Image Gallery, Image Collage, Image Comparison.
     - QR Destination.
  3. Complicated, realistic, edge-case data for every slide type to stress-test visual rendering.

- [ ] **Step 1: Update `generate_full_scope_test.py` with comprehensive slide list and complex multi-column chart**
- [ ] **Step 2: Run `cargo test` to verify binary builds**
- [ ] **Step 3: Rebuild release binary and execute `python3 generate_full_scope_test.py`**
- [ ] **Step 4: Verify generated `dist/full_scope_carousel.html` renders all slide types without errors**
- [ ] **Step 5: Commit & Push Task 3**

```bash
git add generate_full_scope_test.py dist/slideforge-x86_64-linux-gnu
git commit -m "feat(test): upgrade full-scope harness to test all 47 slide types with complex multi-series charts"
git push origin master
```
