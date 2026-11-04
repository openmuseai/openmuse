# wr-16-line-golden

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（`01`–`03` 均 pass）
> 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)

Java 基线：[`tests/fixtures/03_function_line_corpus`](../../../tests/fixtures/03_function_line_corpus/README.md)，
由 Android 14 上的 koffice `TypoTextLine` Stage2 导出器生成，覆盖 latin/cjk/heading/bold/multipage。

## 目标

本波 **出口门闩**：纯文本 corpus 上，Rust 行表与 Java 行表 **L2-line 相等**。没有这个 feature 的绿，不得宣称「与 Java 渲染结果行级一致」。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | fixture `.docx`（仅纯文本+标题+加粗等，见计划 §3）+ 同一 TTF 字节 + Java 导出的行表 JSON。 |
| **O** | CI 判定：pass / fail。失败报告第一处不同的 `page/line/start_cp/end_cp` 或超容差的 `x/y/w/h`。 |

比较器 **忽略** `xpos`、颜色光栅、指令条数。

## Java 对照

从 Java `TypoSnapshot` / Stage2 导出，不是 Android 截图，不是完整 `InstractionBuilder` 中文序列。

## 必须 / 壳 / 延后

- **must：** 计划 §6 所列 corpus 类别至少各 1 个 fixture；中文 fixture 使用含 cmap 的注入字体。
- **stub：** 无。跳过的 Java 行为必须写进 skip 清单并说明将单开 feature（例如标点挤压）。
- **defer：** L3 截图；五端 hash（沿用 P5 只守 Hello）。

## 范围内 / 范围外

- **in：** 行级双跑。
- **out：** 表/图/页眉 fixture；改 Java 金以迁就 greedy wrap；在 Flutter 里重排再比。

## 验收

`cargo test`（名称待 function 拆分时再定）对 corpus 全绿。skip 清单空或仅含已记录的非本波行为。

## 依赖

- **depends：** wr-14 行表（最低 wr-12 行 cp + wr-13 页号）。
- **does not depend：** wr-17；完整 Stage3。

## 不过扩展

禁止把「看起来差不多」的截图当 pass。禁止为过 CI 修改 Java dump 的 cp 切分。
