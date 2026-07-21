#!/usr/bin/env python3
"""
Generate test carousel HTML for each of the 28 campaign presets.

For each preset:
1. Generates design tokens with a unique primary color
2. Fills params_template with realistic test content
3. Generates each slide via `generate-slide` CLI
4. Assembles carousel via `render-carousel` CLI
5. Saves HTML to dist/presets/<preset_id>.html

Tests the full MCP-equivalent pipeline: configure-design → generate-slide → render-carousel.
"""

import json, os, subprocess, sys, random, hashlib

WORKSPACE_DIR = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(WORKSPACE_DIR, "dist", "slideforge-x86_64-linux-gnu")
if not os.path.exists(BIN):
    BIN = os.path.join(WORKSPACE_DIR, "target", "release", "slideforge-rust")

PRESETS_FILE = os.path.join(WORKSPACE_DIR, "docs", "presets", "campaign-presets.json")
OUTPUT_DIR = os.path.join(WORKSPACE_DIR, "dist", "presets")
TOKENS_DIR = os.path.join(OUTPUT_DIR, "tokens")

os.makedirs(OUTPUT_DIR, exist_ok=True)
os.makedirs(TOKENS_DIR, exist_ok=True)

# ── Color palette for preset diversity ──────────────────────────────────
PRESET_COLORS = [
    "#5E5FE0", "#E05252", "#2E7D32", "#F57C00", "#7B1FA2",
    "#00838F", "#C62828", "#283593", "#558B2F", "#D84315",
    "#4527A0", "#00695C", "#AD1457", "#1565C0", "#827717",
    "#6A1B9A", "#00897B", "#BF360C", "#311B92", "#1B5E20",
    "#880E4F", "#0D47A1", "#E65100", "#33691E", "#4A148C",
    "#006064", "#B71C1C", "#1A237E",
]

PRESET_THEMES = ["editorial", "bold", "minimal", "dark", "vibrant", "natural"]
PRESET_ARCHETYPES = ["educator", "thought_leader", "startup_pitch", "brand_storyteller", "data_analyst", "creator"]
ASPECT_RATIOS = ["4:5", "3:4", "1:1", "4:5"]

# ── Unsplash images for variety ─────────────────────────────────────────
IMAGES = [
    "https://images.unsplash.com/photo-1451187580459-43490279c0fa?w=600",
    "https://images.unsplash.com/photo-1518770660439-4636190af475?w=600",
    "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=600",
    "https://images.unsplash.com/photo-1526374965328-7f61d4dc18c5?w=600",
    "https://images.unsplash.com/photo-1519389950473-47ba0277781c?w=600",
    "https://images.unsplash.com/photo-1460925895917-afdab827c52f?w=600",
    "https://images.unsplash.com/photo-1550751827-4bd374c3f58b?w=600",
    "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=600",
    "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=600",
    "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=600",
    "https://images.unsplash.com/photo-1517245386807-bb43f82c33c4?w=600",
    "https://images.unsplash.com/photo-1522202176988-66273c2fd55f?w=600",
]

AVATARS = [
    "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100",
    "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100",
    "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=100",
    "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=100",
    "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=100",
]


def run_cmd(cmd, label, timeout=120):
    """Execute a command, return stdout, exit on failure."""
    print(f"  [{label}] $ {cmd[0].split('/')[-1]} {' '.join(cmd[1:4])}...")
    r = subprocess.run(cmd, capture_output=True, text=True, cwd=WORKSPACE_DIR, timeout=timeout)
    if r.returncode != 0:
        print(f"  ✗ FAILED [{label}]: {r.stderr[-500:] if r.stderr else r.stdout[-500:]}")
        return None
    return r.stdout


def generate_tokens(preset_id, color, theme, output_path):
    """Generate design tokens for a preset."""
    cmd = [
        BIN, "configure-design", color,
        "--style", theme,
        "--preset", "tonal_spot",
        "--output", output_path
    ]
    return run_cmd(cmd, f"tokens:{preset_id}")


