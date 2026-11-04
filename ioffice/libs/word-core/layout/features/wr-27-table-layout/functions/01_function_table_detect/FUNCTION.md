# 01_function_table_detect — PAPX 223 表探测

| 项 | 值 |
|----|----|
| feature | wr-27-table-layout |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `TableLayouter::hasTablePos` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
PAPX 223 表探测

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
- **改写列表：** `TableLayouter,hasTablePos`
- 列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION … */` 停车原模版体。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `TableInfo.getPos` / `DrawingCoordTool.isVertTextflow` | 本 function | wrap 位 + 竖排页 | 是（本文件局部类型对齐 Java） | crate DrawingCoordTool 空壳 | 禁止改 PAPX SAX |
| `IStateEnvoy` | 本 function 局部 trait | getDocument/getCurPage | 是 | flow 路径 | 禁止改 TextLayouter.flowTable 本轮 |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| hasTablePos | TableLayouter.java:91 | mTable.getPos / doc type+vert | bool | pos 空、type 非 0/2、vert |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| TD1 | getPos()!=null | 公开入口 | 是 | TD1 |
| TD2 | 无 pos 且 type∉{0,2} | 失败 | 是 | TD2 |
| TD3 | 无 pos、type=0、vert | 关键分支 | 是 | TD3 |

### 3.3 可测试性
fixture / 内存字节 / 假上游输出；不启动全管线。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| TD1 | TD1 | wrap pos | hasTablePos | true | false |
| TD2 | TD2 | 正文 type=1 | hasTablePos | false | true |
| TD3 | TD3 | type=0 vert | hasTablePos | true | false |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/table/TableLayouter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/table/TableLayouter.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_table_detect

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_table_detect \
  --src src/cn/wps/moffice/writer/layout/base/table/TableLayouter.rs \
  --report libs/word-core/layout/features/wr-27-table-layout/functions/01_function_table_detect/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_table_detect --json`)
date: 2026-09-06 09:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/table/TableLayouter.rs` | 18/84 | 21.4% | 48/121 | 39.7% |
| **function total** | **18/84** | **21.4%** |  |  |
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
  --report libs/word-core/layout/features/wr-27-table-layout/functions/01_function_table_detect/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-06 01:58

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TableLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/table/TableLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/table/TableLayouter.rs` | **PASS** | `TableLayouter`, `dispose`, `hasTablePos` | `reset`, `initTableRowLayouter`, `flow`, `initForCalcTableFit`, `checkRowNeedStartFit`, `afterForCalTableFit`, `getTableInfo`, `getAllowOverCoreHeightForNormal`, `getAllowOverCoreHeightForWrap`, `shouldCutWrapTableIfExceeded`, `flowNormalTableRow`, `hasLinesOfDrawing`, `flowWrapTable`, `getTableRowForAnchor`, `setIsFirstRowOfTable`, `getWrapTableDrawing`, `getEmbedMinFirstLine`, `checkOverlapToFootNote`, `getFloatCotainerPage`, `isOverFlow`, `calJCWidth`, `flowRows`, `processCardInfo`, `adjustTableMargin`, `getWrapTableHeight`, `postFlowWrapTableRow`, `postFlowNormalTableRow`, `onAdjustDrawing` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
