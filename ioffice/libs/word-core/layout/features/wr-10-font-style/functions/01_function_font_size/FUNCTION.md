# 01_function_font_size — 字号 pt→twip

| 项 | 值 |
|----|----|
| feature | wr-10-font-style |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `FontSize::getFontSize(propSet, langType) -> i32` |
| 独立性 | 只读 `PropertySet` + `UnitUtil::PT2TWIPS`；不走 KStyles / FontHost |

## 1. 功能说明

### 1.1 一句话
把 CHP 字号（prop **10** 磅，复杂脚本 **37**）换成与 Java 相同的 twip。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `getFontSize` | `&PropertySet` + `langType` | `FontStyleFactory.getFontSize` |

### 1.3 输出
`i32` twip。默认 **10pt → 200 twip**。`PT2TWIPS(pt) = round(pt * 20)`。

### 1.4 上游
wc-03 已把 `sz` 半磅写入 prop 10 的 **磅** float。本 function 不读 XML。

### 1.5 下游
05 填 `TypoFontStyle.fontSize`。

### 1.6 不变量
1. 非复杂脚本：`PT2TWIPS(getFloat(10, 10.0))`。
2. 复杂脚本（langType 4/5/6/7/8）：先 `PT2TWIPS(getFloat(37, -1))`；若 `<=0` 再走 prop 10。
3. `fontSize <= 0` → 再默认 10pt。
4. `min(fontSize, MAX_FONT_SIZE)`；默认 `MAX = i32::MAX`。SmartFontSize **暂缓**（恒 false）。
5. 不查 FontHost。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `PropertySet.getFloat__int_float` | util | 默认字号 | 是 | — | — |
| `UnitUtil::PT2TWIPS` | util | pt→twip | 是 | — | — |
| `TypoRun.isComplexLangType` | cache | lang 4–8 | 未导出本方法 | 本文件内联等价 switch | 禁止改 TypoRun.rs |
| SmartLayout / 嵌入气球 | Java | 缩字 | 否 | 恒关 | 禁止引入 |

### 1.8 函数改写列表
- `FontSize::getFontSize`
- `FontSize::getFontSizeLimited`（夹紧；并行测试不改全局 MAX）
- `FontSize::isComplexScript`
- `FontSize::setMaxFontSize` / `maxFontSize`
- 本文件无其它方法。

## 2. 核心代码

Java：`FontStyleFactory.getFontSize` / `isComplexScript`。  
Rust：`layout/base/env/FontSize.rs`（新建适配器）。

```text
if complex: size = PT2TWIPS(prop 37, -1)
if size <= 0: size = PT2TWIPS(prop 10, 10)
size = min(size, MAX)
if size <= 0: size = PT2TWIPS(10)
```

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| getFontSize | 缺省 | 空 PS，lang=0 | 200 | 默认 10pt |
| getFontSize | sz 12pt | prop 10=12 | 240 | 主路径 |
| getFontSize | 复杂+37 | lang=5，37=14 | 280 | cs 字号 |
| getFontSize | 复杂无37 | lang=5，仅 10=12 | 240 | cs 回落 |
| getFontSize | 0pt | prop 10=0 | 200 | <=0 回落 |
| getFontSize | clamp | MAX=100，12pt | 100 | MAX 夹紧 |
| isComplexScript | 0 vs 5 | lang | false/true | 分支 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| FS-DEF | getFontSize | 空 PS | 是 | FS1 |
| FS-PT | getFontSize | 12pt | 是 | FS2 |
| FS-H1 | getFontSize | 16pt（标题常用） | 是 | FS3 |
| FS-CS | getFontSize | cs + prop 37 | 是 | FS4 |
| FS-CS-FB | getFontSize | cs 无 37 | 是 | FS5 |
| FS-ZERO | getFontSize | 0pt | 是 | FS6 |
| FS-CLAMP | getFontSize | MAX 夹紧 | 是 | FS7 |
| FS-COMPLEX | isComplexScript | 0/2/5 | 是 | FS8 |

### 3.3 可测试性
`MutablePropertySet` 内存 fixture；测 N 只调 `FontSize::*`，不调 04/05。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 |
|----|------|------|------|------|
| FS1 | FS-DEF | 空 PS | lang=0 | 200 |
| FS2 | FS-PT | 10=12.0 | lang=0 | 240 |
| FS3 | FS-H1 | 10=16.0 | lang=0 | 320 |
| FS4 | FS-CS | 37=14.0 | lang=5 | 280 |
| FS5 | FS-CS-FB | 10=12.0 | lang=5 | 240 |
| FS6 | FS-ZERO | 10=0.0 | lang=0 | 200 |
| FS7 | FS-CLAMP | MAX=100，10=12 | lang=0 | 100；测后恢复 MAX |
| FS8 | FS-COMPLEX | — | 0,2,5 | false,false,true |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontSize.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/FontSize.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_font_size

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_font_size \
  --src src/cn/wps/moffice/writer/layout/base/env/FontSize.rs \
  --report libs/word-core/layout/features/wr-10-font-style/functions/01_function_font_size/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_font_size --json`)
date: 2026-08-23 20:42

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/FontSize.rs` | 25/28 | 89.3% | 36/42 | 85.7% |
| **function total** | **25/28** | **89.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .codex/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.java
```
`getFontSize` / `isComplexScript` / `setMaxFontSize` 在生成模板上改写；其余方法 `DEFERRED_NON_FUNCTION` 停车。已对 Java 双子做方法覆盖 + 邻接校验。

### 3.9 Java↔Rust 结构对齐报告
按 Java 类一行。layout（`// java`）必须 PASS。
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs \
  --report libs/word-core/layout/features/wr-10-font-style/functions/01_function_font_size/FUNCTION.md
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
