# Post 4 · 三平台录屏拼接成片（Desktop → Web → Mobile + 背景音乐）

`post4/` 放的是同一产品在三种客户端上的录屏。本目录把它们按 **Desktop → Web → Mobile** 的顺序
拼成一条 1920×1080 成片，段间用交叉溶解转场，全程铺一条背景音乐。

本期属于单段任务（拼接 + 配乐，不新增画面设计与文案），按
[视频项目规则](../Table-VideoSOP/AGENTS.md) 的单段约定：依据、参数与验收证据留在这份 README，
不新建整期大段表或 QA 表。

## 素材

| 文件 | 时长 | 画幅 | 帧率/编码 | 原声 |
| --- | --- | --- | --- | --- |
| `Desktop.mp4` | 19.77s | 1720×1080 | 30fps / H.264 bt709 | AAC 48k 立体声，**数字静音**（I −70 LUFS，Peak −inf） |
| `Web.mp4` | 12.77s | 1832×1080 | 30fps / H.264 bt709 | 同上 |
| `Mobile.mp4` | 10.67s | 1664×1080 | 30fps / H.264 bt709 | 同上 |

三条都是 1080 高、画幅宽度不同（1.59 / 1.70 / 1.54）。原声实测为全零，因此成片**不含原声**，
BGM 就是唯一声音，不需要做闪避（ducking）。

## 产物（`build/`）

```
build/openmuse-post4-multiplatform-1920x1080.mp4   1920×1080  42.000s  30fps  1260 帧  29.5MB
build/bgm/tokyo-dreams-bed.wav                     70s / −24 LUFS  BGM bed（本地生成）
build/covers/cover-panels.png                      1920×1080  封面·三端实录版（主推）
build/covers/cover-panels-vertical.png             1080×1920  封面·三端实录版竖版
build/covers/cover-text.png / -vertical.png        封面·纯文字版（备选方向）
build/check/                                       自检证据（抽帧、转场、日志）
```

## 跑法

```bash
cd post4
./make_post4_bgm.sh            # ① 合成 BGM bed（只需一次）
./build_post4.sh               # ② 拼接 + 转场 + 配乐
python3 make_post4_covers.py   # ③ 出封面（依赖 ② 的成片抽帧）
./verify_post4.sh              # ④ 自检，结论写入 build/check/verify.log
```

四步互相独立，可单独重跑；`build_post4.sh` 依赖 ① 的产物，`make_post4_covers.py` 依赖 ② 的成片，
缺了会直接报错退出。

## 时间线与转场

| # | 段 | 成片区间 | 转场 |
| --- | --- | --- | --- |
| 1 | Desktop | 0.000 – 19.167s | 开场 0.4s 从黑淡入 |
| — | 转场 | 19.167 – 19.767s | 交叉溶解 0.6s |
| 2 | Web | 19.767 – 31.334s | — |
| — | 转场 | 31.334 – 31.934s | 交叉溶解 0.6s |
| 3 | Mobile | 31.934 – 42.000s | 结尾 0.6s 淡出到黑 |

总长 42.000s = 19.767 + 12.767 + 10.667 − 2 × 0.6。两处转场逐帧实测 alpha 从 0.000 单调升到
1.000，步长 ≈ 0.0556 = 1/18，正好 18 帧／0.600s，是线性交叉溶解，不是硬切也不是其中一个画面
提前结束。

选溶解而不是滑动／缩放匹配：三条素材是不同客户端的界面录屏，镜头之间没有同向连续运动，
也没有共享元素可以接棒（[镜头推进与转场](../Table-VideoSOP/experience/镜头推进与转场.md) 中
「素材不具备连续性时优先干净切换」）。溶解既保持每段画面完整可读，又能让平台切换被看清。

## 画面处理

三条宽度不同，统一到 1920×1080 画布：

- 正片按**高度等比**缩放居中，不拉伸、不裁切内容；三条源片都是 1080 高，所以正片不发生放大，
  也不掉清晰度（成片各段内容区与源片逐帧 SSIM 0.96–0.997）。
