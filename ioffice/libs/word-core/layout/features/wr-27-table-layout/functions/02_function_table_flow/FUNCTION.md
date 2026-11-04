# 02_function_table_flow — TableLayouter.flow 表行几何

| 项 | 值 |
|----|----|
| feature | wr-27-table-layout |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `TableLayouter::flow` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
TableLayouter.flow 表行几何

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | `cn/wps/moffice/writer/layout/base/table/TableLayouter.java` |

### 1.3 输出
见 §1.1 与用例表。

### 1.4 上游
声明的上游 fixture / 假 PropertySet；不跑无关管线。

### 1.5 下游
见 FUNCTIONS.md。

### 1.6 不变量与函数改写列表
- 切在 I/O。
- **改写列表：** `flow,getTableInfo,initForCalcTableFit,checkRowNeedStartFit,afterForCalTableFit,processCardInfo,flowNormalTableRow,flowWrapTable,flowRows,getAllowOverCoreHeightForWrap`
- 列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION … */` 停车原模版体。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| hasTablePos | wr-27 01 | 探测 | loop 01 | — | 禁止改探测语义 |
| TableRowLayouter | table | 行高 | 本轮生成模板 | 非 flow 方法停车 | 禁止 stub |
| 单元格故事 | wc-04 | cell TextDocument | 是 | 绕排表 | 禁止改 parse |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| flow | TableLayouter.java:100 | mHasBreakRow | 提交表行 | wrap vs normal |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| TF1 | 单行表 | 公开入口 | 是 | TF1 |
| TF2 | mHasBreakRow | 状态转换 | 是 | TF2 |
| TF3 | wrap 表 | 关键分支 | 是 | TF3 |
| TF4 | 空表 | 失败 | 是 | TF4 |

### 3.3 可测试性
fixture / 内存字节 / 假上游输出；不启动全管线。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| TF1 | TF1 | 1×1 表 | flow | 至少一行 Typo | 0 行假绿 |
| TF2 | TF2 | break row | flow(true) | 续排 | 重排首行 |
| TF3 | TF3 | wrap | flowWrapTable | wrap 路径 | 走 normal |
| TF4 | TF4 | 无 row | flow | 不 panic | crash |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/table/TableLayouter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/table/02_function_table_flow.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_table_flow

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_table_flow \
  --src src/cn/wps/moffice/writer/layout/base/table/TableLayouter.rs \
  --report libs/word-core/layout/features/wr-27-table-layout/functions/02_function_table_flow/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_table_flow --json`)
date: 2026-09-06 10:13

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/table/TableLayouter.rs` | 119/287 | 41.5% | 211/551 | 38.3% |
| **function total** | **119/287** | **41.5%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/table/TableLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/table/TableLayouter.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/table/TableLayouter.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/table/TableLayouter.rs \
  --report libs/word-core/layout/features/wr-27-table-layout/functions/02_function_table_flow/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-06 02:13

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TableLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/table/TableLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/table/TableLayouter.rs` | **PASS** | `TableLayouter`, `dispose`, `initTableRowLayouter`, `hasTablePos`, `flow`, `initForCalcTableFit`, `checkRowNeedStartFit`, `afterForCalTableFit`, `getTableInfo`, `getAllowOverCoreHeightForWrap`, `flowNormalTableRow`, `flowWrapTable`, `flowRows`, `processCardInfo` | `reset`, `getAllowOverCoreHeightForNormal`, `shouldCutWrapTableIfExceeded`, `hasLinesOfDrawing`, `getTableRowForAnchor`, `setIsFirstRowOfTable`, `getWrapTableDrawing`, `getEmbedMinFirstLine`, `checkOverlapToFootNote`, `getFloatCotainerPage`, `isOverFlow`, `calJCWidth`, `adjustTableMargin`, `getWrapTableHeight`, `postFlowWrapTableRow`, `postFlowNormalTableRow`, `onAdjustDrawing` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
