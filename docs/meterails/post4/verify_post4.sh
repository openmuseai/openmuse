#!/usr/bin/env bash
# post4 成片自检：只定位问题，不判通过。结论仍需用户 1× 审看／审听。
# 用法：./verify_post4.sh    （输出同时写入 build/check/verify.log）
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FF="${FFMPEG:-/opt/homebrew/bin/ffmpeg}"
FP="${FFPROBE:-/opt/homebrew/bin/ffprobe}"
OUT="$ROOT/build/openmuse-post4-multiplatform-1920x1080.mp4"
CHK="$ROOT/build/check"
mkdir -p "$CHK"
exec > >(tee "$CHK/verify.log") 2>&1

N="fps=30,settb=AVTB,setpts=PTS-STARTPTS"
yavg(){ "$FF" -hide_banner -loglevel error -ss "$1" -i "$OUT" -frames:v 1 \
        -vf "${2:-null},signalstats,metadata=print:file=-" -f null - 2>&1 |
        grep -o "YAVG=[0-9.]*" | head -1 | cut -d= -f2; }
# 段内容区窗口 SSIM：$1 成片起点 $2 成片终点 $3 源片 $4 源起点 $5 源终点 $6 内容宽 $7 内容 x
wssim(){ "$FF" -hide_banner -ss "$1" -to "$2" -i "$OUT" -ss "$4" -to "$5" -i "$ROOT/$3" \
  -filter_complex "[0:v]crop=$6:1080:$7:0,${N}[a];[1:v]${N}[b];[a][b]ssim" -f null - 2>&1 |
  grep -o "All:[0-9.]*" | tail -1 | cut -d: -f2; }

echo "### post4 成片自检  $(date '+%Y-%m-%d %H:%M:%S')"
echo "文件：${OUT#$ROOT/}"
echo

echo "## 1. 容器与流"
"$FP" -v error -show_entries format=duration,size,bit_rate \
  -show_entries stream=index,codec_type,codec_name,width,height,r_frame_rate,pix_fmt,sample_rate,channels \
  -of default=noprint_wrappers=1 "$OUT" | sed 's/^/  /'
echo -n "  解码帧数："
"$FP" -v error -count_frames -select_streams v:0 -show_entries stream=nb_read_frames \
  -of csv=p=0 "$OUT"
echo "  预期：42.000s / 1920x1080 / 30fps / 1260 帧"
echo

echo "## 2. 完整解码（无输出即无错误）"
if "$FF" -v error -i "$OUT" -f null - 2>&1 | grep -q .; then
  "$FF" -v error -i "$OUT" -f null - 2>&1 | sed 's/^/  /'
else
  echo "  ok：无解码错误"
fi
echo

echo "## 3. 黑帧（预期只出现在首尾淡入淡出，无 ≥0.4s 黑场）"
"$FF" -hide_banner -i "$OUT" -vf blackdetect=d=0.4:pix_th=0.10 -an -f null - 2>&1 |
  grep blackdetect | sed 's/^/  /' || echo "  ok：无 ≥0.4s 黑场"
echo

echo "## 4. 段序与几何（成片内容区 vs 对应源片；越低越说明不是这一段）"
printf "  Desktop  t=0.6-19.0s  ↔ 源 0.600-19.000s  SSIM=%s\n" "$(wssim 0.6 19.0 Desktop.mp4 0.600 19.000 1720 100)"
printf "  Web      t=20.0-31.3s ↔ 源 0.800-12.100s  SSIM=%s\n" "$(wssim 20.0 31.3 Web.mp4 0.800 12.100 1832 44)"
printf "  Mobile   t=32.0-41.4s ↔ 源 0.634-10.034s  SSIM=%s\n" "$(wssim 32.0 41.4 Mobile.mp4 0.634 10.034 1664 128)"
printf "  负向对照 成片 t=5s ↔ Web 源 t=5s              SSIM=%s\n" \
  "$("$FF" -hide_banner -ss 5 -to 7 -i "$OUT" -ss 5 -to 7 -i "$ROOT/Web.mp4" \
     -filter_complex "[0:v]crop=1720:1080:100:0,${N}[a];[1:v]crop=1720:1080:56:0,${N}[b];[a][b]ssim" \
     -f null - 2>&1 | grep -o "All:[0-9.]*" | tail -1 | cut -d: -f2)"
