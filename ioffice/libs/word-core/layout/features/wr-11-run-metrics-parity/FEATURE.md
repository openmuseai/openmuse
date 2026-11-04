# wr-11-run-metrics-parity

> status: **pass**  
> 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)

## 目标

在 **同一 TTF 字节** 上，按 wr-10 的样式给出与 Java FreeType 路径可对的 **逐字符 advance 和行格**。断行只许用这里的宽度，不许再用「均分 run 宽」或 `width=size`。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | wr-09 的一段 `text` + wr-10 的 `TypoFontStyle` + `FontHost` 已注入的 face 字节（按族名解析，与 wr-03 同一套，**layout 不探盘**）。 |
| **O** | `RunMetrics`：`advance[cp]`（twip）、`width` 总和、`ascent`/`descent`/`line_gap`（twip）。缺 cmap 时 **失败**（与现 FontStack 一致），不得 silently 用字号冒充宽度。 |

换算沿用蓝皮书 / 现 FontStack（**size 为 pt**）：`advance_twip = round(hmtx × size_pt × 20 / upem)`。
若 size 已是 twip：`round(hmtx × size_twip / upem)`。不要再对 `size_twip` 乘一次 20。

## Java 对照

S08：`ITypeface` / FreeTypeJNI `getCharWidth` + hhea/OS2。Rust 继续 ttf-parser + FontHost，不接 Android Paint。

## 必须 / 壳 / 延后

- **must：** 按 style 选 ascii/eastAsia face；bold/italic 用 **同一文件的 style 轴或已注入的 bold face**（本波若只有 regular TTF，bold 用合成标志给绘制，**advance 仍按该 TTF hmtx**——须在金文件里与 Java 同一字体文件约定写死）。
- **stub：** 合成粗体对宽度的偏移（若 Java 用 fake bold 加宽，列出 paint-gap，**不得悄悄改断行**）。
- **defer：** Paint sticky CJK 回退；kerning；ligature；rustybuzz shaping。

## 范围内 / 范围外

- **in：** 纯文本 BMP 字符；中文须注入含 cmap 的 TTF。
- **out：** 断行算法；假宽度；系统字体名碰运气；为过测试写死 `Hello` 宽度。

## 验收

与 Java 同一 TTF、同一 size：逐字 advance 差 ≤ 0.5 twip（S08 口径）。本 feature **不** 断言换行点。

## 依赖

- **depends：** wr-03 FontHost/FontStack；wr-10 样式。
- **does not depend：** wr-12 LineLayouter。

## 不过扩展

禁止在本 feature 实现 wrap。禁止 `FontStack::default()` 探盘。
