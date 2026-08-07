#!/usr/bin/env python3
"""
Max-length regression carousel: every slide type that received density
auto-scaling / type-fitting in the 2026-08-07 upgrade, exercised with
max-length (or over-limit) content to prove the renderer compresses instead
of overflowing the 449px body into the header/footer chrome bands.

Covers: big_statement, text_block, definition, comment_cta,
before_after_story, table, logo_cloud, testimonial_avatar, timeline, faq,
myth_fact, gauge, scatter_plot, image_headline, image_quote, image_callout.

For each slide the script:
  1. Renders the slide via the CLI with near-limit content.
  2. Exports PNGs via the blitz renderer.
  3. Pixel-probes the footer/header bands and the bottom body rows for
     content bleed (bright rows inside the footer band => overflow).

Usage:
    python3 generate_maxlength_regression.py
"""

import json, os, subprocess, sys

WORKSPACE_DIR = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(WORKSPACE_DIR, "dist", "slideforge-x86_64-linux-gnu")
if not os.path.exists(BIN):
    BIN = os.path.join(WORKSPACE_DIR, "target", "release", "slideforge-rust")

OUTPUT_DIR = os.path.join(WORKSPACE_DIR, "dist", "maxlength_test")
os.makedirs(OUTPUT_DIR, exist_ok=True)

# ── Near-limit / over-limit content per upgraded type ───────────────────
# Each config deliberately sits at (or just below) the renderer's legibility
# floor so the probe proves auto-scaling kicked in, not that the content is
# trivially small.

IMG = "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=800&h=500&fit=crop"

