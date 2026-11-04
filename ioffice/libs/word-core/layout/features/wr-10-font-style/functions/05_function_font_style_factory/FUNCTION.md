# 05_function_font_style_factory — 编排 TypoFontStyle

| 项 | 值 |
|----|----|
| feature | wr-10-font-style |
| order | 5 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `FontStyleFactory::create` / `createFromRun` |
| 独立性 | 编排 01–04；**不**调 FontHost / ascent |

## 1. 功能说明

### 1.1 一句话
feature 出口：一个 run → `TypoFontStyle`（ascii / eastAsia / size twip / bold / italic）。

### 1.2 输入
- `create(merged, langType)`：已层叠 CHP。
- `createFromRun(chpx, papx, styles, langType)`：先 04 再 create。

### 1.3 输出
`TypoFontStyle`：`ascii`, `eastAsia`, `fontName`（解析名）, `fontSize`, `bold`, `italic`。可选 `fontColor`（prop 11，默认 -16777216）、`underline`（prop 8，默认 0）给 wr-15，**不**进 wr-12。

### 1.4 上游
01–04。Heading 验收走 `createFromRun` + 04 fixture。

### 1.5 下游
wr-11 度量。本波不测 advance。

### 1.6 不变量
1. `create` 先 `applyHint` 再 01–03。
2. 不填 typeface / maxAscent（wr-11）。
3. 不改 PAPX jc。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| FontSize / RunFlags / FontNames / StyleMerge | 01–04 | 见各 FUNCTION | 本波 | — | — |
| Java `create` 的 ITypeface / calcAscentAndDescent | FontHost | 选字+行高 | 否 | wr-11 | 禁止引入 FontHost |
| `TypoFontStyle` 全量 flag/SHD/BRC | cache | 装饰 | 否 | wr-15 可选色/下划线已带最小字段 | 禁止移植全类 |

### 1.8 函数改写列表
- `TypoFontStyle` 字段（最小集）
- `FontStyleFactory::create`
- `FontStyleFactory::createFromRun`
- Java `create`/`createTypeface`/`calcAscentAndDescent`/`createMath*`：**不在列表**（无 typeface）

## 2. 核心代码

```text
merged = StyleMerge::merge(...)      // createFromRun
lang = FontNames::applyHint(merged, lang)
TypoFontStyle { ascii, eastAsia, fontName, fontSize, bold, italic, ... }
```

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| create | 空 PS | lang=0 | 10pt TNR 不粗 | 默认 |
| create | b+sz+ascii | | 对应字段 | 直接 rPr |
| create | lang=2 eastAsia | | fontName=宋体槽 | CJK |
| create | hint=1 | lang=0 | 走 eastAsia | hint |
| createFromRun | Heading basedOn | | 16pt bold | 标题 |
| createFromRun | 直接 i 覆盖 | | italic | 覆盖 |
| create | color/u | 11/8 | 可选字段 | wr-15 最小 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| FF-DEF | create | 空 | 是 | FF1 |
| FF-DIRECT | create | b/i/sz/ascii | 是 | FF2 |
| FF-CJK | create | lang=2 | 是 | FF3 |
| FF-HINT | create | hint=1 | 是 | FF4 |
| FF-H1 | createFromRun | Heading | 是 | FF5 |
| FF-OVR | createFromRun | 直接 italic | 是 | FF6 |
| FF-COLOR | create | color+u | 是 | FF7 |

### 3.3 可测试性
`create` 不建 KStyles；`createFromRun` 复用 04 的 Heading fixture 形态，不跑 zip。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 |
|----|------|------|------|------|
| FF1 | FF-DEF | 空 PS | lang=0 | size=200, bold=F, ascii=TNR, eastAsia=宋体 |
| FF2 | FF-DIRECT | 5,6,10=12,3=Arial | lang=0 | 240, T,T, Arial |
| FF3 | FF-CJK | 4=黑体 | lang=2 | fontName=黑体 |
| FF4 | FF-HINT | 4=黑体,40=1 | lang=0 | fontName=黑体 |
| FF5 | FF-H1 | Normal+H1 同 04 | lang=0 | 320 + bold |
| FF6 | FF-OVR | H1 + chpx 6=true | lang=0 | italic true，仍 16pt |
| FF7 | FF-COLOR | 11=255, 8=1 | lang=0 | fontColor=255, underline=1 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `.../env/FontStyleFactory.rs` + `TypoFontStyle.rs`  
test: `.../tests/.../env/FontStyleFactory.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 05_function_font_style_factory

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 05_function_font_style_factory \
  --src src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs \
  --src src/cn/wps/moffice/writer/layout/base/env/TypoFontStyle.rs \
  --report libs/word-core/layout/features/wr-10-font-style/functions/05_function_font_style_factory/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 05_function_font_style_factory --json`)
date: 2026-08-24 00:35

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs` | 22/22 | 100.0% | 37/37 | 100.0% |
| `src/cn/wps/moffice/writer/layout/base/env/TypoFontStyle.rs` | 12/15 | 80.0% | 7/10 | 70.0% |
| **function total** | **34/37** | **91.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .codex/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.java
```
`FontStyleFactory.rs` 已走 java2rust 模板（277 条 `// java`，`--java` 覆盖 PASS）。`TypoFontStyle.java` 全叶模板仍待同一流水线；当前 `TypoFontStyle.rs` 仍是 wr-10 最小投影。

### 3.9 Java↔Rust 结构对齐报告
按 Java 类一行。layout（`// java`）必须 PASS。
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs \
  --report libs/word-core/layout/features/wr-10-font-style/functions/05_function_font_style_factory/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-23 18:09

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `FontStyleFactory` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs` | **PASS** | `setMaxFontSize`, `setSmartFontSize`, `getFontSize`, `isComplexScript` | `setIsInEmbedBalloonMode`, `setEmebedFontSize`, `createFontStyle`, `getSubposition`, `getScriptFontsizeScale`, `getSubOrSuperScriptFontSize`, `create`, `calcAscentAndDescent`, `createMathFontStyle`, `updateStyleResult`, `createTypeface`, `createMathTypeface` | — | **PASS** |
| `StyleResult` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs` | **PASS** | — | `reset` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
