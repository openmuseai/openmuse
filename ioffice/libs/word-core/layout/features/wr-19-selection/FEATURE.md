# wr-19-selection

> status: **pass**（`01`–`03`；见 [FUNCTIONS.md](./FUNCTIONS.md)）  
> 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md) §9

## 目标

在已排好的 **`LayoutFrame`（`DrawText.xpos`）** 上，把半开 cp 区间画成选区 **`DrawRect`**（`kind = selection`）。不是编辑，不是 caret 竖线，不是 IME 合成区。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | `LayoutFrame` + `[start, end)`（cp，与可见 `DrawText.text` 标量序；调用方负责屏幕 zoom）。 |
| **O** | `RenderCommand::Rect`：`kind = "selection"`；几何由 xpos 切片 + 行带 `y = baseline − size`、`h = size`。 |

空区间 → 不发 Rect。禁止在 Frame 上改 `DrawText.text`。

## Java 对照

`LineSelecitonRect.drawSelection`：`setSelection(left, lineRect.top, right, lineRect.bottom)`。本波 **不** java2rust 整类（含 `LocateTool.getCharPos`）；用 wr-18 同一套 xpos。

## 必须 / 壳 / 延后

- **must：** 夹紧/对调区间；按 run 求交；发出 selection Rect。
- **stub：** 无。
- **defer：** bidi/subline；TTS/composing；caret 竖条；拖拽；`LineSelecitonRect` 整文件。

## 范围内 / 范围外

- **in：** 产品 Frame 上的反色条。
- **out：** 改 rope；Flutter `TextPainter`；选区手柄。

## 验收

Hello xpos 金：`[1,3)` → 一条 Rect，`x=1900`，`w=200`。

## 依赖

- **depends：** wr-07 `DrawRect`；wr-18 Frame/xpos。
- **does not depend：** wr-20 编辑重排。

## 不过扩展

禁止为选区改 wr-12 断行。需要 `LineSelecitonRect` 时走 java2rust，禁止从空文件手写 300+ 行。
