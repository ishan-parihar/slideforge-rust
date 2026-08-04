#!/usr/bin/env python3
"""Analyze chrome geometry dump from measure_chrome.py (banded chrome)."""
import json
import statistics
import sys

d = json.load(open('/tmp/chrome_geom.json'))
print('slides measured:', len(d))
scale = d[0]['slide']['w'] / 420
print('render scale:', scale)

coll_h = sum(1 for s in d if s['headerCollision'] > 0)
coll_f = sum(1 for s in d if s['footerCollision'] > 0)
print('header collisions (text into 36px header band): %d/%d' % (coll_h, len(d)))
print('footer collisions (text into 40px footer band): %d/%d' % (coll_f, len(d)))

top_deads = [s['topDead'] for s in d if s['topDead'] is not None]
bot_deads = [s['bottomDead'] for s in d if s['bottomDead'] is not None]
if top_deads:
    print('top dead margin (text top vs body top): median %.1f min %.1f max %.1f' % (statistics.median(top_deads), min(top_deads), max(top_deads)))
if bot_deads:
    print('bottom dead margin (body bottom vs text bottom): median %.1f min %.1f max %.1f' % (statistics.median(bot_deads), min(bot_deads), max(bot_deads)))

body_hs = [s['bodyRegion']['h'] for s in d]
if body_hs:
    print('body region height: median %.0f min %.0f max %.0f' % (statistics.median(body_hs), min(body_hs), max(body_hs)))

print()
print('=== slides with body-region collisions ===')
for s in d:
    if s['headerCollision'] > 0 or s['footerCollision'] > 0:
        br = s['bodyRegion']
        print(" slide %d: header_coll=%d footer_coll=%d topDead=%s bottomDead=%s body=%d..%d" % (
            s['i'], s['headerCollision'], s['footerCollision'], s['topDead'], s['bottomDead'], br['top'], br['bottom']))
        if s['textSample']:
            for t in s['textSample']:
                if t['top'] < br['top'] or t['bottom'] > br['bottom']:
                    print("    %-12s top=%5d bottom=%5d h=%4d %s" % (t['tag'], t['top'], t['bottom'], t['h'], t['txt']))

print()
print('=== first 3 slides full text sample ===')
for s in d[:3]:
    print("slide %d:" % s['i'])
    for t in (s['textSample'] or []):
        print("    %-12s top=%5d bottom=%5d h=%4d %s" % (t['tag'], t['top'], t['bottom'], t['h'], t['txt']))