echo "  说明：Web/Mobile 源局部时间比标称早 1 帧（0.033s，xfade 取帧造成），不影响观感。"
echo

echo "## 5. 两侧填充（模糊背景，非黑边；16 为 limited-range 黑）"
for t in 5 25 37; do
  printf "  t=%ss  左 YAVG=%s  右 YAVG=%s  中心 YAVG=%s\n" "$t" \
    "$(yavg "$t" "crop=60:1080:0:0")" "$(yavg "$t" "crop=60:1080:1860:0")" \
    "$(yavg "$t" "crop=200:200:860:440")"
done
echo

echo "## 6. 转场：中点是否为两端真实交叉溶解（合成 50/50 与实测中点比对）"
xfade_check(){ # $1=转场起点 $2=中点 $3=终点
  local a="$1" m="$2" b="$3" d="$CHK"
  "$FF" -hide_banner -loglevel error -y -ss "$a" -i "$OUT" -frames:v 1 "$d/_a.png"
  "$FF" -hide_banner -loglevel error -y -ss "$m" -i "$OUT" -frames:v 1 "$d/_m.png"
  "$FF" -hide_banner -loglevel error -y -ss "$b" -i "$OUT" -frames:v 1 "$d/_b.png"
  local s_blend s_a s_b
  s_blend=$("$FF" -hide_banner -i "$d/_a.png" -i "$d/_b.png" -i "$d/_m.png" \
    -filter_complex "[0:v]format=yuv420p[a];[1:v]format=yuv420p[b];[a][b]blend=all_mode=average,format=yuv420p[ab];[2:v]format=yuv420p[m];[ab][m]ssim" \
    -f null - 2>&1 | grep -o "All:[0-9.]*" | tail -1 | cut -d: -f2)
  s_a=$("$FF" -hide_banner -i "$d/_a.png" -i "$d/_m.png" \
    -filter_complex "[0:v]format=yuv420p[a];[1:v]format=yuv420p[m];[a][m]ssim" \
    -f null - 2>&1 | grep -o "All:[0-9.]*" | tail -1 | cut -d: -f2)
  s_b=$("$FF" -hide_banner -i "$d/_b.png" -i "$d/_m.png" \
    -filter_complex "[0:v]format=yuv420p[b];[1:v]format=yuv420p[m];[b][m]ssim" \
    -f null - 2>&1 | grep -o "All:[0-9.]*" | tail -1 | cut -d: -f2)
  printf "  中点 t=%ss：vs(前段+后段 50/50)=%s   vs 前段=%s   vs 后段=%s\n" "$m" "$s_blend" "$s_a" "$s_b"
}
xfade_check 19.167 19.467 19.767
xfade_check 31.334 31.634 31.934
echo "  说明：中点与 50/50 合成接近、且同时不像任一单端，可判定为真实交叉溶解。"
echo

echo "## 7. 声音（只用 BGM；源声为数字静音，无对白，故不闪避）"
rms(){ "$FF" -hide_banner -ss "$1" -t "$2" -i "$OUT" -af astats -f null - 2>&1 |
       grep "RMS level dB" | tail -1 | awk '{print $NF}'; }
printf "  开场 0.0-0.2s   RMS=%s dB（音乐淡入中）\n" "$(rms 0 0.2)"
printf "  中段 20.0-21.0s RMS=%s dB\n" "$(rms 20 1)"
printf "  结尾 41.8-42.0s RMS=%s dB（音乐淡出中）\n" "$(rms 41.8 0.2)"
"$FF" -hide_banner -i "$OUT" -af ebur128=peak=true -f null - 2>&1 |
  grep -E "^\s+(I|LRA|Peak|LRA low|LRA high):" | tail -3 | sed 's/^/  /'
echo "  目标：≈ -16 LUFS-I，真峰值 ≤ -1 dBTP"
echo

echo "## 8. 抽帧证据（build/check/）：布局 layout.jpg、转场 transitions.jpg"
echo "自检完成：以上只定位问题；主观画面与听感仍需 1× 审看／审听。"
