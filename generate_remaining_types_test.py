#!/usr/bin/env python3
"""
Remaining types diagnostic carousel: every slide type NOT covered by the
dataviz or text-layout carousels, rendered in both dark and light themes
for visual inspection and layout debugging.

Covers 19 types across Story, Image, Social-Proof, Conversion, and Content:
  before_after_story, case_study_result, checklist_action_plan, comparison,
  faq, image_callout, image_caption, image_collage, image_comparison,
  image_gallery, image_headline, image_quote, logo_cloud, myth_fact,
  pricing_plan, problem_solution, process_map, qr_destination, testimonial_avatar

Usage:
    python3 generate_remaining_types_test.py
"""

import json, os, subprocess, sys, tempfile

WORKSPACE_DIR = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(WORKSPACE_DIR, "dist", "slideforge-x86_64-linux-gnu")
if not os.path.exists(BIN):
    BIN = os.path.join(WORKSPACE_DIR, "target", "release", "slideforge-rust")

OUTPUT_DIR = os.path.join(WORKSPACE_DIR, "dist", "remaining_types_test")
CAROUSEL_PATH = os.path.join(WORKSPACE_DIR, "dist", "remaining_types_carousel.html")
TOKENS_FILE = os.path.join(OUTPUT_DIR, "tokens.json")
SLIDES_FILE = os.path.join(OUTPUT_DIR, "compiled_slides.json")

os.makedirs(OUTPUT_DIR, exist_ok=True)

# ── Placeholder image URLs ─────────────────────────────────────────────
IMG_NATURE = "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=800&h=500&fit=crop"
IMG_CITY   = "https://images.unsplash.com/photo-1480714378408-67cf0d13bc1b?w=800&h=500&fit=crop"
IMG_TEAM   = "https://images.unsplash.com/photo-1522071820081-009f0129c71c?w=800&h=500&fit=crop"
IMG_CODE   = "https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=800&h=500&fit=crop"
IMG_PRODUCT= "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=800&h=500&fit=crop"
IMG_OFFICE = "https://images.unsplash.com/photo-1497366216548-37526070297c?w=800&h=500&fit=crop"
IMG_LAPTOP = "https://images.unsplash.com/photo-1496181133206-80ce9b88a853?w=800&h=500&fit=crop"
IMG_COFFEE = "https://images.unsplash.com/photo-1495474472287-4d71bcdd2085?w=800&h=500&fit=crop"


def run_cmd(cmd, label):
    """Execute a command, return stdout, exit on failure."""
    print(f"  [{label}] $ {' '.join(cmd[:4])}...")
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
    with tempfile.NamedTemporaryFile(suffix=".json", delete=False, mode="w") as tmp:
        out_path = tmp.name
    cmd = [
        BIN, "generate-slide", slide_type,
        "--tokens-file", tokens_file,
        "--theme", theme,
        "--bg-style", bg_style,
        "--archetype", archetype,
        "--params", json.dumps(params),
        "--output", out_path,
    ]
    run_cmd(cmd, f"gen:{slide_type}")
    with open(out_path, "r") as f:
        result = json.load(f)
    os.unlink(out_path)
    return result


