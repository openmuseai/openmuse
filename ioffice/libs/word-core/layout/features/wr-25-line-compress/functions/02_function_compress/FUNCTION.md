# 02_function_compress — LineCompressTool.compress

| 项 | 值 |
|----|----|
| feature | wr-25-line-compress |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `LineCompressTool::compress` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
overflow 行：先算 first excess graph，再 hang space/punct/embed 或 `doCompressNormal`；失败则回退 graphPos。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `compress` | graphsPos, subLineWidth, layoutState | LineCompressTool.java:388–463 |

### 1.3 输出
`true` 当最终 `graphIndex==lastGraphIdx`（整段 graphs 纳入）；否则 pos 指向 rollback 点。

### 1.4 上游
`01_function_compress_env` 的 `LineCompressEnv`；假 `LayoutState`/`Graphs` fixture，不跑 TextLayouter.flow。

### 1.5 下游
SubLineLayouterBase.doAddExcessGraphs。

### 1.6 不变量与函数改写列表
- **改写列表：** `LineCompressTool,dispose,getPrevGraphCompressedWidth,isPrevGraphsFirstExcessSpace,canDoFirstCompress,canDoNextCompressIfPrevAndCurIsPartOfWord,canDoNextCompress,canHangPunct,canHangSpace,canHangEmbed,doCompressNormal,getFirstExcessGraphIndexAndUncompressedLeftPos,compress`
- 列表外：无（本类即本 function）。
- 禁止发明第二套 hang 规则。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓（保留 Java 注释+签名） | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `LineCompressEnv` | 本 feature 01 | firstExcess* / consume / getCompressAfter / isFirstCompress | loop 01 | — | 禁止回退 01 语义 |
| `LineRuleGraphsPos` | line | init/setGraphsAndIndex/prev/next/getGraphIndex | 本轮生成模板 | 非 compress 路径方法停车 | — |
| `PunctTool` | punctSpace | isHeadPunct/isTailPunct/isHangablePunct | 本轮生成并实现这 3 个 | 其余 DEFERRED | 禁止 stub 本类 |
| `LayoutState` / `Graphs` / `TypoPara` / `SubLineEnv` | env/graph | 本入口读的字段 | 已有或 CARGO_OFF_SURFACE 接入真实模板 | 不改无关方法 | 禁止工程 stub `cn.wps` |
| `LineLayouter` | wr-12 | 无 | j2r_ty 壳 | 不接入 | 禁止改 |

## 2. 核心代码
`LineCompressTool.java` → `LineCompressTool.rs`。禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `compress` early | :388–405 | firstExcess>last / shouldBeFollowed | true | 两 early return |
| hang space | :409–418 | runType space + canHangSpace | graphIdx=last；firstExcessSpace | wrapTrailSpaces |
| hang punct | :419–427 | runType=18 + canHangPunct | firstExcessPunct | overflowPunct false |
| hang embed | :428–433 | runType=6 | hang embed | viewMode≠0 |
| doCompressNormal | :265–350 | compressLine=false | false | 关压缩 |
| rollback pos | :438–460 | !hasCompressed | prev / atLeastOneChar | invalid pos |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| CP-follow | shouldBeFollowed | 边界 | 是 | CP1 |
| CP-hang-space | space + hang | 公开入口 | 是 | CP2 |
| CP-no-hang-space | wrapTrailSpaces | 关键分支 | 是 | CP3 |
| CP-hang-punct | runType 18 | 公开入口 | 是 | CP4 |
| CP-no-overflow-punct | overflowPunct=false | 失败 | 是 | CP5 |
| CP-normal-off | compressLine=false | 失败 | 是 | CP6 |
| CP-normal-on | consume 后 fit | 公开入口 | 是 | CP7 |
| CP-rollback | !hasCompressed + prev | 状态转换 | 是 | CP8 |
| CP-invalid | isInvalid after rollback | 失败 | 是 | CP9 |

### 3.3 可测试性
内存 Graphs 链 + 假 LayoutState；不跑 `layout_document`。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| CP1 | CP-follow | shouldBeFollowed | compress | true | 误 rollback |
| CP2 | CP-hang-space | wordWrap、!wrapTrail | space overflow | true + firstExcessSpace | 走 doCompress |
| CP3 | CP-no-hang-space | wrapTrailSpaces | space overflow | 不 hang | 假 hang |
| CP4 | CP-hang-punct | overflowPunct、hangable `。` | runType18 | firstExcessPunct | 不 hang |
| CP5 | CP-no-overflow-punct | overflowPunct=false | 同上 | 不 hang punct | 仍 hang |
| CP6 | CP-normal-off | compressLine=false 非 hangable | compress | false 或 rollback | 假 fit |
| CP7 | CP-normal-on | 可压空间>=overflow | doCompressNormal | true | 不 consume |
| CP8 | CP-rollback | 无可压且不可 hang | prev | graphIdx 回退 | 停在 excess |
| CP9 | CP-invalid | prev 失败且 !atLeastOneChar | isInvalid | false | panic |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineCompressTool.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/line/LineCompressTool.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_compress

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_compress \
  --src src/cn/wps/moffice/writer/layout/base/line/LineCompressTool.rs \
  --report libs/word-core/layout/features/wr-25-line-compress/functions/02_function_compress/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_compress --json`)
date: 2026-09-06 09:41

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/line/LineCompressTool.rs` | 202/411 | 49.1% | 506/966 | 52.4% |
| **function total** | **202/411** | **49.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineCompressTool.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineCompressTool.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineCompressTool.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineCompressTool.rs \
  --report libs/word-core/layout/features/wr-25-line-compress/functions/02_function_compress/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-06 01:41

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `LineCompressTool` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineCompressTool.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineCompressTool.rs` | **PASS** | `LineCompressTool`, `dispose`, `getPrevGraphCompressedWidth`, `isPrevGraphsFirstExcessSpace`, `canDoFirstCompress`, `canDoNextCompressIfPrevAndCurIsPartOfWord`, `canDoNextCompress`, `canHangPunct`, `canHangSpace`, `canHangEmbed`, `doCompressNormal`, `getFirstExcessGraphIndexAndUncompressedLeftPos`, `compress` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
