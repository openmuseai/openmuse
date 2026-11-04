# S13 — 定位 / 命中（字符 ↔ 页 / 行 / run）

## 1. 职责

从 cp（或视觉坐标）反查视觉位置：偏移 → para/line/行内偏移；cp → 页（`IPageGrid.pageIndexFromChar`）。供选区/光标/高亮绘制与滚动定位输入。

管线位置：上游 S12（TypoSnapshot）；下游 S18/S19（视口与交互）。

## 2. 核心 Java 代码

| 类 | 职责 |
|----|------|
| `LocateTool` | 字符偏移 → 页/行/行内偏移的定位工具 |
| `LayoutLocater` | 布局命中：坐标 → cp |
| `getTextLayoutPos` | 偏移 → para/line/行内偏移 |
| `IPageGrid.pageIndexFromChar(cp)` | cp → 页下标（二分） |

典型调用：

```java
// 命中测试：屏幕坐标 → cp
int cp = LayoutLocater.getCharIndexAt(visualPoint, snapshot);
// 反向：cp → 页/行/行内 x
TextLayoutPos pos = LocateTool.getTextLayoutPos(snapshot, cp, ...);
```

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| `TextLayoutPos` | 定位结果：para / line / 行内偏移 / 行内 x |
| cp→页 | `IPageGrid.pageIndexFromChar(cp)`（页列表二分） |
| 页→行 | `TypoPageLinesIterator`（页内行迭代）+ `TypoRangeBase.contains(cp)` |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | cp + TypoSnapshot（S12） |
| **O** | 视觉位置（页/行/run/x）——选区/光标/高亮绘制输入；滚动定位输入 |

## 5. 上下游

- **上游**：S12 TypoSnapshot。
- **下游**：S18 滚动定位、S19 交互（点击/选中）。

## 6. 边界 / 失败语义 / 不变量

- mv 只定义接口；交互（wr-08-03）延后。
- 半开区间 `contains(cp)` 保证边界唯一归属（`cp<end`）。
- cp 越界 → 返回 None / 钳制到首尾。

## 7. Rust 承接与现状

wr-01 `04_typo_iterator`（页→行迭代 + contains）+ wr-05 `02_page_grid`（cp→页二分）—— **✗ 接口定义；交互 wr-08-03 延后**。