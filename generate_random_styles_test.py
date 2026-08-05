#!/usr/bin/env python3
"""Generate every slide type with RANDOMIZED styles to stress-test
the style validator, font rendering, color derivation, and bg_style handling.

Covers: 7 font pairings × 5 bg_styles × 6 visual themes × varied type_scale.
Each slide gets a different random combination to surface rendering bugs.
"""
import json
import os
import random
import subprocess
import sys
import tempfile

REPO = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(REPO, "dist", "slideforge-x86_64-linux-gnu")
OUT_DIR = os.path.join(REPO, "dist", "random_styles_exports")
OUT_CAROUSEL = os.path.join(REPO, "dist", "random_styles_carousel.html")
OUT_SLIDES = os.path.join(REPO, "dist", "random_styles_test", "compiled_slides.json")

os.makedirs(os.path.dirname(OUT_SLIDES), exist_ok=True)
os.makedirs(OUT_DIR, exist_ok=True)

# ── Style matrices to randomize across ──────────────────────────────
STYLES = ["editorial", "warm", "technical", "bold", "classic", "rounded", "geometric", "humanist", "slab", "display"]
BG_STYLES = ["dark", "light", "gradient", "mesh", "hero"]
VISUAL_THEMES = ["editorial", "bold", "minimal", "dark", "vibrant", "natural"]
PROGRESS_STYLES = ["chips", "line", "dots"]
PRIMARY_COLORS = [
    "#5E5FE0",  # indigo
    "#E04040",  # red
    "#2D8F6F",  # teal
    "#D4831A",  # amber
    "#8B5CF6",  # purple
    "#0EA5E9",  # sky blue
    "#DC2626",  # crimson
    "#059669",  # emerald
    "#7C3AED",  # violet
    "#CA8A04",  # gold
]
TYPE_SCALE_BASES = [12, 14, 16]
ARCHETYPES = ["data_analyst", "editorial", "growth_strategist", "brand_designer", "product_lead"]
PRESETS = ["tonal_spot", "vibrant", "neutral", "monochrome", "expressive"]

random.seed(42)  # reproducible randomization


def generate_tokens(primary, style, preset="tonal_spot"):
    """Generate a token file for a given primary + style combo."""
    tmp = tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False, dir=os.path.dirname(OUT_SLIDES))
    tmp.close()
    subprocess.run([
        BIN, "configure-design", primary,
        "--style", style,
        "--preset", preset,
        "--output", tmp.name,
    ], check=True, capture_output=True)
    return tmp.name


def run_generate(stype, params, variant, tokens_file, theme, bg, arch, idx):
    """Run slideforge generate-slide with --output tempfile pattern."""
    if variant:
        params = dict(params)
        params["variant"] = variant
    tmp = tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False)
    tmp.close()
    cmd = [
        BIN, "generate-slide", stype,
        "--tokens-file", tokens_file,
        "--theme", theme,
        "--bg-style", bg,
        "--archetype", arch,
        "--params", json.dumps(params),
        "--output", tmp.name,
    ]
    subprocess.run(cmd, check=True, capture_output=True)
    with open(tmp.name) as f:
        return json.load(f)


