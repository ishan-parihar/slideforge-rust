#!/usr/bin/env python3
"""
Generate production-grade carousel HTML for each of the 28 campaign presets.

Every slide gets purpose-specific demo content that tells the story its preset
is designed to deliver. No generic filler — each carousel should feel like a
real campaign you'd publish.

Pipeline: configure-design → generate-slide → render-carousel

Flags:
  --validate    Run composition validation on each preset's slide sequence
  --only ID     Generate only the specified preset(s), comma-separated
  --list        List all preset IDs and exit
"""

import json, os, subprocess, sys, random

WORKSPACE_DIR = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(WORKSPACE_DIR, "dist", "slideforge-x86_64-linux-gnu")
if not os.path.exists(BIN):
    BIN = os.path.join(WORKSPACE_DIR, "target", "release", "slideforge-rust")

PRESETS_FILE = os.path.join(WORKSPACE_DIR, "docs", "presets", "campaign-presets.json")
OUTPUT_DIR = os.path.join(WORKSPACE_DIR, "dist", "presets")
TOKENS_DIR = os.path.join(OUTPUT_DIR, "tokens")

os.makedirs(OUTPUT_DIR, exist_ok=True)
os.makedirs(TOKENS_DIR, exist_ok=True)

# ── Visual diversity ─────────────────────────────────────────────────────
PRESET_COLORS = [
    "#5E5FE0", "#E05252", "#2E7D32", "#F57C00", "#7B1FA2",
    "#00838F", "#C62828", "#283593", "#558B2F", "#D84315",
    "#4527A0", "#00695C", "#AD1457", "#1565C0", "#827717",
    "#6A1B9A", "#00897B", "#BF360C", "#311B92", "#1B5E20",
    "#880E4F", "#0D47A1", "#E65100", "#33691E", "#4A148C",
    "#006064", "#B71C1C", "#1A237E",
]
PRESET_THEMES = ["editorial", "bold", "minimal", "dark", "vibrant", "natural"]
PRESET_ARCHETYPES = [
    "educator", "thought_leader", "startup_pitch",
    "brand_storyteller", "data_analyst", "creator",
]
ASPECT_RATIOS = ["4:5", "3:4", "1:1", "4:5"]

IMAGES = [
    "https://images.unsplash.com/photo-1451187580459-43490279c0fa?w=600",
    "https://images.unsplash.com/photo-1518770660439-4636190af475?w=600",
    "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=600",
    "https://images.unsplash.com/photo-1526374965328-7f61d4dc18c5?w=600",
    "https://images.unsplash.com/photo-1519389950473-47ba0277781c?w=600",
    "https://images.unsplash.com/photo-1460925895917-afdab827c52f?w=600",
    "https://images.unsplash.com/photo-1550751827-4bd374c3f58b?w=600",
    "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=600",
]
AVATARS = [
    "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100",
    "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100",
    "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=100",
    "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=100",
    "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=100",
]


def run_cmd(cmd, label, timeout=120):
    print(f"  [{label}] $ {cmd[0].split('/')[-1]} {' '.join(cmd[1:4])}...")
    r = subprocess.run(cmd, capture_output=True, text=True, cwd=WORKSPACE_DIR, timeout=timeout)
    if r.returncode != 0:
        print(f"  ✗ FAILED [{label}]: {r.stderr[-500:] if r.stderr else r.stdout[-500:]}")
        return None
    return r.stdout


def generate_tokens(preset_id, color, theme, output_path):
    cmd = [BIN, "configure-design", color, "--style", theme, "--preset", "tonal_spot", "--output", output_path]
    return run_cmd(cmd, f"tokens:{preset_id}")


def generate_slide(slide_type, tokens_file, theme, bg_style, archetype, params, variant="", aspect_ratio="4:5"):
    if variant:
        params = dict(params)
        params["variant"] = variant
    # Use --output to get JSON (stdout uses TOON format, not JSON)
    tmp_path = os.path.join(TOKENS_DIR, f"_tmp_{slide_type}.json")
    cmd = [
        BIN, "generate-slide", slide_type,
        "--tokens-file", tokens_file,
        "--theme", theme, "--bg-style", bg_style,
        "--archetype", archetype, "--aspect-ratio", aspect_ratio,
        "--params", json.dumps(params),
        "--output", tmp_path,
    ]
    result = run_cmd(cmd, f"slide:{slide_type}")
    if result is None:
        return None
    try:
        with open(tmp_path, 'r') as f:
            data = json.load(f)
        return data
    except (json.JSONDecodeError, FileNotFoundError) as e:
        print(f"  ✗ JSON parse error for {slide_type}: {e}")
        return None
    finally:
        try:
            os.remove(tmp_path)
        except OSError:
            pass


def render_carousel(slides, tokens_file, output_path, preset_id, aspect_ratio="4:5"):
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
        "--output", output_path,
    ]
    return run_cmd(cmd, f"carousel:{preset_id}")


def expand_repeatable_block(block, preset_id):
    slides = []
    unit_slides = block.get("unit_slides", [])
    repeat_max = block.get("repeat_count", {}).get("max", 2)
    for i in range(repeat_max):
        for unit in unit_slides:
            slide = dict(unit)
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


# ══════════════════════════════════════════════════════════════════════════
#  PER-PRESET CONTENT — unique demo content for every slide
# ══════════════════════════════════════════════════════════════════════════

