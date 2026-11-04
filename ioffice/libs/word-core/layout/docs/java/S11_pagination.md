# S11 — 分页：页满判定 / 分页符 / 节切换 → 页序列

## 1. 职责

把 S10 的行流按页内容高折成页序列：页满判定、`\f` 分页符、节切换（PageSetup 尺寸变化），产出 `TypoLayoutPage`（含行容器），断页行标 `LASTLINEOFPAGE`。

管线位置：上游 S10；下游 S12（快照）。

## 2. 核心 Java 代码

| 类 | 职责 |
|----|------|
| `TextLayouter.processPagination` | 页满判定（行高累加 > 内容高 → 新页）；`FINISHPAGE` 信号 |
| `MainLayouter.flowPage` | 驱动单页排版（行放入页容器） |
| `PageViewLayouter` | 打印页模式（`MODE_PAGE=0`） |
| `SectionLayouter` | `PLCSection` 节序列 → 各节 PageSetup（页尺寸） |
| `TypoLayoutPage` | 主文档页：行容器、页序 mPage、cp 区间 |

分页信号（TypoTextLine flags）：

```text
LASTLINEOFPAGE   — 该行是页内最后一行（页满/分页符）
LASTLINEOFSECTION— 节末
LASTLINEOFCOLUMN — 栏末（MVP 无栏）
```

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| `TypoLayoutPage` | 页：行容器（`[start_cp, end_cp)`）、页序 mPage、margins |
| `TypoTextLine.flags` | FIRST/LAST of para/page/section |
| 节序列 | `PLCSection` 各 `SectionNode`（PageSetup：页宽高/边距） |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | 行流（S10）+ 页内容高（S07）+ PLCSection 节序列 + `\f` 分页符 |
| **O** | 页序列：`TypoLayoutPage`（行容器、页序、cp 区间）；断页标 `LASTLINEOFPAGE`；节切换换页尺寸 |

## 5. 上下游

- **上游**：S10 行、S07 页高、S05 `\f`/PLCSection 锚点。
- **下游**：S12 快照（页序列入库）、S13 定位（cp→页）。

## 6. 边界 / 失败语义 / 不变量

- MVP **单节连续分页**；寡妇孤儿 / 跨节重排 **延后**。
- 页满判定：行 top + 行高 > 页内容底 → 新页（首行必须可放下，否则强制换行）。
- `\f` 分页符强制断页（parse 已记 `mCpLastSectionEnd`）。

## 7. Rust 承接与现状

wr-05：`01_page_break` + `02_section_grid`（layout/base/）—— **✗ 待 loop**。断页标 `LASTLINEOFPAGE`；单节 MVP。