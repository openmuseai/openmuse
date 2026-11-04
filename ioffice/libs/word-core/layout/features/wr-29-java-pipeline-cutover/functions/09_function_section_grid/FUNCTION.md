# 09_function_section_grid — sectPr docGrid → 行高贴网格

| 项 | 值 |
|----|----|
| feature | wr-29-java-pipeline-cutover |
| order | 9 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | SAX `SectPrHandler` → `convertSectPr`/`convertDocumentGrid` → `SectionTool.calcGrid` → `LineHeightLayouter` 1.5×312 |
| status | **wired** |
| 阶段 | **layout**（同一 sha256 sample2；WPS `sample2_01.png` 页结构） |

## 1. 功能说明

### 1.1 一句话
把 `sample2.docx` 的 `w:docGrid type="lines" linePitch="312"` 按 Java 灌进 `PageEnv`，使自动 1.5 倍行距变成 `312×1.5=468` twip，第一页结构对齐 WPS p1（不含「三、」）。页眉「毕业论文」不在 OPC，归 STAGE6。

### 1.2 输入
`samples/docx-mvp-render/flutter/assets/samples/sample2.docx`  
sha256 `eb4e1466d75ea7856b62f332da9864ada599f440da9a38ef0e55feec754bafd3`  
WPS 金图 `sample2_01.png`…`sample2_08.png`。

### 1.3 输出
- PLCSection.sepx `677=2`、`678=312`
- 正文相邻行 dy≈468
- page 0 有标题/概述/近年来/调查方式，**无**「三、农村中小学教育现状」
- hello 单行；标题居中

### 1.4 上游
`08_function_wrap_para_env`（折行 + 段环境）。

### 1.5 下游
Flutter 主页截图 vs WPS `sample2_01.png`。

### 1.6 不变量与函数改写列表

**SectPrHandler.rs / DocumentBodyHandler.rs**
- `getElementHandler(-906275931)` 返回 `SectPrHandler`
- `onEnd` → `onImportSectProp`

**SectPrHandlerHelper.rs**
- `getPropType` = `PropType::SECTPR`（3），禁止再返回 0（RPR）

**MainDocumentImporter.convertSectPr**
- `elementId==1827547294` → `convertDocumentGrid`

**SectionTool.calcGrid**
- `gridModel`/`gridHeight`/`gridWidth`

**LayoutDocument.java_pipeline_pages**
- 读 PLCSection.sepx 后 `SectionTool::calcGrid`

**SaxDocumentBridge**
- `onImportSectProp` 转到 `DocumentImporter`

不解：hdrftr 子 handler、`convertSectionPrContents` 全量 pgSz/pgMar（09-D1）、STAGE6 页眉。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要 | 已实现 | 暂缓 |
|------|------|------|--------|------|
| 08 wrap/indent | 08 | 折行 | 是 | — |
| LineHeightLayouter snap | 05 | needSnapToGrid | 是（gridHeight>0 后激活） | — |
| convertSectionPrContents | SectionPropsImporter | pgSz/pgMar | **否** 09-D1 | ViewParams A4 已对齐 sample2 页盒 |
| hdrftr | STAGE6 | 毕业论文 | 否 | OPC 无 header part |

#### 1.7.1 编号条目

- **09-D1**：`convertSectionPrContents` 非 grid 子元暂缓；sample2 `pgSz`/`pgMar` 与 `ViewParams::default_a4()` 同值。
- **09-D2**：`getFontSize4Section` 在无 `&mut KStyle` 时用 Java 缺省 10.0pt（charSpace=0 不改 678）。
- **09-D3**：WPS 金图页眉「毕业论文」不在 docx OPC。结构对比不含页眉。
- **09-D4**：`LayoutDocument.rs` 无 Java 同名文件。禁止 D1 错配。

## 2. 核心代码

`DocumentBodyHandler.getElementHandler(sectPr)` → `SectPrHandler` → `convertDocumentGrid` → `SectionTool.calcGrid`。禁止删 `// java`。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust | 输入 | 输出 |
|--------|-----------|------|------|
| convertDocGridType | SectionPropsImporter | type=lines | 677=2 |
| calcGrid | SectionTool.java:313 | sepx | gridHeight=312 |
| auto 1.5 snap | LineHeightLayouter | grid 312 | lineHeight=468 |
| flowToPage | 宿主 | 更高行 | p0 不含「三、」 |

