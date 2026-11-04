# 03_function_sel_fill — 发 selection 指令

| 项 | 值 |
|----|----|
| feature | wr-19-selection |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SelectionFill::overlay(frame, start, end) -> Vec<RenderCommand>` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
`kind = "selection"`；颜色常量 `SELECTION_COLOR`。Rect **插在** 原 commands 之前（先填后画字）。空选不插。**改写列表：** `overlay`、`emit`。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| 01 clamp | wr-19 | Option 区间 | 本轮 | — | — |
| 02 rects | wr-19 | 几何 | 本轮 | — | — |
| DrawRect | wr-07 | page/x/y/w/h/kind/color | 是 | — | 改 tag |

## 2. 核心代码
`api/SelectionFill.rs`。`KIND_SELECTION = "selection"`。

## 3. 测试方案
### 3.2
| id | 场景 | 必测 | test |
|---|---|---|---|
| SF-KIND | overlay [1,3) | 是 | SF1 |
| SF-ORDER | Rect 在 Text 前 | 是 | SF2 |
| SF-EMPTY | 空选 | 是 | SF3 |
| SF-COLOR | 颜色常量 | 是 | SF4 |

### 3.4
SF1 一条 Rect kind=selection；SF2 第一条为 Rect；SF3 commands 条数不变；SF4 color==SELECTION_COLOR。
### 3.5
src: `.../api/SelectionFill.rs`  
test: `.../tests/.../SelectionFill.rs`
### 3.6
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_sel_fill
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_sel_fill \
  --src src/cn/wps/moffice/writer/api/SelectionFill.rs \
  --report libs/word-core/layout/features/wr-19-selection/functions/03_function_sel_fill/FUNCTION.md
```
### 3.7
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_sel_fill --json`)
date: 2026-08-27 10:35

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/SelectionFill.rs` | 33/34 | 97.1% | 45/46 | 97.8% |
| **function total** | **33/34** | **97.1%** |  |  |
<!-- coverage-report:end -->
### 3.8
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java
```
### 3.9
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs \
  --report libs/word-core/layout/features/wr-19-selection/functions/03_function_sel_fill/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-27 02:35

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoTextLine` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` | **PASS** | `setPara`, `getPara`, `setTextRectTop`, `getTextRectTop`, `setTextRectHeight`, `setAscDesPos`, `getTextRectHeight`, `getDescentPos`, `getAscentPos`, `setFlags`, `getFlags`, `getGraphIndexStart`, `setGraphIndexStart`, `getGraphIndexEnd`, `setGraphIndexEnd`, `setYPos`, `getYPos`, `isLastLineOfPara`, `setFirstLineOfPara`, `setLastLineOfPara`, `setLastLineOfPage`, `isFirstLineOfPara`, `isLastLineOfPage`, `getStartCP`, `getEndCP`, `contains` | `checkClassType`, `getType`, `alloc`, `isPointer`, `getRelTextRectTop`, `getTextRectBottom`, `offsetTextRectTop`, `getGraphLength`, `getEndChar`, `hasSHDs`, `getSHDs`, `setSHDs`, `getBorders`, `setBorders`, `getHighlights`, `setHighlights`, `getDocFiedls`, `getDocFields`, `setDocFields`, `getLines`, `setLines`, `isFirstLineOfMathPara`, `setIsInMathPara`, `setIsFirstLineOfMathPara`, `setIsLastLineOfMathPara`, `setFixedHeight`, `isFixedHeight`, `setExtendLastSHDAndBrc`, `isExtendLastSHDAndBrc`, `isLastLineOfSection`, `setLastLineOfSection`, `setLastLineOfColumn`, `setRTL`, `hasComment`, `setHasComment`, `hasEmbedsDrawing`, `setHasEmbedsDrawing`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `isTemplateContent`, `setIsTemplateContent`, `hasAutoNumber`, `setHasAutoNumber`, `setHasSubLine`, `hasSubLine`, `setHasBidi`, `hasBidi`, `isInMathPara`, `isLastLineOfMathPara`, `isLastLineOfColumn`, `getFirstValidCp`, `getCPRef`, `setSHDAndBRCs`, `getSHDAndBRCs`, `isRTL`, `setMemEnd`, `getMemEnd`, `rectForPageCore`, `newInstance`, `newObjecctFactory`, `commitOp`, `toString`, `getLineChars`, `isCardBegin`, `setCardBegin`, `isCardEnd`, `setCardEnd`, `setLineBefore`, `getLineBefore`, `setLineAfter`, `getLineAfter`, `setLineBalloonPageHeight`, `getLineBalloonPageHeight` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
