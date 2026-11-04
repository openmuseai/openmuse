# 02_function_do_flow_line — doFlowLine 正文

| 项 | 值 |
|----|----|
| feature | wr-22-text-flow |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `TextLayouter::doFlowLine` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
start ParaLayouter；正文 flowTextLine；table/frame DEFERRED。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | TextLayouter.java:262–275 |

### 1.3 输出
见 §1.1

### 1.4 上游
上游见 FUNCTIONS.md

### 1.5 下游
下游见 FUNCTIONS.md

### 1.6 不变量与函数改写列表
- 切在 I/O\n- 改写列表：本入口；其余 DEFERRED_NON_FUNCTION

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| 上游 feature | 见计划 | 声明的 O | 见 overview | 本 function 不改 | — |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出 | 关键分支 |
|---|---|---|---|---|
| TextLayouter::doFlowLine | TextLayouter.java:262–275 | 见矩阵 | 见矩阵 | 见覆盖矩阵 |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|---|---|---|---|
| DF1 | 正文 | 是 | DF1 |
| DF2 | 新段 | 是 | DF2 |
| DF3 | 表 PAPX | 是 | DF3 |
| DF4 | textFrame | 是 | DF4 |

### 3.3 可测试性
fixture / 假 PropertySet / 假 LayoutState；不跑无关管线。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| DF1 | DF1 | 正文 | 产出行 | greedy |
| DF2 | DF2 | 新段 | 先 onStart | 缺段距 |
| DF3 | DF3 | 表 PAPX | defer | 当正文 |
| DF4 | DF4 | textFrame | defer | 当正文 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/TextLayouter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/TextLayouter.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_do_flow_line

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_do_flow_line \
  --src src/cn/wps/moffice/writer/layout/base/TextLayouter.rs \
  --report libs/word-core/layout/features/wr-22-text-flow/functions/02_function_do_flow_line/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_do_flow_line --json`)
date: 2026-09-04 10:08

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/TextLayouter.rs` | 36/203 | 17.7% | 48/260 | 18.5% |
| `src/cn/wps/moffice/writer/layout/base/graph/GraphReaderTool.rs` | 17/51 | 33.3% | 30/86 | 34.9% |
| **function total** | **53/254** | **20.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/TextLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/TextLayouter.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/TextLayouter.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/TextLayouter.rs \
  --report libs/word-core/layout/features/wr-22-text-flow/functions/02_function_do_flow_line/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-04 02:08

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TextLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/TextLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/TextLayouter.rs` | **PASS** | `TextLayouter`, `preFlow`, `doFlowLine`, `isInTextFrame` | `createLayoutState`, `init`, `initLayouters`, `initTableLayouter`, `initDrawingLayouter`, `initTextFrameLayouter`, `initFootEndNoteLayouter`, `getDrawingLayouter`, `dispose`, `flow`, `isComplete`, `postFlow`, `clearDrawing`, `getLastValidLineInPage`, `doAfterFlowLine`, `rollbackTrailingHiddenParagraphs`, `clearRedundantEmbeds`, `processPagination`, `processMathPara`, `flowLine`, `initGraphBuff`, `rollBackDropCap`, `flowTextFrame`, `flowTable`, `needReflowLine`, `doFlowTextLine`, `hasGraphs`, `flowTextLine`, `doFlowDrawings`, `shouldFlowDrawingsBeforeFinishPage`, `prepareGraphs`, `initSmartFontSize`, `onEmptyGraphSize`, `rollbackLines`, `rewindLineIteratorBeforeRollback`, `doRollbackLines`, `clearFootNoteRefCPs`, `rollbackLine`, `isLastLineOfPage`, `finishPage`, `shouldInvalidateRedundantWrapDrawing`, `doClearRedundantDrawings`, `getMaxWrapDrawingCP`, `getLastValidLineBeforeClearDrawings`, `findMaxRedundantWrapDrawingCP`, `clearRedundantDrawings`, `getRedundantDrawingLimitCP`, `collectPageRange`, `makeSnapshot`, `loadSnapshot`, `flowFootEndNotes`, `needFinishPageForFootNotes`, `flowTextLineForCheckWidow`, `prepareEmbeds`, `endParaLayouter`, `adjustWrapTable`, `postFlowTableHeader`, `resolveTraceTableIndex` | — | **PASS** |
| `GraphReaderTool` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/GraphReaderTool.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphReaderTool.rs` | **PASS** | `isInTable`, `isInTextFrame` | `isVanishedText`, `isVanishedOrRevisionDeleted` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