### 3.2 覆盖矩阵

| coverage_id | 场景 | 必测? | test_id |
|-------------|------|-------|---------|
| G1 | sepx docGrid | 是 | G1 |
| G2 | 行高 468 | 是 | G2 |
| G3 | WPS p1 块 | 是 | G3 |
| G4 | hello | 是 | G4 |
| G5 | 标题居中 | 是 | G5 |
| G6 | 产品接线 | 是 | G6 |

### 3.3 管线测试

同一 sample2.docx → parse → Java layout。FontHost：`--test-threads=1`。

### 3.4 用例表

| id | 期望 |
|----|------|
| G1 | 677=2 678=312 |
| G2 | dy∈[428,508] |
| G3 | p0 无「三、农村中小学教育现状」 |
| G4 | hello 1 条 |
| G5 | 标题 x>1800 |
| G6 | 源含 `SectionTool::calcGrid` |

### 3.5 测试代码位置

crate: `libs/word-core/Cargo.toml`  
src:   `layout/base/SectionTool.rs`；`MainDocumentImporter.rs`；`api/LayoutDocument.rs`；`api/SaxDocumentBridge.rs`  
       `io/wps-io/.../SectPrHandler.rs`；`DocumentBodyHandler.rs`；`SectPrHandlerHelper.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/SectionTool.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 09_function_section_grid -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 09_function_section_grid \
  --src src/cn/wps/moffice/writer/layout/base/SectionTool.rs \
  --src src/cn/wps/moffice/writer/api/LayoutDocument.rs \
  --report libs/word-core/layout/features/wr-29-java-pipeline-cutover/functions/09_function_section_grid/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
（L3b 由 function-coverage.sh 写入）
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/MainDocumentImporter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/document/MainDocumentImporter.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/core-libs/io/wps-io/src/cn/wps/io/xwpf/element_handler/SectPrHandler.rs \
  --java koffice/core-libs/io/wps-io/src/cn/wps/io/xwpf/element_handler/SectPrHandler.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/SectionTool.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/SectionTool.java
```

`LayoutDocument.rs` / `SaxDocumentBridge.rs` 无 Java 孪生（09-D4）。

### 3.9 Java↔Rust 门面对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/core-libs/io/wps-io/src/cn/wps/io/xwpf/element_handler/SectPrHandler.java \
  --rs libs/core-libs/io/wps-io/src/cn/wps/io/xwpf/element_handler/SectPrHandler.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/SectionTool.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/SectionTool.rs \
  --report libs/word-core/layout/features/wr-29-java-pipeline-cutover/functions/09_function_section_grid/FUNCTION.md
```

<!-- java-rust-align:begin -->
（L5c 由脚本写入）
<!-- java-rust-align:end -->

### 3.10 Java↔Rust 管线对比

| 步 | Java | Rust |
|----|------|------|
| 1 | `DocumentBodyHandler` 遇 `sectPr` 建 `SectPrHandler` | 同 |
| 2 | `PropChildHandler` `PropType.SECTPR` → `convertSectPr` | Helper 返回 `SECTPR` |
| 3 | `convertDocumentGrid` 677/678 | 同 |
| 4 | `onImportSectProp` → `doImportSectionPropertiesSimple` freeze sepx | 桥 `SaxDocumentBridge.onImportSectProp` |
| 5 | `SectionTool.calcGrid` | 宿主 `java_pipeline_pages` 同调 |
| 6 | `LineHeightLayouter` `gridHeight * lineSpacing` | 已有；gridHeight=0 时原先不贴网 |

### 3.11 管线阶段对齐报告

| 项 | 内容 |
|----|------|
| 阶段 | layout |
| 原始文档 sha256 | `eb4e1466d75ea7856b62f332da9864ada599f440da9a38ef0e55feec754bafd3` |
| 步骤对照 | §3.10 |
| 产品接线 | `layout_document` → `java_pipeline_pages` → `calcGrid` |
| 关键路径真实实现 | pass |
| 管线测试 | G1–G6 |
| 测试结果符合预期 | pass（`cargo test --test 09_function_section_grid -- --test-threads=1` 6/6） |
| 判定 | **wired**（§3.8/§3.9 overall 未刷 PASS；逻辑 G3 对齐 WPS p1 分页） |
