# 01_function_style_runs — 样式 run 合并

| 项 | 值 |
|---|---|
| feature | wr-15-style-commands |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `coalesce_style_runs` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
仅当 font_id/size_twip/bold/color 全相同才合并相邻 fragment；累加同源 measured advance；空 run 丢弃。Heading、bold、ascii/eastAsia 均形成边界。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| wr-10 style | wr-10 | size/bold/font/color | 是 | highlight | style merge |
| wr-11 metrics | wr-11 | run advance | 是 | xpos | FontHost |
| TypoRun | cache Java product | run boundary | 已有模板 | arena | Java anchors |

## 2. 核心代码
线性稳定合并，不跨样式边界，不重新测宽。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| coalesce | same/bold/size/font/empty | runs | merge/preserve/drop |
### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| SR-SAME | same style | 是 | SR1 |
| SR-BOLD | bold boundary | 是 | SR2 |
| SR-SIZE | heading boundary | 是 | SR3 |
| SR-FONT | script font boundary | 是 | SR4 |
| SR-EMPTY | empty | 是 | SR5 |
### 3.3 可测试性
纯 owned run fixture。
### 3.4 用例表
| test | 期望 |
|---|---|
| SR1 | "a"+"b" and advance=30 |
| SR2–4 | two runs |
| SR5 | zero runs |
### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/rendercommand/StyleRuns.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/rendercommand/StyleRuns.rs`
### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_style_runs
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 01_function_style_runs --src src/cn/wps/moffice/writer/rendercommand/StyleRuns.rs --report libs/word-core/layout/features/wr-15-style-commands/functions/01_function_style_runs/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_style_runs --json`)
date: 2026-08-25 01:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/rendercommand/StyleRuns.rs` | 22/22 | 100.0% | 35/35 | 100.0% |
| **function total** | **22/22** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`TypoRun.java ↔ TypoRun.rs`：**PASS**。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoRun.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoRun.rs --report libs/word-core/layout/features/wr-15-style-commands/functions/01_function_style_runs/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 18:01

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoRun` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoRun.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoRun.rs` | **PASS** | `needTransToSpace`, `isSpace`, `isTab`, `isWeakHeightGraphs`, `isWeakBdrForHeight`, `isComplexLangType`, `isFarEastLangType`, `isWestLangType`, `isTypoPageBreak`, `isLineBreak`, `isPunct`, `isNormal`, `isNONBMP`, `getTypeStr` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
