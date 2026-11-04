# 02_function_l2_compare — L2 比较

| 项 | 值 |
|----|----|
| feature | wr-24-live-stage2 |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `LineCompare（复用 wr-16）` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
cp/页精确；行框 ±0.01px；xpos 不同仍 Ok。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | wr-16 LineCompare |

### 1.3 输出
见 §1.1

### 1.4 上游
上游见 FUNCTIONS.md

### 1.5 下游
下游见 FUNCTIONS.md

### 1.6 不变量与函数改写列表
- 切在 I/O\n- 改写列表：本入口；其余 DEFERRED_NON_FUNCTION

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| 上游 feature | 见计划 | 声明的 O | 见 overview | 本 function 不改 | — |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出 | 关键分支 |
|---|---|---|---|---|
| LineCompare（复用 wr-16） | wr-16 LineCompare | 见矩阵 | 见矩阵 | 见覆盖矩阵 |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|---|---|---|---|
| LC1 | cp 不等 | 是 | LC1 |
| LC2 | x 超容差 | 是 | LC2 |
| LC3 | 几何全等 | 是 | LC3 |
| LC4 | 仅 xpos 不同 | 是 | LC4 |

### 3.3 可测试性
fixture / 假 PropertySet / 假 LayoutState；不跑无关管线。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| LC1 | LC1 | cp 不等 | fail | 放过 |
| LC2 | LC2 | x 超容差 | fail | 放过 |
| LC3 | LC3 | 几何全等 | Ok | 误报 |
| LC4 | LC4 | 仅 xpos 不同 | Ok | 当 fail |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/linegolden/LineCompare.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/linegolden/LineCompare.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_l2_compare

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_l2_compare \
  --src src/cn/wps/moffice/writer/linegolden/LineCompare.rs \
  --report libs/word-core/layout/features/wr-24-live-stage2/functions/02_function_l2_compare/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_l2_compare --json`)
date: 2026-09-05 03:51

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/linegolden/LineCompare.rs` | 32/44 | 72.7% | 56/73 | 76.7% |
| **function total** | **32/44** | **72.7%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs \
  --report libs/word-core/layout/features/wr-24-live-stage2/functions/02_function_l2_compare/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-04 19:51

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoTextLine` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` | **PASS** | `setPara`, `getPara`, `setTextRectTop`, `getTextRectTop`, `setTextRectHeight`, `setAscDesPos`, `getTextRectHeight`, `getDescentPos`, `getAscentPos`, `setFlags`, `getFlags`, `getGraphIndexStart`, `setGraphIndexStart`, `getGraphIndexEnd`, `setGraphIndexEnd`, `setYPos`, `getYPos`, `isLastLineOfPara`, `setFirstLineOfPara`, `setLastLineOfPara`, `setLastLineOfPage`, `isFirstLineOfPara`, `isLastLineOfPage`, `getStartCP`, `getEndCP`, `contains` | `checkClassType`, `getType`, `alloc`, `isPointer`, `getRelTextRectTop`, `getTextRectBottom`, `offsetTextRectTop`, `getGraphLength`, `getEndChar`, `hasSHDs`, `getSHDs`, `setSHDs`, `getBorders`, `setBorders`, `getHighlights`, `setHighlights`, `getDocFiedls`, `getDocFields`, `setDocFields`, `getLines`, `setLines`, `isFirstLineOfMathPara`, `setIsInMathPara`, `setIsFirstLineOfMathPara`, `setIsLastLineOfMathPara`, `setFixedHeight`, `isFixedHeight`, `setExtendLastSHDAndBrc`, `isExtendLastSHDAndBrc`, `isLastLineOfSection`, `setLastLineOfSection`, `setLastLineOfColumn`, `setRTL`, `hasComment`, `setHasComment`, `hasEmbedsDrawing`, `setHasEmbedsDrawing`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `isTemplateContent`, `setIsTemplateContent`, `hasAutoNumber`, `setHasAutoNumber`, `setHasSubLine`, `hasSubLine`, `setHasBidi`, `hasBidi`, `isInMathPara`, `isLastLineOfMathPara`, `isLastLineOfColumn`, `getFirstValidCp`, `getCPRef`, `setSHDAndBRCs`, `getSHDAndBRCs`, `isRTL`, `setMemEnd`, `getMemEnd`, `rectForPageCore`, `newInstance`, `newObjecctFactory`, `commitOp`, `toString`, `getLineChars`, `isCardBegin`, `setCardBegin`, `isCardEnd`, `setCardEnd`, `setLineBefore`, `getLineBefore`, `setLineAfter`, `getLineAfter`, `setLineBalloonPageHeight`, `getLineBalloonPageHeight` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
