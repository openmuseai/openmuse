# 03_function_font_names — ascii / eastAsia 族名

| 项 | 值 |
|----|----|
| feature | wr-10-font-style |
| order | 3 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `FontNames::getAscii` / `getEastAsia` / `getResolvedFontName` / `applyHint` |
| 独立性 | 只读 rFonts 槽与 hint；不查 FontHost |

## 1. 功能说明

### 1.1 一句话
抽出 ascii（prop **3**）与 eastAsia（prop **4**），并按 Java hint/lang **最小集**解析当前 run 用的族名。

### 1.2 输入
`&PropertySet` + `langType`（调用方区分拉丁 vs 中文）。

### 1.3 输出
- `ascii`: 默认 `"Times New Roman"`
- `eastAsia`: 默认 `"宋体"`
- `resolved`: lang 2/3 → eastAsia；default/0/1 → ascii；hint 可改写 lang

### 1.4 上游
fixture PS。主题字体 **暂缓**。

### 1.5 下游
05 填 `TypoFontStyle.ascii` / `eastAsia` / `fontName`。

### 1.6 不变量
1. hint **40**=1 → langType=2（fareast）。
2. hint **40**=2 且 lang 不是 6/7 → langType=5（complex 名槽，本波仍解析 36；最小集测试以 1 为主）。
3. lang 2/3：`getString(4, "宋体")`。
4. default：`getString(3, "Times New Roman")`。
5. 4/5/7/8/9/6 的 35/36 槽按 Java switch **实现**；必测矩阵覆盖 default + CJK + hint1。
6. 不调用 FontHost.get。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `getString__int_string` / `contains` | util | 族名 | 是 | — | — |
| FontHost / FontTable | font | 回退族 | 否 | wr-11 前不查 | 禁止引入 |
| 主题 font | parse | theme | 否 | stub | 禁止改 parse |

### 1.8 函数改写列表
- `FontNames::getAscii`
- `FontNames::getEastAsia`
- `FontNames::applyHint`
- `FontNames::getResolvedFontName`
- 本文件无其它方法。

## 2. 核心代码

Java：`createTypeface` 的 hint switch + fontName switch（到取 string 为止）。  
Rust：`layout/base/env/FontNames.rs`。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| getAscii | 空 | — | Times New Roman | 默认 |
| getEastAsia | 空 | — | 宋体 | 默认 |
| getAscii | prop 3 | Arial | Arial | 直接 |
| getEastAsia | prop 4 | 黑体 | 黑体 | 直接 |
| getResolved | lang=0 | 两套名 | ascii | 拉丁 |
| getResolved | lang=2 | 两套名 | eastAsia | CJK |
| applyHint | 40=1 | lang=0 | lang=2，resolved=eastAsia | hint |
| getResolved | lang=5 | prop 36 | 36 槽 | cs 名 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| FN-DEF | ascii/eastAsia | 空 | 是 | FN1 |
| FN-ASCII | getAscii | 3=Arial | 是 | FN2 |
| FN-EA | getEastAsia | 4=黑体 | 是 | FN3 |
| FN-LATIN | resolved | lang=0 | 是 | FN4 |
| FN-CJK | resolved | lang=2 | 是 | FN5 |
| FN-HINT | applyHint+resolved | hint=1 | 是 | FN6 |
| FN-CS | resolved | lang=5,36 | 是 | FN7 |

### 3.3 可测试性
内存 PS；不调 FontHost / 04 / 05。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 |
|----|------|------|------|------|
| FN1 | FN-DEF | 空 | — | TNR / 宋体 |
| FN2 | FN-ASCII | 3=Arial | — | Arial |
| FN3 | FN-EA | 4=黑体 | — | 黑体 |
| FN4 | FN-LATIN | 3=Arial,4=黑体 | lang=0 | resolved=Arial |
| FN5 | FN-CJK | 同上 | lang=2 | resolved=黑体 |
| FN6 | FN-HINT | 同上+40=1 | lang=0 | resolved=黑体 |
| FN7 | FN-CS | 36=Tahoma | lang=5 | resolved=Tahoma |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontNames.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/FontNames.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_font_names

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_font_names \
  --src src/cn/wps/moffice/writer/layout/base/env/FontNames.rs \
  --report libs/word-core/layout/features/wr-10-font-style/functions/03_function_font_names/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_font_names --json`)
date: 2026-08-23 20:42

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/FontNames.rs` | 22/32 | 68.8% | 35/56 | 62.5% |
| **function total** | **22/32** | **68.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .codex/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontNames.rs
```
新建适配器；邻接 `// java` 必须过。

### 3.9 Java↔Rust 结构对齐报告
按 Java 类一行。layout（`// java`）必须 PASS。
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs \
  --report libs/word-core/layout/features/wr-10-font-style/functions/03_function_font_names/FUNCTION.md
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
