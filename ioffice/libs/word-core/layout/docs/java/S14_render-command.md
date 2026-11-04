# S14 — 渲染指令化：TypoSnapshot → RenderCommand（twip）

## 1. 职责

把 TypoSnapshot 遍历成绘制指令（twip 坐标）：页背景/底纹（FillRect）、行文本（DrawText，逐字 xpos）、下划线/边框（DrawLine/DrawRect，DecoLineCache 语义）。这是 Rust 内核的输出物，Flutter 只消费指令。

管线位置：上游 S12；下游 S15（序列化）。

## 2. 核心 Java 代码

| 类 | 职责 |
|----|------|
| `LayoutRender.renderDocument(Canvas, clip, zoom)` | 顶层渲染入口（snapshot → 页 → 行） |
| `CanvasController` | `setCanvas` + `scale(twip→px)`；save/clip/translate |
| `PageRender.drawPage` | 页背景 + 行 |
| `TypoTextRender.drawLine` | 行 → drawText |
| `IRenderDrawer` | 绘制原语接口（drawText/drawLine/drawRect/…） |
| `InstractionBuilder` | 录制原语成可回放指令块（**wr-07 语义源**） |

坐标换算（钉死）：

```text
canvas.scale(zoom × MetricsUtil.twips2pixel_x_ratio)
twips2pixel_x_ratio = 1.3333536 / 20   // 96 DPI 约定，不是 density
```

InstractionBuilder 已录制原语（MVP 最小集）：**drawText / drawLine / drawRect / drawColor / fillRect**；其余（roundRect/circle/path/picture/bitmap/shape/wordArt）defer。

## 3. 数据模型

| 指令 | 内容 |
|------|------|
| `Save / Restore` | 画布状态栈 |
| `SetClip { x, y, w, h }` | 裁剪 |
| `Translate { dx, dy }` | 平移 |
| `FillRect { x, y, w, h, color }` | 页背景/底纹 |
| `DrawLine { x1, y1, x2, y2, color, width }` | 下划线/边框 |
| `DrawText { page, font_id, size_twip, color, x, y, text, fake_bold, skew_x, xpos[] }` | 行文本（逐字 xpos） |
| `BeginPage/EndPage { index, width, height }` | 页边界 |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | TypoSnapshot（S12）+ 视口矩形 + zoom |
| **O** | 绘制指令（twip 坐标）：`BeginPage/EndPage`、`FillRect`、`DrawText`、`DrawLine/DrawRect` |

## 5. 上下游

- **上游**：S12 TypoSnapshot、S07 zoom。
- **下游**：S15 序列化 + 传输。

## 6. 边界 / 失败语义 / 不变量

- **逐字 xpos 用于 Stage3 对齐**；整串 `TextPainter` 二次排版 = 二次断行，禁止。
- 指令坐标一律 twip；px 换算留在 S16（Flutter 侧）。
- 验收：指令序列 + 字段与 Java `InstractionBuilder` 回放相等。

## 7. Rust 承接与现状

wr-07：`01_text_command` + `02_deco_command`（rendercommand/，twip 指令 enum 见 JAVA_RUST_PIPELINE §3）—— **✗ 待 loop**。InstractionBuilder 为语义源（不移植 int-block 分配器）。