# 01_function_page_hit — 页内选行

| 项 | 值 |
|---|---|
| feature | wr-17-hit-test |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `line_for_y` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
已知 page index，在行框半开区间 `[y,y+height)` 内命中；页内上方夹首行、下方/间隙夹末端方向最近行；无该页返回 None。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| Stage2Line | wr-14 | page/line/y/height | 是 | page global transform | 重排 |
| wr-16 | gate | rows 已对齐 | 是（比较器/corpus seam） | Android 新 golden | 改 Java cp |
| TypoTextLine | cache | line geometry | 是 | table/drawing | Java anchors |

## 2. 核心代码
过滤指定页，先查包含框，否则按页首/页末夹逼。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| line_for_y | inside/above/below/missing | Option line | contain/clamp/None |
### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| PH-IN | inside | 是 | PH1 |
| PH-UP | above | 是 | PH2 |
| PH-DOWN | below | 是 | PH3 |
| PH-NONE | no page | 是 | PH4 |
### 3.3 可测试性
只吃 wr-14 rows，不运行布局。
### 3.4 用例表
PH1 second line；PH2 first；PH3 last；PH4 None。
### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/cache/PageHit.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/cache/PageHit.rs`
### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_page_hit
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 01_function_page_hit --src src/cn/wps/moffice/writer/cache/PageHit.rs --report libs/word-core/layout/features/wr-17-hit-test/functions/01_function_page_hit/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_page_hit --json`)
date: 2026-08-25 01:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/cache/PageHit.rs` | 14/14 | 100.0% | 27/28 | 96.4% |
| **function total** | **14/14** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`TypoTextLine.java ↔ TypoTextLine.rs`：**PASS**。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs --report libs/word-core/layout/features/wr-17-hit-test/functions/01_function_page_hit/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 18:01

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoTextLine` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` | **PASS** | `setPara`, `getPara`, `setTextRectTop`, `getTextRectTop`, `setTextRectHeight`, `setAscDesPos`, `getTextRectHeight`, `getDescentPos`, `getAscentPos`, `setFlags`, `getFlags`, `getGraphIndexStart`, `setGraphIndexStart`, `getGraphIndexEnd`, `setGraphIndexEnd`, `setYPos`, `getYPos`, `isLastLineOfPara`, `setFirstLineOfPara`, `setLastLineOfPara`, `setLastLineOfPage`, `isFirstLineOfPara`, `isLastLineOfPage`, `getStartCP`, `getEndCP`, `contains` | `checkClassType`, `getType`, `alloc`, `isPointer`, `getRelTextRectTop`, `getTextRectBottom`, `offsetTextRectTop`, `getGraphLength`, `getEndChar`, `hasSHDs`, `getSHDs`, `setSHDs`, `getBorders`, `setBorders`, `getHighlights`, `setHighlights`, `getDocFiedls`, `getDocFields`, `setDocFields`, `getLines`, `setLines`, `isFirstLineOfMathPara`, `setIsInMathPara`, `setIsFirstLineOfMathPara`, `setIsLastLineOfMathPara`, `setFixedHeight`, `isFixedHeight`, `setExtendLastSHDAndBrc`, `isExtendLastSHDAndBrc`, `isLastLineOfSection`, `setLastLineOfSection`, `setLastLineOfColumn`, `setRTL`, `hasComment`, `setHasComment`, `hasEmbedsDrawing`, `setHasEmbedsDrawing`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `isTemplateContent`, `setIsTemplateContent`, `hasAutoNumber`, `setHasAutoNumber`, `setHasSubLine`, `hasSubLine`, `setHasBidi`, `hasBidi`, `isInMathPara`, `isLastLineOfMathPara`, `isLastLineOfColumn`, `getFirstValidCp`, `getCPRef`, `setSHDAndBRCs`, `getSHDAndBRCs`, `isRTL`, `setMemEnd`, `getMemEnd`, `rectForPageCore`, `newInstance`, `newObjecctFactory`, `newObject`, `getTClass`, `commitOp`, `toString`, `getLineChars`, `isCardBegin`, `setCardBegin`, `isCardEnd`, `setCardEnd`, `setLineBefore`, `getLineBefore`, `setLineAfter`, `getLineAfter`, `setLineBalloonPageHeight`, `getLineBalloonPageHeight` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
