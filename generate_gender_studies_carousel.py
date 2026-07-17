#!/usr/bin/env python3
import json
import os
import subprocess
import sys

WORKSPACE_DIR = "/home/ishanp/Documents/GitHub/MY-PROJECTS/MCP-AND-CLIS/slideforge-rust"
SLIDEFORGE_BIN = os.path.join(WORKSPACE_DIR, "target/release/slideforge-rust")
TOKENS_FILE = os.path.join(WORKSPACE_DIR, "gender_studies_tokens.json")
SLIDES_JSON_FILE = os.path.join(WORKSPACE_DIR, "gender_studies_slides.json")
OUTPUT_HTML_FILE = os.path.join(WORKSPACE_DIR, "dist/gender_studies_carousel.html")
EXPORT_DIR = os.path.join(WORKSPACE_DIR, "dist/gender_studies_exports")

def run_cmd(args):
    print(f"Running command: {' '.join(args)}")
    result = subprocess.run(args, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"Command failed with code {result.returncode}")
        print(f"Stdout:\n{result.stdout}")
        print(f"Stderr:\n{result.stderr}")
        sys.exit(1)
    return result.stdout.strip()

def main():
    os.makedirs(os.path.dirname(OUTPUT_HTML_FILE), exist_ok=True)
    os.makedirs(EXPORT_DIR, exist_ok=True)

    # Step 1: Configure Design System (Color Palette & Archetype)
    # Using a Slate-Purple themed minimal style to reflect academic sociology
    run_cmd([
        SLIDEFORGE_BIN, "configure-design", "#6366F1", 
        "--style", "editorial", 
        "--preset", "expressive", 
        "--output", TOKENS_FILE
    ])

    slides = []

    # Slide 1: Hero
    slides.append({
        "slide_type": "hero",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "headline": "The Dialectics of Gender",
            "subheadline": "Why holding both feminist and men's studies frames in generative tension is critical for deep sociology.",
            "badge": "KOSMOS SOCIOLOGY",
            "variant": "gradient"
        }
    })

    # Slide 2: Definition
    slides.append({
        "slide_type": "definition",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "term": "Dialectical Tension",
            "definition": "The generative friction between partial maps of the same social territory (e.g., feminism and men's studies) without prematurely collapsing either into the other.",
            "context": "Sociological Methodology",
            "variant": "boxed"
        }
    })

    # Slide 3: Section Divider
    slides.append({
        "slide_type": "section_divider",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "The Empirical Landscape",
            "kicker": "UR QUADRANT: MEASURABLE OUTCOMES",
            "subtitle": "Examining observable behavioral outcomes, health statistics, and institutional disparities.",
            "variant": "minimal"
        }
    })

    # Slide 4: Stat Row
    slides.append({
        "slide_type": "stat_row",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "Extreme Disparities",
            "stats": [
                {"value": "95%", "label": "Workplace Deaths"},
                {"value": "93%", "label": "Incarcerated"},
                {"value": "70%", "label": "Rough Sleepers"}
            ],
            "variant": "cards"
        }
    })

    # Slide 5: Column Chart
    slides.append({
        "slide_type": "column_chart",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "The Higher Education Gap",
            "data": [
                {"label": "1970", "value": 58},
                {"label": "1980", "value": 50},
                {"label": "1990", "value": 46},
                {"label": "2000", "value": 44},
                {"label": "2010", "value": 43},
                {"label": "2020", "value": 41.5}
            ],
            "caption": "Percentage of US undergraduate students who are male (NCES).",
            "variant": "clean"
        }
    })

    # Slide 6: Metric Card
    slides.append({
        "slide_type": "metric_card",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "value": "3.5x",
            "metric": "3.5x",
            "label": "Male Suicide Multiplier",
            "trend": "Stable",
            "context": "Compared to female suicide rates across Western countries.",
            "variant": "compact"
        }
    })

    # Slide 7: Comparison Bars
    slides.append({
        "slide_type": "comparison_bars",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "The Suicide Gap: Dialectical Tension",
            "comparison": {
                "left": {
                    "label": "Mental Health (UL)",
                    "value": 25,
                    "unit": "%"
                },
                "right": {
                    "label": "Structural Stressors (LR)",
                    "value": 75,
                    "unit": "%"
                }
            },
            "description": "Comparing clinical individualism with structural stressors (custody loss, job loss, debt).",
            "variant": "split"
        }
    })

    # Slide 8: Section Divider
    slides.append({
        "slide_type": "section_divider",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "The Cultural Shadow",
            "kicker": "LL QUADRANT: SHADOW NARRATIVES",
            "subtitle": "How the empathy gap and hidden cultural biases shape collective consciousness.",
            "variant": "bold"
        }
    })

    # Slide 9: Grid Cards
    slides.append({
        "slide_type": "grid_cards",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "Four Pillars of Collective Bias",
            "cards": [
                {"title": "Empathy Gap", "description": "The visceral tendency to perceive female suffering as more urgent than male suffering."},
                {"title": "Male Disposability", "description": "Accepting male combat death, workplace injury, and overwork as background noise."},
                {"title": "Agency Asymmetry", "description": "Attributing men's failures to agency (their fault) and women's to structure (society's fault)."},
                {"title": "Temporal Deferral", "description": "The rhetorical rule that men's issues must wait until all women's issues are resolved."}
            ],
            "cols": 2
        }
    })

    # Slide 10: Before & After Story
    slides.append({
        "slide_type": "before_after_story",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "Agency Attribution Shift",
            "before": "Men's issues (homelessness, incarceration) are personal failures of character or 'toxic masculinity'.",
            "after": "Recognizing that these outcomes are systemic, requiring structural intervention rather than moral condemnation.",
            "metric": "Systemic Focus",
            "variant": "boxed"
        }
    })

    # Slide 11: Callout
    slides.append({
        "slide_type": "callout",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "Compassion Gatekeeping",
            "text": "Sympathy for men is often conditional on emotional performance ('men need to talk'). Unconditional compassion is a human right, not a reward for adopting specific therapeutic styles.",
            "icon": "⚠️",
            "variant": "warning"
        }
    })

    # Slide 12: Myth vs Fact
    slides.append({
        "slide_type": "myth_fact",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "myth": "Zero-Sum Equality",
            "fact": "Addressing men's issues does not detract from women's progress. Human suffering is non-zero-sum; structural health benefits all genders.",
            "explanation": "Societal systems are co-arising and deeply interdependent.",
            "variant": "standard"
        }
    })

    # Slide 13: List
    slides.append({
        "slide_type": "list",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "Institutional Friction: Family Courts",
            "items": [
                {"title": "Tender Years shadow: persistent maternal bias in courts."},
                {"title": "High legal cost and structural barriers to joint custody."},
                {"title": "Lack of legal enforcement for paternal visitation rights."},
                {"title": "Downstream developmental impact of severed father-child bonds."}
            ],
            "ordered": False,
            "columns": 1
        }
    })

    # Slide 14: Problem / Solution
    slides.append({
        "slide_type": "problem_solution",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "The Diagnostic Switch",
            "problem": "Treating a systemic 4-quadrant gender pattern using only single-quadrant cognitive or somatic tools.",
            "solution": "Identify which quadrant is holding the pattern back, and reform structural or cultural conditions.",
            "proof_points": [
                {"title": "Somatic Limit", "description": "A regulated nervous system will relapse if returned to a trauma-inducing environment."}
            ],
            "variant": "split"
        }
    })

    # Slide 15: Image Quote (Pexels photo: Man sitting alone)
    slides.append({
        "slide_type": "image_quote",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "image_url": "https://images.pexels.com/photos/568021/pexels-photo-568021.jpeg?auto=compress&cs=tinysrgb&w=800",
            "quote": "The empathy gap is not malice; it is the automatic background noise of a civilization that has historically required male disposability.",
            "author": "Warren Farrell",
            "role": "Sociological Synthesis"
        }
    })

    # Slide 16: Image Stat (Pexels photo: Heavy industry / construction site)
    slides.append({
        "slide_type": "image_stat",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "image_url": "https://images.pexels.com/photos/1210484/pexels-photo-1210484.jpeg?auto=compress&cs=tinysrgb&w=800",
            "stat_value": "95%",
            "stat_label": "Workplace Deaths are Male",
            "description": "In heavy extraction and dangerous physical labor, male disposability is written directly into economic infrastructure.",
            "layout": "overlay"
        }
    })

    # Slide 17: Case Study Result
    slides.append({
        "slide_type": "case_study_result",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "client": "APPG on Men & Boys",
            "challenge": "Addressing male educational underachievement and school dropouts.",
            "solution": "Systematic investigation of curriculum bias, boy-hostile pedagogy, and fatherless households.",
            "results": "Proposed comprehensive policy roadmap targeting structural reforms in primary education."
        }
    })

    # Slide 18: FAQ
    slides.append({
        "slide_type": "faq",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "Addressing Objections",
            "questions": [
                {"question": "Doesn't focusing on men dilute focus on women's oppression?", "answer": "No. Systems are co-arising. You cannot fix one half of a polarity by ignoring the other."},
                {"question": "Isn't patriarchy run by men for men?", "answer": "Patriarchy is a role system. While elites were male, average men served as expendable labor, protectors, and soldiers."}
            ]
        }
    })

    # Slide 19: Process Map (Replacing duplicate CTA)
    slides.append({
        "slide_type": "process_map",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "title": "Sociological Synthesis Workflow",
            "steps": [
                {"title": "Locate empirical disparities (UR quadrant data)."},
                {"title": "Map cultural shadow patterns (LL quadrant narratives)."},
                {"title": "Expose systemic polarities & agency asymmetries."},
                {"title": "Synthesize a 4-quadrant dialectical perspective."}
            ]
        }
    })

    # Slide 20: QR Destination
    slides.append({
        "slide_type": "qr_destination",
        "theme": "dark",
        "bg_style": "dark",
        "archetype": "educator",
        "params": {
            "heading": "Scan to Read the Model",
            "destination_url": "https://ishanparihar.com/gender-studies",
            "cta_text": "Scan to view",
            "short_url": "ishanparihar.com/gender-studies",
            "caption": "Download the Markdown sources, tetra-quadrantic map, and full bibliography.",
            "incentive_text": "Includes the 11-pattern cultural bias taxonomy"
        }
    })

    # Compile all slide definitions
    compiled_slides = []
    for idx, s in enumerate(slides):
        print(f"Generating slide {idx+1}/20 ({s['slide_type']})...")
        cmd = [
            SLIDEFORGE_BIN, "generate-slide", s["slide_type"],
            "--tokens-file", TOKENS_FILE,
            "--theme", s["theme"],
            "--bg-style", s["bg_style"],
            "--archetype", s["archetype"],
            "--params", json.dumps(s["params"])
        ]
        slide_html = run_cmd(cmd)
        
        # Load the slide from HTML block by parsing the generated script inside it
        slide_spec = json.loads(slide_html)
        compiled_slides.append(slide_spec)

    # Save to slides.json
    with open(SLIDES_JSON_FILE, "w") as f:
        json.dump(compiled_slides, f, indent=2)
    print(f"Saved slide specs to {SLIDES_JSON_FILE}")

    # Render Carousel
    run_cmd([
        SLIDEFORGE_BIN, "render-carousel", SLIDES_JSON_FILE,
        "--tokens-file", TOKENS_FILE,
        "--brand-name", "KosmOS Sociology",
        "--brand-handle", "@integral_kosmos",
        "--topic", "Gender Dialectics",
        "--url", "ishanparihar.com",
        "--hashtags", "sociology,genderstudies,mensstudies,feminism,dialectics",
        "--platform", "instagram_portrait",
        "--aspect-ratio", "4:5",
        "--output", OUTPUT_HTML_FILE
    ])
    print(f"Success! HTML Carousel saved to {OUTPUT_HTML_FILE}")

    # Export to PNGs
    run_cmd([
        SLIDEFORGE_BIN, "export", OUTPUT_HTML_FILE,
        "--slides", "20",
        "--output-dir", EXPORT_DIR,
        "--preset", "instagram_portrait"
    ])
    print(f"Success! PNG slides exported to {EXPORT_DIR}")

if __name__ == "__main__":
    main()
