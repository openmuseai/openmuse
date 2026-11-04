# Layout+Render 管线 features（wr-0X / wr-1X）

> 分析日期：2026-08-22；行级对齐波次：2026-08-23  
> 前置：parser MVP（wc-05）；macOS 样例可画字（wr-01..08 产品捷径）  
> 目标分两波：  
> 1. **MVP 可画**：zip 抽文本 → greedy wrap → `LayoutFrame`（已落地）  
> 2. **纯文本行级对齐（已落地）**：TextDocument → 行表叶子 wr-09..16（[LINE_PARITY_PLAN.md](../LINE_PARITY_PLAN.md)）  
> 3. **现行：Java 热路径 + 实排 L2**：wc-06/07 + wr-21..24（[state-java-l2-parity](../../state-java-l2-parity/README.md)）  
> 拆分方法：切 **I/O**，不按类切。wr-21..24 已拆 function（`status=pass`）；wr-25..28 只锁 I/O。

## feature 清单（manifest.csv 同步）

| feature_id | 标题 | depends_on | seeds 方向 |
|------------|------|-----------|-----------|
| wr-01-typo-cache | 布局结果模型（Typo 对象/快照/cp 平移） | wc-02（TextDocument）；无 layout 依赖 | TypoDocument/TypoSnapshot/TypoLayoutPage/TypoPage/TypoTextLine/TypoRun/迭代器 |
| wr-02-viewenv | 视图参数（页尺寸/缩放/模式） | 无 | ViewEnv/PageSetup |
| wr-03-measure-shape | 文本度量（字体栈 + 行宽/字形 + FontHost） | wc-02 KStyles（字号/字体）；thirdparty/font | RichTextLayout 审计 + FontStack + FontHost（注入表，对标 Java FontHost） |
| wr-04-line-break | 断行/行高/对齐（**MVP 贪心，≠ Java**） | wr-02；wr-03 | wrap.rs；行级对齐改走 wr-12 |
| wr-05-page-flow | 分页/节/网格（**MVP 高度累加，≠ Java**） | wr-04；wc-01 PLCSection | 高度分页；行级对齐改走 wr-13 |
| wr-06-typo-build | 布局→快照组装与失效 | wr-01；wr-05 | TypoDocument.newSnapshot/commitSnapshot + ISnapshotListener |
| wr-07-render-command | RenderCommand 指令生成与序列化 | wr-06；wc-02 KStyle/PropertySet | LayoutRender 指令化（文本/装饰/背景） |
| wr-08-ffi-flutter | UniFFI/JSON 桥 + Flutter 消费 | wr-07 | 导出入口 + Flutter painter |
| wr-09-run-stream | 主故事 run 流（TextDocument，不抽 XML） | wc-05 | CHPX/PAPX 游标 |
| wr-10-font-style | 标题/加粗 → TypoFontStyle | wr-09 | FontStyleFactory |
| wr-11-run-metrics-parity | 同 TTF 逐 cp 度量对齐 | wr-10；wr-03 | hmtx / 行格 |
| wr-12-line-layouter | Java 断行（行 cp 区间） | wr-11 | LineLayouter |
| wr-13-page-layouter | Java 分页（含寡妇孤儿最小集） | wr-12 | processPagination |
| wr-14-typo-stage2 | Snapshot + Stage2 行表 | wr-01；wr-13 | commitSnapshot |
| wr-15-style-commands | 带样式的 DrawText | wr-14；wr-07 | 标题字号/bold |
| wr-16-line-golden | 行级双跑 CI（本波出口） | wr-14 | Java 行表 dump |
| wr-17-hit-test | 命中 cp（下一波，只锁 I/O） | wr-16 | LayoutLocater |
| wr-18-session-caret | 编辑会话 + caret cp | wr-17 | DocumentSession / SessionCaret |
| wr-19-selection | 选区 FillRect | wr-18 | SelectionRange / Rects / Fill |
| wr-20-edit-relayout | 编辑后全量重排 | wr-18 | SessionEdit / DocumentEvent |
| wr-21-para-layouter | ParaLayouter 段框 | wc-07；wr-12 | **pass** M12 |
| wr-22-text-flow | TextLayouter.flow | wr-21 | **pass** M13 |
| wr-23-pageview-main | PageView MODE_PAGE | wr-22 | **pass** M14 |
| wr-24-live-stage2 | 产品实排 Stage2 | wr-23；wr-16 | **pass** M15 出口 |
| wr-25-line-compress | 整行压缩 | wr-22 | spec 后波 |
| wr-26-kinsoku | 东亚禁则 | wr-22 | spec 后波 |
| wr-27-table-layout | 表格排版 | wr-22 | pass |
| wr-28-drawing-wrap | 图形绕排 | wr-22 | spec 后波 |
| wr-29-java-pipeline-cutover | Java 排版栈切线（P1–P4） | wr-22；P0 | **looping** M-STAGE3 |

MVP 依赖序：`wr-01 → … → wr-08`。  
行级叶子：`wr-09 → … → wr-16`（已 pass）。`wr-17..20` session（已 pass）。  
热路径：`wc-06 → wc-07 → wr-21 → wr-22 → wr-23 → wr-24`。

## Rust 现状（2026-09-03）

- **产品入口** `layout_document`：SAX → `TextDocument` → `RunStream` → env `LineLayout`/`paginate` → `LayoutFrame`。可画 sample2（含 jc）。**仍不是** Java `TextLayouter.flow`。
- wr-03 FontHost / FontStack：注入表，不探盘；缺 cmap 失败而非假宽度。
- wr-04 `Wrap.rs` / wr-05 高度分页：MVP 捷径；禁止再堆 Java。热路径改走 wr-21..23。
- wr-16 pass = 预导出 JSON 比较器；实排门闩是 wr-24。
- 验收：**L2-line**（页/行/cp 精确；行框 Stage2 ±0.01px）。`xpos[]` / 光栅不进 CI。