# 03_function_page_commands — 页命令编排

| 项 | 值 |
|---|---|
| feature | wr-15-style-commands |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `emit_pages` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
每页严格输出 BeginPage → page-background Rect → line DrawText* → EndPage；`CommandDocument.version=2`。结构标记不产生 Painter 图元。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| 02 style commands | 本 feature | line commands | 是 | — | — |
| RenderCommand | wr-07 | page/text/rect markers | 是 | highlight/border | existing tags |
| Painter/Normalize | API | ignore structural markers | 是 | — | glyph semantics |

## 2. 核心代码
稳定页序串接；空文档无命令，空页仍有三条结构命令。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| emit_pages | empty/one/two pages | document | version/bracket/order |
### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| PC-EMPTY | empty | 是 | PC1 |
| PC-BRACKET | one page | 是 | PC2 |
| PC-ORDER | two pages | 是 | PC3 |
| PC-LINES | page contains line | 是 | PC4 |
### 3.3 可测试性
内存 page，不调用 Flutter。
### 3.4 用例表
| test | 期望 |
|---|---|
| PC1 | v2 + [] |
| PC2 | begin/rect/end |
| PC3 | End(0) before Begin(1) |
### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/rendercommand/PageCommands.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/rendercommand/PageCommands.rs`
### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_page_commands
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 03_function_page_commands --src src/cn/wps/moffice/writer/rendercommand/PageCommands.rs --report libs/word-core/layout/features/wr-15-style-commands/functions/03_function_page_commands/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_page_commands --json`)
date: 2026-08-25 02:04

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/rendercommand/PageCommands.rs` | 23/23 | 100.0% | 26/26 | 100.0% |
| **function total** | **23/23** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`TypoRun.java ↔ TypoRun.rs`：**PASS**；本 function 不编辑 Java 双子。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoRun.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoRun.rs --report libs/word-core/layout/features/wr-15-style-commands/functions/03_function_page_commands/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 18:01

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoRun` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoRun.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoRun.rs` | **PASS** | `needTransToSpace`, `isSpace`, `isTab`, `isWeakHeightGraphs`, `isWeakBdrForHeight`, `isComplexLangType`, `isFarEastLangType`, `isWestLangType`, `isTypoPageBreak`, `isLineBreak`, `isPunct`, `isNormal`, `isNONBMP`, `getTypeStr` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
