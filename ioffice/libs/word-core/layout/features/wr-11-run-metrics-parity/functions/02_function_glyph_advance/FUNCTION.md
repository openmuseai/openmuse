# 02_function_glyph_advance — 单字 hmtx advance

| 项 | 值 |
|----|----|
| feature | wr-11-run-metrics-parity |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `GlyphAdvance::glyphAdvanceTwip(stack, fontName, ch, sizePt)` |
| 独立性 | 只测 named-face hmtx；不编排整 run、不断行。 |

## 1. 功能说明

### 1.1 一句话
在**选定族名**对应的注入 TTF 上读 cmap/hmtx，换成 twip。缺 cmap 失败，不走 covering 链。

### 1.2 输入
见入口。

### 1.3 输出
见入口返回值。缺 cmap **失败**，不得用字号冒充宽度。

### 1.4 上游
注入 TTF（TestFont.ttf）或纯数值；不跑 wr-12。

### 1.5 下游
05 `RunMetrics` / wr-12 断行只许用这里的宽度。

### 1.6 不变量
1. 只查 `blob_for(name)`，不扫其它注入面。
2. 缺 glyph → `MissingGlyph`，不得 `width=size`。
3. 未知名回落到 default 注入面（与 wr-03 blob_for 一致），仍无 cmap 则失败。
4. 24pt = 2×12pt（无 hinting）。
5. 禁止 FontStack::default()。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| FontStack::glyph_advance_twip_on_face | wr-03+本波 | 严格 face | 本波新增 | covering_blob 仍给 wr-03 | 禁止改成静默 size |
| FontHost | wr-03 | 注入 | 已有 | — | 禁止探盘 |
| TypefaceTool.getCharWidths | Java | 多 overload | 停车 | FreeType ITypeface | 禁止 JNI |

### 1.8 函数改写列表
- `GlyphAdvance::glyphAdvanceTwip`
- `FontStack::glyph_advance_twip_on_face`
- TypefaceTool.getCharWidths*：**不在列表**

## 2. 核心代码

严格 face：`has_glyph` 失败即 `MissingGlyph`。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| A/H/e | TestFont | 12pt | hmtx 金 | 主路径 |
| 我/w | | MissingGlyph | 失败 |
| 24pt | A | ×2 | 线性 |
| 未知名 | A | default 金 | blob_for |
| Helo | 逐字 | 金 | 多字母 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| GA-A | glyphAdvanceTwip | A | 是 | GA1 |
| GA-H | | H | 是 | GA2 |
| GA-CJK | | 我 | 是 | GA3 |
| GA-W | | w | 是 | GA4 |
| GA-SCALE | | 24pt | 是 | GA5 |
| GA-NAME | | TestFont | 是 | GA6 |
| GA-UNK | | 未知名 | 是 | GA7 |
| GA-STRICT | | 我 | 是 | GA8 |
| GA-RUN | | Helo | 是 | GA9 |
| GA-NODISK | | default_path | 是 | GA10 |

### 3.3 可测试性
FontHost 注入 `tests/fixtures/01_function_font_stack/TestFont.ttf`；测 N 只调本 function 入口。禁止 `FontStack::default()`。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 |
|----|------|------|------|------|
| GA1 | GA-A | 注入 | A 12pt | =hmtx |
| GA2 | GA-H | | H | =hmtx |
| GA3 | GA-CJK | | 我 | MissingGlyph |
| GA4 | GA-W | | w | MissingGlyph |
| GA5 | GA-SCALE | | 24pt | ×2 |
| GA6 | GA-NAME | | named=default | 等 |
| GA7 | GA-UNK | | NoSuchFont A | default hmtx |
| GA8 | GA-STRICT | | 我 | 不伪造宽 |
| GA9 | GA-RUN | | Helo | 逐字金 |
| GA10 | GA-NODISK | | | path=None |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/GlyphAdvance.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/GlyphAdvance.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_glyph_advance

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_glyph_advance \
  --src src/cn/wps/moffice/writer/layout/base/env/GlyphAdvance.rs \
  --report libs/word-core/layout/features/wr-11-run-metrics-parity/functions/02_function_glyph_advance/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_glyph_advance --json`)
date: 2026-08-24 09:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/GlyphAdvance.rs` | 8/8 | 100.0% | 7/7 | 100.0% |
| **function total** | **8/8** | **100.0%** |  |  |
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
  --report libs/word-core/layout/features/wr-11-run-metrics-parity/functions/02_function_glyph_advance/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 01:58

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypefaceTool` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/tools/TypefaceTool.rs` | **PASS** | `getAdjCalcFactor` | `TypefaceTool`, `getInstance`, `dispose`, `getCharWidths`, `getTextWidth`, `getFontMetrics` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