def preset_content(preset_id):
    """Return a list of param dicts, one per expanded slide, for the given preset."""

    I = IMAGES
    A = AVATARS

    # ── 1. ANNOUNCEMENT (product launch) ────────────────────────────────
    # Arc: Excitement → Understanding → Proof → Action
    if preset_id == "announcement":
        return [
            # hero
            {"headline": "Introducing Nexus 3.0", "subheadline": "The slide engine that writes itself. Describe what you need. Nexus builds the carousel.", "badge": "JUST SHIPPED", "kicker": "LAUNCHING TODAY"},
            # problem_solution
            {"title": "The Content Bottleneck", "problem": "Teams spend 12+ hours per carousel — writing copy, picking layouts, tuning themes, exporting assets. Designers become bottlenecks. Deadlines slip. Quality varies wildly between team members.", "solution": "Nexus turns a one-sentence brief into a production-ready carousel in under 60 seconds. Same quality every time. No designer required.", "proof_points": ["12h → 60s per carousel", "144+ slide compositions", "Zero design skill required"]},
            # grid_cards
            {"title": "What Nexus Ships", "cards": [
                {"icon": "🧠", "title": "Brief-to-Carousel", "description": "Write one paragraph about your product. Nexus generates the full emotional arc, picks slide types, and renders everything."},
                {"icon": "🎨", "title": "Theme Engine", "description": "Material Design tokens with tonal spot palettes. Every carousel gets a unique, consistent visual identity."},
                {"icon": "⚡", "title": "CLI + MCP Server", "description": "Works as a standalone CLI or as an MCP tool any AI agent can call. No UI required."},
                {"icon": "📊", "title": "28 Campaign Presets", "description": "Pre-built emotional arcs for launches, thought leadership, data stories, and more — ready to fill and fire."},
            ]},
            # testimonial_avatar
            {"quote": "We replaced our entire design sprint with a single Nexus call. The output quality matched our best designer's work.", "author": "Priya Mehta", "role": "Head of Content, Relay.so", "avatar_url": A[0]},
            # metric_grid
            {"title": "Launch Metrics", "metrics": [
                {"value": "60s", "label": "Avg generation time"},
                {"value": "144+", "label": "Slide compositions"},
                {"value": "28", "label": "Campaign presets"},
                {"value": "0", "label": "Design skills needed"},
            ]},
            # cta
            {"headline": "Stop Designing Slides. Start Shipping Campaigns.", "button_text": "Try Nexus Free", "button_url": "https://slideforge.dev", "subtext": "No credit card. No onboarding call. Just ship."},
        ]

    # ── 2. AUTHORITY INTRODUCTION (political candidate intro) ───────────
    # Arc: Respect → Empathy → Admiration → Trust
    elif preset_id == "authority_introduction":
        return [
            # image_headline
            {"headline": "Dr. Amara Osei for City Council", "image_url": I[0], "subheadline": "15 years of public health. Now she's running to fix what policy broke."},
            # problem_solution (credentials gap)
            {"problem": "Voters see candidates make promises they can't keep. Expertise without credibility signals gets dismissed as empty rhetoric.", "solution": "Lead with credentials. Anchor with proof. Close with specificity. The three-part authority structure that converts.", "proof_points": ["15 years of public health experience", "12 clinics opened, 40,000+ patients served", "3 bills authored at state level"]},
            # grid_cards (core components)
            {"title": "Her Record Speaks", "cards": [
                {"icon": "🏥", "title": "12 Clinics Opened", "description": "Built free clinics in underserved neighborhoods serving 40,000+ patients annually."},
                {"icon": "📋", "title": "3 Bills Authored", "description": "State-level legislation expanding mental health coverage for uninsured residents."},
                {"icon": "🎓", "title": "Mentored 200+", "description": "Trained the next generation of community health workers through the Osei Fellowship."},
                {"icon": "🤝", "title": "Bipartisan Respect", "description": "Endorsed by both the Medical Association and the Teachers Union — rare common ground."},
            ]},
            # problem_solution (authority signals)
            {"problem": "Without authority signals, even brilliant content gets ignored. The audience doesn't know why they should listen.", "solution": "The Osei campaign leads with credentials in every touchpoint: 'Dr. Osei' not 'Amara.' '15 years' not 'experience.' Specificity builds trust.", "proof_points": ["Authority signals in first slide increase completion by 45%", "Credential anchoring reduces bounce rate by 62%", "Specific claims outperform general by 2.8x"]},
            # image_quote (authority quote)
            {"quote": "I've watched families choose between medication and rent. That's not a healthcare failure — it's a policy failure. And policy failures need policy makers who've lived them.", "author": "Dr. Amara Osei", "image_url": A[0]},
            # problem_solution (expert blind spot)
            {"problem": "Expert candidates assume voters understand the issues. They don't. Jargon and policy details alienate the very people they're trying to reach.", "solution": "Translate expertise into lived experience. 'Mental health coverage' becomes 'your neighbor can see a therapist.' 'Funding formula' becomes 'your kid's school gets $2,400 more.'", "proof_points": ["Plain-language content retains 78% past third slide", "Lived-experience framing increases sharing by 3.1x", "Accessibility-first messaging broadens coalition"]},
            # logo_cloud (credibility logos)
            {"title": "Endorsed By", "logos": ["District 7 Teachers Union", "Metro Medical Assoc.", "Small Business Alliance", "Housing Now PAC", "Young Democrats", "Veterans for Change"]},
            # problem_solution (conversion gap)
            {"problem": "Voters who trust a candidate still don't act. The gap between 'I believe in her' and 'I'll vote for her' is the hardest to bridge.", "solution": "Make action specific and immediate: 'Register by Sept 15' not 'Register to vote.' 'Bring 3 friends' not 'Spread the word.' Specificity drives conversion.", "proof_points": ["Specific CTAs convert 3.2x better than generic", "Social proof in final slide increases sharing by 45%", "Deadline-driven CTAs outperform open-ended by 2.1x"]},
            # qr_destination
            {"destination_url": "https://amaraosei.org", "cta_text": "Join the Campaign", "heading": "Dr. Amara Osei for District 7", "caption": "Scan to volunteer, donate, or learn more", "short_url": "amaraosei.org"},
        ]

    # ── 3. ORIGIN STORY ─────────────────────────────────────────────────
    # Arc: Curiosity → Connection → Admiration → Trust
    elif preset_id == "origin_story":
        return [
            # image_headline
            {"headline": "How a Garage Server Became a Slide Engine", "image_url": I[1], "subheadline": "SlideForge started as a weekend hack in 2023. Here's the real story."},
            # image_quote
            {"quote": "I was building a pitch deck at 2am and thought: why am I still dragging boxes around in Figma? The computer knows what I want. Let it compose.", "author": "Ishan Parihar", "image_url": A[1]},
            # timeline
            {"title": "The Journey", "steps": [
                {"title": "Weekend Hack", "description": "First prototype: a Python script that turned markdown into carousel HTML. Ugly but functional."},
                {"title": "The Python Port", "description": "Built carousel-mcp — the reference implementation that proved the MCP server model works."},
                {"title": "The Rust Rewrite", "description": "Rewrote everything in Rust for speed. 10x faster rendering, single binary, zero runtime deps."},
                {"title": "Campaign Presets", "description": "Added emotional arc templates so any AI agent can generate campaign-grade carousels from a brief."},
            ]},
            # grid_cards
            {"title": "Core Beliefs", "cards": [
                {"icon": "🔧", "title": "Tools, Not Templates", "description": "SlideForge doesn't pick your content. It gives you a composition engine and gets out of the way."},
                {"icon": "📐", "title": "Composition Over Decoration", "description": "Every slide type exists to solve a communication problem, not to look pretty."},
                {"icon": "🚀", "title": "Ship in Seconds", "description": "If generation takes more than 60 seconds, something is wrong. Speed is a feature."},
                {"icon": "🔓", "title": "Open Pipeline", "description": "CLI for humans, MCP for agents. Same engine, same output. No walled gardens."},
            ]},
            # callout
            {"title": "By the Numbers", "text": "85 unit tests passing. 47 slide types. 28 campaign presets. 1 binary. That's the whole system."},
            # cta
            {"headline": "Built by Builders, for Builders", "button_text": "Star on GitHub", "button_url": "https://github.com/ishan-parihar/slideforge-rust", "subtext": "Open source. MIT licensed. Contributions welcome."},
        ]

    # ── 4. ASPIRATION LADDER ────────────────────────────────────────────
    # Arc: Desire → Envy → Motivation → Belief → Action
    elif preset_id == "aspiration_ladder":
        return [
            # image_headline
            {"headline": "The Top 1% of Content Teams Do This Differently", "image_url": I[2], "subheadline": "They stopped designing slides. They started composing campaigns."},
            # metric_grid
            {"title": "The Gap Is Real", "metrics": [
                {"value": "12x", "label": "Output difference"},
                {"value": "4min", "label": "Their avg generation"},
                {"value": "6hrs", "label": "Your avg production"},
                {"value": "∞", "label": "Compounding advantage"},
            ]},
            # image_headline
            {"headline": "While You're Polishing One Deck, They've Shipped Twelve", "image_url": I[3], "subheadline": "Speed isn't about cutting corners. It's about removing the entire corner-cutting workflow."},
            # process_map
            {"title": "The Ascent Path", "steps": [
                {"icon": "🔍", "title": "Audit", "description": "Map your current production bottlenecks"},
                {"icon": "⚙️", "title": "Integrate", "description": "Wire Nexus into your content pipeline"},
                {"icon": "📈", "title": "Accelerate", "description": "Ship 10x more with the same headcount"},
                {"icon": "🏆", "title": "Dominate", "description": "Outpace competitors who still design manually"},
            ]},
            # testimonial_avatar
            {"quote": "Our competitor shipped 8 carousels in the time it took us to approve one. That's when we knew the process was broken.", "author": "Marcus Chen", "role": "VP Growth, Acme Labs", "avatar_url": A[2]},
            # progress_rings
            {"title": "Your Compounding Advantage", "items": [
                {"label": "Speed", "value": 87, "max": 100, "color": "#6366f1"},
                {"label": "Quality", "value": 94, "max": 100, "color": "#8b5cf6"},
                {"label": "Consistency", "value": 98, "max": 100, "color": "#a78bfa"},
            ]},
            # qr_destination
            {"destination_url": "https://slideforge.dev", "cta_text": "Start Now", "heading": "Close the Gap Today", "caption": "Scan to start your free trial", "short_url": "slideforge.dev"},
        ]

    # ── 5. CREDIBILITY CASCADE (testimonial wall) ──────────────────────
    # Arc: Skepticism → Recognition → Trust → Action
    elif preset_id == "credibility_cascade":
        return [
            # testimonial_avatar
            {"quote": "We tried every AI slide tool on the market. Nexus is the first one where I didn't have to redo every slide manually afterward.", "author": "Sarah Kim", "role": "Design Lead, Relay.so", "avatar_url": A[0]},
            # logo_cloud
            {"title": "Teams That Ship With Nexus", "logos": ["Relay.so", "Composabl", "Typeframe", "DeckAI", "Pitchflow", "Slidespkr"]},
            # testimonial_avatar
            {"quote": "Our LinkedIn impressions 3x'd in two weeks. Same team, same strategy — just faster execution.", "author": "James Okonkwo", "role": "Growth Lead, Composabl", "avatar_url": A[3]},
            # metric_grid
            {"title": "Collective Impact", "metrics": [
                {"value": "4.2M", "label": "Impressions generated"},
                {"value": "340%", "label": "Avg engagement lift"},
                {"value": "6", "label": "Teams in case studies"},
                {"value": "98%", "label": "Would recommend"},
            ]},
            # gauge
            {"title": "Trust Score", "value": 96, "max": 100, "unit": "%", "label": "Customer satisfaction"},
            # testimonial_avatar
            {"quote": "The preset architecture is brilliant. I described our annual report and it nailed the emotional arc on the first try.", "author": "Elena Voss", "role": "Head of Comms, Typeframe", "avatar_url": A[2]},
            # callout
            {"title": "The Numbers Don't Lie", "text": "4.2M impressions. 340% avg engagement lift. 98% would recommend. These aren't projections — they're measured outcomes from real teams shipping real carousels."},
            # testimonial_avatar
            {"quote": "The MCP integration means our content agent generates carousels autonomously. We just review and publish.", "author": "Lin Zhao", "role": "CTO, Pitchflow", "avatar_url": A[1]},
            # cta
            {"headline": "Join the Teams That Already Switched", "button_text": "Start Free Trial", "button_url": "https://slideforge.dev", "subtext": "No credit card required. Ship your first carousel in under a minute."},
        ]

    # ── 6. PROOF STACKING (case study) ─────────────────────────────────
    # Arc: Skepticism → Recognition → Conviction → Trust
    elif preset_id == "proof_stacking":
        return [
            # section_divider
            {"kicker": "CASE STUDY", "title": "How Relay.so Cut Content Production by 90%", "subtitle": "From 12-hour design sprints to 60-second generation"},
            # repeatable ×3: problem_solution → before_after → case_study → testimonial
            # Unit 1: The Problem
            {"title": "The Bottleneck", "problem": "Relay.so's 4-person marketing team was spending 48 hours per week on carousel creation alone. Each Instagram campaign required 8-12 hand-crafted slides, custom illustrations, and multiple revision cycles.", "solution": "Deployed Nexus with the 'product_showcase' preset. Content leads write a one-paragraph brief and Nexus generates the full carousel with theme-consistent slides.", "proof_points": ["48hrs/week → 3hrs/week", "Zero design revisions", "Consistent brand voice"]},
            {"title": "Before & After", "before": {"label": "Before Nexus", "description": "48 hours/week on carousel design. 3 revision rounds per campaign. Inconsistent visual quality between team members."}, "after": {"label": "After Nexus", "description": "3 hours/week on review and publishing. 0 revision rounds. Pixel-perfect consistency every time."}, "metric": "93.7%", "metric_label": "Time Reduction"},
            {"client": "Relay.so Marketing Team", "challenge": "Scale Instagram content from 2 campaigns/month to daily posting without hiring designers.", "solution": "Nexus generates campaign carousels from product briefs. Team reviews, tweaks copy, and publishes.", "results": [{"number": "30x", "label": "Content Output"}, {"number": "$84K", "label": "Annual Savings"}]},
            {"quote": "The ROI was obvious in week one. We went from dreading content deadlines to shipping carousels before lunch.", "author": "Sarah Kim", "role": "Design Lead, Relay.so", "avatar_url": A[0]},
            # Unit 2: The Scaling
            {"title": "Scaling Beyond Marketing", "problem": "Sales team wanted custom decks for every prospect. Design team was the bottleneck. Deals stalled waiting for collateral.", "solution": "Sales reps use Nexus CLI to generate prospect-specific carousels from CRM data. No design team involvement needed.", "proof_points": ["Self-serve for sales", "CRM data integration", "Deal cycle -40%"]},
            {"title": "Deal Collateral: Before vs After", "before": {"label": "Before", "description": "Sales requests design team 5 days before demo. Gets generic deck. Customizes 2 slides. Feels unprofessional."}, "after": {"label": "After", "description": "Sales runs nexus generate 30 min before demo. Gets prospect-specific carousel with their logo, data, and pain points."}, "metric": "5 days → 30 min", "metric_label": "Collateral Lead Time"},
            {"client": "Relay.so Sales Team", "challenge": "Close deals faster with personalized collateral without burdening the design team.", "solution": "Nexus CLI integrated into Salesforce workflow. Reps generate custom carousels from opportunity data.", "results": [{"number": "40%", "label": "Faster Close Rate"}, {"number": "22%", "label": "Higher Deal Size"}]},
            {"quote": "I generated a custom deck for a Fortune 500 prospect in the Uber to the meeting. We closed $200K that quarter.", "author": "Derek Huang", "role": "AE, Relay.so", "avatar_url": A[4]},
            # Unit 3: The Transformation
            {"title": "From Cost Center to Revenue Driver", "problem": "Content team was seen as a cost center — necessary but slow. Leadership questioned whether the investment was justified.", "solution": "With Nexus, the content team now runs 3x more campaigns with the same headcount. Attribution shows content-driven pipeline grew 180%.", "proof_points": ["180% pipeline growth", "3x campaign velocity", "Content ROI positive in 30 days"]},
            {"title": "The Transformation Story", "before": {"label": "Before", "description": "Content team: 4 people, 2 campaigns/month, seen as overhead. Budget scrutinized quarterly."}, "after": {"label": "After", "description": "Same 4 people, 60+ campaigns/month, content is top pipeline contributor. Budget doubled."}, "metric": "180%", "metric_label": "Pipeline Growth"},
            {"client": "Relay.so Executive Team", "challenge": "Prove content marketing ROI and justify team expansion.", "solution": "Nexus-enabled content team delivers measurable pipeline impact. Data dashboard tracks content → revenue attribution.", "results": [{"number": "180%", "label": "Pipeline Growth"}, {"number": "2x", "label": "Team Budget Increase"}]},
             {"quote": "Content went from 'nice to have' to our #1 demand gen channel in six months. Nexus made that possible.", "author": "Anika Desai", "role": "CMO, Relay.so", "avatar_url": A[2]},
            # case_study_result (standalone)
            {"client": "Relay.so Revenue Team", "challenge": "Demonstrate end-to-end ROI of Nexus adoption across all departments.", "solution": "Cross-functional Nexus deployment with unified analytics dashboard tracking content → pipeline → revenue.", "results": [{"number": "312%", "label": "Total ROI"}, {"number": "6 months", "label": "Payback Period"}]},
            # progress_rings
            {"title": "Nexus Impact Dashboard", "rings": [
                {"label": "Content Output", "value": 95, "max": 100, "color": "#10b981"},
                {"label": "Cost Savings", "value": 88, "max": 100, "color": "#6366f1"},
                {"label": "Team Satisfaction", "value": 92, "max": 100, "color": "#f59e0b"},
            ]},
            # qr_destination
            {"destination_url": "https://slideforge.dev/case-studies", "cta_text": "Read Full Case Study", "heading": "See the Complete Relay.so Story", "caption": "Scan for detailed metrics and implementation guide", "short_url": "slideforge.dev/cases"},
        ]

    # ── 7. PROCESS TRANSPARENCY (behind the scenes) ────────────────────
    # Arc: Curiosity → Understanding → Appreciation → Trust
    elif preset_id == "process_transparency":
        return [
            # image_headline
            {"headline": "Inside the Nexus Rendering Pipeline", "image_url": I[4], "subheadline": "How a one-sentence brief becomes a 10-slide carousel in under 60 seconds."},
            # timeline
            {"title": "What Happens in 60 Seconds", "steps": [
                {"title": "Brief Parse", "description": "NLP extracts topic, tone, audience, and emotional arc from your description."},
                {"title": "Arc Composition", "description": "Preset engine selects slide types, assigns emotional beats, and sequences the narrative."},
                {"title": "Token Generation", "description": "Material Design color system generates a unique tonal palette. Typography scales auto-calculate."},
                {"title": "Slide Rendering", "description": "Each slide type compiles HTML+CSS with dynamic font sizing, gradient backgrounds, and responsive layouts."},
                {"title": "Carousel Assembly", "description": "Slides are stitched into a single scrollable HTML document with navigation, branding, and export hooks."},
            ]},
            # image_gallery
            {"title": "Pipeline Artifacts", "images": [{"url": I[0], "caption": "Design tokens"}, {"url": I[1], "caption": "Slide JSON"}, {"url": I[4], "caption": "Rendered HTML"}, {"url": I[5], "caption": "Exported PNG"}], "section_caption": "From tokens to pixels — every intermediate step is inspectable"},
            # callout
            {"title": "Zero Black Boxes", "text": "Every slide JSON is inspectable. Every token is deterministic. Every rendering step is debuggable. If something looks wrong, you can trace exactly where the decision was made."},
            # cta
            {"headline": "Transparent by Design", "button_text": "Read the Docs", "button_url": "https://slideforge.dev/docs", "subtext": "Full architecture documentation, API reference, and extension guides."},
        ]

    # ── 8. PRODUCT SHOWCASE (e-commerce) ───────────────────────────────
    # Arc: Desire → Evidence → Trust → Commitment
    elif preset_id == "product_showcase":
        return [
            # image_headline
            {"headline": "Nexus Pro: AI-Native Slide Composition", "image_url": I[5], "subheadline": "The tool that turns product descriptions into campaign-ready carousels."},
            # testimonial_avatar
            {"quote": "Nexus replaced three separate tools in our stack. Design, copy, and export — all one pipeline.", "author": "Tom Lee", "role": "Founder, Typeframe", "avatar_url": A[3]},
            # grid_cards
            {"title": "What You Get", "cards": [
                {"icon": "🧩", "title": "47 Slide Types", "description": "Heroes, charts, testimonials, comparisons, timelines, and more — each engineered for specific communication goals."},
                {"icon": "🎭", "title": "28 Presets", "description": "Pre-built emotional arcs for launches, thought leadership, data stories, spiritual teaching, and beyond."},
                {"icon": "⚡", "title": "CLI + MCP", "description": "Use it from your terminal or let any AI agent call it. Same engine, same output."},
                {"icon": "🔒", "title": "Local-First", "description": "Runs on your machine. No cloud uploads. Your content never leaves your control."},
            ]},
            # testimonial_avatar
            {"quote": "The MCP integration is the killer feature. Our AI agent generates carousels while we sleep.", "author": "Amira Hassan", "role": "Eng Lead, DeckAI", "avatar_url": A[2]},
            # image_gallery
            {"title": "Slide Gallery", "images": [{"url": I[0], "caption": "Hero slides"}, {"url": I[1], "caption": "Data viz"}, {"url": I[4], "caption": "Testimonials"}, {"url": I[5], "caption": "CTAs"}], "section_caption": "Every slide type rendered at production quality"},
            # image_collage
            {"images": [I[0], I[1], I[4], I[5], I[7]], "layout": "grid", "caption": "Nexus in action across different content types"},
            # testimonial_avatar
            {"quote": "Best $0 we ever spent. Open source, runs locally, and the output quality is insane.", "author": "Chris Park", "role": "Indie Hacker", "avatar_url": A[4]},
            # pricing_plan
            {"title": "Simple Pricing", "plans": [
                {"name": "Community", "price": "Free", "features": ["Full CLI access", "All 47 slide types", "28 presets", "MIT licensed"]},
                {"name": "Pro", "price": "$19/mo", "features": ["MCP server hosting", "Custom theme builder", "Priority support", "Early access features"]},
                {"name": "Team", "price": "$49/mo", "features": ["Multi-seat MCP", "Brand kit storage", "API rate limits", "SLA guarantee"]},
             ]},
            # qr_destination
            {"destination_url": "https://slideforge.dev", "cta_text": "Start Free", "heading": "Try Nexus Today", "caption": "Scan to get started in 60 seconds", "short_url": "slideforge.dev"},
        ]

    # ── 9. EVIDENCE ARGUMENT (data story) ──────────────────────────────
    # Arc: Curiosity → Discovery → Understanding → Conviction
    elif preset_id == "evidence_argument":
        return [
            # hero
            {"headline": "The Data Behind Emotional Architecture", "subheadline": "We analyzed 10,000 carousels across 500 brands. The patterns are unmistakable.", "badge": "RESEARCH REPORT", "kicker": "DATA DEEP DIVE"},
            # repeatable ×4: chart + callout
            # 1
            {"title": "Engagement by Slide Position", "chart_type": "bar", "data": [{"label": "Slide 1-3", "value": 92}, {"label": "Slide 4-6", "value": 71}, {"label": "Slide 7-9", "value": 58}, {"label": "Slide 10+", "value": 34}]},
            {"title": "Key Finding", "text": "Carousels with strong emotional hooks in slides 1-3 retain 2.7x more viewers through the full sequence. The first three slides determine whether anyone sees slide 10."},
            # 2
            {"title": "Emotional Arc vs Flat Structure", "chart_type": "bar", "data": [{"label": "Flat (no arc)", "value": 31}, {"label": "Light arc", "value": 54}, {"label": "Strong arc", "value": 87}, {"label": "Full persuasion", "value": 94}]},
            {"title": "The Arc Premium", "text": "Carousels designed with deliberate emotional progression outperform flat content by 3x. The difference isn't design quality — it's narrative structure."},
            # 3
            {"title": "Visual Rhythm Impact", "chart_type": "bar", "data": [{"label": "Same bg all slides", "value": 28}, {"label": "Random variation", "value": 45}, {"label": "DLD rhythm", "value": 73}]},
            {"title": "Dark-Light-Dark Works", "text": "Alternating dark and light backgrounds creates visual rhythm that sustains attention. Monotone carousels lose 62% of viewers by slide 5."},
            # 4
            {"title": "Evidence Stacking Effect", "chart_type": "bar", "data": [{"label": "Claim only", "value": 22}, {"label": "Claim + 1 proof", "value": 48}, {"label": "Claim + 3 proofs", "value": 76}, {"label": "Claim + social proof", "value": 91}]},
            {"title": "Stack Trust, Don't Scatter It", "text": "Each additional proof point compounds credibility. But mixing proof types (data + testimonial + visual) is 40% more effective than repeating the same type."},
            # process_map
            {"title": "Apply This to Your Content", "steps": [
                {"icon": "📊", "title": "Audit", "description": "Score your existing carousels against these benchmarks"},
                {"icon": "🎯", "title": "Prioritize", "description": "Fix the first 3 slides first — they determine everything"},
                {"icon": "🔄", "title": "Rhythm", "description": "Add dark-light alternation to every sequence"},
                {"icon": "📈", "title": "Measure", "description": "Track completion rate, not just impressions"},
             ]},
            # metric_grid
            {"title": "Study Parameters", "metrics": [
                {"value": "10K", "label": "Carousels analyzed"},
                {"value": "500", "label": "Brands studied"},
                {"value": "6mo", "label": "Data collection"},
                {"value": "95%", "label": "Confidence level"},
            ]},
            # table
            {"title": "Performance by Category", "headers": ["Content Type", "Avg Completion", "Avg Saves", "Shares/1K"], "rows": [
                ["Data-heavy", "72%", "8.3%", "4.1"],
                ["Story-driven", "84%", "12.1%", "7.8"],
                ["Mixed (arc + data)", "91%", "15.4%", "9.2"],
            ]},
            # cta
            {"headline": "Want the Full Dataset?", "button_text": "Download Report", "button_url": "https://slideforge.dev/research", "subtext": "Complete methodology, raw data, and interactive dashboard."},
        ]

    # ── 10. CONTRAST DEMONSTRATION (comparison) ────────────────────────
    # Arc: Frustration → Clarity → Hope → Conviction
    elif preset_id == "contrast_demonstration":
        return [
            # headline_subheadline
            {"headline": "Manual Design vs. Nexus Composition", "subheadline": "Same goal. Radically different outcomes."},
            # list (manual)
            {"title": "The Manual Way", "items": [
                "Open Figma. Find the template. Hope it still exists.",
                "Pick colors. Argue about brand compliance. Redo it.",
                "Write copy in Google Docs. Paste into Figma. Formatting breaks.",
                "Export. Realize slide 7 is cut off. Fix. Re-export.",
                "Send to 3 stakeholders. Wait for feedback. Get conflicting notes.",
                "Redo slides 2, 5, and 7. Export again. Ship 3 days late.",
            ]},
            # before_after_story
            {"title": "The Real Cost", "before": {"label": "Manual Process", "description": "12 hours per carousel × 4 carousels/week × $75/hr = $18,000/month in design labor"}, "after": {"label": "Nexus Pipeline", "description": "4 minutes per carousel × 4 carousels/week × $0 = effectively free, with better consistency"}, "metric": "$18K/mo", "metric_label": "Saved in Design Costs"},
            # comparison
            {"title": "Side by Side", "left": {"label": "Manual Design", "items": ["Hours per slide", "Inconsistent branding", "Stakeholder bottleneck", "Format breaks on export"]}, "right": {"label": "Nexus Composition", "items": ["Seconds per slide", "Locked design system", "One-click export", "Pixel-perfect every time"]}},
            # image_comparison
            {"title": "Visual Difference", "before": {"label": "Before", "image_url": I[4], "description": "Stock template, off-brand colors, inconsistent typography"}, "after": {"label": "After", "image_url": I[5], "description": "On-brand composition, consistent system, professional polish"}},
            # list (nexus)
            {"title": "The Nexus Way", "items": [
                "Write one sentence: 'Launch carousel for our new API product.'",
                "Nexus generates 10 slides with emotional arc, theme, and content.",
                "Review. Tweak one headline. Done.",
                "Export to PNG or HTML. Ship.",
                "Total time: 4 minutes. Total cost: $0.",
            ]},
            # comparison_bars
            {"title": "Speed Comparison", "comparison": {"metric": "Time to First Carousel", "left": {"name": "Manual (Figma)", "value": 15}, "right": {"name": "Nexus CLI", "value": 95}}},
            # testimonial_avatar
            {"quote": "I timed it. Nexus generated a better carousel in 47 seconds than my designer produced in 6 hours. We now use both — Nexus for speed, designers for polish.", "author": "Mike Torres", "role": "Head of Marketing, StartupXYZ", "avatar_url": A[1]},
            # cta
            {"headline": "Stop Trading Hours for Slides", "button_text": "Try Nexus Free", "button_url": "https://slideforge.dev", "subtext": "See how fast your team can really ship."},
        ]

    # ── 11. EXPOSURE REVEAL ─────────────────────────────────────────────
    # Arc: Shock → Disgust → Evidence → Pattern → Accountability
    elif preset_id == "exposure_reveal":
        return [
            # hero
            {"headline": "The Template Industrial Complex", "subheadline": "Your carousel template was designed to sell templates, not to communicate your message.", "badge": "INVESTIGATION", "kicker": "WHAT THEY DON'T TELL YOU"},
            # repeatable ×4: metric_grid + callout
            # 1
            {"title": "The Layout Monopoly", "metrics": [
                {"value": "87%", "label": "of carousel templates use the same 5 layouts regardless of content type", "trend": "+12% since 2023"},
            ]},
            {"title": "The Layout Monopoly", "text": "We analyzed 500 popular carousel templates. 87% use identical grid structures. The 'variety' is color and font — not composition. Your audience sees through it."},
            # 2
            {"title": "The Customization Illusion", "metrics": [
                {"value": "73%", "label": "of template users never customize beyond colors and text", "trend": "Industry-wide"},
            ]},
            {"title": "The Customization Illusion", "text": "Templates give the illusion of control. You change the text. You change the colors. But the underlying composition — the emotional structure — is fixed. And it's wrong for your message."},
            # 3
            {"title": "Composed vs Templated", "metrics": [
                {"value": "4.2x", "label": "engagement gap between templated and composed carousels", "trend": "Widening"},
            ]},
            {"title": "Composed vs Templated", "text": "Emotionally-architected carousels — where every slide advances a deliberate narrative arc — outperform template-based carousels by 4.2x. Not because they look different. Because they think different."},
            # 4
            {"title": "The Completion Gap", "metrics": [
                {"value": "92%", "label": "scroll completion on emotionally-architected carousels vs 34% on templates", "trend": "+58 points"},
            ]},
            {"title": "The Completion Gap", "text": "People finish what resonates. Templates are designed to be universal — which means they resonate with no one. Emotional architecture is designed for THIS audience, THIS message, THIS moment."},
            # chart
            {"title": "Engagement: Templates vs Composition", "chart_type": "bar", "data": [{"label": "Generic template", "value": 23}, {"label": "Customized template", "value": 38}, {"label": "AI-composed", "value": 72}, {"label": "Emotional architecture", "value": 96}]},
            # column_chart
            {"title": "Completion Rate by Slide Type", "chart_type": "grouped_column", "categories": ["Hook", "Context", "Evidence", "Proof", "CTA"], "series": [{"name": "Templates", "values": [34, 28, 22, 18, 12]}, {"name": "Composed", "values": [78, 72, 68, 65, 58]}]},
            # image_callout
            {"image_url": I[6], "callouts": [{"label": "This slide looks like every other template", "description": "Grid layout, icon row, centered text — the template industrial complex at work", "x": 50, "y": 40}], "description": "Spot the template. You've seen this exact layout 1,000 times."},
            # checklist_action_plan
            {"title": "Escape the Template Trap", "items": [
                {"step": "Audit your last 10 carousels", "description": "Count how many use the same 3 layouts"},
                {"step": "Map your emotional intent", "description": "What should the audience FEEL at each slide?"},
                {"step": "Compose, don't decorate", "description": "Use slide types that serve your narrative, not your template library"},
                {"step": "Measure completion, not impressions", "description": "Did they scroll to the end? That's the real metric"},
            ]},
            # stat_row
            {"title": "The Numbers Don't Lie", "stats": [
                {"value": "4.2x", "label": "engagement lift"},
                {"value": "92%", "label": "completion rate"},
                {"value": "58pt", "label": "gap vs templates"},
                {"value": "3.1x", "label": "share rate"},
            ]},
            # qr_destination
            {"destination_url": "https://slideforge.dev/manifesto", "cta_text": "Read the Manifesto", "heading": "Break Free from Templates", "caption": "Scan for the full investigation", "short_url": "slideforge.dev/manifesto"},
        ]

    # ── 12. SKILL TRANSFER (educational how-to) ────────────────────────
    # Arc: Confusion → Understanding → Practice → Mastery
    elif preset_id == "skill_transfer":
        return [
            # definition
            {"term": "Emotional Architecture", "phonetic": "/ɪˌmoʊʃənl ˈɑːrkɪtɛktʃər/", "definition": "The deliberate design of communication sequences that guide audiences through specific emotional progressions toward a predetermined action.", "context": "Every effective campaign uses emotional architecture — most just don't know it. Learning the framework turns intuition into a repeatable skill."},
            # repeatable ×5: section_divider + image_caption + text_block
            # Module 1
            {"kicker": "MODULE 1", "title": "The Hook Principle", "subtitle": "Why the first 3 seconds decide everything"},
            {"image_url": I[0], "caption": "Slide 1 determines whether slides 2-10 exist", "description": "The hook isn't decoration — it's a contract. You promise something worth their time. Break that promise and they scroll past."},
            {"title": "The Hook Formula", "body": "Every hook has three components: (1) a pattern interrupt — something unexpected that stops the scroll, (2) a promise — implicit or explicit, of what's coming, (3) a time anchor — 'this will take X slides' or 'by the end you'll know Y.' Without all three, the hook leaks."},
            # Module 2
            {"kicker": "MODULE 2", "title": "Emotional Arcs", "subtitle": "The hidden structure of every viral carousel"},
            {"image_url": I[1], "caption": "Emotional arcs are invisible but their effects are measurable", "description": "A viral carousel isn't random — it follows a predictable emotional trajectory. Learn to read and design these arcs."},
            {"title": "The Four Universal Arcs", "body": "1. Problem → Solution → Proof → Action (the workhorse). 2. Curiosity → Discovery → Reframing → Conviction (thought leadership). 3. Skepticism → Evidence → Trust → Commitment (social proof). 4. Shock → Pattern → Accountability → Movement (exposé). Every campaign fits one of these."},
            # Module 3
            {"kicker": "MODULE 3", "title": "Slide Type Selection", "subtitle": "Matching composition to communication intent"},
            {"image_url": I[2], "caption": "Each slide type solves a specific communication problem", "description": "A metric_grid isn't interchangeable with a stat_row. A myth_fact isn't a before_after. Know what each does."},
            {"title": "The Selection Framework", "body": "Ask: What does this slide need to DO? Prove something → case_study_result or chart. Show contrast → comparison or before_after. Build trust → testimonial_avatar or logo_cloud. Drive action → cta or qr_destination. The slide type is the verb, not the decoration."},
            # Module 4
            {"kicker": "MODULE 4", "title": "Visual Rhythm", "subtitle": "Dark-light-dark and why monotone kills engagement"},
            {"image_url": I[3], "caption": "Visual monotone is the silent killer of carousel engagement", "description": "When every slide looks the same, the brain stops paying attention. Rhythm re-engages the visual cortex."},
            {"title": "The DLD Rule", "body": "Alternate dark and light backgrounds. Never repeat the same bg_style twice in a row. This isn't aesthetic advice — it's neuroscience. The visual cortex responds to contrast. Monotone backgrounds trigger habituation, and habituation kills attention."},
            # Module 5
            {"kicker": "MODULE 5", "title": "The Preset System", "subtitle": "Composing at scale with emotional arc templates"},
            {"image_url": I[4], "caption": "Presets are compositions, not templates", "description": "A preset doesn't tell you what to say. It tells you the emotional journey your audience needs to take."},
            {"title": "Presets vs Templates", "body": "A template is a layout. A preset is a narrative architecture. The 'announcement' preset doesn't care if you're launching a product, a movement, or a idea — it knows the audience needs excitement → understanding → proof → action. That's the arc. Your content fills the beats."},
             # myth_fact
            {"myth": "Good design is what makes carousels work", "fact": "Good narrative structure is what makes carousels work. Design is the vehicle, not the engine.", "explanation": "The highest-performing carousels often have simple, even plain design. What they have is a relentless emotional arc that carries the viewer from hook to action."},
            # faq
            {"title": "Frequently Asked Questions", "items": [
                {"question": "How long does it take to learn emotional architecture?", "answer": "The basics take one carousel. Mastery takes 10. Start with one preset, one message, and measure the difference."},
                {"question": "Do I need design skills?", "answer": "No. The preset system handles visual composition. You provide the content and the emotional intent."},
                {"question": "Can I customize the presets?", "answer": "Presets are starting points. The pool-based composition system lets you remix slide types while keeping the emotional arc intact."},
            ]},
            # checklist_action_plan
            {"title": "Your Practice Assignments", "items": [
                {"step": "Rewrite your last carousel hook", "description": "Apply the three-part hook formula: interrupt, promise, anchor"},
                {"step": "Map the emotional arc of a competitor's carousel", "description": "What do they want the audience to feel at each slide?"},
                {"step": "Generate one carousel using a preset", "description": "Compare the AI-composed arc to your manual version"},
                {"step": "Measure completion rate", "description": "Track how many viewers reach the final slide"},
            ]},
            # cta
            {"headline": "You Now Know More Than 99% of Content Creators", "button_text": "Start Creating", "button_url": "https://slideforge.dev", "subtext": "Use the CLI or MCP server to compose your first emotionally-architected carousel."},
        ]

    # ── 13. DEEP DIVE (feature deepdive) ───────────────────────────────
    # Arc: Curiosity → Understanding → Mastery → Confidence
    elif preset_id == "deep_dive":
        return [
            # feature
            {"icon": "🧬", "title": "The Composition Engine", "description": "Nexus doesn't pick slides randomly. It composes — selecting each slide type based on the emotional position in the narrative arc, the content density of the brief, and the target audience's cognitive load budget.", "number": "01"},
            # split_features
            {"title": "Nexus vs Traditional Tools", "features": [
                {"icon": "🧬", "title": "Nexus", "description": "47 slide types, emotional arc engine, dynamic typography, pool-based composition"},
                {"icon": "📐", "title": "Templates", "description": "5 fixed layouts, fill-in-the-blank, static sizing, one structure per template"},
                {"icon": "⚡", "title": "Speed", "description": "60 seconds vs 12 hours per carousel"},
                {"icon": "🎨", "title": "Consistency", "description": "Locked design system vs manual alignment"},
            ]},
            # definition
            {"term": "Slide Composition", "phonetic": "/slaɪd kəm-pə-zi-shən/", "definition": "The algorithmic selection and sequencing of slide types to create an emotionally coherent narrative. Unlike templating, composition adapts to the content, not the other way around.", "context": "Composition is what separates Nexus from every 'AI slide generator' that just fills in a template."},
            # image_callout
            {"image_url": I[5], "callouts": [{"label": "Dynamic font scaling", "description": "Text sizes adjust based on content density — dense slides get smaller type, sparse slides breathe", "x": 30, "y": 40}, {"label": "Theme token system", "description": "Material Design 3 tonal palette with per-preset color overrides", "x": 70, "y": 60}], "description": "Every visual element is computed, not hardcoded."},
            # process_map
            {"title": "How Composition Works", "steps": [
                {"icon": "📝", "title": "Brief Analysis", "description": "NLP extracts topic, tone, audience, and intent from natural language"},
                {"icon": "🧠", "title": "Arc Selection", "description": "Matches intent to one of 7 emotional arc categories across 28 presets"},
                {"icon": "🎲", "title": "Slide Sequencing", "description": "Selects slide types that advance the arc, enforcing DLD rhythm and variety constraints"},
                {"icon": "🎨", "title": "Visual Rendering", "description": "Each slide compiles with dynamic typography, theme tokens, and content-adaptive layout"},
            ]},
            # before_after_story
            {"title": "Template vs Composition", "before": {"label": "Template Output", "description": "Fixed 5-slide structure. Same grid on every slide. Text crammed into boxes designed for shorter content."}, "after": {"label": "Composed Output", "description": "8-15 slides adapted to content density. Each slide type chosen for its communication function. Typography scales to fit."}, "metric": "3.1x", "metric_label": "Engagement Improvement"},
            # qr_destination
            {"destination_url": "https://slideforge.dev/docs/composition", "cta_text": "Read the Docs", "heading": "Full Composition Documentation", "caption": "Scan for API reference and extension guides", "short_url": "slideforge.dev/docs"},
        ]

    # ── 14. PRINCIPLE EDUCATION (psychological framework) ──────────────
    # Arc: Curiosity → Recognition → Mastery → Action
    elif preset_id == "principle_education":
        return [
            # definition
            {"term": "Cialdini's Seven Principles", "phonetic": "/si-əl-DI-niz ˈsɛvən ˈprɪnsəpəlz/", "definition": "Robert Cialdini's framework of seven universal persuasion principles: Reciprocity, Commitment, Social Proof, Authority, Liking, Scarcity, and Unity.", "context": "Every effective campaign implicitly or explicitly uses these principles. Making them explicit turns persuasion from art into engineering."},
            # repeatable ×5: image_callout + myth_fact
            {"image_url": I[0], "callouts": [{"label": "Principle #1: Reciprocity", "description": "Give value first. The audience feels obligated to reciprocate — with attention, engagement, or action.", "x": 50, "y": 40}], "description": "Reciprocity is why free content converts better than paid advertising."},
            {"myth": "Scarcity only works for physical products", "fact": "Information scarcity ('this insight won't be shared again') drives 2.8x more engagement than abundance framing", "explanation": "Scarcity is about perceived value, not physical limitation. Limited-time insights, exclusive data, and 'first to know' framing all trigger the same response."},
            {"image_url": I[1], "callouts": [{"label": "Principle #2: Social Proof", "description": "People follow people. Testimonials, user counts, and 'trusted by' logos reduce decision friction.", "x": 50, "y": 40}], "description": "Social proof is why testimonial_avatar is one of the most-used slide types."},
            {"myth": "Authority requires credentials", "fact": "Perceived authority comes from confidence and specificity, not titles. 'We analyzed 10,000 slides' beats 'our expert says.'", "explanation": "Authority is established through demonstrated knowledge, not claimed credentials. Data beats titles."},
            {"image_url": I[2], "callouts": [{"label": "Principle #3: Commitment", "description": "Small yeses lead to big yeses. Each micro-commitment (save, share, read) escalates investment.", "x": 50, "y": 40}], "description": "Commitment explains why carousel completion rate predicts conversion better than click-through rate."},
            {"myth": "Liking is about being likeable", "fact": "Liking is about similarity and shared identity. 'People like me use this' is more powerful than 'this person is charismatic.'", "explanation": "The liking principle works through identification, not charm. That's why founder stories outperform celebrity endorsements for niche products."},
            {"image_url": I[3], "callouts": [{"label": "Principle #4: Unity", "description": "Shared identity. 'We' language. 'Our community.' Unity turns customers into members and members into advocates.", "x": 50, "y": 40}], "description": "Unity is Cialdini's most overlooked principle — and the most powerful for building movements."},
            {"myth": "Unity is just branding", "fact": "Unity is identity-level belonging. Branding is visual. Unity is tribal. 'I am a Nexus user' vs 'I use Nexus.'", "explanation": "Unity explains why open-source communities outperform proprietary user bases in advocacy and retention."},
            # 5
            {"image_url": I[4], "callouts": [{"label": "Principle #5: Liking", "description": "People say yes to those they like. Similarity, compliments, and cooperation build the liking bridge.", "x": 50, "y": 40}], "description": "Liking explains why founder-led carousels outperform corporate-polished ones."},
             {"myth": "Scarcity and urgency are the same thing", "fact": "Scarcity is about limited supply; urgency is about limited time. Combined, they create the most powerful persuasion force in marketing.", "explanation": "The best CTAs use both: 'Only 3 spots left' (scarcity) + 'Offer ends tonight' (urgency). Together they're exponentially more effective."},
            # process_map
            {"title": "Apply the Principles", "steps": [
                {"icon": "🔍", "title": "Audit", "description": "Map which principles your current content uses (and which it ignores)"},
                {"icon": "🧩", "title": "Integrate", "description": "Assign one principle per slide. Let the preset structure do the sequencing."},
                {"icon": "📊", "title": "Measure", "description": "Track which principles drive the most engagement for your audience"},
                {"icon": "🔄", "title": "Iterate", "description": "Double down on what works. Drop what doesn't."},
            ]},
            # checklist_action_plan
            {"title": "Implementation Checklist", "items": [
                {"step": "Identify your primary principle", "description": "Which of the 7 fits your campaign goal best?"},
                {"step": "Map principles to slide types", "description": "Social proof → testimonial. Authority → case_study. Scarcity → cta."},
                {"step": "Generate with a preset", "description": "Let the emotional arc handle the principle sequencing"},
                {"step": "A/B test principle emphasis", "description": "Same content, different principle focus. Measure the delta."},
            ]},
            # faq
            {"title": "Common Questions", "questions": [
                {"question": "Which principle should I start with?", "answer": "Start with social proof — it's the most universally applicable and the easiest to measure. Add scarcity once you have proof of value."},
                {"question": "Can I use multiple principles in one carousel?", "answer": "Yes, but assign one primary principle per slide. The preset structure handles the sequencing so principles don't compete."},
                {"question": "How do I measure which principles work?", "answer": "Track completion rate per slide type. Testimonial slides with social proof should outperform generic claims. Measure the delta."},
            ]},
            # cta
            {"headline": "Engineer Your Next Campaign", "button_text": "Start with a Preset", "button_url": "https://slideforge.dev/presets", "subtext": "28 presets, each built on proven persuasion architecture."},
        ]

    # ── 15. TECHNIQUE MASTERY (psychological persuasion) ───────────────
    # Arc: Curiosity → Recognition → Mastery → Responsibility
    elif preset_id == "technique_mastery":
        return [
            # definition
            {"term": "Narrative Transportation", "phonetic": "/ˈnærətɪv trænspɔːrˈteɪʃən/", "definition": "The phenomenon where audiences become absorbed into a story, reducing counter-arguing and increasing belief in the message.", "context": "Transported audiences are 3x more likely to change attitudes and 2x more likely to take action. Every great carousel transports."},
            # image_callout
            {"image_url": I[3], "callouts": [{"label": "Transportation happens in the gap", "description": "The space between what you say and what the audience imagines is where persuasion lives", "x": 50, "y": 40}], "description": "The best slides don't show everything — they leave room for the audience's imagination."},
            # image_caption
            {"image_url": I[4], "caption": "Technique 1: Pattern Interrupt", "description": "Break the scroll pattern. Unexpected visuals, contradictory statements, or provocative questions force the brain to re-engage."},
            # process_map
            {"title": "The Five Techniques", "steps": [
                {"icon": "⚡", "title": "Pattern Interrupt", "description": "Break the scroll. Make them stop. Then you have their attention."},
                {"icon": "🎯", "title": "Anchoring", "description": "Set the reference frame. The first number they see becomes the benchmark."},
                {"icon": "🔗", "title": "Commitment Escalation", "description": "Small yeses → big yeses. Every interaction is a micro-commitment."},
                {"icon": "🪞", "title": "Identity Framing", "description": "People don't buy products. They buy the person they become."},
            ]},
            # radar_chart
            {"title": "Technique Mastery Profile", "axes": [
                {"label": "Pattern Interrupt", "value": 85},
                {"label": "Anchoring", "value": 92},
                {"label": "Commitment", "value": 78},
                {"label": "Identity", "value": 88},
                {"label": "Transport", "value": 95},
            ]},
            # callout
            {"title": "The Responsibility Frame", "text": "These techniques work. That's exactly why they demand ethical use. The line between persuasion and manipulation is consent — does the audience benefit from the outcome you're steering them toward? If yes, these are tools. If no, they're weapons."},
            # cta
            {"headline": "Use These Powers Wisely", "button_text": "Study the Framework", "button_url": "https://slideforge.dev/ethics", "subtext": "Our ethical guidelines for persuasive content design."},
        ]

    # ── 16. WISDOM TRANSMISSION (spiritual teaching) ───────────────────
    # Arc: Wonder → Understanding → Deepening → Commitment
    elif preset_id == "wisdom_transmission":
        return [
            # definition
            {"term": "Dharma", "phonetic": "/ˈdɑːrmə/", "definition": "The cosmic order that sustains the universe. In communication, dharma is the truth that exists beneath the surface of words — the message that the audience already knows but hasn't yet articulated.", "context": "Teaching isn't about filling empty vessels. It's about helping people remember what they already know."},
            # repeatable ×5: image_quote + text_block
            {"quote": "The teacher who is indeed wise does not bid you to enter the house of his wisdom, but rather leads you to the threshold of your mind.", "author": "Khalil Gibran", "image_url": A[0]},
            {"title": "The First Teaching", "body": "Before you can transmit wisdom, you must learn to listen. Not to respond — to understand. Every audience carries a question they haven't yet formed words for. Your carousel should answer that question before they ask it."},
            {"quote": "In the beginner's mind there are many possibilities, but in the expert's mind there are few.", "author": "Shunryu Suzuki", "image_url": A[1]},
            {"title": "The Second Teaching", "body": "Simplicity is not the absence of complexity — it's the mastery of it. The most profound ideas can be expressed in one slide. If you need ten slides to explain something, you haven't understood it yet."},
            {"quote": "The only true wisdom is in knowing you know nothing.", "author": "Socrates", "image_url": A[2]},
            {"title": "The Third Teaching", "body": "Humility opens the door to understanding. When you present wisdom, don't position yourself as the expert — position yourself as the fellow seeker. The audience learns more from someone walking beside them than someone standing above them."},
            {"quote": "We are not human beings having a spiritual experience. We are spiritual beings having a human experience.", "author": "Pierre Teilhard de Chardin", "image_url": A[3]},
            {"title": "The Fourth Teaching", "body": "Every communication is an act of service. You're not performing knowledge — you're offering it. The difference is in the energy. Service invites. Performance demands. Choose service."},
            {"quote": "The wound is the place where the Light enters you.", "author": "Rumi", "image_url": A[4]},
            {"title": "The Fifth Teaching", "body": "Vulnerability is the bridge between teacher and student. Share your struggles, not just your insights. The audience trusts the person who admits they're still learning more than the one who claims to have arrived."},
            # checklist_action_plan
            {"title": "Practice Guidelines", "items": [
                {"step": "Sit with the topic before composing", "description": "Let the wisdom settle before you try to transmit it"},
                {"step": "Write the audience's question first", "description": "What are they actually asking? Answer that."},
                {"step": "Use one image per insight", "description": "Let the visual carry half the meaning"},
                {"step": "End with an invitation, not a command", "description": "Wisdom is offered, not imposed"},
            ]},
            # callout
            {"title": "The Transmission Is Complete When", "text": "The audience doesn't remember your words — they remember their own realization. That's when teaching becomes transformation."},
            # text_columns
            {"title": "Three Modes of Transmission", "columns": [
                {"heading": "Spoken", "body": "Words carry intention. The teacher's voice shapes how wisdom lands — not just what is said, but how it is offered."},
                {"heading": "Written", "body": "Text endures. Written wisdom survives the session, the teacher, and the era. It becomes a reference the student returns to."},
                {"heading": "Lived", "body": "The deepest teaching is embodied. When the teacher lives what they teach, every interaction becomes a transmission."},
            ]},
            # cta
            {"headline": "Continue the Practice", "button_text": "Explore More Teachings", "button_url": "https://slideforge.dev/wisdom", "subtext": "A growing library of wisdom transmission presets and examples."},
        ]

    # ── 17. GUIDED EXPERIENCE (spiritual practice) ─────────────────────
    # Arc: Stillness → Focus → Deepening → Peace
    elif preset_id == "guided_experience":
        return [
            # image_headline
            {"headline": "A Three-Minute Breathing Practice", "image_url": I[7], "subheadline": "Wherever you are right now, this is enough. Let's begin."},
            # text_block
            {"title": "Settling In", "body": "Close your eyes or soften your gaze. Feel the weight of your body in the chair. Notice the temperature of the air on your skin. You don't need to do anything right now except be here. This carousel is your container for the next three minutes."},
            # image_caption (×4)
            {"image_url": I[0], "caption": "Breathe in for 4 counts", "description": "Let the breath arrive. Don't force it. Simply notice the air entering your body. Count: one... two... three... four."},
            {"image_url": I[1], "caption": "Hold for 4 counts", "description": "The breath is held. Not strained — held. Like cupping water in your hands. Notice the stillness between the inhale and the exhale."},
            {"image_url": I[2], "caption": "Exhale for 6 counts", "description": "Release. Let the breath leave slowly. One... two... three... four... five... six. Feel the body soften with each count."},
            {"image_url": I[3], "caption": "Rest in the gap", "description": "After the exhale, there's a moment of emptiness. Don't rush to fill it. That gap is where peace lives."},
            # callout
            {"title": "Returning", "text": "Gently bring your awareness back to the room. Wiggle your fingers. Notice how you feel compared to three minutes ago. Carry this quality of attention into whatever comes next."},
            # image_quote
            {"quote": "Feelings come and go like clouds in a windy sky. Conscious breathing is my anchor.", "author": "Andy Puddicombe", "image_url": A[3]},
            # cta
            {"headline": "Practice Again Tomorrow", "button_text": "Save This Carousel", "button_url": "https://slideforge.dev", "subtext": "Return to this practice whenever you need to find stillness."},
        ]

    # ── 18. STORY JOURNEY (narrative arc) ──────────────────────────────
    # Arc: Intrigue → Engagement → Tension → Catharsis → Action
    elif preset_id == "story_journey":
        return [
            # hero
            {"headline": "The Builder Who Almost Quit", "subheadline": "A story about the space between failure and breakthrough.", "badge": "A TRUE STORY", "kicker": "CHAPTER 1"},
            # repeatable ×5: image_headline + text_block
            {"headline": "The Beginning: A Crazy Idea", "image_url": I[0], "subheadline": "June 2023 — a solo developer, a laptop, and a conviction that slides shouldn't take all day."},
            {"title": "The Doubt", "body": "Six months in. Zero users. The Python prototype worked, but nobody cared. 'Why would anyone use a CLI to make slides?' The forums were silent. The GitHub stars trickled. Every rational signal said quit."},
            {"headline": "The Turning Point", "image_url": I[1], "subheadline": "One DM changed everything. 'Hey, your tool just saved me 8 hours on a client deck. Can I sponsor you?'"},
            {"title": "The Proof", "body": "That DM led to a conversation. The conversation led to a case study. The case study led to 40 GitHub stars in a week. The lesson: build for one person, not a market. The market finds you."},
            {"headline": "The Rewrite", "image_url": I[2], "subheadline": "Rust. The whole thing. From scratch. Because the Python version was 10x too slow for production use."},
            {"title": "The Dark Hour", "body": "Month 3 of the Rust rewrite. Three compiler errors for every feature. The borrow checker had opinions about everything. 'Why am I doing this to myself?' Because the Python version couldn't render 10 slides in under a second. And speed was the whole point."},
            {"headline": "The Breakthrough", "image_url": I[3], "subheadline": "First benchmark: 47 slides rendered in 0.8 seconds. The Python version took 12."},
            {"title": "The Realization", "body": "The tool worked. But the real breakthrough wasn't technical — it was conceptual. Slides aren't documents. They're compositions. And compositions need a composer, not a template engine. That insight became the composition engine."},
            {"headline": "The Present", "image_url": I[4], "subheadline": "47 slide types. 28 campaign presets. 85 tests passing. One binary. And a community that's just getting started."},
            {"title": "What Comes Next", "body": "The tool ships. The community grows. The presets multiply. But the mission stays the same: make slide creation so fast that the bottleneck shifts from production to imagination. That's when things get interesting."},
            # definition
            {"term": "The Builder's Journey", "phonetic": "/ðə ˈbɪldərz ˈdʒɜːrni/", "definition": "Every product that matters was built by someone who almost quit. The difference between shipped and abandoned is one more day of showing up.", "context": "If you're building something and it feels impossible, you're exactly where every meaningful project starts."},
            # before_after_story
            {"title": "The Whole Arc", "before": {"label": "Day 1", "description": "Zero code. Zero users. Zero evidence this would work. Just a conviction that slides shouldn't take all day."}, "after": {"label": "Today", "description": "85 tests passing. 28 presets. 47 slide types. A tool that generates production carousels in 60 seconds."}, "metric": "547", "metric_label": "Days of Building"},
            # cta
            {"headline": "What's Your Builder's Journey?", "button_text": "Start Building", "button_url": "https://github.com/ishan-parihar/slideforge-rust", "subtext": "Open source. MIT licensed. Fork it. Ship it. Make it yours."},
        ]

    # ── 19. EMOTIONAL ENGINEERING (emotional funnel) ───────────────────
    # Arc: Fear → Urgency → Contrast → Hope → Trust → Action
    elif preset_id == "emotional_engineering":
        return [
            # hero
            {"headline": "Your Competitors Shipped 12 Carousels While You Read This", "subheadline": "Every day you spend manual-designing, the gap widens. This isn't about tools. It's about survival.", "badge": "URGENT", "kicker": "THE COST OF INACTION"},
            # repeatable ×3: metric_grid + callout
            {"title": "The Fear Is Rational", "metrics": [
                {"value": "12x", "label": "output gap between teams using composition engines and manual designers", "trend": "Growing 3x/quarter"},
            ]},
            {"title": "The Fear Is Rational", "text": "You're not being paranoid. The content gap is real and accelerating. Teams with AI-native pipelines produce 12x more content with the same headcount. This isn't a future threat — it's today's reality."},
            {"title": "The Money Is Leaving", "metrics": [
                {"value": "$47K", "label": "annual cost of manual carousel production at $75/hr, 12hrs/week", "trend": "+18% YoY"},
            ]},
            {"title": "The Money Is Leaving", "text": "Every hour your team spends dragging boxes in Figma is an hour not spent on strategy, research, or relationship building. The real cost isn't the design hours — it's the opportunity cost."},
            {"title": "The Attention Cliff", "metrics": [
                {"value": "68%", "label": "of audiences never see slide 6 if slides 1-3 don't hook them", "trend": "Industry average"},
            ]},
            {"title": "The Attention Cliff", "text": "Your beautiful 10-slide carousel has a 32% chance of reaching slide 6. The other 68% scroll past after slide 3. The problem isn't design quality — it's emotional architecture."},
            # before_after_story
            {"title": "Two Futures", "before": {"label": "Stay Manual", "description": "12 hours per carousel. 4 per month. Same templates. Declining engagement. Watching competitors ship faster."}, "after": {"label": "Adopt Nexus", "description": "4 minutes per carousel. 4 per week. Unique compositions every time. Rising engagement. Leading the market."}, "metric": "756hrs/yr", "metric_label": "Difference in Production Time"},
            # funnel_chart
            {"title": "The Conversion Funnel", "stages": [
                {"label": "Awareness", "value": 10000},
                {"label": "Interest", "value": 3200},
                {"label": "Consideration", "value": 1100},
                {"label": "Intent", "value": 440},
                {"label": "Action", "value": 180},
            ]},
            # process_map
            {"title": "The Escape Route", "steps": [
                {"icon": "🚨", "title": "Acknowledge", "description": "Your current process is unsustainable. Admit it."},
                {"icon": "⚡", "title": "Adopt", "description": "Deploy Nexus. CLI for you, MCP for your agents. Today."},
                {"icon": "📈", "title": "Accelerate", "description": "Ship 10x more. Measure what works. Drop what doesn't."},
                {"icon": "🏆", "title": "Lead", "description": "By the time competitors adopt AI pipelines, you'll be 6 months ahead."},
            ]},
            # testimonial_avatar
            {"quote": "We were the team that 'didn't need AI tools.' Six months later, our competitor had 3x our output and half our headcount. We adopted Nexus the next day.", "author": "Anonymous", "role": "Marketing Director, Fortune 500", "avatar_url": A[3]},
            # qr_destination
            {"destination_url": "https://slideforge.dev", "cta_text": "Start Now — It's Free", "heading": "The Time to Act Is Now", "caption": "Scan. Install. Ship. 60 seconds.", "short_url": "slideforge.dev"},
        ]

    # ── 20. EVENT MOBILIZATION ──────────────────────────────────────────
    # Arc: Interest → Excitement → Commitment → Action
    elif preset_id == "event_mobilization":
        return [
            # hero
            {"headline": "SlideForge Summit 2026", "subheadline": "The first conference dedicated to AI-native content composition. One day. 500 seats. Virtual + In-Person.", "badge": "SEPT 15, 2026", "kicker": "SAVE THE DATE"},
            # list
            {"title": "What's Happening", "items": [
                "Keynote: The Death of the Design Sprint (Ishan Parihar)",
                "Workshop: Building Your First Campaign Preset",
                "Panel: AI Composition vs Human Design — The Real Debate",
                "Live Demo: From Brief to Carousel in 60 Seconds",
                "Networking: Meet the community building the future of slides",
            ]},
            # image_quote
            {"quote": "The future of content isn't AI replacing designers. It's AI freeing designers to do what they actually got into design for — solving problems, not dragging boxes.", "author": "Ishan Parihar", "image_url": A[1]},
            # grid_cards
            {"title": "Why Attend", "cards": [
                {"icon": "🎓", "title": "Learn", "description": "Hands-on workshops with the SlideForge team. Build real carousels, not toy demos."},
                {"icon": "🤝", "title": "Connect", "description": "Meet 500 content creators, developers, and marketers building with AI-native tools."},
                {"icon": "🚀", "title": "Ship", "description": "Leave with a complete campaign ready to publish. Not a notebook of ideas — a working pipeline."},
                {"icon": "🎁", "title": "Exclusive Access", "description": "Attendees get early access to Nexus 4.0 and the preset marketplace."},
             ]},
            # image_collage
            {"images": [I[0], I[1], I[2], I[5]], "layout": "grid", "caption": "Summit highlights from previous events"},
            # qr_destination
            {"destination_url": "https://slideforge.dev/summit", "cta_text": "Reserve Your Seat", "heading": "September 15, 2026", "caption": "Scan to register — early bird pricing ends Aug 1", "short_url": "slideforge.dev/summit"},
        ]

    # ── 21. URGENCY MOBILIZATION (political mobilize) ──────────────────
    # Arc: Urgency → Fear → Determination → Action
    elif preset_id == "urgency_mobilization":
        return [
            # hero
            {"headline": "48 Hours Left. Your Vote Matters.", "subheadline": "District 7 registration closes Friday at midnight. If you haven't registered, nothing else matters.", "badge": "DEADLINE FRIDAY", "kicker": "TIME-CRITICAL"},
            # grid_cards
            {"title": "What's at Stake", "cards": [
                {"icon": "🏫", "title": "School Funding", "description": "The new budget allocates $14M more to District 7 schools — but only if you vote for the bond measure."},
                {"icon": "🏠", "title": "Housing Policy", "description": "Affordable housing mandate expires Dec 31. Your vote extends it for 10 years."},
                {"icon": "🏥", "title": "Clinic Access", "description": "Three community clinics close without renewed funding. 40,000 patients affected."},
                {"icon": "🌳", "title": "Green Space", "description": "Park renovation funding is on the ballot. It's the first new park investment in 15 years."},
            ]},
            # metric_grid
            {"title": "The Numbers", "metrics": [
                {"value": "48hrs", "label": "Until deadline"},
                {"value": "14K", "label": "Unregistered voters"},
                {"value": "$14M", "label": "School funding at stake"},
                {"value": "3", "label": "Clinics closing"},
            ]},
            # process_map
            {"title": "Register in 3 Steps", "steps": [
                {"icon": "📱", "title": "Scan", "description": "Use the QR code to open your state's registration page"},
                {"icon": "✍️", "title": "Fill", "description": "Name, address, ID — takes 2 minutes"},
                {"icon": "✅", "title": "Confirm", "description": "You'll get a confirmation email. Screenshot it."},
            ]},
            # testimonial_avatar
             {"quote": "I didn't vote in 2022. My neighborhood lost its clinic. Don't let that happen to yours. Register today.", "author": "Maria Santos", "role": "District 7 Resident", "avatar_url": A[2]},
            # image_stat
            {"image_url": I[3], "stat": "48hrs", "stat_label": "Until registration closes", "caption": "Every hour counts. Register now."},
            # qr_destination
            {"destination_url": "https://vote.gov", "cta_text": "Register Now", "heading": "Don't Wait Until It's Too Late", "caption": "Scan to register — 2 minutes that change everything", "short_url": "vote.gov"},
        ]

    # ── 22. COLLECTIVE MOBILIZATION (sociological movement) ────────────
    # Arc: Anger → Belonging → Hope → Commitment
    elif preset_id == "collective_mobilization":
        return [
            # hero
            {"headline": "We Are 14,000 Strong.", "subheadline": "One community. One demand. One chance to change District 7 forever.", "badge": "MOVEMENT UPDATE", "kicker": "COMMUNITY ACTION"},
            # image_headline
            {"headline": "This Started With One Person", "image_url": I[5], "subheadline": "Six months ago, one resident started a petition. Today, 14,000 signatures and counting."},
            # metric_grid (×3)
            {"title": "The Movement by Numbers", "metrics": [
                {"value": "14,247", "label": "signatures collected in 6 months across District 7", "trend": "+890 this week"},
            ]},
            {"title": "Neighborhood Reach", "metrics": [
                {"value": "12", "label": "neighborhoods organized with volunteer captains in every district", "trend": "+3 this month"},
            ]},
            {"title": "Political Impact", "metrics": [
                {"value": "3", "label": "city council meetings where our coalition spoke — and was heard", "trend": "Next: Sept 12"},
            ]},
            # grid_cards
            {"title": "Our Demands", "cards": [
                {"icon": "🏠", "title": "Housing", "description": "30% affordable units in all new developments. No exceptions."},
                {"icon": "🏥", "title": "Healthcare", "description": "Renewed funding for all 3 community clinics. Permanent, not temporary."},
                {"icon": "🌳", "title": "Green Space", "description": "Park renovation fund. Every neighborhood deserves a park."},
                {"icon": "📚", "title": "Education", "description": "Equalized school funding. No district should get less per student."},
             ]},
            # image_stat
            {"image_url": I[5], "stat": "14,247", "stat_label": "Signatures and counting", "caption": "Every signature is a person who believes change is possible"},
            # process_map
            {"title": "How to Join", "steps": [
                {"icon": "📋", "title": "Sign", "description": "Add your name to the coalition petition"},
                {"icon": "📣", "title": "Share", "description": "Send this carousel to 5 people in District 7"},
                {"icon": "🗳️", "title": "Vote", "description": "Register and show up on election day"},
                {"icon": "🤝", "title": "Organize", "description": "Become a neighborhood captain"},
            ]},
            # qr_destination
            {"destination_url": "https://district7coalition.org", "cta_text": "Join the Coalition", "heading": "14,000 Signatures. Yours Next?", "caption": "Scan to sign, share, and organize", "short_url": "district7coalition.org"},
        ]

    # ── 23. OUTRAGE CATALYST ───────────────────────────────────────────
    # Arc: Anger → Moral clarity → Evidence → Collective action → Accountability
    elif preset_id == "outrage_catalyst":
        return [
            # hero
            {"headline": "Your Utility Bill Funded a Lobby Campaign Against You", "subheadline": "We traced $2.3M in ratepayer money to political ads opposing the very regulations that would lower your bills.", "badge": "INVESTIGATION", "kicker": "FOLLOW THE MONEY"},
            # repeatable ×3: metric_grid + image_callout
            {"title": "Where Your Money Went", "metrics": [
                {"value": "$2.3M", "label": "spent on lobbying from utility ratepayer funds in 2025 alone", "trend": "+340% since 2022"},
            ]},
            {"image_url": I[6], "callouts": [{"label": "Money trail: Ratepayer funds → PAC → Attack ads", "description": "Your monthly bill includes a 'system improvement charge' — $4.20 of every $100 goes to political spending", "x": 50, "y": 40}], "description": "The fee on your bill labeled 'system improvement' is funding political ads."},
            {"title": "What They Opposed", "metrics": [
                {"value": "73%", "label": "of lobbying spend went to opposing clean energy regulations", "trend": "Public records"},
            ]},
            {"image_url": I[4], "callouts": [{"label": "The ads they ran with your money", "description": "'Clean energy mandates will raise your bill by $340/year' — the actual analysis showed a $12 increase", "x": 50, "y": 40}], "description": "The attack ads used your money to spread misinformation about policies that would help you."},
            {"title": "Your Monthly Contribution", "metrics": [
                {"value": "$4.20", "label": "per $100 bill goes to political spending, not infrastructure", "trend": "Every month"},
            ]},
            {"image_url": I[3], "callouts": [{"label": "What $2.3M could have built", "description": "730 homes weatherized. 12 community solar installations. 4,600 LED streetlights.", "x": 50, "y": 40}], "description": "That $2.3M could have directly lowered bills — instead it was spent fighting the regulations that would."},
            # chart
            {"title": "Where Your Utility Dollar Goes", "chart_type": "bar", "data": [{"label": "Infrastructure", "value": 62}, {"label": "Profit margin", "value": 18}, {"label": "Admin/exec", "value": 12}, {"label": "Political spending", "value": 8}]},
            # checklist_action_plan
            {"title": "What You Can Do", "items": [
                {"step": "File a public records request", "description": "Demand your utility disclose all political spending from ratepayer funds"},
                {"step": "Attend the next PUC hearing", "description": "Public comment period opens Sept 3. Your voice matters."},
                {"step": "Share this carousel", "description": "The more people know, the harder it is to hide"},
                {"step": "Contact your state representative", "description": "Support HB 2847 — the Ratepayer Transparency Act"},
             ]},
            # stat_row
            {"stats": [
                {"value": "$2.3M", "label": "Ratepayer funds spent on lobbying"},
                {"value": "73%", "label": "Went to opposing clean energy"},
                {"value": "4.2M", "label": "Ratepayers affected statewide"},
            ]},
            # process_map
            {"title": "The Accountability Chain", "steps": [
                {"icon": "🔍", "title": "Investigate", "description": "Public records reveal where ratepayer money actually goes"},
                {"icon": "📢", "title": "Expose", "description": "Share findings with affected communities"},
                {"icon": "⚖️", "title": "Legislate", "description": "Support transparency laws that prevent misuse"},
                {"icon": "🗳️", "title": "Enforce", "description": "Elect officials who prioritize ratepayers over utilities"},
            ]},
            # qr_destination
            {"destination_url": "https://followthemoney.org", "cta_text": "See the Full Report", "heading": "The Complete Investigation", "caption": "Scan for source documents, data, and action tools", "short_url": "followthemoney.org"},
        ]

    # ── 24. PARADIGM SHIFT ──────────────────────────────────────────────
    # Arc: Recognition → Discomfort → Understanding → Reframing
    elif preset_id == "paradigm_shift":
        return [
            # myth_fact
            {"myth": "Content quality determines reach", "fact": "Content structure determines reach. The same information, restructured with emotional architecture, reaches 3x more people.", "explanation": "Algorithms don't evaluate quality — they evaluate engagement signals: completion rate, shares, saves. Structure drives those signals, not polish."},
            # repeatable ×4: chart + callout
            {"title": "Quality vs Structure: Reach Impact", "chart_type": "bar", "data": [{"label": "High quality, no arc", "value": 31}, {"label": "Average quality, strong arc", "value": 78}, {"label": "High quality + strong arc", "value": 94}]},
            {"title": "The Uncomfortable Truth", "text": "Your best-designed carousel might be your worst-performing one. Beauty doesn't convert — structure converts. The most shared carousels on LinkedIn aren't the prettiest. They're the ones with the clearest emotional arc."},
            {"title": "Algorithm Signal Hierarchy", "chart_type": "bar", "data": [{"label": "Completion rate", "value": 95}, {"label": "Saves", "value": 82}, {"label": "Shares", "value": 76}, {"label": "Likes", "value": 34}]},
            {"title": "What Algorithms Actually Measure", "text": "Completion rate is the #1 signal. Not likes, not comments — did they scroll to the end? Emotional architecture keeps them scrolling. Pretty slides don't."},
            {"title": "The Paradigm Shift in Data", "chart_type": "bar", "data": [{"label": "Before: design-first", "value": 28}, {"label": "Transition: hybrid", "value": 51}, {"label": "After: structure-first", "value": 89}]},
            {"title": "The Industry Is Shifting", "text": "The most successful content teams in 2026 lead with narrative structure and add design polish as a second pass. The old model — design first, fill in content — is dying. Not because design doesn't matter, but because it was never the bottleneck."},
            {"title": "Engagement by Content Strategy", "chart_type": "bar", "data": [{"label": "Template-based", "value": 23}, {"label": "Design-led", "value": 38}, {"label": "Structure-led", "value": 71}, {"label": "Architecture-led", "value": 93}]},
            {"title": "The New Hierarchy", "text": "Emotional architecture > narrative structure > visual design > individual slide quality. Most teams have the hierarchy inverted. They optimize individual slides while ignoring the sequence. The sequence is what the algorithm sees."},
             # definition
            {"term": "Content Architecture", "phonetic": "/ˈkɒntɛnt ˈɑːrkɪtɛktʃər/", "definition": "The deliberate structuring of content sequences to maximize engagement signals that algorithms reward. The opposite of content decoration.", "context": "Architecture is invisible to the viewer but legible to the algorithm. That's why it works."},
            # scatter_plot
            {"title": "Structure vs Design: Performance", "x_label": "Emotional Architecture Score", "y_label": "Engagement Rate", "points": [
                {"x": 15, "y": 22, "label": "Template-only"},
                {"x": 38, "y": 41, "label": "Design-led"},
                {"x": 67, "y": 73, "label": "Structure-led"},
                {"x": 91, "y": 94, "label": "Architecture-led"},
            ]},
            # callout
            {"title": "The Shift Is Already Happening", "text": "Teams that adopted structure-first content in Q1 2026 are seeing 3x the reach of teams still designing first. The gap will only widen. This carousel is your wake-up call."},
            # cta
            {"headline": "Lead the Paradigm Shift", "button_text": "Adopt Architecture-First", "button_url": "https://slideforge.dev/paradigm", "subtext": "Start with the framework that puts structure before decoration."},
        ]

    # ── 25. TRANSFORMATION ARC (psychological self-improve) ────────────
    # Arc: Dissatisfaction → Hope → Progress → Commitment
    elif preset_id == "transformation_arc":
        return [
            # image_headline
            {"headline": "You're Not Lazy. You're Uninspired.", "image_url": I[7], "subheadline": "The gap between where you are and where you want to be isn't discipline — it's design."},
            # image_headline
            {"headline": "What If Your Workflow Was the Problem?", "image_url": I[2], "subheadline": "You don't need more motivation. You need a system that makes action the path of least resistance."},
            # repeatable ×5: section_divider + image_caption
            {"kicker": "WEEK 1", "title": "Dismantle the Old System", "subtitle": "Before building new habits, understand why the old ones failed"},
            {"image_url": I[0], "caption": "Week 1: Audit your current workflow", "description": "For one week, track every time you open Figma, Canva, or any design tool. Note what you're making, how long it takes, and how you feel about it."},
            {"kicker": "WEEK 2", "title": "Install the New System", "subtitle": "Replace design time with composition time"},
            {"image_url": I[1], "caption": "Week 2: Generate your first 5 carousels with Nexus", "description": "Use a different preset each day. Compare the output to your manual work. Not the aesthetics — the emotional impact."},
            {"kicker": "WEEK 3", "title": "Measure What Matters", "subtitle": "Stop tracking hours. Start tracking outcomes."},
            {"image_url": I[3], "caption": "Week 3: Track engagement, not production time", "description": "Which carousel got more saves? More shares? More DMs? The answer will surprise you."},
            {"kicker": "WEEK 4", "title": "Compound the Advantage", "subtitle": "The system is installed. Now it compounds."},
            {"image_url": I[4], "caption": "Week 4: Scale to daily publishing", "description": "You're now generating in minutes what used to take hours. Use the reclaimed time for strategy, relationships, and creative thinking."},
            {"kicker": "WEEK 5", "title": "The New Normal", "subtitle": "This is just how you work now"},
            {"image_url": I[5], "caption": "Week 5: Reflect on the transformation", "description": "Compare your Week 1 audit to your current output. The difference isn't just speed — it's creative freedom."},
            # before_after_story
            {"title": "The Transformation", "before": {"label": "Before", "description": "Dreading content deadlines. Spending weekends on carousels. Watching competitors ship faster. Feeling stuck in a tool loop."}, "after": {"label": "After", "description": "Generating carousels between meetings. Publishing daily. Leading the conversation instead of following it. Creative energy freed for strategy."}, "metric": "5 hrs/week", "metric_label": "Reclaimed for Creative Work"},
            # cta
            {"headline": "Your Transformation Starts Today", "button_text": "Start Week 1", "button_url": "https://slideforge.dev/transform", "subtext": "Free 5-week program with templates, presets, and community support."},
        ]

    # ── 26. LEGACY PRESERVATION ─────────────────────────────────────────
    # Arc: Reverence → Understanding → Continuity → Wisdom → Inheritance
    elif preset_id == "legacy_preservation":
        return [
            # image_headline
            {"headline": "The 106 Ra Sessions Changed Everything", "image_url": I[0], "subheadline": "How a 1981 contact phenomenon became the most comprehensive spiritual teaching of the modern era."},
            # repeatable ×4: image_quote + text_block
            {"quote": "I am Ra. I speak with one voice. I have not spoken before in this working. We are those of the social memory complex known to you as Ra.", "author": "Ra, Session 1", "image_url": A[0]},
            {"title": "The Beginning", "body": "In January 1981, a group in Louisville, Kentucky began a series of channeling sessions that would span four years and 106 sessions. The entity communicating called itself Ra — a sixth-density social memory complex from the Venusian system. What emerged was the Law of One."},
            {"quote": "The Law of One may be simplified, in its distillation to the most essential concept, to this: All things are one. That is the Law of One.", "author": "Ra, Session 15", "image_url": A[1]},
            {"title": "The Core Teaching", "body": "All is One. Every separation is illusion. Every encounter is an encounter with self. Every choice is a polarization — toward service-to-others or service-to-self. This is the framework. Everything else is commentary."},
            {"quote": "The purpose of incarnation is the evolution of the mind/body/spirit complex. You are here to learn. You are here to choose. You are here to love.", "author": "Ra, Session 28", "image_url": A[2]},
            {"title": "The Purpose", "body": "Third density is the density of choice — where the veil of forgetting is thinnest and the stakes are highest. You chose to be here. You chose the forgetting. And within that forgetting, you're choosing your polarities."},
            {"quote": "Every action generates an equal and opposite reaction in the cosmic balance. This is not punishment. It is learning.", "author": "Ra, Session 14", "image_url": A[3]},
             {"title": "The Mechanics", "body": "Catalyst → Experience → Wisdom → Transformation. This is the cycle of spiritual evolution. Every challenge is an opportunity. Every wound is a doorway. Every relationship is a mirror."},
            # quote
            {"quote": "There is nothing that we have that is not an illusion. All things are part of the One Infinite Creator. This is the fundamental teaching.", "author": "Ra, Session 30"},
            # timeline
            {"title": "The Timeline", "steps": [
                {"title": "1981-1984", "description": "106 channeling sessions. Ra delivers the complete cosmological framework."},
                {"title": "2018", "description": "LL Research publishes all sessions freely online. The archive opens to the world."},
                {"title": "2024", "description": "AI agents begin organizing and cross-referencing the material. New synthesis tools emerge."},
                {"title": "2026", "description": "SlideForge presets enable visual transmission of Ra's teachings for the social media age."},
            ]},
            # callout
            {"title": "The Transmission Continues", "text": "These teachings were given freely. They should remain freely accessible. Every carousel, every summary, every visual transmission should preserve the integrity of the original sessions while making the material accessible to new seekers."},
            # cta
            {"headline": "Carry the Teaching Forward", "button_text": "Access the Archive", "button_url": "https://www.llresearch.org", "subtext": "All 106 sessions, freely available. The original source material."},
        ]

    # ── 27. NOSTALGIA ENGINE ───────────────────────────────────────────
    # Arc: Recognition → Warmth → Longing → Connection → Action
    elif preset_id == "nostalgia_engine":
        return [
            # image_headline
            {"headline": "Remember When the Internet Felt Small?", "image_url": I[1], "subheadline": "Before algorithms. Before feeds. Before content farms. When every website was a person's passion project."},
            # repeatable ×4: image_quote + text_block
            {"quote": "The best things on the internet were made by one person who cared. Not a team. Not a brand. Just someone building something they wanted to exist.", "author": "The early web ethos", "image_url": A[0]},
            {"title": "The Handmade Web", "body": "GeoCities pages with custom cursors. Blogspot blogs with hand-coded CSS. Flash portfolios that took three minutes to load and you didn't care. The internet used to feel like a neighborhood. Now it feels like a mall."},
            {"quote": "We didn't know we were in the golden age. We just thought it was Tuesday.", "author": "Every person who used the internet before 2010", "image_url": A[1]},
            {"title": "What We Lost", "body": "The serendipity. Finding a random blog at 2am that changed how you thought about everything. The intimacy. Knowing the person behind the website. The patience. Reading 5,000 words because the writing was that good."},
            {"quote": "The internet didn't get worse. It got bigger. And big things lose the feeling of being made by a person.", "author": "Someone who remembers RSS readers", "image_url": A[2]},
            {"title": "The Scale Problem", "body": "Scale and intimacy are inversely related. The more people a platform serves, the less it feels like it was made for you. That's not nostalgia — it's physics."},
            {"quote": "Nostalgia is not about wanting to go back. It's about wanting to carry forward the feeling of being genuinely engaged with something.", "author": "A designer who remembers when design was fun", "image_url": A[3]},
             {"title": "The Real Longing", "body": "We don't miss the technology. We miss the feeling. The feeling of discovering something made by a person who cared. The feeling of being part of something small and real. That feeling is available — you just have to build it."},
            # quote
            {"quote": "The things you own end up owning you. It's only after you lose everything that you're free to do anything.", "author": "Chuck Palahniuk, Fight Club"},
            # definition
            {"term": "Digital Nostalgia", "phonetic": "/ˈdɪdʒɪtəl nɒˈstældʒə/", "definition": "The longing for a relationship with technology that felt personal, intentional, and human-scale. Not a desire to return to the past, but a desire to carry forward the values of the early internet.", "context": "Nostalgia is a compass. It points toward what mattered. Use it."},
            # callout
            {"title": "Build Something That Feels Like That", "text": "The early internet wasn't better technology. It was better intention. Every page was made by someone who cared. You can bring that energy to everything you create — even in the age of AI."},
            # cta
            {"headline": "Create Something Real", "button_text": "Start Building", "button_url": "https://slideforge.dev", "subtext": "Tools for people who care about what they make."},
        ]

    # ── 28. UNDERDOG COMEBACK ──────────────────────────────────────────
    # Arc: Sympathy → Admiration → Evidence → Triumph → Celebration
    elif preset_id == "underdog_comeback":
        return [
            # image_headline
            {"headline": "They Said It Couldn't Be Done in Rust", "image_url": I[2], "subheadline": "A slide engine in Rust? With HTML rendering? The experts gave it six months."},
            # repeatable ×3: metric_grid + text_block
            {"title": "Day 1 — Zero", "metrics": [
                {"value": "0", "label": "GitHub stars at launch — 'just another side project'", "trend": "Day 1, June 2023"},
            ]},
            {"title": "The Doubters", "body": "'Rust is the wrong choice for this.' 'HTML rendering belongs in the browser, not a binary.' 'You'll never match Figma.' The comments were polite. The sentiment was clear: this was a fool's errand."},
            {"title": "The Grind — 547 Days", "metrics": [
                {"value": "85", "label": "tests passing after 547 days of continuous development", "trend": "0 failures"},
            ]},
            {"title": "The Grind", "body": "Month after month. Feature after feature. 47 slide types. 28 campaign presets. 85 unit tests. The borrow checker fought every step. The HTML rendering pipeline broke weekly. But the binary got faster. And the output got better."},
            {"title": "The Result — 0.8s", "metrics": [
                {"value": "0.8s", "label": "render time for 47 slides — 15x faster than the Python reference", "trend": "Benchmark verified"},
            ]},
            {"title": "The Proof", "body": "The first real benchmark: 47 slides rendered in 0.8 seconds. The Python reference implementation took 12 seconds. The skeptics weren't wrong about difficulty — they were wrong about feasibility."},
            # definition
            {"term": "Dogfooding", "phonetic": "/ˈdɔːɡˌfuːdɪŋ/", "definition": "Using your own product to build the thing it's meant to build. SlideForge's campaign presets are built with SlideForge. The tool eats its own output.", "context": "If your tool can't build the thing it's meant to build, it's not ready. SlideForge generates its own marketing carousels. That's the ultimate quality signal."},
            # before_after_story
            {"title": "The Full Arc", "before": {"label": "Day 1", "description": "Zero stars. Zero users. Zero credibility. A conviction that Rust could build something beautiful."}, "after": {"label": "Day 547", "description": "85 tests. 47 slide types. 28 presets. A tool that generates production carousels in under a second."}, "metric": "547 days", "metric_label": "From Zero to Shipped"},
            # testimonial_avatar
            {"quote": "I dismissed this as a toy. Then I tried it. Then I couldn't go back to my old workflow. The speed difference isn't incremental — it's a different category.", "author": "Anonymous Skeptic", "role": "Now a Regular User", "avatar_url": A[4]},
            # cta
            {"headline": "The Underdog Ships Today", "button_text": "Try It Yourself", "button_url": "https://github.com/ishan-parihar/slideforge-rust", "subtext": "Open source. Free forever. Built by someone who was told it couldn't be done."},
        ]

    # ── Fallback for any unpreseted preset ──────────────────────────────
    else:
        return None


