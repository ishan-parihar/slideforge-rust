#!/usr/bin/env python3
"""
Text & Layout diagnostic carousel: every text/layout slide type rendered
in both dark and light themes for visual inspection and layout debugging.

Usage:
    python3 generate_text_layout_test.py
"""

import json, os, subprocess, sys, tempfile

WORKSPACE_DIR = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(WORKSPACE_DIR, "dist", "slideforge-x86_64-linux-gnu")
if not os.path.exists(BIN):
    BIN = os.path.join(WORKSPACE_DIR, "target", "release", "slideforge-rust")

OUTPUT_DIR = os.path.join(WORKSPACE_DIR, "dist", "text_layout_test")
CAROUSEL_PATH = os.path.join(WORKSPACE_DIR, "dist", "text_layout_carousel.html")
TOKENS_FILE = os.path.join(OUTPUT_DIR, "tokens.json")
SLIDES_FILE = os.path.join(OUTPUT_DIR, "compiled_slides.json")

os.makedirs(OUTPUT_DIR, exist_ok=True)


def run_cmd(cmd, label):
    """Execute a command, return stdout, exit on failure."""
    print(f"  [{label}] $ {' '.join(cmd[:4])}...")
    r = subprocess.run(cmd, capture_output=True, text=True, cwd=WORKSPACE_DIR, timeout=180)
    if r.returncode != 0:
        print(f"  ✗ FAILED: {r.stderr[-500:] if r.stderr else r.stdout[-500:]}")
        sys.exit(1)
    return r.stdout


def generate_slide(slide_type, tokens_file, theme, bg_style, archetype, params, variant=""):
    """Call generate-slide and return the compiled slide JSON object."""
    if variant:
        params = dict(params)
        params["variant"] = variant
    with tempfile.NamedTemporaryFile(suffix=".json", delete=False, mode="w") as tmp:
        out_path = tmp.name
    cmd = [
        BIN, "generate-slide", slide_type,
        "--tokens-file", tokens_file,
        "--theme", theme,
        "--bg-style", bg_style,
        "--archetype", archetype,
        "--params", json.dumps(params),
        "--output", out_path,
    ]
    run_cmd(cmd, f"gen:{slide_type}")
    with open(out_path, "r") as f:
        result = json.load(f)
    os.unlink(out_path)
    return result


