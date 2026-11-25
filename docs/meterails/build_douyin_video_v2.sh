#!/usr/bin/env bash
# 把 build/post2-douyin/slides/*.png 合成 1080x1920 竖版抖音视频（静音音轨 + 白场淡入淡出）。
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FFMPEG="${FFMPEG:-/opt/homebrew/bin/ffmpeg}"
SLIDES="$ROOT/build/post2-douyin/slides"
CLIPS="$ROOT/build/post2-douyin/clips"
OUT="$ROOT/build/post2-douyin/openmuse-dsh-codex.mp4"

rm -rf "$CLIPS"; mkdir -p "$CLIPS"

# 幻灯片顺序与每张时长（秒），总时长约 70s
ITEMS=(
  "00-cover.png 5"
  "01-pains.png 7"
  "02-gaps.png 9"
  "03-principle.png 7"
  "04-host-workspace.png 6"
  "05-host-preview.png 6"
  "06-host-trajectory.png 6"
  "07-host-repo.png 7"
  "08-seams.png 9"
  "09-outro.png 8"
)

: > "$CLIPS/list.txt"
i=0
for item in "${ITEMS[@]}"; do
  name="${item%% *}"; dur="${item##* }"
  out_fade=$(awk -v d="$dur" 'BEGIN{printf "%.2f", d-0.5}')
  printf '  · %s (%ss)\n' "$name" "$dur"
  "$FFMPEG" -hide_banner -loglevel error -y \
    -loop 1 -i "$SLIDES/$name" \
    -f lavfi -i anullsrc=r=44100:cl=stereo \
    -t "$dur" \
    -vf "fade=t=in:st=0:d=0.5:color=white,fade=t=out:st=${out_fade}:d=0.5:color=white,scale=1080:1920,format=yuv420p,fps=30" \
    -c:v libx264 -preset medium -crf 18 -pix_fmt yuv420p \
    -c:a aac -b:a 128k -ar 44100 -ac 2 -shortest \
    "$CLIPS/$(printf '%02d' "$i").mp4"
  echo "file '$CLIPS/$(printf '%02d' "$i").mp4'" >> "$CLIPS/list.txt"
  i=$((i+1))
done

echo "拼接…"
"$FFMPEG" -hide_banner -loglevel error -y \
  -f concat -safe 0 -i "$CLIPS/list.txt" \
  -c copy -movflags +faststart "$OUT"

echo "输出：$OUT"
"$(dirname "$FFMPEG")/ffprobe" -hide_banner -v error -show_entries \
  format=duration,size:stream=codec_name,width,height,r_frame_rate \
  -of default=noprint_wrappers=1 "$OUT"
