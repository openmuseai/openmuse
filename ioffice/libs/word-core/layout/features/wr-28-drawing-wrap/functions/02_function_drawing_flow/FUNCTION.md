# 02_function_drawing_flow — DrawingLayouter 锚点页

| 项 | 值 |
|----|----|
| feature | wr-28-drawing-wrap |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `DrawingLayouter::onFlowWrapDrawing` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
DrawingLayouter 锚点页

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | `cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.java` |

### 1.3 输出
见 §1.1 与用例表。

### 1.4 上游
声明的上游 fixture / 假 PropertySet；不跑无关管线。

### 1.5 下游
见 FUNCTIONS.md。

### 1.6 不变量与函数改写列表
- 切在 I/O。
- **改写列表：** `onFlowWrapDrawing,onStart,onEnd`
- 列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION … */` 停车原模版体。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| wrap maxWidth | wr-28 01 | 行宽 | loop 01 | — | 禁止改 01 语义 |
| TypoDrawing | cache | 锚点 | 是 | TextFrame 全文 | 禁止 stub |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| onFlowWrapDrawing | DrawingLayouter | anchorCP | 页上几何 | 空锚 |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| DF1 | 锚点 CP | 公开入口 | 是 | DF1 |
| DF2 | 页外校正 | 关键分支 | 是 | DF2 |
| DF3 | onStart/onEnd | cleanup | 是 | DF3 |

### 3.3 可测试性
fixture / 内存字节 / 假上游输出；不启动全管线。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| DF1 | DF1 | 合法锚点 | 有页矩形 | 丢 drawing |
| DF2 | DF2 | 超出页 | correctIfWrapShapeOutOfPage | 夹回页 | 飞出 |
| DF3 | DF3 | start+end | 不泄漏 | panic |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_drawing_flow

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_drawing_flow \
  --src src/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.rs \
  --report libs/word-core/layout/features/wr-28-drawing-wrap/functions/02_function_drawing_flow/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_drawing_flow --json`)
date: 2026-09-06 14:45

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.rs` | 90/531 | 16.9% | 148/964 | 15.4% |
| **function total** | **90/531** | **16.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.rs \
  --report libs/word-core/layout/features/wr-28-drawing-wrap/functions/02_function_drawing_flow/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-06 06:45

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `DrawingLayouter` | `../../koffice/writer/core/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.java` | `src/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.rs` | **PASS** | `DrawingLayouter`, `onFlowWrapDrawing`, `onStart`, `onEnd`, `correctIfWrapShapeOutOfPage`, `adjustFloatRect`, `doOffsetDrawing`, `calcAffectRect`, `doCalcAffectRect`, `calcLimitRect`, `calcAlignRect`, `getFloatContainerPage` | `adjustTextFrame`, `adjustDropCap`, `adjustDropCapHorizontal`, `adjustDropCapVertical`, `offsetMarginToEdge`, `adjustWrapTable`, `getTablePos`, `correctForCellMargin`, `correctIfWrapTableOutOfPage`, `getAlignRectRelColumnHoriOffset`, `doCalcAlignRectLeftAndRight`, `calcAlignRectLeftAndRightWithPage`, `calcAlignRectTopAndBottomWithPage`, `doCalcAlignRectTopAndBottom`, `calcTextBoxFitRect`, `flowChildShapes`, `createTextboxLayouter`, `getTextBoxLayoutState`, `flowTextBox`, `vAlignTextBox`, `dispose`, `getAnchorHoriPos`, `getEmbedContainerPage`, `getLineContainerPage`, `getHoriAlignPage`, `getVertAlignPage`, `addDrawings`, `getEmbed`, `getFloatDrawing`, `doAddFloatCache`, `needFlow`, `changeShapePos`, `doPreFlowDrawing`, `doPreFlowEmbedDrawing`, `setDrawingShapeRect`, `setDrawingRenderRect`, `setDrawingLayoutRect`, `addDrawingToPage`, `filterFlowDrawing`, `getFilterFloatCache`, `doPostFlowDrawings`, `flowBeforeCP`, `flowForTableRow`, `doFlow`, `getPageSetup` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
