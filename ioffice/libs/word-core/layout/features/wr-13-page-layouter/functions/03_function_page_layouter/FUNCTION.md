# 03_function_page_layouter — 分页编排

| 项 | 值 |
|---|---|
| feature | wr-13-page-layouter |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `paginate(PageLine[], content_height)` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
按 01 装页，`\f` 所在行后强制结页，随后调用 02 回滚；重算每页 y、page index、`LASTLINEOFPAGE`。不改变 cp 切分。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| 01 PageFit | 本 feature | break-before | 是 | — | — |
| 02 WidowControl | 本 feature | rebalance | 是 | — | — |
| TypoTextLine | wr-12/cache | cp,height,y,flag | 是 | pointer arena | Java anchors |
| PageViewLayouter | Java | page lifecycle | 已审计 | 多节/栏/脚注 | 整类 |

## 2. 核心代码
单次线性装页 + 边界回滚 + 线性几何重算；首个超高行仍落页。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| paginate | empty/height/formfeed/flags/oversize | pages | 0/1/N、强制、状态 |
### 3.2 覆盖矩阵
| id | 场景 | 必测? | test |
|---|---|---|---|
| PL-EMPTY | 空 | 是 | PL1 |
| PL-HEIGHT | 高度换页+y | 是 | PL2 |
| PL-FF | `\f` | 是 | PL3 |
| PL-FLAG | page-end flag | 是 | PL4 |
| PL-OVER | 首行超高 | 是 | PL5 |
### 3.3 可测试性
输入为 wr-12 产品行 fixture，不启动 parse/FontHost。
### 3.4 用例表
| test | 输入 | 期望 |
|---|---|---|
| PL1 | [] | 0 page |
| PL2 | 60+60/100 | 2 page, y=0 |
| PL3 | first force-after | 2 page |
| PL4 | two lines | only last flagged |
| PL5 | 120/100 first | one nonempty page |
### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/PageLayout.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/PageLayout.rs`
### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_page_layouter
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 03_function_page_layouter --src src/cn/wps/moffice/writer/layout/base/env/PageLayout.rs --report libs/word-core/layout/features/wr-13-page-layouter/functions/03_function_page_layouter/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_page_layouter --json`)
date: 2026-08-25 01:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/PageLayout.rs` | 43/43 | 100.0% | 70/70 | 100.0% |
| **function total** | **43/43** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`TypoTextLine.java ↔ TypoTextLine.rs` 与 `TypoPage.java ↔ TypoPage.rs` 均 **PASS**。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoPage.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoPage.rs --report libs/word-core/layout/features/wr-13-page-layouter/functions/03_function_page_layouter/FUNCTION.md
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