def generate_slide(slide_type, tokens_file, theme, bg_style, archetype, params, variant="", aspect_ratio="4:5"):
    """Generate a single slide and return parsed JSON."""
    # Inject variant into params if provided (CLI doesn't have --variant flag)
    if variant:
        params = dict(params)
        params["variant"] = variant
    cmd = [
        BIN, "generate-slide", slide_type,
        "--tokens-file", tokens_file,
        "--theme", theme,
        "--bg-style", bg_style,
        "--archetype", archetype,
        "--aspect-ratio", aspect_ratio,
        "--params", json.dumps(params),
    ]
    stdout = run_cmd(cmd, f"slide:{slide_type}")
    if stdout is None:
        return None
    try:
        return json.loads(stdout)
    except json.JSONDecodeError as e:
        print(f"  ✗ JSON parse error for {slide_type}: {e}")
        return None


def render_carousel(slides, tokens_file, output_path, preset_id, aspect_ratio="4:5"):
    """Assemble slides into carousel HTML."""
    slides_file = os.path.join(TOKENS_DIR, f"{preset_id}_slides.json")
    with open(slides_file, "w") as f:
        json.dump(slides, f, indent=2)

    cmd = [
        BIN, "render-carousel", slides_file,
        "--tokens-file", tokens_file,
        "--brand-name", "SLIDEFORGE",
        "--brand-handle", "@slideforge",
        "--topic", preset_id.upper().replace("_", " "),
        "--url", "slideforge.dev",
        "--hashtags", "slides,campaign",
        "--aspect-ratio", aspect_ratio,
        "--output", output_path
    ]
    return run_cmd(cmd, f"carousel:{preset_id}")


