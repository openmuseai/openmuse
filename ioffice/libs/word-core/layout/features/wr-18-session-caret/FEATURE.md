# wr-18-session-caret

> status: **pass**（`01`–`04`；见 [FUNCTIONS.md](./FUNCTIONS.md)）  
> 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md) §9

## 目标

在 **已打开的堆上 `TextDocument`** 上做编辑会话与光标：open → 改 rope/PAPX → 全量 relayout；点 → caret cp。这是 FRB 持有 session 的内核契约，不是选区反色，不是 IME。

`LINE_PARITY_PLAN` 原表把 wr-18 写成选区 `FillRect`。本目录按产品 P7 **session/caret** 占用 **wr-18**；选区指令改为 **wr-19**，`DocumentEvent` 增量重排改为 **wr-20**。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | `.docx` 字节（open）；session + 段文本（append）；`LayoutFrame` + 页坐标 `(page,x,y)` twip（caret；调用方负责屏幕 zoom 反变换）。 |
| **O** | 堆稳定 `DocumentSession`（`Box<TextDocument>`）；append 后主故事长度；relayout 后的 `LayoutFrame`；caret `Option<cp>`。 |

下游只许依赖 session / frame / cp，不许再对同一文档 `layout_document(bytes)` 当编辑路径。

## Java 对照

- 存储写入：`CoreHelper.appendText` / `insertToPapxTable`（已有模板，改写走 **function-java2rust**，禁止从空文件重写）。
- 打开：产品 `DocumentOpen` / `open_text_document`（无同名 Java 类）。
- 光标：`LocateTool` / `LayoutLocater`。本波 **不** 移植 1600 行 `LocateTool`；只锁 `LayoutFrame`（`DrawText.xpos`）→ cp 的 seam。命中几何复用 wr-17 `hit_test`（Stage2 行表），session 热路径尚未把 Stage2 挂到 Frame。

## 必须 / 壳 / 延后

- **must：** heap `Box` open；append 段（text+`\r`+空 CHPX/PAPX）；append 后全量 `layout_text_document`；`cp_for_point` 经 `DrawText.xpos` + wr-17 `cp_for_x`。
- **stub：** 无（本 feature 出口已接线）。
- **defer：** 任意 cp 插入/删除；选区 `FillRect`；IME；CoreThread；增量 cache；`LocateTool` 整类。

## 范围内 / 范围外

- **in：** FRB 将来持有的 native session；wr-17 行表命中作为 caret 上游形状。
- **out：** Flutter `TextPainter` 测宽；在 `LayoutFrame` 上直接改 `DrawText.text`；选区绘制。

## 验收

给定 Hello fixture：open 后 `main_length()>0`；append `"AAA"` 后长度 +4；relayout 的 `DrawText` 同时含 Hello 与 AAA。caret：矩阵 CA1–CA5 在 loop 接线后非 ignore 全绿。

## 依赖

- **depends：** wr-09..15 产品 layout；wr-17 hit-test（caret 几何）；wc-04 `CoreHelper` 写 rope。
- **does not depend：** wr-19 FillRect。

## 不过扩展

禁止本 feature 改 `RunPropsConverterSimple` 或从零生成 `CoreHelper.rs`。需要补 Java 叶时走 java2rust / function-java2rust。
