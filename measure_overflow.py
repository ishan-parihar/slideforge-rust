#!/usr/bin/env python3
"""Measure per-slide overflow in dist/typology_carousel.html via headless Chromium."""
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).parent
CAROUSEL = REPO / "dist" / "typology_carousel.html"
CHROME = "/usr/bin/chromium"

carousel_html = CAROUSEL.read_text()

measure_js = r"""
<script>
(function () {
  function run() {
    var out = [];
    var slides = document.querySelectorAll('.slide');
    for (var i = 0; i < slides.length; i++) {
      var s = slides[i];
      var comp = s.querySelector('.slide-composition');
      var content = s.querySelector('.slide-content') || comp;
      if (!comp) { continue; }
      var compOv = comp.scrollHeight - comp.clientHeight;
      var contentOv = content ? content.scrollHeight - content.clientHeight : 0;
      var worst = [];
      var nodes = comp.querySelectorAll('*');
      for (var k = 0; k < nodes.length; k++) {
        var n = nodes[k];
        var vo = n.scrollHeight - n.clientHeight;
        var ho = n.scrollWidth - n.clientWidth;
        if (vo > 2 || ho > 2) {
          var fs = n.style && n.style.fontSize ? n.style.fontSize : '';
          var txt = (n.textContent || '').trim().slice(0, 28).replace(/\s+/g, ' ');
          worst.push({ tag: n.tagName.toLowerCase(), fs: fs, v: Math.round(vo), h: Math.round(ho), txt: txt });
        }
      }
      worst.sort(function (a, b) { return (b.v + b.h) - (a.v + a.h); });
      if (compOv > 2 || contentOv > 2) {
        out.push({
          i: i,
          compOverflow: Math.round(compOv),
          contentOverflow: Math.round(contentOv),
          bg: s.className.replace(/\s+/g, ' ').slice(0, 60),
          worst: worst.slice(0, 4)
        });
      }
    }
    var pre = document.createElement('pre');
    pre.id = 'RESULT';
    pre.textContent = JSON.stringify({ total: slides.length, overflowing: out.length, slides: out });
    document.body.appendChild(pre);
    document.title = 'DONE_' + out.length;
  }
  if (document.readyState === 'complete') { setTimeout(run, 3000); }
  else { window.addEventListener('load', function () { setTimeout(run, 3000); }); }
})();
</script>
"""

# Inject measurement script right before </body>
if "</body>" in carousel_html:
    combined = carousel_html.replace("</body>", measure_js + "</body>")
else:
    combined = carousel_html + measure_js

with tempfile.NamedTemporaryFile("w", suffix=".html", delete=False) as t:
    t.write(combined)
    path = t.name

try:
    proc = subprocess.run(
        [
            CHROME, "--headless=new", "--no-sandbox", "--disable-gpu",
            "--virtual-time-budget=15000", "--run-all-compositor-stages-before-draw",
            "--dump-dom", f"file://{path}",
        ],
        capture_output=True, text=True, timeout=120,
    )
    dom = proc.stdout
    m = re.search(r"<pre id=\"RESULT\">(.*?)</pre>", dom, re.S)
    if not m:
        print("NO RESULT FOUND. stdout len:", len(dom), "stderr:", proc.stderr[-400:], file=sys.stderr)
        sys.exit(1)
    data = json.loads(m.group(1))
    print(json.dumps(data, indent=1))
finally:
    Path(path).unlink(missing_ok=True)
