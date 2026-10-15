# Post 3 · 录屏成片（封面 + 背景音乐）

`post3/` 放的是第 3 篇的两段工作台录屏。本目录的脚本给它们加**片头封面标题卡**和**背景音乐**，
输出横版（原画幅）与竖版（抖音全屏）两套成片。

## 素材

| 文件 | 时长 | 画幅 | 原声 |
| --- | --- | --- | --- |
| `Cap 2026-10-08 at 01.15.31.mp4` | 41.2s | 1720×1080 | 无（-91 dB，全程静音） |
| `Cap 2026-10-08 at 01.43.57.mp4` | 58.9s | 1720×1080 | 有解说（RMS ≈ -18 dB，含明显停顿） |

内容取自抽帧 OCR：① 工作台里把 post2 的 4 张截图交给 DSH 做中英 OCR、复盘素材并回读
`build/post2-douyin` 的产物；② 跟 README 走查 post2 抖音流水线（交付形态、10 页幻灯片、
扫码登录、发布留痕）。

## 产物（`build/`）

```
build/covers/cover-01.png             1720×1080  横版封面图（可直接上传做平台封面）
build/covers/cover-01-vertical.png    1080×1920  竖版封面图
build/covers/cover-02*.png                       同上
build/bgm/tokyo-dreams-bed.wav        90s / -24 LUFS  BGM bed
build/openmuse-post3-01-landscape.mp4 1720×1080  片头 3s + 正片，横版
build/openmuse-post3-01-douyin.mp4    1080×1920  竖版，模糊背景 + 居中正片
build/openmuse-post3-02-landscape.mp4
build/openmuse-post3-02-douyin.mp4
```

## 跑法

```bash
cd post3
./make_post3_bgm.sh          # ① 合成 BGM bed（只需一次）
python3 make_post3_covers.py # ② 出封面图（横 + 竖）
./build_post3.sh             # ③ 合成 4 条成片
```

单独重做某一步即可，脚本互相独立；`build_post3.sh` 依赖 ①② 的产物，缺了会直接报错退出。

## 背景音乐

音源是 macOS 自带的 **GarageBand Apple Loops**（`/Library/Audio/Apple Loops/Apple/07 Chillwave`），
免版税、可随作品发布，不用联网下载。「Tokyo Dreams」是同一首曲子的分轨（Bass / Pad / Guitar /
Synth / Delay Guitar），每条都是同调同速的 4 小节循环（48 kHz 下正好 512000 采样），
所以循环拼接不会有对不齐的问题。

混音：分轨按配器权重叠加 → `alimiter` 限幅（实测 -8.8 LUFS）→ 乘 0.174 固定衰减到 **-24 LUFS**。
刻意不做动态压缩，混音时电平可预测。

两条片子的用法不同：

- **① 无原声**：BGM 就是全部声音，bed 抬到 1.6 倍（≈ -20 LUFS），纯音乐片才不显得虚。
- **② 有解说**：BGM 基础电平 0.6（≈ -28 LUFS），并以解说为侧链做 **闪避**（`sidechaincompress`，
  阈值 0.03 / 比值 8 / 释放 400ms）——说话时自动压下约 10 dB 到 -38 dB 左右，停顿处回到基础电平。
  封面卡那 3 秒没有说话，音乐自然停在基础电平，片头不会「有画面没声音」。

换音乐只需改 `make_post3_bgm.sh` 里的 `FAMILY` / `LAYERS`（任意 Apple Loops 同曲分轨都可以），
或直接把 `build/bgm/tokyo-dreams-bed.wav` 换成自己的音轨（44.1/48 kHz 立体声即可，
`build_post3.sh` 只按固定文件名读取它）。

## 封面

`make_post3_covers.py` 沿用 post2 系列的视觉语言（白底 + 蓝色强调 + STHeiti 真实字体度量自动折行），
按录屏内容草拟了文案，**文案是占位性质、可随时覆盖重出**：

```bash
python3 make_post3_covers.py --clip 02 \
  --title "一条抖音流水线的完整走查" \
  --sub "交付形态 · 10 页幻灯片 · 扫码登录 · 发布留痕" \
  --desc "……" --chip "原声解说 · 59 秒"
./build_post3.sh
```

片头结构：封面卡停留 3.0s（0.5s 淡入、2.6s 起 0.4s 淡出到黑），正片 0.3s 淡入，硬切干净不闪白。

## 可调参数（`build_post3.sh` 顶部）

| 变量 | 默认 | 说明 |
| --- | --- | --- |
| `COVER_DUR` | 3.0 | 片头封面时长（秒） |
| `XFADE_ST` | 2.6 | 封面开始淡出的时间点 |
| `BG_FADE_OUT` | 3.0 | 结尾音乐淡出时长 |
| `BGM_VOL_VOICE` | 0.6 | 有解说时 BGM 基础电平（≈ -28 LUFS） |
| `BGM_VOL_SILENT` | 1.6 | 无解说时 BGM 电平（≈ -20 LUFS） |
| `BG_BLUR_SIGMA` | 20 | 竖版模糊背景强度 |

## 已知边界

- 竖版是从 1720×1080 横版录屏转的，正片按 1080 宽居中，上下留模糊背景；
  桌面 UI 的字在手机竖屏上会偏小，要看清细节建议用横版。
- 录屏原声里有解说（②），所以 BGM 只能压在下面；如果后续要重配音，先把 `voice=1` 的
  分支换成纯 BGM，或直接替换 `[1:a]` 输入。
