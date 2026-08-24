#!/usr/bin/env python3
"""Deckmill Ellipsis Manual Deck — 20-slide stress deck.

Builds a 20-slide, content-dense carousel from "The Ellipsis Manual"
(Chase Hughes) to stress-test overflow protection, hard caps, and the two
image-integration paths (remote Pexels CDN URL + local file via embed-image).

Pipeline (mirrors generate_stress_test.py):
    configure-design -> generate-slide x20 -> render-carousel ->
    validate-design -> export PNGs
"""

import json
import os
import subprocess
import sys
import glob

REPO = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(REPO, "dist", "deckmill-x86_64-linux-gnu")
OUT = os.path.join(REPO, "dist", "ellipsis_deck")
os.makedirs(OUT, exist_ok=True)

DECK = {
    "id": "ellipsis",
    "primary": "#1E3A5F",
    "style": "technical",
    "preset": "tonal_spot",
    "typology": "technical",
    "color_scheme": "monochrome",
    "theme": "dark",
    "archetype": "thought_leader",
    "brand": "The Ellipsis Manual",
    "topic": "Human Behavior Analysis",
    "url": "ellipsisbehavior.com",
    "hashtags": ["BehaviorAnalysis", "Hughes"],
    "progress": "chips",
    "aspect_ratio": "4:5",
}

# Pexels CDN URLs (verified 200). Remote image path — blitz-net fetches http(s).
PEXELS_DARK = "https://images.pexels.com/photos/1687675/pexels-photo-1687675.jpeg?auto=compress&cs=tinysrgb&w=800"
PEXELS_PORTRAIT = "https://images.pexels.com/photos/1024248/pexels-photo-1024248.jpeg?auto=compress&cs=tinysrgb&w=800"

# One more remote + one local (data URI via embed-image) to exercise both paths.
PEXELS_INTERROGATION = "https://images.pexels.com/photos/3183150/pexels-photo-3183150.jpeg?auto=compress&cs=tinysrgb&w=800"


def run(cmd):
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit("CMD FAILED: %s\n%s" % (" ".join(cmd), (r.stdout or r.stderr)[-2000:]))
    return r


def run_ok(cmd):
    """Run a command that may fail validation (returns rc)."""
    r = subprocess.run(cmd, capture_output=True, text=True)
    return r.returncode, r.stdout, r.stderr


def embed_local():
    """Exercise the local-filesystem image path: download a Pexels image to
    /tmp, then run `deckmill embed-image` -> data URI."""
    import urllib.request
    local = os.path.join("/tmp", "ellipsis_local_photo.jpg")
    if not os.path.exists(local):
        req = urllib.request.Request(
            PEXELS_INTERROGATION,
            headers={"User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36"},
        )
        with urllib.request.urlopen(req, timeout=30) as resp:
            with open(local, "wb") as f:
                f.write(resp.read())
    r = run([BIN, "embed-image", local])
    out = r.stdout.strip()
    # embed-image prints TOON: `data_uri: "data:image/..."` — extract the URI.
    import re
    m = re.search(r'"(data:image/[^"]+)"', out)
    if m:
        return m.group(1)
    if out.startswith("data:image"):
        return out
    raise RuntimeError("embed-image output not understood: " + out[:200])