# ── Slide examples (same content, but style params will be randomized) ──
EXAMPLES = [
    # ── Openers & Hooks
    ("hero", "centered", {"headline": "Ship slides in minutes", "subheadline": "47 types, 28 presets, 1 CLI", "badge": "SlideForge", "tagline": "Design system CLI"}, "Openers & Hooks"),
    ("hero", "split", {"headline": "Beautiful by default", "subheadline": "Editorial themes tuned for carousels", "badge": "SlideForge", "tagline": "Visual proof", "metric_value": "47", "metric_label": "slide types", "background_image": "https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=500&fit=crop&auto=format"}, "Openers & Hooks"),
    ("hero", "chapter", {"headline": "The Composition Layer", "subheadline": "How slides are authored", "badge": "Chapter 3"}, "Openers & Hooks"),
    ("section_divider", "chapter", {"title": "Act II — The Build", "subtitle": "Composition, validation, export", "kicker": "Part Two"}, "Openers & Hooks"),

    # ── Narrative & Story
    ("problem_solution", "default", {"title": "The slide problem", "problem": "Design systems break at scale — tokens drift, components diverge.", "solution": "Compile-time validation catches violations before they ship.", "proof_points": "94 tests. Real-time composition check. Runtime geometry.", "description": "Every deck is a constraint satisfaction problem at fixed geometry."}, "Narrative & Story"),
    ("myth_fact", "default", {"myth": "Slide decks are just static images.", "fact": "They are runtime composition decisions against an aspect-ratio canvas.", "explanation": "Every layout is a constraint satisfaction problem at fixed geometry."}, "Narrative & Story"),
    ("before_after_story", "default", {"before": "Manual 4-hour deck", "after": "5-minute CLI run", "title": "The shift", "metric": "48x faster", "metric_label": "Speed improvement", "description": "SlideForge cuts deck production time from hours to minutes."}, "Narrative & Story"),
    ("case_study_result", "default", {"client": "TechCorp", "challenge": "12-deck/month, 3-day lag", "solution": "Adopted SlideForge CLI", "description": "A single automation pipeline replaced an entire team's manual workflow.", "results": [{"metric": "3x", "label": "Content velocity"}, {"metric": "0", "label": "Token violations"}]}, "Narrative & Story"),

    # ── Social Proof & Trust
    ("testimonial_avatar", "default", {"quote": "Our marketing team generates 3x more content since adopting SlideForge.", "author": "James Park", "role": "CMO, GrowthCo"}, "Social Proof & Trust"),
    ("logo_cloud", "default", {"title": "Trusted by", "logos": ["TechCorp", "ScaleUp", "GrowthCo", "InnovateLabs", "CloudBase", "DataDrive"]}, "Social Proof & Trust"),
    ("quote", "default", {"quote": "We replaced our entire slide toolchain with a single CLI command.", "author": "Elena Rodriguez", "role": "Head of Design, ScaleUp"}, "Social Proof & Trust"),

    # ── Data & Metrics
    ("chart", "bar", {"title": "Weekly output", "chart_type": "bar", "description": "Content velocity grew 167% over four weeks with SlideForge.", "data": [{"label": "Wk1", "value": 12}, {"label": "Wk2", "value": 18}, {"label": "Wk3", "value": 25}, {"label": "Wk4", "value": 32}]}, "Data & Metrics"),
    ("chart", "pie", {"title": "Slide distribution", "chart_type": "pie", "description": "Image and data slides make up over half of the composition.", "data": [{"label": "Hero", "value": 4}, {"label": "Data", "value": 6}, {"label": "Image", "value": 8}, {"label": "Story", "value": 5}]}, "Data & Metrics"),
    ("chart", "line", {"title": "Engagement over time", "chart_type": "line", "description": "Engagement grew 192% from January through April.", "data": [{"label": "Jan", "value": 12}, {"label": "Feb", "value": 19}, {"label": "Mar", "value": 28}, {"label": "Apr", "value": 35}]}, "Data & Metrics"),
    ("chart", "vertical", {"title": "Quarterly comparison", "chart_type": "bar_vertical", "description": "Q4 output peaked at 67 units, a 60% increase over Q1.", "data": [{"label": "Q1", "value": 42}, {"label": "Q2", "value": 58}, {"label": "Q3", "value": 51}, {"label": "Q4", "value": 67}]}, "Data & Metrics"),
    ("scatter_plot", "default", {"title": "Effort vs output", "data": [{"x": 1, "y": 2}, {"x": 2, "y": 5}, {"x": 3, "y": 9}, {"x": 4, "y": 14}, {"x": 5, "y": 22}], "x_label": "Hours", "y_label": "Slides"}, "Data & Metrics"),
    ("gauge", "default", {"title": "Adoption rate", "value": 72, "label": "% teams using"}, "Data & Metrics"),
    ("radar_chart", "default", {"title": "Capability matrix", "description": "Speed and cost lead; flexibility is the growth target.", "data": [{"axis": "Speed", "value": 95}, {"axis": "Quality", "value": 88}, {"axis": "Cost", "value": 92}, {"axis": "Flexibility", "value": 78}]}, "Data & Metrics"),
    ("table", "default", {"title": "Benchmark results", "headers": ["Engine", "Speed", "Quality"], "rows": [["HTML", "fast", "high"], ["Canvas", "med", "med"], ["SVG", "slow", "high"]]}, "Data & Metrics"),
    ("metric_grid", "default", {"title": "Pipeline metrics", "metrics": [{"value": "<10ms", "label": "Compile time", "progress": 0.95}, {"value": "47", "label": "Slide types", "current": 47, "total": 50}, {"value": "28", "label": "Presets", "current": 28, "total": 40}, {"value": "100%", "label": "Validated", "progress": 1.0}]}, "Data & Metrics"),
    ("progress_rings", "default", {"title": "Project coverage", "description": "Build and test are near-complete; deployment is the bottleneck.", "items": [{"label": "Build", "value": 95}, {"label": "Test", "value": 88}, {"label": "Deploy", "value": 72}]}, "Data & Metrics"),
    ("comparison_bars", "default", {"title": "Speed comparison", "description": "SlideForge cuts production time by a third.", "comparison": {"entity_a": "Manual", "value_a": 60, "entity_b": "SlideForge", "value_b": 40, "metric": "Minutes per deck"}}, "Data & Metrics"),
    ("funnel_chart", "default", {"title": "Conversion funnel", "description": "6.4% of visitors convert to paid after activation.", "steps": [{"label": "Visitors", "value": 1000}, {"label": "Sign-ups", "value": 420}, {"label": "Activated", "value": 180}, {"label": "Paid", "value": 64}]}, "Data & Metrics"),

    # ── Structured Content
    ("split_features", "default", {"title": "Built for production", "features": [{"title": "Deterministic", "description": "Same input, same output."}, {"title": "Validated", "description": "Compile-time overflow check."}, {"title": "Fast", "description": "<10ms per slide."}]}, "Structured Content"),
    ("definition", "default", {"term": "SlideForge", "definition": "A 4:5-first slide composition system.", "phonetic": "/slaɪd fɔːrdʒ/", "context": "Composition system, not just a renderer."}, "Structured Content"),
    ("text_block", "default", {"title": "Why 4:5?", "body": "Composition is authored inside a 420×525 base canvas. Final aspect ratios are derived by fitting, not redesigning — backgrounds bleed, content does not recompose.", "eyebrow": "EDITORIAL", "meta": "Read time: 2 min • By SlideForge Team"}, "Structured Content"),
    ("process_map", "default", {"title": "Build pipeline", "steps": [{"label": "Tokens", "description": "Configure design system"}, {"label": "Preset", "description": "Pick from 28 or compose"}, {"label": "Fill", "description": "AI fills content"}, {"label": "Validate", "description": "Compile-time check"}, {"label": "Export", "description": "PNG via Chromium"}]}, "Structured Content"),
    ("timeline", "default", {"title": "Release history", "steps": [{"label": "v1", "description": "Initial release"}, {"label": "v2", "description": "Pool composition"}, {"label": "v3", "description": "Validation suite"}, {"label": "v4", "description": "27 presets + audit"}]}, "Structured Content"),
    ("faq", "default", {"title": "Common questions", "questions": [{"q": "Does it work offline?", "a": "Yes. Fully self-contained binary."}, {"q": "Custom fonts?", "a": "Define in design tokens JSON."}, {"q": "Python SDK?", "a": "Subprocess calls work today."}]}, "Structured Content"),
    ("pricing_plan", "default", {"title": "Choose your plan", "plans": [{"name": "CLI", "price": "Free", "features": ["36 types", "28 presets", "CLI + MCP"]}, {"name": "Pro", "price": "$29/mo", "features": ["Everything in CLI", "Custom themes"], "featured": True}, {"name": "Team", "price": "$99/mo", "features": ["Everything in Pro", "API access"]}]}, "Structured Content"),

    # ── Visual & Images
    ("image_caption", "default", {"image_url": "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=800&h=500&fit=crop", "caption": "Editorial composition", "description": "420x525 base canvas with bleed backgrounds"}, "Visual & Images"),
    ("image_headline", "default", {"image_url": "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=800&h=500&fit=crop", "headline": "Built for carousels", "subheadline": "Composition that respects aspect ratio"}, "Visual & Images"),
    ("image_quote", "default", {"image_url": "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=800&h=500&fit=crop", "quote": "Composition is a constraint problem.", "author": "System"}, "Visual & Images"),
    ("image_callout", "default", {"image_url": "https://images.unsplash.com/photo-1497366216548-37526070297c?w=800&h=500&fit=crop", "callouts": [{"label": "Theme config", "x": 20, "y": 30}, {"label": "Pool remix", "x": 70, "y": 60}], "description": "Composition decisions"}, "Visual & Images"),
    ("image_gallery", "4-grid", {"title": "The pipeline", "section_caption": "Four stages of automated slide generation — from tokens to validated PNG output.", "images": [{"url": "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=400"}, {"url": "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=400"}, {"url": "https://images.unsplash.com/photo-1522071820081-009f0129c71c?w=400"}, {"url": "https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=400"}]}, "Visual & Images"),
    ("image_collage", "default", {"title": "Capabilities", "section_caption": "Token design, layout composition, runtime validation, and CLI export in one pipeline.", "images": [{"url": "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=400"}, {"url": "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=400"}, {"url": "https://images.unsplash.com/photo-1522071820081-009f0129c71c?w=400"}]}, "Visual & Images"),
    ("image_comparison", "default", {"before_image": "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=400&h=300&fit=crop", "after_image": "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=400&h=300&fit=crop", "before_label": "Before", "after_label": "After", "description": "Manual 4-hour deck vs. 5-minute CLI run with compile-time validation."}, "Visual & Images"),

    # ── Call to Action
    ("qr_destination", "default", {"destination_url": "https://crates.io/crates/slideforge", "cta_text": "Install Now", "heading": "Scan to install", "caption": "CLI installation via cargo", "short_url": "sgf.dev/install"}, "Call to Action"),
    ("big_statement", "default", {"heading": "Ship faster.", "body": "The slide engine for Rust.", "cta_text": "Get started"}, "Call to Action"),
    ("big_statement", "stat", {"heading": "Join the creators already shipping faster", "stat_value": "10,000+", "stat_label": "slides generated", "cta_text": "Get started"}, "Call to Action"),
    ("comment_cta", "default", {"heading": "Why you feel like you're not driving your own life", "sub_heading": "Lost your direction? The whole map is in the episode.", "cta_text": "Comment LISTEN and I'll DM you the link.", "keyword": "LISTEN"}, "Call to Action"),
]


