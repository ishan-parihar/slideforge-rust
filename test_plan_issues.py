#!/usr/bin/env python3
"""
Test script: Generate slides for ALL issues mentioned in the plan
docs/superpowers/plans/2026-07-17-layout-fixes-and-dataviz-upgrade.md

This covers:
- Task 1: Slide 9 Grid Card Overflow (long descriptions)
- Task 2: Slide 12 Myth vs Fact Text Scaling (short texts)
- Task 3: Slide 13 List Bullet Points (raw string fallback)
- Task 4: Carousel Validator & Single CTA Enforcement
- Task 5/6: Column Chart Multi-Series (grouped columns)
- Task 7: SVG Multi-Path Line Chart (multi-series)
"""

import json
import os
import subprocess
import sys

WORKSPACE_DIR = os.path.dirname(os.path.abspath(__file__))
SLIDEFORGE_BIN = os.path.join(WORKSPACE_DIR, "target/release/slideforge-rust")
TOKENS_FILE = os.path.join(WORKSPACE_DIR, "test_plan_tokens.json")
SLIDES_JSON_FILE = os.path.join(WORKSPACE_DIR, "test_plan_slides.json")
OUTPUT_HTML_FILE = os.path.join(WORKSPACE_DIR, "dist/test_plan_carousel.html")
EXPORT_DIR = os.path.join(WORKSPACE_DIR, "dist/test_plan_exports")


def run_cmd(args):
    print(f"  Running: {' '.join(args[:6])}...")
    try:
        result = subprocess.run(args, capture_output=True, text=True, timeout=60)
    except subprocess.TimeoutExpired:
        print(f"  TIMED OUT after 60s")
        sys.exit(1)
    if result.returncode != 0:
        print(f"  FAILED (exit {result.returncode})")
        print(f"  Stderr: {result.stderr[:500]}")
        sys.exit(1)
    return result.stdout.strip()


def generate_slide(slide_type, params, theme="dark", bg_style="dark", archetype="educator"):
    """Generate a single slide and return the JSON spec."""
    cmd = [
        SLIDEFORGE_BIN, "generate-slide", slide_type,
        "--tokens-file", TOKENS_FILE,
        "--theme", theme,
        "--bg-style", bg_style,
        "--archetype", archetype,
        "--params", json.dumps(params),
    ]
    raw = run_cmd(cmd)
    try:
        return json.loads(raw)
    except json.JSONDecodeError as e:
        print(f"  JSON parse error: {e}")
        print(f"  Raw output (first 200 chars): {raw[:200]}")
        sys.exit(1)


