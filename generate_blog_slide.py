#!/usr/bin/env python3
import os
import json
import subprocess

WORKSPACE_DIR = "/home/ishanp/Documents/GitHub/MY-PROJECTS/MCP-AND-CLIS/slideforge-rust"
TOKENS_FILE = os.path.join(WORKSPACE_DIR, "blog_design_tokens.json")
SLIDES_JSON_FILE = os.path.join(WORKSPACE_DIR, "blog_slides.json")
OUTPUT_HTML_FILE = os.path.join(WORKSPACE_DIR, "dist", "blog_carousel.html")
OUTPUT_PNG_DIR = os.path.join(WORKSPACE_DIR, "dist", "blog_exports")

# Ensure output directories exist
os.makedirs(os.path.dirname(OUTPUT_HTML_FILE), exist_ok=True)
os.makedirs(OUTPUT_PNG_DIR, exist_ok=True)

# Binary path
SLIDEFORGE_BIN = os.path.join(WORKSPACE_DIR, "target/release/slideforge-rust")

def run_cmd(args):
    print(f"Running command: {' '.join(args)}")
    result = subprocess.run(args, capture_output=True, text=True, cwd=WORKSPACE_DIR)
    if result.returncode != 0:
        print(f"Error: {result.stderr}")
        raise RuntimeError(result.stderr)
    return result.stdout

def main():
    # 1. Configure design with Indigo Accent (#6366F1), dark theme, expressive preset
    run_cmd([
        SLIDEFORGE_BIN, "configure-design", "#6366F1",
        "--style", "technical",
        "--preset", "expressive",
        "--output", TOKENS_FILE
    ])

    # 2. Define the slides array
    slides = [
        # Slide 1: Hero Hook
        {
            "slide_type": "hero",
            "params": {
                "headline": "Trauma Lives in All 4 Quadrants",
                "subheadline": "Why nervous-system regulation alone is only 25% of the healing equation.",
                "badge": "INTEGRAL SYSTEMS",
                "variant": "gradient"
            }
        },
        # Slide 2: Grid Cards (The Four Quadrants)
        {
            "slide_type": "grid_cards",
            "params": {
                "title": "The Four Quadrants of Trauma",
                "variant": "2-col",
                "cards": [
                    {
                        "icon": "🧠",
                        "title": "Subjective (UL)",
                        "description": "The felt sense, internal parts narrative, and meaning made of the experience."
                    },
                    {
                        "icon": "🧬",
                        "title": "Objective (UR)",
                        "description": "Nervous system dysregulation, hypervigilance, and somatic freeze responses."
                    },
                    {
                        "icon": "🗣️",
                        "title": "Cultural (LL)",
                        "description": "Shared linguistic containers, cultural taboos, and validating communities."
                    },
                    {
                        "icon": "⚖️",
                        "title": "Structural (LR)",
                        "description": "Socioeconomic precarity, institutional betrayal, and environmental triggers."
                    }
                ]
            }
        },
        # Slide 3: Problem/Solution Diagnosis
        {
            "slide_type": "problem_solution",
            "params": {
                "title": "The Diagnostic Switch",
                "problem": "Treating a 4-quadrant systemic pattern using only single-quadrant somatic or cognitive tools.",
                "solution": "Identify which quadrant is holding the pattern back, and change structural or cultural conditions.",
                "proof_points": [
                    {
                        "title": "Somatic Limit",
                        "description": "A regulated nervous system will relapse if returned to trauma-inducing environment."
                    }
                ],
                "variant": "split"
            }
        }
    ]

    # Generate slides individually
    generated_slides = []
    for idx, s in enumerate(slides):
        print(f"Generating slide {idx+1}/{len(slides)} ({s['slide_type']})...")
        res_json = run_cmd([
            SLIDEFORGE_BIN, "generate-slide", s["slide_type"],
            "--tokens-file", TOKENS_FILE,
            "--theme", "dark",
            "--bg-style", "dark",
            "--archetype", "educator",
            "--params", json.dumps(s["params"])
        ])
        # Parse the JSON response
        slide_spec = json.loads(res_json)
        generated_slides.append(slide_spec)

    # Save to blog_slides.json
    with open(SLIDES_JSON_FILE, "w") as f:
        json.dump(generated_slides, f, indent=2)
    print(f"Saved slide specs to {SLIDES_JSON_FILE}")

    # 3. Render the carousel
    run_cmd([
        SLIDEFORGE_BIN, "render-carousel", SLIDES_JSON_FILE,
        "--tokens-file", TOKENS_FILE,
        "--brand-name", "Ishan Parihar",
        "--brand-handle", "@integralishan",
        "--topic", "Trauma Systems",
        "--url", "https://ishanparihar.com",
        "--hashtags", "holoos,trauma,integral,healing",
        "--platform", "instagram_portrait",
        "--aspect-ratio", "4:5",
        "--output", OUTPUT_HTML_FILE
    ])
    print(f"Success! HTML Carousel saved to {OUTPUT_HTML_FILE}")

    # 4. Export to PNGs
    run_cmd([
        SLIDEFORGE_BIN, "export", OUTPUT_HTML_FILE,
        "--slides", str(len(slides)),
        "--output-dir", OUTPUT_PNG_DIR,
        "--preset", "instagram_portrait"
    ])
    print(f"Success! PNG slides exported to {OUTPUT_PNG_DIR}")

if __name__ == "__main__":
    main()
