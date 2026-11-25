#!/usr/bin/env bash
# post4 三平台录屏拼接成片：Desktop → Web → Mobile，段间交叉溶解 + 背景音乐。
#
# 产物（post4/build/）：
#   openmuse-post4-multiplatform-1920x1080.mp4   1920x1080 横版总片
#
# 素材画幅不同（Desktop 1720x1080 / Web 1832x1080 / Mobile 1664x1080），处理方式：
#   统一到 1920x1080 画布；每段按高度等比缩放居中（不拉伸、不裁切内容），
#   两侧留白用同段画面的模糊放大填满，避免黑边；三条都是 1080 高，所以正片不放大、不糊。
#
# 声音：三条素材原声实测为数字静音（Peak -inf dBFS），因此成片只用 BGM，
#       不需要闪避（ducking）；BGM 从 bed 的 -24 LUFS 抬到约 -16 LUFS-I。
#
# 依赖：
#   build/bgm/tokyo-dreams-bed.wav   由 make_post4_bgm.sh 生成（-24 LUFS）
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FFMPEG="${FFMPEG:-/opt/homebrew/bin/ffmpeg}"
FFPROBE="${FFPROBE:-/opt/homebrew/bin/ffprobe}"

BED="$ROOT/build/bgm/tokyo-dreams-bed.wav"
OUTDIR="$ROOT/build"
OUT="$OUTDIR/openmuse-post4-multiplatform-1920x1080.mp4"

# ---------- 可调参数 ----------
CANVAS_W=1920          # 画布宽
CANVAS_H=1080          # 画布高
FPS=30                 # 统一帧率
XFADE_DUR=0.6          # 段间交叉溶解时长（秒）
LEAD_FADE_IN=0.4       # 开场从黑淡入
TAIL_FADE_OUT=0.6      # 结尾淡出到黑
BGM_VOL=2.5            # bed 实测 -24 LUFS；×2.5 ≈ -16 LUFS-I
BGM_FADE_IN=1.0        # 音乐淡入
BGM_FADE_OUT=3.0       # 音乐淡出
BLUR_SIGMA=24          # 两侧填充背景的模糊强度
BG_BRIGHTNESS=-0.06    # 填充背景压暗，避免和正片抢注意力（压太狠会把暗部压死）

# 拼接顺序即数组顺序：Desktop → Web → Mobile
CLIPS=(Desktop.mp4 Web.mp4 Mobile.mp4)

for c in "${CLIPS[@]}"; do
  [[ -f "$ROOT/$c" ]] || { echo "缺少素材：$ROOT/$c" >&2; exit 1; }
done
[[ -f "$BED" ]] || { echo "缺少 BGM，请先跑 ./make_post4_bgm.sh" >&2; exit 1; }

# ---------- 读时长，算转场落点 ----------
durs=()
for c in "${CLIPS[@]}"; do
  durs+=("$("$FFPROBE" -v error -show_entries format=duration -of csv=p=0 "$ROOT/$c")")
done

total="$(awk -v d="${durs[*]}" -v x="$XFADE_DUR" 'BEGIN{
  n=split(d,a," "); t=0; for(i=1;i<=n;i++) t+=a[i]; printf "%.3f", t-(n-1)*x }')"
tail_st="$(awk -v t="$total" -v f="$TAIL_FADE_OUT" 'BEGIN{printf "%.3f", t-f}')"
bgm_out="$(awk -v t="$total" -v f="$BGM_FADE_OUT" 'BEGIN{printf "%.3f", t-f}')"

# 每个转场的 offset：前一串累计时长 - 已消耗的溶解
offsets=()
acc="${durs[0]}"
for ((i=1; i<${#CLIPS[@]}; i++)); do
  off="$(awk -v a="$acc" -v x="$XFADE_DUR" 'BEGIN{printf "%.3f", a-x}')"
  offsets+=("$off")
  acc="$(awk -v o="$off" -v d="${durs[$i]}" 'BEGIN{printf "%.3f", o+d}')"
done

echo "▶ 拼接顺序：${CLIPS[*]}"
printf '  段时长：%s\n' "${durs[*]}"
printf '  转场落点：%s  (溶解 %ss)\n' "${offsets[*]}" "$XFADE_DUR"
echo "  成片总长：${total}s"

# ---------- 组装滤镜 ----------
inputs=(); parts=()
for i in "${!CLIPS[@]}"; do
  inputs+=(-i "$ROOT/${CLIPS[$i]}")
  parts+=("[${i}:v]fps=${FPS},settb=AVTB,setpts=PTS-STARTPTS,split=2[bgs${i}][fgs${i}];
           [bgs${i}]scale=${CANVAS_W}:${CANVAS_H}:force_original_aspect_ratio=increase,
                   crop=${CANVAS_W}:${CANVAS_H},gblur=sigma=${BLUR_SIGMA},
                   eq=brightness=${BG_BRIGHTNESS}[bg${i}];
           [fgs${i}]scale=-2:${CANVAS_H}[fg${i}];
           [bg${i}][fg${i}]overlay=(W-w)/2:(H-h)/2,setsar=1,format=yuv420p[v${i}]")
done

prev="v0"
for ((i=1; i<${#CLIPS[@]}; i++)); do
  label="x${i}"
  parts+=("[${prev}][v${i}]xfade=transition=fade:duration=${XFADE_DUR}:offset=${offsets[$((i-1))]}[${label}]")
  prev="$label"
done
parts+=("[${prev}]fade=t=in:st=0:d=${LEAD_FADE_IN},fade=t=out:st=${tail_st}:d=${TAIL_FADE_OUT},format=yuv420p[vout]")

bed_idx="${#CLIPS[@]}"
inputs+=(-i "$BED")
parts+=("[${bed_idx}:a]atrim=0:${total},asetpts=N/SR/TB,aresample=48000,
         volume=${BGM_VOL},afade=t=in:st=0:d=${BGM_FADE_IN},
         afade=t=out:st=${bgm_out}:d=${BGM_FADE_OUT}[aout]")

filter="$(printf '%s;' "${parts[@]}")"
filter="${filter%;}"

echo "▶ 渲染 ${CANVAS_W}x${CANVAS_H} @${FPS}fps → ${OUT#$ROOT/}"
"$FFMPEG" -hide_banner -loglevel error -y \
  "${inputs[@]}" \
  -filter_complex "$filter" \
  -map "[vout]" -map "[aout]" -t "$total" \
  -c:v libx264 -preset medium -crf 20 -pix_fmt yuv420p -r "$FPS" \
  -c:a aac -b:a 192k -ar 48000 -ac 2 -movflags +faststart \
  "$OUT"

echo "▶ 成片信息"
"$FFPROBE" -v error -show_entries \
  format=duration,size:stream=codec_type,codec_name,width,height,r_frame_rate,channels,sample_rate \
  -of default=noprint_wrappers=1 "$OUT" | sed 's/^/  /'

echo "▶ 成片响度（目标 ≈ -16 LUFS-I，真峰值 ≤ -1 dBTP）"
"$FFMPEG" -hide_banner -i "$OUT" -af ebur128=peak=true -f null - 2>&1 |
  grep -E "^\s+(I|LRA|Peak):" | tail -3 | sed 's/^/  /'

echo "完成 → ${OUT#$ROOT/}"
