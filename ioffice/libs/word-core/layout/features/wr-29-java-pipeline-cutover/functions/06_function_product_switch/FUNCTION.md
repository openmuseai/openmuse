# 06_function_product_switch — layout_document 切 Java 栈

| 项 | 值 |
|----|----|
| feature | wr-29-java-pipeline-cutover |
| order | 6 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `layout_document` / `layout_text_document` → `java_layout_text_document` → `java_pipeline_pages` |
| status | **pass** |
| 阶段 | **layout**（产品切线：默认 Frame 来自 Java 行；Stage2 `layout_lines` 仅显式回退） |

## 1. 功能说明

### 1.1 一句话
产品 `layout_document(docx)` 默认走 Java 管线宿主：`GraphBuffFactory.createGraphBuff` → `GraphReader.prepareGraphs` → `GraphFactory.createNodes` → `TextLayouter.doFlowTextLine` → `LineLayouter.flow` 写入 `TypoTextLine` → `PaginatedPage` → `RenderCommand`。删除 `attach_pageview_hot_path`；默认路径不调用 `env::layout_lines`。

### 1.2 输入
同一份 `hello-world.docx`（sha256 `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e`）→ 真实 `TextDocument` + `ViewParams` + `FontBlob`。

### 1.3 输出
- `layout_document` / `layout_text_document` 与 `java_layout_text_document` 同正文 DrawText
- hello ≥1 页、≥1 `TypoTextLine`、≥1 `DrawText`（含 `Hello`，page-absolute x=1800、size=240）
- `GraphBuffFactory.createGraphBuff` → `GraphFactory`
- `attach_pageview_hot_path` 已删除

### 1.4 上游
`05_function_flow_text_line`（`doFlowTextLine` / `LineLayouter.flow` / 镜像行）。

### 1.5 下游
`07_function_e2e_render`（wr-24 L2、FFI、Flutter 真实 docx UI）。

### 1.6 不变量与函数改写列表

**LayoutDocument.rs（本 function 改写列表；无 Java basename 孪生，PS-D1）**

- `java_pipeline_pages`：docx 主故事 → GraphBuffFactory → prepareGraphs → createNodes → doFlowTextLine 循环 → paginate
- `java_layout_text_document`：Java 行 cp 切片度量 runs → DrawText（契约同 V1）
- `layout_text_document` → `java_layout_text_document`
- `layout_document` → parse + `layout_text_document`（不得调 `layout_lines`）
- 删除 `attach_pageview_hot_path`
- 保留 `layout_document_stage2` / `layout_layout_runs_with_styles` 为显式 Stage2 回退

**GraphBuffFactory.rs（本 function 改写列表）**

- `createGraphBuff` → `new GraphFactory`（Java 一行）

**不在列表：** `TextLayouter.flow` / `PageViewLayouter.flowToPage` / `FlowLayout.createDocLayouter` 全链路（PS-D2：Rust 无 GraphFactory IS-A 虚分派，宿主显式 `createNodes`；`TextLayouter.mGraphBuff` 类型为 `GraphBuff` 非 `GraphFactory`）。`flowTable` / `flowTextFrame` 不解。

### 1.7 依赖与实现状态（必写）

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| GraphReader.prepareGraphs | 03 | 填 charsList | 是 | GR-D2 无虚分派 createNodes | 不重写 1559 行 |
| GraphFactory.createNodes | 04 | 量宽 | 是 | — | 不重量宽公式 |
| TextLayouter.doFlowTextLine | 05 | LineLayouter.flow → 行 | 是 | `flow()` 外层 initGraphBuff 会清宿主图 | 不解 flowTable |
| LineLayouter.flow | 05 | TypoTextLine | 是 | — | 不重折行叶 |
| GraphBuffFactory | 本 function | `createGraphBuff` | 是 | — | — |
| TypoPara.getCP / extendWidths | cache | 行 CP / 换行半字宽 | 本 function 接线 | snapshot.alloc 仍 GB-D3 | 不改 TypoSnapshot 叶 |
| PageViewLayouter.flowToPage | wr-23 | Java 产品外层 | 模板在 | 空 snapshot 页表；产品走宿主 | 不解 hdrftr/columns |
| env::layout_lines | wr-12 手写 | Stage2 回退 | 仅 stage2 | 默认禁用 | 不删回退 |