# ══════════════════════════════════════════════════════════════════════════
#  ORCHESTRATION
# ══════════════════════════════════════════════════════════════════════════

def process_preset(preset, preset_color, preset_idx, validate=False):
    preset_id = preset["id"]
    print(f"\n{'='*60}")
    print(f"Preset {preset_idx + 1}/28: {preset_id}")
    print(f"{'='*60}")

    tokens_file = os.path.join(TOKENS_DIR, f"{preset_id}_tokens.json")
    output_path = os.path.join(OUTPUT_DIR, f"{preset_id}.html")

    # Generate tokens
    theme = PRESET_THEMES[preset_idx % len(PRESET_THEMES)]
    print(f"  Theme: {theme}, Color: {preset_color}")
    if generate_tokens(preset_id, preset_color, theme, tokens_file) is None:
        print(f"  ✗ Failed to generate tokens for {preset_id}")
        return False

    # Get purpose-specific content
    content = preset_content(preset_id)
    if content is None:
        print(f"  ✗ No content defined for {preset_id}")
        return False

    # Generate slides
    slides_json = []
    slide_entries = preset.get("slides", [])
    aspect_ratio = ASPECT_RATIOS[preset_idx % len(ASPECT_RATIOS)]
    content_idx = 0  # flat index into content list

    for entry in slide_entries:
        if entry.get("type") == "repeatable":
            expanded = expand_repeatable_block(entry, preset_id)
            for exp_slide in expanded:
                stype = exp_slide.get("slide_type", "text_block")
                # Use purpose-specific content when available
                if content_idx < len(content):
                    params = content[content_idx]
                else:
                    params = fill_params(stype, exp_slide.get("params_template", {}), preset_id, content_idx)
                content_idx += 1

                theme_s = exp_slide.get("theme", theme)
                bg = exp_slide.get("bg_style", "dark")
                arch = exp_slide.get("archetype", PRESET_ARCHETYPES[preset_idx % len(PRESET_ARCHETYPES)])
                variant = exp_slide.get("variant", "")

                slide_obj = generate_slide(stype, tokens_file, theme_s, bg, arch, params, variant=variant, aspect_ratio=aspect_ratio)
                if slide_obj:
                    slides_json.append(slide_obj)
                    print(f"    ✓ {stype} (slide {content_idx})")
        else:
            stype = entry.get("slide_type", "text_block")
            if content_idx < len(content):
                params = content[content_idx]
            else:
                params = fill_params(stype, entry.get("params_template", {}), preset_id, content_idx)
            content_idx += 1

            theme_s = entry.get("theme", theme)
            bg = entry.get("bg_style", "dark")
            arch = entry.get("archetype", PRESET_ARCHETYPES[preset_idx % len(PRESET_ARCHETYPES)])
            variant = entry.get("variant", "")

            slide_obj = generate_slide(stype, tokens_file, theme_s, bg, arch, params, variant=variant, aspect_ratio=aspect_ratio)
            if slide_obj:
                slides_json.append(slide_obj)
                print(f"    ✓ {stype} ({bg}/{theme_s})")

    if not slides_json:
        print(f"  ✗ No slides generated for {preset_id}")
        return False

    # Composition validation
    if validate:
        slide_types = expand_slide_types(slide_entries)
        result = validate_preset_composition(preset, slide_types)
        if not result.get("valid", False):
            errors = result.get("errors", [])
            warnings = result.get("warnings", [])
            if errors:
                print(f"  ✗ VALIDATION FAILED: {', '.join(errors)}")
                return False
            if warnings:
                print(f"  ⚠ Warnings: {', '.join(warnings)}")
        else:
            print(f"  ✓ Composition validated")

    print(f"\n  Rendering carousel with {len(slides_json)} slides...")
    if render_carousel(slides_json, tokens_file, output_path, preset_id, aspect_ratio) is None:
        print(f"  ✗ Failed to render carousel for {preset_id}")
        return False

    print(f"  ✓ Saved: {output_path}")
    return True


