# wr-12-line-layouter

> status: **pass**  
> 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)  
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（`01`–`05` 均 `pass`）

## 目标

**段内分行与 Java `LineLayouter` 一致**：哪些 cp 落在哪一行、行宽、行高、对齐后的行起始 x。这是行级对齐的核心 feature。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | 一段内已度量的 run 序列（wr-11）+ `content_width`（wr-02：页宽−左右边距−本段左右缩进后的可用宽）+ 本段 PAPX：`jc`（左/中/右）、首行缩进、行距、段前段后（段前段后可只影响第一/最后一行高度，按 Java 规则）。 |
| **O** | `TypoTextLine[]`（逻辑行，尚未分页）：每行 `start_cp`, `end_cp`, `width`, `height`, `baseline`（相对段或相对行顶，与 Java 字段同义）, `x`（对齐后）, flags：`FIRST/LAST` of para。 |

**一行的 cp 区间必须与 Java 金文件精确相等。**

## Java 对照

`LineLayouter.java` + `TextLayouter.flowLine`。不在这里 `finishPage`。

## 必须 / 壳 / 延后

- **must：** Java 对纯正文的可断点（拉丁空格回退；中文无空格按 Java 同一套切分；超宽强制切；`\r` 结束段）。行高/基线用 wr-11 行格 + PAPX 行距。`jc` 左/中/右。
- **stub：** 两端对齐、tab、bidi。
- **defer：** 绕排 drawing、kinsoku/标点挤压（若 wr-16 金文件证明 Java 因此换行不同，**另开 feature**，禁止把挤压塞进本文件「顺便做」）。

## 范围内 / 范围外

- **in：** 标题段（更大字号 → 更少字/行，必须反映在 cp 切分上）；加粗（仅当 wr-11 advance 变化时影响断行）。
- **out：** 分页、寡妇孤儿、表、图、改 wr-04 greedy `Wrap.rs` 冒充本 feature。

## 验收

L2-line 的 **行子集**：忽略 `page_index`，`start_cp/end_cp` 与 Java 相等；行 `x/width/height` 走 Stage2 ±0.01px。

## 依赖

- **depends：** wr-11 宽度；wr-02 `content_width`。
- **does not depend：** wr-13 分页；Flutter。

## 不过扩展

禁止实现 `processPagination`。禁止用贪心空格算法替换 Java 断点却标 pass。现有 wr-04 `Wrap.rs` 只服务 MVP Hello，不是本 feature 的实现。
