# 08_function_wrap_para_env — CJK 折行 + 段环境 + 页溢出

| 项 | 值 |
|----|----|
| feature | wr-29-java-pipeline-cutover |
| order | 8 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SubLineLayouter.addRun` 溢出 → `doAddExcessGraphs` → `LayoutState.breakGraphs`；`ParaLayouter.start` 每段 `calcIndents`/`calcLineSpace`；宿主 `java_pipeline_pages` 页溢出续页 |
| status | **pass** |
| 阶段 | **layout**（同一 sha256 sample2 → Java 行结构；render/Flutter 对照 WPS `sample2_01.png`） |

## 1. 功能说明

### 1.1 一句话
对 `sample2.docx` 走产品 `layout_document`：正文 CJK 按行宽折行、首行缩进 PAPX 208/209、`w:line=360` 1.5 倍行距、空段占位、页高溢出后新 `TypoPage`；结构对齐 WPS 全页 PNG（只允许行内像素差）。页眉「毕业论文」不在 OPC，归 STAGE6。

### 1.2 输入
`samples/docx-mvp-render/flutter/assets/samples/sample2.docx`  
sha256 `eb4e1466d75ea7856b62f332da9864ada599f440da9a38ef0e55feec754bafd3`  
WPS 金图 `sample2_01.png`…`sample2_08.png`。回归 `hello-world.docx`。

### 1.3 输出
- 第一段正文 ≥3 条 DrawText（WPS p1 约 6 行），行宽不超过内容区
- 首行 x ≥ 左边距+400（PAPX 208=480 → 实测 2280）
- 概述与第一段、两段正文之间 y 间隙 > 单倍行高
- `page_count≥2`（实测 6；WPS 8，差 STAGE6 页眉占位 + 残余行高）
- hello 仍单行；标题仍居中 18pt

### 1.4 上游
`07_function_e2e_render`（产品 Java 栈已切线）。

### 1.5 下游
`samples/docx-mvp-render` Flutter 主页截图 vs WPS `sample2_01.png`。

### 1.6 不变量与函数改写列表

**SubLineLayouter.rs**
- `addRun`：溢出走 `doAddExcessGraphs`（非数学行）

**SubLineLayouterBase.rs**
- `doAddExcessGraphs`：`getFirstExcessGraphIndexAndUncompressedLeftPos` → `rollbackAndAlignGraphs`
- `rollbackAndAlignGraphs`：`LayoutState.breakGraphs` 分裂超宽 Graphs

**LayoutState.rs**
- `breakGraphs` / `obtainAndInsertAfter` / `push_ptr`

**ParaLayouter.rs**
- `onStart` → `calcParaProp` → `calcIndents` / `calFirstLineIndent`；`calcLineSpace` PAPX 220
- `start` / `end` / `hasStart`（段界）

**TypoPara.rs**
- `getSum` / `getAbsSum`（breakGraphs 切宽）

**LayoutDocument.rs（无 Java basename，08-D1）**
- 段 `end` 必须用**更新 PAPX 之前**的 `old_end`
- `finishPage` 后收页、新 `TypoPage`、清 `linePos`/`curLayoutHeight`、绑回 `SuperEnvoy.setLayoutState`

不解：hdrftr（STAGE6）、drawing wrap、数学回退、`LineCompressTool.compress` 影子 `LayoutState`（08-D3 用 first-excess 同式量宽）。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------|--------|------|----------------------|
| GraphFactory 量宽 | 04 | 每字 widths 平面 | 是 | — | 不重量宽公式 |
| LineLayouter.flow | 05 | wrap 循环 / consumeGraphsBefore | 是 | — | 不重行高叶 |
| ParaLayouter.start | wr-21 / 本 function | indent + lineSpacing | 是 | onEnd BRC/SHD | 不解边框 |
| LineCompressTool.compress | wr-25 | 整行压缩 | **否** 08-D3 | 文件内影子 LayoutState | 不改 compress 全文 |
| PageViewLayouter.flowToPage | wr-23 | 多页 | 宿主循环对等 | 空 snapshot 页表 | 不解分栏/页眉 |
| hdrftr | STAGE6 | 「毕业论文」 | 否 | OPC 无 header part | 不发明页眉 |

#### 1.7.1 编号条目

- **08-D1**：`LayoutDocument.rs` 无 Java 同名文件。禁止 D1 错配。owner：本 function。
- **08-D2**：CHPX 字号未并入 PAPX 时 `getFirstCharSize=0`；`calFirstLineIndent` 回退 PAPX 208。owner：本 function。
- **08-D3**：`LineCompressTool.compress` 仍用文件内影子 `LayoutState`，不能传入真实 env。溢出切点走 Java `getFirstExcessGraphIndexAndUncompressedLeftPos`。owner：compress 叶。
- **08-D4**：WPS 页眉「毕业论文」不在 `sample2.docx` OPC（无 header/footer part）。结构对比不含页眉。owner：STAGE6。
- **08-D5**：产品 `page_count` 实测 6，WPS 金图 8；差在页眉占位与残余空段高度，不是未折行。owner：后续行高/页眉。

## 2. 核心代码

`SubLineLayouter.addRun` 溢出 → `SubLineLayouterBase.doAddExcessGraphs` → `LayoutState.breakGraphs`。  
`ParaLayouter.calFirstLineIndent` / `calcLineSpace`。  
宿主 `java_pipeline_pages` 段界 + 页溢出。禁止删 `// java`。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入 | 输出 | 关键分支 |
|--------|----------------|------|------|----------|
| addRun overflow | SubLineLayouter.java:480 | CJK Graphs 超 mMaxWidth | doAddExcessGraphs | 数学行 DEFERRED |
| breakGraphs | LayoutState.java | graphIndex 切点 | 两段 Graphs | RTL 非 sample2 |
| calcIndents | ParaLayouter | PAPX 208/209 | firstLineIndent | 08-D2 |
| calcLineSpace | PAPX 220 | LineSpacing(0,1.5) | 行距 360 | 缺 220 默认 1.0 |
| flowToPage | 宿主 | finishPage && cp&lt;doc_len | 新 TypoPage | 丢剩余 CP |
| hello | 回归 | Hello world | 1 DrawText | 误折 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|-------------|---------------|----------|-------|---------|
| W0 | dump DrawText | 管线 layout | 是 | W0 |
| W1 | 第一段折行 | 管线 layout | 是 | W1 |
| W2 | 首行缩进 | 正常 | 是 | W2 |
| W3 | 空段+1.5 行距 | 正常 | 是 | W3 |
| W4 | 页溢出 page_count | 管线 layout | 是 | W4 |
| W5 | hello 单行 | 回归 | 是 | W5 |
| W6 | 标题居中 | 回归 | 是 | W6 |
| W7 | WPS p1 块顺序 | 管线 | 是 | W7 |
| W8 | 产品无 layout_lines | 产品接线 | 是 | W8 |

