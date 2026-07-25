#!/usr/bin/env python3
"""
Dataviz-only diagnostic carousel: every data visualization slide type rendered
in both dark and light themes for visual inspection and layout debugging.

Usage:
    python3 generate_dataviz_test.py
"""

import json, os, subprocess, sys, tempfile

WORKSPACE_DIR = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(WORKSPACE_DIR, "dist", "slideforge-x86_64-linux-gnu")
if not os.path.exists(BIN):
    BIN = os.path.join(WORKSPACE_DIR, "target", "release", "slideforge-rust")

OUTPUT_DIR = os.path.join(WORKSPACE_DIR, "dist", "dataviz_test")
CAROUSEL_PATH = os.path.join(WORKSPACE_DIR, "dist", "dataviz_carousel.html")
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


# ── Dataviz slide definitions ─────────────────────────────────────────
# Every metrics/data-viz type, alternating dark/light for visual contrast.
# Each type gets ONE slide per theme variant (dark, light) = 2 slides per type.
SLIDES = [
    # ═══ SECTION: Core Metric Types ═══
    {"section": "SECTION 1 — Core Metric Types"},

    {"slide_type": "metric_grid", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Platform Key Metrics", "metrics": [
         {"value": "10ms", "label": "Compile Latency"},
         {"value": "85+", "label": "Unit Tests Passing"},
         {"value": "47", "label": "Slide Types"},
         {"value": "100%", "label": "Token Compliance"}]}},

    {"slide_type": "metric_grid", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Revenue Performance", "metrics": [
         {"value": "$2.4M", "label": "ARR", "trend": "+32% YoY"},
         {"value": "1,200", "label": "Active Teams"},
         {"value": "94%", "label": "Retention Rate"},
         {"value": "4.8★", "label": "NPS Score"}]}},

    {"slide_type": "gauge", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "CPU Utilization", "value": "72", "label": "Current server load across all worker nodes."}},

    {"slide_type": "gauge", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Code Coverage", "value": "89", "label": "Test coverage across all modules."}},

    {"slide_type": "stat_row", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "System Performance", "stats": [
         {"value": "10ms", "label": "P50 Latency"},
         {"value": "24ms", "label": "P99 Latency"},
         {"value": "99.9%", "label": "Uptime"},
         {"value": "47ms", "label": "P999 Latency"}]}},

    {"slide_type": "stat_row", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "User Growth", "stats": [
         {"value": "12K", "label": "New Users"},
         {"value": "3.2x", "label": "MoM Growth"},
         {"value": "85%", "label": "Activation"},
         {"value": "42%", "label": "Day-7 Retention"}]}},

    # ═══ SECTION: Progress & Ring Charts ═══
    {"section": "SECTION 2 — Progress & Ring Charts"},

    {"slide_type": "progress_rings", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Sprint Goal Completion", "description": "Current sprint velocity tracking across all workstreams.",
                "items": [
         {"label": "Frontend", "value": 85},
         {"label": "Backend", "value": 72},
         {"label": "DevOps", "value": 94},
         {"label": "QA", "value": 68}]}},

    {"slide_type": "progress_rings", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Annual OKR Progress", "description": "Company-wide objective key results tracking.",
                "items": [
         {"label": "Revenue", "value": 78},
         {"label": "Growth", "value": 65},
         {"label": "Retention", "value": 92}]}},

    # ═══ SECTION: Chart Types ═══
    {"section": "SECTION 3 — Chart Types"},

    {"slide_type": "chart", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "variant": "bar",
     "params": {"title": "Monthly Active Users", "chart_type": "bar",
                "caption": "MAU grew 340% year-over-year, with strongest adoption in enterprise segment.",
                "data": [
         {"label": "Jan", "value": 1200},
         {"label": "Feb", "value": 1800},
         {"label": "Mar", "value": 2400},
         {"label": "Apr", "value": 3100},
         {"label": "May", "value": 3800},
         {"label": "Jun", "value": 5280}]}},

    {"slide_type": "chart", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "variant": "line",
     "params": {"title": "Performance Over Time", "chart_type": "line",
                "caption": "P99 latency dropped 71% while throughput increased 4x.",
                "data": [
         {"label": "Q1", "series": [{"name": "Latency", "value": 120}, {"name": "Throughput", "value": 40}]},
         {"label": "Q2", "series": [{"name": "Latency", "value": 80}, {"name": "Throughput", "value": 65}]},
         {"label": "Q3", "series": [{"name": "Latency", "value": 45}, {"name": "Throughput", "value": 110}]},
         {"label": "Q4", "series": [{"name": "Latency", "value": 35}, {"name": "Throughput", "value": 160}]}]}},

    {"slide_type": "chart", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "variant": "area",
     "params": {"title": "Revenue Trajectory", "chart_type": "area",
                "caption": "Monthly recurring revenue crossed $2M milestone in Q4.",
                "data": [
         {"label": "Jan", "series": [{"name": "MRR", "value": 800}, {"name": "Expenses", "value": 600}]},
         {"label": "Apr", "series": [{"name": "MRR", "value": 1200}, {"name": "Expenses", "value": 750}]},
         {"label": "Jul", "series": [{"name": "MRR", "value": 1600}, {"name": "Expenses", "value": 900}]},
         {"label": "Oct", "series": [{"name": "MRR", "value": 2400}, {"name": "Expenses", "value": 1100}]}]}},

    {"slide_type": "chart", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "variant": "donut",
     "params": {"title": "Market Share Distribution", "chart_type": "donut",
                "caption": "SlideForge commands 34% of the AI-powered presentation tool market.",
                "data": [
         {"label": "SlideForge", "value": 34},
         {"label": "Competitor A", "value": 22},
         {"label": "Competitor B", "value": 18},
         {"label": "Others", "value": 26}]}},

    {"slide_type": "column_chart", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Quarterly Revenue by Sector", "caption": "Tech and Health sectors drive 70% of total quarterly revenue.",
                "data": [
         {"label": "Q1", "series": [{"name": "Tech", "value": 85}, {"name": "Health", "value": 60}, {"name": "Finance", "value": 45}]},
         {"label": "Q2", "series": [{"name": "Tech", "value": 92}, {"name": "Health", "value": 68}, {"name": "Finance", "value": 50}]},
         {"label": "Q3", "series": [{"name": "Tech", "value": 110}, {"name": "Health", "value": 75}, {"name": "Finance", "value": 58}]},
         {"label": "Q4", "series": [{"name": "Tech", "value": 130}, {"name": "Health", "value": 88}, {"name": "Finance", "value": 70}]}]}},

    {"slide_type": "column_chart", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Feature Adoption Rates", "caption": "MCP integration saw fastest adoption at 78% in first quarter.",
                "data": [
         {"label": "CLI", "series": [{"name": "Adoption", "value": 65}]},
         {"label": "MCP", "series": [{"name": "Adoption", "value": 78}]},
         {"label": "Export", "series": [{"name": "Adoption", "value": 52}]},
         {"label": "Themes", "series": [{"name": "Adoption", "value": 41}]}]}},

    # ═══ SECTION: Specialized Charts ═══
    {"section": "SECTION 4 — Specialized Charts"},

    {"slide_type": "comparison_bars", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Version Performance Comparison", "description": "v3.0 delivers 5x latency reduction under peak concurrent load.",
                "comparison": {
         "entity_a": "v2.0 (Legacy)", "entity_b": "v3.0 (Current)",
         "metric": "P99 Latency (ms)", "value_a": 120, "value_b": 24}}},

    {"slide_type": "comparison_bars", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Cloud Provider Benchmark", "description": "AWS leads in cold start performance; GCP in sustained throughput.",
                "comparison": {
         "entity_a": "AWS Lambda", "entity_b": "GCP Cloud Run",
         "metric": "Cold Start (ms)", "value_a": 180, "value_b": 320}}},

    {"slide_type": "funnel_chart", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "User Conversion Funnel", "description": "2.4% overall conversion rate from impression to paid subscriber.",
                "steps": [
         {"label": "Impressions", "value": 100000},
         {"label": "Sign-ups", "value": 15000},
         {"label": "Activated", "value": 8500},
         {"label": "Subscribed", "value": 2400}]}},

    {"slide_type": "funnel_chart", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Sales Pipeline", "description": "Lead-to-close conversion across 6-month pipeline window.",
                "steps": [
         {"label": "Leads", "value": 5000},
         {"label": "Qualified", "value": 1200},
         {"label": "Demo", "value": 600},
         {"label": "Proposal", "value": 280},
         {"label": "Closed Won", "value": 95}]}},

    {"slide_type": "radar_chart", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Team Skill Assessment", "description": "Cross-functional competency mapping across engineering teams.",
                "data": [
         {"axis": "Frontend", "value": 90},
         {"axis": "Backend", "value": 85},
         {"axis": "DevOps", "value": 78},
         {"axis": "Security", "value": 92},
         {"axis": "Testing", "value": 88}]}},

    {"slide_type": "radar_chart", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Product Competitiveness", "description": "SlideForge vs industry benchmarks across key dimensions.",
                "data": [
         {"axis": "Speed", "value": 95},
         {"axis": "UX", "value": 82},
         {"axis": "Pricing", "value": 90},
         {"axis": "Ecosystem", "value": 75},
         {"axis": "Support", "value": 88}]}},

    {"slide_type": "scatter_plot", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Complexity vs Latency", "x_label": "Slide Complexity (chars)",
                "y_label": "Render Time (ms)", "data": [
         {"x": 100, "y": 2.1}, {"x": 250, "y": 3.4}, {"x": 500, "y": 5.2},
         {"x": 800, "y": 7.8}, {"x": 1200, "y": 11.2}, {"x": 1800, "y": 14.5}]}},

    {"slide_type": "scatter_plot", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Price vs Performance", "x_label": "Monthly Cost ($)",
                "y_label": "Slides Generated", "data": [
         {"x": 0, "y": 100}, {"x": 29, "y": 500}, {"x": 99, "y": 2000},
         {"x": 299, "y": 8000}, {"x": 499, "y": 15000}]}},

    # ═══ SECTION: Tables & Structured Data ═══
    {"section": "SECTION 5 — Tables & Structured Data"},

    {"slide_type": "table", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Performance Benchmarks", "headers": ["Metric", "v2.0", "v3.0", "Δ"],
                "rows": [
         ["Compilation", "45ms", "8ms", "5.6x"],
         ["Memory", "120MB", "24MB", "5.0x"],
         ["PNG Export", "1.2s", "0.3s", "4.0x"],
         ["Carousel Gen", "2.8s", "0.4s", "7.0x"]]}},

    {"slide_type": "table", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Feature Comparison Matrix", "headers": ["Feature", "Free", "Pro", "Enterprise"],
                "rows": [
         ["Slides/mo", "50", "Unlimited", "Unlimited"],
         ["Themes", "3", "15", "Custom"],
         ["Export", "PNG", "PNG + PDF", "All"],
         ["Support", "Community", "Email", "Dedicated"]]}},

