# 01_function_units_to_twip — font units → twip

| 项 | 值 |
|----|----|
| feature | wr-11-run-metrics-parity |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `UnitsToTwip::unitsToTwip(units, sizePt, upem) -> Result<i32, FontError>` |
| 独立性 | 纯数值换算；可读 TestFont 校验与 hmtx 一致。不选 face、不断行。 |

## 1. 功能说明

### 1.1 一句话
把 hmtx/hhea 的 font units 换成 twip，公式与 FontStack 现网一致。

### 1.2 输入
见入口。

### 1.3 输出
见入口返回值。缺 cmap **失败**，不得用字号冒充宽度。

### 1.4 上游
注入 TTF（TestFont.ttf）或纯数值；不跑 wr-12。

### 1.5 下游
05 `RunMetrics` / wr-12 断行只许用这里的宽度。

### 1.6 不变量
1. `round(units × size_pt × 20 / upem)`。
2. `size_twip` 路径：`size_pt = size_twip / 20`，即 `round(units × size_twip / upem)`。
3. `upem == 0` → `FontErrorKind::Parse`。
4. 禁止再乘一次 20（FEATURE 误读）。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `units_to_twip` | FontStack | 换算 | 本波 | — | 禁止改 covering_blob 语义 |
| TypefaceTool | Java 双子 | 布局/对齐 | getAdjCalcFactor 已实现；其余停车 | IFontTable/FreeType | 禁止接 Android Paint |

### 1.8 函数改写列表
- `UnitsToTwip::unitsToTwip`
- `UnitsToTwip::unitsToTwipFromSizeTwip`
- `FontStack::units_to_twip` / `units_to_twip_from_size_twip`
- TypefaceTool：仅 `getAdjCalcFactor`；其余 `DEFERRED_NON_FUNCTION`

## 2. 核心代码

Rust：`UnitsToTwip.rs` 委托 `FontStack::units_to_twip`。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| unitsToTwip | 1000u 12pt upem1000 | | 240 | 主路径 |
| FromSizeTwip | 240 twip | | 同 12pt | 等价 |
| zero | 0 units | | 0 | 边界 |
| upem0 | | Parse | 失败 |
| 24pt | | 2×12pt | 线性 |
| round | 1031/2048/12 | 121 | 舍入 |
| TestFont A | hmtx | 与 Face 金值 | 真实字体 |
| misread | ×20 again | 不等于 | 防 20 倍 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| UT-PT | unitsToTwip | 12pt | 是 | UT1 |
| UT-TWIP | FromSizeTwip | 等价 | 是 | UT2 |
| UT-ZERO | unitsToTwip | 0 | 是 | UT3 |
| UT-UPEM0 | unitsToTwip | 失败 | 是 | UT4 |
| UT-SCALE | unitsToTwip | 24pt | 是 | UT5 |
| UT-ROUND | unitsToTwip | 半入 | 是 | UT6 |
| UT-TTF | unitsToTwip | TestFont A | 是 | UT7 |
| UT-NO20 | FromSizeTwip | 非误读 | 是 | UT8 |

### 3.3 可测试性
FontHost 注入 `tests/fixtures/01_function_font_stack/TestFont.ttf`；测 N 只调本 function 入口。禁止 `FontStack::default()`。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 |
|----|------|------|------|------|
| UT1 | UT-PT | — | 1000,12,1000 | 240 |
| UT2 | UT-TWIP | — | 1000,240twip,1000 | 240 |
| UT3 | UT-ZERO | — | 0 | 0 |
| UT4 | UT-UPEM0 | — | upem=0 | Parse |
| UT5 | UT-SCALE | — | 24 vs 12 | ×2 |
| UT6 | UT-ROUND | — | 1031,12,2048 | 121 |
| UT7 | UT-TTF | TestFont | A hmtx | 相等 |
| UT8 | UT-NO20 | — | 240 twip | 240 而非 4800 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/UnitsToTwip.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/UnitsToTwip.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_units_to_twip

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_units_to_twip \
  --src src/cn/wps/moffice/writer/layout/base/env/UnitsToTwip.rs \
  --report libs/word-core/layout/features/wr-11-run-metrics-parity/functions/01_function_units_to_twip/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_units_to_twip --json`)
date: 2026-08-24 09:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/UnitsToTwip.rs` | 6/6 | 100.0% | 12/12 | 100.0% |
| **function total** | **6/6** | **100.0%** |  |  |
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
  --report libs/word-core/layout/features/wr-11-run-metrics-parity/functions/01_function_units_to_twip/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 01:58

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypefaceTool` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.rs` | **PASS** | `getAdjCalcFactor` | `TypefaceTool`, `getInstance`, `dispose`, `getCharWidths`, `getTextWidth`, `getFontMetrics` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
