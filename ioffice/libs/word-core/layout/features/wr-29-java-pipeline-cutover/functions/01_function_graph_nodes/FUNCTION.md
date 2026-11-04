# 01_function_graph_nodes — SrcNode / CharsNode 真实类型

| 项 | 值 |
|----|----|
| feature | wr-29-java-pipeline-cutover |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SrcNode` / `CharsNode`（`KPList.Node` 子类） |
| status | **pass** |
| 阶段 | **layout**（类型面；尚未 prepareGraphs） |

## 1. 功能说明

### 1.1 一句话
把 `LayoutState.srcList` / `charsList` / `Graphs.charsRef` 用的 **Copy 空 struct** 换成与 Java 同名的真实 `SrcNode` / `CharsNode` 模块（init / dispose / toString / Alloc）。

### 1.2 输入
空节点或 `Alloc.alloc()`；管线用例同一份 hello-world.docx（只证明 parse 后仍能构造与 rope 对齐的 CORECHARS 节点）。

### 1.3 输出
- `graph/SrcNode.rs`、`graph/CharsNode.rs` 登记进 crate
- `Graphs.rs` 不再内嵌 `pub struct SrcNode` / `CharsNode` 影子
- `LayoutState.rs` 不再 `pub struct SrcNode;` / `CharsNode;` / `Graphs;`（改为 `use` 真实模块）

### 1.4 上游
P0 rope；已有 `Graphs.rs` 交错模板（本 function **禁止**重写 Graphs 方法体）。

### 1.5 下游
`02_function_graph_buff` 的 `prepareGraphs` 往这些节点写字段。

### 1.6 不变量与函数改写列表

**SrcNode.rs（全部在列表上）**

- 常量 `TYPE_SRC_*`
- 字段 `type` `revisionFinal` `doc` `cp` `length` `data` `finalRunProp`
- `init` `SrcNode()` `dispose` `toString`
- `Alloc.alloc`

**CharsNode.rs（全部在列表上）**

- 常量 `TYPE_CHARS_*`
- 字段（Java 全部实例字段）
- `init` `CharsNode()` `dispose` `toString`
- `Alloc.alloc`

**Graphs.rs / LayoutState.rs：** 只改 `use` / 去掉本文件 Copy 影子。**不在改写列表上的方法保持原样**（含 DEFERRED）。

### 1.7 依赖与实现状态（必写，着重）

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `KPList.Node` / `KPNode` | util `KPList.rs` | `super_.dispose()` | 是 | — | 不重写 KPList |
| `Document` | `core/Document.rs` | `doc` 可空回指 | trait 在 | 本 function 只存 `Option<*mut dyn Document>` | 不改 Document 方法 |
| `PropertySet` | util | `finalRunProp` 可空 | 是 | — | — |
| `TypoFontStyle` | Java `cache.style.TypoFontStyle` | `fontStyles[10]` | **layout `env/TypoFontStyle`** 为度量最小集；`cache/style/TypoFontStyle.rs` 仍是生成 stub | 数组槽用 **env** 同名类型（§1.7.1 GN-D1） | 不改写 cache.style 整叶 |
| `GeoText` | drawing | `geoText` | 无 | `Option<()>` + `CARGO_OFF_SURFACE`（hello 无 geotext） | 不引进 drawing 闭包 |
| `Graphs` | wr 已有模板 | `charsRef: Option<CharsNode>` | 方法已在 | 本 function 只换类型、不改编排 | 不改 `isAutoNum` 等 |
| `LayoutState` | env | 字段类型 | 模板在 | 假 `KPList<T>` 仍是 PhantomData（02 再换真列表） | 不解 `prepareGraphs` 调用链 |
| `GraphBuff` | 02 | — | stub | 整叶 | 禁止本 function 登记 GraphBuff |

#### 1.7.1 编号条目

- **GN-D1**：Java `CharsNode.fontStyles` 是 `cache.style.TypoFontStyle[]`。Rust cache.style 叶未接线；用已登记 `layout/base/env/TypoFontStyle` 占 10 槽（可空）。证据：`Graphs.rs` 已 `use env::TypoFontStyle`。owner：wr-10 / 04。
- **GN-D2**：`doc` 与 Java 一样是文档回指，不是 owned。`Option<*mut dyn Document>`。owner：01。
- **GN-D3**：`CharsNode.srcRef` Java 指向 `srcList` 中同一节点。本 function 先 `Option<Box<SrcNode>>` 以便单测 toString；02 接入真 KPList 时改为 `Option<*mut SrcNode>`（登记在 02）。当前单测不把节点链进 LayoutState 假列表。

## 2. 核心代码

对照 `SrcNode.java` / `CharsNode.java` 方法体移植。禁止 dummy `let X = ()`。Java `toString` 在 `srcRef==null` 时 NPE：Rust `toString` 对 `None` 写 `"null"` 并加 `// rust:` 说明（测试覆盖该分支）。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|--------|----------------|-----------|-------------|---------------|
| `SrcNode()` | SrcNode.java:31–34 | 私有 ctor | `type=-1` `cp=-1` | 仅 Alloc 调用 |
| `SrcNode.init` | :21–29 | 任意脏字段 | 全部复位 | `revisionFinal=true` |
| `SrcNode.dispose` | :36–40 | 已设 doc/data/prop | 三字段 null + super | — |
| `SrcNode.toString` | :43–69 | `type` 0/1/2/3/default | 前缀 CORECHARS/AUTONUM/… | switch 五臂 |
| `SrcNode.Alloc.alloc` | :72–75 | — | `new SrcNode()` | — |
| `CharsNode()` | :69–77 | — | `fontStyles.length==10` | authors -1 |
| `CharsNode.init` | :38–67 | 脏 fontStyles | 槽全 null、flag 复位 | 循环 0..size |
| `CharsNode.dispose` | :79–83 | srcRef/prop | null + super | — |
| `CharsNode.toString` | :85–114 | type + srcRef | REF/RESULT/… + `[src]` | srcRef NPE |
| `LayoutState` 影子 | LayoutState.rs:93–99 | 编译 | 无本地 `pub struct SrcNode` | 产品类型同一模块 |
| 管线 | DocumentOpen + hello-world.docx | 真实 parse | 可按 rope 长度填 CORECHARS | 不停在 render |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|-------------|---------------|----------|-------|----------------------|
| GN1 | Alloc+ctor 默认 | 正常主路径 | 是 | GN1 |
| GN2 | init 复位脏字段 | 状态转换 | 是 | GN2 |
| GN3 | toString CORECHARS | 正常 | 是 | GN3 |
| GN4 | toString AUTONUM/BALLOON/MATH | 分支 | 是 | GN4 |
| GN5 | toString INVALID | 边界 | 是 | GN5 |
| GN6 | dispose 清空 | cleanup | 是 | GN6 |
| GN7 | CharsNode ctor 10 槽 | 正常 | 是 | GN7 |
| GN8 | CharsNode.init 清 fontStyles | 循环 N | 是 | GN8 |
| GN9 | toString 五 type + srcRef | 分支 | 是 | GN9 |
| GN10 | toString srcRef=None | 失败/边界 | 是 | GN10 |
| GN11 | LayoutState/Graphs 无 Copy 影子 | 产品接线 | 是 | GN11 |
| GN12 | hello-world.docx → CORECHARS 节点 | 管线 layout | 是 | GN12 |

