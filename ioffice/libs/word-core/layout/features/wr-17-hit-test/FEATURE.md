# wr-17-hit-test

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（`01`–`03` 均 pass）
> 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md) §9

## 目标

在 **已经行级对齐** 的 Snapshot 上做命中：点 → cp。这是光标的数据基础，不是选区绘制，不是编辑。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | wr-14 `TypoSnapshot` + 页坐标 `(x, y)` twip（已是排版空间，调用方负责把屏幕点经 zoom 反变换）。 |
| **O** | `(page_index, line_index, cp)`；点在行外时的夹逼规则与 Java `LayoutLocater` / `getTextLayoutPos` 最小集一致（点落在行框内：按 advance 累加找 cp；行级对齐后行内像素差允许 cp 在相邻字之间差 1——**须在实现时用 Java 单测钉死，不得在本波提前发明**）。 |

## Java 对照

S13：`LocateTool` / `LayoutLocater`。不接 `ITextView`。

## 必须 / 壳 / 延后

- **must：**（实施时）页内行框命中 + 行内按 wr-11 advance 找 cp。
- **stub：** 选区 `FillRect`、拖拽。
- **defer：** 编辑插入、CoreThread、IME。

## 范围内 / 范围外

- **in：** 只读命中。
- **out：** 在 wr-12 未绿时用 LayoutFrame 贪心行做光标；选区反色；改字。

## 验收

（实施时）同一 Snapshot：给定 twip 点，cp 与 Java 一致（容差策略届时按 Java 测试钉死）。

## 依赖

- **depends：** wr-14、wr-16 已绿（否则命中的「行」不是 Java 的行）。
- **does not depend：** 编辑存储。

## 不过扩展

本目录存在只为锁边界：wr-12/13 **不得** 顺便做 hit-test。会话/光标见 [wr-18-session-caret](../wr-18-session-caret/FEATURE.md)；选区 `FillRect` 为 wr-19。