# ── Content fillers per slide type ──────────────────────────────────────
def fill_params(slide_type, params_template, preset_id, slide_idx):
    """Fill a params_template with realistic test content based on slide type."""
    p = {}

    if slide_type == "hero":
        p["headline"] = f"The Future of {preset_id.replace('_', ' ').title()}"
        p["subheadline"] = "A new paradigm in digital communication and engagement"
        p["badge"] = "NEW RELEASE"
        p["kicker"] = "INTRODUCING"

    elif slide_type == "section_divider":
        p["kicker"] = f"SECTION {slide_idx + 1}"
        p["title"] = f"Deep Dive: {preset_id.replace('_', ' ').title()}"
        p["subtitle"] = "Comprehensive analysis and evidence"

    elif slide_type == "headline_subheadline":
        p["headline"] = "Revolutionary Approach"
        p["subheadline"] = "Transforming how we think about communication"

    elif slide_type == "problem_solution":
        p["title"] = "The Core Challenge"
        p["problem"] = "Traditional methods fail to engage modern audiences. Attention spans are shrinking while noise increases exponentially across every channel."
        p["solution"] = "Our framework restructures communication around emotional resonance, not information density."
        p["proof_points"] = ["73% higher engagement", "2.4x retention rate", "41% conversion lift"]

    elif slide_type == "feature":
        p["icon"] = "⚡"
        p["title"] = "Core Innovation"
        p["description"] = "A breakthrough approach that fundamentally reimagines how content creates emotional connection with audiences."
        p["number"] = f"{slide_idx:02d}"

    elif slide_type == "grid_cards":
        p["title"] = "Key Capabilities"
        p["cards"] = [
            {"icon": "📊", "title": "Data-Driven", "description": "Evidence-based decision making with real-time analytics"},
            {"icon": "🎨", "title": "Visual Impact", "description": "Stunning visual compositions that command attention"},
            {"icon": "⚡", "title": "Rapid Deployment", "description": "From concept to published content in minutes"},
            {"icon": "🔒", "title": "Trust Architecture", "description": "Built on principles of transparency and credibility"},
        ]

    elif slide_type == "split_features":
        p["title"] = "Dual Pillars"
        p["features"] = [
            {"icon": "🎯", "title": "Precision Targeting", "description": "Reach the right audience with surgical accuracy"},
            {"icon": "📈", "title": "Measurable Impact", "description": "Every campaign tracked with granular attribution"},
        ]

    elif slide_type == "list":
        p["title"] = "Key Principles"
        p["items"] = [
            "Emotional resonance over information density",
            "Visual storytelling as primary communication channel",
            "Evidence stacking builds irreversible conviction",
            "Action-oriented framing converts attention to commitment",
        ]

    elif slide_type == "checklist_action_plan":
        p["title"] = "Execution Checklist"
        p["items"] = [
            {"step": "Audience Analysis", "description": "Map emotional landscape and information needs"},
            {"step": "Content Architecture", "description": "Design emotional arc and slide sequence"},
            {"step": "Visual Design", "description": "Apply theme, colors, and typography system"},
            {"step": "Deploy & Measure", "description": "Publish and track engagement metrics"},
        ]

    elif slide_type == "definition":
        p["term"] = "Emotional Architecture"
        p["phonetic"] = "/ɪˌmoʊʃənl ˈɑːrkɪtɛktʃər/"
        p["definition"] = "The deliberate structural design of communication sequences to guide audiences through specific emotional progressions toward a desired action."
        p["context"] = "Unlike ad-hoc content creation, emotional architecture treats every slide as a node in a persuasion graph."

    elif slide_type == "myth_fact":
        p["myth"] = "More information = more persuasion"
        p["fact"] = "Emotional resonance drives 3x more action than information density"
        p["explanation"] = "Audiences process emotional cues faster than factual claims."

    elif slide_type == "before_after_story":
        p["title"] = "Transformation Story"
        p["before"] = {"label": "Before", "description": "Scattered content with no emotional throughline, 2% engagement rate"}
        p["after"] = {"label": "After", "description": "Structured emotional arc with 340% engagement lift and measurable conversion"}
        p["metric"] = "340%"
        p["metric_label"] = "Engagement Increase"

    elif slide_type == "timeline":
        p["title"] = "Evolution of Communication"
        p["steps"] = [
            {"title": "Discovery", "description": "Audience research and emotional mapping"},
            {"title": "Architecture", "description": "Emotional arc design and slide sequencing"},
            {"title": "Creation", "description": "Visual composition and content generation"},
            {"title": "Deployment", "description": "Publish, measure, and iterate"},
        ]

    elif slide_type == "process_map":
        p["title"] = "Workflow"
        p["steps"] = [
            {"icon": "🔍", "title": "Research", "description": "Audience analysis"},
            {"icon": "✏️", "title": "Design", "description": "Arc architecture"},
            {"icon": "🏗️", "title": "Build", "description": "Slide composition"},
            {"icon": "🚀", "title": "Launch", "description": "Deploy & measure"},
        ]

    elif slide_type == "chart":
        p["title"] = "Performance Metrics"
        p["chart_type"] = "bar"
        p["data"] = [
            {"label": "Engagement", "value": 85},
            {"label": "Retention", "value": 72},
            {"label": "Conversion", "value": 64},
            {"label": "Advocacy", "value": 91},
        ]

    elif slide_type == "column_chart":
        p["title"] = "Quarterly Growth"
        p["data"] = [
            {"label": "Q1", "value": 42},
            {"label": "Q2", "value": 58},
            {"label": "Q3", "value": 71},
            {"label": "Q4", "value": 89},
        ]

    elif slide_type == "scatter_plot":
        p["title"] = "Impact Correlation"
        p["x_label"] = "Investment"
        p["y_label"] = "Return"
        p["data"] = [
            {"x": 10, "y": 25}, {"x": 20, "y": 45}, {"x": 35, "y": 62},
            {"x": 50, "y": 78}, {"x": 70, "y": 91}, {"x": 90, "y": 95},
        ]

    elif slide_type == "gauge":
        p["title"] = "Campaign Effectiveness"
        p["value"] = 78
        p["max"] = 100
        p["label"] = "Overall Score"

    elif slide_type == "radar_chart":
        p["title"] = "Capability Assessment"
        p["axes"] = ["Impact", "Reach", "Conversion", "Retention", "Advocacy", "Trust"]
        p["data"] = [
            {"label": "Current", "values": [85, 72, 64, 78, 91, 88]},
            {"label": "Target", "values": [95, 90, 85, 92, 95, 95]},
        ]

    elif slide_type == "progress_rings":
        p["title"] = "Progress Dashboard"
        p["rings"] = [
            {"label": "Awareness", "value": 85, "color": "#5E5FE0"},
            {"label": "Engagement", "value": 72, "color": "#E05252"},
            {"label": "Conversion", "value": 64, "color": "#2E7D32"},
        ]

    elif slide_type == "comparison_bars":
        p["title"] = "Head-to-Head Comparison"
        p["comparison"] = {
            "metric": "Performance Score",
            "left": {"name": "Option A", "value": 68},
            "right": {"name": "Option B", "value": 92},
        }

    elif slide_type == "metric_grid":
        p["title"] = "Key Metrics"
        p["metrics"] = [
            {"value": "340%", "label": "Engagement Lift"},
            {"value": "2.4x", "label": "Retention Rate"},
            {"value": "85%", "label": "Conversion Rate"},
            {"value": "91%", "label": "Advocacy Score"},
        ]

    elif slide_type == "metric_card":
        p["metric"] = "Engagement Lift"
        p["value"] = "340%"
        p["label"] = "vs. previous campaign period"
        p["trend"] = "+127%"
        p["trend_direction"] = "up"

    elif slide_type == "funnel_chart":
        p["title"] = "Conversion Funnel"
        p["stages"] = [
            {"label": "Awareness", "value": 10000},
            {"label": "Interest", "value": 6500},
            {"label": "Consideration", "value": 3200},
            {"label": "Intent", "value": 1800},
            {"label": "Action", "value": 920},
        ]

    elif slide_type == "stat_row":
        p["title"] = "Performance Snapshot"
        p["stats"] = [
            {"value": "10K", "label": "Reach"},
            {"value": "340%", "label": "Lift"},
            {"value": "85%", "label": "Score"},
            {"value": "2.4x", "label": "ROI"},
        ]

    elif slide_type == "table":
        p["title"] = "Detailed Comparison"
        p["headers"] = ["Metric", "Ours", "Theirs", "Delta"]
        p["rows"] = [
            ["Engagement", "85%", "42%", "+43%"],
            ["Retention", "78%", "35%", "+43%"],
            ["Conversion", "64%", "28%", "+36%"],
        ]

    elif slide_type == "testimonial_avatar":
        p["quote"] = "This approach fundamentally changed how we think about audience engagement. The results speak for themselves."
        p["author"] = "Sarah Chen"
        p["role"] = "VP Marketing, TechCorp"
        p["avatar_url"] = random.choice(AVATARS)

    elif slide_type == "logo_cloud":
        p["title"] = "Trusted By"
        p["logos"] = ["Google", "Microsoft", "Apple", "Amazon", "Meta", "Netflix"]

    elif slide_type == "case_study_result":
        p["client"] = "Enterprise Client"
        p["challenge"] = "Declining engagement despite increased content production spend."
        p["solution"] = "Implemented emotional architecture framework across all content channels."
        p["results"] = [
            {"number": "340%", "label": "Engagement Lift"},
            {"number": "2.4x", "label": "ROI Increase"},
        ]

    elif slide_type == "faq":
        p["title"] = "Frequently Asked Questions"
        p["questions"] = [
            {"question": "How quickly can we see results?", "answer": "Most campaigns show measurable engagement lift within the first week of deployment."},
            {"question": "Does this work for B2B?", "answer": "Emotional architecture principles apply universally — B2B buyers are still humans making emotional decisions."},
            {"question": "What about compliance?", "answer": "All persuasion techniques used are ethical and transparent. No dark patterns."},
        ]

    elif slide_type == "image_caption":
        p["image_url"] = random.choice(IMAGES[:6])
        p["caption"] = "Visual Communication"
        p["description"] = "Imagery as emotional anchoring in presentation design."

    elif slide_type == "image_headline":
        p["headline"] = "Visual Impact"
        p["image_url"] = random.choice(IMAGES[:6])
        p["subheadline"] = "Where imagery meets narrative"

    elif slide_type == "image_quote":
        p["quote"] = "The best marketing doesn't feel like marketing."
        p["author"] = "Tom Fishburne"
        p["image_url"] = random.choice(AVATARS)

    elif slide_type == "image_callout":
        p["image_url"] = random.choice(IMAGES[:6])
        p["callouts"] = [{"label": "Key Feature", "description": "Critical innovation point", "x": 50, "y": 40}]
        p["description"] = "Annotated visual showing core architectural innovation."

    elif slide_type == "image_gallery":
        p["title"] = "Visual Gallery"
        p["images"] = [{"url": img, "caption": f"Asset {i+1}"} for i, img in enumerate(IMAGES[:4])]
        p["section_caption"] = "Campaign visual assets"

    elif slide_type == "image_collage":
        p["images"] = [{"url": img} for img in IMAGES[:5]]

    elif slide_type == "image_comparison":
        p["title"] = "Before & After"
        p["before_image"] = IMAGES[0]
        p["after_image"] = IMAGES[1]
        p["before_label"] = "BEFORE"
        p["after_label"] = "AFTER"
        p["description"] = "Visual transformation comparison"

    elif slide_type == "image_stat":
        p["image_url"] = random.choice(IMAGES[:6])
        p["stat"] = "340%"
        p["label"] = "Improvement"
        p["description"] = "Key metric overlay on visual"

    elif slide_type == "text_block":
        p["title"] = "Core Philosophy"
        p["body"] = "Effective communication is not about transmitting information — it is about creating emotional resonance that transforms passive viewers into active participants. Every slide in a campaign should advance the audience along a deliberately designed emotional arc."

    elif slide_type == "text_columns":
        p["title"] = "Strategic Framework"
        p["columns"] = [
            {"heading": "Research", "body": "Deep audience analysis and emotional mapping to understand motivations, fears, and aspirations."},
            {"heading": "Architecture", "body": "Design the emotional journey using proven persuasion frameworks and narrative transport theory."},
            {"heading": "Execution", "body": "Compose slides with dynamic typography, visual hierarchy, and evidence stacking."},
        ]

    elif slide_type == "quote":
        p["quote"] = "People don't buy what you do; they buy why you do it."
        p["author"] = "Simon Sinek"

    elif slide_type == "cta":
        p["headline"] = "Ready to Transform?"
        p["button_text"] = "Get Started"
        p["button_url"] = "https://slideforge.dev"
        p["subtext"] = "Join thousands of teams already using emotional architecture."

    elif slide_type == "qr_destination":
        p["destination_url"] = "https://slideforge.dev"
        p["cta_text"] = "Get Started"
        p["heading"] = "Learn More"
        p["caption"] = "Scan to explore the full framework"
        p["short_url"] = "slideforge.dev"

    elif slide_type == "pricing_plan":
        p["title"] = "Choose Your Plan"
        p["plans"] = [
            {"name": "Starter", "price": "Free", "features": ["5 slides/mo", "Basic themes", "Community support"]},
            {"name": "Pro", "price": "$29/mo", "features": ["Unlimited slides", "All themes", "Priority support", "Custom branding"]},
            {"name": "Enterprise", "price": "Custom", "features": ["Everything in Pro", "API access", "Dedicated support", "SLA guarantee"]},
        ]

    elif slide_type == "comparison":
        p["title"] = "Why Choose Us?"
        p["left_title"] = "Our Approach"
        p["right_title"] = "Traditional"
        p["left_items"] = ["Emotional architecture", "Dynamic typography", "Evidence-based", "Measurable results"]
        p["right_items"] = ["Template-based", "Static layouts", "Intuition-driven", "Vague metrics"]

    else:
        # Fallback: fill any remaining fields from template
        for key, val in params_template.items():
            if isinstance(val, str) and val.startswith("{{"):
                var_name = val.strip("{}")
                p[key] = f"Test value for {var_name}"
            elif isinstance(val, str):
                p[key] = val
            elif isinstance(val, list):
                p[key] = val
            elif isinstance(val, dict):
                p[key] = val

    return p