def pick_style(idx):
    """Pick a deterministic-but-varied style combo for each slide."""
    return {
        "primary": PRIMARY_COLORS[idx % len(PRIMARY_COLORS)],
        "style": STYLES[idx % len(STYLES)],
        "bg": BG_STYLES[idx % len(BG_STYLES)],
        "theme": VISUAL_THEMES[idx % len(VISUAL_THEMES)],
        "preset": PRESETS[idx % len(PRESETS)],
        "archetype": ARCHETYPES[idx % len(ARCHETYPES)],
    }


# ── Step 1: pre-generate token files for each unique (primary, style, base, ratio) combo ──
print("Step 1: Pre-generating token files for each style combo...\n")
token_cache = {}
for i in range(len(EXAMPLES)):
    s = pick_style(i)
    key = (s["primary"], s["style"], s["preset"])
    if key not in token_cache:
        token_cache[key] = generate_tokens(s["primary"], s["style"], s["preset"])
        print(f"  ✓ Tokens [{s['primary']}/{s['style']}/{s['preset']}]")
print(f"\n  Generated {len(token_cache)} unique token files\n")


# ── Step 2: generate each slide with randomized styles ─────────────
print(f"Step 2: Generating {len(EXAMPLES)} slides with randomized styles...\n")
compiled_slides = []
slide_metas = []
failures = []

