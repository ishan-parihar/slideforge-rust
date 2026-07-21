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

    # ═══ SECTION: Hero & Social Proof ═══
    {"section": "SECTION 8 — Hero & Social Proof"},
    {"slide_type": "testimonial_avatar", "theme": "editorial", "bg_style": "dark", "archetype": "thought_leader",
     "params": {"quote": "The cleanest slide rendering pipeline I've ever integrated.", "author": "Alex Rivera", "role": "Lead Architect", "avatar_url": "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=150"}},
    {"slide_type": "logo_cloud", "theme": "editorial", "bg_style": "light", "archetype": "thought_leader",
     "params": {"title": "Trusted by Engineering & Marketing Leaders", "logos": [
         {"name": "Rust Core", "logo_url": "https://images.unsplash.com/photo-1618401471353-b98afee0b2eb?w=120"},
         {"name": "Tokio Async", "logo_url": "https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=120"},
         {"name": "Wasm Tech", "logo_url": "https://images.unsplash.com/photo-1526374965328-7f61d4dc18c5?w=120"},
         {"name": "Cargo Crates", "logo_url": "https://images.unsplash.com/photo-1518770660439-4636190af475?w=120"},
         {"name": "Serde Engine", "logo_url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f?w=120"},
         {"name": "Raycast CLI", "logo_url": "https://images.unsplash.com/photo-1519389950473-47ba0277781c?w=120"}
     ]}},

    # ═══ SECTION: Comparisons & Tables ═══
    {"section": "SECTION 9 — Comparisons & Tables"},
    {"slide_type": "table", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "System Performance Benchmarks", "headers": ["Metric", "v1.0", "v2.0", "Improvement"], "rows": [["Compilation", "45ms", "8ms", "5.6x"], ["Memory Usage", "120MB", "24MB", "5.0x"], ["PNG Export", "1.2s", "0.3s", "4.0x"]]}},
    {"slide_type": "pricing_plan", "theme": "bold", "bg_style": "light", "archetype": "startup_pitch",
     "params": {"title": "Simple Transparent Pricing", "plans": [{"name": "Starter", "price": "$0", "period": "/mo", "features": ["100 slides/mo", "Standard templates", "Community support"]}, {"name": "Pro", "price": "$29", "period": "/mo", "features": ["Unlimited slides", "Custom brand tokens", "Priority MCP API", "Headless PNG export"]}]}},
    {"slide_type": "before_after_story", "theme": "editorial", "bg_style": "dark", "archetype": "educator",
     "params": {"title": "Workflow Transformation", "before": "Manual design tweaking, inconsistent margins, and hours spent formatting.", "after": "Instant JSON compilation, token-governed styles, and automated PNG export.", "metric": "10x Faster Output"}},

    # ═══ SECTION: Content & Structural Layouts ═══
    {"section": "SECTION 10 — Content & Structural Layouts"},
    {"slide_type": "timeline", "theme": "editorial", "bg_style": "light", "archetype": "educator",
     "params": {"title": "Product Evolution Timeline", "steps": [{"title": "Q1 2024", "description": "Core Engine compilation pipeline"}, {"title": "Q2 2024", "description": "Native MCP server integration"}, {"title": "Q3 2024", "description": "Headless PNG rasterizer"}]}},
    {"slide_type": "process_map", "theme": "bold", "bg_style": "dark", "archetype": "educator",
     "params": {"title": "3-Step Carousel Creation", "steps": [{"number": "01", "title": "Define Tokens", "description": "Set brand primary color & fonts"}, {"number": "02", "title": "Compile Specs", "description": "Generate slide HTML AST"}, {"number": "03", "title": "Export Media", "description": "Render PNG/PDF assets"}]}},
    {"slide_type": "split_features", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Dual Core Pillars", "features": [{"title": "Performance Engine", "description": "Native Rust binary compiled with zero overhead."}, {"title": "Design Guard", "description": "Strict validator catches visual regressions automatically."}, {"title": "Native Rust Compilation", "description": "100% high throughput slide rendering engine."}]}},
    {"slide_type": "definition", "theme": "editorial", "bg_style": "light", "archetype": "educator",
     "params": {"term": "Slide Composition Engine", "phonetic": "/slīd kəm-pə-zi-shən/", "definition": "A deterministic layout synthesizer that converts structured JSON specs into pixel-perfect presentation slides under strict aspect constraints.", "context": "Static templates are replaced by dynamic layout engines that scale typography and enforce mathematical spacing rules at runtime."}},
    {"slide_type": "callout", "theme": "bold", "bg_style": "dark", "archetype": "thought_leader",
     "params": {"title": "Critical Architecture Note", "text": "Always validate layout bounds before exporting production PNG assets to guarantee zero visual regressions.", "icon": "💡"}},
    {"slide_type": "faq", "theme": "editorial", "bg_style": "light", "archetype": "educator",
     "params": {"title": "Frequently Asked Questions", "questions": [
         {"question": "Is SlideForge open source?", "answer": "Yes, SlideForge is built in Rust under the MIT license."},
         {"question": "Can I export to PDF & PNG?", "answer": "Yes, PDF and PNG rasterization are supported natively."},
         {"question": "How fast is slide compilation?", "answer": "SlideForge compiles a 10-slide carousel in under 10ms."},
         {"question": "Does it support custom themes?", "answer": "Yes, design tokens and color palettes are fully configurable."},
         {"question": "Is there MCP server integration?", "answer": "Yes, built-in MCP server for AI agent workflow automation."},
         {"question": "Does it handle text overflow?", "answer": "Yes, dynamic font scaling prevents body viewport overflows."}
     ]}},

    # ═══ SECTION: Single Metrics & Advanced Data ═══
    {"section": "SECTION 11 — Single Metrics & Advanced Data"},
    {"slide_type": "metric_grid", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Key Telemetry Metrics", "metrics": [{"value": "10ms", "label": "Compile Latency"}, {"value": "85+", "label": "Unit Tests"}, {"value": "47", "label": "Slide Types"}, {"value": "100%", "label": "Token Compliance"}]}},
    {"slide_type": "case_study_result", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"client": "TechCorp Case Study", "challenge": "Manual deck creation was taking 15+ hours weekly per designer.", "solution": "Deployed SlideForge CLI & MCP server automation across team.", "results": [{"number": "80%", "label": "Time Saved"}, {"number": "5x", "label": "Output Increase"}]}},
    {"slide_type": "scatter_plot", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Latency vs Character Density Correlation", "x_label": "Character Mass", "y_label": "Latency (ms)", "data": [{"x": 100, "y": 2.1}, {"x": 250, "y": 3.4}, {"x": 500, "y": 5.2}, {"x": 800, "y": 7.8}]}},

    # ═══ SECTION: Image & Photographic Media ═══
    {"section": "SECTION 12 — Image & Photographic Media"},
    {"slide_type": "image_caption", "theme": "editorial", "bg_style": "dark", "archetype": "thought_leader",
     "params": {"image_url": "https://images.unsplash.com/photo-1451187580459-43490279c0fa?w=600", "caption": "Stock Photo Integration", "description": "Demonstrating background images and photo cards with stock/custom local paths."}},
    {"slide_type": "image_headline", "theme": "bold", "bg_style": "dark", "archetype": "thought_leader",
     "params": {"headline": "Visual Storytelling", "image_url": "https://images.unsplash.com/photo-1518770660439-4636190af475?w=600", "subheadline": "Overlay headlines over custom photographic assets."}},
    {"slide_type": "image_quote", "theme": "editorial", "bg_style": "dark", "archetype": "thought_leader",
     "params": {"quote": "Design is not just what it looks like. Design is how it works.", "author": "Steve Jobs", "image_url": "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=600"}},
    {"slide_type": "image_callout", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"image_url": "https://images.unsplash.com/photo-1526374965328-7f61d4dc18c5?w=600", "callouts": [{"label": "Core Renderer Module", "description": "Sub-millisecond WebAssembly and native compilation engine", "x": 50, "y": 40}], "description": "Native Rust Renderer Core architecture delivering high-performance slide synthesis."}},
    {"slide_type": "image_gallery", "theme": "editorial", "bg_style": "light", "archetype": "thought_leader",
     "params": {"title": "Multi-Image Showcase", "images": [
         {"url": "https://images.unsplash.com/photo-1519389950473-47ba0277781c?w=300", "caption": "AST Parser"},
         {"url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f?w=300", "caption": "Token Engine"},
         {"url": "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=300", "caption": "HTML Synthesizer"},
         {"url": "https://images.unsplash.com/photo-1550751827-4bd374c3f58b?w=300", "caption": "PNG Exporter"}
     ], "section_caption": "Architectural Benefits - High-density multi-image gallery showcasing compilation output assets."}},
    {"slide_type": "image_comparison", "theme": "editorial", "bg_style": "dark", "archetype": "educator",
     "params": {"title": "Native Rust Renderer Core", "before_image": "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=300", "after_image": "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=300", "before_label": "BEFORE", "after_label": "AFTER", "description": "Side-by-side photographic comparison of baseline renderer vs native Rust compilation engine."}},]

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
print(f"Step 2: Generating slides in primary and inverted themes...\n")
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

    # Primary theme slide
    slide_obj = generate_slide(stype, TOKENS_FILE, theme, bg, arch, params, variant=var)
    compiled_slides.append(slide_obj)

    # Inverted theme slide to test 100% theme contrast & stability
    inv_bg = "light" if bg == "dark" else "dark"
    inv_slide_obj = generate_slide(stype, TOKENS_FILE, theme, inv_bg, arch, params, variant=var)
    compiled_slides.append(inv_slide_obj)

    print(f"    ✓ {stype} ({var or 'default'}) [{bg} & {inv_bg}]")

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
