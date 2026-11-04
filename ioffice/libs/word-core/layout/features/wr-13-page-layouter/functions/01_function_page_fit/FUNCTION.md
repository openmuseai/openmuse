# 01_function_page_fit — 装页溢出判定

| 项 | 值 |
|---|---|
| feature | wr-13-page-layouter |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `PageFit::shouldBreakBefore` |
| status | **pass** |

## 1. 功能说明

### 1.6 不变量与函数改写列表
仅改写 `shouldBreakBefore`：已有行时 `used + line > content` 才换页；等高不换；首个超高行必须落页保证进度；负高度按 0。Java 权威为 `TextLayouter.flowLine/finishPage` 的页满分支，不移植表、drawing、脚注。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 需要接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| wr-12 line height | wr-12 | 非负 twip | 是 | — | 断行 |
| `TypoTextLine` | cache | height/页末 flag | 是 | pointer arena | Java 锚点 |
| `TextLayouter` | Java | overflow 规则 | 已审计 | 表/脚注/drawing | 整类移植 |

## 2. 核心代码

用 `i64` 相加避免 `i32` 溢出；只有非空页可 break-before。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `shouldBreakBefore` | `TextLayouter.java` / `PageFit.rs` | used,line,limit,has-lines | bool | exact/overflow/first/negative/MAX |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| PF-EXACT | sum == limit | 边界 | 是 | PF1 |
| PF-OVER | sum > limit | 分支 | 是 | PF2 |
| PF-FIRST | first oversized | 进度 | 是 | PF3 |
| PF-NEG | negative clamp | 非法/边界 | 是 | PF4 |
| PF-MAX | large integer | 溢出 | 是 | PF5 |

### 3.3 可测试性
纯整数 seam，不启动完整 layouter。

### 3.4 用例表
| id | 输入摘要 | 期望 |
|---|---|---|
| PF1 | 60+40/100 | false |
| PF2 | 61+40/100 | true |
| PF3 | empty + 120/100 | false |
| PF4 | negative values | clamped false |
| PF5 | MAX+1/MAX | true, no wrap |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/PageFit.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/PageFit.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_page_fit
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_page_fit \
  --src src/cn/wps/moffice/writer/layout/base/env/PageFit.rs \
  --report libs/word-core/layout/features/wr-13-page-layouter/functions/01_function_page_fit/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_page_fit --json`)
date: 2026-08-25 01:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/PageFit.rs` | 14/14 | 100.0% | 18/18 | 100.0% |
| **function total** | **14/14** | **100.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs
```
产品 Java 双子布局：**PASS**。

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs --report libs/word-core/layout/features/wr-13-page-layouter/functions/01_function_page_fit/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 18:00

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoTextLine` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` | **PASS** | `setPara`, `getPara`, `setTextRectTop`, `getTextRectTop`, `setTextRectHeight`, `setAscDesPos`, `getTextRectHeight`, `getDescentPos`, `getAscentPos`, `setFlags`, `getFlags`, `getGraphIndexStart`, `setGraphIndexStart`, `getGraphIndexEnd`, `setGraphIndexEnd`, `setYPos`, `getYPos`, `isLastLineOfPara`, `setFirstLineOfPara`, `setLastLineOfPara`, `setLastLineOfPage`, `isFirstLineOfPara`, `isLastLineOfPage`, `getStartCP`, `getEndCP`, `contains` | `checkClassType`, `getType`, `alloc`, `isPointer`, `getRelTextRectTop`, `getTextRectBottom`, `offsetTextRectTop`, `getGraphLength`, `getEndChar`, `hasSHDs`, `getSHDs`, `setSHDs`, `getBorders`, `setBorders`, `getHighlights`, `setHighlights`, `getDocFiedls`, `getDocFields`, `setDocFields`, `getLines`, `setLines`, `isFirstLineOfMathPara`, `setIsInMathPara`, `setIsFirstLineOfMathPara`, `setIsLastLineOfMathPara`, `setFixedHeight`, `isFixedHeight`, `setExtendLastSHDAndBrc`, `isExtendLastSHDAndBrc`, `isLastLineOfSection`, `setLastLineOfSection`, `setLastLineOfColumn`, `setRTL`, `hasComment`, `setHasComment`, `hasEmbedsDrawing`, `setHasEmbedsDrawing`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `isTemplateContent`, `setIsTemplateContent`, `hasAutoNumber`, `setHasAutoNumber`, `setHasSubLine`, `hasSubLine`, `setHasBidi`, `hasBidi`, `isInMathPara`, `isLastLineOfMathPara`, `isLastLineOfColumn`, `getFirstValidCp`, `getCPRef`, `setSHDAndBRCs`, `getSHDAndBRCs`, `isRTL`, `setMemEnd`, `getMemEnd`, `rectForPageCore`, `newInstance`, `newObjecctFactory`, `newObject`, `getTClass`, `commitOp`, `toString`, `getLineChars`, `isCardBegin`, `setCardBegin`, `isCardEnd`, `setCardEnd`, `setLineBefore`, `getLineBefore`, `setLineAfter`, `getLineAfter`, `setLineBalloonPageHeight`, `getLineBalloonPageHeight` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
