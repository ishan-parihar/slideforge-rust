#!/usr/bin/env python3
"""Generate audit slides using the proper SlideForge pipeline.

Pipeline: configure-design → generate-slide (with --tokens-file) → render-carousel → export
This ensures slides get the correct corner elements, progress breadcrumbs,
Instagram-style frame, and proper 420x525 composition.
"""

import json
import subprocess
import sys
import os

WORKSPACE_DIR = os.path.dirname(os.path.abspath(__file__))
SLIDEFORGE_BIN = os.path.join(WORKSPACE_DIR, "target", "release", "slideforge-rust")

TOKENS_FILE = os.path.join(WORKSPACE_DIR, "audit_tokens.json")
SLIDES_JSON_FILE = os.path.join(WORKSPACE_DIR, "audit_slides.json")
OUTPUT_HTML_FILE = os.path.join(WORKSPACE_DIR, "dist", "audit_carousel.html")
EXPORT_DIR = os.path.join(WORKSPACE_DIR, "dist", "audit_exports")

SLIDES_DIR = os.path.join(WORKSPACE_DIR, "dist")


def run_cmd(cmd, desc=""):
    """Run a command and handle errors."""
    print(f"  → {' '.join(cmd[:3])}...")
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
    if result.returncode != 0:
        print(f"  ✗ FAILED: {desc or ' '.join(cmd[:3])}")
        print(f"    stderr: {result.stderr[:300]}")
        if result.stdout:
            print(f"    stdout: {result.stdout[:200]}")
        sys.exit(1)
    return result.stdout


# ─── Slide definitions ───
# Each slide uses the proper slide_type, theme, bg_style, archetype, and params
# that the SlideForge pipeline expects.

