# 05_function_shape_tree_dispatch — recursive shape-tree orchestration

> feature: pp-02-shape-tree-parse ｜ 状态：**pass**

## 0. 管线能力解锁

已实现并验收：spTree 编排生成有序、可递归的形状树——leaf 委托 02/03/04、group 递归、AlternateContent 只保留有效输出，结束时清理空 graphic frame 并提交 group row。缺失时断在哪：形状树整体不存在，pl-02/pl-06/pr-01 全断（本 function 是 pp-02 唯一编排入口，最重要）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

把 `p:spTree` 和 `mc:AlternateContent` 事件编排成有序、可递归的 shape tree。普通 leaf 委托
02/03/04；group 递归委托本 function；graphic frame 走独立 producer；AlternateContent 在
Choice/Fallback/ink 路径中只保留有效输出。结束时清除无有效 graphic 的 frame，再提交 group。

### 1.1 I/O 契约

```text
(root KmoShowShape, root Shape, DgStore, HandlerCallback, spTree events)
  -> root children in XML order + committed descendant rows + committed group row
```

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| non-visual/leaf producers | pp-02/01–04 | typed child handler + committed leaf | **pass**；编排测试使用 frozen producer outputs | 无 | 不重写 leaf internals |
| group properties | DML parser | `GroupShapePropertiesHandler` | Rust 当前被裁剪 | DML transform function | 不伪造 transform |
| graphic frame producer | 后续 graphic-frame parser | allocate/init/parse/result validity | Java 存在；Rust 当前工作树已删除 | 非 MVP branch 可按模板停车，但必须显式 | 不用空 graphic 表示成功 |
| `DgStore/KmoShowShape/GraphicObject` | model | allocate child、ordered add/remove、query/update | 当前 app/model 面被裁剪 | model feature | 不造局部 store/shape stub |
| callback/FileManager | pp-01/host | `isGVml/onInk` | callback **pass**；FileManager 使用 host contract | host 实际文件读取 | 不实现 ink 文件解析 |
| transform/fill models | DML model | ink transform、fallback stretch rect | 部分存在 | DML feature | 不改变 fill 规则 |
| XML seam | kfc | event dispatch | Rust 未接线 | 上游 | 不打开 ZIP/slide |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java group | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.java` |
| Rust group | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs`（当前工作树缺失） |
| Java alternate | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwShapeAlertnateContentHandler.java` |
| Rust alternate | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeAlertnateContentHandler.rs` |
| direct caller | `.../handler/slides/CommonSlideDataHandler.java` |

改写列表：`SwGroupShapeHandler` 全部方法及 nested non-visual group handler；
`SwShapeAlertnateContentHandler` 两个 ctor、init、enter/end，及 Choice/ContentPart/FallBack/OleObject
全部方法。loop 要消除当前 Rust raw-pointer nested-owner 方案或给出可验证的 aliasing 依据；不得把
Java 的递归/回退状态机压成一组返回 `None` 的 dispatch。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|
| group ctor/init | root/preallocated group | type=0/id/reset reusable handlers | 首次/复用 |
| group dispatch | alt/ext/pic/nv/grpPr/sp/grp/graphic/cxn/unknown | allocate + ordered child + delegate | 0/1/N、递归 |
| group end | children + graphic validity | remove invalid frames + commit | reverse loop 0/1/N |
| alternate Choice | graphic/contentPart | choice frame 或 rId/transform | present/absent |
| alternate Fallback | pic/sp/grp/graphic/cxn/OLE | selected shape | 各类型、unsupported |
| alternate end | shape/rId/transform/blip/ink/GVML list | insert/drop + store update | 多条件组合 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | empty tree | 边界 | 是 | T01 |
| C02 | leaf sp/pic/cxn order | 编排 | 是 | T02 |
| C03 | nested group 1/N depth | 递归/循环 | 是 | T03–T04 |
| C04 | handler reuse via init | 状态转换 | 是 | T05 |
| C05 | graphic valid/invalid cleanup | cleanup | 是 | T06 |
| C06 | Alternate Choice graphic | 分支 | 是 | T07 |
| C07 | Fallback five shape kinds | 分支 | 是 | T08 |
| C08 | contentPart rId + transform + ink success/fail | 失败/状态 | 是 | T09–T11 |
| C09 | blip stretch rect normalization | 业务分支 | 是 | T12 |
| C10 | GVML `newSps` vs parent insertion | 模式 | 是 | T13 |
| C11 | unknown elements/unsupported frame | 失败语义 | 是 | T14 |
| C12 | store/protocol commit order | 交互 | 是 | T15 |