for i, (stype, variant, params, category) in enumerate(EXAMPLES):
    s = pick_style(i)
    tokens_file = token_cache[(s["primary"], s["style"], s["preset"])]
    idx = i + 1
    try:
        slide_obj = run_generate(stype, params, variant, tokens_file, s["theme"], s["bg"], s["archetype"], idx)
        # Per-slide progress style rotation
        slide_obj["progress_style"] = PROGRESS_STYLES[idx % len(PROGRESS_STYLES)]
        compiled_slides.append(slide_obj)
        slide_metas.append({
            "idx": idx,
            "slide_type": stype,
            "variant": variant,
            "theme": s["theme"],
            "bg": s["bg"],
            "style": s["style"],
            "primary": s["primary"],
            "preset": s["preset"],
            "archetype": s["archetype"],
            "category": category,
            "title": params.get("headline") or params.get("title") or params.get("term") or params.get("quote", "")[:40],
        })
        print(f"  {idx:>2}. {stype:<22} ({variant:<14}) [{s['theme']}/{s['bg']}] [{s['style']}/{s['primary']}]")
    except subprocess.CalledProcessError as e:
        err = e.stderr.decode()[:200] if e.stderr else str(e)
        print(f"  {idx:>2}. {stype:<22} ({variant:<14}) [{s['theme']}/{s['bg']}] FAILED: {err}", file=sys.stderr)
        failures.append((idx, stype, variant, s, err))

