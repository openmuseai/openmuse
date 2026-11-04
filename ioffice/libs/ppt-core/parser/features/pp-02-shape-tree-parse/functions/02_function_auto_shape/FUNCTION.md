# 02_function_auto_shape — p:sp leaf commit

> feature: pp-02-shape-tree-parse ｜ 状态：**pass**

## 0. 管线能力解锁

已实现并验收：p:sp 事件写入 preallocated `KmoShowShape(type=1)`，结束时把 ShowShape protocol 与 Shape row 提交到 DgStore，GVML 下 style 合入 shape properties。缺失时断在哪：形状树缺可渲染 leaf 节点，pl-02/pl-06/pr-01 输入断。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

输入 preallocated `KmoShowShape(type=1)` 与 `p:sp` 事件，解析 non-visual、shape properties、
style、text body 和 extension list；结束时把 `ShowShape` protocol 与 `Shape` row 分别提交到
`KmoShowShape` 和 `DgStore[id]`。GVML mode 还必须把 style 合入 shape properties 后删除 style。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| non-visual producer | 本 feature/01 | `nvSpPr -> ShowShape` | **pass**；dispatch 使用 frozen upstream contract | 无 | 不重复解析 placeholder/media |
| `DgStore/KmoShowShape/ShowShape/Shape/ShapeExt` | model | open/update/save/children/id | 当前保留面不完整 | 各 model feature | 不造 local stub |
| shape properties/style/text handlers | DML parser | typed child handlers | 当前多为删除/rewrite/deferred | 各 DML leaf function | 不把其内部解析并入本 function |
| `HandlerCallback` | pp-01/02 | `isGVml/onConvertStyle2ShapeProps` | **pass**；测试 re-export 真实 trait | 无 | 不实现关系资源解析 |
| XML seam | kfc | attrs + child dispatch | Rust 未接线 | 上游 XML feature | 不打开完整 XML/ZIP |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java/Rust | `.../show/pptxr/handler/shape/SwShapeHandler.java/.rs` |
| direct caller | `SwGroupShapeHandler.java`、`SwShapeAlertnateContentHandler.java` |

改写列表：outer ctor/init/startElement/enterSubElement/endElement；nested
`NonVisualShapePropertiesHandler`；`ShapeExtLstHandler`、`ShapeExtHandler`、`ModularGroupExtHandler`
全部方法。extension list 是本 function 的复用输出能力。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | 输入/状态 | 输出/副作用 | 分支 |
|---|---|---|---|
| ctor/init | preallocated shape | type=1、cNvPr.id | 首次/复用 init |
| startElement | useBgFill attr | ShowShape flag | present/absent |
| enterSubElement | txSp/ext/nv/spPr/style/text/unknown | typed handler | GVML/non-GVML/default |
| ext handlers | 0/1/N ext + uri/modular attrs | saved ext list | empty/unknown/repeated |
| endElement | mode | protocol/store commit | GVML merge vs normal |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | ctor/init id/type | 状态转换 | 是 | T01 |
| C02 | useBgFill | 分支 | 是 | T02 |
| C03 | 六种 child dispatch + unknown | 分支 | 是 | T03 |
| C04 | ext list 0/1/N | 循环/边界 | 是 | T04–T06 |
| C05 | ext uri/type/index/default attrs | 属性分支 | 是 | T07 |
| C06 | normal commit | 副作用 | 是 | T08 |
| C07 | GVML style merge/delete | 模式分支 | 是 | T09 |
| C08 | repeated init | cleanup/复用 | 是 | T10 |

### 3.3 可测试性

用 01 的 frozen `ShowShape` 输出、Recording child factories、Recording `DgStore` 与 callback；
只投递 `p:sp` 事件，不跑 group/slide/package。断言公开提交结果和 callback，不依赖私有字段。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 |
|---|---|---|---|
| T01 | C01 | id=42 | shape type=1、cNvPr.id=42 |
| T02 | C02 | attr absent/false/true | flag 保持/精确写入 |
| T03 | C03 | 每个 child id + unknown | 对应 handler；unknown None |
| T04–T06 | C04 | ext 0/1/3 | 空不写；非空按序 save/forceWrite |
| T07 | C05 | 完整/缺省 modular attrs | 仅 present 字段写入 |
| T08 | C06 | normal end | protocol 与 store 各提交一次 |
| T09 | C07 | GVML end | merge 一次、style 删除、再提交 |
| T10 | C08 | 同 handler init 新 shape | 不泄漏旧 id/ext/state |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs
[[test]] name = "02_function_auto_shape"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_auto_shape
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_auto_shape \
  --src src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs \
  --report libs/ppt-core/parser/features/pp-02-shape-tree-parse/functions/02_function_auto_shape/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_auto_shape --json`)
date: 2026-08-28 14:54

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs` | 273/295 | 92.5% | 277/313 | 88.5% |
| **function total** | **273/295** | **92.5%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwShapeHandler.java
```

Loop 结果：**PASS**（139 条 `// Java:` 模板锚点保留且顺序对齐）。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwShapeHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs \
  --report libs/ppt-core/parser/features/pp-02-shape-tree-parse/functions/02_function_auto_shape/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-28 06:54

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SwShapeHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwShapeHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs` | **PASS** | `SwShapeHandler`, `init`, `startElement`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `ShapeExtLstHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwShapeHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs` | **PASS** | `ShapeExtLstHandler`, `enterSubElement`, `endElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