### 3.3 可测试性

编排测试使用 01–04 的 frozen producer outputs 和 Recording `DgStore`/callback/graphic producer；
事件序列直接模拟 XML reader 输出。T03/T04 只扩展 group 事件，不启动 slide parser。ink 用
Recording callback 返回固定成功 id 或 `-1`，但 shape/fill/transform 仍使用真实模型类型。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望输出/副作用 |
|---|---|---|---|
| T01 | C01 | empty spTree | 空 children，group 提交一次 |
| T02 | C02 | sp,pic,cxn | child type `[1,2,3]` 且顺序一致 |
| T03 | C03 | group→sp | 两层 id/row 正确 |
| T04 | C03 | 三层 group + 多 leaf | 所有 row 一次提交，无丢/重 child |
| T05 | C04 | 连续两个同类 leaf | handler init 新 shape，无旧状态泄漏 |
| T06 | C05 | valid/invalid graphic 混合 | 只逆序删除 invalid，索引稳定 |
| T07 | C06 | Choice graphic | choice frame 进入 tree |
| T08 | C07 | 每种 fallback | 选中类型及 delegate 正确 |
| T09 | C08 | rId+transform、ink id | 创建 shape、写 ink/transform、插入 tree |
| T10 | C08 | rId+transform、`-1` | 不创建/插入假 shape |
| T11 | C08 | 缺 rId 或 transform | 无 ink fallback 副作用 |
| T12 | C09 | tile-like blip no stretch rect | 创建空 `RelativeRect`；其它 fill 不改 |
| T13 | C10 | GVML newSps / normal | 分别写 list 或 parent+group children |
| T14 | C11 | unknown/unsupported frame | `None`/明确停车，不产生有效 row |
| T15 | C12 | mixed tree end | child commits 先于 group，protocol/store 各一次 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs
src:   libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeAlertnateContentHandler.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/handler/shape/SwShapeAlertnateContentHandler.rs
[[test]] name = "05_function_shape_tree_dispatch" path = ".../SwGroupShapeHandler.rs"
```

入口测试用 `#[path = "SwShapeAlertnateContentHandler.rs"] mod alternate;` 拉取同目录辅助测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 05_function_shape_tree_dispatch
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 05_function_shape_tree_dispatch \
  --src src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs \
  --src src/cn/wps/show/pptxr/handler/shape/SwShapeAlertnateContentHandler.rs \
  --report libs/ppt-core/parser/features/pp-02-shape-tree-parse/functions/05_function_shape_tree_dispatch/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 05_function_shape_tree_dispatch --json`)
date: 2026-08-28 15:13

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs` | 210/266 | 78.9% | 242/284 | 85.2% |
| `src/cn/wps/show/pptxr/handler/shape/SwShapeAlertnateContentHandler.rs` | 338/434 | 77.9% | 405/516 | 78.5% |
| **function total** | **548/700** | **78.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeAlertnateContentHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwShapeAlertnateContentHandler.java
```

Loop 结果：**PASS**（group 130 条、AlternateContent 184 条 `// Java:` 模板锚点保留且顺序对齐；
raw-pointer nested-owner 已改为 `Rc<RefCell<State>>`，无 `unsafe`）。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwShapeAlertnateContentHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeAlertnateContentHandler.rs \
  --report libs/ppt-core/parser/features/pp-02-shape-tree-parse/functions/05_function_shape_tree_dispatch/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-28 07:12

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SwGroupShapeHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs` | **PASS** | `SwGroupShapeHandler`, `init`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `SwShapeAlertnateContentHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwShapeAlertnateContentHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeAlertnateContentHandler.rs` | **PASS** | `SwShapeAlertnateContentHandler`, `init`, `enterSubElement`, `endElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
