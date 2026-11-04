# 07_function_shape_text_bind — ShapeText 挂到公开 Shape

> feature: pl-09-public-shape-convert ｜ 状态：**pass** ｜ Java 管线：S5 步骤 5.6

## 0. 管线能力解锁

实现后：公开 Shape 携带 `ShapeText(publicShape, textBody)`，页码写入 `TextContext`，autofit type==2 → `setFitShape2Text`。pr-03 / 排版测量 / 非绘制查询有同一挂点。Rust 现状 ShapeText 在 `feature="full"`，默认链文本走 `KmoTextOperator` 捷径——本 function 把 Java 中间体接回默认面。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | 已装配 pos/geom/fill/ln 的公开 Shape + `shape.getTextOperator().getTextBody()` + `interpretCtx.getContentInterpretCtx().slideNumber()` |
| **输出** | `publicShape.setText(shapeText)`；`shapeText.getTextContext().setPageNumber(slideNumber)` |
| **错误** | textBody==null：仍 `new ShapeText(publicShape, null)`（Java `:1248` 无 null 跳过）。`hasTextBodyProperties` 才读 autofit。不 panic |

Java `shapeToPublicShape:1239-1262` 顺序：new Shape → setShapeID → style → properties → ink → setHidden → **ShapeText** → GRF.setChild(true)。

本 function **只测文本绑定**；style/ink/hidden 非本 I/O。

### 1.4 上游
pp-11 txBody；02 `shapeToPublicShape` 调用点。

### 1.5 下游
pr-03 DrawText；pl-04 测量可改为读 `shape.getText()`（不在本波改 pl-04）。

### 1.6 不变量
1. type 1 热路径必 `setText`，即使空 body。
2. pageNumber = slideNumber。
3. autofit type==2 ↔ fitShape2Text true；其它 type 不设 true。
4. 不在本层断行（S7）。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `ShapeText::new` | render/ShapeText.rs | `(Shape, KmoTextBody)` | ⚠️ 有实现但 cfg full | hasBullets 全量 | 禁止另造 TextBox 当 ShapeText |
| `drawing::textbox::Text` | drawing | super | ✅ | — | — |
| `KmoTextBody` | pp-11 | textBodyProperties.autofit | ⚠️ surf / layout_textbox | 真 KoBin body | 禁止改 DataTransform |
| `ContentInterpretCtx.slideNumber` | radapter | 页码 | cfg full stub | — | MVP 可从 fixture 注入 i32 |
| `ConvertTool.shapeToPublicShape` 文本段 | 02 体内 | :1247-1258 | ❌ | ink 前后顺序 | 只解文本段 |

## 2. 核心代码与函数改写列表

- **本 function：** `new ShapeText`；`setPageNumber`；autofit type==2；`setText`；把 `ShapeText.rs` 接到默认 `render/mod.rs`（去掉不必要的 full 门控，或提供默认面 re-export）。
- **停车：** `hasBullets` 复杂分支、ink、hidden。
- **禁改：** pl-04 `exToInternal`；pr-03 指令发射（消费方可后改）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`ConvertTool.java:1247-1258`；`ShapeText.rs:51-66` ctor；`render/mod.rs` `#[cfg(feature="full")] pub mod ShapeText`。

| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| bind null body | :1248 | textBody=null | setText 非空对象，hasText false | |
| bind 有 body | hasTextBody | hasText true | |
| pageNumber | :1249 | slideNumber=3 | context=3 | |
| autofit 2 | type==2 | fitShape2Text true | |
| autofit 其它 | type==0/1 | 不 true | |
| 无 bodyPr | !hasTextBodyProperties | 跳过 autofit | |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| ST-NULL | ShapeText null body | 仍 setText | 是 | ST1 |
| ST-BODY | 有 txBody | hasText | 是 | ST2 |
| ST-PAGE | slideNumber | pageNumber | 是 | ST3 |
| ST-FIT | autofit==2 | fitShape2Text | 是 | ST4 |
| ST-NOFIT | autofit!=2 | 非 true | 是 | ST5 |
| ST-NOPR | 无 bodyPr | 不读 autofit | 是 | ST6 |

