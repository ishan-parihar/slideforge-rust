#!/usr/bin/env python3
"""Measure slide chrome geometry: where the header/footer bands and corner text
sit vs the .slide-body (the ONLY place slide types render) and the composition
bounds. Dumps JSON to stdout.

Architecture (banded chrome):
  .slide (420x525 base)
    .slide-composition (420x525)
      .slide-header   (36px band: brand left, topic right)
      .slide-body     (flex:1 — slide types render ONLY here)
      .slide-footer   (40px band: url left, progress, hashtags right)
      .swipe-arrow
"""
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).parent
CAROUSEL = REPO / "dist" / "typology_carousel.html"

# Find a chrome binary
CHROME = None
for cand in [
    "google-chrome", "google-chrome-stable", "chromium", "chromium-browser",
    "/usr/bin/google-chrome", "/usr/bin/chromium",
    str(Path.home() / ".cache" / "ms-playwright" / "chromium-*/chrome-linux/chrome"),
]:
    import glob
    if "*" in cand:
        hits = glob.glob(cand)
        if hits:
            CHROME = hits[0]
            break
    else:
        r = subprocess.run(["which", cand], capture_output=True, text=True)
        if r.returncode == 0:
            CHROME = cand
            break

if not CHROME:
    print("no chrome found", file=sys.stderr)
    sys.exit(2)

html = CAROUSEL.read_text()
# Inject measurement script just before </body>
measure_js = r"""
<script>
window.__chromeMeasure = () => {
  const slides = document.querySelectorAll('.slide');
  const out = [];
  const HDR = 36, FTR = 40;  // banded chrome heights (base px)
  slides.forEach((s, i) => {
    if (i > 40) return; // sample first 41 slides
    const comp = s.querySelector('.slide-composition');
    const header = s.querySelector('.slide-header');
    const body = s.querySelector('.slide-body');
    const footer = s.querySelector('.slide-footer');
    const content = s.querySelector('.slide-content');
    const brand = s.querySelector('.overlay__brand');
    const topic = s.querySelector('.overlay__topic');
    const url = s.querySelector('.overlay__url');
    const hashtags = s.querySelector('.overlay__hashtags');
    const progress = s.querySelector('.breadcrumb-progress') || s.querySelector('.progress-line');
    const sr = s.getBoundingClientRect();
    const scale = sr.width / 420;
    const rect = (el) => {
      if (!el) return null;
      const r = el.getBoundingClientRect();
      return {top: Math.round((r.top - sr.top) / scale), bottom: Math.round((r.bottom - sr.top) / scale),
              left: Math.round((r.left - sr.left) / scale), right: Math.round((r.right - sr.left) / scale),
              w: Math.round(r.width / scale), h: Math.round(r.height / scale)};
    };
    const contentPad = content ? getComputedStyle(content) : null;
    // Body-region text elements (inside .slide-body only — chrome is excluded
    // by construction since chrome lives in the header/footer bands).
    const textEls = [];
    if (body) {
      body.querySelectorAll('p,h1,h2,h3,h4,h5,h6,blockquote,span,li,strong,em').forEach((el) => {
        const t = (el.textContent || '').trim();
        if (!t) return;
        if (el.closest('.swipe-arrow')) return;
        const r = el.getBoundingClientRect();
        if (r.width < 4 || r.height < 4) return;
        textEls.push({tag: el.tagName.toLowerCase(), top: Math.round((r.top - sr.top) / scale), bottom: Math.round((r.bottom - sr.top) / scale), h: Math.round(r.height / scale), txt: t.slice(0, 32)});
      });
    }
    const bodyRect = body ? rect(body) : null;
    const bodyTop = bodyRect ? bodyRect.top : HDR;
    const bodyBottom = bodyRect ? bodyRect.bottom : (sr.height / scale - FTR);
    const bodyText = textEls.length ? textEls : null;
    const firstTextTop = bodyText ? Math.min(...bodyText.map(e => e.top)) : null;
    const lastTextBottom = bodyText ? Math.max(...bodyText.map(e => e.bottom)) : null;
    // Chrome collisions: body text that escapes the body region (into bands).
    const headerCollision = bodyText ? bodyText.filter(e => e.top < bodyTop).length : 0;
    const footerCollision = bodyText ? bodyText.filter(e => e.bottom > bodyBottom).length : 0;
    const topDead = firstTextTop !== null ? firstTextTop - bodyTop : null;
    const bottomDead = lastTextBottom !== null ? bodyBottom - lastTextBottom : null;
    out.push({
      i,
      slide: {w: Math.round(sr.width), h: Math.round(sr.height)},
      comp: rect(comp),
      header: rect(header), body: bodyRect, footer: rect(footer),
      content: rect(content),
      contentPadding: contentPad ? {t: contentPad.paddingTop, b: contentPad.paddingBottom, l: contentPad.paddingLeft, r: contentPad.paddingRight} : null,
      brand: rect(brand), topic: rect(topic), url: rect(url), hashtags: rect(hashtags),
      progress: rect(progress),
      bodyRegion: {top: bodyTop, bottom: bodyBottom, h: Math.round(bodyBottom - bodyTop)},
      bodyTextCount: bodyText ? bodyText.length : 0,
      headerCollision, footerCollision,
      topDead, bottomDead,
      textSample: bodyText ? bodyText.slice(0, 4) : null,
    });
  });
  return out;
};
window.__chromeData = null;
</script>
"""
html = html.replace("</body>", measure_js + "\n<script>window.__chromeData = window.__chromeMeasure();"
    + "\ndocument.body.appendChild(Object.assign(document.createElement('div'), {id: 'chrome-data', textContent: JSON.stringify(window.__chromeData)}));"
    + "</script>\n</body>")

# Render the data into a DOM node and dump it.
tmp = tempfile.NamedTemporaryFile(suffix=".html", delete=False)
tmp.write(html.encode())
tmp.close()

dump_js = """
const data = window.__chromeData;
if (!data) { console.log('NO_DATA'); process.exit(3); }
console.log('CHROME_DATA_START');
console.log(JSON.stringify(data));
console.log('CHROME_DATA_END');
process.exit(0);
"""
script = tmp.name.replace(".html", "_dump.js")
Path(script).write_text(dump_js)

cmd = [
    CHROME, "--headless=new", "--no-sandbox", "--disable-gpu",
    "--disable-dev-shm-usage", "--virtual-time-budget=3000",
    "--run-all-compositor-stages-before-draw", "--dump-dom",
    "--allow-file-access-from-files", "file://" + tmp.name,
]
r = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
dom = r.stdout
m = re.search(r'<div id="chrome-data">(.*?)</div>', dom, re.S)
if m:
    # The JSON is embedded as text content; unescape basic entities.
    raw = m.group(1).replace("&quot;", "\"").replace("&amp;", "&").replace("&#39;", "'")
    print(raw)
else:
    print("NO_MEASUREMENT", file=sys.stderr)
    print(dom[-2000:], file=sys.stderr)
    sys.exit(4)

Path(script).unlink(missing_ok=True)
Path(tmp.name).unlink(missing_ok=True)
