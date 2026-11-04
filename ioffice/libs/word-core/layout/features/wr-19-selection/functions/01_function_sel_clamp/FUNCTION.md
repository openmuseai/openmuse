# 01_function_sel_clamp — 夹紧选区 cp

| 项 | 值 |
|----|----|
| feature | wr-19-selection |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SelectionRange::clamp(start, end, len) -> Option<(i32,i32)>` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
半开 `[lo,hi)`；`start>end` 对调；夹进 `[0,len]`；`lo==hi` → `None`。**改写列表：** `clamp`。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| 无存储 | — | 纯整数 | — | — | 禁止碰 rope |

## 2. 核心代码
Rust：`api/SelectionRange.rs`。Java：`RenderSetting.selectionStart/End` 空选语义，无独立类。

## 3. 测试方案
### 3.1
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| clamp | 有序 | Some | 主路径 |
| clamp | 逆序 | 对调 | 交换 |
| clamp | 负 start | 夹 0 | 边界 |
| clamp | end>len | 夹 len | 边界 |
| clamp | 空 | None | 失败 |
| clamp | len=0 | None | 空故事 |

### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| CL-OK | 2,5,10 | 是 | CL1 |
| CL-SWAP | 5,2,10 | 是 | CL2 |
| CL-NEG | -1,3,10 | 是 | CL3 |
| CL-PAST | 8,20,10 | 是 | CL4 |
| CL-EMPTY | 3,3,10 | 是 | CL5 |
| CL-ZERO | 0,1,0 | 是 | CL6 |

### 3.3 可测试性
纯函数，无 zip。
### 3.4
CL1 `(2,5)`；CL2 `(2,5)`；CL3 `(0,3)`；CL4 `(8,10)`；CL5 None；CL6 None。
### 3.5
src: `libs/word-core/src/cn/wps/moffice/writer/api/SelectionRange.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/api/SelectionRange.rs`
### 3.6
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_sel_clamp
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_sel_clamp \
  --src src/cn/wps/moffice/writer/api/SelectionRange.rs \
  --report libs/word-core/layout/features/wr-19-selection/functions/01_function_sel_clamp/FUNCTION.md
```
### 3.7
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_sel_clamp --json`)
date: 2026-08-27 10:35

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/SelectionRange.rs` | 12/12 | 100.0% | 19/19 | 100.0% |
| **function total** | **12/12** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8
`SelectionRange` 无 Java 模板。校验行几何（选区框所在行）：
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
  --report libs/word-core/layout/features/wr-19-selection/functions/01_function_sel_clamp/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-27 02:35

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoTextLine` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` | **PASS** | `setPara`, `getPara`, `setTextRectTop`, `getTextRectTop`, `setTextRectHeight`, `setAscDesPos`, `getTextRectHeight`, `getDescentPos`, `getAscentPos`, `setFlags`, `getFlags`, `getGraphIndexStart`, `setGraphIndexStart`, `getGraphIndexEnd`, `setGraphIndexEnd`, `setYPos`, `getYPos`, `isLastLineOfPara`, `setFirstLineOfPara`, `setLastLineOfPara`, `setLastLineOfPage`, `isFirstLineOfPara`, `isLastLineOfPage`, `getStartCP`, `getEndCP`, `contains` | `checkClassType`, `getType`, `alloc`, `isPointer`, `getRelTextRectTop`, `getTextRectBottom`, `offsetTextRectTop`, `getGraphLength`, `getEndChar`, `hasSHDs`, `getSHDs`, `setSHDs`, `getBorders`, `setBorders`, `getHighlights`, `setHighlights`, `getDocFiedls`, `getDocFields`, `setDocFields`, `getLines`, `setLines`, `isFirstLineOfMathPara`, `setIsInMathPara`, `setIsFirstLineOfMathPara`, `setIsLastLineOfMathPara`, `setFixedHeight`, `isFixedHeight`, `setExtendLastSHDAndBrc`, `isExtendLastSHDAndBrc`, `isLastLineOfSection`, `setLastLineOfSection`, `setLastLineOfColumn`, `setRTL`, `hasComment`, `setHasComment`, `hasEmbedsDrawing`, `setHasEmbedsDrawing`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `isTemplateContent`, `setIsTemplateContent`, `hasAutoNumber`, `setHasAutoNumber`, `setHasSubLine`, `hasSubLine`, `setHasBidi`, `hasBidi`, `isInMathPara`, `isLastLineOfMathPara`, `isLastLineOfColumn`, `getFirstValidCp`, `getCPRef`, `setSHDAndBRCs`, `getSHDAndBRCs`, `isRTL`, `setMemEnd`, `getMemEnd`, `rectForPageCore`, `newInstance`, `newObjecctFactory`, `commitOp`, `toString`, `getLineChars`, `isCardBegin`, `setCardBegin`, `isCardEnd`, `setCardEnd`, `setLineBefore`, `getLineBefore`, `setLineAfter`, `getLineAfter`, `setLineBalloonPageHeight`, `getLineBalloonPageHeight` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
