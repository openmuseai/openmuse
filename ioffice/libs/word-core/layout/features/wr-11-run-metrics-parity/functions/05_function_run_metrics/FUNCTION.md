# 05_function_run_metrics — 编排 RunMetrics

| 项 | 值 |
|----|----|
| feature | wr-11-run-metrics-parity |
| order | 5 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `measure_run_metrics(stack, style, text) -> RunMetrics` |
| 独立性 | 编排 01–04；仍不 wrap。 |

## 1. 功能说明

### 1.1 一句话
feature 出口：text + TypoFontStyle + 注入 FontStack → per-cp advance、width、ascent/descent/line_gap。

### 1.2 输入
见入口。

### 1.3 输出
见入口返回值。缺 cmap **失败**，不得用字号冒充宽度。

### 1.4 上游
注入 TTF（TestFont.ttf）或纯数值；不跑 wr-12。

### 1.5 下游
05 `RunMetrics` / wr-12 断行只许用这里的宽度。

### 1.6 不变量
1. size_pt = fontSize(twip)/20。
2. 每非控制字符：selectFace → glyph_advance_twip_on_face；控制字符 advance=0。
3. 缺 cmap 整 run 失败。
4. bold/italic 本波不改 hmtx。
5. width = Σ advances；空串 advances=[]、width=0，仍有行格。
6. 禁止探盘。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| FaceSelect / GlyphAdvance / LineMetrics / UnitsToTwip | 01–04 | 见各 FUNCTION | 本波 | — | — |
| measure_run (RunMeasure) | wr-03 | 旧合计宽 | 保留 | 无 per-cp / line_gap | 禁止删旧 API |
| TypefaceTool | Java | getCharWidths 编排 | 单例停车；getAdjCalcFactor 活 | IFontTable | 禁止 FontStack::default |

### 1.8 函数改写列表
- `RunMetrics` 字段与 `measure_run_metrics`
- TypefaceTool::getAdjCalcFactor（已实现）
- TypefaceTool 其余：**不在列表**

## 2. 核心代码

`for ch in text`: select + strict advance；行格取首个非控制字符的 face。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| empty | | width0 | 空 |
| Helo | 12pt | per-cp 金 | 主 |
| 控制符 | A\n\t | 0 宽 | skip |
| 我 / He我llo | | MissingGlyph | 失败 |
| bold+italic | Helo | 同 regular | stub |
| 24pt | He | ×2 | 线性 |
| scheme 名 | Times/宋体 | 可测 A | FontHost |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| RM-EMPTY | measure_run_metrics | 空 | 是 | RM1 |
| RM-HELO | | Helo | 是 | RM2 |
| RM-CTRL | | 控制符 | 是 | RM3 |
| RM-CJK | | 我 | 是 | RM4 |
| RM-MIX | | He我llo | 是 | RM5 |
| RM-ASCII | | A | 是 | RM6 |
| RM-BI | | bold/italic | 是 | RM7 |
| RM-SCALE | | 24pt | 是 | RM8 |
| RM-SUM | | width=Σ | 是 | RM9 |
| RM-GAP | | line_gap | 是 | RM10 |
| RM-HOST | | scheme | 是 | RM11 |
| RM-NODISK | | + adjFactor | 是 | RM12 |

### 3.3 可测试性
FontHost 注入 `tests/fixtures/01_function_font_stack/TestFont.ttf`；测 N 只调本 function 入口。禁止 `FontStack::default()`。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 |
|----|------|------|------|------|
| RM1 | RM-EMPTY | 注入 | "" | width0 |
| RM2 | RM-HELO | | Helo | 金 advances |
| RM3 | RM-CTRL | | A\n\t | 0s |
| RM4 | RM-CJK | | 我 | MissingGlyph |
| RM5 | RM-MIX | | mixed | MissingGlyph |
| RM6 | RM-ASCII | | A | ascii face |
| RM7 | RM-BI | | b+i | 同宽 |
| RM8 | RM-SCALE | | 480 twip | ×2 |
| RM9 | RM-SUM | | | sum |
| RM10 | RM-GAP | | | =hhea |
| RM11 | RM-HOST | scheme | A | 可测 |
| RM12 | RM-NODISK | | | path=None；factor 1/4 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/RunMetrics.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 05_function_run_metrics

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 05_function_run_metrics \
 \
  --src src/cn/wps/moffice/writer/layout/base/env/RunMetrics.rs \
  --src src/cn/wps/moffice/writer/layout/base/env/FaceSelect.rs \
  --report libs/word-core/layout/features/wr-11-run-metrics-parity/functions/05_function_run_metrics/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 05_function_run_metrics --json`)
date: 2026-08-24 09:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/FaceSelect.rs` | 26/44 | 59.1% | 28/46 | 60.9% |
| `src/cn/wps/moffice/writer/layout/base/line/RunMetrics.rs` | 42/64 | 65.6% | 67/100 | 67.0% |
| `src/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.rs` | 8/65 | 12.3% | 9/60 | 15.0% |
| **function total** | **76/173** | **43.9%** |  |  |
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
  --report libs/word-core/layout/features/wr-11-run-metrics-parity/functions/05_function_run_metrics/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 01:58

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypefaceTool` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.rs` | **PASS** | `getAdjCalcFactor` | `TypefaceTool`, `getInstance`, `dispose`, `getCharWidths`, `getTextWidth`, `getFontMetrics` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