- 两侧留白用同段画面的模糊放大填满（`gblur` + 轻微压暗），不出现黑边。Desktop 两侧各 100px、
  Web 各 44px、Mobile 各 128px。
- 填充背景压暗取 −0.06：早先试过 −0.10，会把 Desktop 深色界面的暗部压到 limited-range 黑
  （Y=16）以下，实测两侧亮度 12.4，已改回。

## 声音

音源是 macOS 自带的 **GarageBand Apple Loops**（`/Library/Audio/Apple Loops/Apple/07 Chillwave`
的「Tokyo Dreams」分轨：Bass / Pad / Guitar / Synth / Delay Guitar），与 post3 同一首曲子，
同系列跨篇听感一致。免版税、可随作品发布，不需要联网下载。

- bed：分轨按配器权重叠加 → `alimiter` 限幅 → 静态衰减到 **−24 LUFS**（实测 −24.0 LUFS，Peak −16.1 dBFS）。
- 成片：bed 抬到 ×2.5 ≈ **−16.2 LUFS-I**，真峰值 −7.9 dBFS（≤ −1 dBTP）。1.0s 淡入、3.0s 淡出。
- 三条素材原声为数字静音，成片没有原声轨，也就不需要 sidechain 闪避。

## 封面

主题「DSH 多端运行的全家桶」。沿用 post2 / post3 系列的视觉语言（白底 + 蓝 `#2563EB` 强调 +
STHeiti 真实字体度量折行），出两个方向：

| 文件 | 画幅 | 方向 |
| --- | --- | --- |
| `cover-panels.png` | 1920×1080 | **三端实录版（主推）**：标题下方并排三条**成片里真实抽出的画面** |
| `cover-panels-vertical.png` | 1080×1920 | 同上竖版 |
| `cover-text.png` / `-vertical.png` | 1920×1080 / 1080×1920 | 纯文字版（沿用 post3 封面构图，备选） |

主推版的取舍：主题本身就是「多端」，纯文字说不清三端长什么样，所以三条缩略图直接来自成片
（Desktop t=5s、Web t=25s、Mobile t=37s，各取内容区裁切，与 `build_post4.sh` 的画布处理同一套
坐标）。三条缩略图等高不等宽，和成片里「三端同高、两侧模糊填充」的处理一致，**不裁切、不拉伸**，
是真实画面而非示意图。成片里 Web 端是很亮的白色界面，白底上光靠圆角立不住卡片，所以补了柔和投影
和略深的描边。

文案（可用 `--kicker` / `--title` / `--sub` / `--desc` / `--chip` 覆盖后重出）：

- kicker：OpenMuse × DSH · 多端实录
- 标题：**DSH 多端运行的全家桶**（用户给定，未改）
- 副标题：一套 DSH，桌面 / 浏览器 / 手机三端跑通
- chip：桌面 → 浏览器 → 手机 · 42 秒成片
- 出处：github.com/openmuseai/openmuse

```bash
python3 make_post4_covers.py                    # 两个方向 × 横竖两式
python3 make_post4_covers.py --style panels     # 只出三端实录版
python3 make_post4_covers.py --title "…" --sub "…"
python3 make_post4_covers.py --preview          # 附带字符预览与版式自检
```

`make_post4_covers.py` 每版都跑一次程序化自检：文字对比度（最低 4.66:1）、文本块是否越出版心、
文本块之间是否重叠、字符是否都在字体内（STHeiti 覆盖了本版全部字符，含 `→` `·` `×`）。
文案改长时会触发版式守卫直接报错退出，不会悄悄压住别的元素。



## 可调参数（`build_post4.sh` 顶部）