{"slide_type": "split_features", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
      "params": {"title": "Architecture Pillars", "features": [
          {"title": "Compile Pipeline", "description": "Sub-10ms deterministic slide rendering via Rust native binary with zero runtime dependencies."},
          {"title": "Validation Engine", "description": "Runtime overflow detection catches text clipping before export — never in production HTML."},
          {"title": "Token System", "description": "Design tokens govern typography, spacing, and color across all 47 slide types uniformly."}]}},

    # ═══ SECTION: Image + Metric Composites ═══
    {"section": "SECTION 6 — Image + Metric Composites"},

    {"slide_type": "image_stat", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"image_url": "https://images.unsplash.com/photo-1551288049-bebda4e38f71?w=600",
                "stat_value": "340%", "stat_label": "YoY Revenue Growth",
                "description": "Enterprise adoption drove record expansion in Q4 2024."}},

    {"slide_type": "image_stat", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"image_url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f?w=600",
                "stat_value": "5.6x", "stat_label": "Compilation Speedup",
                "description": "Rust native compiler delivers dramatic performance gains."}},
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
print(f"Step 2: Generating dataviz slides...\n")
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
    "--topic", "DATAVIZ AUDIT",
    "--url", "slideforge.dev",
    "--hashtags", "rust,slides,dataviz",
    "--output", CAROUSEL_PATH
]
run_cmd(cmd_carousel, "carousel")
print(f"  ✓ Carousel: {CAROUSEL_PATH}\n")

print(f"✅ Dataviz test complete! {len(compiled_slides)} slides rendered.")