SLIDES = [
    # ══════════════════════════════════════════════════════════════
    # SECTION: Column Chart (Task 5-6 from plan)
    # ══════════════════════════════════════════════════════════════
    {
        "slide_type": "section_divider",
        "theme": "editorial",
        "bg_style": "dark",
        "archetype": "data_analyst",
        "params": {
            "kicker": "TASK 5-6",
            "title": "Column Chart — Single vs Grouped Series",
            "subtitle": "Registry schema + grouped bar layout",
        },
    },
    {
        "slide_type": "column_chart",
        "theme": "editorial",
        "bg_style": "light",
        "archetype": "data_analyst",
        "params": {
            "title": "Graduation Rates by Decade",
            "data": [
                {"label": "1970", "value": 58},
                {"label": "1980", "value": 62},
                {"label": "1990", "value": 71},
                {"label": "2000", "value": 78},
                {"label": "2010", "value": 85},
            ],
        },
    },
    {
        "slide_type": "column_chart",
        "theme": "editorial",
        "bg_style": "light",
        "archetype": "data_analyst",
        "params": {
            "title": "Graduation Rates by Gender",
            "data": [
                {
                    "label": "1970",
                    "series": [
                        {"name": "Men", "value": 58},
                        {"name": "Women", "value": 42},
                    ],
                },
                {
                    "label": "1980",
                    "series": [
                        {"name": "Men", "value": 60},
                        {"name": "Women", "value": 64},
                    ],
                },
                {
                    "label": "1990",
                    "series": [
                        {"name": "Men", "value": 68},
                        {"name": "Women", "value": 74},
                    ],
                },
                {
                    "label": "2000",
                    "series": [
                        {"name": "Men", "value": 75},
                        {"name": "Women", "value": 82},
                    ],
                },
                {
                    "label": "2010",
                    "series": [
                        {"name": "Men", "value": 81},
                        {"name": "Women", "value": 89},
                    ],
                },
            ],
        },
    },
    {
        "slide_type": "column_chart",
        "theme": "editorial",
        "bg_style": "dark",
        "archetype": "data_analyst",
        "params": {
            "title": "Employment by Sector (3-Series)",
            "data": [
                {
                    "label": "2020",
                    "series": [
                        {"name": "Tech", "value": 45},
                        {"name": "Health", "value": 30},
                        {"name": "Finance", "value": 25},
                    ],
                },
                {
                    "label": "2022",
                    "series": [
                        {"name": "Tech", "value": 52},
                        {"name": "Health", "value": 35},
                        {"name": "Finance", "value": 28},
                    ],
                },
                {
                    "label": "2024",
                    "series": [
                        {"name": "Tech", "value": 60},
                        {"name": "Health", "value": 40},
                        {"name": "Finance", "value": 30},
                    ],
                },
            ],
        },
    },
    # ══════════════════════════════════════════════════════════════
    # SECTION: Line/Area Chart (Task 7 from plan)
    # ══════════════════════════════════════════════════════════════
    {
        "slide_type": "section_divider",
        "theme": "editorial",
        "bg_style": "dark",
        "archetype": "data_analyst",
        "params": {
            "kicker": "TASK 7",
            "title": "Line Chart — Single vs Multi-Path SVG",
            "subtitle": "Multi-series SVG rendering with legends",
        },
    },
    {
        "slide_type": "chart",
        "theme": "editorial",
        "bg_style": "light",
        "archetype": "data_analyst",
        "params": {
            "title": "Revenue Growth (Single Series)",
            "chart_type": "line",
            "data": [
                {"label": "Q1", "value": 120},
                {"label": "Q2", "value": 180},
                {"label": "Q3", "value": 150},
                {"label": "Q4", "value": 220},
            ],
        },
    },
    {
        "slide_type": "chart",
        "theme": "editorial",
        "bg_style": "dark",
        "archetype": "data_analyst",
        "params": {
            "title": "Revenue by Product Line",
            "chart_type": "line",
            "data": [
                {
                    "label": "Q1",
                    "series": [
                        {"name": "Product A", "value": 120},
                        {"name": "Product B", "value": 80},
                        {"name": "Product C", "value": 45},
                    ],
                },
                {
                    "label": "Q2",
                    "series": [
                        {"name": "Product A", "value": 180},
                        {"name": "Product B", "value": 110},
                        {"name": "Product C", "value": 60},
                    ],
                },
                {
                    "label": "Q3",
                    "series": [
                        {"name": "Product A", "value": 150},
                        {"name": "Product B", "value": 130},
                        {"name": "Product C", "value": 75},
                    ],
                },
                {
                    "label": "Q4",
                    "series": [
                        {"name": "Product A", "value": 220},
                        {"name": "Product B", "value": 160},
                        {"name": "Product C", "value": 90},
                    ],
                },
            ],
        },
    },
    {
        "slide_type": "chart",
        "theme": "editorial",
        "bg_style": "light",
        "archetype": "data_analyst",
        "params": {
            "title": "Market Share Trends (Area)",
            "chart_type": "area",
            "data": [
                {
                    "label": "2020",
                    "series": [
                        {"name": "Alpha Corp", "value": 35},
                        {"name": "Beta Inc", "value": 28},
                    ],
                },
                {
                    "label": "2021",
                    "series": [
                        {"name": "Alpha Corp", "value": 40},
                        {"name": "Beta Inc", "value": 32},
                    ],
                },
                {
                    "label": "2022",
                    "series": [
                        {"name": "Alpha Corp", "value": 38},
                        {"name": "Beta Inc", "value": 35},
                    ],
                },
                {
                    "label": "2023",
                    "series": [
                        {"name": "Alpha Corp", "value": 45},
                        {"name": "Beta Inc", "value": 30},
                    ],
                },
            ],
        },
    },
    # ══════════════════════════════════════════════════════════════
    # SECTION: Grid Cards Overflow (Task 1 from plan)
    # ══════════════════════════════════════════════════════════════
    {
        "slide_type": "section_divider",
        "theme": "editorial",
        "bg_style": "dark",
        "archetype": "data_analyst",
        "params": {
            "kicker": "TASK 1",
            "title": "Grid Cards — Overflow Scaling",
            "subtitle": "Dynamic font/padding scaling for dense content",
        },
    },
    {
        "slide_type": "grid_cards",
        "theme": "editorial",
        "bg_style": "light",
        "archetype": "educator",
        "params": {
            "title": "Core Values",
            "cards": [
                {"title": "Innovation", "description": "Pushing boundaries with creative solutions."},
                {"title": "Integrity", "description": "Honest and transparent in all dealings."},
                {"title": "Impact", "description": "Making a measurable difference."},
            ],
            "variant": "3-col",
        },
    },
    {
        "slide_type": "grid_cards",
        "theme": "editorial",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "Detailed Platform Features",
            "cards": [
                {
                    "title": "Advanced Analytics Engine",
                    "description": "Our comprehensive analytics platform provides real-time data processing, predictive modeling, and interactive dashboards that help organizations make data-driven decisions with unprecedented accuracy and speed across multiple data sources and business units.",
                },
                {
                    "title": "Enterprise Security Suite",
                    "description": "Military-grade encryption, multi-factor authentication, role-based access control, audit logging, and compliance monitoring ensure your data remains protected against evolving threats while meeting regulatory requirements across global jurisdictions.",
                },
                {
                    "title": "Collaborative Workspace",
                    "description": "Real-time document editing, video conferencing, project management tools, and team communication channels create a seamless environment where distributed teams can collaborate effectively regardless of geographic location and time zone differences.",
                },
            ],
            "variant": "3-col",
        },
    },
    # ══════════════════════════════════════════════════════════════
    # SECTION: Myth vs Fact (Task 2 from plan)
    # ══════════════════════════════════════════════════════════════
    {
        "slide_type": "section_divider",
        "theme": "editorial",
        "bg_style": "dark",
        "archetype": "data_analyst",
        "params": {
            "kicker": "TASK 2",
            "title": "Myth vs Fact — Short Text Scaling",
            "subtitle": "+5px font boost when both myth and fact are short",
        },
    },
    {
        "slide_type": "myth_fact",
        "theme": "editorial",
        "bg_style": "light",
        "archetype": "educator",
        "params": {
            "myth": "You need to work 80-hour weeks to succeed as a founder",
            "fact": "Research shows founders who maintain work-life balance are 33% more likely to achieve sustainable growth and avoid burnout",
            "explanation": "Studies from Stanford and Harvard Business School consistently show that sustainable work patterns correlate with better long-term outcomes.",
        },
    },
    {
        "slide_type": "myth_fact",
        "theme": "editorial",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "myth": "Zero-Sum Game",
            "fact": "Suffering is non-zero-sum",
            "explanation": "Compassion expands rather than divides emotional resources.",
        },
    },
    # ══════════════════════════════════════════════════════════════
    # SECTION: List (Task 3 from plan)
    # ══════════════════════════════════════════════════════════════
    {
        "slide_type": "section_divider",
        "theme": "editorial",
        "bg_style": "dark",
        "archetype": "data_analyst",
        "params": {
            "kicker": "TASK 3",
            "title": "List — Raw String Fallback",
            "subtitle": "Both raw strings and structured objects render correctly",
        },
    },
    {
        "slide_type": "list",
        "theme": "editorial",
        "bg_style": "light",
        "archetype": "educator",
        "params": {
            "title": "Key Takeaways",
            "items": [
                "Multi-series charts enable side-by-side comparisons",
                "Raw strings render as proper bullet points",
                "Font scaling prevents text overflow on dense cards",
                "CTA validation ensures single-call-to-action decks",
            ],
            "variant": "bullet",
        },
    },
    {
        "slide_type": "list",
        "theme": "editorial",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "Implementation Checklist",
            "items": [
                {"label": "Registry schema update", "description": "Document multi-series parameter format"},
                {"label": "Grouped column renderer", "description": "Side-by-side bars with 6-color palette"},
                {"label": "Multi-path SVG line chart", "description": "Separate path per series with legend"},
                {"label": "Validator rules", "description": "Single CTA enforcement and warnings"},
            ],
            "variant": "checklist",
        },
    },
    # ══════════════════════════════════════════════════════════════
    # SECTION: Info-Dense Grid Cards (from info_dense_mockup.html)
    # ══════════════════════════════════════════════════════════════
    {
        "slide_type": "section_divider",
        "theme": "editorial",
        "bg_style": "dark",
        "archetype": "data_analyst",
        "params": {
            "kicker": "TASK 4",
            "title": "Info-Dense Slides — Grid Card Variants",
            "subtitle": "dense / compact / list-dense for information-rich content",
        },
    },
    {
        "slide_type": "grid_cards",
        "theme": "editorial",
        "bg_style": "light",
        "archetype": "educator",
        "params": {
            "title": "Platform Capabilities",
            "cards": [
                {"title": "Real-Time Analytics", "icon": "📊", "description": "Live dashboards with predictive insights and custom KPI tracking across all business units."},
                {"title": "Enterprise Security", "icon": "🔒", "description": "SOC 2 compliant with end-to-end encryption, MFA, and role-based access control."},
                {"title": "Team Collaboration", "icon": "🤝", "description": "Shared workspaces with real-time editing, comments, and version history."},
                {"title": "API Integration", "icon": "⚡", "description": "RESTful APIs with webhooks for seamless third-party app connectivity."},
            ],
            "variant": "dense",
        },
    },
    {
        "slide_type": "grid_cards",
        "theme": "editorial",
        "bg_style": "light",
        "archetype": "educator",
        "params": {
            "title": "Implementation Checklist",
            "cards": [
                {"title": "Schema Design", "description": "Define data models and relationships"},
                {"title": "API Layer", "description": "Build REST endpoints with auth"},
                {"title": "Frontend UI", "description": "Create responsive dashboards"},
                {"title": "Testing Suite", "description": "Unit, integration, and E2E tests"},
                {"title": "Deploy Pipeline", "description": "CI/CD with staging environments"},
                {"title": "Monitoring", "description": "Observability and alerting setup"},
            ],
            "variant": "compact",
        },
    },
    {
        "slide_type": "grid_cards",
        "theme": "editorial",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "Key Architecture Decisions",
            "cards": [
                {"title": "Event-Driven Architecture", "icon": "🔄", "description": "Decoupled microservices communicating through message queues for scalability."},
                {"title": "Edge Computing", "icon": "🌐", "description": "Processing at the edge reduces latency to under 50ms for global users."},
                {"title": "Data Mesh Pattern", "icon": "🔗", "description": "Domain-oriented data ownership with self-serve data infrastructure."},
                {"title": "Zero Trust Security", "icon": "🛡️", "description": "Never trust, always verify — every request authenticated and authorized."},
                {"title": 'Event Sourcing', "icon": "📝", "description": "Immutable event log enables complete audit trail and time-travel debugging."},
            ],
            "variant": "list-dense",
        },
    },
]


