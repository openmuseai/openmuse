# 04_function_geometry — a:prstGeom / a:custGeom 最小集

> feature: pp-10-dml-sppr ｜ 状态：**pass** ｜ Java 管线环节：S0 parse → S1 convert 输入

## 0. 管线能力解锁

形状有坐标/填充/线但**无几何**时，S1 convert 的 `GeomtryConvert` 无 prst id 或无 custGeom path →
pr-02 无 `DrawPath` 的路径数据（矩形只能靠缺省兜底）。实现后发起者拿到**预置几何 id / 自定义
path** → S4 能画出形状体（矩形、圆、箭头等预置形状 + 自定义路径最小集）；缺失则链断在
「形状无几何」。

## 1. 功能说明

- `PresetGeometryHandler`（`p:prstGeom` → `Geometry2D.putType(0)`，由 01 分发入）：`startElement`
  读 prst=1114293 → `DGMap.presetShapeTypeMap`（miss 不写，如 `roundRect`/`ellipse`/`rect`…）→
  `putPrst`；`enterSubElement` 的 avLst=1114422 → `AdjustValueListHanlder`（gd=1114392 →
  `GeomGuideHandler`：name=1114136、fmla=1114391）；`endElement` 把 avLst save 后 write。
- `CustomGeometryHandler`（`p:custGeom` → `putType(1)`）**最小子集**：avLst=1114422 与
  gdLst=1114423（`GeomGuide` 同构复用）；rect=1114426 → `RectHandler`（l=1114213、t=1114214、
  r=1114193、b=1114195 → `processAdjCoordinateValue`：`Integer.parseInt` 成功 → putCoordinate，
  失败 → putGeomGuideName）；pathLst=1114427 → `PathListHandler` → path（attr extrusionOk=1114421
  `toBoolean`、fill=1114380 → `fillModeMap`、stroke=1114420 `toBoolean`、w=1114304/h=1114305
  `toInt`）→ 段：moveTo=1114415/lnTo=1114416（`PathSegmentPointHandler`→`AdjustPoint2DHandler`：
  x=1114161/y=1114162 走 processAdjCoordinate）/close=1114414（无点）。**quadBezTo=1114418、
  cubicBezTo=1114419、arcTo=1114417 延后**（fixture/契约旁路）。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `a:prstGeom`/`a:custGeom` attrs + 子元素事件 + 构造入参 `PresetGeometry`/`CustomGeometry`（经 `ShapePr.geom()` 分发，type 由 01 置 0/1）。 |
| **输出** | `presetGeometry.putPrst` + `avLst()`（GeomGuide 列表）；`customGeometry.avLst/gdLst`（GeomGuide 列表）+ `rect()`（l/t/r/b 的 AdjCoordinate 或 guide 名）+ `pathLst()`（最小段序列：close/moveTo/lnTo + AdjustPoint2D）。 |
| **状态与副作用** | gd/gs 用单一 handler `init` 循环复用（list 追加 + 绑定当前项）。`endElement` 判空 write（adjustableValues/shapeGuides/paths）。`processAdjCoordinateValue`：数字字面量 → putCoordinate；否则按 guide 名存。 |
| **错误语义** | `Integer.parseInt` 失败 → 走 guide 名分支（不是异常路径）。未知子元素 → null。`presetShapeTypeMap`/`fillModeMap` miss → 不 put。`cubicBezTo` 等未实现段 id → 不返回 handler（延后，注释标明；**不静默吞段**——夹具中标记「非最小集」提示）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止去改 |
|---|---|---|---|---|---|
| 分发入口 | pp-10:01 | prstGeom=1114429/custGeom=1114428 → 本功能 handler + `geom().putType(0/1)` | spec（随 01 落地） | — | 不改 01 |
| `Geometry2D`/`PresetGeometry`/`CustomGeometry`/`GeomGuide`/`AdjCoordinate` 等 | `data/geometry/*.rs` | putPrst/avLst/gdLst/rect/pathLst 写入接口 | 已存在 | — | 不改字段语义 |
| `DGMap` 几何 map | util | `presetShapeTypeMap`、`fillModeMap` | 审计 delete 空壳 | 以测试字面量/最小常量重建 | 不整表恢复 |
| sppr/geometry `.rs` | 本 feature | Java 结构模板 | **重做**：审计 `PresetGeometryHandler.rs` 129 行（31 stub）、`CustomGeometryHandler.rs` 1191 行（8 todo/381 stub，「dispatch empty」）；模块接线已注释 | — | 不增量 |

