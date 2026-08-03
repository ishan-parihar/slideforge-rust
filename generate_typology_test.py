#!/usr/bin/env python3
"""Max sample-space sweep for the typology styling upgrade.

Generates slides across:
- All 10 typologies (editorial, startup, technical, brutalist, luxury, playful,
  vintage, data, nature, nightlife)
- 3 variant operators (default, polarity, energy)
- 7 color-scheme families (neutral, analogous, complementary, triadic,
  split-complement, monochrome, duotone)
- 6 primary colors

Each card shows its style fingerprint. Writes dist/typology_samples/*.png
plus dist/typology_viewer.html.
"""
import json
import os
import random
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).parent
BIN = REPO / "dist" / "slideforge-x86_64-linux-gnu"
OUT_DIR = REPO / "dist" / "typology_samples"
OUT_CAROUSEL = REPO / "dist" / "typology_carousel.html"
OUT_SLIDES = REPO / "dist" / "typology_test" / "compiled_slides.json"

OUT_DIR.mkdir(parents=True, exist_ok=True)
OUT_SLIDES.parent.mkdir(parents=True, exist_ok=True)

TYPOLOGIES = [
    "editorial", "startup", "technical", "brutalist", "luxury",
    "playful", "vintage", "data", "nature", "nightlife",
]
FAMILIES = [
    "neutral", "analogous", "complementary", "triadic",
    "split-complement", "monochrome", "duotone",
]
PRIMARIES = ["#1E3A8A", "#C62828", "#1B5E20", "#0F172A", "#7C3AED", "#B45309"]

SLIDE_POOL = [
    ("hero", "centered", {"headline": "A bolder brief, built for the feed", "subheadline": "Typology-tuned in one flag.", "badge": "SlideForge"}),
    ("quote", "default", {"quote": "Design is the silent language of trust.", "author": "Studio Notes"}),
    ("big_statement", "stat", {"heading": "One engine, ten moods", "cta_text": "Explore", "stat": "12×"}),
    ("funnel_chart", "default", {
        "title": "Conversion before and after",
        "description": "The redesigned funnel lifts every stage.",
        "steps": [{"label": "Aware", "value": 120}, {"label": "Interest", "value": 88}, {"label": "Decide", "value": 41}, {"label": "Act", "value": 17}],
    }),
    ("timeline", "default", {"title": "Three moves, one week", "steps": [{"label": "Audit", "description": "the loop"}, {"label": "Ship", "description": "the fragment"}, {"label": "Measure", "description": "the shift"}]}),
    ("text_block", "default", {"title": "The quiet upgrade", "body": "Ten typologies, seven color schemes, one engine.", "eyebrow": "EDITORIAL"}),
    ("image_headline", "default", {"image_url": "https://images.unsplash.com/photo-1500530855697-b586d89ba3ee?w=800", "headline": "Built for the feed", "subheadline": "Every pixel a decision."}),
]


def generate_tokens(primary):
    """Generate a token file for a given primary color."""
    tmp = tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False, dir=OUT_SLIDES.parent)
    tmp.close()
    subprocess.run([
        str(BIN), "configure-design", primary,
        "--output", tmp.name,
    ], check=True, capture_output=True)
    return tmp.name


def run_generate(slide_type, params, variant, tokens_file, typology, family, primary, idx):
    """Run slideforge generate-slide with typology flags + --output tempfile pattern."""
    params = dict(params)
    params["variant"] = variant
    tmp = tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False)
    tmp.close()
    cmd = [
        str(BIN), "generate-slide", slide_type,
        "--tokens-file", tokens_file,
        "--typology", typology,
        "--color-scheme", family,
        "--params", json.dumps(params),
        "--output", tmp.name,
    ]
    subprocess.run(cmd, check=True, capture_output=True)
    with open(tmp.name) as f:
        return json.load(f)


