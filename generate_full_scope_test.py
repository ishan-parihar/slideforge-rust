#!/usr/bin/env python3
"""
Full-scope SlideForge test: every slide type × complex multi-series charts × overflow stress-testing.

Generates a single carousel with all slide types in varied light/dark themes,
including complex multi-column charts, multi-line charts, descriptions on all slides,
and intentionally long text to stress-test overflow containment.

Usage:
    python3 generate_full_scope_test.py
"""

import json, os, subprocess, sys

WORKSPACE_DIR = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(WORKSPACE_DIR, "dist", "slideforge-x86_64-linux-gnu")
if not os.path.exists(BIN):
    BIN = os.path.join(WORKSPACE_DIR, "target", "release", "slideforge-rust")

OUTPUT_DIR = os.path.join(WORKSPACE_DIR, "dist", "full_scope_test")
CAROUSEL_PATH = os.path.join(WORKSPACE_DIR, "dist", "full_scope_carousel.html")
TOKENS_FILE = os.path.join(OUTPUT_DIR, "tokens.json")
SLIDES_FILE = os.path.join(OUTPUT_DIR, "compiled_slides.json")

os.makedirs(OUTPUT_DIR, exist_ok=True)

def run_cmd(cmd, label):
    """Execute a command, return stdout, exit on failure."""
    print(f"  [{label}] $ {' '.join(cmd[:6])}...")
    r = subprocess.run(cmd, capture_output=True, text=True, cwd=WORKSPACE_DIR, timeout=60)
    if r.returncode != 0:
        print(f"  ✗ FAILED: {r.stderr[-500:] if r.stderr else r.stdout[-500:]}")
        sys.exit(1)
    return r.stdout

def generate_slide(slide_type, tokens_file, theme, bg_style, archetype, params, variant=""):
    """Call generate-slide and return the compiled slide JSON object."""
    if variant:
        params = dict(params)
        params["variant"] = variant
    cmd = [
        BIN, "generate-slide", slide_type,
        "--tokens-file", tokens_file,
        "--theme", theme,
        "--bg-style", bg_style,
        "--archetype", archetype,
        "--params", json.dumps(params),
    ]
    stdout = run_cmd(cmd, f"gen:{slide_type}")
    return json.loads(stdout)

