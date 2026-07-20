#!/usr/bin/env python3
import os
import json
import subprocess

# Define configuration paths
WORKSPACE_DIR = "/home/ishanp/Documents/GitHub/MY-PROJECTS/MCP-AND-CLIS/slideforge-rust"
TOKENS_FILE = os.path.join(WORKSPACE_DIR, "design_tokens.json")
SLIDES_JSON_FILE = os.path.join(WORKSPACE_DIR, "habits_slides.json")
OUTPUT_HTML_FILE = os.path.join(WORKSPACE_DIR, "dist", "habits_carousel.html")

# Create dist directory if it doesn't exist
os.makedirs(os.path.dirname(OUTPUT_HTML_FILE), exist_ok=True)

# Binary path (using target/debug/slideforge-rust built earlier)
SLIDEFORGE_BIN = os.path.join(WORKSPACE_DIR, "target/debug/slideforge-rust")

def run_cmd(args):
    print(f"Running command: {' '.join(args)}")
    result = subprocess.run(args, capture_output=True, text=True, cwd=WORKSPACE_DIR)
    if result.returncode != 0:
        print(f"Error executing command: {result.stderr}")
        raise RuntimeError(result.stderr)
    return result.stdout

def main():
    # 1. Configure design with #00FFCC
    # Using technical style (Space Grotesk) and vibrant preset for professional neon accent feel
    run_cmd([
        SLIDEFORGE_BIN, "configure-design", "#00FFCC",
        "--style", "technical",
        "--preset", "vibrant",
        "--output", TOKENS_FILE
    ])

    # 2. Define parameters for all 5 slides
    slides_definition = [
        # Slide 1: Hero Hook
        {
            "type": "hero",
            "theme": "dark",
            "bg_style": "dark",
            "params": {
                "headline": "3 Habits of Exceptional Developers",
                "subheadline": "Write cleaner, more maintainable code with these simple daily workflow shifts.",
                "badge": "CODING HABITS",
                "variant": "gradient"
            }
        },
        # Slide 2: Myth vs Fact (Habit 1: Write Simple Code)
        {
            "type": "myth_fact",
            "theme": "dark",
            "bg_style": "dark",
            "params": {
                "myth": "Exceptional developers write complex, clever code.",
                "fact": "Exceptional developers write simple, readable code.",
                "explanation": "Clever code is hard to read, debug, and maintain. Always prioritize clarity over cleverness."
            }
        },
        # Slide 3: Checklist (Habit 2: Boy Scout Rule)
        {
            "type": "checklist_action_plan",
            "theme": "dark",
            "bg_style": "dark",
            "params": {
                "title": "The Boy Scout Rule Checklist",
                "items": [
                    {"label": "Rename vague or ambiguous variables"},
                    {"label": "Break complex conditions into named functions"},
                    {"label": "Delete dead code and outdated comments"},
                    {"label": "Write a unit test for the code you changed"}
                ],
                "variant": "checklist"
            }
        },
        # Slide 4: Problem-Solution (Habit 3: Tight Feedback Loops)
        {
            "type": "problem_solution",
            "theme": "dark",
            "bg_style": "dark",
            "params": {
                "title": "Short Feedback Loops",
                "problem": "Bugs found in production are 100x more expensive to fix.",
                "solution": "Verify assumptions continuously as you write code.",
                "proof_points": [
                    {"title": "Test-Driven Writing", "description": "Write code and tests in small, alternating increments."},
                    {"title": "Continuous Peer Review", "description": "Share early drafts to catch design issues before they merge."}
                ],
                "variant": "split"
            }
        },
        # Slide 5: Call to Action (Conclusion)
        {
            "type": "cta",
            "theme": "dark",
            "bg_style": "dark",
            "params": {
                "headline": "Start Improving Your Habits Today",
                "subheadline": "Which of these 3 habits will you focus on first? Share in the comments!",
                "button_text": "Save & Share",
                "variant": "centered"
            }
        }
    ]

    # Generate slides individually
    generated_slides = []
    for idx, slide in enumerate(slides_definition):
        print(f"Generating slide {idx+1}/{len(slides_definition)} ({slide['type']})...")
        
        # Serialize params to JSON string
        params_str = json.dumps(slide["params"])
        
        stdout = run_cmd([
            SLIDEFORGE_BIN, "generate-slide", slide["type"],
            "--tokens-file", TOKENS_FILE,
            "--theme", slide["theme"],
            "--bg-style", slide["bg_style"],
            "--archetype", "educator",
            "--params", params_str
        ])
        
        # Parse output JSON and append to slides list
        slide_spec = json.loads(stdout)
        generated_slides.append(slide_spec)

    # Save slides array to a temporary file
    with open(SLIDES_JSON_FILE, "w") as f:
        json.dump(generated_slides, f, indent=2)
    print(f"Saved slides JSON specs to {SLIDES_JSON_FILE}")

    # 3. Render full carousel HTML document
    run_cmd([
        SLIDEFORGE_BIN, "render-carousel", SLIDES_JSON_FILE,
        "--tokens-file", TOKENS_FILE,
        "--brand-name", "Codecraft Insights",
        "--brand-handle", "@codecraft",
        "--topic", "Coding Habits",
        "--url", "https://codecraft.dev",
        "--hashtags", "programming,cleancode,webdev,softwareengineering",
        "--platform", "instagram_portrait",
        "--aspect-ratio", "4:5",
        "--output", OUTPUT_HTML_FILE
    ])

    print(f"\nSuccess! Final rendered HTML carousel is saved at:\n{OUTPUT_HTML_FILE}")

if __name__ == "__main__":
    main()