def slides():
    return [
        # 1 — Opening hook
        {"type": "hero", "params": {
            "headline": "The Ellipsis Manual",
            "subheadline": "Analysis & Engineering of Human Behavior",
            "badge": "CHASE HUGHES",
            "variant": "chapter",
        }},
        # 2 — Big statement
        {"type": "big_statement", "params": {
            "heading": "Behavior is data. Every gesture is a sentence.",
            "body": "The Behavioral Table of Elements maps 100+ gestures to the deception and stress signals they carry — each cell holding 14 data points for field decoding.",
        }},
        # 3 — Definition: the BToE
        {"type": "definition", "params": {
            "term": "Behavioral Table of Elements",
            "definition": "A two-axis field chart. Vertical = body region (head to feet). Horizontal = stress & deception likelihood, rising left to right. Every cell carries 14 data points.",
            "context": "Head-to-toe gestures, plus object interaction and verbal-expression syntax.",
            "phonetic": "B-TOE",
        }},
        # 4 — Split features: the 14 cell data points (3 tiles max)
        {"type": "split_features", "params": {
            "title": "The Key: 14 Data Points Per Cell",
            "features": [
                {"icon": "🔢", "title": "Reference + Symbol", "description": "Stable location number and memorized abbreviation, e.g. 'acc.' for arm cross."},
                {"icon": "🔁", "title": "Confirming & Amplifying", "description": "Gestures that confirm a read (Fw → Cl, Jp) or amplify its meaning (Lc → Ct, Shg)."},
                {"icon": "🎯", "title": "Deception Rating", "description": "0–4.0 score with B/D/A timeframe. 4.0 = extremely deceptive; 1.0 = not likely."},
            ],
        }},
        # 5 — Metric grid: the Phillips case deception tally.
        # Trend badges add NEW signal (timing/qualifier) — they must NOT echo
        # the metric value (validator hard error). Description is mandatory.
        {"type": "metric_grid", "params": {
            "title": "One Response, Five Signals",
            "description": "Four signals scored extreme deception; the palm exposure read as deception-not-likely.",
            "metrics": [
                {"value": "4.0", "label": "Résumé statement", "trend": "DURING RESPONSE", "progress": 80},
                {"value": "4.0", "label": "Non-contraction", "trend": "DID NOT CONTRACT", "progress": 80},
                {"value": "4.0", "label": "Single shrug", "trend": "ONE-SIDED ONLY", "progress": 80},
                {"value": "1.0", "label": "Palm exposure", "trend": "DNL — NOT LIKELY", "progress": 20, "trend_direction": "positive"},
            ],
        }},
        # 6 — Process map: behavior-analysis process
        {"type": "process_map", "params": {
            "title": "The Behavior-Analysis Process",
            "description": "Observe → reference → confirm → tally. One question can fire five signals.",
            "steps": [
                {"title": "Observe", "description": "Watch before, during, after the response."},
                {"title": "Reference", "description": "Match gestures to BToE cells."},
                {"title": "Confirm", "description": "Cross-check confirming & conflicting cues."},
                {"title": "Tally", "description": "Sum deception ratings per response."},
            ],
        }},
        # 7 — Timeline: deception timeframe
        {"type": "timeline", "params": {
            "title": "Deception Timeframe",
            "steps": [
                {"title": "Before (B)", "description": "Elapsed pause from question to answer."},
                {"title": "During (D)", "description": "Behaviors inside the response itself."},
                {"title": "After (A)", "description": "Signals after the statement ends."},
            ],
        }},
        # 8 — Table: Human Needs Map
        {"type": "table", "params": {
            "title": "Human Needs Map",
            "headers": ["Need", "Type", "Signal"],
            "rows": [
                ["Appreciation", "Primary", "Benefits others"],
                ["Acceptance", "Primary", "Conforms to group"],
                ["Intelligence", "Secondary", "Flags education"],
                ["Power", "Secondary", "Needs to feel seen"],
            ],
        }},
        # 9 — Myth vs fact: baselining
        {"type": "myth_fact", "params": {
            "myth": "Baselining is unreliable — subjects fake it, so skip it.",
            "fact": "Baseline everyone. Faked gestures conflict across cells and raise red flags anyway.",
            "explanation": "Gather data before high-stress questions; deceptive baselining is itself a signal.",
        }},
        # 10 — Quote
        {"type": "quote", "params": {
            "quote": "An observed behavior is only as valuable as the stimulus that causes it.",
            "author": "Chase Hughes",
            "role": "The Ellipsis Manual",
        }},
        # 11 — Gauge: deception confidence
        {"type": "gauge", "params": {
            "title": "Deception Confidence",
            "value": 88,
            "label": "17.5 / 20 — near certain",
        }},
        # 12 — Radar: gestural markers
        {"type": "radar_chart", "params": {
            "title": "Gestural Markers",
            "description": "Gesture targets: operator (OP), operator's mouth (OMP), subject (SP), subject's face (SFP), external (EP), item (IP).",
            "data": [
                {"axis": "OP", "value": 5},
                {"axis": "OMP", "value": 4},
                {"axis": "SP", "value": 5},
                {"axis": "SFP", "value": 4},
                {"axis": "EP", "value": 3},
                {"axis": "IP", "value": 2},
            ],
        }},
        # 13 — Case study: Mr. Phillips interrogation
        {"type": "case_study_result", "params": {
            "client": "Field Interrogation",
            "title": "The Phillips Interview",
            "challenge": "One question: 'Did anything happen in the car?' The suspect's own denial was the tell.",
            "solution": "Documented each gesture against the BToE: résumé statement, non-contracted denial, single-sided shrug, head shake, palm exposure.",
            "description": "One response, five signals, score 17.5 — a single question can expose a deceptive cluster.",
            "results": [
                {"icon": "▮", "title": "4.0×3", "description": "Résumé, non-contraction, shrug"},
                {"icon": "✓", "title": "17.5", "description": "Total deception score (12 = extreme)"},
                {"icon": "✕", "title": "DNL", "description": "Palm exposure flagged as not-likely"},
            ],
        }},
        # 14 — Text block: the 6 influencing factors
        {"type": "text_block", "params": {
            "title": "Six Observation-Influencing Factors",
            "body": "Temperature (every 10° below 69° strips 1 point from closed gestures), interviewer behavior (each confrontational act −2 from 4.0-rated cells), subject emotional state (fear, aggression, defensiveness), proxemics, handicaps, and the presence of others. Non-measurable effects must be annotated in the report so human factors stay visible.",
        }},
        # 15 — FAQ
        {"type": "faq", "params": {
            "title": "Field FAQs",
            "questions": [
                {"question": "Does hypnosis exist?", "answer": "The book reframes it: the brain's autopilot handles obedience — influence works on that system."},
                {"question": "Why baseline at all?", "answer": "Deceptive baselining leaks intent; conflicting gestures raise red flags across cells."},
                {"question": "One gesture = deception?", "answer": "Never. A single shrug is innocuous alone; clusters of high-rated behaviors confirm."},
                {"question": "Judgment during interviews?", "answer": "Suspend it. Opinions corrupt the profile and can end with someone getting hurt."},
            ],
        }},
        # 16 — Before/after: baseline vs high-stress
        {"type": "before_after_story", "params": {
            "title": "Baseline vs. Interview",
            "before": "Calm small talk: even speech, open posture, steady blink rate.",
            "after": "High-stress questions: palm exposure, single shrug, vocal pitch rise, digital flexion.",
            "metric": "One baseline catches all five tells",
            "description": "Gather data before high-stress questions; a false baseline leaks intent anyway.",
        }},
        # 17 — Comparison bars: deception scale
        {"type": "comparison_bars", "params": {
            "title": "Deception Rating Scale",
            "description": "4.0-rated cells signal heavy deception; 1.0 carries a DNL (deception-not-likely) tag.",
            "comparison": {
                "left_label": "4.0 extreme",
                "left_value": 4,
                "right_label": "1.0 not-likely",
                "right_value": 1,
            },
        }},
        # 18 — Image headline: REMOTE Pexels URL path
        {"type": "image_headline", "params": {
            "image_url": PEXELS_DARK,
            "headline": "Read the Room Before You Speak",
            "subheadline": "Remote CDN image path — fetched live by the blitz renderer.",
            "overlay_position": "bottom",
        }},
        # 19 — Image quote: LOCAL file via embed-image (data URI path)
        {"type": "image_quote", "params": {
            "image_url": "__LOCAL_URI__",
            "quote": "The minus sign means the behavior is absent — absence is still data.",
            "author": "Behavioral Table of Elements",
        }},
        # 20 — CTA (single CTA rule: final closing slide)
        {"type": "qr_destination", "params": {
            "destination_url": "https://ellipsisbehavior.com",
            "cta_text": "Get the Manual",
            "heading": "Train the read.",
            "caption": "Scan for the interactive Behavioral Table of Elements.",
            "incentive_text": "Free field wallet cards with the book",
        }},
    ]


