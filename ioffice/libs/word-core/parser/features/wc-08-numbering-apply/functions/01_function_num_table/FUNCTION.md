# 01_function_num_table — numbering.xml → AutoNumTable

| 项 | 值 |
|----|----|
| feature | wc-08-numbering-apply |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `AutoNumTable ctor + Lst/Lfo 表` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
numbering.xml → AutoNumTable

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | `cn/wps/moffice/writer/data/autonum/AutoNumTable.java` |

### 1.3 输出
见 §1.1 与用例表。

### 1.4 上游
声明的上游 fixture / 假 PropertySet；不跑无关管线。

### 1.5 下游
见 FUNCTIONS.md。

### 1.6 不变量与函数改写列表
- 切在 I/O。
- **改写列表：** `AutoNumTable,getLstLfoTool` 已有模板则填体；importer 另叶若需则生成
- 列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION … */` 停车原模版体。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| LstDataTable / LfoDataTable | autonum | 表存储 | 部分模板 | 完整 numbering SAX | 禁止 stub AutoNumTable |
| Transaction | util | ctor | 是 | — | 禁止改 Transaction |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| AutoNumTable | AutoNumTable.java | transaction | mLstTable/mLfoTable 非空 | null ctor |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| NT1 | ctor | 公开入口 | 是 | NT1 |
| NT2 | getLstLfoTool | 公开入口 | 是 | NT2 |
| NT3 | 空 numbering | 失败 | 是 | NT3 |

### 3.3 可测试性
fixture / 内存字节 / 假上游输出；不启动全管线。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| NT1 | NT1 | new AutoNumTable | lst/lfo Some | None |
| NT2 | NT2 | getLstLfoTool | 非空 | null |
| NT3 | NT3 | 空 xml | 空表可查询 | panic |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/data/autonum/AutoNumTable.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/data/autonum/AutoNumTable.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_num_table

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_num_table \
  --src src/cn/wps/moffice/writer/data/autonum/AutoNumTable.rs \
  --report libs/word-core/parser/features/wc-08-numbering-apply/functions/01_function_num_table/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_num_table --json`)
date: 2026-09-06 14:49

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/data/autonum/AutoNumTable.rs` | 22/33 | 66.7% | 42/67 | 62.7% |
| **function total** | **22/33** | **66.7%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/data/autonum/AutoNumTable.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/data/autonum/AutoNumTable.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/data/autonum/AutoNumTable.java --rs libs/word-core/src/cn/wps/moffice/writer/data/autonum/AutoNumTable.rs \
  --report libs/word-core/parser/features/wc-08-numbering-apply/functions/01_function_num_table/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-06 06:49

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `AutoNumTable` | `../../koffice/writer/core/cn/wps/moffice/writer/data/autonum/AutoNumTable.java` | `src/cn/wps/moffice/writer/data/autonum/AutoNumTable.rs` | **PASS** | `AutoNumTable`, `getLstDataTable`, `getLfoDataTable`, `dispose`, `getLstLfoTool` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
