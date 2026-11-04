# 02_function_widow_control — 寡妇/孤行回滚

| 项 | 值 |
|---|---|
| feature | wr-13-page-layouter |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `WidowControl::rebalance` |
| status | **pass** |

## 1. 功能说明

### 1.6 不变量与函数改写列表
同段跨页且 prop 197 开启：下一页仅一行时从前页回滚一行；前页仅留段首一行时把它移到下一页；保持顺序并删除空页。列表外：多栏、表、脚注、drawing。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 需要接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `PageLine.para_id` | 03 输入 | 同段判定 | 是 | — | PAPX 解析 |
| prop 197 | wc-03 | `widow_control` bool | 是（输入） | — | importer |
| `TextLayouter.processPagination` | Java | probe/rollback 语义 | 已审计 | `PaginationTool` 表/keep | 整类 |

## 2. 核心代码
逐相邻页统计 left suffix / right prefix；只移动页边界上的连续同段行。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust | 输入 | 输出 | 分支 |
|---|---|---|---|---|
| `rebalance` | `processPagination` / `WidowControl.rs` | pages | pages | widow/orphan/disabled/different para |

### 3.2 覆盖矩阵
| coverage_id | 场景 | 必测? | test_id |
|---|---|---|---|
| WC-WIDOW | 3/1 → 2/2 | 是 | WC1 |
| WC-ORPHAN | 1/2 → 0/3 | 是 | WC2 |
| WC-OFF | prop off | 是 | WC3 |
| WC-PARA | different para | 是 | WC4 |

### 3.3 可测试性
用内存 `PageLine` fixture，不运行全分页。

### 3.4 用例表
| id | 输入 | 期望 |
|---|---|---|
| WC1 | 同段 3/1 | 2/2，顺序不变 |
| WC2 | 同段 1/2 | 单页 3 行 |
| WC3 | widow=false | 不移动 |
| WC4 | 两段 | 不移动 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/WidowControl.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/WidowControl.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_widow_control
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 02_function_widow_control --src src/cn/wps/moffice/writer/layout/base/env/WidowControl.rs --report libs/word-core/layout/features/wr-13-page-layouter/functions/02_function_widow_control/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_widow_control --json`)
date: 2026-08-25 02:04

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/WidowControl.rs` | 38/42 | 90.5% | 56/63 | 88.9% |
| **function total** | **38/42** | **90.5%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` → **PASS**。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs --report libs/word-core/layout/features/wr-13-page-layouter/functions/02_function_widow_control/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 18:01

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoTextLine` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` | **PASS** | `setPara`, `getPara`, `setTextRectTop`, `getTextRectTop`, `setTextRectHeight`, `setAscDesPos`, `getTextRectHeight`, `getDescentPos`, `getAscentPos`, `setFlags`, `getFlags`, `getGraphIndexStart`, `setGraphIndexStart`, `getGraphIndexEnd`, `setGraphIndexEnd`, `setYPos`, `getYPos`, `isLastLineOfPara`, `setFirstLineOfPara`, `setLastLineOfPara`, `setLastLineOfPage`, `isFirstLineOfPara`, `isLastLineOfPage`, `getStartCP`, `getEndCP`, `contains` | `checkClassType`, `getType`, `alloc`, `isPointer`, `getRelTextRectTop`, `getTextRectBottom`, `offsetTextRectTop`, `getGraphLength`, `getEndChar`, `hasSHDs`, `getSHDs`, `setSHDs`, `getBorders`, `setBorders`, `getHighlights`, `setHighlights`, `getDocFiedls`, `getDocFields`, `setDocFields`, `getLines`, `setLines`, `isFirstLineOfMathPara`, `setIsInMathPara`, `setIsFirstLineOfMathPara`, `setIsLastLineOfMathPara`, `setFixedHeight`, `isFixedHeight`, `setExtendLastSHDAndBrc`, `isExtendLastSHDAndBrc`, `isLastLineOfSection`, `setLastLineOfSection`, `setLastLineOfColumn`, `setRTL`, `hasComment`, `setHasComment`, `hasEmbedsDrawing`, `setHasEmbedsDrawing`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `isTemplateContent`, `setIsTemplateContent`, `hasAutoNumber`, `setHasAutoNumber`, `setHasSubLine`, `hasSubLine`, `setHasBidi`, `hasBidi`, `isInMathPara`, `isLastLineOfMathPara`, `isLastLineOfColumn`, `getFirstValidCp`, `getCPRef`, `setSHDAndBRCs`, `getSHDAndBRCs`, `isRTL`, `setMemEnd`, `getMemEnd`, `rectForPageCore`, `newInstance`, `newObjecctFactory`, `newObject`, `getTClass`, `commitOp`, `toString`, `getLineChars`, `isCardBegin`, `setCardBegin`, `isCardEnd`, `setCardEnd`, `setLineBefore`, `getLineBefore`, `setLineAfter`, `getLineAfter`, `setLineBalloonPageHeight`, `getLineBalloonPageHeight` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
