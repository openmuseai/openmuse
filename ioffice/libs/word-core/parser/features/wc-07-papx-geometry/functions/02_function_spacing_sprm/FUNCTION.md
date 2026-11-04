# 02_function_spacing_sprm — spacing → 214/217/220

| 项 | 值 |
|----|----|
| feature | wc-07-papx-geometry |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `convertLineSpacing / before / after` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
w:spacing before/after/line 写入 217/214/220。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | ParaPropsConverterSimple.convertLineSpacing |

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
| convertLineSpacing / before / after | ParaPropsConverterSimple.convertLineSpacing | 见矩阵 | 见矩阵 | 见覆盖矩阵 |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|---|---|---|---|
| SP1 | before | 是 | SP1 |
| SP2 | after | 是 | SP2 |
| SP3 | line auto | 是 | SP3 |
| SP4 | 无 spacing | 是 | SP4 |

### 3.3 可测试性
fixture / 假 PropertySet / 假 LayoutState；不跑无关管线。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| SP1 | SP1 | before | 217 | 未写 |
| SP2 | SP2 | after | 214 | 对调 |
| SP3 | SP3 | line auto | 220 对象 | 当 float |
| SP4 | SP4 | 无 spacing | !contains 217 | 写 0 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_spacing_sprm

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_spacing_sprm \
  --src src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs \
  --src src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverter.rs \
  --src src/cn/wps/moffice/writer/data/value/LineSpacing.rs \
  --report libs/word-core/parser/features/wc-07-papx-geometry/functions/02_function_spacing_sprm/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_spacing_sprm --json`)
date: 2026-09-03 17:31

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/data/value/LineSpacing.rs` | 11/53 | 20.8% | 13/68 | 19.1% |
| `src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverter.rs` | 46/68 | 67.6% | 76/112 | 67.9% |
| `src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs` | 277/634 | 43.7% | 454/822 | 55.2% |
| **function total** | **334/755** | **44.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverter.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/data/value/LineSpacing.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/data/value/LineSpacing.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.java --rs libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverter.java --rs libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/data/value/LineSpacing.java --rs libs/word-core/src/cn/wps/moffice/writer/data/value/LineSpacing.rs \
  --report libs/word-core/parser/features/wc-07-papx-geometry/functions/02_function_spacing_sprm/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-03 09:31

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ParaPropsConverterSimple` | `koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.java` | `libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs` | **PASS** | `ParaPropsConverterSimple`, `convertProp`, `setAlignment`, `setIlvl`, `setStyle`, `setNumId`, `convertPPrChange`, `correctProp`, `correctPPrBase`, `correntStyle`, `getDifferenceParaProp`, `setTxTabStop`, `correctTabs`, `getTbd`, `convertTabFontBase`, `convertTabFont03`, `convertTabFont`, `convertTBDJC`, `convertFrameProperties`, `setWrap`, `setTextDirection`, `toTextFlow`, `setOutlineLvl`, `setTextboxTightWrap`, `setTextAlignment`, `convertParaIndentation`, `setRightIndent`, `setHangingIndent`, `setLeftIndent`, `setFirstLineIndent`, `convertLineSpacing03`, `convertLineSpacing`, `convertLineSpacingBase`, `convertLineSpacingAndRule`, `isConvertExact2AtLeast`, `isConvertExactRuleAndValue`, `isConvertExactValue`, `convertlineSpacingRule`, `convertOMathParaPrJc` | — | — | **PASS** |
| `ParaPropsConverter` | `koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverter.java` | `libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverter.rs` | **PASS** | `convertSpaceAfterAuto`, `convertSpaceAfterLines`, `convertSpaceAfter`, `convertSpaceBeforeAuto`, `convertSpaceBeforeLines`, `convertSpaceBefore` | `getDefaultMergeStyleProp`, `getDefaultMergeNormalProp`, `getNormalStyleProp` | — | **PASS** |
| `LineSpacing` | `koffice/writer/core/cn/wps/moffice/writer/data/value/LineSpacing.java` | `libs/word-core/src/cn/wps/moffice/writer/data/value/LineSpacing.rs` | **PASS** | `LineSpacing`, `setRule`, `setSpacing`, `getRule`, `getSpacing`, `hashCode`, `equals`, `toString` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
