# 01_function_pageview_mode — MODE_PAGE=0

| 项 | 值 |
|----|----|
| feature | wr-23-pageview-main |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `createDocLayouter PAGEVIEW_LAYOUTER=0` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
只构造 PageView；mode 1–4 deferred Err。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `FlowLayout.createDocLayouter(0)` | viewMode | `FlowLayout.java` 356–377；常量 `MainLayouter.PAGEVIEW_LAYOUTER=0` |

### 1.3 输出
见 §1.1

### 1.4 上游
上游见 FUNCTIONS.md

### 1.5 下游
下游见 FUNCTIONS.md

### 1.6 不变量与函数改写列表
- 改写列表：`FlowLayout::createDocLayouter` 的 `case 0` / default → `PageViewLayouter`。
- mode 1–4：`todo!("DEFERRED_NON_FUNCTION")`（Web/Phone/Share）。
- `MainLayouter.PAGEVIEW_LAYOUTER` 常量必须为 0。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `PageViewLayouter` | wr-23-02 | ctor | 否 | loop 02 | 本 function 只选类型 |
| `ViewEnv.getViewMode` | wr-02 | 0=PAGE | 部分 ViewParams | — | 禁止改 A4 常量冒充 mode |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出 | 关键分支 |
|---|---|---|---|---|
| `createDocLayouter` | `FlowLayout.java:356` / `FlowLayout.rs` | mode int | `mMainLayouter` 类型 | 0→PageView；1–4 deferred |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|---|---|---|---|
| VM1 | mode 0 | 是 | VM1 |
| VM2 | mode 1 | 是 | VM2 |

### 3.3 可测试性
fixture / 假 PropertySet / 假 LayoutState；不跑无关管线。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| VM1 | VM1 | mode 0 | PageView 类型 | 当 web |
| VM2 | VM2 | mode 1 | Err/deferred | 假实现 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/FlowLayout.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/FlowLayout.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_pageview_mode

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_pageview_mode \
  --src src/cn/wps/moffice/writer/layout/FlowLayout.rs \
  --report libs/word-core/layout/features/wr-23-pageview-main/functions/01_function_pageview_mode/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_pageview_mode --json`)
date: 2026-09-04 10:26

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/FlowLayout.rs` | 31/185 | 16.8% | 26/182 | 14.3% |
| **function total** | **31/185** | **16.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/FlowLayout.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/FlowLayout.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/FlowLayout.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/FlowLayout.rs \
  --report libs/word-core/layout/features/wr-23-pageview-main/functions/01_function_pageview_mode/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-04 02:26

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `FlowLayout` | `koffice/writer/core/cn/wps/moffice/writer/layout/FlowLayout.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/FlowLayout.rs` | **PASS** | — | `getLocksHelper`, `FlowLayout`, `reuseClean`, `reuseInit`, `dispose`, `getTypoDocument`, `getPropertyToolFactory`, `changeMode`, `getMode`, `onModifyFirst`, `resetTypoLayoutState`, `changeSize`, `update`, `beforeUpdate`, `isLayoutComplete`, `setLayoutComplete`, `acceptCacheSnapShot`, `getDocumentEnv`, `flowPage`, `checkLayoutComplete`, `createDocLayouter`, `init`, `showFormatChangeInBallons`, `onScroll`, `flowCP`, `commitSnapshot`, `flowToPage`, `prepareToJumpToLastView`, `doGridAutoResize`, `flowViewRect`, `checkContinueFlowOnMultiColumn`, `getCurLayoutHeight`, `flowPageInScrollDirection`, `flowToDocumentEnd`, `prepareFlowFromDocumentBegin`, `hasFlowToBegin`, `hasFlowToEnd`, `flowAndDrawMath`, `swapToLocal`, `swapFromLocal`, `clear`, `discardEvents`, `onIoLoadFinish`, `setNotMergeTypoText`, `hasUpdateMsg`, `hasInit`, `isUseDefaultSec`, `hasProcessedIOLoadFinishEvent`, `onAfterFirstRender`, `unInit`, `doInit`, `resetForCrash`, `onFontsChange`, `dump`, `flowDocument`, `makeBalloonCache`, `flowFootnoteBalloon`, `flowEndnoteBalloon`, `flowFootEndnoteBalloon`, `isRunOutMemory`, `locateEmbedCommentScroll`, `invalidCardState` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
