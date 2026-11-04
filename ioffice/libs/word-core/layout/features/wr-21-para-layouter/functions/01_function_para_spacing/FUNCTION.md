# 01_function_para_spacing — 段前/段后

| 项 | 值 |
|----|----|
| feature | wr-21-para-layouter |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `ParaLayouter::calcParaBefore` / `calcParaAfter` / `calcPrevParaAfter` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
把 PAPX 217/218/219（段前）与 214/215/216（段后）写成 `ParaPropEnv.paraBefore/paraAfter`，规则与 Java 一致；邻段塌缩在行框侧用 `max(0, before − prevAfter)`，不在本入口发明 `max(before, after)`。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `calcParaBefore` | 当前段 `PropertySet` + `prevParaPropEnv` + `rangeStart` + viewMode | `ParaLayouter.java` 295–337 |
| `calcParaAfter` | 当前段 + `nextParaPropEnv` | 同文件 345–375 |
| `calcPrevParaAfter` | 上一段（跨段边界时重算） | 同文件 377–411 |

### 1.3 输出
`curParaPropEnv.paraBefore` / `paraAfter`（twip）。Auto spacing（219/216 true）字面量 **280**。`beforeLines` 218 / `afterLines` 215：有网格则 `lines * gridHeight / 100`，否则 `lines * 312 / 100`。

产品现状：`LayoutDocument::para_papx_from` 只 `getInt(217)` / `getInt(214)`，忽略 218/219/215/216，且 `LineBox::para_before_applied` 已按 Java `Math.max(0, cur.paraBefore - pre.paraAfter)`。

### 1.4 上游
wc-07 必须把 214/217（及 215/218/216/219 若 XML 有）写入 PAPX。本 function 只读桶。

### 1.5 下游
`04_function_para_start` 填 `ParaEnv`；`LineBox.measure` 用 before/after。

### 1.6 不变量与函数改写列表
- 改写列表：`calcParaBefore`、`calcParaAfter`、`calcPrevParaAfter`、`isAutoAfterSpacing`、`hasSameAutoNum`。
- 本 function 不改：`calAlign`、`calcIndents`、`calcLineSpace`、`calcAutoNum`、DropCap 排版（256 早退可测但 DropCap 对象 deferred）。
- 禁止 `max(before, after)` 当塌缩。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `PropertySet.getObject/getBoolean` | wave0 | 214–219 Integer/Boolean | 是 | — | — |
| PAPX 写入 | wc-03/wc-07 | 桶含 listed sprm | 部分 | wc-07 loop | 禁止在 layout 写 OOXML |
| `ParaEnv` / `ParaPropEnv` | wr-12 壳 | prev/cur/next 指针 | 部分 | 完整 LayoutState | 禁止改 wr-04 Wrap |
| `LineBox::para_before_applied` | env | `max(0, before-prevAfter)` | 是 | — | 本 function 不改公式，只供 04 接线 |

## 2. 核心代码
`koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java` `calcParaBefore` / `calcParaAfter`。Rust 目标：`libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs`（loop 生成完整交错模板）。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|--------|----------------|-----------|-------------|---------------|
| `calcParaBefore` | ParaLayouter.java:295 | 219 auto | 280 或 0 | rangeStart==paraStart 不加 280；同编号 auto-num 不加 |
| `calcParaBefore` | 同 | 218 beforeLines | lines*312/100 | 网格非 0 用 gridHeight |
| `calcParaBefore` | 同 | 217 Integer | =值 | 无 218 才读 217 |
| `calcParaBefore` | 同 | DropCap 256 + pageMode + linePos==1 | early return 保持 0 | 本波 corpus 无 DropCap；测 skip |
| `calcParaAfter` | :345 | 216 auto | 280 除非下段同 auto-num | next=null → 280 |
| `calcParaAfter` | 同 | 215/214 | 同 before 公式 | |
| `calcPrevParaAfter` | :377 | 上一段 214/216 | 写入 prev.paraAfter | rangeEnd==prev.end 则 auto 不加 |
| `para_before_applied` | LineBox.rs:56 | first_of_para | max(0,b-a) | 非段首行 0 |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|-------------|---------------|----------|-------|----------------------|
| PB-AUTO | 219 true 非段 range 起点 | 正常 | 是 | PB1 |
| PB-AUTO-START | 219 且 rangeStart==paraStart | 边界 | 是 | PB2 |
| PB-SPACE | 仅 217 | 正常 | 是 | PB3 |
| PB-LINES | 仅 218、无网格 | 分支 | 是 | PB4 |
| PA-AUTO | 216 true 无下一段 | 正常 | 是 | PA1 |
| PA-SPACE | 仅 214 | 正常 | 是 | PA2 |
| COLLAPSE | LineBox first line | 交互 | 是 | CL1 |
| DROP | 256 DropCap 早退 | 分支 | 否 | 后波；方案保留 ignore |

### 3.3 可测试性
内存 `PropertySet` + 假 `IStateEnvoy`（prev/cur/next）；不 open docx、不跑 `layout_document`。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|----|--------|----------|----------|-----------------|----------|
| PB1 | PB-AUTO | 新 envoy | 219=true，range 中段 | paraBefore=280 | 当 0 或当 217 |
| PB2 | PB-AUTO-START | 页/段起点 | 219=true，rangeStart=para.start | paraBefore=0 | 误加 280 |
| PB3 | PB-SPACE | 无 218/219 | 217=240 | paraBefore=240 | getInt 默认 0 当缺 key |
| PB4 | PB-LINES | grid=0 | 218=100 | paraBefore=312 | 当 100 或当 twip 直读 |
| PA1 | PA-AUTO | next=null | 216=true | paraAfter=280 | 0 |
| PA2 | PA-SPACE | 无 215/216 | 214=200 | paraAfter=200 | |
| CL1 | COLLAPSE | 段首行 | before=400, prevAfter=200 | 行附加高 200 | `max(400,200)` 或相加 600 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/ParaLayouter.rs`  
`[[test]]` 在 loop L2 挂 Cargo.toml。

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_para_spacing

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_para_spacing \
  --src src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --report libs/word-core/layout/features/wr-21-para-layouter/functions/01_function_para_spacing/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_para_spacing --json`)
date: 2026-09-03 19:13

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs` | 78/228 | 34.2% | 188/503 | 37.4% |
| **function total** | **78/228** | **34.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --report libs/word-core/layout/features/wr-21-para-layouter/functions/01_function_para_spacing/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-03 11:16

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ParaLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs` | **PASS** | `ParaLayouter`, `isAutoAfterSpacing`, `calcParaBefore`, `hasSameAutoNum`, `calcParaAfter`, `calcPrevParaAfter` | `dispose`, `onStart`, `calcFinalPropertySet`, `getFinalParaProp`, `calclastBrcBetween`, `calcParaProp`, `calcBRC`, `calcSHD`, `getVaildShd`, `calcAutoNum`, `onEnd`, `isLineWithParaBreakAndSectionBreak`, `shouldCalcLastParaAfter`, `shouldAdjustRightMarginInCharGridModel`, `calcLineSpace`, `calcIndents`, `calFirstLineIndent`, `getNormalParaFinalProperty`, `getCharGridExtraFirstLineIndent`, `calLeftAndRightIndent`, `getCharGridExtraRightIndent`, `getFirstCharSize`, `calcShouldCompressLine`, `calAlign`, `isFirstLineOfPara` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
