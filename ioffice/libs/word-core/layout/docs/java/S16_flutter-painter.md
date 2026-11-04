# S16 — Flutter 解析指令 → Canvas 绘制（twip → px）

## 1. 职责

Flutter 侧把 S15 的 JSON 反序列化后，用 `Canvas`/`TextPainter` 执行指令：`scale = zoom × 1.3333536/20 × devicePixelRatio`，逐指令转成 Canvas 调用（transform → BeginPage/EndPage → fillRect / 逐字文本）。

管线位置：上游 S15；下游 S17（引擎光栅化）。

## 2. 核心实现

```dart
// 96 DPI 约定（与 Java twips2pixel_x_ratio 同源，再乘 DPR）
final scale = zoom * 1.3333536 / 20 * devicePixelRatio;
canvas
  ..save()
  ..scale(scale)          // twip → 逻辑 px
  ..translate(-scrollDx, -scrollDy);
for (cmd in commands) {
  switch (cmd.type) {
    case 'BeginPage':  // 兼容：transform 已含缩放
    case 'FillRect':   canvas.drawRect(rect(cmd), paint(color: cmd.color));
    case 'DrawText':   // 逐字 xpos：对每个字画 TextPainter（或同 run 一个）
  }
}
```

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| 指令 JSON | S15 反序列化结果 |
| canvas transform | `zoom × 1.3333536/20 × DPR`（twip → 物理 px） |
| 文本 | **逐字 xpos** 绘制（每 run 一个 TextPainter，禁止整串 TextPainter 二次排版） |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | RenderCommand JSON（S15）+ 视口尺寸 / DPR |
| **O** | Canvas 调用序列：transform(scale) → 兼容 Begin/EndPage → fillRect / 文本描画 |

## 5. 上下游

- **上游**：S15 JSON。
- **下游**：S17 字形光栅化（引擎）。

## 6. 边界 / 失败语义 / 不变量

- **逐字 xpos 是 Stage3 对齐的唯一方式**；整串 `TextPainter` 会二次排版 ≠ Java 断行，仅允许装饰性差值场景。
- 内核禁止写死任何 Android density；DPR 由 Flutter 侧提供。
- 单位换算公式与 Java `twips2pixel_x_ratio = 1.3333536/20` 完全一致。

## 7. Rust 承接与现状

wr-08 `02_flutter_painter`（解析 JSON → Canvas/TextPainter）—— **✗ 待 loop**。