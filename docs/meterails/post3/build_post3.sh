#!/usr/bin/env bash
# post3 录屏成片：片头封面标题卡 + 背景音乐（人声自动闪避）。
#
# 产物（post3/build/）：
#   openmuse-post3-01-landscape.mp4  1720x1080 横版（原画幅，不裁切）
#   openmuse-post3-01-douyin.mp4     1080x1920 竖版（模糊背景填充，抖音全屏）
#   02 同上一组
#
# 依赖：
#   build/bgm/tokyo-dreams-bed.wav  由 make_post3_bgm.sh 生成（-24 LUFS）
#   build/covers/cover-0*.png       由 make_post3_covers.py 生成
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FFMPEG="${FFMPEG:-/opt/homebrew/bin/ffmpeg}"
FFPROBE="${FFPROBE:-/opt/homebrew/bin/ffprobe}"

SRC_DIR="$ROOT"
BED="$ROOT/build/bgm/tokyo-dreams-bed.wav"
COVERS="$ROOT/build/covers"
OUTDIR="$ROOT/build"

COVER_DUR=3.0          # 片头封面停留时长（秒）
XFADE_ST=2.6           # 封面淡出到黑场的起点
BG_FADE_OUT=3.0        # 结尾音乐淡出
BGM_VOL_VOICE=0.6      # 有解说时 BGM 基础电平（≈ -28 LUFS）；说话时再被侧链压下去约 10 dB
BGM_VOL_SILENT=1.6     # 无解说时 BGM 电平（≈ -20 LUFS，纯音乐片更扎实）

# 竖版里居中视频两侧的模糊背景强度
BG_BLUR_SIGMA=20

# 可选：只渲染某一条，./build_post3.sh 01 或 ./build_post3.sh --clip=01
ONLY="${1:-all}"
ONLY="${ONLY#--clip=}"

CLIPS=(
  "01|Cap 2026-10-08 at 01.15.31.mp4|voice=0"
  "02|Cap 2026-10-08 at 01.43.57.mp4|voice=1"
)

for spec in "${CLIPS[@]}"; do
  IFS='|' read -r cid srcname flags <<<"$spec"
  [[ "$ONLY" == "all" || "$ONLY" == "$cid" ]] || continue
  src="$SRC_DIR/$srcname"
  [[ -f "$src" ]] || { echo "缺少素材：$src" >&2; exit 1; }
  [[ -f "$BED" ]] || { echo "缺少 BGM，请先跑 make_post3_bgm.sh" >&2; exit 1; }

  has_voice=0
  [[ "$flags" == *"voice=1"* ]] && has_voice=1

  dur=$("$FFPROBE" -v error -show_entries format=duration -of csv=p=0 "$src")
  total=$(awk -v d="$dur" -v c="$COVER_DUR" 'BEGIN{printf "%.3f", d+c}')
  fac=$(awk -v t="$total" -v f="$BG_FADE_OUT" 'BEGIN{printf "%.2f", t-f}')
  delay_ms=$(awk -v c="$COVER_DUR" 'BEGIN{printf "%d", c*1000}')

  if [[ "$has_voice" == "1" ]]; then
    bgm_vol="$BGM_VOL_VOICE"
    # BGM 以解说为侧链做闪避：说话时压下去，停顿处回到 bed 音量
    audio="[2:a]atrim=0:${total},asetpts=N/SR/TB,volume=${bgm_vol},afade=t=in:st=0:d=1.5,afade=t=out:st=${fac}:d=${BG_FADE_OUT}[bgm];
           [1:a]aresample=48000,adelay=delays=${delay_ms}:all=1,volume=1.0,asplit=2[voice][sc];
           [bgm][sc]sidechaincompress=threshold=0.03:ratio=8:attack=20:release=400[bgmd];
           [voice][bgmd]amix=inputs=2:normalize=0:duration=longest[aout]"
  else
    audio="[2:a]atrim=0:${total},asetpts=N/SR/TB,volume=${BGM_VOL_SILENT},afade=t=in:st=0:d=1.5,afade=t=out:st=${fac}:d=${BG_FADE_OUT}[aout]"
  fi

  # ---------- 横版：原画幅，不裁切 ----------
  out_l="$OUTDIR/openmuse-post3-${cid}-landscape.mp4"
  echo "▶ ${cid} 横版 1720x1080（片头 ${COVER_DUR}s + 正片 ${dur}s，解说=${has_voice}）"
  "$FFMPEG" -hide_banner -loglevel error -y \
    -loop 1 -t "$COVER_DUR" -i "$COVERS/cover-${cid}.png" \
    -i "$src" \
    -i "$BED" \
    -filter_complex "
      [0:v]fps=30,scale=1720:1080,setsar=1,format=yuv420p,
           fade=t=in:st=0:d=0.5,fade=t=out:st=${XFADE_ST}:d=0.4[cv];
      [1:v]fps=30,setsar=1,format=yuv420p,fade=t=in:st=0:d=0.3[mv];
      [cv][mv]concat=n=2:v=1:a=0[vout];
      ${audio}" \
    -map "[vout]" -map "[aout]" -t "$total" \
    -c:v libx264 -preset medium -crf 20 -pix_fmt yuv420p -r 30 \
    -c:a aac -b:a 192k -ar 48000 -ac 2 -movflags +faststart \
    "$out_l"

  # ---------- 竖版：模糊背景 + 居中正片 ----------
  out_v="$OUTDIR/openmuse-post3-${cid}-douyin.mp4"
  echo "▶ ${cid} 竖版 1080x1920（抖音全屏）"
  "$FFMPEG" -hide_banner -loglevel error -y \
    -loop 1 -t "$COVER_DUR" -i "$COVERS/cover-${cid}-vertical.png" \
    -i "$src" \
    -i "$BED" \
    -filter_complex "
      [0:v]fps=30,scale=1080:1920,setsar=1,format=yuv420p,
           fade=t=in:st=0:d=0.5,fade=t=out:st=${XFADE_ST}:d=0.4[cv];
      [1:v]fps=30,setsar=1,split=2[bg][fg];
      [bg]scale=1080:1920:force_original_aspect_ratio=increase,crop=1080:1920,
          gblur=sigma=${BG_BLUR_SIGMA},eq=brightness=-0.08[bgz];
      [fg]scale=1080:-2[fgz];
      [bgz][fgz]overlay=(W-w)/2:(H-h)/2,setsar=1,format=yuv420p,
          fade=t=in:st=0:d=0.3[mv];
      [cv][mv]concat=n=2:v=1:a=0[vout];
      ${audio}" \
    -map "[vout]" -map "[aout]" -t "$total" \
    -c:v libx264 -preset medium -crf 20 -pix_fmt yuv420p -r 30 \
    -c:a aac -b:a 192k -ar 48000 -ac 2 -movflags +faststart \
    "$out_v"

  for f in "$out_l" "$out_v"; do
    printf '  ✓ %s\n' "${f#$ROOT/}"
    "$FFPROBE" -v error -show_entries \
      format=duration,size:stream=codec_type,codec_name,width,height,r_frame_rate,channels,sample_rate \
      -of default=noprint_wrappers=1 "$f" | sed 's/^/      /'
  done
done

echo "全部完成 → $OUTDIR"
