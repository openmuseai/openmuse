# wr-20-edit-relayout

> status: **pass**（`01`–`03`；见 [FUNCTIONS.md](./FUNCTIONS.md)）  
> 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md) §9

## 目标

在堆上 **`DocumentSession`** 上应用 **`DocumentEvent` 语义的插入/删除**，然后 **全量** `layout_text_document`。先改 `TextDocument` 存储，再排版。不是 CoreThread，不是增量 cache，不是在 `LayoutFrame` 上改字。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | 已 open 的 session + `cp`/`[start,end)` + 文本（INSERT）；或区间（REMOVE）。 |
| **O** | `DocumentEvent`（`getType`/`getOffset`/`getLength`）+ 新的 `main_length`；relayout 后的 `LayoutFrame`。 |

禁止改 Frame 上的 `DrawText.text`。空插入/空删除不改变 rope。

## Java 对照

`DocumentEvent.EVENT_INSERT=1` / `EVENT_REMOVE=2`；`DefaultModelEvent.obtain` 填 offset/length/type。布局侧 Java `FlowLayout.update(DocumentEvent)` 走增量；本波 **只做全量重排**。rope 写入对标 `ITextRope.insert` / `erase`（与 `CoreHelper.appendText` 同一指针缝）。

## 必须 / 壳 / 延后

- **must：** 任意合法 cp 插入；半开区间删除；事后全量 layout 可见新字。
- **stub：** 无。
- **defer：** CoreThread；EventCollector 合并；KRange.deleteRange 全路径；形状/表/节 STRUCTURE 事件。

## 范围内 / 范围外

- **in：** FRB 编辑后重画。
- **out：** 撤销栈；IME composing；Flutter 测宽。

## 验收

Hello fixture：cp 0 插入 `"Z"` 后 length+1，layout 的 `DrawText` 同时含 `Z` 与 `Hello`。

## 依赖

- **depends：** wr-18 session；wr-09..15 产品 layout；`ITextRope`。
- **does not depend：** wr-19 选区绘制。

## 不过扩展

禁止本 feature 实现增量行 cache。需要 `DefaultModelEvent` 对象池时走 java2rust，禁止从空文件手写池。