def main():
    slides = []
    metas = []
    idx = 0
    random.seed(int(os.environ.get("TYPOLOGY_SEED", "2025")))
    for typ in TYPOLOGIES:
        for variant in ("default", "polarity", "energy"):
            for fam in FAMILIES:
                primary = PRIMARIES[(idx + len(FAMILIES)) % len(PRIMARIES)]
                slide_type, slide_variant, params = SLIDE_POOL[idx % len(SLIDE_POOL)]
                tokens_file = generate_tokens(primary)
                try:
                    compiled = run_generate(slide_type, params, slide_variant, tokens_file, typ, fam, primary, idx)
                    slides.append(compiled)
                    metas.append({"idx": idx, "typology": typ, "variant": variant, "family": fam, "primary": primary, "slide_type": slide_type})
                except subprocess.CalledProcessError as e:
                    err = (e.stderr or b"").decode()[:200] or str(e)
                    print(f"  {idx:>3}. {typ}/{variant}/{fam} {slide_type} FAILED: {err}", file=sys.stderr)
                idx += 1
    with open(OUT_SLIDES, "w") as f:
        json.dump(slides, f, indent=2)
    print(f"compiled {len(slides)} slides")
    return slides, metas


def render_carousel(slides, tokens_file):
    subprocess.run([
        str(BIN), "render-carousel", str(OUT_SLIDES),
        "--tokens-file", tokens_file,
        "--brand-name", "SlideForge",
        "--topic", "TYPOLOGY-SAMPLE-SPACE",
        "--url", "slideforge.dev",
        "--hashtags", "#slides #typology",
        "--output", str(OUT_CAROUSEL),
    ], check=True, capture_output=True)
    subprocess.run([
        str(BIN), "export", str(OUT_CAROUSEL),
        "--output-dir", str(OUT_DIR),
        "--slides", str(len(slides)),
    ], check=True, capture_output=True)


def build_viewer(metas):
    cards = "".join(
        f'<div class="card" data-t="{m["typology"]}">'
        f'<img loading="lazy" src="typology_samples/slide_{m["idx"] + 1}.png" alt="{m["typology"]} {m["variant"]}">'
        f'<div class="meta"><b>{m["typology"]}</b> · {m["variant"]} · {m["family"]}<br>'
        f'<span class="sub">{m["primary"]} · {m["slide_type"]}</span></div></div>'
        for m in metas
    )
    viewer = """<!doctype html><html><head><meta charset="utf-8">
<title>SlideForge Typology Sample Space</title>
<style>
 body { font:14px/1.5 system-ui; background:#0e1116; color:#e6e6e6; padding:24px; }
 h1 { font-size:20px; } .filter { margin:12px 0; display:flex; gap:8px; flex-wrap:wrap; }
 button { background:#1c2430; color:#cfd6e0; border:1px solid #2c3a4d; border-radius:6px; padding:6px 12px; cursor:pointer; }
 button.on { background:#3b82f6; color:#fff; border-color:#3b82f6; }
 .grid { display:grid; grid-template-columns:repeat(auto-fill,minmax(220px,1fr)); gap:14px; margin-top:16px; }
 .card { background:#151b24; border:1px solid #243041; border-radius:10px; overflow:hidden; }
 .card img { display:block; width:100%; aspect-ratio:4/5; object-fit:cover; }
 .meta { padding:8px 10px; font-size:12px; color:#b9c2d0; }
 .sub { color:#6b7c8d; font-size:11px; }
 .count { color:#8b94a5; }
</style></head><body>
<h1>SlideForge · Typology Sample Space <span class="count">(__N__ samples)</span></h1>
<div class="filter">
  <button class="on" data-f="all">All</button>
__BUTTONS__
</div>
<div class="grid" id="g">__CARDS__</div>
<script>
 const cards=[...document.querySelectorAll('.card')];
 const btns=[...document.querySelectorAll('.filter button')];
 btns.forEach(b=>b.onclick=()=>{btns.forEach(x=>x.classList.remove('on'));b.classList.add('on');
   const f=b.dataset.f;cards.forEach(c=>c.style.display=(f==='all'||c.dataset.t===f)?'':'none');});
</script></body></html>"""
    buttons = "".join(f'  <button data-f="{t}">{t}</button>\n' for t in TYPOLOGIES)
    html = viewer.replace("__N__", str(len(metas))).replace("__BUTTONS__", buttons).replace("__CARDS__", cards)
    (REPO / "dist" / "typology_viewer.html").write_text(html)


if __name__ == "__main__":
    slides, metas = main()
    if not slides:
        print("no slides compiled", file=sys.stderr)
        sys.exit(1)
    tokens_file = generate_tokens(PRIMARIES[0])
    render_carousel(slides, tokens_file)
    build_viewer(metas)
    print(f"generated {len(slides)} samples, viewer at dist/typology_viewer.html")