# ── Text & Layout slide definitions ─────────────────────────────────────────
# Every text/layout type, alternating dark/light for visual contrast.
# Each type gets ONE slide per theme variant (dark, light) = 2 slides per type.
SLIDES = [
    # ═══ SECTION: Core Text Types ═══
    {"section": "SECTION 1 — Core Text Types"},

    {"slide_type": "hero", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"headline": "Build Faster. Ship Better.", "subheadline": "Sub-10ms slide rendering via Rust native binary.", "badge": "v3.0", "cta_text": "Get Started"}},

    {"slide_type": "hero", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"headline": "Your Data, Visualized", "subheadline": "Transform raw metrics into compelling carousel stories.", "badge": "NEW", "cta_text": "Try Demo"}},

    {"slide_type": "feature", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Compile Pipeline", "description": "Deterministic slide rendering in under 10ms with zero runtime dependencies.", "icon": "⚡", "badge": "Core"}},

    {"slide_type": "feature", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Validation Engine", "description": "Runtime overflow detection catches text clipping before export.", "icon": "🛡️", "badge": "Quality"}},

    {"slide_type": "list", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Architecture Pillars", "items": [
         "Native Rust binary — no Node, no Python at runtime",
         "Design tokens govern all 47 slide types uniformly",
         "Headless Chromium export for pixel-perfect PNGs",
         "Build-time validator catches layout issues early"
     ], "icon": "✓", "columns": 1}},

    {"slide_type": "list", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Export Formats", "items": [
         "HTML carousel with embedded tokens",
         "PNG/WebP at 4K resolution",
         "PDF rasterization for print",
         "JSON slide objects for programmatic use"
     ], "icon": "📤", "columns": 1}},

    {"slide_type": "quote", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"quote": "The best code is the code never written.", "author": "Senior Dev", "role": "SlideForge Team", "rating": 5}},

    {"slide_type": "quote", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "params": {"quote": "Validation that catches bugs before users see them.", "author": "QA Lead", "role": "Engineering", "rating": 5}},

    {"slide_type": "cta", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"headline": "Ready to Build?", "button_text": "Install SlideForge", "subheadline": "One command. Zero config. Production-ready slides.", "secondary_text": "MIT licensed · Rust native · Zero deps"}},

    {"slide_type": "cta", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"headline": "Join the Community", "button_text": "View on GitHub", "subheadline": "Contribute, report issues, request features.", "secondary_text": "100+ stars · Active development"}},

    # ═══ SECTION: Comparison & Stats ═══
    {"section": "SECTION 2 — Comparison & Stats"},

    {"slide_type": "comparison", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Before vs After", "columns": ["v2.0 (Legacy)", "v3.0 (Current)"], "rows": [
         ["Compile Time", "45ms", "8ms"],
         ["Memory", "120MB", "24MB"],
         ["PNG Export", "1.2s", "0.3s"]
     ], "variant": "table"}},

    {"slide_type": "comparison", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Cloud Benchmark", "columns": ["AWS Lambda", "GCP Cloud Run"], "rows": [
         ["Cold Start", "180ms", "320ms"],
         ["Cost/1M req", "$0.20", "$0.40"],
         ["Max Memory", "10GB", "32GB"]
     ], "variant": "table"}},

    {"slide_type": "stat_row", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "System Performance", "stats": [
         {"value": "10ms", "label": "P50 Latency"},
         {"value": "24ms", "label": "P99 Latency"},
         {"value": "99.9%", "label": "Uptime"}
     ]}},

    {"slide_type": "stat_row", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "User Growth", "stats": [
         {"value": "12K", "label": "New Users"},
         {"value": "3.2x", "label": "MoM Growth"},
         {"value": "85%", "label": "Activation"}
     ]}},

    # ═══ SECTION: Structured Content ═══
    {"section": "SECTION 3 — Structured Content"},

    {"slide_type": "timeline", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Release Milestones", "steps": [
         {"label": "v1.0", "text": "Initial Python prototype", "date": "Jan 2024"},
         {"label": "v2.0", "text": "Rust rewrite + MCP server", "date": "Jun 2024"},
         {"label": "v3.0", "text": "Native binary + validation", "date": "Jul 2024"},
         {"label": "v4.0", "text": "Pool-based presets + full scope", "date": "Jul 2024"}
     ]}},

    {"slide_type": "timeline", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Sprint Plan", "steps": [
         {"label": "Week 1", "text": "Design tokens + theme system"},
         {"label": "Week 2", "text": "Slide registry + dispatch"},
         {"label": "Week 3", "text": "Validator + export pipeline"},
         {"label": "Week 4", "text": "MCP integration + docs"}
     ]}},

    {"slide_type": "callout", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "⚠ Breaking Change", "text": "metric_card deprecated — use metric_grid, gauge, or progress_rings instead.", "icon": "🛑", "variant": "warning"}},

    {"slide_type": "callout", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "💡 Pro Tip", "text": "Use --output tempfile pattern for all binary calls — stdout is YAML, not JSON.", "icon": "💡", "variant": "info"}},

    {"slide_type": "split_features", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Architecture Pillars", "features": [
         {"title": "Compile Pipeline", "description": "Sub-10ms deterministic slide rendering via Rust native binary with zero runtime dependencies."},
         {"title": "Validation Engine", "description": "Runtime overflow detection catches text clipping before export — never in production HTML."},
         {"title": "Token System", "description": "Design tokens govern typography, spacing, and color across all 47 slide types uniformly."}
     ]}},

    {"slide_type": "split_features", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Export Capabilities", "features": [
         {"title": "HTML Carousel", "description": "Self-contained HTML with embedded design tokens and responsive layout."},
         {"title": "PNG Export", "description": "Headless Chromium rendering at 4K resolution with exact visual fidelity."},
         {"title": "PDF Rasterization", "description": "Multi-page PDF output for print and documentation workflows."}
     ]}},

    {"slide_type": "grid_cards", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Slide Categories", "cards": [
         {"title": "Text & Layout", "description": "16 types for copy, structure, and emphasis", "icon": "📝"},
         {"title": "Data Viz", "description": "11 types for charts, gauges, and metrics", "icon": "📊"},
         {"title": "Story", "description": "10 types for narrative flows", "icon": "📖"},
         {"title": "Image", "description": "8 types for visual compositions", "icon": "🖼️"}
     ], "cols": 2}},

    {"slide_type": "grid_cards", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Visual Themes", "cards": [
         {"title": "Editorial", "description": "Clean, magazine-inspired, sharp edges", "icon": "📰"},
         {"title": "Bold", "description": "High-contrast, dynamic, strong shadows", "icon": "🎯"},
         {"title": "Minimal", "description": "Restrained, generous whitespace", "icon": "⚪"},
         {"title": "Dark", "description": "Dark-mode-first, glassmorphism", "icon": "🌙"}
     ], "cols": 2}},

    {"slide_type": "definition", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"term": "Design Tokens", "definition": "Centralized design decisions (colors, spacing, typography, radii) encoded as platform-agnostic values that compile to CSS custom properties.", "phonetic": "deh-ZINE TOH-kens", "context": "SlideForge uses tokens for all 47 slide types.", "variant": "card"}},

    {"slide_type": "definition", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"term": "Pool-Based Presets", "definition": "Presets define allowed slide pools and constraints; AI agents compose carousels by selecting from pools rather than hardcoding sequences.", "phonetic": "pool-BAYST PREH-sets", "context": "Enables flexible, validated carousel generation.", "variant": "card"}},

    {"slide_type": "text_block", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Why Rust?", "body": "SlideForge chose Rust for the compile pipeline because it delivers deterministic sub-10ms rendering, zero runtime dependencies, and memory safety without garbage collection pauses. The binary embeds all templates and design tokens, making it a true single-file deployment artifact.", "variant": "default"}},

    {"slide_type": "text_block", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Validation Philosophy", "body": "Our validator catches layout issues at build time: text overflow, contrast violations, clipped content, and composition errors. It runs on the rendered HTML, not the source — what you validate is what you ship.", "variant": "default"}},

    {"slide_type": "section_divider", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Text & Layout", "kicker": "SECTION", "subtitle": "16 slide types for copy, structure, and emphasis", "variant": "default"}},

    {"slide_type": "section_divider", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "End of Audit", "kicker": "COMPLETE", "subtitle": "All 16 text/layout types rendered in both themes", "variant": "default"}},

    {"slide_type": "text_columns", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Technical Specifications", "columns": [
         {"heading": "Rendering", "body": "Native Rust binary, sub-10ms per slide, 420×525 base composition exported to target aspect ratios."},
         {"heading": "Validation", "body": "Runtime text-overflow detection, dynamic font scaling, overflow:hidden only on outermost container."},
         {"heading": "Export", "body": "Headless Chromium PNG export, PDF rasterization, carousel HTML output with embedded tokens."}
     ]}},

    {"slide_type": "text_columns", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Platform Targets", "columns": [
         {"heading": "Instagram", "body": "4:5 portrait, 1:1 square, 9:16 story/reels — all from same composition."},
         {"heading": "LinkedIn", "body": "4:5 landscape, 1:1 square — professional themes, CTA-optimized."},
         {"heading": "TikTok", "body": "9:16 vertical — bold themes, high-contrast for feed retention."}
     ]}},
]


