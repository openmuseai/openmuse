# 01_function_indent_sprm — ind → 208–213

| 项 | 值 |
|----|----|
| feature | wc-07-papx-geometry |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `ParaPropsConverterSimple::convertProp(ind)` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
w:ind 的 firstLine/left/right/hanging 写入 208–213，缺属性 !contains。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | ParaPropsConverterSimple ind 分支 |

### 1.3 输出
见 §1.1

### 1.4 上游
上游见 FUNCTIONS.md

### 1.5 下游
下游见 FUNCTIONS.md

### 1.6 不变量与函数改写列表
- 改写列表：`ParaPropsConverterSimple` 的 `ind` 分支（Java ~735–805）：`hanging`→208 负值；`firstLine`→208；`left`/`start`→210；`right`/`end`→212；`*Chars`→209/211/213。
- 其余 convert 方法 `todo!("DEFERRED_NON_FUNCTION")`。
- 缺属性必须 `!contains`，禁止写 0 冒充缺省。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `ConvertHelper.getLong` | wc-03 | attr 解析 | 是 | — | — |
| `PropertySet.setInt` | wave0 | 208–213 | 是 | — | — |
| SAX `w:ind` 送到 convert | wc-04/05 | 属性 map | 部分 | wc-06 金文件证明 | 禁止本 function 改 PHandler |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出 | 关键分支 |
|---|---|---|---|---|
| ParaPropsConverterSimple::convertProp | ParaPropsConverterSimple ind 分支 | 见矩阵 | 见矩阵 | 见覆盖矩阵 |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|-------------|----------|-------|---------|
| IN1 | firstLine | 是 | IN1 |
| IN2 | left | 是 | IN2 |
| IN3 | right | 是 | IN3 |
| IN4 | 无 ind | 是 | IN4 |
| IN5 | hanging | 是 | IN5 |
| IN6 | hangingChars | 是 | IN6 |

### 3.3 可测试性
fixture / 假 PropertySet / 假 LayoutState；不跑无关管线。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| IN1 | IN1 | firstLine=480 | 208=480, contains | 未写 |
| IN2 | IN2 | left=360 | 210=360 | 只写 208 |
| IN3 | IN3 | right=240 | 212=240 | 忽略 |
| IN4 | IN4 | 无 ind 元素 | !contains(208/210/212) | 写 0 |
| IN5 | IN5 | hanging=480 | 208=-480 | 正 firstLine |
| IN6 | IN6 | hangingChars=100 | 209=-100 | 未写 209 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_indent_sprm

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_indent_sprm \
  --src src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs \
  --report libs/word-core/parser/features/wc-07-papx-geometry/functions/01_function_indent_sprm/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_indent_sprm --json`)
date: 2026-09-03 17:19

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs` | 183/520 | 35.2% | 274/602 | 45.5% |
| **function total** | **183/520** | **35.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.java --rs libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs \
  --report libs/word-core/parser/features/wc-07-papx-geometry/functions/01_function_indent_sprm/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-03 09:19

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ParaPropsConverterSimple` | `koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.java` | `libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs` | **PASS** | `ParaPropsConverterSimple`, `convertProp`, `setAlignment`, `setIlvl`, `setStyle`, `setNumId`, `convertPPrChange`, `correctProp`, `correctPPrBase`, `correntStyle`, `getDifferenceParaProp`, `setTxTabStop`, `correctTabs`, `getTbd`, `convertTabFontBase`, `convertTabFont03`, `convertTabFont`, `convertTBDJC`, `convertFrameProperties`, `setWrap`, `setTextDirection`, `toTextFlow`, `setOutlineLvl`, `setTextboxTightWrap`, `setTextAlignment`, `convertParaIndentation`, `setRightIndent`, `setHangingIndent`, `setLeftIndent`, `setFirstLineIndent`, `convertLineSpacing03`, `convertLineSpacing`, `convertLineSpacingBase`, `convertLineSpacingAndRule`, `isConvertExact2AtLeast`, `isConvertExactRuleAndValue`, `isConvertExactValue`, `convertlineSpacingRule`, `convertOMathParaPrJc` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
