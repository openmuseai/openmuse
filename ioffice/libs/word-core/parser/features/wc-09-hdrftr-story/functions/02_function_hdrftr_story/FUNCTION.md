# 02_function_hdrftr_story — header/footer part → SubDocument 故事

| 项 | 值 |
|----|----|
| feature | wc-09-hdrftr-story |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `HdrFtrDocumentImporter::reviseNodeHeaderEven 等 end 填故事` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
header/footer part → SubDocument 故事

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | `cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.java` |

### 1.3 输出
见 §1.1 与用例表。

### 1.4 上游
声明的上游 fixture / 假 PropertySet；不跑无关管线。

### 1.5 下游
见 FUNCTIONS.md。

### 1.6 不变量与函数改写列表
- 切在 I/O。
- **改写列表：** `reviseNodeHeaderEven,reviseNodeHeaderOdd,reviseNodeFooterEven,reviseNodeFooterOdd,reviseNodeHeaderFirst,reviseNodeFooterFirst`
- 列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION … */` 停车原模版体。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| addNode* | wc-09 01 | PLC 节点 | loop 01 | — | 禁止改探测顺序 |
| SubDocument.getLength | core | 故事长度 | 是 | 页眉排版 | 禁止改主故事 L2 |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| reviseNode* | HdrFtrDocumentImporter | 导入后 length | moveRight 到 endCp | 未 add 先 revise |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| HS1 | revise even header | 公开入口 | 是 | HS1 |
| HS2 | 空 header | 边界 | 是 | HS2 |
| HS3 | 未 add 就 revise | 失败 | 是 | HS3 |

### 3.3 可测试性
fixture / 内存字节 / 假上游输出；不启动全管线。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| HS1 | HS1 | 写入文本后 revise | odd 节点右移 | CP 不变 |
| HS2 | HS2 | 空 part | length 合法 | panic |
| HS3 | HS3 | 无 add | assert | 静默 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/document/02_function_hdrftr_story.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_hdrftr_story

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_hdrftr_story \
  --src src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.rs \
  --report libs/word-core/parser/features/wc-09-hdrftr-story/functions/02_function_hdrftr_story/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_hdrftr_story --json`)
date: 2026-09-06 15:04

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.rs` | 63/209 | 30.1% | 96/350 | 27.4% |
| **function total** | **63/209** | **30.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.java --rs libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.rs \
  --report libs/word-core/parser/features/wc-09-hdrftr-story/functions/02_function_hdrftr_story/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-06 07:04

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `HdrFtrDocumentImporter` | `koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.java` | `libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.rs` | **PASS** | `HdrFtrDocumentImporter`, `addNodeHeaderEven`, `reviseNodeHeaderEven`, `addNodeHeaderOdd`, `reviseNodeHeaderOdd`, `addNodeFooterEven`, `reviseNodeFooterEven`, `addNodeFooterOdd`, `reviseNodeFooterOdd`, `addNodeHeaderFirst`, `reviseNodeHeaderFirst`, `addNodeFooterFirst`, `reviseNodeFooterFirst`, `getHeaderEven`, `getHeaderOdd`, `getFooterEven`, `getFooterOdd`, `getHeaderFirst`, `getFooterFirst`, `disposeHeaderFooterData` | `startNodeHeaderEven`, `startNodeHeaderOdd`, `startNodeFooterEven`, `startNodeFooterOdd`, `startNodeHeaderFirst`, `startNodeFooterFirst`, `endNodeHeaderEven`, `endNodeHeaderOdd`, `endNodeFooterEven`, `endNodeFooterOdd`, `endNodeHeaderFirst`, `endNodeFooterFirst` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
