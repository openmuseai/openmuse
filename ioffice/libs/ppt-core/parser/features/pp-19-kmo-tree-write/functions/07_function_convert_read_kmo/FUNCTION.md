# 07_function_convert_read_kmo — ConvertTool 改读真实 Kmo / ShapePr

> feature: pp-19-kmo-tree-write ｜ 状态：**pass** ｜ Java 管线：S5 消费 S4（不重做 convert 公式）

## 0. 管线能力解锁

实现后：pl-09 热路径的**类型**与 Java 一致。今日 `ConvertTool.rs:69,149` 等 `use layout_shape::{ShapePr, KmoShowShape, GeomObj as PresetGeometry}`。

**本 function 不改** GeomtryConvert 路径指令语义、FillConvert 六类、LineStyleConvert min 12700——那些已在 pl-09 pass。只切 host 类型。必须走 **function-java2rust 修模板 import**，禁止手工删 `// java` 改别名。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `show::app::shape::KmoShowShape` + `data::shape::ShapePr`（xfrm/geom/fill/ln） |
| **输出** | 与 pl-09 相同的公开 `moffice.drawing.Shape` |
| **门闩** | `rg "layout_shape::KmoShowShape" libs/ppt-core/src/cn/wps/show/convert` 对 ConvertTool/GeomtryConvert/FillConvert/LineStyleConvert **零命中**（j2r_hosts 过渡期允许，但 07 pass 前必须清 Convert 四文件） |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| Convert 热路径 | pl-09 | getShape/dispatch/xfrm/geom/fill/ln | ✅ | effect/3D | 禁止改 preset 公式冒充 host 切换 |
| 真实 Kmo/ShapePr | 01–06 | type/shapePr/txBody | ✅ | follow | 禁止 07 先于 05 改签名 |
| `layout_shape` | fixture | 旧测试 | ✅ | 删除文件 | 禁止本波删 layout_shape（pl-03 测试仍用） |

## 2. 核心代码与函数改写列表

- **本 function：** 四 convert 文件 + `j2r_hosts.rs` 中 ShapePr/KmoShowShape 再导出；`ShowTypoShape.getShape` 入参类型随 ConvertTool。
- **停车：** effectLst/3D 仍 DEFERRED。
- **禁改：** `preset_path_in_rect`；Vertices 表项；`// java` 行。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`ConvertTool.rs` layout_shape imports；pl-09 `04_function_geom_path.rs` fixture `KmoShowShape { shape_type, sp_pr, .. }`；`getShape` 已调用 ConvertTool。

### 3.2 覆盖矩阵

| coverage_id | 场景 | 必测? | test_id |
|-------------|------|-------|---------|
| CV-NOW | 今日 layout_shape 仍能 convert type1（回归 pl-09） | 是 | CV0 |
| CV-HOST | convert 四文件无 `layout_shape::KmoShowShape` | 是 | CV1 |
| CV-PRST | 真 ShapePr prst=1 → 公开 shape 非空 | 是 | CV2 |
| CV-LN | 真 Outline 宽度进入 LineProperty | 是 | CV3 |

### 3.4 用例表

| id | 期望 | 失败语义 |
|----|------|----------|
| CV0 | pl-09 同 fixture convert Some | 07 过早改签名 |
| CV1 | 四 convert 文件无 `layout_shape::KmoShowShape` | host 未切 |
| CV2 | 真 ShapePr prst=1 → 公开 shape 名为 rect | 仍读 layout GeomObj |
| CV3 | 真 Outline 宽度 → LineProperty Some | LineStyleConvert 未吃 data Outline |

### 3.5 测试代码位置

test: `tests/cn/wps/show/convert/07_function_convert_read_kmo.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 07_function_convert_read_kmo -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 07_function_convert_read_kmo \
  --src src/cn/wps/show/convert/ConvertTool.rs \
  --src src/cn/wps/show/convert/GeomtryConvert.rs \
  --src src/cn/wps/show/convert/FillConvert.rs \
  --src src/cn/wps/show/convert/LineStyleConvert.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/07_function_convert_read_kmo/FUNCTION.md
```

