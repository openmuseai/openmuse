# wc-06-stage1-live

> status: **pass**  
> 计划：[state-java-l2-parity/NEXT_PLAN.md](../../../state-java-l2-parity/NEXT_PLAN.md) M11  
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)

## 目标

对真实 `.docx` 跑产品 `open_text_document`，dump **Stage1**（rope + 影响 L2 的 PAPX/CHPX），与 Java 同文档 dump **严格相等**。这是改断行之前的门闩：桶不对则禁止宣称 wr-12/21 对齐。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | corpus `.docx` 字节 + 可选 Java Stage1 JSON |
| **O** | Rust Stage1 JSON；比较器：文本全等；每段/每 run 列出的 sprm 全等 |

## Java 对照

`TextDocument.getMainDocument()`：`getText`、`IPapxTable.seek`、`IChpxTable.seek`。不是 layout。

## 必须 / 壳 / 延后

- **must：** rope（含 `\r`/`\f`）；PAPX 191/193/197/208/210/212/214/217/220（缺 key 与 Java `contains` 一致）；CHPX 字号/粗体/eastAsia 族名。
- **stub：** 修订、域结果对象。
- **defer：** 表单元格内 PAPX 完整集；页眉故事。

## 验收

sample2：标题段 193=1、sz→360 twip、`w:b`；正文首行 208≠0（若 Java dump 有）。缺 sprm → 记 wc-07，本 feature 比较器仍 fail。

## 不过扩展

禁止在本 feature 里改 LineLayouter。禁止为过 CI 改 Java dump。