### 3.3 单元测试 / 管线测试

单元：直接 `Alloc.alloc` / `init`，不启动 GraphBuff。  
管线：同一 `hello-world.docx` sha256 `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` → `DocumentOpen` 真实 parse → 主故事 `getLength()>0` → 构造 `SrcNode` `TYPE_SRC_CORECHARS` `cp=0` `length=getLength()`。停在 **layout 类型面**，不要求 `flowToPage` 出行。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|----|--------|----------|----------|-----------------|----------|
| GN1 | GN1 | — | Alloc.alloc | type==-1 cp==-1 | ctor 未跑 |
| GN2 | GN2 | 脏 type/cp/data | init | type=-1 revisionFinal=true data=None | 残留脏值 |
| GN3 | GN3 | CORECHARS cp=0 length=3 | toString | 含 `CORECHARS` `cp=0` `length=3` | 走 default |
| GN4 | GN4 | type=1/2/3 data=x | toString | AUTONUM/BALLOONINFO/MATH + data | 缺臂 |
| GN5 | GN5 | type=99 | toString | `INVALID` | 当 CORECHARS |
| GN6 | GN6 | doc/data/prop Some | dispose | 三者 None | 泄漏 |
| GN7 | GN7 | Alloc | fontStyles.len()==10 | 不是 10 | |
| GN8 | GN8 | 槽 0 有 style | init | 槽全 None | 循环没跑 |
| GN9 | GN9 | type 0–4 + srcRef | 前缀 REF/RESULT/OMATH/MATH_FUNCTION/EQUATION | 错字 |
| GN10 | GN10 | srcRef None | toString 含 `null` 且不 panic | 与 Java NPE 差：登记 rust: |
| GN11 | GN11 | 源码/类型 | `LayoutState::default` 的 Graphs 与 `graph::Graphs` 同一 type_id；rs 无 `pub struct SrcNode;` 影子 | 仍影子 |
| GN12 | GN12 | hello-world.docx | parse 后 length>0；SrcNode CORECHARS length 等于 getLength | parse 失败或 length=0 |