loop L6 必须回归：

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --offline \
  --test 01_function_getshape_cache --test 02_function_type_dispatch \
  --test 03_function_xfrm_pos --test 04_function_geom_path \
  --test 05_function_fill_base --test 06_function_line_property \
  --test 07_function_shape_text_bind -- --test-threads=1
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 07_function_convert_read_kmo --json`)
date: 2026-09-12 16:45

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/convert/ConvertTool.rs` | 149/622 | 24.0% | 346/1257 | 27.5% |
| `src/cn/wps/show/convert/FillConvert.rs` | 0/89 | 0.0% | 0/113 | 0.0% |
| `src/cn/wps/show/convert/GeomtryConvert.rs` | 32/297 | 10.8% | 57/437 | 13.0% |
| `src/cn/wps/show/convert/LineStyleConvert.rs` | 37/243 | 15.2% | 55/194 | 28.4% |
| **function total** | **218/1251** | **17.4%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

L5b **PASS**（ConvertTool / FillConvert / LineStyleConvert / GeomtryConvert / KmoShowShape）。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/convert/ConvertTool.rs \
  --java koffice/presentation/core/cn/wps/show/convert/ConvertTool.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/convert/FillConvert.rs \
  --java koffice/presentation/core/cn/wps/show/convert/FillConvert.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/convert/LineStyleConvert.rs \
  --java koffice/presentation/core/cn/wps/show/convert/LineStyleConvert.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/convert/GeomtryConvert.rs \
  --java koffice/presentation/core/cn/wps/show/convert/GeomtryConvert.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/app/shape/KmoShowShape.rs \
  --java koffice/presentation/core/cn/wps/show/app/shape/KmoShowShape.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/convert/ConvertTool.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/ConvertTool.rs \
  --java koffice/presentation/core/cn/wps/show/convert/FillConvert.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/FillConvert.rs \
  --java koffice/presentation/core/cn/wps/show/convert/LineStyleConvert.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/LineStyleConvert.rs \
  --java koffice/presentation/core/cn/wps/show/convert/GeomtryConvert.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/GeomtryConvert.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/07_function_convert_read_kmo/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-12 08:45

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ConvertTool` | `koffice/presentation/core/cn/wps/show/convert/ConvertTool.java` | `libs/ppt-core/src/cn/wps/show/convert/ConvertTool.rs` | **PASS** | `convertShapeToPublicShape`, `shapeInkToPublicShape`, `shapePropertiesToPublicShape`, `hasShape3D`, `getOverlayFill`, `groupShapePropertiesToPublicShape`, `shapeStyleToPublicShape`, `connectionShapeToPublicShape`, `groupShapeToPublicShape`, `pictureShapeToPublicShape`, `shapeToPublicShape`, `shapeToGraphicFrameShape`, `filterFill` | `hasTransformTextIn3D`, `isIgnoreExtrusionShape`, `mergeColorForAlpha`, `getGradMainColor`, `readExtrusionClr`, `readExtrusionPos`, `readMeterial`, `readScene3dCamera`, `read3dRotation`, `readScene3dLightRig`, `convertSp3d`, `adjustShapePropertyFor3d`, `checkFlatTx`, `isLegacy3D`, `convertGlow`, `convertSoftEdge`, `convertReflection`, `convertOuterShadow`, `convertInnerShadow`, `convertPresetShadow`, `lightenColor`, `transBlendMode`, `cellToPublicShape`, `backgroudToPublicShape`, `shapeToOleObjectShape`, `shapeToModel3DShape`, `getClipPic`, `saveBitmap`, `getAllRect`, `getClipRect`, `getGraphicsPaths`, `getPath`, `convertPath`, `getPaths`, `getShapePath` | — | **PASS** |
| `FillConvert` | `koffice/presentation/core/cn/wps/show/convert/FillConvert.java` | `libs/ppt-core/src/cn/wps/show/convert/FillConvert.rs` | **PASS** | `convertToFillBase`, `convertToSolidFill`, `convertToBlipFill`, `convertToGradFill`, `convertToPatternFill`, `getAlphaColorTransform` | `getFillFlip`, `getFillAlignment`, `getPatternPrst` | — | **PASS** |
| `LineStyleConvert` | `koffice/presentation/core/cn/wps/show/convert/LineStyleConvert.java` | `libs/ppt-core/src/cn/wps/show/convert/LineStyleConvert.rs` | **PASS** | `convertToLineProperty`, `convetToArrow` | — | — | **PASS** |
| `GeomtryConvert` | `koffice/presentation/core/cn/wps/show/convert/GeomtryConvert.java` | `libs/ppt-core/src/cn/wps/show/convert/GeomtryConvert.rs` | **PASS** | `convertDataToPublicShape`, `convertPlaceHolderToPublicShape`, `convertBackgroundToPublicShape`, `covertToPublicRectShape`, `convertPath` | `revisePath`, `setCustGemoRect`, `setGemoryFormula`, `setGemoryAdjust`, `setConnectionSite` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
