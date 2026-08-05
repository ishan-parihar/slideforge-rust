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

def run_generate(stype, params, variant="", theme="editorial", bg="dark", arch="data_analyst", idx=None):
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
# Each entry: (slide_type, variant, theme, bg, params, category)
# category drives the viewer grouping. Tuned to fit on a 420x525 slide.
EXAMPLES = [
    # ── Openers & Hooks ────────────────────────────────────────────────
    ("hero", "centered", "editorial", "dark", {"headline": "Ship slides in minutes", "subheadline": "47 types, 28 presets, 1 CLI", "badge": "SlideForge", "tagline": "Design system CLI"}, "Openers & Hooks"),
    ("hero", "split", "bold", "light", {"headline": "Beautiful by default", "subheadline": "Editorial themes tuned for carousels", "badge": "SlideForge", "tagline": "Visual proof", "metric_value": "47", "metric_label": "slide types", "background_image": "https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=500&fit=crop&auto=format"}, "Openers & Hooks"),
    ("hero", "chapter", "editorial", "dark", {"headline": "The Composition Layer", "subheadline": "How slides are authored", "badge": "Chapter 3", "variant": "chapter"}, "Openers & Hooks"),
    # ── Narrative & Story ──────────────────────────────────────────────
    ("problem_solution", "default", "editorial", "dark", {"title": "The slide problem", "problem": "Design systems break at scale — tokens drift, components diverge.", "solution": "Compile-time validation catches violations before they ship.", "proof_points": "94 tests. Real-time composition check. Runtime geometry.", "description": "Every deck is a constraint satisfaction problem at fixed geometry."}, "Narrative & Story"),
    ("myth_fact", "default", "editorial", "dark", {"myth": "Slide decks are just static images.", "fact": "They are runtime composition decisions against an aspect-ratio canvas.", "explanation": "Every layout is a constraint satisfaction problem at fixed geometry."}, "Narrative & Story"),
    ("before_after_story", "default", "editorial", "dark", {"before": "Manual 4-hour deck", "after": "5-minute CLI run", "title": "The shift", "metric": "48x faster", "metric_label": "Speed improvement", "description": "SlideForge cuts deck production time from hours to minutes."}, "Narrative & Story"),
    ("case_study_result", "default", "editorial", "dark", {"client": "TechCorp", "challenge": "12-deck/month, 3-day lag", "solution": "Adopted SlideForge CLI", "description": "A single automation pipeline replaced an entire team's manual workflow.", "results": [{"metric": "3x", "label": "Content velocity"}, {"metric": "0", "label": "Token violations"}]}, "Narrative & Story"),

    # ── Social Proof & Trust ───────────────────────────────────────────
    ("testimonial_avatar", "default", "editorial", "dark", {"quote": "Our marketing team generates 3x more content since adopting SlideForge.", "author": "James Park", "role": "CMO, GrowthCo"}, "Social Proof & Trust"),
    ("logo_cloud", "default", "editorial", "dark", {"title": "Trusted by", "logos": ["TechCorp", "ScaleUp", "GrowthCo", "InnovateLabs", "CloudBase", "DataDrive"]}, "Social Proof & Trust"),
    ("quote", "default", "editorial", "dark", {"quote": "We replaced our entire slide toolchain with a single CLI command.", "author": "Elena Rodriguez", "role": "Head of Design, ScaleUp"}, "Social Proof & Trust"),

    # ── Data & Metrics ─────────────────────────────────────────────────
    ("chart", "bar", "editorial", "dark", {"title": "Weekly output", "chart_type": "bar", "description": "Content velocity grew 167% over four weeks with SlideForge.", "data": [{"label": "Wk1", "value": 12}, {"label": "Wk2", "value": 18}, {"label": "Wk3", "value": 25}, {"label": "Wk4", "value": 32}]}, "Data & Metrics"),
    ("chart", "pie", "bold", "light", {"title": "Slide distribution", "chart_type": "pie", "description": "Image and data slides make up over half of the composition.", "data": [{"label": "Hero", "value": 4}, {"label": "Data", "value": 6}, {"label": "Image", "value": 8}, {"label": "Story", "value": 5}]}, "Data & Metrics"),
    ("chart", "line", "editorial", "dark", {"title": "Engagement over time", "chart_type": "line", "description": "Engagement grew 192% from January through April.", "data": [{"label": "Jan", "value": 12}, {"label": "Feb", "value": 19}, {"label": "Mar", "value": 28}, {"label": "Apr", "value": 35}]}, "Data & Metrics"),
    ("chart", "vertical", "editorial", "dark", {"title": "Quarterly comparison", "chart_type": "bar_vertical", "description": "Q4 output peaked at 67 units, a 60% increase over Q1.", "data": [{"label": "Q1", "value": 42}, {"label": "Q2", "value": 58}, {"label": "Q3", "value": 51}, {"label": "Q4", "value": 67}]}, "Data & Metrics"),
    ("scatter_plot", "default", "editorial", "dark", {"title": "Effort vs output", "data": [{"x": 1, "y": 2}, {"x": 2, "y": 5}, {"x": 3, "y": 9}, {"x": 4, "y": 14}, {"x": 5, "y": 22}], "x_label": "Hours", "y_label": "Slides"}, "Data & Metrics"),
    ("gauge", "default", "editorial", "dark", {"title": "Adoption rate", "value": 72, "label": "% teams using"}, "Data & Metrics"),
    ("radar_chart", "default", "editorial", "dark", {"title": "Capability matrix", "description": "Speed and cost lead; flexibility is the growth target.", "data": [{"axis": "Speed", "value": 95}, {"axis": "Quality", "value": 88}, {"axis": "Cost", "value": 92}, {"axis": "Flexibility", "value": 78}]}, "Data & Metrics"),
    ("table", "default", "editorial", "dark", {"title": "Benchmark results", "headers": ["Engine", "Speed", "Quality"], "rows": [["HTML", "fast", "high"], ["Canvas", "med", "med"], ["SVG", "slow", "high"]]}, "Data & Metrics"),
    ("metric_grid", "default", "editorial", "dark", {"title": "Pipeline metrics", "metrics": [{"value": "<10ms", "label": "Compile time", "progress": 0.95}, {"value": "47", "label": "Slide types", "current": 47, "total": 50}, {"value": "28", "label": "Presets", "current": 28, "total": 40}, {"value": "100%", "label": "Validated", "progress": 1.0}]}, "Data & Metrics"),
    ("progress_rings", "default", "editorial", "dark", {"title": "Project coverage", "description": "Build and test are near-complete; deployment is the bottleneck.", "items": [{"label": "Build", "value": 95}, {"label": "Test", "value": 88}, {"label": "Deploy", "value": 72}]}, "Data & Metrics"),
    ("comparison_bars", "default", "editorial", "dark", {"title": "Speed comparison", "description": "SlideForge cuts production time by a third.", "comparison": {"entity_a": "Manual", "value_a": 60, "entity_b": "SlideForge", "value_b": 40, "metric": "Minutes per deck"}}, "Data & Metrics"),
    ("funnel_chart", "default", "editorial", "dark", {"title": "Conversion funnel", "description": "6.4% of visitors convert to paid after activation.", "steps": [{"label": "Visitors", "value": 1000}, {"label": "Sign-ups", "value": 420}, {"label": "Activated", "value": 180}, {"label": "Paid", "value": 64}]}, "Data & Metrics"),

    # ── Structured Content ─────────────────────────────────────────────
    ("split_features", "default", "editorial", "dark", {"title": "Built for production", "features": [{"title": "Deterministic", "description": "Same input, same output."}, {"title": "Validated", "description": "Compile-time overflow check."}, {"title": "Fast", "description": "<10ms per slide."}]}, "Structured Content"),
    ("definition", "default", "editorial", "dark", {"term": "SlideForge", "definition": "A 4:5-first slide composition system.", "phonetic": "/slaɪd fɔːrdʒ/", "context": "Composition system, not just a renderer."}, "Structured Content"),
    ("text_block", "default", "bold", "light", {"title": "Why 4:5?", "body": "Composition is authored inside a 420×525 base canvas. Final aspect ratios are derived by fitting, not redesigning — backgrounds bleed, content does not recompose.", "eyebrow": "EDITORIAL", "meta": "Read time: 2 min • By SlideForge Team"}, "Structured Content"),
    ("process_map", "default", "editorial", "dark", {"title": "Build pipeline", "steps": [{"label": "Tokens", "description": "Configure design system"}, {"label": "Preset", "description": "Pick from 28 or compose"}, {"label": "Fill", "description": "AI fills content"}, {"label": "Validate", "description": "Compile-time check"}, {"label": "Export", "description": "PNG via Chromium"}]}, "Structured Content"),
    ("timeline", "default", "editorial", "dark", {"title": "Release history", "steps": [{"label": "v1", "description": "Initial release"}, {"label": "v2", "description": "Pool composition"}, {"label": "v3", "description": "Validation suite"}, {"label": "v4", "description": "27 presets + audit"}]}, "Structured Content"),
    ("faq", "default", "editorial", "dark", {"title": "Common questions", "questions": [{"q": "Does it work offline?", "a": "Yes. Fully self-contained binary."}, {"q": "Custom fonts?", "a": "Define in design tokens JSON."}, {"q": "Python SDK?", "a": "Subprocess calls work today."}]}, "Structured Content"),
    ("pricing_plan", "default", "editorial", "dark", {"title": "Choose your plan", "plans": [{"name": "CLI", "price": "Free", "features": ["36 types", "28 presets", "CLI + MCP"]}, {"name": "Pro", "price": "$29/mo", "features": ["Everything in CLI", "Custom themes"], "featured": True}, {"name": "Team", "price": "$99/mo", "features": ["Everything in Pro", "API access"]}]}, "Structured Content"),

    # ── Visual & Images ────────────────────────────────────────────────
    ("image_caption", "default", "editorial", "dark", {"image_url": "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=800&h=500&fit=crop", "caption": "Editorial composition", "description": "420x525 base canvas with bleed backgrounds"}, "Visual & Images"),
    ("image_headline", "default", "editorial", "dark", {"image_url": "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=800&h=500&fit=crop", "headline": "Built for carousels", "subheadline": "Composition that respects aspect ratio"}, "Visual & Images"),
    ("image_quote", "default", "editorial", "dark", {"image_url": "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=800&h=500&fit=crop", "quote": "Composition is a constraint problem.", "author": "System"}, "Visual & Images"),
    ("image_callout", "default", "editorial", "dark", {"image_url": "https://images.unsplash.com/photo-1497366216548-37526070297c?w=800&h=500&fit=crop", "callouts": [{"label": "Theme config", "x": 20, "y": 30}, {"label": "Pool remix", "x": 70, "y": 60}], "description": "Composition decisions"}, "Visual & Images"),
    ("image_gallery", "4-grid", "editorial", "dark", {"title": "The pipeline", "section_caption": "Four stages of automated slide generation — from tokens to validated PNG output.", "images": [{"url": "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=400"}, {"url": "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=400"}, {"url": "https://images.unsplash.com/photo-1522071820081-009f0129c71c?w=400"}, {"url": "https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=400"}]}, "Visual & Images"),
    ("image_collage", "default", "editorial", "dark", {"title": "Capabilities", "section_caption": "Token design, layout composition, runtime validation, and CLI export in one pipeline.", "images": [{"url": "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=400"}, {"url": "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=400"}, {"url": "https://images.unsplash.com/photo-1522071820081-009f0129c71c?w=400"}]}, "Visual & Images"),
    ("image_comparison", "default", "editorial", "dark", {"before_image": "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=400&h=300&fit=crop", "after_image": "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=400&h=300&fit=crop", "before_label": "Before", "after_label": "After", "description": "Manual 4-hour deck vs. 5-minute CLI run with compile-time validation."}, "Visual & Images"),

    # ── Call to Action ─────────────────────────────────────────────────
    ("qr_destination", "default", "editorial", "dark", {"destination_url": "https://crates.io/crates/slideforge", "cta_text": "Install Now", "heading": "Scan to install", "caption": "CLI installation via cargo", "short_url": "sgf.dev/install"}, "Call to Action"),

    # ── CTA slide types (structurally distinct persuasion architectures) ──
    ("big_statement", "default", "editorial", "dark", {"heading": "Ship faster.", "body": "The slide engine for Rust.", "cta_text": "Get started"}, "Call to Action"),
    ("big_statement", "stat", "editorial", "dark", {"heading": "Join the creators already shipping faster", "stat_value": "10,000+", "stat_label": "slides generated", "cta_text": "Get started"}, "Call to Action"),
    ("comment_cta", "default", "editorial", "dark", {"heading": "Why you feel like you're not driving your own life", "sub_heading": "Lost your direction? The whole map is in the episode.", "cta_text": "Comment LISTEN and I'll DM you the link.", "keyword": "LISTEN"}, "Call to Action"),
]