| 变量 | 默认 | 说明 |
| --- | --- | --- |
| `XFADE_DUR` | 0.6 | 段间交叉溶解时长（秒） |
| `LEAD_FADE_IN` / `TAIL_FADE_OUT` | 0.4 / 0.6 | 开场淡入、结尾淡出 |
| `BGM_VOL` | 2.5 | BGM 电平（bed 为 −24 LUFS，×2.5 ≈ −16 LUFS-I） |
| `BGM_FADE_IN` / `BGM_FADE_OUT` | 1.0 / 3.0 | 音乐淡入、淡出 |
| `BLUR_SIGMA` / `BG_BRIGHTNESS` | 24 / −0.06 | 两侧填充的模糊强度与压暗量 |
| `CANVAS_W` / `CANVAS_H` / `FPS` | 1920 / 1080 / 30 | 画布与帧率 |
| `CLIPS` | Desktop, Web, Mobile | 拼接顺序即数组顺序 |

换音乐只需改 `make_post4_bgm.sh` 里的 `FAMILY` / `LAYERS`，或直接把
`build/bgm/tokyo-dreams-bed.wav` 换成自己的音轨（44.1/48 kHz 立体声即可，`build_post4.sh`
按固定文件名读取）。

## 自检结果（`./verify_post4.sh`，结论见 `build/check/verify.log`）

| 检查项 | 方法 | 实测 | 结论 |
| --- | --- | --- | --- |
| 段序与几何 | 成片各段内容区 vs 对应源片窗口 SSIM | Desktop 0.9957 / Web 0.9969 / Mobile 0.9603；负向对照 0.4605 | 通过 |
| 转场是否真溶解 | 逐帧 alpha 拟合 | 两处均 0.000→1.000 单调，步长 ≈ 1/18，恰 0.600s | 通过 |
| 首尾淡入淡出 | 逐帧 YAVG | 开场 0.4s 由黑（Y=16）线性升到画面亮度；结尾 0.6s 反向 | 通过 |
| 意外黑帧 | `blackdetect` ≥0.4s | 无 | 通过 |
| 两侧填充非黑边 | 左右 60px 条带 YAVG | 25–193（limited-range 黑为 16） | 通过 |
| 完整解码 | `ffmpeg -f null -` | 无错误，解码 1260 帧 | 通过 |
| 响度与真峰值 | `ebur128` | −16.2 LUFS-I / −7.9 dBFS | 通过 |
| 音乐淡入淡出 | `astats` 分段 RMS | 首 0.2s −35.2 dB、中段 −17.5 dB、末 0.2s −46.5 dB | 通过 |
| 1× 完整审看／审听 | — | 本 Agent 无法以正常速度观看或收听 | 未验证 |
| 字幕 | — | 本期无字幕 | 不适用 |

抽帧证据在 `build/check/`：`layout.jpg`（三段常态构图）、`transitions.jpg`（两处转场各三帧）、
`fades.jpg`（首尾淡入淡出）。

## 已知边界

- **Web / Mobile 段比标称早 1 帧（0.033s）**：`xfade` 取帧造成，实测由源片局部 0.800s / 0.634s
  对齐最好（SSIM 0.9969 / 0.9603）。33ms 在无对白的界面录屏上不可感知，故不修正。
- 转场中点 alpha 用相邻帧差反推时前段波动较大（Desktop 尾、Web 尾有鼠标或画面运动），
  这是素材运动，不是溶解非线性；alpha 步长本身恒定。
- 只出一条横版成片。需要抖音竖版时可加一路 1080×1920 模糊背景版（post3 的
  `build_post3.sh` 竖版分支可直接套用）。
- 未加平台标签、字幕与封面卡（本期未要求）。要区分三段客户端，可在每段入场处加一个小角标。
- 素材原声为静音；若后续补录解说，需把音频改成「BGM + 人声 + sidechain 闪避」。
- **封面文案是草稿**：标题「DSH 多端运行的全家桶」按用户给定未改，kicker / 副标题 / chip /
  正文是可覆盖的占位文案，改完重跑 `make_post4_covers.py` 即可。
- **封面观感未验证**：本 Agent 无法看图，只做了程序化自检（对比度、版心、重叠、字体覆盖、
  缩略图与成片逐像素一致）。构图是否好看、三条缩略图选帧是否最有代表性，需要你看图后定。
  换抽帧时间点改 `SEGMENTS` 里的秒数，或先删 `build/covers/frames/` 再跑。