def generate_tokens():
    tf = os.path.join(OUT, "tokens.json")
    cmd = [BIN, "configure-design", DECK["primary"],
           "--style", DECK["style"],
           "--preset", DECK["preset"],
           "--typology", DECK["typology"],
           "--color-scheme", DECK["color_scheme"],
           "--output", tf]
    run(cmd)
    return tf


def generate_slide(tf, s, local_uri):
    p = dict(s["params"])
    if "__LOCAL_URI__" in json.dumps(p):
        p = json.loads(json.dumps(p).replace("__LOCAL_URI__", local_uri))
    tmp = os.path.join(OUT, "_tmp_slide.json")
    cmd = [BIN, "generate-slide", s["type"],
           "--tokens-file", tf,
           "--theme", DECK["theme"],
           "--bg-style", "dark",
           "--archetype", DECK["archetype"],
           "--typology", DECK["typology"],
           "--color-scheme", DECK["color_scheme"],
           "--params", json.dumps(p),
           "--output", tmp]
    rc, out, err = run_ok(cmd)
    if rc != 0:
        print("  [FAIL] %s -> %s" % (s["type"], (out or err)[-400:].replace("\n", " ")))
        return None
    with open(tmp) as f:
        return json.load(f)


def render_carousel(tf, slides):
    slides_file = os.path.join(OUT, "slides.json")
    with open(slides_file, "w") as f:
        json.dump(slides, f, indent=2)
    carousel = os.path.join(OUT, "carousel.html")
    cmd = [BIN, "render-carousel", slides_file,
           "--tokens-file", tf,
           "--brand-name", DECK["brand"],
           "--topic", DECK["topic"],
           "--url", DECK["url"],
           "--hashtags", ",".join(DECK["hashtags"]),
           "--platform", "instagram_portrait",
           "--aspect-ratio", DECK["aspect_ratio"],
           "--show-progress",
           "--progress-style", DECK["progress"],
           "--output", carousel]
    run(cmd)
    return carousel


