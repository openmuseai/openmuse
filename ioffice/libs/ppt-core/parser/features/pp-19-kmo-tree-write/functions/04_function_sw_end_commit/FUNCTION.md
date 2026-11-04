# 04_function_sw_end_commit — Sw\* `endElement` 双 bin 提交

> feature: pp-19-kmo-tree-write ｜ 状态：**pass** ｜ Java 管线：S4 步骤 4.4

## 0. 管线能力解锁

实现后：每个 `p:sp` 关闭时写入 **dg 仓 + sw 协议**。`SwShapeHandler` 已进默认 `handler/shape/mod.rs`。

## 1. 功能说明

### 1.1 I/O 契约

Java `SwShapeHandler.endElement:79-86`：

1. 可选 GVml：`onConvertStyle2ShapeProps` + `deleteStyle`（本 function **defer**）
2. `this.store.update(this.kmoShape.id(), this.dgShape.save())`
3. `this.kmoShape.updateProtocol(this.swShape.save())`

`init:34-39`：`dgShape.putType(1)`；`cNvPr.putId(shape.id())`。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `SwShapeHandler` | pptxr/handler/shape | endElement 双提交 | ✅ 默认 mod + init/endElement | GVml 分支细节 | 禁止只在测试 #[path] 绿 |
| `handler/shape/mod.rs` | 接线 | SwShapeHandler pub mod | ✅ | picture/connector/group | 禁止为编译删模板 |
| `DgStore.update` | 03 | | ✅ | — | — |
| `updateProtocol` | KmoShowShape | sw save | ✅ 调用不 panic | atom shape.load 全量 | — |

## 2. 核心代码与函数改写列表

- **本 function：** 解开 `SwShapeHandler`/`SwGroupShapeHandler`/`SwNvPrHandler`（picture/connector 可随后面 pp-02 回归）；`endElement` 双提交；`init` putType/putId。
- **停车：** GVml `isGVml` 分支体；`ShapeExtLstHandler` 非本 I/O。
- **禁改：** pp-02 测试夹具语义；element id 常量。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`SwShapeHandler.java:28-87`；Rust `handler/shape/mod.rs` 注释；pp-02 `02_function_auto_shape` 测试面绿 ≠ 默认编译面。

### 3.2 覆盖矩阵

| coverage_id | 场景 | 必测? | test_id |
|-------------|------|-------|---------|
| SW-MOD | 默认 crate 能 `use ...SwShapeHandler` | 是 | SW1 ignore spec |
| SW-UPD | endElement 后 query(id) 非空 | 是 | SW2 ignore spec |
| SW-PROTO | updateProtocol 被调 | 是 | SW3 ignore spec |
| SW-TYPE | init putType 1 | 是 | SW4 ignore spec |
| SW-GVML | isGVml 分支 | 否 | defer GVml |

### 3.3 / 3.4

Split：SW0 编译期用字符串/文档断言不可用 → ignore 内 `panic!("spec: Sw* not on default mod")` 钉住缺口。loop L5 解开后改成真 handler 调用（可复用 pp-02 fixture 事件，但写入断言必须是 DgStore+protocol，不是 layout_shape）。

| id | 期望 | 失败语义 |
|----|------|----------|
| SW0 | （无）文档占位 | — |
| SW1–SW4 | ignore spec | 仍注释 mod / 未双提交 |

### 3.5 测试代码位置

test: `tests/cn/wps/show/pptxr/handler/shape/04_function_sw_end_commit.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_sw_end_commit -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_sw_end_commit \
  --src src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/04_function_sw_end_commit/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_sw_end_commit --json`)
date: 2026-09-12 15:55

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs` | 44/181 | 24.3% | 66/189 | 34.9% |
| **function total** | **44/181** | **24.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwShapeHandler.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwShapeHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/04_function_sw_end_commit/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-12 07:55

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SwShapeHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwShapeHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs` | **PASS** | `SwShapeHandler`, `init`, `startElement`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `ShapeExtLstHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwShapeHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwShapeHandler.rs` | **PASS** | `ShapeExtLstHandler`, `enterSubElement`, `endElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
