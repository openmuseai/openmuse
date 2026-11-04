# wr-15-style-commands

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（`01`–`03` 均 pass）
> 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)

## 目标

从 Snapshot **按行** 发出带 **标题字号 / 加粗 / 字体 id** 的 `RenderCommand`，样例能看出样式。本 feature **不是** 行级门闩（门闩在 wr-16）。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | wr-14 `TypoSnapshot` + FontHost 的 `font_id` 映射。 |
| **O** | `RenderCommand[]`：`BeginPage/EndPage`、页背景 `FillRect`、每行（或每 run）`DrawText`（`font_id`, `size_twip`, `bold` 或等价 face, `color`, 文本, 行框原点）。下划线走现有 `emit_deco`。 |

`DrawText.y` 仍为基线（`LAYOUT_FRAME_VERSION = 2`）。`xpos[]` 可填，**CI 不比 xpos**。

## Java 对照

`TypoTextRender` + `IRenderDrawer.drawText`；不移植 `RenderDrawer`/Paint。

## 必须 / 壳 / 延后

- **must：** 不同 Heading 字号出现在指令里；bold run 与非 bold 分开；eastAsia/ascii 的 `font_id` 与度量同源。
- **stub：** 高亮底纹、段落边框。
- **defer：** 以完整 Stage3 序列（含 xpos）对齐中文标题文档。

## 范围内 / 范围外

- **in：** 纯文本样式绘制。
- **out：** 用 xpos 当本波 pass 条件；表/图指令；Flutter 测宽。

## 验收

目视/单测：Heading 的 `size_twip` > 正文；bold 标志或 bold `font_id` 出现。行级 cp 仍只由 wr-16 判。

## 依赖

- **depends：** wr-14；wr-07 指令 enum。
- **does not depend：** wr-17。

## 不过扩展

禁止为「像素更好看」改 wr-12 断点。P4 拉丁 Stage3 金文件保持回归，不把中文标题强行塞进 xpos 序列相等。