def main():
    os.makedirs(os.path.dirname(OUTPUT_HTML_FILE), exist_ok=True)
    os.makedirs(EXPORT_DIR, exist_ok=True)

    # Step 0: Configure design tokens
    print("\n=== Step 0: Configure Design Tokens ===")
    run_cmd([
        SLIDEFORGE_BIN, "configure-design", "#6366F1",
        "--style", "editorial",
        "--preset", "expressive",
        "--output", TOKENS_FILE,
    ])

    slides = []

    # ============================================================
    # HEADER SLIDE - Test overview
    # ============================================================
    print("\n=== Header: Test Overview ===")
    slides.append(generate_slide("hero", {
        "headline": "Plan Issue Test Carousel",
        "subheadline": "Visual verification for layout fixes & data viz upgrades (Tasks 1-7)",
        "badge": "SLIDEFORGE QA",
        "variant": "gradient",
    }))

    # ============================================================
    # TASK 1: Grid Cards with LONG descriptions (overflow test)
    # ============================================================
    print("\n=== Task 1: Grid Card Overflow (Long Descriptions) ===")
    slides.append(generate_slide("grid_cards", {
        "title": "Four Pillars of Collective Bias (Overflow Test)",
        "cards": [
            {
                "title": "Empathy Gap",
                "description": "The visceral tendency to perceive female suffering as more urgent than male suffering, leading to unequal allocation of social resources and compassion across gender lines in healthcare, criminal justice, and public policy."
            },
            {
                "title": "Male Disposability",
                "description": "Accepting male combat death, workplace injury, and overwork as background noise -- a cultural assumption so deeply embedded that challenging it is seen as controversial or reactionary."
            },
            {
                "title": "Agency Asymmetry",
                "description": "Attributing men's failures to agency (their fault) and women's to structure (society's fault), creating a double standard that blocks systemic analysis of male outcomes."
            },
            {
                "title": "Temporal Deferral",
                "description": "The rhetorical rule that men's issues must wait until all women's issues are resolved -- an indefinite deferral that effectively silences structural male disadvantage."
            },
        ],
        "cols": 2,
    }))

    # EXTREME descriptions (beyond 350 chars)
    print("=== Task 1b: Grid Card EXTREME Overflow ===")
    slides.append(generate_slide("grid_cards", {
        "title": "Extreme Overflow Test (350+ chars)",
        "cards": [
            {
                "title": "Crisis 1",
                "description": "This is an extremely long description designed to trigger the dense mode scaling in grid_cards_slide. It contains well over 80 characters to push the total character mass above the 350-character threshold, forcing the renderer to scale down paddings, gaps, and font sizes to prevent overflow beyond the 365px slide body bounds."
            },
            {
                "title": "Crisis 2",
                "description": "Similarly verbose description text here to add to the cumulative character count. The goal is to verify that the dynamic scaling system correctly compresses the grid layout when the total text mass exceeds the safe threshold without clipping or cutoff."
            },
            {
                "title": "Crisis 3",
                "description": "Yet another lengthy explanation to push the character count higher. Each card adds to the total mass, and the system must adapt gracefully by reducing font sizes and spacing to maintain readability within the fixed composition bounds."
            },
            {
                "title": "Crisis 4",
                "description": "Final verbose card description to complete the extreme overflow test. With four cards of this length, the total character count should significantly exceed 350 characters, triggering the most aggressive scaling tier in the grid layout system."
            },
        ],
        "cols": 2,
    }))

    # EDGE CASE: Exactly 240 chars boundary
    print("=== Task 1c: Grid Card 240-char Boundary ===")
    desc_a = "This description is carefully crafted to be exactly the right length to test the first scaling tier boundary. "  # ~110 chars
    desc_b = "The second card adds more text to approach the threshold. "  # ~57 chars
    desc_c = "A third card contributing characters toward the boundary. "  # ~57 chars
    desc_d = "Fourth and final card for the 240-char boundary test. "  # ~55 chars
    slides.append(generate_slide("grid_cards", {
        "title": "Boundary Test (~240 chars)",
        "cards": [
            {"title": "A", "description": desc_a},
            {"title": "B", "description": desc_b},
            {"title": "C", "description": desc_c},
            {"title": "D", "description": desc_d},
        ],
        "cols": 2,
    }))

    # ============================================================
    # TASK 2: Myth vs Fact text scaling
    # ============================================================
    print("\n=== Task 2: Myth vs Fact Short Text Scaling ===")
    slides.append(generate_slide("myth_fact", {
        "myth": "Zero-Sum Game",
        "fact": "Suffering is non-zero-sum",
        "explanation": "Both genders face distinct structural challenges.",
        "variant": "standard",
    }))

    # Normal-length texts (baseline)
    print("=== Task 2b: Myth vs Fact Normal Text (baseline) ===")
    slides.append(generate_slide("myth_fact", {
        "myth": "Men don't face real systemic disadvantage in education, health outcomes, or criminal sentencing -- any disparities are self-inflicted consequences of toxic masculinity.",
        "fact": "Documented disparities in educational attainment, suicide rates, workplace deaths, and sentencing lengths reflect structural patterns, not individual moral failures.",
        "explanation": "Structural analysis requires examining all quadrants of the gender landscape.",
        "variant": "standard",
    }))

    # EDGE CASE: Only ONE is short (should NOT trigger +5px scale-up)
    print("=== Task 2c: Myth vs Fact Only Myth Short ===")
    slides.append(generate_slide("myth_fact", {
        "myth": "Short myth",
        "fact": "This is a long fact that explains how structural patterns in education, health, and criminal justice create measurable disparities between genders that cannot be reduced to individual choices.",
        "explanation": "Only short myth should NOT trigger scaling.",
        "variant": "standard",
    }))

    # ============================================================
    # TASK 3: List with RAW STRING items (string fallback)
    # ============================================================
    print("\n=== Task 3: List Raw String Fallback ===")
    slides.append(generate_slide("list", {
        "title": "Raw String Items (Fallback Test)",
        "items": [
            "First raw string item -- should render as bullet point",
            "Second raw string item -- no object wrapping needed",
            "Third raw string item -- tests the string-check fallback",
            "Fourth raw string item -- all items should be visible",
        ],
        "ordered": False,
        "columns": 1,
    }))

    # Structured objects (baseline)
    print("=== Task 3b: List Structured Objects (baseline) ===")
    slides.append(generate_slide("list", {
        "title": "Structured Object Items (Baseline)",
        "items": [
            {"title": "First structured item with object format"},
            {"title": "Second structured item with object format"},
            {"title": "Third structured item with object format"},
            {"title": "Fourth structured item with object format"},
        ],
        "ordered": False,
        "columns": 1,
    }))

    # EDGE CASE: Mixed array with both raw strings AND objects
    print("=== Task 3c: List Mixed Items ===")
    slides.append(generate_slide("list", {
        "title": "Mixed Items (Strings + Objects)",
        "items": [
            "Raw string item mixed in",
            {"title": "Structured object item"},
            "Another raw string item",
            {"title": "Another structured object"},
        ],
        "ordered": False,
        "columns": 1,
    }))

    # ============================================================
    # TASK 4: CTA Validation
    # ============================================================
    print("\n=== Task 4: Single CTA Validation ===")
    slides.append(generate_slide("qr_destination", {
        "heading": "Learn More",
        "destination_url": "https://example.com/plan-test",
        "cta_text": "Scan to view",
        "short_url": "example.com/plan-test",
        "caption": "This is the only CTA slide in the deck.",
        "incentive_text": "Full test results",
    }))

    # ============================================================
    # TASK 5/6: Column Chart Multi-Series
    # ============================================================
    print("\n=== Task 6: Grouped Column Chart (Multi-Series) ===")
    slides.append(generate_slide("column_chart", {
        "title": "Gender Gap in Education (Grouped)",
        "data": [
            {
                "label": "1970",
                "series": [
                    {"name": "Male", "value": 58},
                    {"name": "Female", "value": 42},
                ],
            },
            {
                "label": "1990",
                "series": [
                    {"name": "Male", "value": 47},
                    {"name": "Female", "value": 53},
                ],
            },
            {
                "label": "2010",
                "series": [
                    {"name": "Male", "value": 43},
                    {"name": "Female", "value": 57},
                ],
            },
            {
                "label": "2020",
                "series": [
                    {"name": "Male", "value": 41},
                    {"name": "Female", "value": 59},
                ],
            },
        ],
        "caption": "Percentage of US undergraduate students by gender (NCES).",
        "variant": "clean",
    }))

    # Single-series (backward compat)
    print("=== Task 6b: Single Series Column Chart (Backward Compat) ===")
    slides.append(generate_slide("column_chart", {
        "title": "Single Series (Backward Compat)",
        "data": [
            {"label": "A", "value": 80},
            {"label": "B", "value": 60},
            {"label": "C", "value": 45},
            {"label": "D", "value": 90},
        ],
        "caption": "This should render as a standard single-color column chart.",
        "variant": "clean",
    }))

    # EDGE CASE: 3+ series
    print("=== Task 6c: Column Chart 3+ Series ===")
    slides.append(generate_slide("column_chart", {
        "title": "3-Series Column Chart",
        "data": [
            {
                "label": "Q1",
                "series": [
                    {"name": "Product A", "value": 40},
                    {"name": "Product B", "value": 30},
                    {"name": "Product C", "value": 25},
                ],
            },
            {
                "label": "Q2",
                "series": [
                    {"name": "Product A", "value": 50},
                    {"name": "Product B", "value": 35},
                    {"name": "Product C", "value": 30},
                ],
            },
            {
                "label": "Q3",
                "series": [
                    {"name": "Product A", "value": 45},
                    {"name": "Product B", "value": 40},
                    {"name": "Product C", "value": 35},
                ],
            },
        ],
        "caption": "Testing 6-color palette with 3 series.",
        "variant": "clean",
    }))

    # EDGE CASE: Zero values
    print("=== Task 6d: Column Chart Zero Values ===")
    slides.append(generate_slide("column_chart", {
        "title": "Zero Value Edge Case",
        "data": [
            {"label": "A", "value": 0},
            {"label": "B", "value": 50},
            {"label": "C", "value": 0},
            {"label": "D", "value": 100},
        ],
        "caption": "Testing zero-value bars (min-height guard).",
        "variant": "clean",
    }))

    # ============================================================
    # TASK 7: SVG Multi-Path Line Chart
    # ============================================================
    print("\n=== Task 7: Multi-Path SVG Line Chart ===")
    slides.append(generate_slide("chart", {
        "title": "Literacy Rate Trends (Multi-Series)",
        "chart_type": "line",
        "data": [
            {
                "label": "1980",
                "series": [
                    {"name": "Men", "value": 78},
                    {"name": "Women", "value": 72},
                ],
            },
            {
                "label": "1990",
                "series": [
                    {"name": "Men", "value": 83},
                    {"name": "Women", "value": 80},
                ],
            },
            {
                "label": "2000",
                "series": [
                    {"name": "Men", "value": 87},
                    {"name": "Women", "value": 86},
                ],
            },
            {
                "label": "2010",
                "series": [
                    {"name": "Men", "value": 90},
                    {"name": "Women", "value": 91},
                ],
            },
            {
                "label": "2020",
                "series": [
                    {"name": "Men", "value": 92},
                    {"name": "Women", "value": 93},
                ],
            },
        ],
        "variant": "line",
    }))

    # Single-series line chart (backward compat)
    print("=== Task 7b: Single Series Line Chart (Backward Compat) ===")
    slides.append(generate_slide("chart", {
        "title": "Single Series Line (Backward Compat)",
        "chart_type": "line",
        "data": [
            {"label": "Jan", "value": 30},
            {"label": "Feb", "value": 45},
            {"label": "Mar", "value": 38},
            {"label": "Apr", "value": 55},
            {"label": "May", "value": 48},
        ],
        "variant": "line",
    }))

    # EDGE CASE: Only 2 data points (minimum viable line)
    print("=== Task 7c: Line Chart 2 Data Points ===")
    slides.append(generate_slide("chart", {
        "title": "Minimum 2-Point Line",
        "chart_type": "line",
        "data": [
            {
                "label": "Start",
                "series": [
                    {"name": "Alpha", "value": 10},
                    {"name": "Beta", "value": 90},
                ],
            },
            {
                "label": "End",
                "series": [
                    {"name": "Alpha", "value": 90},
                    {"name": "Beta", "value": 10},
                ],
            },
        ],
        "variant": "line",
    }))

    # EDGE CASE: Different-length series
    print("=== Task 7d: Line Chart Different-Length Series ===")
    slides.append(generate_slide("chart", {
        "title": "Different-Length Series",
        "chart_type": "line",
        "data": [
            {
                "label": "2020",
                "series": [
                    {"name": "Short", "value": 50},
                ],
            },
            {
                "label": "2021",
                "series": [
                    {"name": "Short", "value": 60},
                    {"name": "Long", "value": 40},
                ],
            },
            {
                "label": "2022",
                "series": [
                    {"name": "Long", "value": 55},
                ],
            },
            {
                "label": "2023",
                "series": [
                    {"name": "Long", "value": 70},
                ],
            },
        ],
        "variant": "line",
    }))

    # ============================================================
    # FOOTER: Close the deck
    # ============================================================
    print("\n=== Footer: Close Deck ===")
    slides.append(generate_slide("callout", {
        "title": "Test Complete",
        "text": "All 7 plan issues have been rendered. Check the carousel for visual correctness of each slide type.",
        "icon": "✅",
        "variant": "info",
    }))

    # ============================================================
    # Save slides JSON
    # ============================================================
    print(f"\n=== Saving {len(slides)} slides to {SLIDES_JSON_FILE} ===")
    with open(SLIDES_JSON_FILE, "w") as f:
        json.dump(slides, f, indent=2)

    # ============================================================
    # Render carousel
    # ============================================================
    print("\n=== Rendering Carousel ===")
    run_cmd([
        SLIDEFORGE_BIN, "render-carousel", SLIDES_JSON_FILE,
        "--tokens-file", TOKENS_FILE,
        "--brand-name", "SlideForge QA",
        "--brand-handle", "@slideforge",
        "--topic", "Plan Issue Verification",
        "--url", "example.com",
        "--hashtags", "slideforge,qa,testing",
        "--platform", "instagram_portrait",
        "--aspect-ratio", "4:5",
        "--output", OUTPUT_HTML_FILE,
    ])
    print(f"\n✅ Carousel saved to: {OUTPUT_HTML_FILE}")

    # ============================================================
    # Validate design
    # ============================================================
    print("\n=== Running validate-design ===")
    result = subprocess.run(
        [SLIDEFORGE_BIN, "validate-design", OUTPUT_HTML_FILE],
        capture_output=True, text=True,
    )
    print(f"Validation output:\n{result.stdout}")
    if result.returncode != 0:
        print(f"Validation stderr:\n{result.stderr}")

    # ============================================================
    # Export to PNGs
    # ============================================================
    print("\n=== Exporting to PNGs ===")
    run_cmd([
        SLIDEFORGE_BIN, "export", OUTPUT_HTML_FILE,
        "--slides", str(len(slides)),
        "--output-dir", EXPORT_DIR,
        "--preset", "instagram_portrait",
    ])
    print(f"\n✅ PNGs exported to: {EXPORT_DIR}")
    print(f"\n🎉 Test complete! {len(slides)} slides generated, validated, and exported.")


if __name__ == "__main__":
    main()
