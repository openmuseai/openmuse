# 05_function_sppr_data_shapepr — spPr 写入 `drawing.data.shape.ShapePr`

> feature: pp-19-kmo-tree-write ｜ 状态：**pass** ｜ Java 管线：S4 步骤 4.5

## 0. 管线能力解锁

实现后：`dgShape.spPr()` 是 Java `ShapePr`（xfrm/geom/fill/**ln**）。今日 `ShapePropertiesHandler` `use layout_shape::{ShapePr, GeomObj, OutlineObj}`，pp-10 文档自称写 `data/shape/ShapePr.rs` **与源码不符**；评审记 `a:ln` 提交被丢。

## 1. 功能说明

### 1.1 I/O 契约

Java ctor：`ShapePropertiesHandler(ShapePr shapePr, HandlerCallback)`。  
`enterSubElement`：ln=1114148 → `shapePr.ln()`；fill 1114370–74+3211586；custGeom 1114428 `geom.putType(1)`；prstGeom 1114429 `putType(0)`；xfrm=3211589。  
**pass 口径：** handler 字段类型 = `ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr`；`endElement` 后 `ln` 仍可读（不得只改 clone 再丢）。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `data/shape/ShapePr.rs` | drawing | xfrm/geom/fill/ln | ✅ 1300 行；`ln()` 内 follow `todo!` | follow 链 | 禁止删 KoBin 布局 |
| `ShapePropertiesHandler` | dmlr | 七链分发 | ⚠️ 写 layout_shape | 3D/effect | 禁止继续 alias GeomObj |
| OutlineHandler | pp-10 | w/cap/join | 测试面 | pattern | 禁止提交丢 ln |

## 2. 核心代码与函数改写列表

- **本 function：** `ShapePropertiesHandler` 入参/字段改为 data `ShapePr`；`enterSubElement` ln/fill/geom/xfrm；commit 后 ln 仍在同一对象。
- **停车：** scene3D/sp3d/effectLst `todo!("DEFERRED_NON_FUNCTION")`；`ShapePr.follow`。
- **禁改：** DGMap 枚举值；element id。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`ShapePropertiesHandler.java:26-94`；Rust handler 15 行 `layout_shape` import；`ShapePr.rs` `ln()` follow todo。

### 3.2 覆盖矩阵

| coverage_id | 场景 | 必测? | test_id |
|-------------|------|-------|---------|
| PR-TY | 构造类型是 data ShapePr | 是 | PR1 |
| PR-LN | ln 1114148 后 has ln | 是 | PR2 |
| PR-PST | 1114429 putType 0 | 是 | PR3 |
| PR-XFR | 3211589 xfrm | 是 | PR4 |
| PR-UNK | 未知子元素 null | 是 | PR5 |
| PR-HOST | 今日 handler 对 data ShapePr 分发 | 是 | PR0 |

### 3.4 用例表

| id | 期望 | 失败语义 |
|----|------|----------|
| PR0 | handler 对 data ShapePr 分发 ln/prst/unknown | 误拆旧路径 |
| PR1–PR5 | data ShapePr 构造、ln 存活、putType 0、xfrm 同对象、unknown None | 仍写 fixture |

### 3.5 测试代码位置

test: `tests/cn/wps/drawing/dmlr/handler/shape/sppr/05_function_sppr_data_shapepr.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 05_function_sppr_data_shapepr -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 05_function_sppr_data_shapepr \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs \
  --src src/cn/wps/drawing/data/shape/ShapePr.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/05_function_sppr_data_shapepr/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 05_function_sppr_data_shapepr --json`)
date: 2026-09-12 16:09

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/data/shape/ShapePr.rs` | 66/768 | 8.6% | 89/1209 | 7.4% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs` | 34/105 | 32.4% | 41/136 | 30.1% |
| **function total** | **100/873** | **11.5%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/data/shape/ShapePr.rs \
  --java koffice/presentation/core/cn/wps/drawing/data/shape/ShapePr.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/data/shape/ShapePr.java \
  --rs libs/ppt-core/src/cn/wps/drawing/data/shape/ShapePr.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/05_function_sppr_data_shapepr/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-12 08:10

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShapePropertiesHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs` | **PASS** | `ShapePropertiesHandler`, `startElement`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `ShapePr` | `koffice/presentation/core/cn/wps/drawing/data/shape/ShapePr.java` | `libs/ppt-core/src/cn/wps/drawing/data/shape/ShapePr.rs` | **PASS** | `create`, `initialize`, `open`, `ShapePr`, `loadBin`, `load`, `saveBin`, `equalsLax`, `equals`, `hashCode`, `dirty`, `save`, `lastBin`, `discard`, `close`, `cleanFollow`, `cleanCache`, `follow`, `unFollow`, `hasParent`, `parent`, `cleanParent`, `bwMode`, `hasBwMode`, `deleteBwMode`, `putBwMode`, `xfrm`, `hasXfrm`, `deleteXfrm`, `writeXfrm`, `forceWriteXfrm`, `geom`, `hasGeom`, `deleteGeom`, `writeGeom`, `forceWriteGeom`, `fill`, `hasFill`, `deleteFill`, `writeFill`, `forceWriteFill`, `ln`, `hasLn`, `deleteLn`, `writeLn`, `forceWriteLn`, `effect`, `hasEffect`, `deleteEffect`, `writeEffect`, `forceWriteEffect`, `scene3D`, `hasScene3D`, `deleteScene3D`, `writeScene3D`, `forceWriteScene3D`, `sp3d`, `hasSp3d`, `deleteSp3d`, `writeSp3d`, `forceWriteSp3d` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