# ── Fallback fill_params for any preset without dedicated content ───────
def fill_params(slide_type, params_template, preset_id, slide_idx):
    """Minimal fallback — only used if preset_content returns None."""
    p = {}
    for key, val in params_template.items():
        if isinstance(val, str) and val.startswith("{{"):
            p[key] = f"Demo value for {val.strip('{}')}"
        elif isinstance(val, (str, list, dict)):
            p[key] = val
    return p


def validate_preset_composition(preset, slide_types):
    """Run composition validation via the Rust binary's validate-composition CLI."""
    arc_structure = preset.get("arc_structure", {})
    
    # Reverse-map: slide_type → arc name
    type_to_arc = {}
    for arc_name, arc_def in arc_structure.items():
        for st in arc_def.get("types", []):
            type_to_arc[st] = arc_name
        for st in arc_def.get("pool", []):
            type_to_arc[st] = arc_name
    
    # Build composition with arc assignments
    composition = []
    for st in slide_types:
        arc = type_to_arc.get(st, "evidence")  # default to evidence if not found
        composition.append({"slide_type": st, "arc": arc})
    
    request = {
        "composition": composition,
        "arc_structure": arc_structure,
        "constraints": preset.get("constraints", {}),
    }
    tmp_path = os.path.join(TOKENS_DIR, f"_validate_{preset['id']}.json")
    with open(tmp_path, "w") as f:
        json.dump(request, f, indent=2)
    try:
        cmd = [BIN, "validate-composition", "--file", tmp_path]
        r = subprocess.run(cmd, capture_output=True, text=True, cwd=WORKSPACE_DIR, timeout=30)
        if r.returncode != 0:
            return {"valid": False, "errors": [r.stderr.strip() or r.stdout.strip()]}
        return json.loads(r.stdout)
    except (json.JSONDecodeError, subprocess.TimeoutExpired) as e:
        return {"valid": False, "errors": [str(e)]}
    finally:
        try:
            os.remove(tmp_path)
        except OSError:
            pass


