# state-java-l2-parity

> 快照：2026-09-03  
> 目录原占位名 `state-xxxx` **定名为** `state-java-l2-parity`：与 Java 排版管线对齐，验收 **L2 行级几何**；**只允许行内像素级偏差**（`xpos[]` / 光栅 / AA，不进 CI）。

| 文件 | 用途 |
|------|------|
| [PIPELINE_OVERVIEW.md](./PIPELINE_OVERVIEW.md) | 已实现 parser/layout feature 盘点（权威现状） |
| [NEXT_PLAN.md](./NEXT_PLAN.md) | 下一步开发计划、里程碑、验收口径 |
| [features.csv](./features.csv) | 本波新 feature 清单（与 parser/layout `manifest.csv` 同步） |

**上一波计划**（行级对齐 wr-09..16 + 产品 session wr-18..20）仍在 [layout/LINE_PARITY_PLAN.md](../layout/LINE_PARITY_PLAN.md)。本目录是 **那一波落地之后** 的下一波：把产品入口接到 Java `TextLayouter`/`ParaLayouter` 热路径，并用 **实排 Stage2** 对 Java dump。

**执行 skill：** feature-function（本波已拆 function，`status=spec`；loop 时再改 src / 落 `tests/` / 刷覆盖率与对齐报告）。

**第一个 loop：** `parser/features/wc-06-stage1-live` → `01_function_rope_dump`（须用户点名 `loop` 才改 src）。
