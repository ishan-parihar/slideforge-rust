# SlideForge Rust

SlideForge Rust is a Rust-native carousel slide generator, CLI, and MCP server for AI agents. It produces HTML carousels, validates layout quality, and can export rendered slides through a headless browser pipeline.

## Current Status

- CLI binary: `slideforge`
- MCP transport: stdio
- Full-scope generator coverage: 46 slide types
- Release target: `x86_64-unknown-linux-musl`
- Main quality gate: `validate_design`, which now checks image visibility, contrast, absolute frame overlap, bottom-caption image overlays, unitless CSS dimensions, constricted image frames, and narrow text containers that can cause one-word-per-line wrapping.

## Install

From a local release artifact:

```bash
./scripts/install-slideforge-rust.sh --local ./dist/slideforge-x86_64-unknown-linux-musl --bin-dir "$HOME/.local/bin"
```

From a release URL:

```bash
curl -fsSL https://example.com/slideforge-x86_64-unknown-linux-musl -o /tmp/slideforge
./scripts/install-slideforge-rust.sh --local /tmp/slideforge
```

The installer copies the binary to `~/.local/bin/slideforge` by default, makes it executable, verifies CLI startup, and prints MCP configuration JSON.

## CLI Usage

```bash
slideforge list-slides
slideforge list-platforms
slideforge list-archetypes
slideforge configure-design '#4F46E5' --style modern --preset tonal_spot --output design_tokens.json
slideforge test-full-scope --output-dir ./test-drafts/full-scope-test-output-rust
```

Start the MCP server:

```bash
slideforge mcp
```

Without a subcommand, `slideforge` also starts the MCP server over stdio.

## MCP Configuration

For Codex-style MCP clients:

```json
{
  "mcpServers": {
    "slideforge": {
      "command": "slideforge",
      "args": ["mcp"]
    }
  }
}
```

Available MCP tools include:

- `configure_design`
- `design_system`
- `generate_slide`
- `render_carousel`
- `export_carousel_slides`
- `list_slide_types`
- `get_slide_type_info`
- `get_slide_types_for_context`
- `list_platforms`
- `list_archetypes`
- `validate_layout`
- `validate_and_fix`
- `validate_design`
- `list_themes`
- `recommend_color_scheme`

## Agent Workflow

1. Call `list_platforms` and choose `platform` plus `aspect_ratio`.
2. Call `configure_design` with `platform`, `aspect_ratio`, brand, topic, URL, and hashtags.
3. Call `get_slide_types_for_context` for each narrative job.
4. Use `qr_destination` for off-platform actions such as blog posts, donations, digital products, newsletters, or link hubs.
5. Call `validate_layout` before generation and `validate_design` after rendering.
6. Export with the same `platform` and `aspect_ratio` used during render.

## Quality Gates

Run the local verification suite before release:

```bash
cargo fmt --check
cargo test
cargo build --release
cargo build --release --target x86_64-unknown-linux-musl
python3 test_full_scope_rust.py
```

The full-scope HTML fixtures are written to:

```text
test-drafts/full-scope-test-output-rust/
```

## Release

Build the musl binary:

```bash
cargo build --release --target x86_64-unknown-linux-musl
mkdir -p dist
cp target/x86_64-unknown-linux-musl/release/slideforge-rust dist/slideforge-x86_64-unknown-linux-musl
chmod +x dist/slideforge-x86_64-unknown-linux-musl
```

Create a GitHub release when a remote is configured:

```bash
git tag v0.1.0
git push origin master --tags
gh release create v0.1.0 dist/slideforge-x86_64-unknown-linux-musl --title "slideforge-rust v0.1.0" --notes "Static Linux musl build of SlideForge Rust."
```

Create a GitLab release when using GitLab:

```bash
git tag v0.1.0
git push origin master --tags
glab release create v0.1.0 dist/slideforge-x86_64-unknown-linux-musl --name "slideforge-rust v0.1.0" --notes "Static Linux musl build of SlideForge Rust."
```

## Recent Layout Fixes

- `carousel_10_brand_storyteller` (split_features): image height is now adaptive based on feature count (240px for 1, 180px for 2, 140px for 3+) so the composition never overflows the 525px canvas. Default padding reduced from `52px 36px 60px` to `44px 32px 52px` for better vertical budget.
- `carousel_40_brand_storyteller` (image_headline) and `carousel_41_data_analyst` (image_quote): now use `slide_base_bleed()` which emits `.slide-content--bleed` instead of `.slide-content`, allowing the primary image to fill the entire slide canvas (525×525 for 1:1, 420×747 for 9:16) instead of being clipped to the 420×525 composition.
- `breadcrumb-progress`: moved from `bottom:42px` to `bottom:8px` so it sits cleanly below the overlay-bottom text with a ~13px gap, eliminating the previous 3px collision.
- `slide__overlay` and `breadcrumb-progress`: re-parented from `.slide-composition` to `.slide` so they anchor to the full canvas for full-bleed aspect ratios (1:1, 9:16, 3:4), not the 420×525 composition.
- `inject_background_image`: fade masks reduced from 30% to 10% of canvas height to minimize visible bands on full-bleed slides.
- `validate_design`: now flags common layout failures before manual review, including:
  - `progress_overlay_collision`: breadcrumb-progress too close to overlay-bottom text (<12px gap)
  - `missing_full_bleed_stretch_rule`: full-bleed slides missing the first-of-type stretch CSS rule
  - `full_bleed_image_trapped_in_content`: image-primary slides using `.slide-content` instead of `.slide-content--bleed` on full-bleed canvases
  - `bg_image_mask_band`: aggressive bg-image masks (<85% black) creating visible bands on full-bleed canvases
  - Plus existing checks: component overlap, invalid CSS dimensions, constricted images, bottom caption overlays, bad text wrapping, progress indicator thickness, image aspect distortion, edge effect bleed, and slide body overflow.