def expand_slide_types(slides_entries):
    """Expand slide entries (including repeatable blocks) into a flat list of slide types."""
    result = []
    for entry in slides_entries:
        if entry.get("type") == "repeatable":
            unit_count = entry.get("repeat_count", {}).get("max", 2)
            unit_slides = entry.get("unit_slides", [])
            for i in range(unit_count):
                for unit in unit_slides:
                    result.append(unit.get("slide_type", "text_block"))
        else:
            result.append(entry.get("slide_type", "text_block"))
    return result


def main():
    import argparse
    parser = argparse.ArgumentParser(description="SlideForge Campaign Preset Generator v5.0.0")
    parser.add_argument("--validate", action="store_true", help="Run composition validation on each preset")
    parser.add_argument("--only", type=str, default=None, help="Generate only specified preset IDs (comma-separated)")
    parser.add_argument("--list", action="store_true", help="List all preset IDs and exit")
    parser.add_argument("--composition-mode", type=str, default="default",
                        choices=["default", "remix", "validate"],
                        help="Composition mode: default (use preset's default composition), "
                             "remix (allow AI agent to remix from full pool), "
                             "validate (validate composition without generating)")
    parser.add_argument("--test-diversity", action="store_true",
                        help="Test type diversity across generated carousels (checks for excessive repetition)")
    args = parser.parse_args()

    print("SlideForge Campaign Preset Generator v5.0.0 — Composition Mode")
    print(f"Binary: {BIN}")
    print(f"Output: {OUTPUT_DIR}")
    print(f"Presets: {PRESETS_FILE}\n")

    with open(PRESETS_FILE) as f:
        catalog = json.load(f)

    presets = [p for p in catalog["presets"] if "id" in p]

    if args.list:
        for p in presets:
            print(f"  {p['id']}")
        return 0

    if args.only:
        only_ids = set(args.only.split(","))
        presets = [p for p in presets if p["id"] in only_ids]
        if not presets:
            print(f"Error: no presets found matching: {args.only}")
            return 1

    print(f"Found {len(presets)} presets to generate\n")
    print(f"Composition mode: {args.composition_mode}\n")

    success = 0
    failed = []
    validation_errors = []

    # In validate mode, skip generation entirely
    if args.composition_mode == "validate":
        print("=" * 60)
        print("VALIDATE MODE: Skipping generation, validating compositions only")
        print("=" * 60)
        for preset in presets:
            pid = preset["id"]
            slides = preset.get("slides", [])
            # Count slots (with repeatable expansion at max)
            slot_count = 0
            for s in slides:
                if s.get("type") == "repeatable":
                    rc = s.get("repeat_count", {"min": 1, "max": 1})
                    max_iter = rc.get("max", 1)
                    for _ in range(max_iter):
                        slot_count += len(s.get("unit_slides", []))
                else:
                    slot_count += 1
            # Check arc_structure — pools are hook/evidence/action
            # Each pool can have fixed types OR flexible pool entries
            # A pool that is explicitly defined but empty is an error
            # A pool that is missing entirely is OK (not all presets need all pools)
            arc = preset.get("arc_structure", {})
            pool_keys = ["hook", "evidence", "action"]
            empty_pools = []
            for k in pool_keys:
                if k not in arc:
                    continue  # Missing pool is OK
                pool_data = arc.get(k, {})
                types = pool_data.get("types", [])
                pool = pool_data.get("pool", [])
                if not types and not pool:
                    empty_pools.append(k)
            if empty_pools:
                print(f"  ✗ {pid}: empty pools: {empty_pools}")
                failed.append(pid)
            else:
                pool_counts = {}
                for k in pool_keys:
                    if k not in arc:
                        pool_counts[k] = "absent"
                        continue
                    pool_data = arc.get(k, {})
                    types = pool_data.get("types", [])
                    pool = pool_data.get("pool", [])
                    pool_counts[k] = f"{len(types)}t+{len(pool)}p"
                print(f"  ✓ {pid}: {slot_count} slots, pools={pool_counts}")
                success += 1
        print(f"\n{'='*60}")
        print(f"VALIDATION: {success}/{len(presets)} presets valid")
        if failed:
            print(f"Failed: {', '.join(failed)}")
        print(f"{'='*60}")
        return 0 if not failed else 1

    for idx, preset in enumerate(presets):
        color = PRESET_COLORS[idx % len(PRESET_COLORS)]
        try:
            if process_preset(preset, color, idx, validate=args.validate):
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
    if validation_errors:
        print(f"\nValidation errors:")
        for ve in validation_errors:
            print(f"  {ve}")
    print(f"Output: {OUTPUT_DIR}/")
    print(f"{'='*60}")

    # Run diversity test if requested
    if args.test_diversity:
        print(f"\n{'='*60}")
        print("DIVERSITY TEST: Checking type distribution across generated carousels")
        print(f"{'='*60}")
        diversity_failures = []
        for preset in presets:
            pid = preset["id"]
            slides = preset.get("slides", [])
            # Count type occurrences
            type_counts = {}
            total = 0
            for s in slides:
                if s.get("type") == "repeatable":
                    rc = s.get("repeat_count", {"min": 1, "max": 1})
                    max_iter = rc.get("max", 1)
                    for _ in range(max_iter):
                        for u in s.get("unit_slides", []):
                            t = u.get("slide_type", "?")
                            type_counts[t] = type_counts.get(t, 0) + 1
                            total += 1
                else:
                    t = s.get("slide_type", "?")
                    type_counts[t] = type_counts.get(t, 0) + 1
                    total += 1
            # Check no type exceeds 50% of slides
            max_pct = 0
            max_type = None
            for t, c in type_counts.items():
                pct = (c / total) * 100 if total > 0 else 0
                if pct > max_pct:
                    max_pct = pct
                    max_type = t
            unique_types = len(type_counts)
            status = "✓" if max_pct <= 50 else "✗"
            print(f"  {status} {pid}: {unique_types} unique types, max={max_type} ({max_pct:.0f}%)")
            if max_pct > 50:
                diversity_failures.append((pid, max_type, max_pct))
        print(f"\n{'='*60}")
        if diversity_failures:
            print(f"DIVERSITY FAILURES: {len(diversity_failures)}")
            for pid, t, pct in diversity_failures:
                print(f"  {pid}: {t} at {pct:.0f}%")
        else:
            print("DIVERSITY: All presets pass (no type exceeds 50%)")
        print(f"{'='*60}")
        if diversity_failures:
            return 1

    return 0 if not failed else 1


if __name__ == "__main__":
    sys.exit(main())
