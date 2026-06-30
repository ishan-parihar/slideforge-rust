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

- `carousel_10_brand_storyteller`: split-feature image slides now use a balanced vertical image-first layout instead of a cramped left-column image.
- `carousel_44_thought_leader`: image-gallery labels are top chips instead of bottom overlays, so they no longer collide visually with the section caption.
- `validate_design`: now flags common layout failures before manual review, including component overlap, invalid CSS dimensions, constricted images, bottom caption overlays, and bad text wrapping constraints.
