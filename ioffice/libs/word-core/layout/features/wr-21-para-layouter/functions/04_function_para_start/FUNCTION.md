# 04_function_para_start — onStart 编排

| 项 | 值 |
|----|----|
| feature | wr-21-para-layouter |
| order | 4 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `ParaLayouter::onStart` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
final props → firstLine → calAlign(193) → before/after/lineSpace。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | ParaLayouter.java:61–83 |

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
| ParaLayouter::onStart | ParaLayouter.java:61–83 | 见矩阵 | 见矩阵 | 见覆盖矩阵 |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|---|---|---|---|
| PS1 | 193=1 | 是 | PS1 |
| PS2 | 调用顺序 | 是 | PS2 |
| PS3 | 251 RTL | 是 | PS3 |
| PS4 | 无 AutoNum | 是 | PS4 |

### 3.3 可测试性
fixture / 假 PropertySet / 假 LayoutState；不跑无关管线。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| PS1 | PS1 | 193=1 | alignment=1 | 仍 0 |
| PS2 | PS2 | 调用顺序 | before 在 lineSpace 前 | 乱序 |
| PS3 | PS3 | 251 RTL | isRTL | 忽略 |
| PS4 | PS4 | 无 AutoNum | 不加编号串 | panic |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/ParaLayouter.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_para_start

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 04_function_para_start \
  --src src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --report libs/word-core/layout/features/wr-21-para-layouter/functions/04_function_para_start/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_para_start --json`)
date: 2026-09-03 19:30

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs` | 159/417 | 38.1% | 273/780 | 35.0% |
| **function total** | **159/417** | **38.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --report libs/word-core/layout/features/wr-21-para-layouter/functions/04_function_para_start/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-03 11:30

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ParaLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs` | **PASS** | `ParaLayouter`, `onStart`, `isAutoAfterSpacing`, `shouldAdjustRightMarginInCharGridModel`, `calcParaBefore`, `hasSameAutoNum`, `calcParaAfter`, `calcPrevParaAfter`, `calcLineSpace`, `calcIndents`, `calFirstLineIndent`, `getCharGridExtraFirstLineIndent`, `calLeftAndRightIndent`, `getFirstCharSize`, `calcShouldCompressLine`, `calAlign`, `isFirstLineOfPara` | `dispose`, `calcFinalPropertySet`, `getFinalParaProp`, `calclastBrcBetween`, `calcParaProp`, `calcBRC`, `calcSHD`, `getVaildShd`, `calcAutoNum`, `onEnd`, `isLineWithParaBreakAndSectionBreak`, `shouldCalcLastParaAfter`, `getNormalParaFinalProperty`, `getCharGridExtraRightIndent` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