#### 1.7.1 编号条目

- **PS-D1**：`LayoutDocument.rs` 无 Java 同名文件（非 `LayoutManager.java`）。禁止 D1 配对错名。产品宿主是 Rust 编排，逐步对照 Java `FlowLayout`/`PageViewLayouter` 步骤，不断开 `// java` 叶子。owner：本 function。
- **PS-D2（GR-D2）**：Java `GraphReader.createNodes` 虚分派到 `GraphFactory`。Rust `GraphReader.createNodes` 与 `GraphFactory.createNodes` 是两套方法。宿主在 `prepareGraphs` 后显式遍历 `charsList` 调 `GraphFactory.createNodes`。owner：本 function。
- **PS-D3**：`TypoPara.getCP` Java 为 `return getStartCP(...)`。Rust 用与 `getNextCP` 相同的 `cp_map` mirror（05）。`// java return getStartCP(...)` 保留。owner：cache。
- **PS-D4**：`TextLayouter.init()` 内 `mGraphBuff = GraphBuffFactory.createGraphBuff` 仍 `DEFERRED_NON_FUNCTION`（类型 `Option<GraphBuff>` 不能持有 `GraphFactory`）。产品不经 `init()`。owner：后续 IS-A。
- **PS-D5**：`snapshot_from_pages` 信封（wr-14）非本 function 切线条件；Frame 直接从 Java `TypoTextLine` cp 切片。07 L2 可再接 `rows_from_snapshot`。

## 2. 核心代码

`api/LayoutDocument.rs` 产品入口；`graph/GraphBuffFactory.rs` 工厂。禁止 dummy。禁止删 `// java`。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|--------|----------------|-----------|-------------|---------------|
| java_pipeline_pages | LayoutDocument.rs | hello.docx | 1 页 ≥1 行 startCP=0 | 空 charsList |
| java_layout_text_document | 同上 | hello | DrawText Hello x=1800 size=240 | 空页 empty_frame |
| layout_document | 产品入口 | 同 sha256 | 正文 == java 栈 | 误走 layout_lines |
| GraphBuffFactory | GraphBuffFactory.java:8 | MainState/doc/view | GraphFactory runType=1 | 返回 GraphBuff 基类 |
| G-cut | 源码 | layout_document 体 | 无 layout_lines；无 attach_pageview | 占位残留 |
| stage2 | layout_document_stage2 | hello | 仍出行 | 回退被删 |
| G-probe | java_pipeline_pages | doFlowTextLine | LineLayouter.flow 写入 endCP>0 | 只 prepare 不 flow |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|-------------|---------------|----------|-------|----------------------|
| J1 | java_pipeline_pages hello 页/行 | 管线 layout | 是 | J1 |
| J2 | java Frame DrawText 契约 | 正常主路径 | 是 | J2 |
| J3 | layout_document == java 栈 | 产品接线 | 是 | J3 |
| J4 | GraphBuffFactory.createGraphBuff | 正常 | 是 | J4 |
| J5 | G-cut 无 layout_lines / 无 attach | 产品接线 | 是 | J5 |
| J6 | stage2 显式回退 | 边界 | 是 | J6 |
| J7 | G-probe Java 行 → 产品 DrawText | 状态转换 | 是 | J7 |
| J8 | G-P0-pages Java 栈 `'\f'` → ≥2 页 | 管线 layout | 是 | J8 |

### 3.3 单元测试 / 管线测试

单元：`GraphBuffFactory.createGraphBuff`；源码断言 G-cut。  
管线：同一 `hello-world.docx` → 真实 parse → Java 宿主 → Frame。停在 **layout 产品 Frame**，render/FFI 归 07。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|----|--------|----------|----------|-----------------|----------|
| J1 | J1 | hello.docx | java_pipeline_pages | 1 页；行 startCP=0 endCP>0 | 空页 |
| J2 | J2 | 同上 | java_layout_text_document | pipeline=v1；Hello；x=1800；size=240 | 手写栈坐标 |
| J3 | J3 | 同上 | layout_document / layout_text_document | 正文向量 == java | 仍 Stage2 |
| J4 | J4 | 默认 env | createGraphBuff | test_op_runType==1 | 返回空 GraphBuff |
| J5 | J5 | 源码 | layout_document 体 | 无 layout_lines；无 attach_pageview；宿主含 GraphBuffFactory+doFlowTextLine | 占位残留 |
| J6 | J6 | hello | layout_document_stage2 | 非空行 start_cp=0 | 回退坏 |
| J7 | J7 | hello | pages+layout_document | page_count 一致；DrawText Hello；Java endCP>0 | 丢行 |
| J8 | J8 | multipage.docx | layout_document | page_count≥2 | `force_page_after` 未接 `'\f'` |

