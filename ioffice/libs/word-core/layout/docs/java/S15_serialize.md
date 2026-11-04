# S15 — 指令序列化 + 跨端传输

## 1. 职责

把 S14 的 RenderCommand 序列序列化成 JSON（serde）与字节流，经 UniFFI / dart:ffi 传输给 Flutter。Stage3 对比文件就是本环节输出。

管线位置：上游 S14；下游 S16（Flutter 解析）。

## 2. 核心实现

| 项 | 内容 |
|----|------|
| 序列化 | serde_json（word-core 已依赖）→ JSON/二进制 |
| 传输 | UniFFI（`cfg(feature = uniffi)`）或 dart:ffi（RUST_REWRITE_RULE 模式） |
| 版本 | 指令流带 `version` 字段，Flutter 侧按版本解析 |

伪结构：

```text
RenderCommandJSON = {
  "version": 1,
  "commands": [
    { "type": "BeginPage", "page": 0, "width_twip": 11906, "height_twip": 16838 },
    { "type": "FillRect", "x": 0, "y": 0, "w": 11906, "h": 16838, "color": -1 },
    { "type": "DrawText", "page": 0, "font_id": 3, "size_twip": 210,
      "color": -16777216, "x": 1800, "y": 1440, "text": "H",
      "xpos": [1800, 1905] },
    { "type": "EndPage", "page": 0 }
  ]
}
```

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| `RenderCommand` enum | S14 指令集（Save/SetClip/Translate/FillRect/StrokeRect/DrawLine/DrawText/BeginPage/EndPage） |
| JSON 流 | `version` + 指令数组；字段名与 Java `InstractionBuilder` 语义对应 |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | RenderCommand 序列（S14） |
| **O** | JSON 字节流（含 version 字段）；Flutter 侧反序列化 |

## 5. 上下游

- **上游**：S14。
- **下游**：S16 Flutter 解析绘制。

## 6. 边界 / 失败语义 / 不变量

- 坐标单位维持 **twip**（px 换算在 S16）。
- 字段名尽量对齐 Java 语义，避免二次发明。
- JSON 体积/性能非本阶段目标（指令化先对齐语义）。

## 7. Rust 承接与现状

wr-07 `03_serialize`（serde JSON）+ wr-08 `01_uniffi_export`（`cfg(feature = uniffi)`）—— **✗ 待 loop**。S15 输出即 Stage3 对比文件。