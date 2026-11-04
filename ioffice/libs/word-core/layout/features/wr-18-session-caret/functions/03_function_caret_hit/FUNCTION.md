# 03_function_caret_hit — LayoutFrame 点到 caret cp

| 项 | 值 |
|----|----|
| feature | wr-18-session-caret |
| order | 3 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SessionCaret::cp_for_point(frame, page, x, y) -> Option<i32>` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
在产品 `LayoutFrame` 上把页坐标打到 **插入 cp**。不重做 wr-17 行表算法；用 `DrawText.xpos`（半开区间与 `CpHit::cp_for_x` 相同：过半进下一 cp）。

### 1.2 输入
内存构造的 `LayoutFrame`（或 04 的输出形状）。`(x,y)` 已是排版 twip。

### 1.3 输出
`Some(cp)` 或 `None`（无该页 / y 不在 run 带 / 无 Text 指令）。

### 1.4 上游
wr-17 `cp_for_x` 规则；04 Frame。Java：`LocateTool`（本 function **不** java2rust 整文件）。

### 1.5 下游
样例光标；不写选区。

### 1.6 不变量与函数改写列表
1. 不打开 zip。
2. 不修改 session。
3. Split 实现返回 `None`；loop 已按 xpos 接 `cp_for_x`。
4. **改写列表：** `cp_for_point` 仅此。`LocateTool.java` 不在本轮移植。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `LayoutFrame` / `DrawText.xpos` | wr-07/15 | page/x/y/text/xpos | 是 | Stage2 挂 Frame | 禁止改 emit 语义冒充 caret |
| `CpHit::cp_for_x` | wr-17 | 半点规则 | 是 | — | 禁止复制第二套规则到 Java 名 |
| `LocateTool` | Java | 全量定位 | 否 | 整类 java2rust | 禁止手写 1600 行空壳 |
| `hit_test` | wr-17 | Stage2 rows | 是 | session 未产 rows | 本 function 不测 hit_test 回归 |

### 1.8 函数改写列表
- `SessionCaret::cp_for_point`

## 2. 核心代码

Rust：`libs/word-core/src/cn/wps/moffice/writer/api/SessionCaret.rs`  
Loop 预期：过滤 `page`；y 落在 baseline 附近（或行框，若 Frame 日后带 height）；x 在 `xpos` 上走 wr-17 半点。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| cp_for_point | SessionCaret.rs | Hello xpos 左缘 | Some(0) | 主路径 |
| cp_for_point | x 极大 | Some(5) | 夹 end |
| cp_for_point | y=0 | None | y miss |
| cp_for_point | page=1 | None | 缺页 |
| cp_for_point | 空 commands | None | 空 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| CA-LEFT | cp_for_point | 左缘 | 是 | CA1 |
| CA-END | cp_for_point | 行尾外 | 是 | CA2 |
| CA-Y | cp_for_point | y miss | 是 | CA3 |
| CA-PAGE | cp_for_point | 错页 | 是 | CA4 |
| CA-EMPTY | cp_for_point | 空 Frame | 是 | CA5 |

全部 ignore 至 loop 接线。**本轮已接线，CA1–CA5 非 ignore。**

### 3.3 可测试性
构造 5 个 xpos 的 Hello `DrawText`；不跑 SAX。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| CA1 | CA-LEFT | fixture Frame | (0,1800,1632) | Some(0) | 返回 None 未接线 |
| CA2 | CA-END | 同上 | x=10000 | Some(5) | 夹成 0 |
| CA3 | CA-Y | 同上 | y=0 | None | 仍命中 |
| CA4 | CA-PAGE | 同上 | page=1 | None | 跨页 |
| CA5 | CA-EMPTY | 清 commands | 左缘 | None | 假 cp |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/api/SessionCaret.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/api/SessionCaret.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_caret_hit

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_caret_hit \
  --src src/cn/wps/moffice/writer/api/SessionCaret.rs \
  --report libs/word-core/layout/features/wr-18-session-caret/functions/03_function_caret_hit/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_caret_hit --json`)
date: 2026-08-27 10:27

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/SessionCaret.rs` | 36/41 | 87.8% | 55/63 | 87.3% |
| **function total** | **36/41** | **87.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
`SessionCaret` 无 Java 模板。Loop 若改 LocateTool，必须先 **java2rust** 生成交错模板再改写：

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java
```
（与 wr-17 相同：行几何模板；不是把 LocateTool 从零手写进 SessionCaret。）本轮布局：**PASS**。

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs \
  --report libs/word-core/layout/features/wr-18-session-caret/functions/03_function_caret_hit/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-27 02:27

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoTextLine` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoTextLine.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoTextLine.rs` | **PASS** | `setPara`, `getPara`, `setTextRectTop`, `getTextRectTop`, `setTextRectHeight`, `setAscDesPos`, `getTextRectHeight`, `getDescentPos`, `getAscentPos`, `setFlags`, `getFlags`, `getGraphIndexStart`, `setGraphIndexStart`, `getGraphIndexEnd`, `setGraphIndexEnd`, `setYPos`, `getYPos`, `isLastLineOfPara`, `setFirstLineOfPara`, `setLastLineOfPara`, `setLastLineOfPage`, `isFirstLineOfPara`, `isLastLineOfPage`, `getStartCP`, `getEndCP`, `contains` | `checkClassType`, `getType`, `alloc`, `isPointer`, `getRelTextRectTop`, `getTextRectBottom`, `offsetTextRectTop`, `getGraphLength`, `getEndChar`, `hasSHDs`, `getSHDs`, `setSHDs`, `getBorders`, `setBorders`, `getHighlights`, `setHighlights`, `getDocFiedls`, `getDocFields`, `setDocFields`, `getLines`, `setLines`, `isFirstLineOfMathPara`, `setIsInMathPara`, `setIsFirstLineOfMathPara`, `setIsLastLineOfMathPara`, `setFixedHeight`, `isFixedHeight`, `setExtendLastSHDAndBrc`, `isExtendLastSHDAndBrc`, `isLastLineOfSection`, `setLastLineOfSection`, `setLastLineOfColumn`, `setRTL`, `hasComment`, `setHasComment`, `hasEmbedsDrawing`, `setHasEmbedsDrawing`, `hasNotClipEmbeds`, `setHasNotClipEmbeds`, `isTemplateContent`, `setIsTemplateContent`, `hasAutoNumber`, `setHasAutoNumber`, `setHasSubLine`, `hasSubLine`, `setHasBidi`, `hasBidi`, `isInMathPara`, `isLastLineOfMathPara`, `isLastLineOfColumn`, `getFirstValidCp`, `getCPRef`, `setSHDAndBRCs`, `getSHDAndBRCs`, `isRTL`, `setMemEnd`, `getMemEnd`, `rectForPageCore`, `newInstance`, `newObjecctFactory`, `commitOp`, `toString`, `getLineChars`, `isCardBegin`, `setCardBegin`, `isCardEnd`, `setCardEnd`, `setLineBefore`, `getLineBefore`, `setLineAfter`, `getLineAfter`, `setLineBalloonPageHeight`, `getLineBalloonPageHeight` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
