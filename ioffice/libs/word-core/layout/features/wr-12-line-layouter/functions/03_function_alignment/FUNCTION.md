# 03_function_alignment — jc → 行起始 x

| 项 | 值 |
|----|----|
| feature | wr-12-line-layouter |
| order | 3 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `LineAlign::alignX(jc, line_width, content_width) -> i32` |
| 独立性 | 纯整数；不折行、不改 PAPX。 |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
把 PAPX **193**（`paragraph_alignmentEx`）换成行在内容区里的起始 x。两端对齐 stub 为左。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `alignX` | `jc` + 行宽 + 内容宽（本行 `layoutRight-layoutLeft`，与 01/02 同一坐标系） | `AlignLeftHandler` / `AlignCenterHandler` / `AlignRightHandler.adjustLine`；`LineLayouter.getHAlignHandler(paraEnv.alignment)` |

常量（与 wr-04 `Alignment.rs` / S10 一致）：`0` 左、`1` 中、`2` 右、`3` 两端。

### 1.3 输出
相对内容区左缘的 `x`（twip）。行宽 ≥ 内容宽 → `0`（Java：首 run 超宽则 `setLeft(layoutLeft)`）。

### 1.4 上游
02 的 `line.width`；内容宽用 01 的 `max_width`。

### 1.5 下游
05 写入行 `x` / `setLeft`。

### 1.6 不变量
1. 左 / 未知 jc：`x = 0`。
2. 中：`x = (content_width - line_width) / 2`（整数除法）。FEATURE / S10 锁死此式。
3. 右：`x = content_width - line_width`。
4. 两端（3）：**与左相同**（`AlignBothHandler` 列表外）。
5. `line_width >= content_width`：`x = 0`。
6. 不处理 bidi `adjustBidiLine`、节破线 `shouldAdjustSecBreakLine`、无 wordWrap 时 `alignRightGraphs` 裁行尾空格。

Java 笔记：`AlignCenterHandler.adjustLine` 在 `alignRightGraphs == null` 时写 `setLeft((layoutLeft+layoutRight)/2)`（未减行宽）。wordWrap 为真时该字段通常为 null。**本 function 以 FEATURE/S10 的 (内容宽−行宽)/2 为契约**；若 wr-16 金文件证明须跟 Java 那一分支，再改 §1.6 并重测。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| PAPX 193 | wc-03 | getInt 默认 0 | 是 | — | 禁止改导入器 |
| wr-04 `align_x` | wr-04 | 同公式 | 是 | — | 可委托；**不要**改成 justify 伸展 |
| `AlignBothHandler` | Java | 按字符伸展 | 否 | stub=左 | 禁止移植 mSpacePerChar |
| bidi | Java | adjustBidiLine | 否 | stub | 禁止 RTL |

### 1.8 函数改写列表
- `LineAlign::alignX`（及 jc 常量）
- 可选委托 `text/Alignment.rs::align_x`（若公式保持一致）
- AlignBoth / bidi / 节破：**不在列表**

## 2. 核心代码

```text
if line_width >= content_width { return 0 }
match jc {
  1 => (content_width - line_width) / 2
  2 => content_width - line_width
  3 | 0 | _ => 0
}
```

Rust：`layout/base/env/LineAlign.rs`（loop 新建门面）。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| alignX | 左 | jc=0, 100, 200 | 0 | 默认 |
| alignX | 中 | jc=1 | 50 | 整除 |
| alignX | 右 | jc=2 | 100 | 右 |
| alignX | 满 | line==content | 0 | 边界 |
| alignX | 超宽 | line>content | 0 | 夹紧 |
| alignX | 两端 | jc=3 | 0 | stub |
| alignX | 奇数差 | 101 vs 200 | 49 | 整除截断 |
| alignX | 未知 | jc=99 | 0 | 回落 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| AL-LEFT | alignX | jc=0 | 是 | AL1 |
| AL-CENTER | alignX | jc=1 | 是 | AL2 |
| AL-RIGHT | alignX | jc=2 | 是 | AL3 |
| AL-FULL | alignX | 等宽 | 是 | AL4 |
| AL-OVER | alignX | 行更宽 | 是 | AL5 |
| AL-JUST | alignX | jc=3 | 是 | AL6 |
| AL-ODD | alignX | 奇数差 | 是 | AL7 |
| AL-UNK | alignX | 非法 jc | 是 | AL8 |

### 3.3 可测试性
纯函数。不构造 Graphs / SubLineEnv。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| AL1 | AL-LEFT | — | 0, 100, 200 | 0 | 左偏移 |
| AL2 | AL-CENTER | — | 1, 100, 200 | 50 | 居中错 |
| AL3 | AL-RIGHT | — | 2, 100, 200 | 100 | 右错 |
| AL4 | AL-FULL | — | 1, 200, 200 | 0 | 满仍偏移 |
| AL5 | AL-OVER | — | 2, 250, 200 | 0 | 超宽负 x |
| AL6 | AL-JUST | — | 3, 100, 200 | 0 | 误做伸展 |
| AL7 | AL-ODD | — | 1, 101, 200 | 49 | 浮点除 |
| AL8 | AL-UNK | — | 99, 100, 200 | 0 | 未回落 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/LineAlign.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/LineAlign.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_alignment

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_alignment \
  --src src/cn/wps/moffice/writer/layout/base/env/LineAlign.rs \
  --report libs/word-core/layout/features/wr-12-line-layouter/functions/03_function_alignment/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_alignment --json`)
date: 2026-08-24 12:17

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/LineAlign.rs` | 3/3 | 100.0% | 6/6 | 100.0% |
| **function total** | **3/3** | **100.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java
```
本 function 产品 I/O 在 `LineAlign` 门面（委托 wr-04 `align_x`，justify 仍为左）。LineLayouter 列表外方法停车。本 loop 布局校验 **PASS**（713 条 `// java`）。

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
按 **Java 类** 一行。layout（`// java`）必须 PASS。
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --report libs/word-core/layout/features/wr-12-line-layouter/functions/03_function_alignment/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 04:17

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `LineLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs` | **PASS** | — | `LineLayouter`, `initLineHeightLayouter`, `doInitTextWrapLayouter`, `initTextWrapLayouter`, `onAddWrapDrawing`, `clearTextWrapLayouter`, `getTextWrapLayouter`, `dispose`, `needReflowLine`, `flow`, `processCardInfo`, `isSpaceAfterSoftBreak`, `hasParaBreakIntention`, `isSecBreakNoContinue`, `compress`, `doGetCompressFactor`, `isOverflowBeforeFlow`, `getHAlignHandler`, `getVAlignHandler`, `getDocFieldCollect`, `getGridHandler`, `getCharSpaceHandler`, `getAutoSpaceHandler`, `getPunctSpaceHandler`, `getKerningHandler`, `getCharBorderHandler`, `isOverflow`, `prepareSubLineEnvs`, `preCalcLineHeight`, `postCalcLineHeight`, `calcParaBefore`, `calcParaAfter`, `regulateToTextLine`, `regulateToCache`, `getParaBrcLayouter`, `compressEmptyLine`, `initCollectors`, `regulateSubLinesAndRuns`, `regulateSubLine`, `adjustEmbedRun`, `setLineHeight`, `setAlignLine`, `setAscDesPos`, `setForceVCenterAlign` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
