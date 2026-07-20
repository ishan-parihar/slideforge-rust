#!/usr/bin/env python3
"""
Full-scope SlideForge test: every slide type × style variations × overflow stress-testing.

Generates a single carousel with ALL slide types in light/dark themes,
including intentionally long text to stress-test overflow containment.

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

# ── Short text (normal) ────────────────────────────────────────────────
SHORT_MYTH = "Breakfast is the most important meal of the day."
SHORT_FACT = "Studies show no significant difference in weight between breakfast eaters and skippers."
SHORT_EXPL = "The breakfast myth was largely popularized by cereal companies."

# ── Long text (overflow stress) ───────────────────────────────────────
LONG_MYTH = "Breakfast is universally recognized as the single most important meal of the day, and rigorous scientific consensus has established that anyone who skips breakfast will inevitably gain significant weight, suffer from decreased cognitive function, experience dangerously low blood sugar levels, and potentially develop long-term metabolic disorders that could have been entirely prevented."
LONG_FACT = "Despite decades of conventional wisdom suggesting otherwise, a growing body of peer-reviewed research has consistently found no significant correlation between breakfast consumption and weight management, cognitive performance, or metabolic health in adults, with some studies even suggesting that intermittent fasting including breakfast skipping may offer certain health benefits."
LONG_EXPL = "The widespread belief in breakfast importance can be traced back to aggressive marketing campaigns by cereal manufacturers in the early twentieth century, particularly James Caleb Jackson and John Harvey Kellogg, who had both financial and ideological motivations."

# ── Slide definitions ─────────────────────────────────────────────────
SLIDES = [
    # ═══ SECTION: Hero ═══
    {"section": "SECTION 1 — Hero Slides"},
    {"slide_type": "section_divider", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"kicker": "SECTION 1", "title": "Hero Slides", "subtitle": "Full-scope layout verification"}},
    {"slide_type": "hero", "theme": "bold", "bg_style": "dark", "archetype": "thought_leader",
     "params": {"headline": "Building the Future of Slide Design", "subheadline": "A comprehensive system for beautiful, data-driven presentations.", "badge": "SlideForge v0.3"}},
    {"slide_type": "hero", "theme": "editorial", "bg_style": "light", "archetype": "educator",
     "variant": "centered",
     "params": {"headline": "Visual Storytelling at Scale", "subheadline": "From data analyst to brand storyteller — every archetype covered."}},

    # ═══ SECTION: Feature ═══
    {"section": "SECTION 2 — Feature Slides"},
    {"slide_type": "feature", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"icon": "📊", "title": "Data-Driven Decisions", "description": "Real-time analytics dashboards with predictive modeling.", "number": "01"}},
    {"slide_type": "feature", "theme": "bold", "bg_style": "dark", "archetype": "startup_pitch",
     "variant": "icon-left",
     "params": {"icon": "🚀", "title": "Lightning Fast Rendering", "description": "Generate complete slide decks in under 2 seconds.", "number": ""}},

    # ═══ SECTION: List ═══
    {"section": "SECTION 3 — List Slides"},
    {"slide_type": "list", "theme": "editorial", "bg_style": "light", "archetype": "educator",
     "params": {"title": "Key Benefits", "items": [
         {"label": "Rapid prototyping", "sub": "Go from idea to deck in minutes"},
         {"label": "Data visualization", "sub": "Charts, graphs, and infographics built-in"},
         {"label": "Multi-platform export", "sub": "Instagram, TikTok, LinkedIn, and more"},
         {"label": "AI-powered content", "sub": "Generate and refine with natural language"}]}},
    {"slide_type": "list", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "variant": "card", "numbered": True,
     "params": {"title": "Technical Architecture", "items": [
         {"label": "Rust core engine"}, {"label": "HTML/CSS composition"},
         {"label": "MCP protocol integration"}, {"label": "Chromium headless export"}]}},

    # ═══ SECTION: Quote ═══
    {"section": "SECTION 4 — Quote Slides"},
    {"slide_type": "quote", "theme": "editorial", "bg_style": "light", "archetype": "thought_leader",
     "params": {"quote": "Design is not just what it looks like and feels like. Design is how it works.", "author": "Steve Jobs", "role": "Co-founder, Apple"}},

    # ═══ SECTION: Grid Cards ═══
    {"section": "SECTION 5 — Grid Cards"},
    {"slide_type": "grid_cards", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "variant": "dense",
     "params": {"title": "Platform Capabilities", "cards": [
         {"icon": "📊", "title": "Analytics", "description": "Real-time dashboards"},
         {"icon": "🔒", "title": "Security", "description": "End-to-end encryption"},
         {"icon": "🤝", "title": "Collaboration", "description": "Team workspaces"},
         {"icon": "⚡", "title": "Performance", "description": "Sub-second responses"}]}},
    {"slide_type": "grid_cards", "theme": "bold", "bg_style": "dark", "archetype": "startup_pitch",
     "variant": "compact",
     "params": {"title": "Feature Roadmap", "cards": [
         {"icon": "1", "title": "Core Engine", "description": "Rust-based rendering"},
         {"icon": "2", "title": "MCP Server", "description": "Tool protocol integration"},
         {"icon": "3", "title": "Export Pipeline", "description": "PNG/PDF/carousel output"},
         {"icon": "4", "title": "AI Assistant", "description": "Natural language creation"}]}},
    {"slide_type": "grid_cards", "theme": "editorial", "bg_style": "light", "archetype": "educator",
     "variant": "list-dense",
     "params": {"title": "Research Methodology", "cards": [
         {"icon": "🔍", "title": "Literature Review", "description": "200+ papers analyzed"},
         {"icon": "📋", "title": "Survey Design", "description": "2400 participants"},
         {"icon": "🧪", "title": "Controlled Trials", "description": "Double-blind experiments"},
         {"icon": "📈", "title": "Statistical Modeling", "description": "Bayesian inference"},
         {"icon": "✅", "title": "Peer Review", "description": "External validation"}]}},

    # ═══ SECTION: Grid Cards — Overflow Stress ═══
    {"section": "SECTION 6 — Grid Cards Overflow Stress"},
    {"slide_type": "grid_cards", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "variant": "dense",
     "params": {"title": "Detailed Platform Features", "cards": [
         {"icon": "📊", "title": "Advanced Analytics Engine", "description": "Real-time data processing, predictive modeling, interactive dashboards that help organizations make data-driven decisions with unprecedented accuracy."},
         {"icon": "🔒", "title": "Enterprise Security Suite", "description": "Military-grade encryption, multi-factor authentication, role-based access control, audit logging, compliance monitoring for evolving threats."},
         {"icon": "🤝", "title": "Collaborative Workspace", "description": "Real-time document editing, video conferencing, project management, team communication channels for distributed teams."},
         {"icon": "⚡", "title": "API Gateway", "description": "RESTful and GraphQL APIs, webhook management, third-party service connectors, rate limiting, comprehensive developer docs."}]}},

    # ═══ SECTION: Myth-Fact (debunk only) ═══
    {"section": "SECTION 7 — Myth vs Fact Debunk"},
    {"slide_type": "myth_fact", "theme": "editorial", "bg_style": "light", "archetype": "educator",
     "variant": "debunk",
     "params": {"myth": SHORT_MYTH, "fact": SHORT_FACT, "explanation": SHORT_EXPL}},

    # ═══ SECTION: Column Chart ═══
    {"section": "SECTION 8 — Column Charts"},
    {"slide_type": "column_chart", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "params": {"title": "Graduation Rates by Decade", "data": [
         {"label": "1970", "value": 58}, {"label": "1980", "value": 62},
         {"label": "1990", "value": 71}, {"label": "2000", "value": 78}, {"label": "2010", "value": 85}]}},
    {"slide_type": "column_chart", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Employment by Sector", "data": [
         {"label": "2020", "series": [{"name": "Tech", "value": 45}, {"name": "Health", "value": 30}, {"name": "Finance", "value": 25}]},
         {"label": "2022", "series": [{"name": "Tech", "value": 52}, {"name": "Health", "value": 35}, {"name": "Finance", "value": 28}]},
         {"label": "2024", "series": [{"name": "Tech", "value": 60}, {"name": "Health", "value": 40}, {"name": "Finance", "value": 30}]}]}},

    # ═══ SECTION: Line/Area Charts ═══
    {"section": "SECTION 9 — Line & Area Charts"},
    {"slide_type": "chart", "theme": "editorial", "bg_style": "light", "archetype": "data_analyst",
     "variant": "line",
     "params": {"title": "Revenue Growth", "chart_type": "line", "data": [
         {"label": "Q1", "value": 120}, {"label": "Q2", "value": 190},
         {"label": "Q3", "value": 160}, {"label": "Q4", "value": 220}]}},
    {"slide_type": "chart", "theme": "bold", "bg_style": "dark", "archetype": "data_analyst",
     "variant": "area",
     "params": {"title": "Market Share Trends", "chart_type": "area", "data": [
         {"label": "Q1", "series": [{"name": "Alpha Corp", "value": 45}, {"name": "Beta Inc", "value": 30}]},
         {"label": "Q2", "series": [{"name": "Alpha Corp", "value": 55}, {"name": "Beta Inc", "value": 35}]},
         {"label": "Q3", "series": [{"name": "Alpha Corp", "value": 60}, {"name": "Beta Inc", "value": 25}]},
         {"label": "Q4", "series": [{"name": "Alpha Corp", "value": 70}, {"name": "Beta Inc", "value": 20}]}]}},

    # ═══ SECTION: Overflow Stress — All Types ═══
    {"section": "SECTION 10 — Overflow Stress All Types"},
    {"slide_type": "feature", "theme": "editorial", "bg_style": "light", "archetype": "educator",
     "params": {"icon": "🔥", "title": "Extremely Long Feature Title That Tests Wrapping Behavior In The Component Layout System", "description": "This is an intentionally long description designed to test whether the feature slide properly handles text overflow. The content should wrap gracefully within the available space without clipping or extending beyond the slide boundaries.", "number": "42"}},
    {"slide_type": "quote", "theme": "editorial", "bg_style": "light", "archetype": "thought_leader",
     "params": {"quote": "This is an extremely long quote designed to test whether the quote slide properly handles text overflow. The quote should scale down gracefully and remain readable without clipping or extending beyond the slide boundaries. Good design means content always fits.", "author": "Test Author Name", "role": "QA Engineer, SlideForge"}},
    {"slide_type": "hero", "theme": "bold", "bg_style": "dark", "archetype": "thought_leader",
     "params": {"headline": "This Is An Extremely Long Headline That Tests Whether The Hero Slide Handles Text Overflow Gracefully Within Its Composition Bounds", "subheadline": "An equally long subheadline that pushes the boundaries of the hero layout to ensure everything stays within the 420x525 composition without any visual clipping or layout distortion."}},
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