def expand_repeatable_block(block, preset_id):
    """Expand a repeatable block into individual slides."""
    slides = []
    unit_slides = block.get("unit_slides", [])
    repeat_max = block.get("repeat_count", {}).get("max", 2)

    for i in range(repeat_max):
        for unit in unit_slides:
            slide = dict(unit)
            # Fill template variables
            if "params_template" in slide:
                filled = {}
                for k, v in slide["params_template"].items():
                    if isinstance(v, str):
                        v = v.replace("{{unit_index}}", str(i + 1))
                        v = v.replace("{{unit_number}}", str(i + 1))
                    filled[k] = v
                slide["params"] = filled
            slides.append(slide)

    return slides


def process_preset(preset, preset_color, preset_idx):
    """Generate carousel HTML for a single preset."""
    preset_id = preset["id"]
    print(f"\n{'='*60}")
    print(f"Preset {preset_idx + 1}/28: {preset_id}")
    print(f"{'='*60}")

    tokens_file = os.path.join(TOKENS_DIR, f"{preset_id}_tokens.json")
    output_path = os.path.join(OUTPUT_DIR, f"{preset_id}.html")

    # Step 1: Generate tokens
    theme = PRESET_THEMES[preset_idx % len(PRESET_THEMES)]
    print(f"  Theme: {theme}, Color: {preset_color}")
    result = generate_tokens(preset_id, preset_color, theme, tokens_file)
    if result is None:
        print(f"  ✗ Failed to generate tokens for {preset_id}")
        return False

    # Step 2: Generate slides
    slides_json = []
    slide_entries = preset.get("slides", [])
    aspect_ratio = ASPECT_RATIOS[preset_idx % len(ASPECT_RATIOS)]

    slide_idx = 0
    for entry in slide_entries:
        # Handle repeatable blocks
        if entry.get("type") == "repeatable":
            expanded = expand_repeatable_block(entry, preset_id)
            for exp_slide in expanded:
                stype = exp_slide.get("slide_type", "text_block")
                # .get("params") returns {} when preset provides "params": {},
                # which is truthy — so fill_params fallback never fires.
                # Also catch unresolved template vars (e.g. "{{struggle_1_value}}")
                # left over from expand_repeatable_block's params_template expansion.
                preset_params = exp_slide.get("params", {})
                has_unresolved = any(isinstance(v, str) and "{{" in v for v in preset_params.values())
                if preset_params and not has_unresolved:
                    params = preset_params
                else:
                    params = fill_params(stype, exp_slide.get("params_template", {}), preset_id, slide_idx)
                theme_s = exp_slide.get("theme", theme)
                bg = exp_slide.get("bg_style", "dark")
                arch = exp_slide.get("archetype", PRESET_ARCHETYPES[preset_idx % len(PRESET_ARCHETYPES)])
                variant = exp_slide.get("variant", "")

                slide_obj = generate_slide(stype, tokens_file, theme_s, bg, arch, params, variant=variant, aspect_ratio=aspect_ratio)
                if slide_obj:
                    slides_json.append(slide_obj)
                    print(f"    ✓ {stype} (repeatable #{slide_idx + 1})")
                slide_idx += 1
        else:
            stype = entry.get("slide_type", "text_block")
            params_template = entry.get("params_template", {})
            params = fill_params(stype, params_template, preset_id, slide_idx)
            theme_s = entry.get("theme", theme)
            bg = entry.get("bg_style", "dark")
            arch = entry.get("archetype", PRESET_ARCHETYPES[preset_idx % len(PRESET_ARCHETYPES)])
            variant = entry.get("variant", "")

            slide_obj = generate_slide(stype, tokens_file, theme_s, bg, arch, params, variant=variant, aspect_ratio=aspect_ratio)
            if slide_obj:
                slides_json.append(slide_obj)
                print(f"    ✓ {stype} ({bg}/{theme_s})")
            slide_idx += 1

    if not slides_json:
        print(f"  ✗ No slides generated for {preset_id}")
        return False

    # Step 3: Render carousel
    print(f"\n  Rendering carousel with {len(slides_json)} slides...")
    result = render_carousel(slides_json, tokens_file, output_path, preset_id, aspect_ratio)
    if result is None:
        print(f"  ✗ Failed to render carousel for {preset_id}")
        return False

    print(f"  ✓ Saved: {output_path}")
    return True


def main():
    print(f"SlideForge Campaign Preset Generator v4.0.0")
    print(f"Binary: {BIN}")
    print(f"Output: {OUTPUT_DIR}")
    print(f"Presets: {PRESETS_FILE}\n")

    # Load presets
    with open(PRESETS_FILE) as f:
        catalog = json.load(f)

    presets = [p for p in catalog["presets"] if "id" in p]
    print(f"Found {len(presets)} presets to generate\n")

    success = 0
    failed = []

    for idx, preset in enumerate(presets):
        color = PRESET_COLORS[idx % len(PRESET_COLORS)]
        try:
            if process_preset(preset, color, idx):
                success += 1
            else:
                failed.append(preset["id"])
        except Exception as e:
            print(f"  ✗ Exception for {preset['id']}: {e}")
            failed.append(preset["id"])

    print(f"\n{'='*60}")
    print(f"RESULTS: {success}/{len(presets)} presets generated successfully")
    if failed:
        print(f"Failed: {', '.join(failed)}")
    print(f"Output: {OUTPUT_DIR}/")
    print(f"{'='*60}")

    return 0 if not failed else 1


if __name__ == "__main__":
    sys.exit(main())
