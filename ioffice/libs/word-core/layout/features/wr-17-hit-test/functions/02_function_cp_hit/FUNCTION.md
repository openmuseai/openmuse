# 02_function_cp_hit — 行内 x→cp

| 项 | 值 |
|---|---|
| feature | wr-17-hit-test |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `cp_for_x` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
从行 x 按 wr-11 per-cp advance 累加；字符中点前取当前 cp，中点及之后取下一插入 cp；左右夹 start/end；零宽字符仍推进索引。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| wr-11 advances | wr-11 | per-cp twip | 是 | bidi visual order | FontHost |
| Stage2Line | wr-14 | x/start/end | 是 | xpos | line split |
| LocateTool/TextLineHitServer | Java | char positions/boundary | 已审计 | table/drawing | 整类移植 |

## 2. 核心代码
最多消费 `end_cp-start_cp` 个 advance，结果始终在闭区间 `[start_cp,end_cp]`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| cp_for_x | left/mid-before/mid/after/zero | cp | clamp/threshold/progress |
### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| CH-LEFT | before | 是 | CH1 |
| CH-MID | at midpoint | 是 | CH2 |
| CH-FIRST | before midpoint | 是 | CH3 |
| CH-RIGHT | after | 是 | CH4 |
| CH-ZERO | zero widths | 是 | CH5 |
### 3.3 可测试性
整数 advances fixture，不跑 FontHost。
### 3.4 用例表
CH1 start；CH2 next；CH3 current；CH4 end；CH5 skips zero-width cps。
### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/cache/CpHit.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/cache/CpHit.rs`
### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_cp_hit
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 02_function_cp_hit --src src/cn/wps/moffice/writer/cache/CpHit.rs --report libs/word-core/layout/features/wr-17-hit-test/functions/02_function_cp_hit/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_cp_hit --json`)
date: 2026-08-25 01:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/cache/CpHit.rs` | 15/15 | 100.0% | 31/31 | 100.0% |
| **function total** | **15/15** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`TypoTextLine.java ↔ TypoTextLine.rs`：**PASS**。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs --report libs/word-core/layout/features/wr-17-hit-test/functions/02_function_cp_hit/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 18:01

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoTextLine` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` | **PASS** | `setPara`, `getPara`, `setTextRectTop`, `getTextRectTop`, `setTextRectHeight`, `setAscDesPos`, `getTextRectHeight`, `getDescentPos`, `getAscentPos`, `setFlags`, `getFlags`, `getGraphIndexStart`, `setGraphIndexStart`, `getGraphIndexEnd`, `setGraphIndexEnd`, `setYPos`, `getYPos`, `isLastLineOfPara`, `setFirstLineOfPara`, `setLastLineOfPara`, `setLastLineOfPage`, `isFirstLineOfPara`, `isLastLineOfPage`, `getStartCP`, `getEndCP`, `contains` | `checkClassType`, `getType`, `alloc`, `isPointer`, `getRelTextRectTop`, `getTextRectBottom`, `offsetTextRectTop`, `getGraphLength`, `getEndChar`, `hasSHDs`, `getSHDs`, `setSHDs`, `getBorders`, `setBorders`, `getHighlights`, `setHighlights`, `getDocFiedls`, `getDocFields`, `setDocFields`, `getLines`, `setLines`, `isFirstLineOfMathPara`, `setIsInMathPara`, `setIsFirstLineOfMathPara`, `setIsLastLineOfMathPara`, `setFixedHeight`, `isFixedHeight`, `setExtendLastSHDAndBrc`, `isExtendLastSHDAndBrc`, `isLastLineOfSection`, `setLastLineOfSection`, `setLastLineOfColumn`, `setRTL`, `hasComment`, `setHasComment`, `hasEmbedsDrawing`, `setHasEmbedsDrawing`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `isTemplateContent`, `setIsTemplateContent`, `hasAutoNumber`, `setHasAutoNumber`, `setHasSubLine`, `hasSubLine`, `setHasBidi`, `hasBidi`, `isInMathPara`, `isLastLineOfMathPara`, `isLastLineOfColumn`, `getFirstValidCp`, `getCPRef`, `setSHDAndBRCs`, `getSHDAndBRCs`, `isRTL`, `setMemEnd`, `getMemEnd`, `rectForPageCore`, `newInstance`, `newObjecctFactory`, `newObject`, `getTClass`, `commitOp`, `toString`, `getLineChars`, `isCardBegin`, `setCardBegin`, `isCardEnd`, `setCardEnd`, `setLineBefore`, `getLineBefore`, `setLineAfter`, `getLineAfter`, `setLineBalloonPageHeight`, `getLineBalloonPageHeight` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
