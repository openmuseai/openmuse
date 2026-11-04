# 01_function_prst_bind — prstGeom@prst 落到 ShapePr

> feature: pr-06-prstgeom-skeleton ｜ 状态：**pass**

## 0. 管线能力解锁

没有整数 `prst`，`geometry_commands` 无法区分 rect/ellipse/line/pentagon。当前 handler 写在 `geom_obj.clone()` 上，`commit()` 不拷回。

## 1. 功能说明

Java `PresetGeometryHandler.startElement`：`attrs.getByName(1114293)` → `DGMap.presetShapeTypeMap.get(prst)` → `presetGeometry.putPrst`。产品 `walk_xml` 必须映射属性 `prst→1114293`；`ShapePropertiesHandler.commit` 必须写回 `geom_obj`。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `a:prstGeom prst="…"`（可在 `p:spPr` 内）。 |
| **输出** | `shape.sp_pr.geom_type == Some(0)`；`geom_obj.prst == Some(n)`，n 为 Java 表：rect=1, ellipse=3, line=20, pentagon=56。 |
| **错误** | 未知 prst 字符串 → `prst` 保持 None（或 map miss），不 panic。无 prstGeom → None。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| `attr_id("prst")` | pptx_mvp.rs | 1114293 | ✅ | — | — |
| `DGMap::presetShapeTypeMap` | DGMap.rs | Java 全表至少 line/ellipse/pentagon/rect | ✅ rect/roundRect/ellipse/line/pentagon | 星形等 | 不发明第二套 id |
| `PresetGeometryHandler` | 同名 .rs | 写同一 Rc | ✅ | avLst | — |
| `ShapePropertiesHandler::commit` | ShapePropertiesHandler.rs | 拷 `geom_obj` | ✅ 共享 geom Rc + commit | ln 完整 | — |

## 2. 核心代码

| Java | Rust |
|---|---|
| `drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.java` | `PresetGeometryHandler.rs` |
| `drawing/dmlr/util/DGMap.java` presetShapeTypeMap | `DGMap.rs` |
| `ShapePropertiesHandler.endElement` | `ShapePropertiesHandler.rs` `commit` |

改写列表：attr_id、map 四键、commit geom_obj、handler 共用 Rc。停车：custGeom、avLst 调节点。禁改：PptxrSlide。

## 3. 测试方案

### 3.1 代码审计

已读：`PresetGeometryHandler.rs` 34-40；`ShapePropertiesHandler.rs` 51-61、91-102；`DGMap.rs` 155-158；Java DGMap 59-70（pentagon=56, line=20）。

| symbol | 输出 | 分支 |
|---|---|---|
| prst=pentagon | Some(56) | clone 丢失 |
| prst=line | Some(20) | map miss |
| 无 prstGeom | None | |
| 未知 "foo" | None | panic 禁 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 必测? | test_id |
|---|---|---|---|
| C01 | pentagon→56 | 是 | T01 |
| C02 | line→20；ellipse→3 | 是 | T02 |
| C03 | 无 prstGeom | 是 | T03 |
| C04 | attr_id("prst")==1114293 | 是 | T04 |
| C05 | 未知 prst 不 panic | 是 | T05 |

### 3.3 可测试性

`walk_xml`+`ShapePropertiesHandler` 或 `visual_pptx` 一段 sp。不跑 Flutter。

### 3.4 用例表

| id | 覆盖项 | 输入 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | `<a:prstGeom prst="pentagon"/>` | geom_obj.prst==56 | clone 丢掉 |
| T02 | C02 | line / ellipse | 20 / 3 | map 只有 ellipse |
| T03 | C03 | 仅 xfrm | prst None | 伪造 rect=1 |
| T04 | C04 | attr_id | 1114293 | walker 不进 startElement |
| T05 | C05 | prst="notAShape" | None，不 panic | |

### 3.5 位置

```text
src: src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs
     src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.rs
     src/cn/wps/drawing/dmlr/util/DGMap.rs
     src/cn/wps/show/pptxr/pptx_mvp.rs
test: tests/cn/wps/show/pptxr/01_function_prst_bind.rs
```

`#[ignore = "spec: pr-06-01 prst bind"]`

### 3.6 脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_prst_bind

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_prst_bind \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.rs \
  --src src/cn/wps/drawing/dmlr/util/DGMap.rs \
  --report libs/ppt-core/render/features/pr-06-prstgeom-skeleton/functions/01_function_prst_bind/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_prst_bind --json`)
date: 2026-09-09 21:03

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs` | 40/98 | 40.8% | 55/138 | 39.9% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.rs` | 16/34 | 47.1% | 18/44 | 40.9% |
| `src/cn/wps/drawing/dmlr/util/DGMap.rs` | 12/219 | 5.5% | 11/247 | 4.5% |
| **function total** | **68/351** | **19.4%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.java
```

Loop 后布局：**PASS**。禁 `--force` PptxrSlide。

### 3.9 Java↔Rust 结构对齐

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.rs \
  --report libs/ppt-core/render/features/pr-06-prstgeom-skeleton/functions/01_function_prst_bind/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-09 13:03

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PresetGeometryHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.rs` | **PASS** | `PresetGeometryHandler`, `endElement`, `startElement`, `enterSubElement` | — | — | **PASS** |
| `ShapePropertiesHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs` | **PASS** | `ShapePropertiesHandler`, `startElement`, `enterSubElement`, `endElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
