# 02_function_run_flags — 粗体 / 斜体

| 项 | 值 |
|----|----|
| feature | wr-10-font-style |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `RunFlags::getBold` / `getItalic` |
| 独立性 | 只读 CHP；复杂脚本走 38/39，否则 5/6 |

## 1. 功能说明

### 1.1 一句话
从已层叠的 CHP 取出 bold / italic，规则与 `FontStyleFactory.createTypeface` 一致。

### 1.2 输入
`&PropertySet` + `langType`。

### 1.3 输出
`(bool, bool)` 或分别 `getBold` / `getItalic`。缺省均为 `false`。

### 1.4 上游
测本 function 用 fixture PS；产品路径吃 04 的合并结果。

### 1.5 下游
05 `TypoFontStyle.bold` / `italic`。

### 1.6 不变量
1. 非 cs：`getBoolean(5,false)` / `getBoolean(6,false)`。
2. cs（lang 4–8）：prop **38** / **39**。
3. 不合成 FontHost style 位；不读主题。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `PropertySet.getBoolean__int_boolean` | util | 默认 false | 是 | — | — |
| `FontSize::isComplexScript` | 01 | lang 判定 | 是（01） | — | 禁止改 TypoRun.rs |

### 1.8 函数改写列表
- `RunFlags::getBold`
- `RunFlags::getItalic`
- 本文件无其它方法。

## 2. 核心代码

Java：`createTypeface` 内 `getBoolean(5/6)` 或 `38/39`。  
Rust：`layout/base/env/RunFlags.rs`。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| getBold | 空 | lang=0 | false | 默认 |
| getBold | prop 5 | true | true | 直接 b |
| getItalic | prop 6 | true | true | 直接 i |
| getBold | cs 38 | lang=5 | true | cs |
| getItalic | cs 无 39 有 6 | lang=5 | false | cs 不读 6 |
| 二者 | 5+6 | lang=0 | true,true | 组合 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| RF-DEF | getBold/getItalic | 空 PS | 是 | RF1 |
| RF-B | getBold | b=true | 是 | RF2 |
| RF-I | getItalic | i=true | 是 | RF3 |
| RF-BI | 二者 | b+i | 是 | RF4 |
| RF-CS-B | getBold | lang=5, 38 | 是 | RF5 |
| RF-CS-IGN | getItalic | lang=5, 仅 6 | 是 | RF6 |

### 3.3 可测试性
内存 `MutablePropertySet`；不调 04/05。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 |
|----|------|------|------|------|
| RF1 | RF-DEF | 空 | lang=0 | F,F |
| RF2 | RF-B | 5=true | lang=0 | T,F |
| RF3 | RF-I | 6=true | lang=0 | F,T |
| RF4 | RF-BI | 5+6 | lang=0 | T,T |
| RF5 | RF-CS-B | 38=true | lang=5 | T,F |
| RF6 | RF-CS-IGN | 6=true | lang=5 | F,F |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/RunFlags.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/RunFlags.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_run_flags

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_run_flags \
  --src src/cn/wps/moffice/writer/layout/base/env/RunFlags.rs \
  --report libs/word-core/layout/features/wr-10-font-style/functions/02_function_run_flags/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_run_flags --json`)
date: 2026-08-23 20:42

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/RunFlags.rs` | 10/10 | 100.0% | 16/16 | 100.0% |
| **function total** | **10/10** | **100.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .codex/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/RunFlags.rs
```
新建适配器；邻接 `// java` 必须过。

### 3.9 Java↔Rust 结构对齐报告
按 Java 类一行。layout（`// java`）必须 PASS。
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs \
  --report libs/word-core/layout/features/wr-10-font-style/functions/02_function_run_flags/FUNCTION.md
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