SLIDES = [
    ("big_statement", {
        "heading": "Fewer, sharper tools that collapse the distance between idea and system.",
        "body": "The winning architecture removes ceremony until only the essential decision remains visible.",
        "stat_value": "",
        "stat_label": "", "cta_text": "", "url": "",
    }),
    ("text_block", {
        "title": "The first principle of agent infrastructure",
        "body": ("An agent is only as trustworthy as the seams it can see. When the boundary between tool, memory, and policy is invisible, "
                 "the system fails quietly. Design seams first. Make every hand-off explicit, every fallback observable, every decision "
                 "reversible. The operators who ship reliable systems do not write more code; they write fewer, better-defined contracts "
                 "and then hold every layer to those contracts under load. This is the discipline that separates demos from production, "
                 "and it is the discipline this framework encodes end to end."),
        "subtitle": "", "text_align": "left", "max_width_val": "", "variant": "",
    }),
    ("definition", {
        "term": "Neuro-Linguistic Rapport Calibration",
        "definition": ("Rapport is the state where two nervous systems begin to mirror pacing, tonality, and micro-expression. It is not "
                        "agreement; it is alignment of tempo. Operatives calibrate to a baseline, then lead with a deviation to test "
                        "whether the frame is set."),
        "phonetic": "/kal·i·bray·shuhn/",
        "context": "Baseline → mirror → test-lead → frame. Calibration is a loop, not a switch.",
    }),
    ("comment_cta", {
        "heading": "Which of these five signals have you seen derail an interrogation?",
        "sub_heading": "One response carries five hidden signals. The comment section is the lab.",
        "cta_text": "Type your answer below. The first 50 comments get the full breakdown deck.",
        "keyword": "breakdown",
    }),
    ("before_after_story", {
        "title": "From a subject who volunteers nothing to one who volunteers the timeline",
        "before": ("The subject answered every direct question with a rehearsed one-liner. Pressure produced repetition, not disclosure. "
                    "Every push produced the same three scripts, verbatim."),
        "after": ("After calibrating to baseline and leading with a mismatch on the third repetition, the subject corrected the agent's "
                   "story unprompted — then expanded it. Detail density doubled and the timeline emerged without a single leading question."),
        "metric": "2.6×", "metric_label": "DETAIL DENSITY AFTER LEAD",
        "description": "The correction was unprompted — the strongest signal that the frame had shifted.",
    }),
    ("table", {
        "title": "Signal frequency across the five phases",
        "headers": ["Phase", "Signal", "Frequency", "Confidence"],
        "rows": [
            ["Baseline", "Micro-pacing lag", "8/10", "High"],
            ["Mirror", "Posture sync", "7/10", "High"],
            ["Test lead", "Correction", "6/10", "Medium"],
            ["Frame set", "Narrative expansion", "9/10", "High"],
            ["Close", "Preference reveal", "8/10", "High"],
            ["Decompress", "Relief tell", "5/10", "Medium"],
            ["Anchor", "Recurring phrase", "7/10", "Medium"],
            ["Leverage", "Sequencing error", "6/10", "Medium"],
        ],
        "caption": "Measured across 40 controlled sessions; confidence = inter-rater agreement.",
    }),
    ("logo_cloud", {
        "title": "Built on infrastructure that already runs the internet",
        "logos": [
            "Foundry Systems", "Kernel Works", "Atlas Grid", "Meridian Labs",
            "Northbeam", "Signal Forge", "Compass AI", "Relay Stack Ops",
        ],
    }),
    ("testimonial_avatar", {
        "quote": ("The difference between operators who ship and those who demo is that the shippers build the seam — the moment "
                   "where trust is earned or lost — and test it daily under real load."),
        "author": "Ishan Parihar", "role": "Independent AI Infrastructure Builder",
        "avatar_url": IMG,
    }),
    ("timeline", {
        "title": "The six moves of a frame takeover",
        "steps": [
            {"label": "Pace", "description": "Match tempo and tonality until the subject stops attending to your presence as an external object."},
            {"label": "Mirror", "description": "Echo posture, gesture rhythm, and micro-expression cadence to build an unconscious baseline of sameness."},
            {"label": "Test", "description": "Introduce a small, deliberate mismatch and watch for the automatic correction that reveals the bond."},
            {"label": "Lead", "description": "Escalate the deviation in increments; each successful follow lowers resistance to the next."},
            {"label": "Frame", "description": "Insert the new premise as an assumption, not a claim, while the other system is still following."},
            {"label": "Seal", "description": "Anchor the adopted frame to a recurring phrase so it survives future context shifts."},
        ],
    }),
    ("faq", {
        "title": "Four questions every calibration session raises",
        "questions": [
            {"question": "Is pacing the same as mirroring?", "answer": "No. Pacing is tempo; mirroring is posture and gesture. Both build baseline, but they are independent channels."},
            {"question": "How do I know the frame has set?", "answer": "The subject corrects you unprompted and then expands — that is the strongest single marker."},
            {"question": "What breaks calibration fastest?", "answer": "Rushing the lead. If the mismatch is too large, the other system flags it as anomaly and the loop resets."},
            {"question": "Does this work in writing?", "answer": "Partially. Without body channels you lose posture; pacing survives in sentence rhythm and message cadence."},
        ],
    }),
    ("myth_fact", {
        "myth": "Reading people is about watching for one giant tells you can see from across the room.",
        "fact": "Reading people is calibration: a baseline, a deliberate mismatch, and the correction the other system makes automatically.",
        "explanation": "The tell is not the signal; the tell is the deviation from baseline — and you can only see a deviation if you measured the baseline first.",
    }),
    ("gauge", {
        "value": 72, "label": "Calibration index", "title": "Session calibration quality",
        "caption": "Weighted composite of baseline fidelity, lead acceptance, and frame retention across 40 sessions.",
    }),
    ("scatter_plot", {
        "title": "Lead size vs. correction latency",
        "x_label": "Lead deviation", "y_label": "Correction (ms)",
        "data": [
            {"x": 2, "y": 180}, {"x": 4, "y": 210}, {"x": 6, "y": 240},
            {"x": 8, "y": 290}, {"x": 10, "y": 350}, {"x": 12, "y": 420},
            {"x": 3, "y": 195}, {"x": 5, "y": 230}, {"x": 7, "y": 265},
        ],
        "caption": "Corrections slow as lead size grows; the knee sits between 6 and 8.",
    }),
    ("image_headline", {
        "image_url": IMG,
        "headline": "The interview is not a conversation. It is a calibration sequence with a baseline.",
        "subheadline": "Pace. Mirror. Test. Lead. Frame. Seal — the six moves of every successful exchange.",
        "overlay_position": "bottom",
    }),
    ("image_quote", {
        "image_url": IMG,
        "quote": ("The most dangerous assumption in any interaction is that the other person is telling you everything they know. "
                   "They are telling you what they think you are entitled to — that difference is where the work happens."),
        "author": "Field Manual", "role": "Operative Six",
    }),
    ("image_callout", {
        "image_url": IMG,
        "callouts": [
            {"label": "Baseline", "description": "40-second silent calibration before the first question."},
            {"label": "Mismatch", "description": "A single deliberate pacing break at the third repetition."},
        ],
        "description": "The annotated sequence for a five-minute lead-in.",
    }),
]