print(f"Step 2: Generating {len(EXAMPLES)} slides...\n")
compiled_slides = []
slide_metas = []
i = 0
for stype, variant, theme, bg, params, category in EXAMPLES:
    i += 1
    try:
        slide_obj = run_generate(stype, params, variant, theme, bg, idx=i)
        compiled_slides.append(slide_obj)
        slide_metas.append({"idx": i, "slide_type": stype, "variant": variant, "theme": theme, "bg": bg, "category": category, "title": params.get("headline") or params.get("title") or params.get("term") or params.get("quote", "")[:40]})
        print(f"  {i:>2}. {stype:<22} ({variant:<14}) [{theme}/{bg}] [{category}]")
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
    "--brand-name", "SlideForge",
    "--topic", "ALL-SLIDE-TYPES",
    "--url", "slideforge.dev",
    "--hashtags", "#slides #rust",
    "--output", OUT_CAROUSEL
], check=True)

# ── Step 4: export PNGs ──────────────────────────────────────────────
print("Step 4: Exporting PNGs...")
subprocess.run([
    BIN, "export", OUT_CAROUSEL,
    "--output-dir", OUT_DIR,
    "--slides", str(len(compiled_slides)),
], check=True, capture_output=True)

# ── Step 5: generate audit viewer HTML ────────────────────────────────
print("\nStep 5: Generating audit viewer HTML...")

