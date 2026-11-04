# 04_function_connector_shape — p:cxnSp leaf commit

> feature: pp-02-shape-tree-parse ｜ 状态：**pass**

## 0. 管线能力解锁

已实现并验收：p:cxnSp 事件提交 type=3 的 shape/protocol，提交后经 `HandlerCallback.onConnectorShape` 把原始 start/end connection id 重定向到 core id。缺失时断在哪：连接线节点不在形状树中，pl-02/pl-06/pr-01 输入断。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

把 `p:cxnSp` 事件写入 preallocated connector，提交 type=3 的 shape/protocol，并在提交后调用
`HandlerCallback.onConnectorShape(id)`，供后续把原始 start/end connection id 重定向到 core id。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| non-visual producer | pp-02/01 | common nvPr | **pass**；使用 frozen upstream contract | 无 | 不重复解析 placeholder/media |
| `CNvPr/CNvCxnSpPr/ShapeProperties/ShapeStyle` handlers | DML parser | typed child handler | 当前被裁剪或 deferred | 各 DML function | 不用 `None` 假装支持 |
| callback | pp-01/02 | GVML merge + connector notification | **pass**；测试 re-export 真实 trait | 无 | 不实现 connector resolve 算法 |
| store/model/XML seam | model+kfc | id/type/commit/dispatch | 不完整 | 上游 | 不造 local stub |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java/Rust | `.../show/pptxr/handler/shape/SwConnectorShapeHandler.java/.rs` |
| callers | `SwGroupShapeHandler.java`、`SwShapeAlertnateContentHandler.java` |

改写列表：outer ctor/init/enterSubElement/endElement；nested
`NonVisualConnectionShapePropertiesHandler.enterSubElement`。loop 必须明确 Java callback 是否可空；
当前 Rust 无条件 unwrap 与 Java 无条件调用的失败语义要由测试锁定，不能静默吞掉通知。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| ctor/init | connector | type=3、id | 首次/复用 |
| outer dispatch | ext/spPr/style/nvCxn/unknown | typed handler | ext 当前 Java 忽略 |
| nested dispatch | cNvPr/nvPr/cNvCxn/unknown | typed handler | 四路 |
| endElement | normal/GVML + callback | commit + notification | callback 缺失错误语义 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | ctor/init | 状态 | 是 | T01 |
| C02 | outer child ids | 分支 | 是 | T02 |
| C03 | nested child ids | 分支 | 是 | T03 |
| C04 | ext/unknown ignored | 边界 | 是 | T04 |
| C05 | normal commit order | 副作用 | 是 | T05 |
| C06 | GVML merge/delete | 模式 | 是 | T06 |
| C07 | connector callback id/order | 交互 | 是 | T07 |
| C08 | callback missing | 失败语义 | 是 | T08 |
| C09 | repeated init | cleanup | 是 | T09 |

### 3.3 可测试性

Recording store/callback 记录 `merge → store/protocol commit → connector notification` 顺序；child
handler 用 typed factory seam。callback missing 用 Java 权威语义决定 panic/error，不接受固定成功。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 |
|---|---|---|---|
| T01 | C01 | id=9 | type=3、cNvPr.id=9 |
| T02 | C02 | 主 child ids | spPr/style/nvCxn handler；ext/unknown None |
| T03 | C03 | nested ids | 三个 typed handler + unknown None |
| T04 | C04 | ext/unknown | 无模型写入 |
| T05 | C05 | normal end | store/protocol/notify 各一次 |
| T06 | C06 | GVML end | merge/delete 后 commit/notify |
| T07 | C07 | id=9 | notify 参数 9 且发生在 commit 后 |
| T08 | C08 | callback absent | 与 Java 构造/调用契约一致的明确失败 |
| T09 | C09 | reuse id=10 | 只提交新 id |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwConnectorShapeHandler.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/handler/shape/SwConnectorShapeHandler.rs
[[test]] name = "04_function_connector_shape"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_connector_shape
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_connector_shape \
  --src src/cn/wps/show/pptxr/handler/shape/SwConnectorShapeHandler.rs \
  --report libs/ppt-core/parser/features/pp-02-shape-tree-parse/functions/04_function_connector_shape/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_connector_shape --json`)
date: 2026-08-28 15:02

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/shape/SwConnectorShapeHandler.rs` | 139/145 | 95.9% | 136/147 | 92.5% |
| **function total** | **139/145** | **95.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwConnectorShapeHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwConnectorShapeHandler.java
```

Loop 结果：**PASS**（65 条 `// Java:` 模板锚点保留且顺序对齐）。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwConnectorShapeHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwConnectorShapeHandler.rs \
  --report libs/ppt-core/parser/features/pp-02-shape-tree-parse/functions/04_function_connector_shape/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-28 07:02

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SwConnectorShapeHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwConnectorShapeHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwConnectorShapeHandler.rs` | **PASS** | `SwConnectorShapeHandler`, `init`, `enterSubElement`, `endElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