### 3.5 测试代码位置

crate: `libs/word-core/Cargo.toml`  
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/SrcNode.rs`  
       `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/CharsNode.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/graph/SrcNode.rs`  
       （`#[path = "CharsNode.rs"]` 若拆文件；本 function 入口一个 `[[test]]`）

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_graph_nodes

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_graph_nodes \
  --src src/cn/wps/moffice/writer/layout/base/graph/SrcNode.rs \
  --src src/cn/wps/moffice/writer/layout/base/graph/CharsNode.rs \
  --report libs/word-core/layout/features/wr-29-java-pipeline-cutover/functions/01_function_graph_nodes/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_graph_nodes --json`)
date: 2026-10-02 14:07

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/graph/CharsNode.rs` | 104/135 | 77.0% | 87/124 | 70.2% |
| `src/cn/wps/moffice/writer/layout/base/graph/SrcNode.rs` | 73/73 | 100.0% | 77/77 | 100.0% |
| **function total** | **177/208** | **85.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/SrcNode.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/SrcNode.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/CharsNode.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/CharsNode.java
```

### 3.9 Java↔Rust 门面对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/SrcNode.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/SrcNode.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/CharsNode.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/CharsNode.rs \
  --report libs/word-core/layout/features/wr-29-java-pipeline-cutover/functions/01_function_graph_nodes/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-10-02 06:06

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SrcNode` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/SrcNode.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/SrcNode.rs` | **PASS** | `init`, `dispose`, `toString` | — | — | **PASS** |
| `Alloc` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/SrcNode.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/SrcNode.rs` | **PASS** | `alloc` | — | — | **PASS** |
| `CharsNode` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/CharsNode.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/CharsNode.rs` | **PASS** | `init`, `dispose`, `toString` | — | — | **PASS** |
| `Alloc` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/CharsNode.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/CharsNode.rs` | **PASS** | `alloc` | — | — | **PASS** |

**overall: PASS** — basename aligned; every Java class has layout PASS and a Rust counterpart for each public method.
<!-- java-rust-align:end -->

### 3.10 Java↔Rust 管线对比（逻辑 + 产品接线）

| 步 | Java | Rust |
|----|------|------|
| 1 | `new SrcNode.Alloc().alloc()` | `SrcNode::Alloc.alloc` |
| 2 | `node.init()` / 填 CORECHARS | 同名方法 |
| 3 | `LayoutState.srcList` 元素类型 `SrcNode` | 同一模块 `graph::SrcNode`（假 KPList 尚未存节点） |

产品：`attach_pageview_hot_path` / `LayoutState::default` 字段类型指向真实模块，不是测试旁路第二套 `SrcNode`。

### 3.11 管线阶段对齐报告

| 项 | 内容 |
|----|------|
| 阶段（parse / layout / render） | layout（图节点类型） |
| 原始文档（同一份，sha256） | `tests/fixtures/p1_layout_document/hello-world.docx` `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` |
| 步骤对照 | 见 §3.10；parse=`DocumentOpen`/`fill_text_document_from_docx` |
| 产品接线 | `LayoutState` / `Graphs` 使用 `graph::SrcNode`/`CharsNode` |
| 关键路径真实实现 | **pass**（`init`/`dispose`/`toString`/`Alloc.alloc` 无 DEFERRED） |
| 管线测试 test_id | GN12 |
| 测试结果符合预期 | **pass**（12/12 `cargo test --test 01_function_graph_nodes`） |
| 判定 | **pass** |