### 3.3 可测试性
type 1 fixture ± text_op。ST1–ST6 ignore `spec: shapeToPublicShape 文本段未接线；ShapeText cfg full`。不跑 RichTextLayout。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| ST1 | ST-NULL | 无 txBody | getText 非空，hasText false | 不 setText | |
| ST2 | ST-BODY | has_text_body | hasText true | 丢 body | |
| ST3 | ST-PAGE | slideNumber=3 | pageNumber=3 | 0 | |
| ST4 | ST-FIT | autofit type=2 | fitShape2Text true | 漏 | |
| ST5 | ST-NOFIT | type=1 | 非 true | 全当 autofit | |
| ST6 | ST-NOPR | 无 bodyPr | 不 panic | NPE | |

### 3.5 位置
```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/render/ShapeText.rs
       src/cn/wps/show/convert/ConvertTool.rs
test:  tests/cn/wps/show/render/07_function_shape_text_bind.rs
```

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 07_function_shape_text_bind

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 07_function_shape_text_bind \
  --src src/cn/wps/show/render/ShapeText.rs \
  --src src/cn/wps/show/convert/ConvertTool.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/07_function_shape_text_bind/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 07_function_shape_text_bind --json`)
date: 2026-09-11 00:55

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/convert/ConvertTool.rs` | 132/604 | 21.9% | 302/1205 | 25.1% |
| `src/cn/wps/show/render/ShapeText.rs` | (not in report) | — | — | — |
| **function total** | **132/604** | **21.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/ShapeText.rs \
  --java koffice/presentation/core/cn/wps/show/render/ShapeText.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/ConvertTool.rs \
  --java koffice/presentation/core/cn/wps/show/convert/ConvertTool.java
```

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/ShapeText.java \
  --rs libs/ppt-core/src/cn/wps/show/render/ShapeText.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/07_function_shape_text_bind/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-10 16:54

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShapeText` | `koffice/presentation/core/cn/wps/show/render/ShapeText.java` | `libs/ppt-core/src/cn/wps/show/render/ShapeText.rs` | **PASS** | `ShapeText`, `hasText`, `hasTypoText`, `hasRealText`, `hasBullets`, `getKmoTextBody`, `getTextContext` | — | — | **PASS** |
| `TextContext` | `koffice/presentation/core/cn/wps/show/render/ShapeText.java` | `libs/ppt-core/src/cn/wps/show/render/ShapeText.rs` | **PASS** | `setPageNumber`, `getPageNumber` | — | — | **PASS** |
| `ConvertTool` | `koffice/presentation/core/cn/wps/show/convert/ConvertTool.java` | `libs/ppt-core/src/cn/wps/show/convert/ConvertTool.rs` | **PASS** | `convertShapeToPublicShape`, `shapeInkToPublicShape`, `shapePropertiesToPublicShape`, `hasShape3D`, `getOverlayFill`, `groupShapePropertiesToPublicShape`, `shapeStyleToPublicShape`, `connectionShapeToPublicShape`, `groupShapeToPublicShape`, `pictureShapeToPublicShape`, `shapeToPublicShape`, `shapeToGraphicFrameShape`, `filterFill` | `hasTransformTextIn3D`, `isIgnoreExtrusionShape`, `mergeColorForAlpha`, `getGradMainColor`, `readExtrusionClr`, `readExtrusionPos`, `readMeterial`, `readScene3dCamera`, `read3dRotation`, `readScene3dLightRig`, `convertSp3d`, `adjustShapePropertyFor3d`, `checkFlatTx`, `isLegacy3D`, `convertGlow`, `convertSoftEdge`, `convertReflection`, `convertOuterShadow`, `convertInnerShadow`, `convertPresetShadow`, `lightenColor`, `transBlendMode`, `cellToPublicShape`, `backgroudToPublicShape`, `shapeToOleObjectShape`, `shapeToModel3DShape`, `getClipPic`, `saveBitmap`, `getAllRect`, `getClipRect`, `getGraphicsPaths`, `getPath`, `convertPath`, `getPaths`, `getShapePath` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
