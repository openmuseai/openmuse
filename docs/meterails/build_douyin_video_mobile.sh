#!/usr/bin/env bash
# 手机版 DSH 上线 · 20 秒竖版抖音视频（1080x1920）
# 时间轴刻意做成前 6 页快剪、最后一页（含 GitHub 地址）停留 5s。
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FFMPEG="${FFMPEG:-/opt/homebrew/bin/ffmpeg}"
SLIDES="$ROOT/build/post2-douyin-mobile/slides"
CLIPS="$ROOT/build/post2-douyin-mobile/clips"
OUT="$ROOT/build/post2-douyin-mobile/openmuse-mobile-dsh-20s.mp4"

rm -rf "$CLIPS"; mkdir -p "$CLIPS"

# 顺序与时长（秒）——合计 20.0s，末页 5s
ITEMS=(
  "00-cover-mobile.png 2.5"
  "01-mobile-gap.png 2.5"
  "02-gaps.png 3.0"
  "03-host-workspace.png 2.0"
  "04-host-repo.png 2.5"
  "05-seams.png 2.5"
  "06-outro-github.png 5.0"
)

: > "$CLIPS/list.txt"
i=0
TOTAL=0
for item in "${ITEMS[@]}"; do
  name="${item%% *}"; dur="${item##* }"
  out_fade=$(awk -v d="$dur" 'BEGIN{printf "%.2f", d-0.4}')
  printf '  · %-26s %ss\n' "$name" "$dur"
  "$FFMPEG" -hide_banner -loglevel error -y \
    -loop 1 -i "$SLIDES/$name" \
    -f lavfi -i anullsrc=r=44100:cl=stereo \
    -t "$dur" \
    -vf "fade=t=in:st=0:d=0.4:color=white,fade=t=out:st=${out_fade}:d=0.4:color=white,scale=1080:1920,format=yuv420p,fps=30" \
    -c:v libx264 -preset medium -crf 18 -pix_fmt yuv420p \
    -c:a aac -b:a 128k -ar 44100 -ac 2 -shortest \
    "$CLIPS/$(printf '%02d' "$i").mp4"
  echo "file '$CLIPS/$(printf '%02d' "$i").mp4'" >> "$CLIPS/list.txt"
  TOTAL=$(awk -v t="$TOTAL" -v d="$dur" 'BEGIN{printf "%.1f", t+d}')
  i=$((i+1))
done
printf '时间轴合计：%ss\n' "$TOTAL"

echo "拼接…"
"$FFMPEG" -hide_banner -loglevel error -y \
  -f concat -safe 0 -i "$CLIPS/list.txt" \
  -c copy -movflags +faststart "$OUT"

echo "输出：$OUT"
"$(dirname "$FFMPEG")/ffprobe" -hide_banner -v error -show_entries \
  format=duration,size:stream=codec_name,width,height,r_frame_rate \
  -of default=noprint_wrappers=1 "$OUT"