# ── Step 1: Generate tokens ────────────────────────────────────────────
print("Step 1: Generating design tokens...")
cmd_tokens = [
    BIN, "configure-design", "#5E5FE0",
    "--style", "editorial",
    "--preset", "modern_minimal",
    "--output", TOKENS_FILE
]
run_cmd(cmd_tokens, "tokens")
print(f"  ✓ Tokens: {TOKENS_FILE}\n")

# ── Step 2: Compile slides ─────────────────────────────────────────────
print(f"Step 2: Generating text/layout slides...\n")
compiled_slides = []
for entry in SLIDES:
    if "section" in entry:
        print(f"  ── {entry['section']} ──")
        continue

    stype = entry["slide_type"]
    theme = entry.get("theme", "editorial")
    bg = entry.get("bg_style", "dark")
    arch = entry.get("archetype", "data_analyst")
    var = entry.get("variant", "")
    params = entry.get("params", {})

    slide_obj = generate_slide(stype, TOKENS_FILE, theme, bg, arch, params, variant=var)
    compiled_slides.append(slide_obj)
    print(f"    ✓ {stype} ({var or 'default'}) [{bg}]")

with open(SLIDES_FILE, "w") as f:
    json.dump(compiled_slides, f, indent=2)

print(f"\n  ✓ Saved compiled slides to: {SLIDES_FILE}\n")

# ── Step 3: Render carousel HTML ───────────────────────────────────────
print(f"Step 3: Rendering carousel with {len(compiled_slides)} slides...")
cmd_carousel = [
    BIN, "render-carousel", SLIDES_FILE,
    "--tokens-file", TOKENS_FILE,
    "--brand-name", "SLIDEFORGE",
    "--brand-handle", "@slideforge",
    "--topic", "TEXT & LAYOUT AUDIT",
    "--url", "slideforge.dev",
    "--hashtags", "rust,slides,text,layout",
    "--output", CAROUSEL_PATH
]
run_cmd(cmd_carousel, "carousel")
print(f"  ✓ Carousel: {CAROUSEL_PATH}\n")

print(f"✅ Text & Layout test complete! {len(compiled_slides)} slides rendered.")