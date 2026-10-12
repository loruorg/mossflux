"""Check that the landing page's type stays legible over every background frame.

    .venv/bin/python scripts/site/check_contrast.py

Independent of build_frames.py's measurement: it decodes the SHIPPED WebP
frames, applies the veil and scrim exactly as site/bg.js does (same constants,
same rounding), and checks every frame and every crossfade midpoint. Then it
checks the fixed surfaces (plates, rail, caption chip) over a pure white frame,
the worst case there is. Exit 1 on any failure.
"""

import json
import os
import sys

import numpy as np
from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
BG = os.path.join(ROOT, "site", "bg")

# site/bg.js
VEIL, PLATEAU, S_MIN = 0.14, 0.90, 0.4
FLOOR = np.array([8, 10, 9]) / 255.0
BLOCK = 16


def lin(c):
    c = np.asarray(c, dtype=np.float64)
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


def lum(rgb):
    x = lin(rgb)
    return x[..., 0] * 0.2126 + x[..., 1] * 0.7152 + x[..., 2] * 0.0722


def hexrgb(h):
    h = h.lstrip("#")
    return np.array([int(h[i:i + 2], 16) for i in (0, 2, 4)]) / 255.0


def contrast(la, lb):
    hi, lo = max(la, lb), min(la, lb)
    return (hi + 0.05) / (lo + 0.05)


def over(c, colour, alpha):
    return c * (1 - alpha) + colour * alpha


def scrim_opacity(need):
    s = (1 - (1 - need) / (1 - VEIL)) / PLATEAU
    s = min(1.0, max(S_MIN, s))
    return np.ceil(s * 50) / 50


def brightest_block(rgb):
    y = lum(rgb)
    h, w = y.shape
    h, w = h - h % BLOCK, w - w % BLOCK
    return float(y[:h, :w].reshape(h // BLOCK, BLOCK, w // BLOCK, BLOCK).mean(axis=(1, 3)).max())


BONE = float(lum(hexrgb("DDE4D7")))


def main():
    txt = open(os.path.join(BG, "manifest.js")).read()
    m = json.loads(txt[txt.index("{"): txt.rindex("}") + 1])
    target = m["contrast"]
    failures = 0
    worst = (99.0, "")

    for theme in m["order"]:
        for o in ("l", "p"):
            meta = m["themes"][theme][o]
            frames = [np.asarray(Image.open(os.path.join(BG, theme, o, f"{k:03d}.webp")).convert("RGB"),
                                 dtype=np.float64) / 255.0 for k in range(meta["n"])]
            for k in range(meta["n"]):
                pairs = [(k, k, 0.0)]
                if k + 1 < meta["n"]:
                    pairs.append((k, k + 1, 0.5))
                for a, b, t in pairs:
                    need = max(meta["a"][a], meta["a"][b])
                    s = scrim_opacity(need)
                    img = frames[a] * (1 - t) + frames[b] * t
                    img = over(img, FLOOR, VEIL)                 # canvas veil
                    img = over(img, FLOOR, PLATEAU * s)          # .scrim plateau
                    c = contrast(BONE, brightest_block(img))
                    if c < worst[0]:
                        worst = (c, f"{theme}-{o} frame {a}" + (f"/{b}" if b != a else ""))
                    if c < target:
                        failures += 1
                        print(f"FAIL {theme}-{o} frame {a}/{b}: bone {c:.2f}:1 < {target}:1")
            print(f"{theme}-{o}: {meta['n']} frames + crossfades checked")

    print(f"free type: worst bone contrast {worst[0]:.2f}:1 at {worst[1]} (target {target}:1)")

    # Fixed surfaces over a pure white frame, after the veil.
    white = over(np.ones(3), FLOOR, VEIL)
    surfaces = {
        "plate 90% ground": over(white, hexrgb("0E120F"), 0.90),
        "rail 80% floor": over(white, FLOOR, 0.80),
        "caption chip 78% floor": over(white, FLOOR, 0.78),
        "terminal 70% floor on plate": over(over(white, hexrgb("0E120F"), 0.90), FLOOR, 0.70),
    }
    inks = {"bone": "DDE4D7", "bone-dim": "97A593", "amber": "AE8F61", "amber-lit": "D2BC93"}
    allowed = {
        "plate 90% ground": inks,
        "terminal 70% floor on plate": inks,
        "rail 80% floor": {"bone": inks["bone"]},
        "caption chip 78% floor": {"bone": inks["bone"]},
    }
    for name, bgc in surfaces.items():
        lb = float(lum(bgc))
        row = []
        for ink, hx in allowed[name].items():
            c = contrast(float(lum(hexrgb(hx))), lb)
            row.append(f"{ink} {c:.2f}")
            if c < 4.5:
                failures += 1
                print(f"FAIL {name}: {ink} {c:.2f}:1 < 4.5:1")
        print(f"{name} over pure white: " + ", ".join(row))

    print("OK" if not failures else f"{failures} failure(s)")
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
