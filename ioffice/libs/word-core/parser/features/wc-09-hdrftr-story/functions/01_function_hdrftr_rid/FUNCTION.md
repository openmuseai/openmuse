# 01_function_hdrftr_rid — sectPr header/footer rId → PLCHeader 节点

| 项 | 值 |
|----|----|
| feature | wc-09-hdrftr-story |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `HdrFtrDocumentImporter::addNodeHeaderEven 等` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
sectPr header/footer rId → PLCHeader 节点

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
- **改写列表：** `addNodeHeaderEven,addNodeHeaderOdd,addNodeFooterEven,addNodeFooterOdd,addNodeHeaderFirst,addNodeFooterFirst,disposeHeaderFooterData`
- 列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION … */` 停车原模版体。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| PLCHeader | data | addNodeEvenHeader 等 | 部分 | 完整 PLC 编辑 | 禁止 stub HdrFtrDocumentImporter |
| SubDocumentImporter | io | super ctor | 模板/壳 | 全文 importer | 禁止改主故事 DocumentImporter 热路径 |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| addNode* | HdrFtrDocumentImporter.java:33+ | 首次调用 | PLC.Node 非空 | 二次调用 assert |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| HF1 | add even header | 公开入口 | 是 | HF1 |
| HF2 | 二次 add 失败 | 失败 | 是 | HF2 |
| HF3 | dispose 后再 add | cleanup | 是 | HF3 |
| HF4 | footer odd 链 | 状态转换 | 是 | HF4 |

### 3.3 可测试性
fixture / 内存字节 / 假上游输出；不启动全管线。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| HF1 | HF1 | 新 importer | addNodeHeaderEven | mHeaderEven Some | None |
| HF2 | HF2 | 已 add | 再 add | assert/panic | 静默覆盖 |
| HF3 | HF3 | disposeHeaderFooterData | 节点清空 | 悬挂 |
| HF4 | HF4 | even→odd→footer | 链 CP 递增 | 乱序 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_hdrftr_rid

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_hdrftr_rid \
  --src src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.rs \
  --report libs/word-core/parser/features/wc-09-hdrftr-story/functions/01_function_hdrftr_rid/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_hdrftr_rid --json`)
date: 2026-09-06 15:04

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.rs` | 74/209 | 35.4% | 116/350 | 33.1% |
| **function total** | **74/209** | **35.4%** |  |  |
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
  --report libs/word-core/parser/features/wc-09-hdrftr-story/functions/01_function_hdrftr_rid/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-06 07:04

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `HdrFtrDocumentImporter` | `koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.java` | `libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/HdrFtrDocumentImporter.rs` | **PASS** | `HdrFtrDocumentImporter`, `addNodeHeaderEven`, `reviseNodeHeaderEven`, `addNodeHeaderOdd`, `reviseNodeHeaderOdd`, `addNodeFooterEven`, `reviseNodeFooterEven`, `addNodeFooterOdd`, `reviseNodeFooterOdd`, `addNodeHeaderFirst`, `reviseNodeHeaderFirst`, `addNodeFooterFirst`, `reviseNodeFooterFirst`, `getHeaderEven`, `getHeaderOdd`, `getFooterEven`, `getFooterOdd`, `getHeaderFirst`, `getFooterFirst`, `disposeHeaderFooterData` | `startNodeHeaderEven`, `startNodeHeaderOdd`, `startNodeFooterEven`, `startNodeFooterOdd`, `startNodeHeaderFirst`, `startNodeFooterFirst`, `endNodeHeaderEven`, `endNodeHeaderOdd`, `endNodeFooterEven`, `endNodeFooterOdd`, `endNodeHeaderFirst`, `endNodeFooterFirst` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