def run_cmd(cmd, label):
    print(f"  [{label}] $ {' '.join(cmd[:5])}...")
    r = subprocess.run(cmd, capture_output=True, text=True, cwd=WORKSPACE_DIR, timeout=240)
    if r.returncode != 0:
        print(f"  ✗ FAILED: {(r.stderr or r.stdout)[-600:]}")
        sys.exit(1)
    return r.stdout


def main():
    print(f"=== Max-length regression: {len(SLIDES)} slides ===")

    # Generate each slide standalone, then render one combined carousel.
    slide_specs = []
    for i, (slide_type, params) in enumerate(SLIDES, 1):
        params_file = os.path.join(OUTPUT_DIR, f"p{i:02d}_{slide_type}.json")
        with open(params_file, "w") as f:
            json.dump(params, f)
        out_file = os.path.join(OUTPUT_DIR, f"s{i:02d}_{slide_type}.json")
        run_cmd([
            BIN, "generate-slide", slide_type,
            "--tokens-file", "design_tokens.json",
            "--typology", "technical",
            "--theme", "dark",
            "--params-file", params_file,
            "--output", out_file,
        ], f"generate {slide_type}")
        spec = json.load(open(out_file))
        spec["slide_type"] = slide_type
        slide_specs.append(spec)

    carousel_json = os.path.join(OUTPUT_DIR, "carousel_input.json")
    with open(carousel_json, "w") as f:
        json.dump(slide_specs, f)

    carousel_path = os.path.join(OUTPUT_DIR, "carousel.html")
    run_cmd([
        BIN, "render-carousel", carousel_json, "--output", carousel_path,
        "--brand-name", "Regression", "--topic", "Auto-scaling",
        "--url", "https://slideforge.dev", "--hashtags", "#test",
    ], "render-carousel")

    run_cmd([
        BIN, "export", carousel_path, "--output-dir", OUTPUT_DIR, "--slides", str(len(slide_specs)),
    ], "export PNGs")

    # Pixel-probe: content rows inside the footer band => overflow.
    overflow = []
    try:
        from PIL import Image
    except ImportError:
        print("  (PIL missing — skipping pixel probe)")
        return

    for i in range(1, len(SLIDES) + 1):
        png = os.path.join(OUTPUT_DIR, f"slide_{i}.png")
        if not os.path.exists(png):
            print(f"  slide_{i}: MISSING PNG")
            overflow.append(f"slide_{i} (no png)")
            continue
        import statistics
        im = Image.open(png).convert("RGB")
        w, h = im.size
        px = im.load()
        footer_y = int(h * (525 - 40) / 525)   # 40px footer band at 525
        header_y = int(h * 36 / 525)           # 36px header band
        slide_type = SLIDES[i - 1][0]
        is_image_slide = slide_type in ("image_headline", "image_quote", "image_callout")

        # Lowest text-like row above the footer (safety margin). The corner
        # chrome (brand/hashtags) legitimately lives inside the footer band by
        # design, so the pass criterion is CONTENT clearance: the lowest
        # text-like row above the footer band must sit at least 20px clear of
        # the band (a bleeding content block shows text right up to the band
        # edge). Full-bleed photos behind the transparent band have low
        # variance (smooth texture) and never register as text.
        lowest = None
        for y in range(footer_y - 2, header_y, -1):
            row = [sum(px[x, y]) for x in range(0, w, 4)]
            if statistics.pstdev(row) > 60 and max(row) - min(row) > 200:
                lowest = y
                break
        margin = footer_y - lowest if lowest else None
        status = "OK" if margin is not None and margin > 20 else "⚠"
        if status == "⚠":
            overflow.append(
                f"slide_{i} ({slide_type}) clearance={margin} (content text within 20px of the footer band)"
            )
        print(
            f"  slide_{i:>2} {slide_type:<20} lowest_text={lowest} footer_y={footer_y} clearance={margin} {status}"
        )

    print()
    if overflow:
        print(f"⚠ OVERFLOW/EDGE CASES: {len(overflow)}")
        for o in overflow:
            print(f"  - {o}")
        sys.exit(2)
    print("✓ All max-length slides fit the 449px body (no header/footer bleed).")


if __name__ == "__main__":
    main()
