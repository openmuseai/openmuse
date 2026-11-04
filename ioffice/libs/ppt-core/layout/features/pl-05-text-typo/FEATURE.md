# pl-05-text-typo — 框内贪心断行

> 层：layout ｜ 状态：**pass** ｜ Java：`RichTextLayout.typo`（**对照锚点，非对齐目标**）+ `BaseTextLayout.setTextRect`（文本区几何，02 函数精确对齐）

> **一致性声明（钉死）：** MVP 贪心断行 **≠** Java `RichTextLayout`（wpslayout）。Rust 侧 `core_libs_wpslayout` 的 `RichTextLayout.rs`（5346 行）仍为未实现模板（计划期审计记 178 文件 / 3352 个 `todo!`；现况 23 个文件留有 `TODO_RUST_CODE` 未填）。**Java 一致的断行另开 feature**，本 feature 禁止冒充。度量口径对齐 writer：与 wr-04 `Wrap.rs` 同源贪心思想，但不改 writer 代码。

## 目标

在 pl-04 的 `TextBoxData` 上做**可画**的框内断行：给定框宽高，把一个或多个段落切成「行列表」（每行 `{text, startCp, endCp, x, y, w, h}`），使 pr-03 能直接画字。MVP 用贪心断行绕开 wpslayout 引擎（对齐 `PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S2 风险 1 与 §3.3），先画、后对齐。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | `TextBoxData`（pl-04：段文本 + `SpanInfo` + `ParaInfo` + insets/anchor/vertical/wrap）+ 框尺寸（`boxW × boxH`，layout 单位）+ 度量（FontHost 注入表：每 span `fontSize/字体` → advance 宽，对齐 writer wr-03 口径） |
| **O** | 行列表 `[{text, startCp, endCp, x, y, w, h}]`（layout 单位；`x/y` 已含 insets + 垂直 anchor 效果）+ 文本区内缩矩形（02 的产物，供 01 作可用宽） |
| 单位 | layout 单位全链路；度量宽可由 fixture 注入（**禁止用假宽度假装断行正确**，`features/README.md` 红线） |

断行语义（MVP，钉死）：按 CMS（中日韩）单字与拉丁词为单位贪心累计，超可用宽则折行；单 token 超宽强制折；`\r` 切段；行高 = 该行最大字号（MVP 简化，Java 精确行高另开）。段落内 SpanInfo 只参与度量，不断 span。

## Java 管线对照

S2 断点③（`PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S2）：Java 在 pl-04 之后调 `new ShowRichTextLayout().typo(boxData, w, h, forceTypo)`（`ShowTextLayout.java:40` 的 layout 入口，typo 调用在 `:56`）。本 feature 只做**可对齐到 Java 语义子集**的部分：

- **断行（01，对照非对齐）：** `RichTextLayout.typo`（`RichTextLayout.java:78`）→ `BaseTextLayout.typo`（→ `processBeforeTypo`：`\r` 段切分 / trailing space / bidi 探测）→ `doTypo` → `typoOnePara`（`RichTextLayout.java:528`）→ Android `Layout`（`mTypoCaches`，行几何可经 `getLineStart/End/Width/Height` 读）。**MVP 只对齐「行 = 放得下的字符区间 + 行高/行宽」这一层语义**；`TextPaint`/`Layout`/`TypoLayoutGen` 不移植。Java 一致版（逐位对齐断点、kinsoku、标点挤压等）另开 feature。
- **文本区（02，可精确对齐）：** `BaseTextLayout.setTextRect(textBoxData, w, h)`（`BaseTextLayout.java:88`）——`left+leftInset`、`right-rightInset`、`top+topInset`、`bottom-bottomInset`，越界时报 **atten 均衡**（`if (left > width) { atten=(left-width)/2; left-=atten; right-=atten }` 等一组公式，`BaseTextLayout.java:110-137`）。**这段公式必须逐行对齐 Java**。
- **绘制侧参考：** `ShapeTextRender._render`（`ShapeTextRender.java:28`）取 `renderModel.getTextRect(w,h)`（`Text.getTextRect` → `Shape.getTextRect`（`Shape.java:948`）→ `DataCal.getTextRect`（`DataCal.java:55`，预置几何取 `pathData.getTextBoxRect()`，普通形状 `(0,0,w,h)`）。MVP 先按普通形状处理（文本框 inset 由 02 承载）；预置几何 inset 待 pl-02/pr-02 确认。

## 能力解锁

| function | Java 对照 | 实现后管线新增 |
|---|---|---|
| `01_function_greedy_wrap` | `RichTextLayout.typo` 断点语义（贪心子集） | 框内行几何（cp 区间 + 行高 + 行首 x）→ pr-03 `DrawText` 有原料 |
| `02_function_text_box_geometry` | `BaseTextLayout.setTextRect` + `renderModel.getTextRect` | 文本区内缩矩形 → 断行可用宽与画的 x/y 正确 |

缺失断点：无 01 → 文本只存在于 `TextBoxData` 无行（pr-03 空转）；无 02 → 行画在框物理边界（insets 未扣）、首行顶到框沿。

## 必须 / 壳 / 延后

- **must（01）：** 贪心断行（CJK 单字 / 拉丁词 / 超宽强制折）、`\r` 段切分、行 `w = 行内最大 adv 累计`、行 `h = 该行最大字号`（MVP）、行 `x` 左对齐 + `y` 从上沿 `topInset` 计、输出含 `startCp/endCp`。
- **must（02）：** `setTextRect` 公式逐行对齐 Java（含 atten 均衡分支），输入 `boxW/boxH + TextBoxData insets` → 输出内缩矩形。
- **stub：** 垂直 anchor 只落 `t/b/ctr` 三态的 `y` 平移；两端对齐、tab；wrap=false 时单行长。
- **defer：** Java `RichTextLayout` 断行一致（另开 feature）、竖排/warp/autofit 连锁、`\r` 之外的软换行（`\u000b`）、kinsoku/标点挤压、bidi 重排、`textAutofit` 缩放。

## 范围

- **in：** 非竖排文本框的普通正文（含中文/英文混排）；ph 文本框。
- **out：** bullet 复杂、prompt 文本、表格 cell 文本、autofit 缩放、Java 精确断行。

## 验收

fixture `TextBoxData` × 固定宽度（宽度来自 02 或显式 fixture）：01 的行 cp 区间与「按可用宽手算」的期望逐行相等；02 对 5 组 inset 组合的输出与 Java `setTextRect`（含 atten 分支）±0.01px 一致；最终 `{x, y}` 吞掉 insets+anchor。全部通过才可标 **pass**（现为 **spec**）。

## 依赖

- **upstream：** pl-04（`TextBoxData`）；pl-01（单位）；度量表 FontHost（writer wr-03 注入口径，本波 fixture 化）。
- **downstream：** pl-06（可选挂行几何）；pr-03（`DrawText`）。
- **不依赖：** wpslayout 排版引擎、Flutter。

## 不过扩展

禁止以任何形式声称本 feature 与 Java `RichTextLayout` 断行一致；禁止改 writer `libs/word-core` 的 wr-04 `Wrap.rs` 冒充；禁止把竖排/autofit「顺便做」进本 feature；禁止在 01 里注入假的「每行等宽」冒充度量。