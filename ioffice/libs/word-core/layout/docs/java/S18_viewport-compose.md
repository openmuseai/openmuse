# S18 — 视口合成（滚动 / 缩放 / 页间隙）

## 1. 职责

Flutter 视口把已光栅化的页组合成最终屏幕：页竖向排列（页间隙）、滚动 offset、可见页范围（S13 cp→页）、重绘当前可见页。

管线位置：上游 S16/S17；下游 S19（交互回环）。

## 2. 核心实现

```dart
// 页高度表来自 S11（每页 height_twip）
visiblePages = pagesInRange(scrollOffset, viewportHeight);   // cp→页 / 页码表
for (page in visiblePages) {
  canvas.drawPicture(page.picture, offset(pageIndex * (pageH + gap) - scrollOffset));
}
```

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| 页高度表 | S11 每页 `height_twip` → px |
| 页间隙 | 页与页之间固定间距（如 16px） |
| 可见页 | 滚动 offset + 视口高 → 页子集（S13 定位辅助） |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | 手势/滚动 offset + 页高度表（S11）+ DPR |
| **O** | 屏幕最终像素（可见页子集合成） |

## 5. 上下游

- **上游**：S16/S17 页光栅化、S13 定位。
- **下游**：S19 交互（点击/滚动反馈）。

## 6. 边界 / 失败语义 / 不变量

- MVP 先固定第一页；滚动/缩放属 wr-08-03。
- 可见页 min(cp 范围) 决定重绘子集，不重绘不可见页。

## 7. Rust 承接与现状

wr-08 `03_interaction`（MVP 固定第一页）—— **✗ 待 loop**。