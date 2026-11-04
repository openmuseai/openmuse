# S17 — 字形光栅化与合成（像素点）

## 1. 职责

Flutter 引擎层把 Canvas 调用（S16）光栅化成屏幕像素：Skia/Impeller 内的 shaping（类 HarfBuzz）+ FreeType/系统字体 rasterizer → glyph 位图/路径 → 按 font_id 着色 → 图层合成。**本环节是引擎职责，Rust 无代码**，但要保证字体文件一致 + 坐标/字号与 S08 度量同源。

管线位置：上游 S16；下游 S18（视口合成）。

## 2. 核心实现（引擎内部，Rust 只保证输入）

```text
Canvas 调用序列（S16）
  → shaping：按 font_id / 字号把文本切成 glyph（类 HarfBuzz）
  → rasterize：FreeType/系统字体 → glyph 位图或路径
  → fill：按 DrawText.color 着色
  → 图层合成：layer tree → Skia picture → GPU 光栅化
```

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| glyph | 字体编码 → 轮廓/位图 |
| layer tree | 页背景/文本/装饰按 z-order 的合成树 |
| picture / raster | GPU 的绘制记录 → 物理像素 |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | Canvas 调用序列（S16）+ 字体资源（Flutter 侧须能按 font_id 找到同一字体文件） |
| **O** | **屏幕像素**：每页光栅化到图层/位图；页背景/文本/装饰按 z-order 合成 |

## 5. 上下游

- **上游**：S16 Canvas 调用。
- **下游**：S18 视口合成（滚动/缩放/页间隙）。

## 6. 边界 / 失败语义 / 不变量

- **比特级与 Android 截图不对齐**（`Paint.measureText`/`Canvas.drawText` 无法被 FreeType+Flutter 比特级替换）。
- 验收以 Stage2/3 JSON 为准（几何与指令），不验收光栅像素。
- 关键一致性：Flutter 侧 font_id → 同一字体文件；坐标/字号与 S08 度量同源。

## 7. Rust 承接与现状

**无 Rust**——引擎职责。Rust 保证：字体文件一致 + 坐标/字号与 S08 度量同源。