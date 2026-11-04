# docs/java — Java 管线逐环节详解

> 依据：`libs/word-core/layout/JAVA_RUST_PIPELINE.md`（类级蓝皮书）。
> 本文档把 S01–S19 每个环节拆成独立文档，各文档固定包含：**职责 / 核心 Java 代码 / 数据模型 / 输入输出 / 上下游 / 边界与不变量 / Rust 承接**。

## 读法

- 主线按阶段号顺序读（S01 → S19）。
- 每个文档末尾有 `Rust 承接`，标注对应 wave0 / wc-XX / wr-XX feature 及其状态。
- 涉及坐标/单位只信 `00_overview.md` 的全局单位表。

## 文档索引

| 阶段 | 文档 | 一句话 |
|------|------|--------|
| — | [00_overview.md](./00_overview.md) | 端到端总览：架构切断点、单位、验收 |
| S01 | [S01_opc-package.md](./S01_opc-package.md) | `.docx` 字节 → OPC 包（part/类型/关系） |
| S02 | [S02_xml-events.md](./S02_xml-events.md) | Part 流 → XML 元素事件（elementId/attrs） |
| S03 | [S03_handler-dispatch.md](./S03_handler-dispatch.md) | 事件 → handler 栈分发 |
| S04 | [S04_props-convert.md](./S04_props-convert.md) | 属性事件 → PropertySet 内核 propId 桶 |
| S05 | [S05_paragraph-import.md](./S05_paragraph-import.md) | 正文事件 → rope/CHPX/PAPX/节落库 |
| S06 | [S06_docx-reader-finish.md](./S06_docx-reader-finish.md) | 编排收尾 → 完整 TextDocument（Stage1） |
| S07 | [S07_assembly-viewenv.md](./S07_assembly-viewenv.md) | LayoutManager 装配 + ViewEnv 页几何 |
| S08 | [S08_measure-font.md](./S08_measure-font.md) | 字体解析 + 字符 advance/行高 |
| S09 | [S09_line-break.md](./S09_line-break.md) | run 序列 → 行（折行） |
| S10 | [S10_line-geometry.md](./S10_line-geometry.md) | 行高/基线/对齐 → 行框 |
| S11 | [S11_pagination.md](./S11_pagination.md) | 页满/分页符/节切换 → 页序列 |
| S12 | [S12_snapshot.md](./S12_snapshot.md) | TypoDocument/TypoSnapshot 快照组装 |
| S13 | [S13_locate.md](./S13_locate.md) | 字符 ↔ 页/行/run 定位命中 |
| S14 | [S14_render-command.md](./S14_render-command.md) | TypoSnapshot → RenderCommand（twip） |
| S15 | [S15_serialize.md](./S15_serialize.md) | 指令序列化（serde JSON）+ 传输 |
| S16 | [S16_flutter-painter.md](./S16_flutter-painter.md) | Flutter 解析指令 → Canvas（twip→px） |
| S17 | [S17_rasterize.md](./S17_rasterize.md) | 字形光栅化与图层合成（像素） |
| S18 | [S18_viewport-compose.md](./S18_viewport-compose.md) | 滚动/缩放/页间隙视口合成 |
| S19 | [S19_interaction-loop.md](./S19_interaction-loop.md) | 输入 → 重排/重绘闭环 |

## 三条贯穿性约定（任何环节不得违反）

1. **cp 一致**：全文线性 Character Position，与 TextDocument rope / CHPX / PAPX / PLCSection 同一套 cp（S01–S06 锁死）。
2. **单位一致**：排版内一律 twip（1pt=20twip）；px 只出现在 S16–S18（`zoom × 1.3333536/20`，96DPI 约定，再乘 devicePixelRatio）。
3. **验收一致**：Stage1=DocumentModel JSON 严格相等；Stage2=TypoSnapshot JSON ±0.01px；Stage3=RenderCommand 序列+字段相等。