# 02_function_style_commands — 行 DrawText

| 项 | 值 |
|---|---|
| feature | wr-15-style-commands |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `emit_styled_line` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
先调用 01 合并，再按 measured advance 累加 run_x；写 page、x、baseline y、font_id、size、bold、color。y 是 baseline，版本语义不降级。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| 01 StyleRuns | 本 feature | coalesce/advance | 是 | — | — |
| TextCommand | wr-07 | emit_text | 是 | xpos optional | baseline contract |
| TypoRun/TypoTextLine | cache | run/line product | 是 | arena | Java anchors |

## 2. 核心代码
每个合并后的 run 生成一个 `RenderCommand::Text`，下一个 x 加上前一 run 的 measured advance。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| emit_styled_line | heading/bold/two fonts/same style | commands | style/baseline/x/merge |
### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| ST-STYLE | heading+bold+font | 是 | ST1 |
| ST-X | advance accumulation | 是 | ST2 |
| ST-MERGE | same style | 是 | ST3 |
### 3.3 可测试性
内存 styled line，不跑 painter。
### 3.4 用例表
| test | 期望 |
|---|---|
| ST1 | page/x/y/size/font/bold 精确 |
| ST2 | second x = line x + first advance |
| ST3 | one command |
### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/rendercommand/StyleCommands.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/rendercommand/StyleCommands.rs`
### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_style_commands
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 02_function_style_commands --src src/cn/wps/moffice/writer/rendercommand/StyleCommands.rs --report libs/word-core/layout/features/wr-15-style-commands/functions/02_function_style_commands/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_style_commands --json`)
date: 2026-08-25 01:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/rendercommand/StyleCommands.rs` | 23/23 | 100.0% | 25/25 | 100.0% |
| **function total** | **23/23** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`TypoRun.java ↔ TypoRun.rs`、`TypoTextLine.java ↔ TypoTextLine.rs`：**PASS**。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoRun.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoRun.rs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs --report libs/word-core/layout/features/wr-15-style-commands/functions/02_function_style_commands/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 18:01

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoRun` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoRun.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoRun.rs` | **PASS** | `needTransToSpace`, `isSpace`, `isTab`, `isWeakHeightGraphs`, `isWeakBdrForHeight`, `isComplexLangType`, `isFarEastLangType`, `isWestLangType`, `isTypoPageBreak`, `isLineBreak`, `isPunct`, `isNormal`, `isNONBMP`, `getTypeStr` | — | — | **PASS** |
| `TypoTextLine` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` | **PASS** | `setPara`, `getPara`, `setTextRectTop`, `getTextRectTop`, `setTextRectHeight`, `setAscDesPos`, `getTextRectHeight`, `getDescentPos`, `getAscentPos`, `setFlags`, `getFlags`, `getGraphIndexStart`, `setGraphIndexStart`, `getGraphIndexEnd`, `setGraphIndexEnd`, `setYPos`, `getYPos`, `isLastLineOfPara`, `setFirstLineOfPara`, `setLastLineOfPara`, `setLastLineOfPage`, `isFirstLineOfPara`, `isLastLineOfPage`, `getStartCP`, `getEndCP`, `contains` | `checkClassType`, `getType`, `alloc`, `isPointer`, `getRelTextRectTop`, `getTextRectBottom`, `offsetTextRectTop`, `getGraphLength`, `getEndChar`, `hasSHDs`, `getSHDs`, `setSHDs`, `getBorders`, `setBorders`, `getHighlights`, `setHighlights`, `getDocFiedls`, `getDocFields`, `setDocFields`, `getLines`, `setLines`, `isFirstLineOfMathPara`, `setIsInMathPara`, `setIsFirstLineOfMathPara`, `setIsLastLineOfMathPara`, `setFixedHeight`, `isFixedHeight`, `setExtendLastSHDAndBrc`, `isExtendLastSHDAndBrc`, `isLastLineOfSection`, `setLastLineOfSection`, `setLastLineOfColumn`, `setRTL`, `hasComment`, `setHasComment`, `hasEmbedsDrawing`, `setHasEmbedsDrawing`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `isTemplateContent`, `setIsTemplateContent`, `hasAutoNumber`, `setHasAutoNumber`, `setHasSubLine`, `hasSubLine`, `setHasBidi`, `hasBidi`, `isInMathPara`, `isLastLineOfMathPara`, `isLastLineOfColumn`, `getFirstValidCp`, `getCPRef`, `setSHDAndBRCs`, `getSHDAndBRCs`, `isRTL`, `setMemEnd`, `getMemEnd`, `rectForPageCore`, `newInstance`, `newObjecctFactory`, `newObject`, `getTClass`, `commitOp`, `toString`, `getLineChars`, `isCardBegin`, `setCardBegin`, `isCardEnd`, `setCardEnd`, `setLineBefore`, `getLineBefore`, `setLineAfter`, `getLineAfter`, `setLineBalloonPageHeight`, `getLineBalloonPageHeight` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
