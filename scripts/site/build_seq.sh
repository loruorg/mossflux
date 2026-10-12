#!/usr/bin/env bash
# Turn each source clip into a scroll-scrubbed frame sequence for the site:
# grayscale, faded in from black, 12 frames per second, WebP.
#   scripts/site/build_seq.sh ignite warp ...
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
for id in "$@"; do
  out="$here/../../site/seq/$id"
  rm -rf "$out"; mkdir -p "$out"
  tmp="$(mktemp -d)"
  ffmpeg -v error -y -i "$here/out/$id-l.mp4" -t 10 \
    -vf "fps=12,scale=1024:-2,hue=s=0,fade=in:st=0:d=1.2,eq=contrast=1.08" "$tmp/%03d.png"
  for f in "$tmp"/*.png; do
    cwebp -quiet -q 48 -m 6 "$f" -o "$out/$(basename "${f%.png}").webp"
  done
  rm -rf "$tmp"
  n=$(ls "$out" | wc -l | tr -d ' ')
  echo "$id: $n frames, $(du -sh "$out" | cut -f1)"
done
