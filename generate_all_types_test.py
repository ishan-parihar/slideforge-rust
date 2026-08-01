#!/usr/bin/env python3
"""Generate one rendered example of EVERY slide type in the registry,
for the redundancy audit viewer.

Per AGENTS.md #1660: produce diagnostic evidence before purging.
"""
import json
import os
import subprocess
import sys
import tempfile

REPO = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(REPO, "dist", "slideforge-x86_64-linux-gnu")
OUT_DIR = os.path.join(REPO, "dist", "all_types_exports")
OUT_CAROUSEL = os.path.join(REPO, "dist", "all_types_carousel.html")
OUT_TOKENS = os.path.join(REPO, "dist", "all_types_tokens.json")
OUT_SLIDES = os.path.join(REPO, "dist", "all_types_test", "compiled_slides.json")

os.makedirs(os.path.dirname(OUT_SLIDES), exist_ok=True)
os.makedirs(OUT_DIR, exist_ok=True)

# ── Step 1: tokens ────────────────────────────────────────────────────
print("Step 1: Generating design tokens...")
subprocess.run([
    BIN, "configure-design", "#5E5FE0",
    "--style", "editorial",
    "--preset", "modern_minimal",
    "--output", OUT_TOKENS
], check=True)
print(f"  ✓ Tokens: {OUT_TOKENS}\n")


def run_generate(stype, params, variant="", theme="editorial", bg="dark", arch="data_analyst"):
    """Run slideforge generate-slide with --output tempfile pattern."""
    if variant:
        params = dict(params)
        params["variant"] = variant
    tmp = tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False)
    tmp.close()
    cmd = [
        BIN, "generate-slide", stype,
        "--tokens-file", OUT_TOKENS,
        "--theme", theme,
        "--bg-style", bg,
        "--archetype", arch,
        "--params", json.dumps(params),
        "--output", tmp.name,
    ]
    subprocess.run(cmd, check=True, capture_output=True)
    with open(tmp.name) as f:
        return json.load(f)


