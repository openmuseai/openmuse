# 02_function_sel_rects — xpos 切片成框

| 项 | 值 |
|----|----|
| feature | wr-19-selection |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SelectionRects::rects_for_range(frame, lo, hi)` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
按命令序累加可见 `DrawText` 标量 cp。求交后 `x=xpos[i0]`，右缘 `xpos[i1]` 或末字宽（末对差分）。`y = baseline − size`，`h = size`。**改写列表：** `rects_for_range`。禁止改 `LocateTool`。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| DrawText.xpos | wr-07/18 | 页绝对 x | 是 | 无 xpos 的旧金 | emit |
| LineSelecitonRect | Java | left/right/top/bottom | 否 | 整类 java2rust | 从零手写 |

## 2. 核心代码
`api/SelectionRects.rs`。Java：`setSelection(left, top, right, bottom)`。

## 3. 测试方案
### 3.1
Hello Frame：xpos 1800..2200 step 100，size 240，y 1632。
### 3.2
| id | 场景 | 必测 | test |
|---|---|---|---|
| SX-ALL | [0,5) | 是 | SX1 |
| SX-MID | [1,3) | 是 | SX2 |
| SX-EMPTY | lo==hi | 是 | SX3 |
| SX-MISS | 区间在 run 外 | 是 | SX4 |
| SX-Y | 行顶 | 是 | SX5 |

### 3.3 内存 Frame，无 zip。
### 3.4
SX1 x=1800 w=500（末字宽 100）；SX2 x=1900 w=200；SX3 空；SX4 空；SX5 y=1392 h=240。
### 3.5
src: `.../api/SelectionRects.rs`  
test: `.../tests/.../SelectionRects.rs`
### 3.6
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_sel_rects
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_sel_rects \
  --src src/cn/wps/moffice/writer/api/SelectionRects.rs \
  --report libs/word-core/layout/features/wr-19-selection/functions/02_function_sel_rects/FUNCTION.md
```
### 3.7
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_sel_rects --json`)
date: 2026-08-27 10:35

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/SelectionRects.rs` | 37/41 | 90.2% | 71/76 | 93.4% |
| **function total** | **37/41** | **90.2%** |  |  |
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
  --report libs/word-core/layout/features/wr-19-selection/functions/02_function_sel_rects/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-27 02:35

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoTextLine` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` | **PASS** | `setPara`, `getPara`, `setTextRectTop`, `getTextRectTop`, `setTextRectHeight`, `setAscDesPos`, `getTextRectHeight`, `getDescentPos`, `getAscentPos`, `setFlags`, `getFlags`, `getGraphIndexStart`, `setGraphIndexStart`, `getGraphIndexEnd`, `setGraphIndexEnd`, `setYPos`, `getYPos`, `isLastLineOfPara`, `setFirstLineOfPara`, `setLastLineOfPara`, `setLastLineOfPage`, `isFirstLineOfPara`, `isLastLineOfPage`, `getStartCP`, `getEndCP`, `contains` | `checkClassType`, `getType`, `alloc`, `isPointer`, `getRelTextRectTop`, `getTextRectBottom`, `offsetTextRectTop`, `getGraphLength`, `getEndChar`, `hasSHDs`, `getSHDs`, `setSHDs`, `getBorders`, `setBorders`, `getHighlights`, `setHighlights`, `getDocFiedls`, `getDocFields`, `setDocFields`, `getLines`, `setLines`, `isFirstLineOfMathPara`, `setIsInMathPara`, `setIsFirstLineOfMathPara`, `setIsLastLineOfMathPara`, `setFixedHeight`, `isFixedHeight`, `setExtendLastSHDAndBrc`, `isExtendLastSHDAndBrc`, `isLastLineOfSection`, `setLastLineOfSection`, `setLastLineOfColumn`, `setRTL`, `hasComment`, `setHasComment`, `hasEmbedsDrawing`, `setHasEmbedsDrawing`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `isTemplateContent`, `setIsTemplateContent`, `hasAutoNumber`, `setHasAutoNumber`, `setHasSubLine`, `hasSubLine`, `setHasBidi`, `hasBidi`, `isInMathPara`, `isLastLineOfMathPara`, `isLastLineOfColumn`, `getFirstValidCp`, `getCPRef`, `setSHDAndBRCs`, `getSHDAndBRCs`, `isRTL`, `setMemEnd`, `getMemEnd`, `rectForPageCore`, `newInstance`, `newObjecctFactory`, `commitOp`, `toString`, `getLineChars`, `isCardBegin`, `setCardBegin`, `isCardEnd`, `setCardEnd`, `setLineBefore`, `getLineBefore`, `setLineAfter`, `getLineAfter`, `setLineBalloonPageHeight`, `getLineBalloonPageHeight` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
