# 03_function_hit_test — 点命中编排

| 项 | 值 |
|---|---|
| feature | wr-17-hit-test |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `hit_test` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
01 选页内行，02 算 cp，返回 `(page_index,line_index,cp)`；无页 None；缺 advance 安全夹到 start cp。输入已是 twip，zoom 反变换在调用方。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| 01 PageHit | 本 feature | line | 是 | — | — |
| 02 CpHit | 本 feature | cp | 是 | bidi | — |
| wr-14/16 | upstream | aligned rows | 是（seam） | device golden refresh | reflow |

## 2. 核心代码
以 rows 全局索引选择对应 advances，再组合结果；不复制选行/选 cp 逻辑。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| hit_test | valid/missing page/missing widths | Option hit | compose/None/fallback |
### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| HT-OK | second line+midpoint | 是 | HT1 |
| HT-NONE | missing page | 是 | HT2 |
| HT-NOWIDTH | no advances | 是 | HT3 |
### 3.3 可测试性
用上游 rows + advances fixture，不启动 parse/layout。
### 3.4 用例表
HT1=(0,1,3)；HT2=None；HT3=start cp。
### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/cache/HitTest.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/cache/HitTest.rs`
### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_hit_test
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 03_function_hit_test --src src/cn/wps/moffice/writer/cache/HitTest.rs --report libs/word-core/layout/features/wr-17-hit-test/functions/03_function_hit_test/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_hit_test --json`)
date: 2026-08-25 01:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/cache/HitTest.rs` | 17/17 | 100.0% | 26/27 | 96.3% |
| **function total** | **17/17** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`TypoTextLine.java ↔ TypoTextLine.rs`：**PASS**。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs --report libs/word-core/layout/features/wr-17-hit-test/functions/03_function_hit_test/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 18:01

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoTextLine` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` | **PASS** | `setPara`, `getPara`, `setTextRectTop`, `getTextRectTop`, `setTextRectHeight`, `setAscDesPos`, `getTextRectHeight`, `getDescentPos`, `getAscentPos`, `setFlags`, `getFlags`, `getGraphIndexStart`, `setGraphIndexStart`, `getGraphIndexEnd`, `setGraphIndexEnd`, `setYPos`, `getYPos`, `isLastLineOfPara`, `setFirstLineOfPara`, `setLastLineOfPara`, `setLastLineOfPage`, `isFirstLineOfPara`, `isLastLineOfPage`, `getStartCP`, `getEndCP`, `contains` | `checkClassType`, `getType`, `alloc`, `isPointer`, `getRelTextRectTop`, `getTextRectBottom`, `offsetTextRectTop`, `getGraphLength`, `getEndChar`, `hasSHDs`, `getSHDs`, `setSHDs`, `getBorders`, `setBorders`, `getHighlights`, `setHighlights`, `getDocFiedls`, `getDocFields`, `setDocFields`, `getLines`, `setLines`, `isFirstLineOfMathPara`, `setIsInMathPara`, `setIsFirstLineOfMathPara`, `setIsLastLineOfMathPara`, `setFixedHeight`, `isFixedHeight`, `setExtendLastSHDAndBrc`, `isExtendLastSHDAndBrc`, `isLastLineOfSection`, `setLastLineOfSection`, `setLastLineOfColumn`, `setRTL`, `hasComment`, `setHasComment`, `hasEmbedsDrawing`, `setHasEmbedsDrawing`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `isTemplateContent`, `setIsTemplateContent`, `hasAutoNumber`, `setHasAutoNumber`, `setHasSubLine`, `hasSubLine`, `setHasBidi`, `hasBidi`, `isInMathPara`, `isLastLineOfMathPara`, `isLastLineOfColumn`, `getFirstValidCp`, `getCPRef`, `setSHDAndBRCs`, `getSHDAndBRCs`, `isRTL`, `setMemEnd`, `getMemEnd`, `rectForPageCore`, `newInstance`, `newObjecctFactory`, `newObject`, `getTClass`, `commitOp`, `toString`, `getLineChars`, `isCardBegin`, `setCardBegin`, `isCardEnd`, `setCardEnd`, `setLineBefore`, `getLineBefore`, `setLineAfter`, `getLineAfter`, `setLineBalloonPageHeight`, `getLineBalloonPageHeight` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
