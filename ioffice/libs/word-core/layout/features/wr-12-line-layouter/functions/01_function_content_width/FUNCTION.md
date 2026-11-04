# 01_function_content_width — 本行可用宽

| 项 | 值 |
|----|----|
| feature | wr-12-line-layouter |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `ContentWidth::lineMaxWidth(content_width, first_line_indent, first_sub_line) -> i32` |
| 独立性 | 纯整数；不读 TTF、不断行、不分页。 |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
在 wr-02 已扣左右缩进的 `content_width` 上，按是否首行再加减 PAPX **208** 首行缩进，得到本行 `mMaxWidth`。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `lineMaxWidth` | `content_width`（wr-02 twip）+ `first_line_indent`（PAPX **208**，已由 `ParaLayouter.calcFirstLineIndent` 解析；209 字符缩进在上游算完）+ `first_sub_line` | `SubLineLayouter.flow`：`layoutLeft += firstLineIndent`（仅 `firstSubLine`）；`mMaxWidth = layoutRight - layoutLeft` |

FEATURE 钉死：`content_width` = 页宽 − 左右边距 − **本段左右缩进（210/212）**。本 function **禁止再扣 210/212**。

### 1.3 输出
`i32` twip。`layoutRight < layoutLeft` 时夹紧为 0。

### 1.4 上游
wr-02 内容宽；已解析的 `ParaPropEnv.firstLineIndent`。不跑 02 wrap。

### 1.5 下游
02 wrap 的 `max_width`；05 首行与后续行分别取一次。

### 1.6 不变量
1. `first_sub_line == true`：`max_width = content_width - first_line_indent`（悬挂 **208 < 0** 则首行更宽）。
2. `first_sub_line == false`：`max_width = content_width`（不应用 208）。
3. 结果 `max(0, …)`。
4. 不读 209 字符网格；不处理 `hasAutoNums` / 卡片边距 / 电话嵌入超宽回退——loop 列表外停车。
5. `enableLeftIndent == false` 的 `max(layoutLeft, leftIndent)` 分支本波不测（MVP 视为已 enable）。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| wr-02 `content_width` | wr-02 | 已扣 210/212 的整数 | 上游 | — | 禁止在此重算页边距 |
| PAPX 208 | wc-03 / `ParaLayouter.calcFirstLineIndent` | 已解析 twip | 解析在上游 | 209 字符缩进网格 | 禁止实现 char-grid |
| `SubLineLayouter` 卡片/嵌入 | Java | 改 layoutLeft | 否 | 电话/卡片 | 禁止引入 ViewEnv |
| LineLayouter 全类 | Java 双子 | 布局/对齐 | 已生成交错模板；本 function 方法全部 `DEFERRED_NON_FUNCTION`；`cache_run_widths` 保留 | flow/分页（05） | 禁止改 wr-04 `Wrap.rs` |

### 1.8 函数改写列表
- `ContentWidth::lineMaxWidth`
- Java `SubLineLayouter.flow` 缩进块语义（门面，不整方法移植）
- LineLayouter：本 function **不在改写列表**；`todo!("DEFERRED_NON_FUNCTION")` 停车

## 2. 核心代码

Java（`SubLineLayouter.flow`，MVP 只取 enableLeftIndent + firstSubLine；右缩进已在 wr-02 扣过）：

```text
if firstSubLine: layoutLeft += firstLineIndent
mMaxWidth = layoutRight - layoutLeft
```

Rust：`layout/base/env/ContentWidth.rs`（loop 新建）。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| lineMaxWidth | 首行 | w=5000, ind=480, first=true | 4520 | 扣 208 |
| lineMaxWidth | 后续行 | 同上 first=false | 5000 | 不扣 208 |
| lineMaxWidth | 悬挂 | ind=-480, first=true | 5480 | 负缩进 |
| lineMaxWidth | 0 缩进 | ind=0 | 5000 | 与后续相同 |
| lineMaxWidth | 过扣 | ind=6000, first=true | 0 | 夹紧 |
| lineMaxWidth | 不双扣 | 传入已扣 210 的 w | 不再减 leftIndent | 防双扣 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| CW-FIRST | lineMaxWidth | 首行正缩进 | 是 | CW1 |
| CW-REST | lineMaxWidth | 非首行 | 是 | CW2 |
| CW-HANG | lineMaxWidth | 悬挂负值 | 是 | CW3 |
| CW-ZERO | lineMaxWidth | 208=0 | 是 | CW4 |
| CW-CLAMP | lineMaxWidth | 缩进 > 宽 | 是 | CW5 |
| CW-NODBL | lineMaxWidth | 不减 210/212 | 是 | CW6 |

### 3.3 可测试性
纯函数；测 N 只调本入口。禁止启动 `LineLayouter.flow` / 分页。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| CW1 | CW-FIRST | — | 5000, 480, true | 4520 | 首行未扣 208 |
| CW2 | CW-REST | — | 5000, 480, false | 5000 | 后续行误扣 |
| CW3 | CW-HANG | — | 5000, -480, true | 5480 | 悬挂方向反 |
| CW4 | CW-ZERO | — | 5000, 0, true | 5000 | 零缩进仍变宽 |
| CW5 | CW-CLAMP | — | 200, 480, true | 0 | 负数宽 |
| CW6 | CW-NODBL | 传入已是 wr-02 宽 | 5000, 480, true | 4520 而非再减 leftIndent | 双扣左右缩进 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/ContentWidth.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/ContentWidth.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_content_width

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_content_width \
  --src src/cn/wps/moffice/writer/layout/base/env/ContentWidth.rs \
  --report libs/word-core/layout/features/wr-12-line-layouter/functions/01_function_content_width/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_content_width --json`)
date: 2026-08-24 11:43

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/ContentWidth.rs` | 8/8 | 100.0% | 9/9 | 100.0% |
| **function total** | **8/8** | **100.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java
```
本 function 产品 I/O 在 `ContentWidth` 门面。LineLayouter 列表外方法停车。本 loop 布局校验 **PASS**（713 条 `// java`）。

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
按 **Java 类** 一行。layout（`// java`）必须 PASS。
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --report libs/word-core/layout/features/wr-12-line-layouter/functions/01_function_content_width/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 03:43

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `LineLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs` | **PASS** | — | `LineLayouter`, `initLineHeightLayouter`, `doInitTextWrapLayouter`, `initTextWrapLayouter`, `onAddWrapDrawing`, `clearTextWrapLayouter`, `getTextWrapLayouter`, `dispose`, `needReflowLine`, `flow`, `processCardInfo`, `isSpaceAfterSoftBreak`, `hasParaBreakIntention`, `isSecBreakNoContinue`, `compress`, `doGetCompressFactor`, `isOverflowBeforeFlow`, `getHAlignHandler`, `getVAlignHandler`, `getDocFieldCollect`, `getGridHandler`, `getCharSpaceHandler`, `getAutoSpaceHandler`, `getPunctSpaceHandler`, `getKerningHandler`, `getCharBorderHandler`, `isOverflow`, `prepareSubLineEnvs`, `preCalcLineHeight`, `postCalcLineHeight`, `calcParaBefore`, `calcParaAfter`, `regulateToTextLine`, `regulateToCache`, `getParaBrcLayouter`, `compressEmptyLine`, `initCollectors`, `regulateSubLinesAndRuns`, `regulateSubLine`, `adjustEmbedRun`, `setLineHeight`, `setAlignLine`, `setAscDesPos`, `setForceVCenterAlign` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