### 3.3 单元测试 / 管线测试

管线：同一 sample2.docx → 真实 parse → Java layout → Frame。对照 WPS 页结构（折行/缩进/段距/页数），非整页 SSIM。FontHost 全局表：`--test-threads=1`。

### 3.4 用例表

| id | 期望 | 失败语义 |
|----|------|----------|
| W0 | dump 非空 | 空窗 |
| W1 | 近年来…然而 之间 ≥3 行且行宽&lt;内容区+200 | 未折行溢出 |
| W2 | 近年来 x≥左边距+400 | 未灌 PAPX 208 |
| W3 | 概述→近年来 dy&gt;240；近年来→然而 dy&gt;400 | 空段/1.5 未接线 |
| W4 | page_count≥2 且 page≥1 有正文 | finishPage 丢剩余 |
| W5 | hello 1 条 Hello | 误折 |
| W6 | 标题 大学生 360pt x&gt;1800 | 居中回归坏 |
| W7 | 标题&lt;概述&lt;近年来&lt;然而&lt;农村中小学&lt;调查方式 | 段序乱 |
| W8 | layout_document 无 layout_lines | 切线回退 |

### 3.5 测试代码位置

crate: `libs/word-core/Cargo.toml`  
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/SubLineLayouter.rs`  
       `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.rs`  
       `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/LayoutState.rs`  
       `libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs`  
       `libs/word-core/src/cn/wps/moffice/writer/api/LayoutDocument.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/line/SubLineLayouter.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 08_function_wrap_para_env -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 08_function_wrap_para_env \
  --src src/cn/wps/moffice/writer/layout/base/line/SubLineLayouter.rs \
  --src src/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.rs \
  --src src/cn/wps/moffice/writer/layout/base/env/LayoutState.rs \
  --src src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --src src/cn/wps/moffice/writer/api/LayoutDocument.rs \
  --report libs/word-core/layout/features/wr-29-java-pipeline-cutover/functions/08_function_wrap_para_env/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 08_function_wrap_para_env --json`)
date: 2026-10-03 23:43

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/LayoutDocument.rs` | 485/938 | 51.7% | 801/1485 | 53.9% |
| `src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs` | 243/457 | 53.2% | 389/828 | 47.0% |
| `src/cn/wps/moffice/writer/layout/base/env/LayoutState.rs` | 302/574 | 52.6% | 447/816 | 54.8% |
| `src/cn/wps/moffice/writer/layout/base/line/SubLineLayouter.rs` | 193/336 | 57.4% | 272/473 | 57.5% |
| `src/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.rs` | 47/227 | 20.7% | 105/425 | 24.7% |
| **function total** | **1270/2532** | **50.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）