# ── Step 2: one example per slide type ────────────────────────────────
# Each entry: (slide_type, variant, theme, bg, params)
# Tuned to be informative AND fit on a 420x525 slide without overflow.
EXAMPLES = [
    # ── Text & Layouts ─────────────────────────────────────────────────
    ("hero", "centered", "editorial", "dark", {"headline": "Ship slides in minutes", "subheadline": "47 types, 28 presets, 1 CLI", "badge": "SlideForge", "cta_text": "Install"}),
    ("hero", "split", "bold", "light", {"headline": "Beautiful by default", "subheadline": "Editorial themes tuned for carousels"}),
    # feature/list/cta/callout retired 2026-07-30 — see split_features + qr_destination below
    ("quote", "default", "editorial", "dark", {"quote": "We replaced our entire slide toolchain with a single CLI command.", "author": "Elena Rodriguez", "role": "Head of Design, ScaleUp"}),
    ("split_features", "default", "editorial", "dark", {"title": "Built for production", "features": [{"title": "Deterministic", "description": "Same input, same output."}, {"title": "Validated", "description": "Compile-time overflow check."}, {"title": "Fast", "description": "<10ms per slide."}]}),
    # grid_cards retired 2026-07-30 — covered by split_features (icon grid variant)
    ("definition", "default", "editorial", "dark", {"term": "SlideForge", "definition": "A 4:5-first slide composition system.", "phonetic": "/slaɪd fɔːrdʒ/", "context": "Composition system, not just a renderer."}),
    ("text_block", "default", "bold", "light", {"title": "Why 4:5?", "body": "Composition is authored inside a 420×525 base canvas. Final aspect ratios are derived by fitting, not redesigning — backgrounds bleed, content does not recompose."}),
    ("section_divider", "default", "editorial", "dark", {"title": "The Composition Layer", "kicker": "Chapter 3", "subtitle": "How slides are authored"}),
    # text_columns retired 2026-07-30 — covered by split_features

    # ── Story Flows ────────────────────────────────────────────────────
    ("problem_solution", "default", "editorial", "dark", {"title": "The slide problem", "problem": "Design systems break at scale — tokens drift, components diverge.", "solution": "Compile-time validation catches violations before they ship.", "proof_points": "94 tests. Real-time composition check. Runtime geometry."}),
    ("myth_fact", "default", "editorial", "dark", {"myth": "Slide decks are just static images.", "fact": "They are runtime composition decisions against an aspect-ratio canvas.", "explanation": "Every layout is a constraint satisfaction problem at fixed geometry."}),
    ("case_study_result", "default", "editorial", "dark", {"client": "TechCorp", "challenge": "12-deck/month, 3-day lag", "solution": "Adopted SlideForge CLI", "results": [{"metric": "3x", "label": "Content velocity"}, {"metric": "0", "label": "Token violations"}]}),
    ("before_after_story", "default", "editorial", "dark", {"before": "Manual 4-hour deck", "after": "5-minute CLI run", "title": "The shift", "metric": "48x faster"}),
    ("process_map", "default", "editorial", "dark", {"title": "Build pipeline", "steps": [{"label": "Tokens", "description": "Configure design system"}, {"label": "Preset", "description": "Pick from 28 or compose"}, {"label": "Fill", "description": "AI fills content"}, {"label": "Validate", "description": "Compile-time check"}, {"label": "Export", "description": "PNG via Chromium"}]}),
    ("checklist_action_plan", "default", "editorial", "dark", {"title": "Launch checklist", "items": ["Run validators", "Check all 28 presets", "Export PNGs", "Push to dist"]}),
    ("pricing_plan", "default", "editorial", "dark", {"title": "Choose your plan", "plans": [{"name": "CLI", "price": "Free", "features": ["36 types", "28 presets", "CLI + MCP"]}, {"name": "Pro", "price": "$29/mo", "features": ["Everything in CLI", "Custom themes"], "featured": True}, {"name": "Team", "price": "$99/mo", "features": ["Everything in Pro", "API access"]}]}),
    ("testimonial_avatar", "default", "editorial", "dark", {"quote": "Our marketing team generates 3x more content since adopting SlideForge.", "author": "James Park", "role": "CMO, GrowthCo"}),
    ("logo_cloud", "default", "editorial", "dark", {"title": "Trusted by", "logos": ["TechCorp", "ScaleUp", "GrowthCo", "InnovateLabs", "CloudBase", "DataDrive"]}),
    ("faq", "default", "editorial", "dark", {"title": "Common questions", "questions": [{"q": "Does it work offline?", "a": "Yes. Fully self-contained binary."}, {"q": "Custom fonts?", "a": "Define in design tokens JSON."}, {"q": "Python SDK?", "a": "Subprocess calls work today."}]}),

    # ── Data Visualizations ────────────────────────────────────────────
    ("chart", "bar", "editorial", "dark", {"title": "Weekly output", "chart_type": "bar", "data": [{"label": "Wk1", "value": 12}, {"label": "Wk2", "value": 18}, {"label": "Wk3", "value": 25}, {"label": "Wk4", "value": 32}]}),
    ("chart", "pie", "bold", "light", {"title": "Slide distribution", "chart_type": "pie", "data": [{"label": "Hero", "value": 4}, {"label": "Data", "value": 6}, {"label": "Image", "value": 8}, {"label": "Story", "value": 5}]}),
    ("chart", "line", "editorial", "dark", {"title": "Engagement over time", "chart_type": "line", "data": [{"label": "Jan", "value": 12}, {"label": "Feb", "value": 19}, {"label": "Mar", "value": 28}, {"label": "Apr", "value": 35}]}),
    ("chart", "vertical", "editorial", "dark", {"title": "Quarterly comparison", "chart_type": "bar", "data": [{"label": "Q1", "value": 42}, {"label": "Q2", "value": 58}, {"label": "Q3", "value": 51}, {"label": "Q4", "value": 67}]}),
    ("scatter_plot", "default", "editorial", "dark", {"title": "Effort vs output", "data": [{"x": 1, "y": 2}, {"x": 2, "y": 5}, {"x": 3, "y": 9}, {"x": 4, "y": 14}, {"x": 5, "y": 22}], "x_label": "Hours", "y_label": "Slides"}),
    ("gauge", "default", "editorial", "dark", {"title": "Adoption rate", "value": 72, "label": "% teams using"}),
    ("radar_chart", "default", "editorial", "dark", {"title": "Capability matrix", "data": [{"axis": "Speed", "value": 95}, {"axis": "Quality", "value": 88}, {"axis": "Cost", "value": 92}, {"axis": "Flexibility", "value": 78}]}),
    ("table", "default", "editorial", "dark", {"title": "Benchmark results", "headers": ["Engine", "Speed", "Quality"], "rows": [["HTML", "fast", "high"], ["Canvas", "med", "med"], ["SVG", "slow", "high"]]}),
    ("metric_grid", "default", "editorial", "dark", {"title": "Pipeline metrics", "metrics": [{"value": "<10ms", "label": "Compile time"}, {"value": "47", "label": "Slide types"}, {"value": "28", "label": "Presets"}, {"value": "100%", "label": "Validated"}]}),
    ("progress_rings", "default", "editorial", "dark", {"title": "Project coverage", "items": [{"label": "Build", "value": 95}, {"label": "Test", "value": 88}, {"label": "Deploy", "value": 72}]}),
    ("comparison_bars", "default", "editorial", "dark", {"title": "Speed comparison", "comparison": {"entity_a": "Manual", "value_a": 240, "entity_b": "SlideForge", "value_b": 5, "metric": "Minutes per deck"}}),
    ("funnel_chart", "default", "editorial", "dark", {"title": "Conversion funnel", "steps": [{"label": "Visitors", "value": 1000}, {"label": "Sign-ups", "value": 420}, {"label": "Activated", "value": 180}, {"label": "Paid", "value": 64}]}),

    # ── Image Integration ──────────────────────────────────────────────
    ("image_caption", "default", "editorial", "dark", {"image_url": "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=800&h=500&fit=crop", "caption": "Editorial composition", "description": "420x525 base canvas with bleed backgrounds"}),
    ("image_headline", "default", "editorial", "dark", {"image_url": "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=800&h=500&fit=crop", "headline": "Built for carousels", "subheadline": "Composition that respects aspect ratio"}),
    ("image_quote", "default", "editorial", "dark", {"image_url": "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=800&h=500&fit=crop", "quote": "Composition is a constraint problem.", "author": "System"}),
    ("image_callout", "default", "editorial", "dark", {"image_url": "https://images.unsplash.com/photo-1497366216548-37526070297c?w=800&h=500&fit=crop", "callouts": [{"label": "Theme config", "x": 20, "y": 30}, {"label": "Pool remix", "x": 70, "y": 60}], "description": "Composition decisions"}),
    # image_stat retired 2026-07-30 — covered by image_callout (image+text combos)
    ("image_gallery", "4-grid", "editorial", "dark", {"title": "The pipeline", "images": [{"url": "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=400"}, {"url": "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=400"}, {"url": "https://images.unsplash.com/photo-1522071820081-009f0129c71c?w=400"}, {"url": "https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=400"}]}),
    ("image_collage", "default", "editorial", "dark", {"title": "Capabilities", "images": [{"url": "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=400"}, {"url": "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=400"}, {"url": "https://images.unsplash.com/photo-1522071820081-009f0129c71c?w=400"}]}),
    ("image_comparison", "default", "editorial", "dark", {"before_image": "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=400&h=300&fit=crop", "after_image": "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=400&h=300&fit=crop", "before_label": "Before", "after_label": "After"}),

    # ── Conversion ─────────────────────────────────────────────────────
    ("qr_destination", "default", "editorial", "dark", {"destination_url": "https://crates.io/crates/slideforge", "cta_text": "Install Now", "heading": "Scan to install", "caption": "CLI installation via cargo", "short_url": "sgf.dev/install"}),
    ("timeline", "default", "editorial", "dark", {"title": "Release history", "steps": [{"label": "v1", "description": "Initial release"}, {"label": "v2", "description": "Pool composition"}, {"label": "v3", "description": "Validation suite"}, {"label": "v4", "description": "27 presets + audit"}]}),
]

