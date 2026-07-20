#!/usr/bin/env python3
"""
Full-scope SlideForge test: every slide type × complex multi-series charts × overflow stress-testing.

Generates a single carousel with all slide types in light/dark themes,
including complex multi-column charts and intentionally long text to stress-test overflow containment.

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
     "params": {"kicker": "COMPREHENSIVE TEST", "title": "SlideForge System Audit", "subtitle": "Full layout & dataviz stress testing"}},
    {"slide_type": "hero", "theme": "bold", "bg_style": "dark", "archetype": "thought_leader",
     "params": {"headline": "Next-Gen Slide Composition System", "subheadline": "Automated visual layout engine built with Rust and modern HTML/CSS.", "badge": "SlideForge v0.4"}},
    {"slide_type": "headline_subheadline", "theme": "editorial", "bg_style": "light", "archetype": "educator",
     "params": {"headline": "Architectural Excellence", "subheadline": "Clean layout primitives, dynamic density scaling, and strict aspect-ratio containment."}},

    # ═══ SECTION: Feature & Cards ═══
    {"section": "SECTION 2 — Feature & Grid Cards"},
    {"slide_type": "feature", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"icon": "⚡", "title": "Sub-Second Compilation", "description": "Compiles complex multi-slide presentation carousels in milliseconds.", "number": "01"}},
    {"slide_type": "grid_cards", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "variant": "dense",
     "params": {"title": "Core Platform Capabilities", "cards": [
         {"icon": "📊", "title": "Analytics Engine", "description": "Real-time streaming dashboards"},
         {"icon": "🔒", "title": "Zero Trust Security", "description": "End-to-end encrypted storage"},
         {"icon": "🤝", "title": "Team Workspace", "description": "Multi-user live collaboration"},
         {"icon": "⚡", "title": "High Throughput", "description": "Sub-10ms query performance"}]}},
    {"slide_type": "grid_cards", "theme": "editorial", "bg_style": "light", "archetype": "educator",
     "variant": "list-dense",
     "params": {"title": "Research Methodology Breakdown", "cards": [
         {"icon": "🔍", "title": "Literature Synthesis", "description": "250+ academic papers reviewed"},
         {"icon": "📋", "title": "Cohort Survey", "description": "Sample size of 3,500 respondents"},
         {"icon": "🧪", "title": "Empirical Experiments", "description": "Randomized controlled trials"},
         {"icon": "📈", "title": "Bayesian Modeling", "description": "Multi-variable regression model"},
         {"icon": "✅", "title": "Peer Review Audit", "description": "Independent double-blind verification"}]}},

    # ═══ SECTION: Grid Cards Overflow Stress ═══
    {"section": "SECTION 3 — Grid Cards Overflow Stress"},
    {"slide_type": "grid_cards", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "variant": "dense",
     "params": {"title": "Enterprise Feature Specification", "cards": [
         {"icon": "📊", "title": "Advanced Predictive Analytics", "description": "Real-time stream processing and high-dimensional predictive modeling engine designed to convert unstructured telemetry into actionable operational intelligence."},
         {"icon": "🔒", "title": "Military-Grade Security Suite", "description": "Multi-region key management, fine-grained role-based access control, cryptographic audit trails, and automated vulnerability scanning across all tiers."},
         {"icon": "🤝", "title": "Real-Time Collaboration Mesh", "description": "Operational Transformation (OT) powered multi-cursor editing, presence tracking, and contextual commenting for globally distributed engineering teams."},
         {"icon": "⚡", "title": "Unified API Gateway", "description": "High-capacity RESTful and GraphQL endpoints featuring automatic rate limiting, payload validation, token bucket throttling, and edge caching."}]}},

    # ═══ SECTION: Story & Myth vs Fact ═══
    {"section": "SECTION 4 — Story & Myth-Fact"},
    {"slide_type": "myth_fact", "theme": "editorial", "bg_style": "light", "archetype": "educator",
     "variant": "debunk",
     "params": {"myth": "Breakfast is the most critical meal for weight management.",
                "fact": "Clinical trials show no statistically significant difference in weight loss between breakfast eaters and skippers.",
                "explanation": "Meta-analyses demonstrate total daily caloric deficit and macronutrient distribution determine weight outcomes, regardless of breakfast intake timing."}},
    {"slide_type": "problem_solution", "theme": "editorial", "bg_style": "dark", "archetype": "startup_pitch",
     "params": {"title": "Market Disruption", "problem": "Legacy presentation tools require hours of manual layout tweaking.", "solution": "SlideForge turns structured JSON into polished carousels instantly.",
                "proof_points": [{"title": "10x Faster", "description": "Instant generation"}, {"title": "100% Consistent", "description": "Design system governed"}]}},

    # ═══ SECTION: Complex Multi-Column & Data Charts ═══
    {"section": "SECTION 5 — Complex Multi-Column & Multi-Series Dataviz"},
    {"slide_type": "column_chart", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Quarterly Revenue by Industry Sector (Complex Multi-Series)", "data": [
         {"label": "Q1 2024", "series": [{"name": "Tech", "value": 85}, {"name": "Health", "value": 60}, {"name": "Finance", "value": 45}, {"name": "Energy", "value": 30}]},
         {"label": "Q2 2024", "series": [{"name": "Tech", "value": 92}, {"name": "Health", "value": 68}, {"name": "Finance", "value": 50}, {"name": "Energy", "value": 35}]},
         {"label": "Q3 2024", "series": [{"name": "Tech", "value": 110}, {"name": "Health", "value": 75}, {"name": "Finance", "value": 58}, {"name": "Energy", "value": 42}]},
         {"label": "Q4 2024", "series": [{"name": "Tech", "value": 130}, {"name": "Health", "value": 88}, {"name": "Finance", "value": 70}, {"name": "Energy", "value": 52}]}]}},
    {"slide_type": "chart", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "variant": "area",
     "params": {"title": "Market Share Trajectory (Multi-Series Area)", "chart_type": "area", "data": [
         {"label": "2021", "series": [{"name": "Alpha Corp", "value": 40}, {"name": "Beta Inc", "value": 25}, {"name": "Gamma Ltd", "value": 15}]},
         {"label": "2022", "series": [{"name": "Alpha Corp", "value": 52}, {"name": "Beta Inc", "value": 30}, {"name": "Gamma Ltd", "value": 18}]},
         {"label": "2023", "series": [{"name": "Alpha Corp", "value": 65}, {"name": "Beta Inc", "value": 22}, {"name": "Gamma Ltd", "value": 20}]},
         {"label": "2024", "series": [{"name": "Alpha Corp", "value": 82}, {"name": "Beta Inc", "value": 18}, {"name": "Gamma Ltd", "value": 24}]}]}},

    # ═══ SECTION: Advanced Data Visualizations ═══
    {"section": "SECTION 6 — Additional Data Visualizations"},
    {"slide_type": "progress_rings", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Product Goal Completion", "items": [
         {"label": "Active Users", "value": 85}, {"label": "Retention Rate", "value": 72}, {"label": "CSAT Score", "value": 94}]}},
    {"slide_type": "comparison_bars", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Benchmark Comparison", "comparison": {
         "entity_a": "System v1.0", "entity_b": "System v2.0",
         "metric": "Latency (ms)", "value_a": 120, "value_b": 24}}},
    {"slide_type": "radar_chart", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "System Capabilities Assessment", "data": [
         {"axis": "Speed", "value": 90}, {"axis": "Security", "value": 95}, {"axis": "Flexibility", "value": 80}, {"axis": "Scale", "value": 88}, {"axis": "DX", "value": 92}]}},
    {"slide_type": "funnel_chart", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "User Conversion Funnel", "steps": [
         {"label": "Impressions", "value": 100000}, {"label": "Sign-ups", "value": 15000}, {"label": "Activated", "value": 8500}, {"label": "Subscribed", "value": 2400}]}},

    # ═══ SECTION: Conversion & Call to Action ═══
    {"section": "SECTION 7 — Conversion & CTAs"},
    {"slide_type": "qr_destination", "theme": "bold", "bg_style": "dark", "archetype": "startup_pitch",
     "params": {"title": "Scan to Try SlideForge Live", "heading": "Instant Interactive Demo", "destination_url": "https://slideforge.dev/demo", "short_url": "slideforge.dev/demo", "cta_text": "Scan QR Code for Instant Access"}},
    {"slide_type": "cta", "theme": "editorial", "bg_style": "light", "archetype": "thought_leader",
     "params": {"headline": "Start Building Presentations in Rust Today", "button_text": "Get Started Free", "subheadline": "Join thousands of developers automating slide creation."}},
]

# ── Step 1: Generate tokens ────────────────────────────────────────────
print("Step 1: Generating design tokens...")
run_cmd([BIN, "configure-design", "#5E5FE0",
         "--style", "editorial", "--preset", "expressive",
         "--output", TOKENS_FILE], "tokens")
print(f"  ✓ Tokens: {TOKENS_FILE}")

# ── Step 2: Generate all slides ────────────────────────────────────────
print(f"\nStep 2: Generating {len([s for s in SLIDES if 'slide_type' in s])} slides...")
compiled_slides = []
for i, entry in enumerate(SLIDES):
    if "section" in entry:
        print(f"\n  ── {entry['section']} ──")
        continue
    slide_type = entry["slide_type"]
    variant = entry.get("variant", "")
    numbered = entry.get("numbered", False)
    params = dict(entry["params"])
    if numbered:
        params["numbered"] = True
    try:
        slide_data = generate_slide(
            slide_type, TOKENS_FILE,
            entry["theme"], entry["bg_style"], entry["archetype"],
            params, variant
        )
        compiled_slides.append(slide_data)
        print(f"    ✓ {slide_type} ({variant or 'default'})")
    except Exception as e:
        print(f"    ✗ {slide_type}: {e}")

# ── Step 3: Write compiled slides ──────────────────────────────────────
with open(SLIDES_FILE, "w") as f:
    json.dump(compiled_slides, f, indent=2)

# ── Step 4: Render carousel ────────────────────────────────────────────
print(f"\nStep 3: Rendering carousel with {len(compiled_slides)} slides...")
run_cmd([BIN, "render-carousel", SLIDES_FILE,
         "--tokens-file", TOKENS_FILE,
         "--output", CAROUSEL_PATH], "carousel")
print(f"  ✓ Carousel: {CAROUSEL_PATH}")

# ── Step 5: Export PNGs ────────────────────────────────────────────────
print(f"\nStep 4: Exporting PNGs...")
run_cmd([BIN, "export", CAROUSEL_PATH,
         "--slides", str(len(compiled_slides)),
         "--output-dir", OUTPUT_DIR], "export")
print(f"  ✓ PNGs: {OUTPUT_DIR}/")

print(f"\n✅ Full-scope test complete! {len(compiled_slides)} slides generated.")
