#!/usr/bin/env bash
# 合成 post4 的背景音乐 bed。
#
# 音源：macOS 自带 GarageBand Apple Loops（免版税，可随作品发布），与 post3 同一首曲子
#       07 Chillwave /「Tokyo Dreams」的分轨：Bass / Pad / Guitar / Synth / Delay Guitar。
#       同一系列沿用同一 bed，跨篇听感一致；每条分轨都是同调同速的 4 小节循环。
#
# 做法：分轨各循环铺满 BED_DUR → 按乐器权重混音 → 限幅 → 静态衰减到 -24 LUFS。
#       不做动态压缩，成片混音时电平可预测。
# 输出：post4/build/bgm/tokyo-dreams-bed.wav
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FFMPEG="${FFMPEG:-/opt/homebrew/bin/ffmpeg}"
LOOPS="/Library/Audio/Apple Loops/Apple/07 Chillwave"
FAMILY="Tokyo Dreams"
OUT="$ROOT/build/bgm"
STEMS="$OUT/stems"

BED_DUR=70            # bed 长度（秒），需 ≥ 成片总长（本期约 42s）
BED_GAIN=0.174        # 混音限幅后实测约 -8.8 LUFS，乘 0.174 ≈ -24 LUFS

# 乐器:权重（同曲分轨，权重即配器比例）
LAYERS=("Bass:0.95" "Pad:0.85" "Guitar:0.70" "Synth:0.60" "Delay Guitar:0.35")

[[ -d "$LOOPS" ]] || { echo "找不到 Apple Loops 目录：$LOOPS" >&2; exit 1; }
mkdir -p "$STEMS"

inputs=(); filters=(); weights=(); i=0
for layer in "${LAYERS[@]}"; do
  name="${layer%%:*}"; w="${layer##*:}"
  src="$LOOPS/$FAMILY $name.caf"
  [[ -f "$src" ]] || { echo "缺少音源：$src" >&2; exit 1; }
  wav="$STEMS/$(echo "$name" | tr ' ' '_' | tr 'A-Z' 'a-z').wav"
  if [[ ! -f "$wav" ]]; then
    echo "  解码 $FAMILY $name.caf"
    "$FFMPEG" -hide_banner -loglevel error -y -i "$src" -ar 48000 -ac 2 -c:a pcm_s16le "$wav"
  fi
  inputs+=(-stream_loop -1 -i "$wav")
  filters+=("[${i}:a]")
  weights+=("$w")
  i=$((i+1))
done

wstr="${weights[*]}"
mix="$(printf '%s' "${filters[@]}")amix=inputs=${i}:normalize=0:weights=${wstr},alimiter=limit=0.9:level=disabled,volume=${BED_GAIN}[m]"

echo "▶ 混音 ${i} 轨 → ${BED_DUR}s bed"
"$FFMPEG" -hide_banner -loglevel error -y \
  "${inputs[@]}" \
  -filter_complex "$mix" \
  -map "[m]" -t "$BED_DUR" -c:a pcm_s16le "$OUT/tokyo-dreams-bed.wav"

echo "▶ 校验响度（目标 -24 LUFS）"
"$FFMPEG" -hide_banner -i "$OUT/tokyo-dreams-bed.wav" -af ebur128=peak=true -f null - 2>&1 |
  grep -E "^\s+(I|Peak):" | sed 's/^/  /'

echo "输出：${OUT#$ROOT/}/tokyo-dreams-bed.wav"