# ── Slide definitions ─────────────────────────────────────────────────
SLIDES = [
    # ═══ SECTION: Hero & Dividers ═══
    {"section": "SECTION 1 — Hero & Dividers"},
    {"slide_type": "section_divider", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"kicker": "COMPREHENSIVE AUDIT", "title": "SlideForge Engine Evaluation", "subtitle": "Full visual layout, typography, and dataviz stress test."}},
    {"slide_type": "headline_subheadline", "theme": "bold", "bg_style": "light", "archetype": "educator",
     "params": {"headline": "Architectural Excellence", "subheadline": "Clean layout primitives, dynamic density scaling, and strict aspect-ratio containment across all devices."}},

    # ═══ SECTION: Feature & Grid Cards ═══
    {"section": "SECTION 2 — Feature & Grid Cards"},
    {"slide_type": "feature", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"icon": "⚡", "title": "Sub-Second Compilation", "description": "Compiles complex multi-slide presentation carousels in under 10 milliseconds using native Rust primitives.", "number": "01"}},
    {"slide_type": "grid_cards", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "variant": "dense",
     "params": {"title": "Core Platform Capabilities", "cards": [
         {"icon": "📊", "title": "Analytics Engine", "description": "Real-time streaming dashboards"},
         {"icon": "🔒", "title": "Zero Trust Security", "description": "End-to-end encrypted storage"},
         {"icon": "🤝", "title": "Team Workspace", "description": "Multi-user live collaboration"},
         {"icon": "⚡", "title": "High Throughput", "description": "Sub-10ms query performance"}]}},
    {"slide_type": "grid_cards", "theme": "editorial", "bg_style": "dark", "archetype": "educator",
     "variant": "list-dense",
     "params": {"title": "Research Methodology Breakdown", "cards": [
         {"icon": "🔍", "title": "Literature Synthesis", "description": "250+ academic papers reviewed"},
         {"icon": "📋", "title": "Cohort Survey", "description": "Sample size of 3,500 respondents"},
         {"icon": "🧪", "title": "Empirical Experiments", "description": "Randomized controlled trials"},
         {"icon": "📈", "title": "Bayesian Modeling", "description": "Multi-variable regression model"},
         {"icon": "✅", "title": "Peer Review Audit", "description": "Independent double-blind verification"}]}},
    {"slide_type": "grid_cards", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "compact",
     "params": {"title": "System Architecture Modules", "cards": [
         {"icon": "⚙️", "title": "Core Compiler", "description": "Rust-based HTML AST renderer"},
         {"icon": "🎨", "title": "Token Engine", "description": "Dynamic palette & typography system"},
         {"icon": "📏", "title": "Layout Guard", "description": "Strict 420x525 aspect boundary"},
         {"icon": "📈", "title": "Dataviz Engine", "description": "Native SVG path & bar generator"},
         {"icon": "📤", "title": "Export Pipeline", "description": "Headless PNG & PDF rasterizer"},
         {"icon": "🔌", "title": "MCP Server", "description": "Model Context Protocol interface"}]}},

    # ═══ SECTION: Grid Cards Overflow Stress ═══
    {"section": "SECTION 3 — Grid Cards Overflow Stress"},
    {"slide_type": "grid_cards", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "dense",
     "params": {"title": "Enterprise Feature Specification", "cards": [
         {"icon": "📊", "title": "Advanced Predictive Analytics", "description": "Real-time stream processing and high-dimensional predictive modeling engine designed to convert unstructured telemetry into actionable operational intelligence."},
         {"icon": "🔒", "title": "Military-Grade Security Suite", "description": "Multi-region key management, fine-grained role-based access control, cryptographic audit trails, and automated vulnerability scanning across all tiers."},
         {"icon": "🤝", "title": "Real-Time Collaboration Mesh", "description": "Operational Transformation (OT) powered multi-cursor editing, presence tracking, and contextual commenting for globally distributed engineering teams."},
         {"icon": "⚡", "title": "Unified API Gateway", "description": "High-capacity RESTful and GraphQL endpoints featuring automatic rate limiting, payload validation, token bucket throttling, and edge caching."}]}},

    # ═══ SECTION: Story & Myth vs Fact ═══
    {"section": "SECTION 4 — Story & Myth-Fact"},
    {"slide_type": "myth_fact", "theme": "editorial", "bg_style": "dark", "archetype": "educator",
     "variant": "debunk",
     "params": {"myth": "Breakfast is the most critical meal for weight management.",
                "fact": "Clinical trials show no statistically significant difference in weight loss between breakfast eaters and skippers.",
                "explanation": "Meta-analyses demonstrate total daily caloric deficit and macronutrient distribution determine weight outcomes, regardless of breakfast intake timing."}},
    {"slide_type": "problem_solution", "theme": "editorial", "bg_style": "light", "archetype": "startup_pitch",
     "params": {"title": "Market Disruption", "problem": "Legacy presentation tools require hours of manual layout tweaking.", "solution": "SlideForge turns structured JSON into polished carousels instantly.",
                "proof_points": [{"title": "10x Faster", "description": "Instant generation"}, {"title": "100% Consistent", "description": "Design system governed"}]}},

    # ═══ SECTION: Complex Multi-Column & Multi-Series Dataviz ═══
    {"section": "SECTION 5 — Complex Multi-Column & Multi-Line Dataviz"},
    {"slide_type": "column_chart", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Quarterly Revenue by Industry Sector",
                "caption": "Tech and Health sectors drive 70% of total quarterly revenue growth across all geographic regions.",
                "data": [
         {"label": "Q1 2024", "series": [{"name": "Tech", "value": 85}, {"name": "Health", "value": 60}, {"name": "Finance", "value": 45}, {"name": "Energy", "value": 30}]},
         {"label": "Q2 2024", "series": [{"name": "Tech", "value": 92}, {"name": "Health", "value": 68}, {"name": "Finance", "value": 50}, {"name": "Energy", "value": 35}]},
         {"label": "Q3 2024", "series": [{"name": "Tech", "value": 110}, {"name": "Health", "value": 75}, {"name": "Finance", "value": 58}, {"name": "Energy", "value": 42}]},
         {"label": "Q4 2024", "series": [{"name": "Tech", "value": 130}, {"name": "Health", "value": 88}, {"name": "Finance", "value": 70}, {"name": "Energy", "value": 52}]}]}},
    {"slide_type": "chart", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "variant": "line",
     "params": {"title": "Multi-Line Performance Benchmark", "chart_type": "line",
                "caption": "P99 latency dropped by 71% while CPU and memory consumption stabilized under sustained load.",
                "data": [
         {"label": "Jan", "series": [{"name": "Latency (ms)", "value": 120}, {"name": "CPU (%)", "value": 45}, {"name": "Memory (%)", "value": 60}]},
         {"label": "Feb", "series": [{"name": "Latency (ms)", "value": 95}, {"name": "CPU (%)", "value": 40}, {"name": "Memory (%)", "value": 55}]},
         {"label": "Mar", "series": [{"name": "Latency (ms)", "value": 60}, {"name": "CPU (%)", "value": 30}, {"name": "Memory (%)", "value": 50}]},
         {"label": "Apr", "series": [{"name": "Latency (ms)", "value": 35}, {"name": "CPU (%)", "value": 25}, {"name": "Memory (%)", "value": 45}]}]}},
    {"slide_type": "chart", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "variant": "area",
     "params": {"title": "Market Share Trajectory", "chart_type": "area",
                "caption": "Alpha Corp expanded market share from 40% to 82% over 3 years, consolidating market leadership.",
                "data": [
         {"label": "2021", "series": [{"name": "Alpha Corp", "value": 40}, {"name": "Beta Inc", "value": 25}, {"name": "Gamma Ltd", "value": 15}]},
         {"label": "2022", "series": [{"name": "Alpha Corp", "value": 52}, {"name": "Beta Inc", "value": 30}, {"name": "Gamma Ltd", "value": 18}]},
         {"label": "2023", "series": [{"name": "Alpha Corp", "value": 65}, {"name": "Beta Inc", "value": 22}, {"name": "Gamma Ltd", "value": 20}]},
         {"label": "2024", "series": [{"name": "Alpha Corp", "value": 82}, {"name": "Beta Inc", "value": 18}, {"name": "Gamma Ltd", "value": 24}]}]}},

    # ═══ SECTION: Advanced Data Visualizations ═══
    {"section": "SECTION 6 — Additional Data Visualizations"},
    {"slide_type": "progress_rings", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Product Goal Completion",
                "description": "CSAT score and retention rates exceeded target annual key performance indicators.",
                "items": [
         {"label": "Active Users", "value": 85}, {"label": "Retention Rate", "value": 72}, {"label": "CSAT Score", "value": 94}]}},
    {"slide_type": "comparison_bars", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Benchmark Comparison",
                "description": "System v2.0 delivers 5x latency reduction under peak concurrent load.",
                "comparison": {
         "entity_a": "System v1.0", "entity_b": "System v2.0",
         "metric": "Latency (ms)", "value_a": 120, "value_b": 24}}},
    {"slide_type": "radar_chart", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "System Capabilities Assessment",
                "description": "Security and Developer Experience (DX) achieved top-tier capability ratings in independent audits.",
                "data": [
         {"axis": "Speed", "value": 90}, {"axis": "Security", "value": 95}, {"axis": "Flexibility", "value": 80}, {"axis": "Scale", "value": 88}, {"axis": "DX", "value": 92}]}},
    {"slide_type": "funnel_chart", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "User Conversion Funnel",
                "description": "2.4% overall conversion rate from initial impression to paid subscriber.",
                "steps": [
         {"label": "Impressions", "value": 100000}, {"label": "Sign-ups", "value": 15000}, {"label": "Activated", "value": 8500}, {"label": "Subscribed", "value": 2400}]}},

    # ═══ SECTION: Conversion & Call to Action ═══
    {"section": "SECTION 7 — Conversion & CTAs"},
    {"slide_type": "qr_destination", "theme": "bold", "bg_style": "light", "archetype": "startup_pitch",
     "params": {"title": "Scan to Try SlideForge Live", "heading": "Instant Interactive Demo", "destination_url": "https://slideforge.dev/demo", "short_url": "slideforge.dev/demo", "cta_text": "Scan QR Code for Instant Access", "caption": "Scan with any smartphone camera to open the interactive sandbox immediately."}},
    {"slide_type": "cta", "theme": "editorial", "bg_style": "dark", "archetype": "thought_leader",
     "params": {"headline": "Start Building Presentations in Rust Today", "button_text": "Get Started Free", "button_url": "https://github.com/ishan-parihar/slideforge-rust", "subtext": "Built with high-performance Rust for sub-10ms slide rendering and instant export."}},
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
print(f"Step 2: Generating {len([s for s in SLIDES if 'slide_type' in s])} slides...\n")
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
    print(f"    ✓ {stype} ({var or 'default'})")

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
    "--topic", "SYSTEM AUDIT",
    "--url", "slideforge.dev",
    "--hashtags", "rust,slides",
    "--output", CAROUSEL_PATH
]
run_cmd(cmd_carousel, "carousel")
print(f"  ✓ Carousel: {CAROUSEL_PATH}\n")

# ── Step 4: Export PNGs ────────────────────────────────────────────────
print("Step 4: Exporting PNGs...")
cmd_export = [
    BIN, "export", CAROUSEL_PATH,
    "--slides", str(len(compiled_slides)),
    "--output-dir", OUTPUT_DIR
]
run_cmd(cmd_export, "export")
print(f"  ✓ PNGs: {OUTPUT_DIR}\n")

print(f"✅ Full-scope test complete! {len(compiled_slides)} slides generated.")