CATEGORY_LABELS = {
    "Openers & Hooks": "Openers & Hooks",
    "Narrative & Story": "Narrative & Story",
    "Social Proof & Trust": "Social Proof & Trust",
    "Data & Metrics": "Data & Metrics",
    "Structured Content": "Structured Content",
    "Visual & Images": "Visual & Images",
    "Call to Action": "Call to Action",
}
CATEGORY_ORDER = [
    "Openers & Hooks",
    "Narrative & Story",
    "Social Proof & Trust",
    "Data & Metrics",
    "Structured Content",
    "Visual & Images",
    "Call to Action",
]

# Group slides by category
by_cat = {}
for m in slide_metas:
    cat = m["category"]
    if cat not in by_cat:
        by_cat[cat] = []
    by_cat[cat].append(m)

def render_card(m):
    return f'''
<div class="slide-card" data-classification="none">
  <a href="all_types_exports/slide_{m['idx']}.png" target="_blank">
    <img class="slide-thumb" src="all_types_exports/slide_{m['idx']}.png" alt="{m['slide_type']} ({m['variant']})" loading="lazy">
  </a>
  <div class="slide-meta">
    <div class="slide-type">
      <span>{m['slide_type']}</span>
      <span class="badge">#{m['idx']}</span>
    </div>
    <div class="slide-variant">variant: {m['variant']}</div>
    <div class="slide-theme">
      <span class="dot {m['bg']}"></span>
      <span>{m['theme']} / {m['bg']}</span>
    </div>
  </div>
</div>'''