### 1.8 不实现（显式）

`ahLst`（AdjustHandleList，1114424）、`cxnLst`（ConnectionSiteList，1114425）、
quadBezTo/cubicBezTo/arcTo、`AdjustValueListHanlder` 之外的 guide 种类——MVP「能画出」不需
编辑器手柄与连接点；贝塞尔段归 post-MVP（几何扩展）。

## 2. 核心代码与函数改写列表

| Java 类 | Rust 目标 | 关键方法/字段 |
|---|---|---|
| `PresetGeometryHandler` | `…/sppr/geometry/PresetGeometryHandler.rs` | `ctor(PresetGeometry)`、`startElement`（prst）、`enterSubElement`（avLst）、`endElement`（write avLst） |
| `CustomGeometryHandler`（最小子集） | `…/sppr/geometry/CustomGeometryHandler.rs` | `ctor(CustomGeometry)`、`enterSubElement`（avLst/gdLst/rect/pathLst）、`endElement`（判空 write）；nested `RectHandler`、`PathListHandler`→`PathHandler`→`PathSegmentPointHandler` |
| `AdjustValueListHanlder` + `GeomGuideHandler` | `…/sppr/geometry/AdjustValueListHanlder.rs`（nested 可合入） | gd 循环复用；fmla/name |
| `AdjustPoint2DHandler` | `…/sppr/geometry/AdjustPoint2DHandler.rs` | x/y → `processAdjCoordinateValue` 双分支 |

id 已查证（`dml/A.java`）：prst=1114293、avLst=1114422、gdLst=1114423、rect=1114426、pathLst=1114427、
gd=1114392、name=1114136、fmla=1114391、l=1114213、t=1114214、r=1114193、b=1114195、
path=1114340、moveTo=1114415、lnTo=1114416、close=1114414、pt=1114409、w=1114304、h=1114305、
fill=1114380、stroke=1114420、extrusionOk=1114421、x=1114161、y=1114162（x/y 亦见 pml 3211405/3211406）。

函数改写列表：

- `PresetGeometryHandler`：prst 单 attr + avLst 写入。
- `CustomGeometryHandler` 最小集：avLst/gdLst 共用 `AdjustValueListHanlder`；`RectHandler` 四边
  processAdjCoordinate；`PathHandler` 段分发只实现 close/moveTo/lnTo，其余段 id 返回 None 并标注
  延后（不静默吞段）。
- `AdjustPoint2DHandler`：x/y 数字/guide 名双分支（`parse::<i32>()` ok → putCoordinate，err →
  putGeomGuideName）。
- `todo!("DEFERRED_NON_FUNCTION")` 停车项：ahLst、cxnLst、贝塞尔/arcTo 段、`AdjustValueListHanlder`
  的 guide 变种。

## 3. 测试方案

### 3.1 代码审计与测试面

- `PresetGeometryHandler.java`/`CustomGeometryHandler.java` 的段分支与 attrs；`GeomGuide.java`、
  `AdjCoordinate.java` 语义；对照 Rust `data/geometry/*.rs`。