`LayoutDocument.rs` 无 Java 孪生（08-D1）。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/SubLineLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/SubLineLayouter.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/LayoutState.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/env/LayoutState.java
```

### 3.9 Java↔Rust 门面对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/SubLineLayouter.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/SubLineLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/env/LayoutState.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/LayoutState.rs \
  --report libs/word-core/layout/features/wr-29-java-pipeline-cutover/functions/08_function_wrap_para_env/FUNCTION.md
```

<!-- java-rust-align:begin -->
（L5c 由 java-rust-align-report.mjs 写入）
<!-- java-rust-align:end -->

### 3.10 Java↔Rust 管线对比（逻辑 + 产品接线）

| 步 | Java | Rust |
|----|------|------|
| 1 | `SubLineLayouter.addRun` 溢出 `getRightWithBdr()-mMaxWidth>=2` | 同名 |
| 2 | `doAddExcessGraphs` → compress / firstExcess | firstExcess 同式（08-D3） |
| 3 | `LayoutState.breakGraphs` | 同名指针版 |
| 4 | `ParaLayouter.start` 每段 `calcIndents`/`calcLineSpace` | 同；宿主 old_end 后再灌 PAPX |
| 5 | `PageViewLayouter.flowToPage` 页满续页 | 宿主 finishPage → 新 TypoPage |
| 6 | 产品 `layout_document` | 同 06 Java 栈 |

### 3.11 管线阶段对齐报告

| 项 | 内容 |
|----|------|
| 阶段（parse / layout / render） | layout（产品 Frame）；Flutter 截图为 render 验收 |
| 原始文档（同一份，sha256） | `samples/docx-mvp-render/flutter/assets/samples/sample2.docx` `eb4e1466d75ea7856b62f332da9864ada599f440da9a38ef0e55feec754bafd3` |
| 步骤对照 | 见 §3.10 |
| 产品接线 | FFI / Flutter `layoutDocument` → `layout_document` |
| 关键路径真实实现 | pass（breakGraphs / addRun 溢出 / ParaLayouter.start 每段 / 页溢出续页） |
| 管线测试 test_id | W0–W8 |
| 测试结果符合预期 | pass（`cargo test --test 08_function_wrap_para_env -- --test-threads=1` 9/9） |
| 判定 | pass |
