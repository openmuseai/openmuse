# 01_function_wrap_maxwidth — 绕排后行 maxWidth

| 项 | 值 |
|----|----|
| feature | wr-28-drawing-wrap |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SubLineEnv layoutLeft/layoutRight 或 wrap rect → 行宽` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
绕排后行 maxWidth

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
- **改写列表：** `adjustFloatRect` 相关行宽；列表外停车
- 列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION … */` 停车原模版体。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| SubLineEnv | line | layoutLeft/Right | 模板在 | 不改 LineLayouter j2r_ty | 禁止改 LineLayouter 壳 |
| Drawing 锚点 | cache | wrap 类型 | 部分 | 复杂绕排算法停车须标 deferred | 禁止 stub DrawingLayouter |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| wrap width | DrawingLayouter | wrap rect | maxWidth < 页宽 | 无图 |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| WW1 | 左绕排缩行宽 | 公开入口 | 是 | WW1 |
| WW2 | 无图整页宽 | 边界 | 是 | WW2 |
| WW3 | 双图夹文 | 关键分支 | 是 | WW3 |

### 3.3 可测试性
fixture / 内存字节 / 假上游输出；不启动全管线。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| WW1 | WW1 | 左 wrap | maxWidth < page | 仍满宽 |
| WW2 | WW2 | 无 drawing | 满内容宽 | 误缩 |
| WW3 | WW3 | 左右 wrap | 中间条带 | 0 宽 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/drawing/01_function_wrap_maxwidth.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_wrap_maxwidth

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_wrap_maxwidth \
  --src src/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.rs \
  --report libs/word-core/layout/features/wr-28-drawing-wrap/functions/01_function_wrap_maxwidth/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_wrap_maxwidth --json`)
date: 2026-09-06 14:44

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.rs` | 98/524 | 18.7% | 161/945 | 17.0% |
| **function total** | **98/524** | **18.7%** |  |  |
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
  --report libs/word-core/layout/features/wr-28-drawing-wrap/functions/01_function_wrap_maxwidth/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-06 06:43

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `DrawingLayouter` | `../../koffice/writer/core/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.java` | `src/cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.rs` | **PASS** | `DrawingLayouter`, `onFlowWrapDrawing`, `correctIfWrapShapeOutOfPage`, `adjustFloatRect`, `doOffsetDrawing`, `calcAffectRect`, `doCalcAffectRect`, `calcLimitRect`, `calcAlignRect`, `getFloatContainerPage` | `adjustTextFrame`, `adjustDropCap`, `adjustDropCapHorizontal`, `adjustDropCapVertical`, `offsetMarginToEdge`, `adjustWrapTable`, `getTablePos`, `correctForCellMargin`, `onStart`, `onEnd`, `correctIfWrapTableOutOfPage`, `getAlignRectRelColumnHoriOffset`, `doCalcAlignRectLeftAndRight`, `calcAlignRectLeftAndRightWithPage`, `calcAlignRectTopAndBottomWithPage`, `doCalcAlignRectTopAndBottom`, `calcTextBoxFitRect`, `flowChildShapes`, `createTextboxLayouter`, `getTextBoxLayoutState`, `flowTextBox`, `vAlignTextBox`, `dispose`, `getAnchorHoriPos`, `getEmbedContainerPage`, `getLineContainerPage`, `getHoriAlignPage`, `getVertAlignPage`, `addDrawings`, `getEmbed`, `getFloatDrawing`, `doAddFloatCache`, `needFlow`, `changeShapePos`, `doPreFlowDrawing`, `doPreFlowEmbedDrawing`, `setDrawingShapeRect`, `setDrawingRenderRect`, `setDrawingLayoutRect`, `addDrawingToPage`, `filterFlowDrawing`, `getFilterFloatCache`, `doPostFlowDrawings`, `flowBeforeCP`, `flowForTableRow`, `doFlow`, `getPageSetup` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