### 3.5 测试代码位置

crate: `libs/word-core/Cargo.toml`  
src:   `libs/word-core/src/cn/wps/moffice/writer/api/LayoutDocument.rs`  
       `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphBuffFactory.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/api/java_pipeline_pages.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 06_function_product_switch

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 06_function_product_switch \
  --src src/cn/wps/moffice/writer/api/LayoutDocument.rs \
  --src src/cn/wps/moffice/writer/layout/base/graph/GraphBuffFactory.rs \
  --report libs/word-core/layout/features/wr-29-java-pipeline-cutover/functions/06_function_product_switch/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 06_function_product_switch --json`)
date: 2026-10-03 19:36

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/LayoutDocument.rs` | 583/835 | 69.8% | 973/1290 | 75.4% |
| `src/cn/wps/moffice/writer/layout/base/graph/GraphBuffFactory.rs` | 3/3 | 100.0% | 6/6 | 100.0% |
| **function total** | **586/838** | **69.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）

`LayoutDocument.rs` 无 Java 孪生（PS-D1），不配对错名。本 function Java 叶：

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphBuffFactory.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/GraphBuffFactory.java
```

### 3.9 Java↔Rust 门面对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/GraphBuffFactory.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphBuffFactory.rs \
  --report libs/word-core/layout/features/wr-29-java-pipeline-cutover/functions/06_function_product_switch/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-10-03 11:35

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `GraphBuffFactory` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/GraphBuffFactory.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphBuffFactory.rs` | **PASS** | `createGraphBuff` | — | — | **PASS** |

**overall: PASS** — basename aligned; every Java class has layout PASS and a Rust counterpart for each public method.
<!-- java-rust-align:end -->

### 3.10 Java↔Rust 管线对比（逻辑 + 产品接线）

| 步 | Java | Rust |
|----|------|------|
| 1 | `GraphBuffFactory.createGraphBuff` → `new GraphFactory` | 同名；返回 `GraphFactory` |
| 2 | `GraphFactory.prepareGraphs` → `doPrepareGraphs` | 同名（03） |
| 3 | `GraphReader.createNodes` 虚分派 `GraphFactory` | 宿主显式 `gf.createNodes`（PS-D2） |
| 4 | `TextLayouter.flow` → `flowLine` → `flowTextLine` → `doFlowTextLine` | 宿主 `doFlowTextLine`（内 `LineLayouter.flow`）；`flow()` 因 PS-D4 不经产品 |
| 5 | `LineLayouter.flow` → `TypoTextLine` | 同名（05） |
| 6 | snapshot 行 → 绘制 | Java 行 cp 切片 → `RenderCommand`（格式 V1） |
| 7 | 产品入口 | `layout_document` → `layout_text_document` → `java_layout_text_document` |

产品：默认不调 `env::layout_lines`。Stage2 仅 `layout_document_stage2`。

### 3.11 管线阶段对齐报告

| 项 | 内容 |
|----|------|
| 阶段（parse / layout / render） | layout（产品 Frame） |
| 原始文档（同一份，sha256） | `tests/fixtures/p1_layout_document/hello-world.docx` `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` |
| 步骤对照（Java 类.方法 ↔ Rust 类.方法 / 方案名） | 见 §3.10 |
| 产品接线（谁调用本阶段门面） | FFI / `layout_document` → `java_layout_text_document` |
| 关键路径真实实现（无 stub / 无另起实现） | **pass**（宿主真实 GraphReader/GraphFactory/doFlowTextLine/LineLayouter.flow；`layout_lines` 不在默认路径；`attach_pageview_hot_path` 已删） |
| 管线测试 test_id | J1 J2 J3 J5 J7 J8 |
| 测试结果符合预期 | **pass**（8/8 `cargo test --test 06_function_product_switch`） |
| 判定 | **pass** |
