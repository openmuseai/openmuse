# 05_function_line_layouter — 编排 TypoTextLine[]

| 项 | 值 |
|----|----|
| feature | wr-12-line-layouter |
| order | 5 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `layout_lines(runs, content_width, papx) -> Vec<TypoTextLine>` |
| 独立性 | 编排 01–04；**禁止** `finishPage` / `isOverflow`（页高）/ wr-04 `wrap()`。 |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
feature 出口：一段已度量 run + 内容宽 + PAPX → 逻辑行 `TypoTextLine[]`（尚未分页）。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `layout_lines` | wr-11 度量后的 run 序列（每 cp advance + 行格）+ wr-02 `content_width` + PAPX（193/208/220/217/214） | `LineLayouter.flow` 中 **不含** `isOverflowBeforeFlow` / `isOverflow` / 绕排 wrap env 的正文路径；`TextLayouter.flowLine` 调 `mLineLayouter.flow` |

不发明第二套行类型：写已有 `TypoTextLine`（`start_cp`/`end_cp`/`setFirstLineOfPara`/`setLastLineOfPara`/`setTextRectHeight`/`setAscDesPos`）。

### 1.3 输出
`Vec<TypoTextLine>`：每行 `start_cp`, `end_cp`, `width`, `height`, `baseline`, `x`，flags FIRST/LAST of para。忽略 `page_index`（wr-13）。

### 1.4 上游
01–04；wr-11 宽度。测 N 用 fixture advances，不跑完整 `TextLayouter` 管线。

### 1.5 下游
wr-13 分页。本波不插入 `curPage.insertLine` 的页列表（可只返回 Vec）。

### 1.6 不变量
1. 第 i 行 `max_width` = 01(`content_width`, 208, i==0)。
2. cp 区间 = 02 输出；必须可与 Java 金文件逐行相等。
3. `x` = 03(jc, line_width, 该行内容宽)。
4. `height`/`baseline` = 04（首行带段前，末行带段后）。
5. 仅一行：同时 FIRST+LAST。多行：首 FIRST、末 LAST、中间皆否。
6. **不**调用 `isOverflow` / `finishPage` / `processPagination`。
7. justify：x 与左对齐相同。
8. 现网 `LineLayouter.rs` 的 `cache_run_widths` 是 wr-04 胶水；loop 生成 Java 模板后 **保留或外移**，禁止删掉导致 wr-04 测试红。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| ContentWidth / LineWrap / LineAlign / LineBox | 01–04 | 见各 FUNCTION | 是 | — | — |
| `TypoTextLine` | wr-01 cache | start/end cp、flags、rect、left/width | 本 function 写入 | 完整 snapshot arena | 禁止新行类型 |
| `LineLayouter.java` | Java 双子 | flow 编排 | 列表外停车；编排在 `env/LineLayout.rs` | 分页/绕排/装饰 collector | 禁止实现 `isOverflow` / `finishPage` |
| wr-04 Wrap/LineHeight | wr-04 | Hello | 是 | — | **禁止调用** |
| `isOverflow*` | Java | 页高 | 否 | wr-13 | 禁止实现 |

### 1.8 函数改写列表
- `layout_lines`（或 `LineLayouter` 上等价入口）
- 调用 01–04 填 `TypoTextLine`
- Java `flow` / `isOverflow*` / `prepareSubLineEnvs` / `regulateToCache` 分页与绕排：**不在列表**（`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION */`）
- `cache_run_widths`：保留 wr-04 行为，不在本 function 语义内

## 2. 核心代码

```text
lines = []
first = true
for each wrapped range in LineWrap(advances, max_width(first)):
  box = LineBox.measure(..., first, last)
  x = LineAlign.alignX(jc, range.width, max_width)
  emit TypoTextLine { start_cp, end_cp, width, height, baseline, x, flags }
  first = false
```

Java `flow` 在 `preCalcLineHeight` 之后若 `isOverflowBeforeFlow` 则整行放弃——那是 **页满**，wr-12 当「无限页高」：该分支恒 false。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| layout_lines | 短段 | 一行放得下 | 1 行 FIRST+LAST | 主路径 |
| layout_lines | 需折 | CJK/空格超宽 | N 行 cp 衔接 | 折行 |
| layout_lines | 首行缩进 | 208>0 | 首行更少 cp | 01 |
| layout_lines | jc 中 | 193=1 | x=(W-w)/2 | 03 |
| layout_lines | 段前 | 217>0 | 仅首行 height 更大 | 04 |
| layout_lines | `\r` | 段末 | LAST | 段标 |
| layout_lines | 标题字号 | 更大 advance | 更少字/行 | FEATURE in |
| layout_lines | justify | 193=3 | x=0 | stub |
| layout_lines | 空 | 空 text | 0/1 行 | 与 02 一致 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| LL-ONE | layout_lines | 单行 | 是 | LL1 |
| LL-MULTI | layout_lines | 多行 cp 衔接 | 是 | LL2 |
| LL-FIRST | layout_lines | 仅首 FIRST | 是 | LL3 |
| LL-IND | layout_lines | 208 影响切分 | 是 | LL4 |
| LL-JC | layout_lines | 左中右 x | 是 | LL5 |
| LL-SPC | layout_lines | 217/214 首末 | 是 | LL6 |
| LL-CR | layout_lines | `\r` | 是 | LL7 |
| LL-HEAD | layout_lines | 大字号 | 是 | LL8 |
| LL-JUST | layout_lines | jc=3 | 是 | LL9 |
| LL-NOPAGE | layout_lines | 不碰页高 | 是 | LL10 |
| LL-NO04 | layout_lines | 不用 Wrap.rs | 是 | LL11 |
| LL-TYPE | layout_lines | 类型是 TypoTextLine | 是 | LL12 |

