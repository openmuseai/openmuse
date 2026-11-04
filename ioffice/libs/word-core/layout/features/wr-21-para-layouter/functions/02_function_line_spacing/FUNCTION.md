# 02_function_line_spacing — 行距 220

| 项 | 值 |
|----|----|
| feature | wr-21-para-layouter |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `ParaLayouter::calcLineSpace` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
getObject(220, DefaultValue.paragraph_lineSpacing)。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | ParaLayouter.java:415 |

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
| ParaLayouter::calcLineSpace | ParaLayouter.java:415 | 见矩阵 | 见矩阵 | 见覆盖矩阵 |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|---|---|---|---|
| LS1 | 缺 220 | 是 | LS1 |
| LS2 | auto | 是 | LS2 |
| LS3 | exact | 是 | LS3 |

### 3.3 可测试性
fixture / 假 PropertySet / 假 LayoutState；不跑无关管线。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| LS1 | LS1 | 缺 220 | Java default | 1.0 发明 |
| LS2 | LS2 | auto | 同行高因子 | 当 exact |
| LS3 | LS3 | exact | 行格 twip | 当倍数 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/ParaLayouter.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_line_spacing

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_line_spacing \
  --src src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --report libs/word-core/layout/features/wr-21-para-layouter/functions/02_function_line_spacing/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_line_spacing --json`)
date: 2026-09-03 19:22

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs` | 25/249 | 10.0% | 36/532 | 6.8% |
| **function total** | **25/249** | **10.0%** |  |  |
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
  --report libs/word-core/layout/features/wr-21-para-layouter/functions/02_function_line_spacing/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-03 11:22

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ParaLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs` | **PASS** | `ParaLayouter`, `isAutoAfterSpacing`, `calcParaBefore`, `hasSameAutoNum`, `calcParaAfter`, `calcPrevParaAfter`, `calcLineSpace` | `dispose`, `onStart`, `calcFinalPropertySet`, `getFinalParaProp`, `calclastBrcBetween`, `calcParaProp`, `calcBRC`, `calcSHD`, `getVaildShd`, `calcAutoNum`, `onEnd`, `isLineWithParaBreakAndSectionBreak`, `shouldCalcLastParaAfter`, `shouldAdjustRightMarginInCharGridModel`, `calcIndents`, `calFirstLineIndent`, `getNormalParaFinalProperty`, `getCharGridExtraFirstLineIndent`, `calLeftAndRightIndent`, `getCharGridExtraRightIndent`, `getFirstCharSize`, `calcShouldCompressLine`, `calAlign`, `isFirstLineOfPara` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
