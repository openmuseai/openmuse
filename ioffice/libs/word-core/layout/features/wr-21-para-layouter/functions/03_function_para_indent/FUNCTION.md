# 03_function_para_indent — 208–213 缩进

| 项 | 值 |
|----|----|
| feature | wr-21-para-layouter |
| order | 3 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `ParaLayouter 缩进块 FIRST_INDENT_CHARS_RATIO=100` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
`calcIndents` / `calFirstLineIndent` / `calLeftAndRightIndent`：209 chars 优先于 208 twip；210/211 左、212/213 右；chars 用 `gridWidth` EMU→twip × chars/100。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | ParaLayouter.java:419–506 |

### 1.3 输出
见 §1.1

### 1.4 上游
上游见 FUNCTIONS.md

### 1.5 下游
下游见 FUNCTIONS.md

### 1.6 不变量与函数改写列表
- 改写列表：`calcIndents`、`calFirstLineIndent`、`calLeftAndRightIndent`。
- deferred：`getCharGridExtraFirstLineIndent` 网格 1/3、`shouldAdjustRightMarginInCharGridModel`（PageView 默认 false）。
- Java：`leftIndentChars==0 && firstLineIndentChars>=0` 才读 210 twip；否则用 211×grid。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| PAPX 208–213 | wc-07 | contains + getInt | 部分 | hanging 负 208 | 禁止在本 function convert OOXML |
| `PageEnv.gridWidth` | wr-02 | 无网格时 chars 路径 | 默认 0 | 网格排版 | 禁止改 ViewParams A4 |
| `para_papx_from` | LayoutDocument | 现只读 208 | 是（不足） | 210/212 | 本 function 替换该简化式 |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出 | 关键分支 |
|---|---|---|---|---|
| ParaLayouter 缩进块 FIRST_INDENT_CHARS_RATIO=100 | ParaLayouter.java:419–506 | 见矩阵 | 见矩阵 | 见覆盖矩阵 |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|-------------|----------|-------|---------|
| ID1 | 仅 208 twip | 是 | ID1 |
| ID2 | 209≠0 覆盖 208 | 是 | ID2 |
| ID3 | 210 left twip | 是 | ID3 |
| ID4 | 212 right twip | 是 | ID4 |
| ID5 | 211/213 chars | 是 | ID5 |
| ID6 | hanging 负 208 | 是 | ID6 |

### 3.3 可测试性
fixture / 假 PropertySet / 假 LayoutState；不跑无关管线。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| ID1 | ID1 | 仅 208=480 | firstLineIndent=480 | 当 0 |
| ID2 | ID2 | 209=100, 208=480 | first 按字号×chars/100 | 仍用 480 |
| ID3 | ID3 | 210=360, 211=0 | leftIndent=360 | 只写进 first |
| ID4 | ID4 | 212=240 | rightIndent=240 | 忽略 |
| ID5 | ID5 | 211≠0 | left 走 grid×chars | 当 210 |
| ID6 | ID6 | 208=-480 hanging | firstLineIndent 负 | 当正 firstLine |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/ParaLayouter.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_para_indent

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_para_indent \
  --src src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --report libs/word-core/layout/features/wr-21-para-layouter/functions/03_function_para_indent/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_para_indent --json`)
date: 2026-09-03 19:30

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/core/CharFontTool.rs` | 8/18 | 44.4% | 20/29 | 69.0% |
| `src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs` | 69/417 | 16.5% | 103/780 | 13.2% |
| **function total** | **77/435** | **17.7%** |  |  |
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
  --report libs/word-core/layout/features/wr-21-para-layouter/functions/03_function_para_indent/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-03 11:30

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ParaLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs` | **PASS** | `ParaLayouter`, `onStart`, `isAutoAfterSpacing`, `shouldAdjustRightMarginInCharGridModel`, `calcParaBefore`, `hasSameAutoNum`, `calcParaAfter`, `calcPrevParaAfter`, `calcLineSpace`, `calcIndents`, `calFirstLineIndent`, `getCharGridExtraFirstLineIndent`, `calLeftAndRightIndent`, `getFirstCharSize`, `calcShouldCompressLine`, `calAlign`, `isFirstLineOfPara` | `dispose`, `calcFinalPropertySet`, `getFinalParaProp`, `calclastBrcBetween`, `calcParaProp`, `calcBRC`, `calcSHD`, `getVaildShd`, `calcAutoNum`, `onEnd`, `isLineWithParaBreakAndSectionBreak`, `shouldCalcLastParaAfter`, `getNormalParaFinalProperty`, `getCharGridExtraRightIndent` | — | **PASS** |
| `CharFontTool` | `koffice/writer/core/cn/wps/moffice/writer/core/CharFontTool.java` | `libs/word-core/src/cn/wps/moffice/writer/core/CharFontTool.rs` | **PASS** | `getFirstCharSize` | `getFontSizePropId`, `getMultiPropertyId`, `getCharFontPropId`, `getMultiDefaultValue` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