def render_category(cat, slides):
    label = CATEGORY_LABELS.get(cat, cat)
    cards = "\n".join(render_card(m) for m in slides)
    return f'''
<div class="category-section" data-category="{cat}">
  <div class="category-header">
    <span>{label}</span>
    <span class="meta">{len(slides)} examples</span>
  </div>
  <div class="slide-grid">
    {cards}
  </div>
</div>'''

# Build catalog HTML
catalog_html = "\n".join(render_category(cat, by_cat[cat]) for cat in CATEGORY_ORDER if cat in by_cat)

# Build SLIDES JSON for JS
slides_json = json.dumps(slide_metas, ensure_ascii=False)

viewer_html = f'''<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>SlideForge Slide Type Audit — Visual Reference</title>
<style>
:root {{
  --bg: #0a0b10;
  --surface: #14151c;
  --surface-2: #1c1e27;
  --border: #2a2c3d;
  --text: #edeef5;
  --text-dim: #9098a8;
  --accent: #5e5fe0;
  --warn: #f59e0b;
  --danger: #ef4444;
  --ok: #10b981;
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
.summary-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 16px; margin-top: 12px; }}
.summary-card {{ background: var(--surface-2); border: 1px solid var(--border); border-radius: 6px; padding: 14px 16px; }}
.summary-card .count {{ font-size: 32px; font-weight: 700; letter-spacing: -0.02em; color: var(--accent); }}
.summary-card .label {{ font-size: 12px; color: var(--text-dim); margin-top: 4px; }}
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
.slide-theme {{ font-size: 11px; color: var(--text-dim); display: flex; gap: 8px; }}
.dot {{ display: inline-block; width: 8px; height: 8px; border-radius: 50%; }}
.dot.dark {{ background: #1a1c25; border: 1px solid #3a3d4f; }}
.dot.light {{ background: #f8f9fc; border: 1px solid #c5c7d1; }}
.filter-bar {{ display: flex; gap: 8px; margin-bottom: 24px; flex-wrap: wrap; align-items: center; }}
.filter-bar .label {{ color: var(--text-dim); font-size: 12px; margin-right: 4px; }}
</style>
</head>
<body>

<h1>SlideForge Slide Type Audit — Visual Reference</h1>
<p class="subtitle">
  {len(slide_metas)} rendered slides covering every active type in the registry.
  Click any thumbnail to open the full PNG.
</p>

<div class="summary">
  <h2>Quick Stats</h2>
  <div class="summary-grid">
    <div class="summary-card">
      <div class="count">{len(slide_metas)}</div>
      <div class="label">Slide examples rendered</div>
    </div>
    <div class="summary-card">
      <div class="count">{len(CATEGORY_ORDER)}</div>
      <div class="label">Categories</div>
    </div>
  </div>
  <div style="margin-top: 16px; display: flex; flex-wrap: wrap; gap: 8px;">
    {''.join(f'<span style="background:var(--surface-2);border:1px solid var(--border);border-radius:4px;padding:6px 10px;font-size:12px;"><strong style="color:var(--accent);">{CATEGORY_LABELS[cat]}</strong> <span style="color:var(--text-dim);">{len(by_cat.get(cat, []))}</span></span>' for cat in CATEGORY_ORDER if cat in by_cat)}
  </div>
</div>

<div class="filter-bar">
  <span class="label">Category:</span>
  <button type="button" class="btn active" data-filter="all">All</button>
  {''.join(f'<button type="button" class="btn" data-filter="{cat}">{CATEGORY_LABELS[cat]}</button>' for cat in CATEGORY_ORDER if cat in by_cat)}
</div>

<div id="catalog">
{catalog_html}
</div>

<script>
const CATEGORY_ORDER = {json.dumps(CATEGORY_ORDER)};

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

OUT_VIEWER = os.path.join(REPO, "dist", "slide_audit_viewer.html")
with open(OUT_VIEWER, "w") as f:
    f.write(viewer_html)
print(f"  ✓ Viewer: {OUT_VIEWER}")

print(f"\n✅ All {len(compiled_slides)} slide types rendered.")
print(f"   Carousel: {OUT_CAROUSEL}")
print(f"   PNGs: {OUT_DIR}/")
print(f"   Viewer: {OUT_VIEWER}")
