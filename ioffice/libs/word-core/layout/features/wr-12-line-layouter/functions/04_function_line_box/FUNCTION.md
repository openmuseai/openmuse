# 04_function_line_box — 行高 / 基线

| 项 | 值 |
|----|----|
| feature | wr-12-line-layouter |
| order | 4 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `LineBox::measure(metrics, spacing, para_before, para_after, first_of_para, last_of_para, prev_para_after) -> LineBox` |
| 独立性 | 不折行、不对齐；用 wr-11 行格 + PAPX 220/217/214。 |
| status | **pass** |

`LineBox`：`height`, `baseline`（相对行顶的 ascent 位置，对标 `TypoTextLine.setAscDesPos`）。

## 1. 功能说明

### 1.1 一句话
用 wr-11 的 ascent/descent 和 PAPX 行距规则算出行高与基线；段前/段后只加在段首/段末行。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `measure` | wr-11 `ascent`,`descent`（twip）；`LineSpacing`（PAPX **220**，rule+spacing）；`para_before`（**217**）、`para_after`（**214**）；是否首/末行；上一段 `paraAfter` | `LineHeightLayouter.calcLineHeightInfoByLineSpacingRule`；`LineLayouter.calcParaBefore` / `calcParaAfter`；`regulateToCache`：`setHeight(mLineHeight + mParaBefore + mParaAfter)` |

`LineSpacing` Java：`rule==0` auto，`1` exact，`2` atLeast；默认 `new LineSpacing(0, 1.0F)`。

### 1.3 输出
`height`（含首行段前、末行段后）、`baseline`。中间行不加 217/214。

### 1.4 上游
wr-11 `RunMetrics`。测 N 注入整数 ascent/descent，不探盘。

### 1.5 下游
05 写 `textRectHeight` / `ascentPos` / 行 `height`。

### 1.6 不变量
1. **auto (rule=0)**：`channel = ascent + descent`（Java `getChannelHeight`，**不含**单独 line_gap 字段）。`extra = baseHeight * (spacing - 1)`，`lineHeight = round(channel + extra)`（Java `+ 0.5F` 再转 int）。`spacing==1` 且无 grid 时即 round(channel)。
2. **exact (rule=1)**：`lineHeight = round(abs(spacing))`；不跟 metrics 拉高。
3. **atLeast (rule=2)**：`lineHeight = round(max(channel, spacing))`。
4. 本波 **关闭** snap-to-grid / `mNeedSnapToGrid`（列表外）。
5. 首行：`paraBefore' = max(0, para_before - prev_para_after)`（`LineLayouter.calcParaBefore`）；加到 height。
6. 末行：加 `para_after`（`calcParaAfter`）；卡片/页首行 Java 会跳过段后——本波不测卡片。
7. `baseline`：相对行顶 ≈ `paraBefore' + ascent`（与 `setAscDesPos` 同义；loop 对照 `LineHeightLayouter.mAscentPos`）。
8. **禁止**用 wr-04 `LineHeight.rs` 的 `ceil(ceil(pt×20)×lineSpacing)` 冒充本 function（那是字号行格，不是 Java `LineHeightLayouter`）。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| wr-11 ascent/descent | wr-11 | twip | 是 | line_gap 是否并入通道 | 禁止改 FontStack |
| PAPX 220/217/214 | wc-03 | LineSpacing + int | 导入已有 | auto 行距 lines 215/218 | 禁止读 XML |
| wr-04 `line_height` | wr-04 | 字号 ceil | 是（**不同公式**） | Hello MVP | **禁止调用** |
| snap-to-grid | Java | gridHeight | 否 | 网格 feature | 恒关 |
| 嵌入/弱图元通道 | Java `mVMetricsDatas[3..6]` | 否 | 纯正文只走文本 ascent/descent | 禁止图 |

### 1.8 函数改写列表
- `LineBox::measure` 及 spacing 三规则 + 首/末段距
- `LineHeightLayouter.calcLineHeightInfoByLineSpacingRule` 语义（门面）
- grid / 嵌入通道 / `updateLineHeightInfoByLastLine`：**不在列表**

## 2. 核心代码

Java auto（无 grid，正文通道）：

```text
channel = maxAscent + maxDescent
extra   = baseHeight * (lineSpacing - 1)
mLineHeight = (int)(channel + extra + 0.5F)
```

exact：`(int)(Math.abs(lineSpacing) + 0.5F)`。
atLeast：`(int)(Math.max(channel, lineSpacing) + 0.5F)`。