print(f"Step 2: Generating {len(EXAMPLES)} slides...\n")
compiled_slides = []
i = 0
for stype, variant, theme, bg, params in EXAMPLES:
    i += 1
    try:
        slide_obj = run_generate(stype, params, variant, theme, bg)
        compiled_slides.append(slide_obj)
        print(f"  {i:>2}. {stype:<22} ({variant:<14}) [{theme}/{bg}]")
    except subprocess.CalledProcessError as e:
        print(f"  {i:>2}. {stype:<22} ({variant:<14}) [{theme}/{bg}] FAILED: {e.stderr.decode()[:100]}", file=sys.stderr)
        sys.exit(1)

with open(OUT_SLIDES, "w") as f:
    json.dump(compiled_slides, f, indent=2)
print(f"\n  ✓ Saved {len(compiled_slides)} compiled slides to: {OUT_SLIDES}\n")

# ── Step 3: render carousel ──────────────────────────────────────────
print(f"Step 3: Rendering carousel with {len(compiled_slides)} slides...")
subprocess.run([
    BIN, "render-carousel", OUT_SLIDES,
    "--tokens-file", OUT_TOKENS,
    "--topic", "ALL-SLIDE-TYPES",
    "--output", OUT_CAROUSEL
], check=True)

# ── Step 4: export PNGs ──────────────────────────────────────────────
print("Step 4: Exporting PNGs...")
subprocess.run([
    BIN, "export", OUT_CAROUSEL,
    "--output-dir", OUT_DIR,
    "--slides", str(len(compiled_slides)),
], check=True, capture_output=True)

print(f"\n✅ All {len(compiled_slides)} slide types rendered.")
print(f"   Carousel: {OUT_CAROUSEL}")
print(f"   PNGs: {OUT_DIR}/")
