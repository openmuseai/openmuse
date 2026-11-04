# 02_function_wrap — Java 断点折行

| 项 | 值 |
|----|----|
| feature | wr-12-line-layouter |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `LineWrap::wrap(advances, chars, max_width) -> Vec<LineRange>` |
| 独立性 | 只吃 wr-11 的 per-cp 宽 + 01 的 max_width；不跑对齐/行高/分页。 |
| status | **pass** |

`LineRange`：`start_cp`, `end_cp`（半开）, `width`。不发明第二套行产品类型；05 再写入 `TypoTextLine`。

## 1. 功能说明

### 1.1 一句话
按 Java `SubLineLayouter.addRun` 的溢出与 `SubLineLayouterBase.rollback` 的回退，把一段 per-cp 宽折成多行 cp 区间。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `wrap` | 与文本等长的 `advances[]`（wr-11 twip）+ `chars` + `max_width`（01） | `addRun`：`run.getRightWithBdr() - mMaxWidth >= 2` 才溢出；然后 `doAddExcessGraphs` → `rollback` |

禁止：wr-04 `wrap()` 贪心；均分 run 宽；`width = fontSize`。

### 1.3 输出
`Vec<LineRange>`。空段 / 仅 `\r`：行数以 Java 金文件为准（loop 钉死 0 或 1）。

### 1.4 上游
01 `max_width`（首行/后续可由 05 每行重算）；wr-11 advances。测 N 用整数 fixture，不调 FontHost。

### 1.5 下游
03 用每行 `width`；05 写 `start_cp`/`end_cp`。

### 1.6 不变量
1. **溢出阈值 2 twip**：`accum - max_width < 2` 仍留在本行（Java `>= 2` 才溢出）。
2. 拉丁：空格为回退点；超宽单词无空格 → 强制在 `max_width` 处截断（至少一个 cp，防死循环）。
3. CJK 无空格：每个汉字图元可断（Java 一 char 一 Graphs）；**不**做 kinsoku / 标点挤压。
4. `\r`（及 Java para-break runType）结束本段最后一行，不折到下一逻辑行。
5. NBSP（`\u00A0`）不当普通空格断点。
6. 禁止调用 `layout/base/text/Wrap.rs`。
7. tab / 软连字符 / bidi / 压缩整行塞入：stub 或列表外。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| wr-11 `advance[]` | wr-11 | per-cp twip | 是 | — | 禁止改 FontStack |
| 01 max_width | 本 feature | i32 | 是 | — | — |
| wr-04 `Wrap.rs` | wr-04 Hello | 贪心空格 | 是（**错误语义**） | 保留给 MVP | **禁止改/调用** |
| `SubLineLayouterBase.rollback` kinsoku | Java | 标点回退 | 否 | 另开 feature | 禁止移植 AsiaTypography |
| `LineCompressTool.compress` | Java | 整行压缩后仍 fit | 否 | 挤压 feature | 溢出后当压缩失败再 rollback |
| tab / hyphen | Java | TabHandler | 否 | stub | 禁止排 tab |

### 1.8 函数改写列表
- `LineWrap::wrap` / `LineRange`
- 溢出阈值与空格回退 / CJK 逐字 / 强制截断 / `\r`
- LineLayouter / SubLineLayouter 其余：**不在列表**

## 2. 核心代码

Java 伪逻辑（FEATURE + `SubLineLayouter.addRun`）：

```text
accum = 0
for each cp:
  if char == '\r': 封行；结束段
  if accum + adv - max_width >= 2:
    若有空格/CJK 断点 → 回退到断点封行
    否则强制截断（本行至少 1 cp）
  else: 吸收
```

Rust：`layout/base/env/LineWrap.rs`（loop 新建）。**不要**写进 `text/Wrap.rs`。

行尾空格是否留在本行：以 `SubLineLayouterBase.rollback` / 空格 Graphs 为准，loop 对照 Java 后再改 §3.4 期望，不得按「看起来合理」编。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| wrap | 单行放得下 | 宽足够 | 1 行全段 | 主路径 |
| wrap | 拉丁空格 | "aa bb" 只放得下 aa | 在空格折 | 回退点 |
| wrap | CJK | 每字宽 400，max=500 | 一字一行 | 无空格可断 |
| wrap | 超宽单词 | "Hello" 无空格超宽 | 强制切 | 防死循环 |
| wrap | 阈值 1 twip | accum = max+1 | 仍本行 | Java <2 |
| wrap | 阈值 2 twip | 第二字使溢出 ≥2 | 折行 | >=2 |
| wrap | `\r` | 文+CR | CR 在末行 | 段末 |
| wrap | NBSP | a+NBSP+b 超宽 | 不在 NBSP 折 | 不断点 |
| wrap | 空 | [] | 0 或 1 空行 | 金文件 |
| wrap | 标题更大字 | 更大 advance | 更少字/行 | FEATURE in |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| WP-FIT | wrap | 全放入 | 是 | WP1 |
| WP-SPACE | wrap | 空格回退 | 是 | WP2 |
| WP-CJK | wrap | 无空格中文 | 是 | WP3 |
| WP-FORCE | wrap | 超宽拉丁词 | 是 | WP4 |
| WP-SLACK | wrap | 溢出 1 twip | 是 | WP5 |
| WP-GE2 | wrap | 溢出 2 twip | 是 | WP6 |
| WP-CR | wrap | `\r` 段末 | 是 | WP7 |
| WP-NBSP | wrap | NBSP | 是 | WP8 |
| WP-EMPTY | wrap | 空 advances | 是 | WP9 |
| WP-MULTI | wrap | 折 N 行 | 是 | WP10 |
| WP-HEAD | wrap | 更大 advance | 是 | WP11 |
| WP-NO04 | wrap | 不调用 Wrap.rs | 是 | WP12 |

