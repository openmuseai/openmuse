# 02_function_flow_to_page — flowToPage

| 项 | 值 |
|----|----|
| feature | wr-23-pageview-main |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `PageViewLayouter.flowToPage 最小集` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
对页调用 TextLayouter.flow 直到 complete；\f 分页。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | PageViewLayouter + TextLayouter.flow |

### 1.3 输出
见 §1.1

### 1.4 上游
上游见 FUNCTIONS.md

### 1.5 下游
下游见 FUNCTIONS.md

### 1.6 不变量与函数改写列表
- 改写列表：`PageViewLayouter::flowToPage`（Java 532–568）：`pageIndex<1` → -1；循环 `flowPage` 直到 snapshot 页数或 `isComplete`。
- deferred：`swapToLocal`/`onScroll` 视口、脚注 `rollbackLines`。
- 本波测「排满 N 页」可用 `flowDocument` 等价：反复 `flowPage` 直到 complete（`MainLayouter.flowDocument` 126–160）。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `TextLayouter.flow` | wr-22 | 一页行流 | 否 | M13 | 禁止本 function 再写 greedy wrap |
| `processPagination` / widow 197 | wr-13 | 页满 | env 壳 | Java 热路径 | 禁止改 wr-05 高度累加冒充 |
| TypoSnapshot 页表 | wr-14 | layoutPageSize | 叶子 | 产品 IR | — |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出 | 关键分支 |
|---|---|---|---|---|
| PageViewLayouter.flowToPage 最小集 | PageViewLayouter + TextLayouter.flow | 见矩阵 | 见矩阵 | 见覆盖矩阵 |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|---|---|---|---|
| TP1 | Hello | 是 | TP1 |
| TP2 | \f | 是 | TP2 |
| TP3 | widow 197 | 是 | TP3 |

### 3.3 可测试性
fixture / 假 PropertySet / 假 LayoutState；不跑无关管线。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| TP1 | TP1 | Hello | 1 页 | 0 页 |
| TP2 | TP2 | \f | 2 页 | 软换行 |
| TP3 | TP3 | widow 197 | Java 页界 | 高度累加 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/pageview/PageViewLayouter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/pageview/PageViewLayouter.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_flow_to_page

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_flow_to_page \
  --src src/cn/wps/moffice/writer/layout/pageview/PageViewLayouter.rs \
  --report libs/word-core/layout/features/wr-23-pageview-main/functions/02_function_flow_to_page/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_flow_to_page --json`)
date: 2026-09-04 12:22

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/pageview/PageViewLayouter.rs` | 76/197 | 38.6% | 143/291 | 49.1% |
| **function total** | **76/197** | **38.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/pageview/PageViewLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/pageview/PageViewLayouter.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/pageview/PageViewLayouter.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/pageview/PageViewLayouter.rs \
  --report libs/word-core/layout/features/wr-23-pageview-main/functions/02_function_flow_to_page/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-04 04:22

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PageViewLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/pageview/PageViewLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/pageview/PageViewLayouter.rs` | **PASS** | `flowPage`, `flowToPage`, `isComplete`, `getType`, `swapToLocal`, `swapFromLocal` | `PageViewLayouter`, `dispose`, `preFlow`, `checkAndSetMirrorMaringOffset`, `hasEffectContent`, `postFlow`, `changePageSize`, `ensureValidEndPage`, `initLayouters`, `initTableLayouter`, `initDrawingLayouter`, `initFootEndNoteLayouter`, `createLayoutState`, `finishPage`, `needFlowRestFootnote`, `flowCP`, `hasFlowCP`, `prepareToJumpToLastView`, `jumpForGrid`, `flowTextFrame`, `rollbackLine`, `restartSectionLayouter`, `rollbackLines`, `onNewRangeEnd`, `doAfterFlowLine`, `getFootEndNoteLayouter`, `prepareFlowFromDocumentBegin`, `prepareFlowInScrollDirection`, `prepareFlowToDocumentEnd`, `acceptFirstCache`, `onModifyFirst`, `processEvents`, `getEditor`, `flowTextLine`, `doFlowLine`, `processPageBreakBeforePara`, `flowColumns`, `clearTypoDocument`, `reset`, `resetForInit`, `doOnIoLoadFinish`, `onAfterFirstRender`, `clearFootNoteRefCPs`, `clearDrawing`, `makeBalloonCaches`, `flowDocument`, `rollBackDropCap` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
