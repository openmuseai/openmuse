# wr-14-typo-stage2

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（`01`–`03` 均 pass）
> 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)

## 目标

把 wr-13 的页/行写成 **可查询的 `TypoSnapshot`**，并导出 Stage2 **行表 JSON**。排版真相在 Snapshot，不在 zip 捷径的 `LayoutFrame` 私有行缓存。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | wr-13 的 `TypoLayoutPage[]` + 文本 cp 范围 + 版本号（`VER_FIRSTCACHE` 语义的 Rust owned 等价）。 |
| **O** | owned `TypoSnapshot`：按页/行迭代、`getStartCP`/`getEndCP`、行几何 twip。派生 JSON：每行 `page, line, start_cp, end_cp, x, y, width, height`。 |

不复刻 Java `int pointer` 竞技场。

## Java 对照

`TypoDocument.newSnapshot` / `commitSnapshot`；对外契约对齐 Stage2，不是内存布局。

## 必须 / 壳 / 延后

- **must：** 全量提交；行迭代；与 wr-01 已有 Typo 结构字段对齐。
- **stub：** `ISnapshotListener` 失效回调（可 no-op，测试不依赖）。
- **defer：** 编辑 `updatePageRanges`、增量 dirty。

## 范围内 / 范围外

- **in：** 只读快照 + 行表 dump（给 wr-16）。
- **out：** RenderCommand；hit-test 算法（只提供数据）；Flutter。

## 验收

同一 fixture：Stage2 行表 vs Java dump，L2-line 字段全过。xpos 不在本 JSON。

## 依赖

- **depends：** wr-01 结构；wr-13 页流。可复用 wr-06 组装思路，**输入必须是真实行流**。
- **does not depend：** wr-15 emit。

## 不过扩展

禁止在 Snapshot 里重新断行。禁止把 LayoutFrame 当 Stage2 真相。
