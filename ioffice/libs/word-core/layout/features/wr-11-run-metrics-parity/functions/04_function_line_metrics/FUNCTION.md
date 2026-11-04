# 04_function_line_metrics — hhea 行格

| 项 | 值 |
|----|----|
| feature | wr-11-run-metrics-parity |
| order | 4 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `LineMetrics::lineBoxTwip(stack, fontName, sizePt) -> LineBox` |
| 独立性 | 只读 hhea；不累加 glyph。 |

## 1. 功能说明

### 1.1 一句话
ascent/descent/**line_gap** 全部来自 hhea，按与 advance 相同公式缩放到 twip。

### 1.2 输入
见入口。

### 1.3 输出
见入口返回值。缺 cmap **失败**，不得用字号冒充宽度。

### 1.4 上游
注入 TTF（TestFont.ttf）或纯数值；不跑 wr-12。

### 1.5 下游
05 `RunMetrics` / wr-12 断行只许用这里的宽度。

### 1.6 不变量
1. scale = size_pt × 20 / upem。
2. line_gap 用 `Face::line_gap()`，不得丢。
3. 未知名 → default 注入面。
4. 与旧 `line_metrics_twip` 的 ascent/descent 一致。
5. 禁止探盘。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| FontStack::line_box_twip | 本波 | 三元组 | 本波 | OS/2 winAscent | 禁止改成 size 冒充 |
| TypefaceTool.getFontMetrics | Java | FontMetrics | 停车 | FreeType | 禁止 JNI |

### 1.8 函数改写列表
- `LineMetrics::lineBoxTwip`
- `FontStack::line_box_twip`
- TypefaceTool.getFontMetrics*：**不在列表**

## 2. 核心代码

hhea × 同一 scale。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| lineBox | 12pt | =hhea | 主 |
| line_gap | | =hhea | 新增 |
| 24pt | | ×2 | 线性 |
| named/default/unknown | | 等 | blob_for |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| LM-AD | lineBoxTwip | 12pt | 是 | LM1 |
| LM-GAP | | lineGap | 是 | LM2 |
| LM-SCALE | | 24pt | 是 | LM3 |
| LM-NAME | | named=def | 是 | LM4 |
| LM-UNK | | 未知名 | 是 | LM5 |
| LM-LEGACY | line_metrics_twip | 对拍 | 是 | LM6 |
| LM-NODISK | | path | 是 | LM7 |

### 3.3 可测试性
FontHost 注入 `tests/fixtures/01_function_font_stack/TestFont.ttf`；测 N 只调本 function 入口。禁止 `FontStack::default()`。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 |
|----|------|------|------|------|
| LM1 | LM-AD | 注入 | 12pt | =hhea |
| LM2 | LM-GAP | | | =line_gap |
| LM3 | LM-SCALE | | 24pt | ×2 |
| LM4 | LM-NAME | | | 相等 |
| LM5 | LM-UNK | | NoSuch | default |
| LM6 | LM-LEGACY | | | pair 一致 |
| LM7 | LM-NODISK | | | path=None |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/LineMetrics.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/LineMetrics.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_line_metrics

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 04_function_line_metrics \
  --src src/cn/wps/moffice/writer/layout/base/env/LineMetrics.rs \
  --report libs/word-core/layout/features/wr-11-run-metrics-parity/functions/04_function_line_metrics/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_line_metrics --json`)
date: 2026-08-24 09:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/LineMetrics.rs` | 12/12 | 100.0% | 11/12 | 91.7% |
| **function total** | **12/12** | **100.0%** |  |  |
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
  --report libs/word-core/layout/features/wr-11-run-metrics-parity/functions/04_function_line_metrics/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 01:58

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypefaceTool` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.rs` | **PASS** | `getAdjCalcFactor` | `TypefaceTool`, `getInstance`, `dispose`, `getCharWidths`, `getTextWidth`, `getFontMetrics` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