Rust：`layout/base/env/LineBox.rs`（loop 新建）。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| measure | auto×1 | asc=180 des=60 | h=240 bl=180 | 单倍 |
| measure | auto×1.5 | 同上 | extra=0.5*base | 多倍 |
| measure | exact 240 | metrics 任意 | h=240 | 固定 |
| measure | atLeast 低于通道 | spacing=100, ch=240 | 240 | max |
| measure | atLeast 高于通道 | spacing=400 | 400 | 拉高 |
| measure | 首行段前 | before=200, prevAfter=0 | h+=200 | 217 |
| measure | 段距塌缩 | before=200, prevAfter=80 | +=120 | calcParaBefore |
| measure | 末行段后 | after=120 | h+=120 | 214 |
| measure | 中间行 | first=last=false | 不加段距 | 只中间 |
| measure | 负 exact | spacing=-240 | 240 | abs |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| LB-AUTO1 | measure | auto 1.0 | 是 | LB1 |
| LB-AUTO15 | measure | auto 1.5 | 是 | LB2 |
| LB-EXACT | measure | rule=1 | 是 | LB3 |
| LB-ATLO | measure | atLeast < channel | 是 | LB4 |
| LB-ATHI | measure | atLeast > channel | 是 | LB5 |
| LB-BEF | measure | 首行段前 | 是 | LB6 |
| LB-COLLAPSE | measure | 与上段 after 塌缩 | 是 | LB7 |
| LB-AFT | measure | 末行段后 | 是 | LB8 |
| LB-MID | measure | 中间行 | 是 | LB9 |
| LB-ABS | measure | exact 负值 | 是 | LB10 |
| LB-BL | measure | baseline=ascent | 是 | LB11 |
| LB-NO04 | measure | 不调用 LineHeight.rs | 是 | LB12 |

### 3.3 可测试性
结构体字面量；不排版、不 FontHost。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| LB1 | LB-AUTO1 | — | asc=180,des=60,rule=0,spc=1 | height=240 | 用了 pt×20 ceil |
| LB2 | LB-AUTO15 | base=240 | spc=1.5 | extra=120 → h=360（若 channel==base） | 乘数错 |
| LB3 | LB-EXACT | metrics=999 | rule=1,spc=400 | 400 | 仍用通道 |
| LB4 | LB-ATLO | ch=240 | rule=2,spc=100 | 240 | 误用 exact |
| LB5 | LB-ATHI | ch=240 | rule=2,spc=400 | 400 | 未拉高 |
| LB6 | LB-BEF | first | before=200 | height=通道+200 | 段前丢 |
| LB7 | LB-COLLAPSE | first, prevAfter=80, before=200 | +120 | 未塌缩 |
| LB8 | LB-AFT | last | after=120 | +120 | 段后丢 |
| LB9 | LB-MID | 非首非末 | before=after=200 | 不加 | 中间行也加 |
| LB10 | LB-ABS | exact -400 | 400 | 未 abs |
| LB11 | LB-BL | LB1 | baseline=180 | 基线错 |
| LB12 | LB-NO04 | — | 不 `use …::line_height` | 公式被 Hello 版替换 |

LB2 的 `baseHeight` 取 Java `updateLineInfo` 正文分支：`(int)(maxAscent+maxDescent+0.5F)`。loop 用同一整数。

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/LineBox.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/LineBox.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_line_box

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 04_function_line_box \
  --src src/cn/wps/moffice/writer/layout/base/env/LineBox.rs \
  --report libs/word-core/layout/features/wr-12-line-layouter/functions/04_function_line_box/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_line_box --json`)
date: 2026-08-24 15:28

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/LineBox.rs` | 51/51 | 100.0% | 65/65 | 100.0% |
| **function total** | **51/51** | **100.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java
```
本 function 产品 I/O 在 `LineBox` 门面。未生成 `LineHeightLayouter.rs`。LineLayouter 列表外方法停车。本 loop 布局校验 **PASS**（713 条 `// java`）。

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
按 **Java 类** 一行。layout（`// java`）必须 PASS。
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --report libs/word-core/layout/features/wr-12-line-layouter/functions/04_function_line_box/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 07:21

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `LineLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs` | **PASS** | — | `LineLayouter`, `initLineHeightLayouter`, `doInitTextWrapLayouter`, `initTextWrapLayouter`, `onAddWrapDrawing`, `clearTextWrapLayouter`, `getTextWrapLayouter`, `dispose`, `needReflowLine`, `flow`, `processCardInfo`, `isSpaceAfterSoftBreak`, `hasParaBreakIntention`, `isSecBreakNoContinue`, `compress`, `doGetCompressFactor`, `isOverflowBeforeFlow`, `getHAlignHandler`, `getVAlignHandler`, `getDocFieldCollect`, `getGridHandler`, `getCharSpaceHandler`, `getAutoSpaceHandler`, `getPunctSpaceHandler`, `getKerningHandler`, `getCharBorderHandler`, `isOverflow`, `prepareSubLineEnvs`, `preCalcLineHeight`, `postCalcLineHeight`, `calcParaBefore`, `calcParaAfter`, `regulateToTextLine`, `regulateToCache`, `getParaBrcLayouter`, `compressEmptyLine`, `initCollectors`, `regulateSubLinesAndRuns`, `regulateSubLine`, `adjustEmbedRun`, `setLineHeight`, `setAlignLine`, `setAscDesPos`, `setForceVCenterAlign` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