def main():
    os.makedirs(os.path.dirname(OUTPUT_HTML_FILE), exist_ok=True)
    os.makedirs(EXPORT_DIR, exist_ok=True)

    print("🔍 SlideForge Audit — Proper Pipeline")
    print(f"   Binary: {SLIDEFORGE_BIN}")
    print(f"   Tokens: {TOKENS_FILE}")
    print(f"   Output: {OUTPUT_HTML_FILE}")
    print()

    # Step 1: Configure design tokens
    print("Step 1: Configuring design tokens...")
    run_cmd([
        SLIDEFORGE_BIN, "configure-design", "#767CFF",
        "--style", "editorial",
        "--preset", "expressive",
        "--output", TOKENS_FILE,
    ], "configure-design")
    print("  ✓ Tokens generated")

    # Step 2: Generate each slide
    print("\nStep 2: Generating slides...")
    compiled_slides = []
    for i, slide in enumerate(SLIDES):
        slide_type = slide["slide_type"]
        name = slide["params"].get("title", slide_type)[:40]
        print(f"  [{i+1}/{len(SLIDES)}] {slide_type}: {name}")

        params_json = json.dumps(slide["params"])
        cmd = [
            SLIDEFORGE_BIN, "generate-slide", slide_type,
            "--tokens-file", TOKENS_FILE,
            "--theme", slide["theme"],
            "--bg-style", slide["bg_style"],
            "--archetype", slide["archetype"],
            "--params", params_json,
        ]

        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
        if result.returncode != 0:
            print(f"    ✗ FAILED: {result.stderr[:200]}")
            continue

        try:
            slide_data = json.loads(result.stdout)
            compiled_slides.append(slide_data)
            print(f"    ✓ OK")
        except json.JSONDecodeError:
            print(f"    ✗ Failed to parse JSON output")
            continue

    # Save compiled slides
    with open(SLIDES_JSON_FILE, "w") as f:
        json.dump(compiled_slides, f, indent=2)
    print(f"\n  ✓ {len(compiled_slides)} slides compiled → {SLIDES_JSON_FILE}")

    # Step 3: Render carousel
    print("\nStep 3: Rendering carousel...")
    run_cmd([
        SLIDEFORGE_BIN, "render-carousel", SLIDES_JSON_FILE,
        "--tokens-file", TOKENS_FILE,
        "--brand-name", "SlideForge Audit",
        "--brand-handle", "@slideforge",
        "--topic", "Layout & Dataviz Upgrade",
        "--url", "slideforge.dev",
        "--hashtags", "slideforge,data-viz,column-chart,line-chart",
        "--platform", "instagram_portrait",
        "--aspect-ratio", "4:5",
        "--show-progress",
        "--output", OUTPUT_HTML_FILE,
    ], "render-carousel")
    print(f"  ✓ Carousel rendered → {OUTPUT_HTML_FILE}")

    # Step 4: Export PNGs
    print("\nStep 4: Exporting PNGs...")
    slide_count = str(len(compiled_slides))
    run_cmd([
        SLIDEFORGE_BIN, "export", OUTPUT_HTML_FILE,
        "--slides", slide_count,
        "--output-dir", EXPORT_DIR,
        "--preset", "instagram_portrait",
    ], "export")
    print(f"  ✓ PNGs exported → {EXPORT_DIR}/")

    # Summary
    print("\n" + "=" * 60)
    print("✅ Audit generation complete!")
    print(f"   Carousel: {OUTPUT_HTML_FILE}")
    print(f"   PNGs:     {EXPORT_DIR}/")
    print(f"   Slides:   {len(compiled_slides)}")
    print("=" * 60)

    # List exported PNGs
    pngs = sorted([f for f in os.listdir(EXPORT_DIR) if f.endswith(".png")])
    print(f"\n🖼️  Exported PNGs ({len(pngs)}):")
    for png in pngs:
        size = os.path.getsize(os.path.join(EXPORT_DIR, png))
        print(f"   {png} ({size:,} bytes)")


if __name__ == "__main__":
    main()
