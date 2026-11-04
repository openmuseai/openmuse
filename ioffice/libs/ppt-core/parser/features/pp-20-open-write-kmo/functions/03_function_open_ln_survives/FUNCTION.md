# 03_function_open_ln_survives — open 路径 ln 不丢

> feature: pp-20-open-write-kmo ｜ 状态：**pass** ｜ Java 管线：S4 产线写入

## 0. 管线能力解锁

pp-19:05 只保证 handler 测试面。产线 `parse_one_sp` 把 data ShapePr 摊进 layout_shape，ln 常丢。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | spPr 含 a:ln w=25400 的 zip |
| **输出** | dgShape.spPr().ln() 可查宽度 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| ShapePropertiesHandler | dmlr/sppr | 1114148 → `OutlineHandler(this.shapePr.ln())` | **本 function**：width/noFill 写入同一 data ShapePr | 3D/effect | 禁止改 element id / ctor 契约 |
| OutlineHandler | dmlr/sppr/outline | startElement attr 1114304 putWidth；noFill 1114370 | **本 function** 写穿到 `shapePr.ln()` | dash/headEnd/tailEnd 细节 | 禁止删 `// java` |
| open_pptx_document_bytes | pptx_mvp | zip → film 树 + dgShape.spPr | **本 function**：feed_open_sp_pr walk 内层 spPr | PptxReader 全编排 | 禁止 --force PptxrSlide |

## 2. 核心代码与函数改写列表

**本 function：** open 后查询走 `drawing.data.shape.ShapePr.ln()`（Java `Shape.open(kmo.drawingShape()).spPr()`）。
**§3.8：** ShapePropertiesHandler、OutlineHandler。pptx_mvp 无一对一 Java 叶。
**禁改：** 改 pp-19:05 断言冒充本测。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 `parse_one_sp:812-817`：`pr.geom_obj/fill_obj/xfrm` 从 handler 拷出，**无 ln 字段赋值**。Java `SwShapeHandler.endElement` 提交整个 dgShape.spPr()。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| LN-W | open+spPr.ln | a:ln w=25400 | ln 宽度 == 25400 EMU | 公开入口 |
| LN-NF | a:ln/a:noFill | noline | getfLine 等价 false 或无 stroke 宽 | 关键分支 |
| LN-MISS | spPr 仅 xfrm | 无 a:ln | hasLn==false，不 panic | 边界 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 线宽 | open+spPr.ln | 公开入口 | 是 | LN-W |
| noFill 线 | a:ln/a:noFill | 关键分支 | 是 | LN-NF |
| 无 ln 标签 | spPr 仅 xfrm | 边界 | 是 | LN-MISS |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| LN-W | 线宽 | — | a:ln w=25400 | ln 宽度 == 25400 EMU | open 后 ln None |
| LN-NF | noFill 线 | — | noline | getfLine 等价 false 或无 stroke 宽 | 空线当实线 |
| LN-MISS | 无 ln 标签 | — | 无 a:ln | hasLn==false，不 panic | 捏造默认黑线 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/pptxr/pptx_mvp.rs, src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs, src/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.rs
test:  tests/cn/wps/show/pptxr/03_function_open_ln_survives.rs
[[test]] name = "03_function_open_ln_survives"
```

Loop 已落地：LN-W/NF/MISS 三条非 ignore。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_open_ln_survives -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_open_ln_survives \
  --src src/cn/wps/show/pptxr/pptx_mvp.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.rs \
  --report libs/ppt-core/parser/features/pp-20-open-write-kmo/functions/03_function_open_ln_survives/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_open_ln_survives --json`)
date: 2026-09-15 18:33

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs` | 50/110 | 45.5% | 59/143 | 41.3% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.rs` | 57/224 | 25.4% | 69/325 | 21.2% |
| `src/cn/wps/show/pptxr/pptx_mvp.rs` | 624/1059 | 58.9% | 1015/1869 | 54.3% |
| **function total** | **731/1393** | **52.5%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

**PASS**（2026-09-15）：ShapePropertiesHandler / OutlineHandler / SwShapeHandler 布局校验均 PASS。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.java --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.java --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.rs \
  --report libs/ppt-core/parser/features/pp-20-open-write-kmo/functions/03_function_open_ln_survives/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-15 10:33

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShapePropertiesHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs` | **PASS** | `ShapePropertiesHandler`, `startElement`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `OutlineHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.rs` | **PASS** | `OutlineHandler`, `init`, `startElement`, `endElement`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