- `DGMap.presetShapeTypeMap` 常用 key（rect/ellipse/roundRect 等）作为期望值来源。
- Rust 现状：129/1191 行壳；不增量。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | prstGeom prst id + avLst gd(fmla/name) | 数据 | 是 | T01 |
| C02 | custGeom avLst/gdLst/rect（l/t/r/b 数字与 guide 名混合） | 分支 | 是 | T02 |
| C03 | pathLst：close/moveTo/lnTo 段序 + pt 坐标 | 循环/数据 | 是 | T03 |
| C04 | path attrs（w/h/fill/stroke/extrusionOk） | 数据 | 是 | T04 |
| C05 | 延后段（quadBezTo 等）→ None 且不吞 | 失败/边界 | 是 | T05 |

### 3.3 可测试性

内存 attrs/events；真实 `data/geometry` 模型；多段 path fixture 顺序断言。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | prstGeom(prst=“roundRect”) + avLst 一条 gd | prst id 与 Java 一致；gd fmla/name 写入 | 红测停在本 function |
| T02 | C02 | rect 四边：l=“0”、r=“21600”、t=“g1”、b=“5” | 数字→putCoordinate；字符串→putGeomGuideName | 红测停在本 function |
| T03 | C03 | pathLst：[moveTo(pt 0,0), lnTo(pt 21600,0), close] | 段序与坐标写入 `pathSegmentLst` | 红测停在本 function |
| T04 | C04 | path attrs w/h/fill/stroke/extrusionOk | 四个字段写入；fill map miss 不写 | 红测停在本 function |
| T05 | C05 | path 下 quadBezTo/cubicBezTo/arcTo | 返回 None；段序中不产生错位；夹具提示延后 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.rs（+CustomGeometryHandler.rs）
test:  libs/ppt-core/tests/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/04_function_geometry.rs
[[test]] name = "04_function_geometry"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_geometry

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_geometry \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/CustomGeometryHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustValueListHanlder.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/GeomGuideHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustPoint2DHandler.rs \
  --report libs/ppt-core/parser/features/pp-10-dml-sppr/functions/04_function_geometry/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_geometry --json`)
date: 2026-09-01 11:43

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustPoint2DHandler.rs` | 27/35 | 77.1% | 37/51 | 72.5% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustValueListHanlder.rs` | 11/17 | 64.7% | 15/23 | 65.2% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/CustomGeometryHandler.rs` | 131/286 | 45.8% | 175/363 | 48.2% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/GeomGuideHandler.rs` | 18/25 | 72.0% | 24/34 | 70.6% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.rs` | 28/34 | 82.4% | 37/44 | 84.1% |
| **function total** | **215/397** | **54.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/CustomGeometryHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/CustomGeometryHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustValueListHanlder.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustValueListHanlder.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/GeomGuideHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/GeomGuideHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustPoint2DHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustPoint2DHandler.java
```

Loop 结果：PASS（2026-09-01）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/CustomGeometryHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/CustomGeometryHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustValueListHanlder.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustValueListHanlder.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/GeomGuideHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/GeomGuideHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustPoint2DHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustPoint2DHandler.rs \
  --report libs/ppt-core/parser/features/pp-10-dml-sppr/functions/04_function_geometry/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 03:43

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PresetGeometryHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/PresetGeometryHandler.rs` | **PASS** | `PresetGeometryHandler`, `endElement`, `startElement`, `enterSubElement` | — | — | **PASS** |
| `CustomGeometryHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/CustomGeometryHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/CustomGeometryHandler.rs` | **PASS** | `CustomGeometryHandler`, `endElement`, `enterSubElement`, `processAdjCoordinateValue` | `processAdjAngleValue` | — | **PASS** |
| `AdjustValueListHanlder` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustValueListHanlder.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustValueListHanlder.rs` | **PASS** | `AdjustValueListHanlder`, `enterSubElement` | — | — | **PASS** |
| `GeomGuideHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/GeomGuideHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/GeomGuideHandler.rs` | **PASS** | `GeomGuideHandler`, `init`, `startElement` | — | — | **PASS** |
| `AdjustPoint2DHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustPoint2DHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/geometry/AdjustPoint2DHandler.rs` | **PASS** | `AdjustPoint2DHandler`, `init`, `startElement`, `processAdjCoordinateValue` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->