## 发布记录（抖音）

用 Easel 插件发布，落款与时序如下。

| 项目 | 值 |
| --- | --- |
| 平台 / 入口 | 抖音创作者中心 creator.douyin.com（Easel `douyin/publish-video`） |
| 账号 | OpenMuse |
| 作品 id | `7694681964212686122`（插件读回对账通过） |
| 发布时间 | 2026-10-09 22:55 CST |
| 标题 | DSH 多端运行的全家桶 |
| 简介 | 一套 DSH，桌面 / 浏览器 / 手机三端跑通。 |
| 话题 | #DeepSeek #Codex #DSH |
| 发布文件 | `build/easel/openmuse-post4-with-cover.mp4`（44.000s，sha256 前 16 位 `9ba2610dbeb6aa3c`） |

发布用的不是 `build/` 里的 42s 主成片，而是**带封面片头的 44s 版**：插件用
`douyin/publish-video` 时**没有单独传封面的参数**，它的封面能力是「上传前选 AI 封面 /
抖音默认取首帧」。所以「带上封面」的做法是在正片前加 2 秒封面卡
（`easel video process` 的原生做法：`[封面]2s + [正片]`），首帧就是封面，平台的自动封面
也就落在这一帧上。封面卡用的是 `build/covers/cover-panels.png`。

```bash
# ① 造带封面片头的发布版（用插件自己的实现；直接调 openmuse 会因插件取不到 ffprobe 失败）
P="$HOME/Library/Application Support/com.openmuseai.office/OpenMuse/Workspace/plugins/com.openmuse.easel"
PATH="/opt/homebrew/bin:$PATH" "$P/runtime/venv/bin/python" "$P/skills/runtime/easel_cli.py" process \
  --input  build/openmuse-post4-multiplatform-1920x1080.mp4 \
  --output build/easel/openmuse-post4-with-cover.mp4 \
  --cover  build/covers/cover-panels.png

# ② 离线预览（不发布）
PATH="/opt/homebrew/bin:$PATH" openmuse easel douyin plan \
  --title "DSH 多端运行的全家桶" --video build/easel/openmuse-post4-with-cover.mp4 \
  --tags "#DeepSeek #Codex #DSH"

# ③ 真正发布（--exec 才会提交）
PATH="/opt/homebrew/bin:$PATH" openmuse easel douyin publish-video \
  --title "DSH 多端运行的全家桶" --content "一套 DSH，桌面 / 浏览器 / 手机三端跑通。" \
  --video build/easel/openmuse-post4-with-cover.mp4 --tags "#DeepSeek #Codex #DSH" --exec
```

两处与原始指令不同，已确认过的取舍：

- **话题拼写**：指令里写的是 `#DeekSeek`，按官方写法改用 **`#DeepSeek`**（`DeekSeek` 是
  DeepSeek 的字母换位）。若确实要按原拼写发，改 `--tags` 重发即可。
- **时长 44s 而非 42s**：封面卡占 2s。抖音不带独立封面参数，只有加片头这一条路。

**已知环境问题**：`openmuse easel video process` 在本机会失败——插件 `easel_cli.py` 直接调用
裸命令 `ffprobe` / `ffmpeg`，而 `openmuse` 传给插件的 PATH 里没有 `/opt/homebrew/bin`
（`runtime/install-status.json` 里记的绝对路径没有被使用）。上面的 ① 是用插件自己的 venv
解释器直接跑它的实现绕开的，未改动系统 PATH，也未新增符号链接。



## 素材来源与授权

| 用途 | 素材 | 来源 | 授权 |
| --- | --- | --- | --- |
| 画面 | `Desktop.mp4` / `Web.mp4` / `Mobile.mp4` | 本项目自行录制 | 自有 |
| BGM | Tokyo Dreams（Bass / Pad / Guitar / Synth / Delay Guitar） | macOS 自带 GarageBand Apple Loops：`/Library/Audio/Apple Loops/Apple/07 Chillwave` | 免版税，可随作品发布；bed 由脚本在本机生成，不随本仓库再分发 |