with open(OUT_SLIDES, "w") as f:
    json.dump(compiled_slides, f, indent=2)
print(f"\n  ✓ Saved {len(compiled_slides)} compiled slides to: {OUT_SLIDES}")
if failures:
    print(f"\n  ⚠ {len(failures)} failures:")
    for idx, stype, variant, s, err in failures:
        print(f"    #{idx} {stype}({variant}) [{s['theme']}/{s['bg']}]: {err[:100]}")


# ── Step 3: render carousel ──────────────────────────────────────────
print(f"\nStep 3: Rendering carousel with {len(compiled_slides)} slides...")
subprocess.run([
    BIN, "render-carousel", OUT_SLIDES,
    "--tokens-file", token_cache[list(token_cache.keys())[0]],
    "--brand-name", "SlideForge",
    "--topic", "RANDOMIZED-STYLES-AUDIT",
    "--url", "slideforge.dev",
    "--hashtags", "#slides #rust",
    "--progress-style", PROGRESS_STYLES[0],
    "--output", OUT_CAROUSEL
], check=True)


# ── Step 4: export PNGs ──────────────────────────────────────────────
print("Step 4: Exporting PNGs...")
subprocess.run([
    BIN, "export", OUT_CAROUSEL,
    "--output-dir", OUT_DIR,
    "--slides", str(len(compiled_slides)),
], check=True, capture_output=True)


# ── Step 5: generate audit viewer HTML ───────────────────────────────
print("\nStep 5: Generating randomized styles audit viewer HTML...")

CATEGORY_ORDER = [
    "Openers & Hooks",
    "Narrative & Story",
    "Social Proof & Trust",
    "Data & Metrics",
    "Structured Content",
    "Visual & Images",
    "Call to Action",
]

by_cat = {}
for m in slide_metas:
    cat = m["category"]
    if cat not in by_cat:
        by_cat[cat] = []
    by_cat[cat].append(m)


def render_card(m):
    return f'''
<div class="slide-card" data-classification="none">
  <a href="random_styles_exports/slide_{m['idx']}.png" target="_blank">
    <img class="slide-thumb" src="random_styles_exports/slide_{m['idx']}.png" alt="{m['slide_type']} ({m['variant']})" loading="lazy">
  </a>
  <div class="slide-meta">
    <div class="slide-type">
      <span>{m['slide_type']}</span>
      <span class="badge">#{m['idx']}</span>
    </div>
    <div class="slide-variant">variant: {m['variant']}</div>
    <div class="slide-theme">
      <span class="color-dot" style="background:{m['primary']};"></span>
      <span>{m['theme']} / {m['bg']}</span>
    </div>
    <div class="slide-details">
      <span>{m['style']}</span> · <span>{m['archetype']}</span> · <span>{m['preset']}</span>
    </div>
  </div>
</div>'''


