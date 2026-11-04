# 01_function_stage2_rows — Snapshot 行表投影

| 项 | 值 |
|---|---|
| feature | wr-14-typo-stage2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `rows_from_pages` / `rows_from_snapshot` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
只投影 page,line,start_cp,end_cp,x,y,width,height；不重新断行，不输出 xpos；每页 line 从 0 开始。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| PaginatedPage | wr-13 | page/lines | 是 | — | 分页 |
| TypoSnapshot/Page/TextLine | cache | owned pages/lines/geometry | 是 | pointer arena | Java anchors |

## 2. 核心代码
两种输入后端共享同一 `Stage2Line` 契约，按页序/行序线性投影。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| rows_from_pages | pages | rows | all fields/empty/index reset |
| rows_from_snapshot | snapshot | rows | 由 02 集成覆盖 |
### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| R-FIELDS | 全字段 | 是 | S2R1 |
| R-EMPTY | 空 | 是 | S2R2 |
| R-RESET | 页间 line reset | 是 | S2R3 |
| R-SNAPSHOT | Snapshot 后端 | 是 | S2R4;S2R5 |
### 3.3 可测试性
内存产品页，不跑完整排版。
### 3.4 用例表
| test | 期望 |
|---|---|
| S2R1 | 八字段逐一相等 |
| S2R2 | 空向量 |
| S2R3 | 第二页首行 index=0 |
### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/cache/Stage2Rows.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/cache/Stage2Rows.rs`
### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_stage2_rows
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 01_function_stage2_rows --src src/cn/wps/moffice/writer/cache/Stage2Rows.rs --report libs/word-core/layout/features/wr-14-typo-stage2/functions/01_function_stage2_rows/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_stage2_rows --json`)
date: 2026-08-25 02:04

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/cache/Stage2Rows.rs` | 37/37 | 100.0% | 54/54 | 100.0% |
| **function total** | **37/37** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`TypoTextLine.java ↔ TypoTextLine.rs`、`TypoPage.java ↔ TypoPage.rs`：**PASS**。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoPage.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoPage.rs --report libs/word-core/layout/features/wr-14-typo-stage2/functions/01_function_stage2_rows/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 18:01

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoTextLine` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` | **PASS** | `setPara`, `getPara`, `setTextRectTop`, `getTextRectTop`, `setTextRectHeight`, `setAscDesPos`, `getTextRectHeight`, `getDescentPos`, `getAscentPos`, `setFlags`, `getFlags`, `getGraphIndexStart`, `setGraphIndexStart`, `getGraphIndexEnd`, `setGraphIndexEnd`, `setYPos`, `getYPos`, `isLastLineOfPara`, `setFirstLineOfPara`, `setLastLineOfPara`, `setLastLineOfPage`, `isFirstLineOfPara`, `isLastLineOfPage`, `getStartCP`, `getEndCP`, `contains` | `checkClassType`, `getType`, `alloc`, `isPointer`, `getRelTextRectTop`, `getTextRectBottom`, `offsetTextRectTop`, `getGraphLength`, `getEndChar`, `hasSHDs`, `getSHDs`, `setSHDs`, `getBorders`, `setBorders`, `getHighlights`, `setHighlights`, `getDocFiedls`, `getDocFields`, `setDocFields`, `getLines`, `setLines`, `isFirstLineOfMathPara`, `setIsInMathPara`, `setIsFirstLineOfMathPara`, `setIsLastLineOfMathPara`, `setFixedHeight`, `isFixedHeight`, `setExtendLastSHDAndBrc`, `isExtendLastSHDAndBrc`, `isLastLineOfSection`, `setLastLineOfSection`, `setLastLineOfColumn`, `setRTL`, `hasComment`, `setHasComment`, `hasEmbedsDrawing`, `setHasEmbedsDrawing`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `isTemplateContent`, `setIsTemplateContent`, `hasAutoNumber`, `setHasAutoNumber`, `setHasSubLine`, `hasSubLine`, `setHasBidi`, `hasBidi`, `isInMathPara`, `isLastLineOfMathPara`, `isLastLineOfColumn`, `getFirstValidCp`, `getCPRef`, `setSHDAndBRCs`, `getSHDAndBRCs`, `isRTL`, `setMemEnd`, `getMemEnd`, `rectForPageCore`, `newInstance`, `newObjecctFactory`, `newObject`, `getTClass`, `commitOp`, `toString`, `getLineChars`, `isCardBegin`, `setCardBegin`, `isCardEnd`, `setCardEnd`, `setLineBefore`, `getLineBefore`, `setLineAfter`, `getLineAfter`, `setLineBalloonPageHeight`, `getLineBalloonPageHeight` | — | **PASS** |
| `TypoPage` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoPage.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoPage.rs` | **PASS** | `getDocumentType`, `setDocumentType`, `setMargin`, `getMarginLeft`, `setMarginLeft`, `getMarginTop`, `setMarginTop`, `getMarginRight`, `setMarginRight`, `getMarginBottom`, `setMarginBottom`, `getLineCount`, `hasLines`, `insertLine`, `contains` | `isPointer`, `getFlagString`, `checkClassType`, `getDocument`, `getLines`, `getPageBRCs`, `setPageBRCs`, `isBodyPage`, `getMinLineSize`, `removeLine`, `getFlag`, `setBoolBitSet`, `setIntBitSet`, `hasModify`, `setModify`, `getTextFlow`, `setTextFlow`, `getRotationAsia`, `setRotationAsia`, `getExtendRanges`, `addExtendRange`, `removeExtendRanges`, `copyRange`, `copyExtendRanges`, `getMinStartCP`, `getMaxEndCP`, `updatePageRanges`, `getCoreWidth`, `getCoreHeight`, `getParentPageVisualTextflow`, `isRotatedWithParentPage`, `hasParaShdAndBrcs`, `setHasParaShdAndBrcs`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `getVisualTextflow`, `rectForPageCoreToRectForPage`, `rectForPageCoreToRectForLayoutPage`, `rectForPageCoreToRectForGlobal`, `rectForPageToRectForPageCore`, `pointForPageToPointForPageCore`, `getRenderWidth`, `getRenderHeight`, `hasIntersect`, `getLineIndexByY`, `toString`, `dumpLines` | — | **PASS** |
| `PageRangeIterator` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoPage.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoPage.rs` | **PASS** | — | `init`, `reset`, `isEnd`, `next`, `getStartCP`, `getEndCP`, `setStartCP`, `setEndCP`, `assertInitialised`, `assertIterated` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
