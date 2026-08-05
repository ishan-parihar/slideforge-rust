#!/usr/bin/env python3
"""SlideForge Stress Test — 10 deeply-populated carousel decks.

Purpose: exercise context-management, overflow-protection, automated scaling,
and word/char limits across ALL active slide types and their style variants.

Each deck is a real-world use case with a narrative arc
(hook -> context -> evidence -> structured content -> proof -> conversion).
Content is intentionally RICH (not shallow) so the stress surface is real.

Reads deck definitions from stress_decks/deck_*.json, runs the standard
pipeline (configure-design -> generate-slide xN -> render-carousel ->
validate-design), then builds ONE master viewer HTML that embeds every deck
via srcdoc iframes (self-contained single file).
"""

import json
import os
import subprocess
import sys
import glob
import html as _html

REPO = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(REPO, "dist", "slideforge-x86_64-linux-gnu")
DECK_DIR = os.path.join(REPO, "stress_decks")
OUT = os.path.join(REPO, "dist", "stress_test")
os.makedirs(OUT, exist_ok=True)


def run(cmd):
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit("CMD FAILED: %s\n%s" % (" ".join(cmd), r.stderr[-1500:]))
    return r


def generate_tokens(deck):
    tf = os.path.join(OUT, "tokens_%s.json" % deck["id"])
    cmd = [BIN, "configure-design", deck["primary"],
           "--style", deck["style"],
           "--preset", deck.get("preset", "tonal_spot"),
           "--typology", deck["typology"],
           "--color-scheme", deck.get("color_scheme", "neutral"),
           "--output", tf]
    run(cmd)
    return tf


def generate_slide(deck, tf, s):
    p = dict(s["params"])
    if s.get("variant"):
        p["variant"] = s["variant"]
    tmp = os.path.join(OUT, "_tmp_%s.json" % deck["id"])
    cmd = [BIN, "generate-slide", s["type"],
           "--tokens-file", tf,
           "--theme", s.get("theme", deck["theme"]),
           "--bg-style", s.get("bg", "dark"),
           "--archetype", deck["archetype"],
           "--typology", deck["typology"],
           "--color-scheme", deck.get("color_scheme", "neutral"),
           "--params", json.dumps(p),
           "--output", tmp]
    run(cmd)
    with open(tmp) as f:
        return json.load(f)


def render_deck(deck, slides):
    slides_file = os.path.join(OUT, "slides_%s.json" % deck["id"])
    with open(slides_file, "w") as f:
        json.dump(slides, f, indent=2)
    carousel = os.path.join(OUT, "deck_%s.html" % deck["id"])
    cmd = [BIN, "render-carousel", slides_file,
           "--tokens-file", os.path.join(OUT, "tokens_%s.json" % deck["id"]),
           "--brand-name", deck["brand"],
           "--topic", deck["topic"],
           "--url", deck["url"],
           "--hashtags", ",".join(deck["hashtags"]),
           "--platform", "instagram_portrait",
           "--aspect-ratio", deck["aspect_ratio"],
           "--show-progress",
           "--progress-style", deck["progress"],
           "--output", carousel]
    run(cmd)
    return carousel


def validate(carousel):
    r = subprocess.run([BIN, "validate-design", carousel],
                       capture_output=True, text=True)
    txt = r.stdout or r.stderr
    try:
        j = json.loads(txt)
        return j.get("error_count", -1), j.get("warning_count", -1), j.get("slide_count", 0), txt
    except Exception:
        return -1, -1, 0, txt[:800]


def load_decks():
    decks = []
    for path in sorted(glob.glob(os.path.join(DECK_DIR, "deck_*.json"))):
        with open(path) as f:
            decks.append(json.load(f))
    return decks