def render_category(cat, slides):
    cards = "\n".join(render_card(m) for m in slides)
    return f'''
<div class="category-section" data-category="{cat}">
  <div class="category-header">
    <span>{cat}</span>
    <span class="meta">{len(slides)} examples</span>
  </div>
  <div class="slide-grid">
    {cards}
  </div>
</div>'''


catalog_html = "\n".join(render_category(cat, by_cat[cat]) for cat in CATEGORY_ORDER if cat in by_cat)

viewer_html = f'''<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>SlideForge — Randomized Styles Audit</title>
<style>
:root {{
  --bg: #0a0b10;
  --surface: #14151c;
  --surface-2: #1c1e27;
  --border: #2a2c3d;
  --text: #edeef5;
  --text-dim: #9098a8;
  --accent: #5e5fe0;
}}
* {{ box-sizing: border-box; margin: 0; padding: 0; }}
body {{
  font-family: -apple-system, "Segoe UI", Inter, "Helvetica Neue", sans-serif;
  background: var(--bg);
  color: var(--text);
  line-height: 1.5;
  padding: 32px;
  max-width: 1600px;
  margin: 0 auto;
}}
h1 {{ font-size: 28px; font-weight: 700; letter-spacing: -0.02em; margin-bottom: 8px; }}
.subtitle {{ color: var(--text-dim); margin-bottom: 24px; font-size: 14px; }}
.summary {{ background: var(--surface); border: 1px solid var(--border); border-radius: 8px; padding: 20px 24px; margin-bottom: 32px; }}
.summary h2 {{ font-size: 18px; font-weight: 700; margin-bottom: 12px; }}
.summary-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 12px; margin-top: 12px; }}
.summary-card {{ background: var(--surface-2); border: 1px solid var(--border); border-radius: 6px; padding: 14px 16px; }}
.summary-card .count {{ font-size: 28px; font-weight: 700; letter-spacing: -0.02em; color: var(--accent); }}
.summary-card .label {{ font-size: 11px; color: var(--text-dim); margin-top: 4px; }}
.actions {{ display: flex; gap: 12px; margin-bottom: 32px; flex-wrap: wrap; }}
.btn {{ padding: 8px 16px; border: 1px solid var(--border); background: var(--surface); color: var(--text); border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 600; transition: all 0.15s ease; }}
.btn:hover {{ background: var(--surface-2); border-color: var(--accent); }}
.btn.active {{ background: var(--accent); border-color: var(--accent); color: white; }}
.category-section {{ margin-bottom: 40px; }}
.category-header {{ font-size: 20px; font-weight: 700; margin-bottom: 16px; padding-bottom: 8px; border-bottom: 2px solid var(--accent); display: flex; justify-content: space-between; align-items: baseline; }}
.category-header .meta {{ font-size: 13px; color: var(--text-dim); font-weight: 400; }}
.slide-grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; }}
.slide-card {{ background: var(--surface); border: 1px solid var(--border); border-radius: 8px; overflow: hidden; transition: border-color 0.15s ease; }}
.slide-card:hover {{ border-color: var(--accent); }}
.slide-thumb {{ width: 100%; aspect-ratio: 4/5; background: #000; display: block; object-fit: contain; }}
.slide-meta {{ padding: 12px 14px; border-top: 1px solid var(--border); }}
.slide-type {{ font-size: 14px; font-weight: 700; font-family: "JetBrains Mono", "SF Mono", Consolas, monospace; color: var(--text); display: flex; align-items: center; gap: 6px; margin-bottom: 4px; }}
.slide-type .badge {{ font-size: 10px; font-weight: 600; padding: 2px 6px; border-radius: 3px; background: rgba(94,95,224,0.18); color: #a8a9ff; text-transform: uppercase; letter-spacing: 0.04em; }}
.slide-variant {{ font-size: 11px; color: var(--text-dim); font-family: "JetBrains Mono", "SF Mono", Consolas, monospace; margin-bottom: 6px; }}
.slide-theme {{ font-size: 11px; color: var(--text-dim); display: flex; gap: 6px; align-items: center; margin-bottom: 2px; }}
.slide-details {{ font-size: 10px; color: var(--text-dim); opacity: 0.7; }}
.color-dot {{ display: inline-block; width: 10px; height: 10px; border-radius: 50%; flex-shrink: 0; border: 1px solid rgba(255,255,255,0.15); }}
.filter-bar {{ display: flex; gap: 8px; margin-bottom: 24px; flex-wrap: wrap; align-items: center; }}
.filter-bar .label {{ color: var(--text-dim); font-size: 12px; margin-right: 4px; }}
.failure-notice {{ background: #2a1015; border: 1px solid #ef4444; border-radius: 8px; padding: 16px 20px; margin-bottom: 32px; color: #fca5a5; }}
.failure-notice strong {{ color: #ef4444; }}
</style>
</head>
<body>

<h1>SlideForge — Randomized Styles Audit</h1>
<p class="subtitle">
  {len(slide_metas)} slides rendered with randomized font pairing × bg_style × visual theme × type_scale × primary color.
  Each slide uses a different combination to stress-test the full style matrix.
  Click any thumbnail to open the full PNG.
</p>

{"<div class='failure-notice'><strong>" + str(len(failures)) + " generation failures</strong> — see console output for details.</div>" if failures else ""}

<div class="summary">
  <h2>Style Matrix Coverage</h2>
  <div class="summary-grid">
    <div class="summary-card">
      <div class="count">{len(set(m['style'] for m in slide_metas))}/{len(STYLES)}</div>
      <div class="label">Font pairings tested</div>
    </div>
    <div class="summary-card">
      <div class="count">{len(set(m['bg'] for m in slide_metas))}/{len(BG_STYLES)}</div>
      <div class="label">BG styles tested</div>
    </div>
    <div class="summary-card">
      <div class="count">{len(set(m['theme'] for m in slide_metas))}/{len(VISUAL_THEMES)}</div>
      <div class="label">Visual themes tested</div>
    </div>
    <div class="summary-card">
      <div class="count">{len(set(m['primary'] for m in slide_metas))}/{len(PRIMARY_COLORS)}</div>
      <div class="label">Primary colors tested</div>
    </div>
    <div class="summary-card">
      <div class="count">{len(slide_metas)}</div>
      <div class="label">Total slides rendered</div>
    </div>
  </div>
</div>

<div class="filter-bar">
  <span class="label">Category:</span>
  <button type="button" class="btn active" data-filter="all">All</button>
  {''.join(f'<button type="button" class="btn" data-filter="{cat}">{cat}</button>' for cat in CATEGORY_ORDER if cat in by_cat)}
</div>

<div id="catalog">
{catalog_html}
</div>

<script>
document.querySelectorAll('[data-filter]').forEach(btn => {{
  btn.addEventListener('click', () => {{
    document.querySelectorAll('[data-filter]').forEach(b => b.classList.remove('active'));
    btn.classList.add('active');
    const filter = btn.dataset.filter;
    document.querySelectorAll('.category-section').forEach(sec => {{
      sec.style.display = (filter === 'all' || sec.dataset.category === filter) ? '' : 'none';
    }});
  }});
}});
</script>

</body>
</html>'''

OUT_VIEWER = os.path.join(REPO, "dist", "random_styles_viewer.html")
with open(OUT_VIEWER, "w") as f:
    f.write(viewer_html)
print(f"  ✓ Viewer: {OUT_VIEWER}")

print(f"\n✅ All {len(compiled_slides)} slides rendered with randomized styles.")
print(f"   Carousel: {OUT_CAROUSEL}")
print(f"   PNGs: {OUT_DIR}/")
print(f"   Viewer: {OUT_VIEWER}")
if failures:
    print(f"\n⚠ {len(failures)} FAILURES — fix these before auditing:")
    for idx, stype, variant, s, err in failures:
        print(f"   #{idx} {stype}({variant}) [{s['theme']}/{s['bg']}] [{s['style']}/{s['primary']}]: {err[:120]}")
