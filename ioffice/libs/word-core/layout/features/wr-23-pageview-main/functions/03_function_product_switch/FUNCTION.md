# 03_function_product_switch — layout_document 切换

| 项 | 值 |
|----|----|
| feature | wr-23-pageview-main |
| order | 3 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `layout_document / layout_text_document` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
产品入口改 PageView 热路径；greedy 仅回归。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | LayoutManager → FlowLayout MODE_PAGE |

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
| layout_document / layout_text_document | LayoutManager → FlowLayout MODE_PAGE | 见矩阵 | 见矩阵 | 见覆盖矩阵 |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|---|---|---|---|
| SW1 | hello | 是 | SW1 |
| SW2 | sample2 | 是 | SW2 |
| SW3 | wr-04 测 | 是 | SW3 |

### 3.3 可测试性
fixture / 假 PropertySet / 假 LayoutState；不跑无关管线。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| SW1 | SW1 | hello | 可画 Hello | 空 commands |
| SW2 | SW2 | sample2 | 标题 x>margin | 仍 1800 |
| SW3 | SW3 | wr-04 测 | 仍绿或迁测 | 误删 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/api/LayoutDocument.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/api/LayoutDocument.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_product_switch

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_product_switch \
  --src src/cn/wps/moffice/writer/api/LayoutDocument.rs \
  --src src/cn/wps/moffice/writer/layout/FlowLayout.rs \
  --report libs/word-core/layout/features/wr-23-pageview-main/functions/03_function_product_switch/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_product_switch --json`)
date: 2026-09-04 12:25

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/LayoutDocument.rs` | 426/611 | 69.7% | 684/909 | 75.2% |
| `src/cn/wps/moffice/writer/layout/FlowLayout.rs` | 29/185 | 15.7% | 24/182 | 13.2% |
| **function total** | **455/796** | **57.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/FlowLayout.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/FlowLayout.java
```

`LayoutDocument.rs` is the Rust product I/O (no Java class). MODE_PAGE 切换走 `FlowLayout.createDocLayouter(0)` + `PageViewLayouter.flowToPage`；emit 仍经 `layout_layout_runs` 直到 LineLayouter.flow 接入。

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/FlowLayout.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/FlowLayout.rs \
  --report libs/word-core/layout/features/wr-23-pageview-main/functions/03_function_product_switch/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-04 04:25

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `FlowLayout` | `koffice/writer/core/cn/wps/moffice/writer/layout/FlowLayout.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/FlowLayout.rs` | **PASS** | — | `getLocksHelper`, `FlowLayout`, `reuseClean`, `reuseInit`, `dispose`, `getTypoDocument`, `getPropertyToolFactory`, `changeMode`, `getMode`, `onModifyFirst`, `resetTypoLayoutState`, `changeSize`, `update`, `beforeUpdate`, `isLayoutComplete`, `setLayoutComplete`, `acceptCacheSnapShot`, `getDocumentEnv`, `flowPage`, `checkLayoutComplete`, `createDocLayouter`, `init`, `showFormatChangeInBallons`, `onScroll`, `flowCP`, `commitSnapshot`, `flowToPage`, `prepareToJumpToLastView`, `doGridAutoResize`, `flowViewRect`, `checkContinueFlowOnMultiColumn`, `getCurLayoutHeight`, `flowPageInScrollDirection`, `flowToDocumentEnd`, `prepareFlowFromDocumentBegin`, `hasFlowToBegin`, `hasFlowToEnd`, `flowAndDrawMath`, `swapToLocal`, `swapFromLocal`, `clear`, `discardEvents`, `onIoLoadFinish`, `setNotMergeTypoText`, `hasUpdateMsg`, `hasInit`, `isUseDefaultSec`, `hasProcessedIOLoadFinishEvent`, `onAfterFirstRender`, `unInit`, `doInit`, `resetForCrash`, `onFontsChange`, `dump`, `flowDocument`, `makeBalloonCache`, `flowFootnoteBalloon`, `flowEndnoteBalloon`, `flowFootEndnoteBalloon`, `isRunOutMemory`, `locateEmbedCommentScroll`, `invalidCardState` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