def build_master_viewer(deck_results):
    # deck_results: list of dicts {deck, slides_html, carousel_html, errors, warnings, slides_count, used}
    # Each carousel is rendered at its export canvas (e.g. 1080x1736 for 4:5).
    # The carousel shell self-fits to the iframe width (viewport-fit script), so
    # we size each iframe to the deck's canvas aspect ratio and add per-preview
    # zoom controls that resize only that iframe — zooming the slide, not the page.
    import re as _re

    cards = []
    for i, dr in enumerate(deck_results):
        d = dr["deck"]
        esc_carousel = _html.escape(dr["carousel_html"], quote=True)
        badge = ("gate-pass" if dr["errors"] == 0 else "gate-fail")
        badge_txt = ("0 errors" if dr["errors"] == 0 else "%d errors" % dr["errors"])
        types_used = ", ".join(dr["used"])
        m = _re.search(r'id="sf-canvas"[^>]*width:(\d+)px;height:(\d+)px', dr["carousel_html"])
        cw, ch = (int(m.group(1)), int(m.group(2))) if m else (1080, 1736)
        # Pre-seed the CSS aspect var so the preview iframe is sized to the
        # deck's ACTUAL canvas ratio before the zoom JS runs (the .carousel-frame
        # CSS otherwise defaults to --ar:1.6, flashing a wrong-height box).
        deck_ar = (ch / cw) if cw else 1.6
        cards.append(f"""
<div class="deck" id="deck-{i}" data-cw="{cw}" data-ch="{ch}" style="--ar: {deck_ar:.4f}">
  <div class="deck-head">
    <div class="deck-index">{i+1:02d}</div>
    <div class="deck-title">
      <h2>{_html.escape(d['name'])}</h2>
      <div class="deck-meta">{_html.escape(d['deck_caption'])}</div>
    </div>
    <div class="deck-stats">
      <span class="chip">{dr['slides_count']} slides</span>
      <span class="chip">{d['theme']} / {d['typology']}</span>
      <span class="chip">{d['aspect_ratio']} · {d['progress']}</span>
      <span class="chip {badge}">{badge_txt} · {dr['warnings']} warn</span>
    </div>
  </div>
  <div class="deck-body">
    <div class="pv-wrap">
      <iframe class="carousel-frame" srcdoc="{esc_carousel}" title="{_html.escape(d['name'])}"></iframe>
      <div class="pv-zoom">
        <button class="pv-btn" data-act="out" title="Zoom out">−</button>
        <span class="pv-pct">100%</span>
        <button class="pv-btn" data-act="in" title="Zoom in">+</button>
        <button class="pv-btn" data-act="fit" title="Reset">⤾</button>
      </div>
    </div>
    <div class="deck-slides"><span class="lbl">TYPES</span>{_html.escape(types_used)}</div>
  </div>
</div>""")
    total_slides = sum(dr["slides_count"] for dr in deck_results)
    total_errors = sum(dr["errors"] for dr in deck_results)
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>SlideForge Stress Test — 10 Carousel Decks</title>
<style>
:root {{ --bg:#0a0b10; --surface:#14151c; --surface-2:#1c1e27; --border:#2a2c3d;
  --text:#edeef5; --dim:#9098a8; --accent:#5e5fe0; --ok:#10b981; --bad:#ef4444; }}
* {{ box-sizing:border-box; margin:0; padding:0; }}
body {{ font-family:-apple-system,"Segoe UI",Inter,"Helvetica Neue",sans-serif;
  background:var(--bg); color:var(--text); line-height:1.5; padding:28px; max-width:1500px; margin:0 auto; }}
h1 {{ font-size:26px; font-weight:800; letter-spacing:-0.02em; margin-bottom:6px; }}
.subtitle {{ color:var(--dim); font-size:13.5px; margin-bottom:20px; }}
.summary {{ display:flex; gap:16px; flex-wrap:wrap; margin-bottom:26px; }}
.stat {{ background:var(--surface); border:1px solid var(--border); border-radius:10px; padding:14px 20px; min-width:150px; }}
.stat b {{ font-size:28px; display:block; letter-spacing:-0.02em; color:var(--accent); }}
.stat span {{ font-size:11.5px; color:var(--dim); text-transform:uppercase; letter-spacing:0.06em; }}
.deck {{ background:var(--surface); border:1px solid var(--border); border-radius:14px; padding:20px; margin-bottom:24px; }}
.deck-head {{ display:flex; align-items:flex-start; gap:16px; flex-wrap:wrap; margin-bottom:14px; }}
.deck-index {{ font-size:30px; font-weight:900; color:var(--accent); letter-spacing:-0.03em; line-height:1; }}
.deck-title {{ flex:1; min-width:220px; }}
.deck-title h2 {{ font-size:19px; font-weight:800; letter-spacing:-0.01em; }}
.deck-meta {{ color:var(--dim); font-size:12px; margin-top:4px; }}
.deck-stats {{ display:flex; gap:8px; flex-wrap:wrap; }}
.chip {{ font-size:11px; font-weight:700; padding:5px 10px; border-radius:999px;
  background:var(--surface-2); border:1px solid var(--border); color:var(--dim); }}
.chip.gate-pass {{ color:var(--ok); border-color:var(--ok); background:transparent; }}
.chip.gate-fail {{ color:var(--bad); border-color:var(--bad); background:transparent; }}
.deck-body {{ display:flex; gap:20px; align-items:flex-start; flex-wrap:wrap; }}
/* Preview iframe: sized to the deck's canvas aspect at a 380px base width.
   The carousel shell self-fits to the iframe width, so the full slide is
   always visible — no zoomed-in crop. Zoom buttons resize only this iframe. */
.pv-wrap {{ flex:0 0 auto; }}
.carousel-frame {{ display:block; width:380px; height:calc(380px * var(--ar, 1.6)); border:1px solid var(--border);
  border-radius:8px; background:#111; box-shadow:0 10px 30px rgba(0,0,0,.35); transition:width .2s ease, height .2s ease; }}
.pv-zoom {{ display:flex; align-items:center; gap:8px; margin-top:8px; }}
.pv-btn {{ width:28px; height:28px; border-radius:8px; border:1px solid var(--border);
  background:var(--surface-2); color:var(--text); font-size:15px; font-weight:700; cursor:pointer;
  line-height:1; transition:background .15s, border-color .15s; }}
.pv-btn:hover {{ background:var(--accent); border-color:var(--accent); color:#fff; }}
.pv-pct {{ min-width:44px; text-align:center; font-size:11.5px; font-weight:700; color:var(--dim);
  font-variant-numeric:tabular-nums; }}
.deck-body > .deck-slides {{ flex:1; min-width:200px; color:var(--dim); font-size:12px; line-height:1.6; padding-top:6px; }}
.deck-slides .lbl {{ display:block; font-size:10.5px; letter-spacing:0.1em; color:var(--accent); font-weight:800; margin-bottom:6px; }}
</style>
</head>
<body>
<h1>SlideForge Stress Test — 10 Carousel Decks</h1>
<div class="subtitle">Deep-population audit: overflow protection, automated scaling, per-slide typology variance. Use each deck's +/− controls to zoom that slide alone — the page itself never zooms.</div>
<div class="summary">
  <div class="stat"><b>{len(deck_results)}</b><span>decks</span></div>
  <div class="stat"><b>{total_slides}</b><span>slides</span></div>
  <div class="stat"><b>{total_errors}</b><span>validation errors</span></div>
</div>
{''.join(cards)}
<script>
// Per-preview zoom: resizes the iframe (the carousel self-fits), so zooming
// only affects that deck's preview — never the whole page.
(function () {{
  const BASE_W = 380;
  document.querySelectorAll('.deck').forEach(deck => {{
    const cw = parseInt(deck.dataset.cw || 1080, 10);
    const ch = parseInt(deck.dataset.ch || 1736, 10);
    const frame = deck.querySelector('.carousel-frame');
    const pct = deck.querySelector('.pv-pct');
    let zoom = 1;
    function apply() {{
      const w = Math.round(BASE_W * zoom);
      const h = Math.round(w * ch / cw);
      frame.style.width = w + 'px';
      frame.style.height = h + 'px';
      pct.textContent = Math.round(zoom * 100) + '%';
    }}
    deck.querySelectorAll('.pv-btn').forEach(btn => {{
      btn.addEventListener('click', () => {{
        const act = btn.dataset.act;
        if (act === 'in') zoom = Math.min(zoom * 1.25, 6);
        else if (act === 'out') zoom = Math.max(zoom / 1.25, 0.25);
        else zoom = 1;
        apply();
      }});
    }});
    apply();
  }});
}})();
</script>
</body>
</html>"""


def main():
    decks = load_decks()
    if not decks:
        sys.exit("no decks found in %s" % DECK_DIR)
    print("Loading %d decks from %s" % (len(decks), DECK_DIR))

    # Full active-type inventory for the coverage report (type -> variants).
    # Variant lists mirror src/slide_registry.rs exactly.
    INVENTORY = {
        "hero": ["centered", "split", "chapter"],
        "quote": ["centered", "card", "large-quote", "with-avatar", "minimal"],
        "timeline": ["horizontal", "vertical", "numbered", "arrow-chain"],
        "split_features": ["two-column", "three-column", "icon-grid", "minimal"],
        "definition": ["card", "inline", "highlighted", "minimal"],
        "text_block": ["left", "centered", "narrow", "wide"],
        "problem_solution": ["split", "proof-grid"],
        "myth_fact": ["debunk"],
        "case_study_result": ["summary", "results-grid"],
        "pricing_plan": ["cards", "offer-stack"],
        "testimonial_avatar": ["centered", "profile"],
        "logo_cloud": ["grid", "strip"],
        "faq": ["stack", "compact"],
        "process_map": ["numbered", "map"],
        "before_after_story": ["split", "metric"],
        "chart": ["bar", "line", "pie", "donut", "scatter", "area", "horizontal-bar"],
        "image_caption": ["image-top", "image-bottom", "image-left", "image-right", "image-overlay"],
        "image_headline": ["center", "bottom", "top"],
        "image_quote": ["default"],
        "image_callout": ["default"],
        "image_gallery": ["2-grid", "3-grid", "4-grid", "featured-1-2", "featured-2-1"],
        "image_collage": ["scattered", "layered", "geometric", "editorial_stack", "mosaic", "filmstrip"],
        "image_comparison": ["line", "arrow"],
        "scatter_plot": ["default"],
        "gauge": ["default"],
        "radar_chart": ["default"],
        "progress_rings": ["default"],
        "comparison_bars": ["default"],
        "metric_grid": ["default"],
        "funnel_chart": ["default"],
        "table": ["default"],
        "qr_destination": ["theme-bg", "image-bg", "minimal", "full-conversion", "split-card", "poster", "stacked-badge", "compact"],
        "big_statement": ["default", "stat"],
        "comment_cta": ["default"],
    }

    deck_results = []
    grand_used = {}
    for d in decks:
        print("\n=== DECK: %s (%s) ===" % (d["name"], d["id"]))
        tf = generate_tokens(d)
        slides = []
        used = []
        for i, s in enumerate(d["slides"], 1):
            obj = generate_slide(d, tf, s)
            slides.append(obj)
            key = s["type"] + ":" + (s.get("variant") or "default")
            used.append(key)
            grand_used.setdefault(s["type"], set()).add(s.get("variant") or "default")
            print("  %2d. %-20s (%-14s) [%s/%s]" % (i, s["type"], s.get("variant", "-"), s.get("theme", d["theme"]), s.get("bg")))
        carousel = render_deck(d, slides)
        errors, warnings, count, _ = validate(carousel)
        print("  -> %s | %d slides | %d errors | %d warnings" % (carousel, count, errors, warnings))
        with open(carousel) as f:
            carousel_html = f.read()
        deck_results.append({
            "deck": d, "used": sorted(set(used)), "carousel_html": carousel_html,
            "errors": errors, "warnings": warnings, "slides_count": count,
        })

    # Coverage report
    print("\n=== COVERAGE REPORT ===")
    missing = []
    for t, variants in INVENTORY.items():
        got = grand_used.get(t, set())
        missing_v = [v for v in variants if v not in got]
        status = "FULL" if not missing_v else ("partial" if got else "MISSING")
        print("  %-22s %-8s got=%-28s missing=%s" % (t, status, sorted(got) if got else "-", missing_v or "-"))
        if status != "FULL":
            missing.append((t, missing_v))
    total_types = len(INVENTORY)
    covered = sum(1 for t in INVENTORY if grand_used.get(t))
    print("  %d/%d types covered" % (covered, total_types))

    master = build_master_viewer(deck_results)
    master_path = os.path.join(REPO, "dist", "stress_test_master.html")
    with open(master_path, "w") as f:
        f.write(master)
    print("\nMaster viewer: %s" % master_path)


if __name__ == "__main__":
    main()
