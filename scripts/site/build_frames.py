"""Cut the background videos into scroll frames and measure what text needs.

    .venv/bin/python scripts/site/build_frames.py [theme-id ...]

Reads scripts/site/out/<theme>-<l|p>.mp4 (see gen_videos.mjs) and writes

    site/bg/<theme>/<l|p>/000.webp ... 071.webp
    site/bg/manifest.js        window.MOSSFLUX_BG, read by site/bg.js
    site/index.html            the theme list between /*@themes*/ and /*@end*/

For every frame it also records `a`: the smallest uniform dark overlay alpha
that keeps bone text (#DDE4D7) at CONTRAST:1 or better over the BRIGHTEST
16 px block anywhere in the frame. Compositing is done in sRGB space, the way
browsers blend, and the overlay colour is the page floor. The maximum block is
used, not a percentile, so the guarantee holds for any crop the page shows.
The page turns `a` into the opacity of the scrim behind free-standing text.
"""

import json
import os
import re
import shutil
import subprocess
import sys
import tempfile

import numpy as np
from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
SRC = os.path.join(HERE, "out")
DST = os.path.join(ROOT, "site", "bg")

FRAMES = 72
SIZE = {"l": (1280, 720), "p": (576, 1024)}
QUALITY = {"l": 66, "p": 64}
BLOCK = 16
CONTRAST = 5.0
FLOOR = np.array([8, 10, 9], dtype=np.float64) / 255.0  # --floor, #080A09
BONE = (221, 228, 215)                                   # --bone, #DDE4D7


def srgb_to_linear(c):
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


def rel_lum(rgb):
    lin = srgb_to_linear(rgb)
    return lin[..., 0] * 0.2126 + lin[..., 1] * 0.7152 + lin[..., 2] * 0.0722


L_BONE = float(rel_lum(np.array(BONE, dtype=np.float64) / 255.0))
L_MAX = (L_BONE + 0.05) / CONTRAST - 0.05


def brightest_block(rgb, alpha):
    """Max 16x16 block-mean luminance after an sRGB-space overlay of FLOOR."""
    c = rgb * (1.0 - alpha) + FLOOR * alpha
    y = rel_lum(c)
    h, w = y.shape
    h, w = h - h % BLOCK, w - w % BLOCK
    y = y[:h, :w].reshape(h // BLOCK, BLOCK, w // BLOCK, BLOCK).mean(axis=(1, 3))
    return float(y.max())


def needed_alpha(rgb):
    if brightest_block(rgb, 0.0) <= L_MAX:
        return 0.0
    lo, hi = 0.0, 1.0
    for _ in range(14):
        mid = (lo + hi) / 2
        if brightest_block(rgb, mid) <= L_MAX:
            hi = mid
        else:
            lo = mid
    return hi


def frame_count(path):
    out = subprocess.check_output([
        "ffprobe", "-v", "error", "-count_frames", "-select_streams", "v:0",
        "-show_entries", "stream=nb_read_frames", "-of", "csv=p=0", path,
    ])
    return int(out.strip())


def build(theme, orient):
    src = os.path.join(SRC, f"{theme}-{orient}.mp4")
    if not os.path.exists(src):
        return None
    w, h = SIZE[orient]
    total = frame_count(src)
    picks = [round(k * (total - 1) / (FRAMES - 1)) for k in range(FRAMES)]
    out_dir = os.path.join(DST, theme, orient)
    shutil.rmtree(out_dir, ignore_errors=True)
    os.makedirs(out_dir)

    with tempfile.TemporaryDirectory() as tmp:
        subprocess.check_call([
            "ffmpeg", "-v", "error", "-y", "-i", src,
            "-vf", f"scale={w}:{h}:flags=lanczos,format=rgb24",
            "-fps_mode", "passthrough", os.path.join(tmp, "%04d.png"),
        ])
        alphas, sizes = [], []
        for k, n in enumerate(picks):
            png = os.path.join(tmp, f"{n + 1:04d}.png")
            webp = os.path.join(out_dir, f"{k:03d}.webp")
            subprocess.check_call([
                "cwebp", "-quiet", "-q", str(QUALITY[orient]), "-m", "6",
                "-sharp_yuv", "-metadata", "none", png, "-o", webp,
            ])
            sizes.append(os.path.getsize(webp))
            # Measure the bytes that ship: lossy coding can lift a highlight.
            rgb = np.asarray(Image.open(webp).convert("RGB"), dtype=np.float64) / 255.0
            alphas.append(needed_alpha(rgb))

    # A scrim that flickers frame to frame reads as a fault, so each value is
    # the max over a five-frame window: never lower than any frame it covers.
    a = np.array(alphas)
    smooth = [float(a[max(0, i - 2): i + 3].max()) for i in range(len(a))]
    return {
        "n": FRAMES, "w": w, "h": h,
        "kb": round(sum(sizes) / 1024),
        "a": [round(v + 0.0005, 3) for v in smooth],
    }


def main():
    cfg = json.load(open(os.path.join(HERE, "themes.json")))
    only = set(sys.argv[1:])
    manifest_path = os.path.join(DST, "manifest.js")
    themes = {}
    if os.path.exists(manifest_path):
        txt = open(manifest_path).read()
        themes = json.loads(txt[txt.index("{"): txt.rindex("}") + 1]).get("themes", {})

    for t in cfg["themes"]:
        if only and t["id"] not in only:
            continue
        entry = {"name": t["name"]}
        for o in ("l", "p"):
            r = build(t["id"], o)
            if r:
                entry[o] = r
                print(f"{t['id']}-{o}: {r['n']} frames, {r['kb']} KiB, "
                      f"scrim alpha {min(r['a']):.2f}..{max(r['a']):.2f}")
        if "l" in entry and "p" in entry:
            themes[t["id"]] = entry
        else:
            print(f"{t['id']}: incomplete, left out of the manifest")

    order = [t["id"] for t in cfg["themes"] if t["id"] in themes]
    data = {"v": 1, "contrast": CONTRAST, "order": order,
            "themes": {k: themes[k] for k in order}}
    with open(manifest_path, "w") as f:
        f.write("/* generated by scripts/site/build_frames.py, do not edit */\n")
        f.write("window.MOSSFLUX_BG = " + json.dumps(data, separators=(",", ":")) + ";\n")
    print(f"manifest: {len(order)} themes -> {os.path.relpath(manifest_path, ROOT)}")

    # The head script in index.html picks a theme before first paint, so it
    # carries its own copy of the list. Rewrite it here so the two never drift.
    index = os.path.join(ROOT, "site", "index.html")
    html = open(index).read()
    marked = "/*@themes*/" + json.dumps(order, separators=(",", ":")) + "/*@end*/"
    html, hits = re.subn(r"/\*@themes\*/.*?/\*@end\*/", lambda m: marked, html, count=1)
    if hits != 1:
        sys.exit("index.html: theme marker not found")
    open(index, "w").write(html)


if __name__ == "__main__":
    main()
