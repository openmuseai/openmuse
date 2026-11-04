# wr-13-page-layouter

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（`01`–`03` 均 pass）
> 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)

## 目标

**分页与 Java 一致**：哪一行落在哪一页、页满、`\\f`、以及纯文本下 Java 默认的寡妇/孤儿控制。打印页数与页内行集合必须与 Java 相同。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | wr-12 的文档行流（可按段拼接）+ 单节 `content_height`（页高−上下边距）+ 行高 + PAPX `widowControl`（prop 197，Java 默认常见为 on）+ 源中的 `\f`。 |
| **O** | `TypoLayoutPage[]`：每页行列表、页序、页内 cp 区间；行 flag `LASTLINEOFPAGE`；`page_count`。 |

不改变 wr-12 已经切好的行（除 Java `processPagination` 为寡妇孤儿 **整行回滚到下一页** 的那些行）。

## Java 对照

`TextLayouter.processPagination` + `MainLayouter.flowPage` / `finishPage`。单节 `PageViewLayouter`。

## 必须 / 壳 / 延后

- **must：** 内容区高度不足则换页（y + lineHeight > content_height）；`\f` 强制换页；prop 197 开启时的段末寡妇处理（与 Java：非段末行触页底时探测下一行是否段末并 rollback）。
- **stub：** 孤行（段首单行留在页底）若 Java 同源函数覆盖则 must，否则列 skip 并在 wr-16 fixture 标明。
- **defer：** 多节不同 PageSetup、分栏、表跨页、drawing 清页、`clearRedundantDrawings`。

## 范围内 / 范围外

- **in：** 纯文本多页；标题大行高导致提前换页；段前距计入页高。
- **out：** 改断行；IPageGrid 完整；CoreThread；页眉占位。

## 验收

L2-line 的 **页字段**：每行 `page_index` 与 Java 精确相等；因此 `page_count` 相等。

## 依赖

- **depends：** wr-12 行流；wr-02 页几何（单节 A4 默认即可）。
- **does not depend：** wr-07 指令；命中测试。

## 不过扩展

禁止为「好看」在 Rust 侧多留一行。禁止把节属性解析做成新 parse 项目。
