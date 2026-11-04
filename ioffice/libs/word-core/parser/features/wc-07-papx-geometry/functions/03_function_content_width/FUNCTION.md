# 03_function_content_width — 页宽−边距−左右缩进

| 项 | 值 |
|----|----|
| feature | wc-07-papx-geometry |
| order | 3 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `ContentWidth::paraMaxWidth` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
210/212 纳入行可用宽；再叠加首行 208。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | ParaLayouter left/right 后 LineLayouter.maxWidth |

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
| ContentWidth::paraMaxWidth | ParaLayouter left/right 后 LineLayouter.maxWidth | 见矩阵 | 见矩阵 | 见覆盖矩阵 |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|---|---|---|---|
| CW1 | 仅左 210 | 是 | CW1 |
| CW2 | 仅右 212 | 是 | CW2 |
| CW3 | 无缩进 | 是 | CW3 |
| CW4 | +firstLine | 是 | CW4 |

### 3.3 可测试性
fixture / 假 PropertySet / 假 LayoutState；不跑无关管线。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| CW1 | CW1 | 仅左 210 | 内容宽减少 | 忽略 210 |
| CW2 | CW2 | 仅右 212 | 减少 | 忽略 |
| CW3 | CW3 | 无缩进 | 等于页内容宽 | 误扣 |
| CW4 | CW4 | +firstLine | 与 Java 同行宽 | 扣两次 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/ContentWidth.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/ContentWidth.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_content_width

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_content_width \
  --src src/cn/wps/moffice/writer/layout/base/env/ContentWidth.rs \
  --report libs/word-core/parser/features/wc-07-papx-geometry/functions/03_function_content_width/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_content_width --json`)
date: 2026-09-03 17:32

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/ContentWidth.rs` | 13/14 | 92.9% | 15/16 | 93.8% |
| **function total** | **13/14** | **92.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java
```
本 function 产品 I/O 在 `ContentWidth::paraMaxWidth`（210/212）。LineLayouter 本 loop 不改写。

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --report libs/word-core/parser/features/wc-07-papx-geometry/functions/03_function_content_width/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-03 09:32

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `LineLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs` | **PASS** | — | `LineLayouter`, `initLineHeightLayouter`, `doInitTextWrapLayouter`, `initTextWrapLayouter`, `onAddWrapDrawing`, `clearTextWrapLayouter`, `getTextWrapLayouter`, `dispose`, `needReflowLine`, `flow`, `processCardInfo`, `isSpaceAfterSoftBreak`, `hasParaBreakIntention`, `isSecBreakNoContinue`, `compress`, `doGetCompressFactor`, `isOverflowBeforeFlow`, `getHAlignHandler`, `getVAlignHandler`, `getDocFieldCollect`, `getGridHandler`, `getCharSpaceHandler`, `getAutoSpaceHandler`, `getPunctSpaceHandler`, `getKerningHandler`, `getCharBorderHandler`, `isOverflow`, `prepareSubLineEnvs`, `preCalcLineHeight`, `postCalcLineHeight`, `calcParaBefore`, `calcParaAfter`, `regulateToTextLine`, `regulateToCache`, `getParaBrcLayouter`, `compressEmptyLine`, `initCollectors`, `regulateSubLinesAndRuns`, `regulateSubLine`, `adjustEmbedRun`, `setLineHeight`, `setAlignLine`, `setAscDesPos`, `setForceVCenterAlign` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