### 3.3 可测试性
构造 `advances` 整数表 + 字符串；不启动 FontHost / `TextLayouter.flow`。用 01 输出或字面 `max_width`。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| WP1 | WP-FIT | — | "ab" adv=[100,100] max=500 | 1 行 [0,2) w=200 | 误折 |
| WP2 | WP-SPACE | — | "aa bb" 字母 100、空格 50，max=250 | 2 行：[0,3) w=250（含行尾空格）、[3,5) w=200 | 在单词中折 |
| WP3 | WP-CJK | — | "你好" adv=[400,400] max=500 | 两行各 1 字 | 整段溢出 |
| WP4 | WP-FORCE | — | "WW" adv=[300,300] max=400 | 至少 2 行，每行 ≥1 cp | 死循环 |
| WP5 | WP-SLACK | — | 一字 adv=max_width+1 | 1 行 | 1 twip 误折 |
| WP6 | WP-GE2 | — | 两字，第二字使累计溢出 ≥2 | 第二字起新行 | 阈值错 |
| WP7 | WP-CR | — | "a\\r" | 1 行含 CR；不再开下一逻辑行 | 段标错 |
| WP8 | WP-NBSP | — | "a\\u00A0b" 超宽 | 不断在 NBSP | 当普通空格 |
| WP9 | WP-EMPTY | — | 空 | 0 行 | 空错 |
| WP10 | WP-MULTI | — | CJK 5 字 max=1 字宽 | 5 行 | 少折 |
| WP11 | WP-HEAD | — | 同文案更大 advance | 行数 ≥ 小字号 | 标题仍挤一行 |
| WP12 | WP-NO04 | — | 任意 | 测试不 `use …::Wrap::wrap` | 误用贪心 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/LineWrap.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/LineWrap.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_wrap

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_wrap \
  --src src/cn/wps/moffice/writer/layout/base/env/LineWrap.rs \
  --report libs/word-core/layout/features/wr-12-line-layouter/functions/02_function_wrap/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_wrap --json`)
date: 2026-08-24 12:13

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/LineWrap.rs` | 82/82 | 100.0% | 109/110 | 99.1% |
| **function total** | **82/82** | **100.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java
```
本 function 产品 I/O 在 `LineWrap` 门面。LineLayouter 列表外方法停车。本 loop 布局校验 **PASS**（713 条 `// java`）。

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
按 **Java 类** 一行。layout（`// java`）必须 PASS。
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --report libs/word-core/layout/features/wr-12-line-layouter/functions/02_function_wrap/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 04:14

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `LineLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs` | **PASS** | — | `LineLayouter`, `initLineHeightLayouter`, `doInitTextWrapLayouter`, `initTextWrapLayouter`, `onAddWrapDrawing`, `clearTextWrapLayouter`, `getTextWrapLayouter`, `dispose`, `needReflowLine`, `flow`, `processCardInfo`, `isSpaceAfterSoftBreak`, `hasParaBreakIntention`, `isSecBreakNoContinue`, `compress`, `doGetCompressFactor`, `isOverflowBeforeFlow`, `getHAlignHandler`, `getVAlignHandler`, `getDocFieldCollect`, `getGridHandler`, `getCharSpaceHandler`, `getAutoSpaceHandler`, `getPunctSpaceHandler`, `getKerningHandler`, `getCharBorderHandler`, `isOverflow`, `prepareSubLineEnvs`, `preCalcLineHeight`, `postCalcLineHeight`, `calcParaBefore`, `calcParaAfter`, `regulateToTextLine`, `regulateToCache`, `getParaBrcLayouter`, `compressEmptyLine`, `initCollectors`, `regulateSubLinesAndRuns`, `regulateSubLine`, `adjustEmbedRun`, `setLineHeight`, `setAlignLine`, `setAscDesPos`, `setForceVCenterAlign` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
