#!/usr/bin/env bash
# Encode one generated clip into a seamless background loop for site/bg/.
#
#   scripts/site/encode_bg.sh <theme> <source.mp4> <xfade|pingpong> [fade-seconds]
#
# xfade:    blends the last fade-seconds (default 1) into the start; for clips
#           whose last frame is near the first.
# pingpong: plays forward, then backward; for clips that move one way.
# Writes site/bg/<theme>.av1.mp4 (1080p), .hevc.mp4 (1080p), .720.mp4 (H.264)
# and .webp (poster: the first frame of the loop). No audio.
set -euo pipefail

id=$1 src=$2 mode=$3 fade=${4:-1}
root=$(cd "$(dirname "$0")/../.." && pwd)
out=$root/site/bg
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$out"

ffmpeg -v error -y -i "$src" -vf "fps=24,scale=1920:1080:force_original_aspect_ratio=increase:flags=lanczos,crop=1920:1080,setsar=1" \
  -an -c:v ffv1 "$tmp/fwd.mkv"
n=$(ffprobe -v error -count_frames -select_streams v -show_entries stream=nb_read_frames -of csv=p=0 "$tmp/fwd.mkv")
dur=$(echo "$n / 24" | bc -l)

case $mode in
  xfade)
    # The loop starts at source time (dur - fade) and ends just before it, so the wrap is invisible.
    body=$(echo "$dur - $fade" | bc -l)
    ffmpeg -v error -y -i "$tmp/fwd.mkv" -filter_complex "
      [0:v]split=3[a][b][c];
      [a]trim=0:$fade,setpts=PTS-STARTPTS[head];
      [b]trim=$fade:$body,setpts=PTS-STARTPTS[mid];
      [c]trim=$body,setpts=PTS-STARTPTS[tail];
      [tail][head]xfade=transition=fade:duration=$fade:offset=0[seam];
      [seam][mid]concat=n=2:v=1[v]" -map "[v]" -c:v ffv1 "$tmp/loop.mkv"
    ;;
  pingpong)
    # Forward, then backward without the two turning frames.
    ffmpeg -v error -y -i "$tmp/fwd.mkv" -filter_complex "
      [0:v]split[f][r];
      [r]reverse,trim=start_frame=1:end_frame=$((n - 1)),setpts=PTS-STARTPTS[b];
      [f][b]concat=n=2:v=1[v]" -map "[v]" -c:v ffv1 "$tmp/loop.mkv"
    ;;
  *) echo "mode must be xfade or pingpong" >&2; exit 2 ;;
esac

SVT_LOG=1 ffmpeg -v error -y -i "$tmp/loop.mkv" -c:v libsvtav1 -preset 4 -crf 42 -g 240 -pix_fmt yuv420p10le \
  -svtav1-params tune=0:film-grain=6 -movflags +faststart -an "$out/$id.av1.mp4"
ffmpeg -v error -y -i "$tmp/loop.mkv" -c:v libx265 -preset slow -crf 29 -tag:v hvc1 -pix_fmt yuv420p \
  -x265-params log-level=error:keyint=240 -movflags +faststart -an "$out/$id.hevc.mp4"
ffmpeg -v error -y -i "$tmp/loop.mkv" -vf scale=1280:720:flags=lanczos -c:v libx264 -preset slow -crf 26 \
  -profile:v high -pix_fmt yuv420p -g 240 -movflags +faststart -an "$out/$id.720.mp4"
ffmpeg -v error -y -i "$tmp/loop.mkv" -frames:v 1 "$tmp/poster.png"
cwebp -quiet -q 72 "$tmp/poster.png" -o "$out/$id.webp"

ls -l "$out/$id".* | awk '{ printf "%-40s %6.0f KB\n", $NF, $5 / 1024 }'