### 3.3 可测试性
内存 `PropertySet` + 假 `advances[]`（或 wr-11 TestFont 已测过的宽）。不启动 `TextLayouter.flow` 全循环。不读页面 `maxLayoutHeight`。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| LL1 | LL-ONE | 宽足够 | "Hi" | 1 行 [0,2)，FIRST+LAST | 误折 |
| LL2 | LL-MULTI | CJK 超宽 | 两字 max=一字 | 行1[0,1) 行2[1,2)，end=next.start | cp 重叠/空洞 |
| LL3 | LL-FIRST | LL2 | flags | 仅行0 FIRST、仅末行 LAST | flag 错 |
| LL4 | LL-IND | 208 很大 | 首行 max 变小 | 首行 cp 更少 | 缩进未进 wrap |
| LL5 | LL-JC | jc=1, 行宽 < 内容宽 | x=(cw-lw)/2 | 未对齐 |
| LL6 | LL-SPC | 217/214 | 仅首行/末行 height 含段距 | 每行都加 |
| LL7 | LL-CR | "a\\r" | 1 行 LAST | CR 丢掉 |
| LL8 | LL-HEAD | 同文更大 advance | 行数增加 | 标题仍一行 |
| LL9 | LL-JUST | jc=3 | x 与 jc=0 相同 | 伸展 |
| LL10 | LL-NOPAGE | 极小虚构页高 | 仍返回全部逻辑行 | 误做 wr-13 |
| LL11 | LL-NO04 | — | 不调用 `text::Wrap::wrap` | 贪心切分 |
| LL12 | LL-TYPE | — | `getStartCP`/`getEndCP` | 第二套行 struct |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/LineLayout.rs`（编排入口；`LineLayouter.rs` 列表外方法停车，保留 wr-04 `cache_run_widths`）
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/LineLayout.rs`（LL1–LL12；wr-04 RW1/RW2 仍在 `tests/.../line/LineLayouter.rs` / `01_function_run_width`）

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 05_function_line_layouter

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 05_function_line_layouter \
  --src src/cn/wps/moffice/writer/layout/base/env/LineLayout.rs \
  --report libs/word-core/layout/features/wr-12-line-layouter/functions/05_function_line_layouter/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 05_function_line_layouter --json`)
date: 2026-08-24 17:57

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/LineLayout.rs` | 86/89 | 96.6% | 147/152 | 96.7% |
| **function total** | **86/89** | **96.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java
```
`flow` / `isOverflow*` 不在列表：停车。禁止为瘦编译面删 `// java` 注释。本 loop 布局校验 **PASS**（713 条 `// java`）。编排 I/O 在 `LineLayout` 门面。

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
按 **Java 类** 一行。layout（`// java`）必须 PASS。
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --report libs/word-core/layout/features/wr-12-line-layouter/functions/05_function_line_layouter/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 09:58

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `LineLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs` | **PASS** | — | `LineLayouter`, `initLineHeightLayouter`, `doInitTextWrapLayouter`, `initTextWrapLayouter`, `onAddWrapDrawing`, `clearTextWrapLayouter`, `getTextWrapLayouter`, `dispose`, `needReflowLine`, `flow`, `processCardInfo`, `isSpaceAfterSoftBreak`, `hasParaBreakIntention`, `isSecBreakNoContinue`, `compress`, `doGetCompressFactor`, `isOverflowBeforeFlow`, `getHAlignHandler`, `getVAlignHandler`, `getDocFieldCollect`, `getGridHandler`, `getCharSpaceHandler`, `getAutoSpaceHandler`, `getPunctSpaceHandler`, `getKerningHandler`, `getCharBorderHandler`, `isOverflow`, `prepareSubLineEnvs`, `preCalcLineHeight`, `postCalcLineHeight`, `calcParaBefore`, `calcParaAfter`, `regulateToTextLine`, `regulateToCache`, `getParaBrcLayouter`, `compressEmptyLine`, `initCollectors`, `regulateSubLinesAndRuns`, `regulateSubLine`, `adjustEmbedRun`, `setLineHeight`, `setAlignLine`, `setAscDesPos`, `setForceVCenterAlign` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
