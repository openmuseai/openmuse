# 03_function_face_select — ascii vs eastAsia 选面

| 项 | 值 |
|----|----|
| feature | wr-11-run-metrics-parity |
| order | 3 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `FaceSelect::selectFace(style, ch) -> &str` |
| 独立性 | 不读 TTF、不度量。 |

## 1. 功能说明

### 1.1 一句话
按字符脚本从 TypoFontStyle 选出 ascii 或 eastAsia 族名（混合 run 的 wr-11 产品切面）。

### 1.2 输入
见入口。

### 1.3 输出
见入口返回值。缺 cmap **失败**，不得用字号冒充宽度。

### 1.4 上游
注入 TTF（TestFont.ttf）或纯数值；不跑 wr-12。

### 1.5 下游
05 `RunMetrics` / wr-12 断行只许用这里的宽度。

### 1.6 不变量
1. Han/Kana/Hangul/CJK 标点/全角 → eastAsia（非空）。
2. 其它 → ascii（非空）。
3. 空槽回落 fontName，再 ascii，再 eastAsia。
4. 空格/数字走 ascii。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| TypoFontStyle | wr-10 | ascii/eastAsia/fontName | 已有 | typeface 句柄 | 禁止改 wr-10 字段集 |
| FontStyleFactory.createTypeface | Java | 整 run langType | 不移植 | 按 run 而非按 cp | 禁止本波改 Factory |

### 1.8 函数改写列表
- `FaceSelect::selectFace`
- `is_east_asia`
- TypefaceTool：**不在列表**（选面不是该类方法）

## 2. 核心代码

Unicode 区段判断 + 空名回落。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| A/空格/数字 | | ascii 名 | 拉丁 |
| 中/あ/한/。/ア | | 宋体 | CJK |
| eastAsia 空 | 中 | fontName | 回落 |
| A中B | | 交替 | 混合 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| FS-LATIN | selectFace | A | 是 | FS1 |
| FS-HAN | | 中 | 是 | FS2 |
| FS-HIRA | | あ | 是 | FS3 |
| FS-HANG | | 한 | 是 | FS4 |
| FS-SP | | 空格 | 是 | FS5 |
| FS-NUM | | 3 | 是 | FS6 |
| FS-PUNCT | | 。 | 是 | FS7 |
| FS-FB | | 空 eastAsia | 是 | FS8 |
| FS-MIX | | A中B | 是 | FS9 |
| FS-KATA | | ア | 是 | FS10 |

### 3.3 可测试性
FontHost 注入 `tests/fixtures/01_function_font_stack/TestFont.ttf`；测 N 只调本 function 入口。禁止 `FontStack::default()`。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 |
|----|------|------|------|------|
| FS1 | FS-LATIN | style | A | Times New Roman |
| FS2 | FS-HAN | | 中 | 宋体 |
| FS3 | FS-HIRA | | あ | 宋体 |
| FS4 | FS-HANG | | 한 | 宋体 |
| FS5 | FS-SP | | space | ascii |
| FS6 | FS-NUM | | 3 | ascii |
| FS7 | FS-PUNCT | | 。 | 宋体 |
| FS8 | FS-FB | eastAsia="" | 中 | fontName |
| FS9 | FS-MIX | | A中B | 交替 |
| FS10 | FS-KATA | | ア | 宋体 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FaceSelect.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/FaceSelect.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_face_select

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_face_select \
 \
  --src src/cn/wps/moffice/writer/layout/base/env/FaceSelect.rs \
  --report libs/word-core/layout/features/wr-11-run-metrics-parity/functions/03_function_face_select/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_face_select --json`)
date: 2026-08-24 09:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/FaceSelect.rs` | 31/44 | 70.5% | 33/46 | 71.7% |
| **function total** | **31/44** | **70.5%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.java
```
`getAdjCalcFactor` 在生成模板上改写；`getCharWidths` / `getTextWidth` / `getFontMetrics` / 单例 **不在列表**：`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION */` 停车。产品 I/O 在 env 适配器 + `FontStack`。

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
按 **Java 类** 一行。layout（`// java`）必须 PASS。
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.rs \
  --report libs/word-core/layout/features/wr-11-run-metrics-parity/functions/03_function_face_select/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 01:58

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypefaceTool` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.rs` | **PASS** | `getAdjCalcFactor` | `TypefaceTool`, `getInstance`, `dispose`, `getCharWidths`, `getTextWidth`, `getFontMetrics` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