# ── Remaining slide definitions ────────────────────────────────────────
# 19 types × 2 themes = 38 slides
SLIDES = [
    # ═══ SECTION: Story / Transformation ═══
    {"section": "SECTION 1 — Story / Transformation"},

    {"slide_type": "before_after_story", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Deployment Pipeline", "before": "Manual deploys taking 45 minutes with frequent rollback failures. Zero automated testing, no staging environment.", "after": "Automated CI/CD pipeline deploys in 90 seconds with 100% test coverage, canary releases, and instant rollback.", "metric": "98%", "metric_label": "Faster deploys"}},

    {"slide_type": "before_after_story", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "metric",
     "params": {"title": "Code Review Process", "before": "3-day average review cycle, 40% of comments unresolved, tribal knowledge lost in Slack threads.", "after": "Same-day reviews with automated linting, structured feedback templates, and persistent decision logs.", "metric": "3x", "metric_label": "Faster reviews"}},

    {"slide_type": "case_study_result", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"client": "TechCorp Inc.", "challenge": "Engineering team spending 12 hours/week on manual report generation instead of building features.", "solution": "Implemented SlideForge CLI pipeline integrated with their CI/CD to auto-generate stakeholder decks.", "results": "Report generation dropped to 15 minutes/week. Engineering velocity increased 23%. Stakeholder satisfaction score went from 3.2 to 4.7."}},

    {"slide_type": "case_study_result", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "results-grid",
     "params": {"challenge": "Marketing team unable to produce on-brand social content at the pace of product launches.", "solution": "Deployed 28 campaign presets with pool-based composition for rapid carousel generation.", "results": "Content production speed: 5x faster. Brand consistency score: 94%. Monthly output: 12 decks → 60 decks.", "title": "Content Velocity Case Study"}},

    {"slide_type": "myth_fact", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"myth": "AI-generated slides always look generic and templated.", "fact": "With proper design tokens, archetypes, and composition constraints, AI-generated slides match or exceed hand-crafted quality.", "explanation": "The key is systematic design systems, not prompt engineering alone. SlideForge's 47-type registry ensures visual coherence across any composition."}},

    {"slide_type": "myth_fact", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "params": {"myth": "You need a design tool to create professional presentations.", "fact": "Code-first slide pipelines produce consistent, version-controllable, API-automatable output that scales beyond what manual tools allow.", "explanation": "SlideForge generates HTML+CSS slides from JSON specs, rendered to PNG via headless Chromium."}},

    {"slide_type": "problem_solution", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"problem": "Teams waste 6+ hours per week creating presentations manually, with inconsistent branding and frequent layout errors.", "solution": "CLI-driven slide generation with 47 validated types, design token enforcement, and automated overflow detection.", "proof_points": "Compile time under 10ms per slide. Zero manual layout fixes needed. 100% brand consistency across all generated output.", "title": "The Presentation Problem"}},

    {"slide_type": "problem_solution", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "proof-grid",
     "params": {"problem": "Design systems break down at scale — tokens drift, components diverge, and nobody notices until the quarterly review.", "solution": "Compile-time validation catches token violations, layout drift, and composition errors before they ship.", "proof_points": "94 automated tests. Real-time composition validation. Overflow detection from runtime geometry.", "title": "Scale Without Drift"}},

    {"slide_type": "process_map", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Carousel Generation Pipeline", "steps": [
         {"label": "Theme Config", "description": "Set brand color, archetype, design tokens"},
         {"label": "Preset Select", "description": "Choose from 28 campaign presets or compose from pool"},
         {"label": "Pool Remix", "description": "AI agent selects/arranges slides from 47-type registry"},
         {"label": "Content Fill", "description": "Populate each slide with contextual content"},
         {"label": "Validate", "description": "Composition + per-slide validation, overflow detection"},
         {"label": "Export", "description": "HTML carousel → PNG slides via headless Chromium"}
     ]}},

    {"slide_type": "process_map", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "map",
     "params": {"title": "Validation Pipeline", "steps": [
         {"label": "Parse Spec", "description": "Deserialize JSON slide specification"},
         {"label": "Shape Check", "description": "Verify required params and types"},
         {"label": "Geom Compute", "description": "Calculate font sizes, card dimensions, spacing"},
         {"label": "Overflow Gate", "description": "Flag text clipping, layout drift, composition errors"},
         {"label": "Render HTML", "description": "Generate slide HTML with CSS custom properties"},
         {"label": "Rasterize", "description": "Headless Chromium → PNG at 2x resolution"}
     ]}},

    {"slide_type": "checklist_action_plan", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Launch Day Checklist", "items": [
         "Verify all 28 presets render without overflow",
         "Run composition validator on generated carousels",
         "Check dark and light theme variants",
         "Export final PNGs and verify pixel dimensions",
         "Push to dist/ and update documentation"
     ]}},

    {"slide_type": "checklist_action_plan", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "checklist",
     "params": {"title": "New Slide Type Checklist", "items": [
         "Register in slides.rs with layout_family",
         "Implement render function in components.rs",
         "Add to dispatch table with correct param mapping",
         "Write unit test with valid and empty params",
         "Add to SKILL.md and slide-types-manifest.md",
         "Include in diagnostic carousel for visual audit"
     ]}},

    # ═══ SECTION: Comparison / Content ═══
    {"section": "SECTION 2 — Comparison / Content"},

    {"slide_type": "comparison", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Free vs Pro", "columns": ["Free", "Pro"], "rows": [
         ["Slides per month", "5", "Unlimited"],
         ["Custom typography", "—", "✓"],
         ["Brand presets", "1", "Unlimited"],
         ["Export format", "PNG", "PNG + PDF"],
         ["API access", "—", "✓"]
     ], "highlight_column": 1, "variant": "table"}},

    {"slide_type": "comparison", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "cards",
     "params": {"title": "SlideForge vs Manual", "columns": ["Manual", "SlideForge"], "rows": [
         ["Time per deck", "4 hours", "5 minutes"],
         ["Brand consistency", "60%", "100%"],
         ["Error rate", "15%", "0%"],
         ["Scalability", "1 deck at a time", "Unlimited parallel"]
     ]}},

    {"slide_type": "faq", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Common Questions", "questions": [
         {"q": "Does SlideForge work offline?", "a": "Yes. The binary is fully self-contained — no API calls, no internet required after install."},
         {"q": "Can I use custom fonts?", "a": "Define fonts in your design tokens JSON. SlideForge renders any font available in the headless Chromium environment."},
         {"q": "What about accessibility?", "a": "All slides follow WCAG contrast ratios. The validator checks color contrast at build time."},
         {"q": "Is there a Python SDK?", "a": "Not yet. The CLI and MCP server are the primary interfaces. Python integration works via subprocess calls."}
     ]}},

    {"slide_type": "faq", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "compact",
     "params": {"title": "Pricing FAQ", "questions": [
         {"q": "Is there a free tier?", "a": "The CLI is open source and free forever."},
         {"q": "What about enterprise support?", "a": "Email support, SLA guarantees, and custom integrations available on Pro and Enterprise plans."},
         {"q": "Can I self-host?", "a": "Absolutely. The binary runs anywhere Rust compiles — Linux, macOS, Windows."}
     ]}},

    # ═══ SECTION: Image Types ═══
    {"section": "SECTION 3 — Image Types"},

    {"slide_type": "image_headline", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"image_url": IMG_NATURE, "headline": "Ship Beautiful Slides", "subheadline": "From JSON to PNG in under 10ms"}},

    {"slide_type": "image_headline", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "center",
     "params": {"image_url": IMG_CITY, "headline": "Design at Scale", "subheadline": "47 types, infinite compositions"}},

    {"slide_type": "image_caption", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"image_url": IMG_TEAM, "caption": "Our engineering team builds slide infrastructure that scales to thousands of carousels per day, with consistent brand quality across every output.", "description": "Team Collaboration"}},

    {"slide_type": "image_caption", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "image-bottom",
     "params": {"image_url": IMG_OFFICE, "caption": "Modern slide creation is a systems problem, not a design problem. Solve it with code.", "description": "Office Environment"}},

    {"slide_type": "image_quote", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"image_url": IMG_NATURE, "quote": "The best slide is the one you never had to manually create.", "author": "SlideForge Team", "role": "Design Philosophy"}},

    {"slide_type": "image_callout", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"image_url": IMG_CODE, "callouts": [
         {"text": "Compile-time validation", "x": 20, "y": 30},
         {"text": "Runtime overflow detection", "x": 60, "y": 50},
         {"text": "Token enforcement", "x": 40, "y": 75}
     ], "description": "Architecture Overview"}},

    {"slide_type": "image_comparison", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"before_image": IMG_LAPTOP, "after_image": IMG_CITY, "before_label": "Before SlideForge", "after_label": "After SlideForge", "description": "From manual chaos to automated precision."}},

    {"slide_type": "image_comparison", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "arrow",
     "params": {"before_image": IMG_COFFEE, "after_image": IMG_OFFICE, "before_label": "Coffee Shop MVP", "after_label": "Production Scale", "description": "Growth journey from prototype to platform."}},

    {"slide_type": "image_stat", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"image_url": IMG_NATURE, "stat_value": "47", "stat_label": "Slide Types", "description": "Registered in the system"}},

    {"slide_type": "image_stat", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "params": {"image_url": IMG_CITY, "stat_value": "<10ms", "stat_label": "Compile Time", "description": "Per slide, deterministic"}},

    # ═══ SECTION: Collage & Gallery ═══
    {"section": "SECTION 4 — Collage & Gallery"},

    {"slide_type": "image_collage", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"images": [
         {"url": IMG_NATURE, "caption": "Scale"},
         {"url": IMG_CITY, "caption": "Precision"},
         {"url": IMG_TEAM, "caption": "Quality"}
     ], "title": "SlideForge Capabilities", "variant": "scattered"}},

    {"slide_type": "image_gallery", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"images": [
         {"url": IMG_NATURE, "caption": "Compile"},
         {"url": IMG_CITY, "caption": "Validate"},
         {"url": IMG_CODE, "caption": "Export"},
         {"url": IMG_TEAM, "caption": "Scale"}
     ], "title": "The Pipeline", "variant": "4-grid"}},

    {"slide_type": "image_gallery", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "featured-1-2",
     "params": {"images": [
         {"url": IMG_NATURE, "caption": "Primary"},
         {"url": IMG_CITY, "caption": "Supporting A"},
         {"url": IMG_OFFICE, "caption": "Supporting B"}
     ], "title": "Featured Layout"}},

    # ═══ SECTION: Social Proof ═══
    {"section": "SECTION 5 — Social Proof"},

    {"slide_type": "testimonial_avatar", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"quote": "We replaced our entire slide toolchain with a single CLI command. The consistency improvement alone justified the switch.", "author": "Elena Rodriguez", "role": "Head of Design, ScaleUp"}},

    {"slide_type": "testimonial_avatar", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "profile",
     "params": {"quote": "Our marketing team generates 3x more content since adopting SlideForge. The quality hasn't dropped once.", "author": "James Park", "role": "CMO, GrowthCo"}},

    {"slide_type": "logo_cloud", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Trusted By", "logos": [
         {"name": "TechCorp"},
         {"name": "ScaleUp"},
         {"name": "GrowthCo"},
         {"name": "InnovateLabs"},
         {"name": "CloudBase"},
         {"name": "DataDrive"}
     ]}},

    {"slide_type": "logo_cloud", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "strip",
     "params": {"title": "Partners", "logos": [
         {"name": "AWS"},
         {"name": "Vercel"},
         {"name": "Supabase"},
         {"name": "Railway"},
         {"name": "Fly.io"}
     ]}},

    # ═══ SECTION: Conversion ═══
    {"section": "SECTION 6 — Conversion"},

    {"slide_type": "qr_destination", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"destination_url": "https://crates.io/crates/slideforge", "cta_text": "Install Now", "heading": "Scan to Install", "caption": "CLI installation via cargo", "short_url": "sgf.dev/install", "variant": "full-conversion"}},

    {"slide_type": "qr_destination", "theme": "bold", "bg_style": "light", "archetype": "data_analyst",
     "variant": "split-card",
     "params": {"destination_url": "https://docs.slideforge.dev", "cta_text": "Open Docs", "heading": "Read the Docs", "caption": "Full API reference and guides", "short_url": "docs.sgfd.dev", "brand_name": "SlideForge"}},

    {"slide_type": "pricing_plan", "theme": "editorial", "bg_style": "dark", "archetype": "data_analyst",
     "params": {"title": "Choose Your Plan", "plans": [
         {"name": "CLI", "price": "Free", "features": ["47 slide types", "28 presets", "CLI + MCP", "Community support"]},
         {"name": "Pro", "price": "$29/mo", "features": ["Everything in CLI", "Custom themes", "Priority support", "API access"], "featured": True},
         {"name": "Enterprise", "price": "Custom", "features": ["Everything in Pro", "SSO + RBAC", "SLA guarantee", "Custom integrations"]}
     ]}},
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
print(f"Step 2: Generating remaining type slides...\n")
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
    print(f"    ✓ {stype} ({var or 'default'}) [{bg}]")

with open(SLIDES_FILE, "w") as f:
    json.dump(compiled_slides, f, indent=2)

print(f"\n  ✓ Saved compiled slides to: {SLIDES_FILE}\n")

# ── Step 3: Render carousel HTML ───────────────────────────────────────
print(f"Step 3: Rendering carousel with {len(compiled_slides)} slides...")
cmd_carousel = [
    BIN, "render-carousel", SLIDES_FILE,
    "--tokens-file", TOKENS_FILE,
    "--topic", "STORY · IMAGE · SOCIAL · CONVERSION",
    "--output", CAROUSEL_PATH
]
run_cmd(cmd_carousel, "carousel")
print(f"  ✓ Carousel: {CAROUSEL_PATH}\n")

print(f"✅ Remaining types test complete! {len(compiled_slides)} slides rendered.")