def main():
    print("== Ellipsis Manual 20-slide deck ==")
    print("1/5 configure-design tokens")
    tf = generate_tokens()

    print("2/5 embed local image (local-filesystem path)")
    local_uri = embed_local()
    print("   local data URI: %s...%s" % (local_uri[:40], local_uri[-24:]))

    print("3/5 generate-slide x%d (dense content)" % len(slides()))
    compiled = []
    for i, s in enumerate(slides(), 1):
        spec = generate_slide(tf, s, local_uri)
        if spec is None:
            print("   [ABORT] slide %d failed — fixing required" % i)
            sys.exit(1)
        compiled.append(spec)
        print("   %2d/20 %-18s ok" % (i, s["type"]))

    print("4/5 render-carousel")
    carousel = render_carousel(tf, compiled)

    print("5/5 validate-design")
    rc, out, err = run_ok([BIN, "validate-design", carousel])
    try:
        j = json.loads(out)
        print("   slide_count=%d errors=%d warnings=%d" % (
            j.get("slide_count", 0), j.get("error_count", -1), j.get("warning_count", -1)))
        if j.get("errors"):
            for e in j["errors"][:12]:
                print("   E: %s" % e)
    except Exception:
        print("   validate output (non-JSON):", (out or err)[-600:])

    print("== export ==")
    exports = os.path.join(OUT, "exports")
    run([BIN, "export", carousel, "--output-dir", exports, "--slides", "20",
         "--preset", "instagram_portrait"])
    pngs = sorted(glob.glob(os.path.join(exports, "slide_*.png")))
    print("exported %d PNGs -> %s" % (len(pngs), exports))
    if len(pngs) != 20:
        print("  [WARN] expected 20 PNGs, got %d" % len(pngs))
    print("done: %s" % carousel)


if __name__ == "__main__":
    main()
