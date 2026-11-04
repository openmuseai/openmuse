# 02_function_type_dispatch — type 0–4 分派 + setShapeID

> feature: pl-09-public-shape-convert ｜ 状态：**pass** ｜ Java 管线：S5 步骤 5.2

## 0. 管线能力解锁

实现后：任意 `KmoShowShape.type()` 进入 convert 都得到带 `shapeID` 的公开 Shape 壳（或 type 未知时 null）。01 的 miss 路径只调这一门面，不自己 switch。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `KmoShowShape` + `InterpretCtx` |
| **输出** | `Shape` 或 None（Java `null`）。非空则 `getShapeID()==drawable.id()` |
| **状态** | 无全局状态；内部调用 03–07 / 组/图专用路径 |
| **错误** | type 非 0–4 → null，不 panic。id 默认 0 合法 |

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `ConvertTool.convertShapeToPublicShape` | `drawable.type()` 0=group / 1=sp / 2=pic / 3=cxnSp / 4=graphicFrame | `ConvertTool.java:100-125` |

### 1.3 输出
| type | 调用 | 本 function 断言 |
|------|------|------------------|
| 0 | `groupShapeToPublicShape` | 壳 + GRF.isGroup（组 childOff 框可最小）；不递归子 convert |
| 1 | `shapeToPublicShape` | 走 03–07 |
| 2 | `pictureShapeToPublicShape` | 走 03–06；blip 由 05 填 |
| 3 | `connectionShapeToPublicShape` | 走 03–06 |
| 4 | `shapeToGraphicFrameShape` | 壳 + xfrm/fill 可走 03/05；Chart/OLE **空** |
| 其它 | 不进 switch | null，不 setShapeID |

Java `:120-122`：仅 `shape!=null` 时 `setShapeID`。`:1201` picture 路径内部也会 set 一次——幂等。

### 1.4 上游
pp-10 `shape_type` 已写入；03–07 叶子（type 1 热路径）。

### 1.5 下游
01 miss；调用方不得再 switch type。

### 1.6 不变量
1. 成功路径 `getShapeID()==drawable.id()`。
2. 不在本层算 path/fill（委托 03–06）。
3. type 4 不 panic、不假装 Chart。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `KmoShowShape.type/id` | pp-02 / layout_shape | `shape_type` / `id` | ✅ host 字段 | 真 `app/shape` 类 | 禁止改 spid 编码 |
| `ConvertTool.convertShapeToPublicShape` | 本 function | switch + setShapeID | ❌ 空 stub 未进默认 mod | 1671 行其余方法 | 禁止瘦删 Java 方法注释 |
| `drawing::Shape` | core_libs_drawing | `Shape_new` / `setShapeID` | ✅ | — | 禁止本地再造 Shape |
| `shapeToPublicShape` 装配 | 03–07 | pos/geom/fill/ln/text | ❌ | ink | 禁止在 02 复制 03 公式 |
| `SlideCache.getDShape` | Java 组/pic 双视图 | dg `load(drawingShape)` | ❌ | 双 bin | MVP 可从 layout_shape 字段读，不伪造 dg Shape.rs |

## 2. 核心代码与函数改写列表

- **本 function：** `convertShapeToPublicShape`；`setShapeID`；type 0/1/2/3/4 分派函数**签名保留**。
- **type 1 体内调用：** 03–07（本 function 测 ID/分派，装配断言在 03–07）。
- **停车：** `shapeInkToPublicShape`、`shapeToOleObjectShape`、`shapeToModel3DShape`、`cellToPublicShape`、clip/path 工具方法。
- **禁改：** `PptxrSlide.rs`。

`ConvertTool.rs` 现状不是交错模板——loop **必须** function-java2rust 重生，禁止在空 stub 上堆手写。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`ConvertTool.java:100-125` 五路 switch；`:1156-1197` group（GRF.setGroup、childOff→733）；`:1199+` picture；`:1143` cxn；`:1265` graphicFrame；`:1239` sp+ShapeText。Rust `ConvertTool.rs` 全 `todo!()` 且 `convert/mod.rs` 只有 `PresetShapeTypeMap`；`show/mod.rs` `pub mod convert` 在 `feature="full"`。

| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| convert type1 | :107 | id=5 type=1 | Shape id=5 | 热路径 |
| convert type2/3 | :111/:113 | pic/cxn | Shape id 透传 | 分派 |
| convert type0 | :104 | group | Shape；不要求子已 convert | 组壳 |
| convert type4 | :116 | graphicFrame | 非 None 壳 | 不 Chart |
| convert type99 | default | — | None | 漏 setID |
| drawing Shape ID | Shape.rs:862 | setShapeID(9) | getShapeID=9 | 产物类型存在 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| TD-SP | type==1 | 壳+ID | 是 | TD1 |
| TD-PIC | type==2 | 壳+ID | 是 | TD2 |
| TD-CXN | type==3 | 壳+ID | 是 | TD3 |
| TD-GRP | type==0 | 组壳 | 是 | TD4 |
| TD-GF | type==4 | 空壳非 panic | 是 | TD5 |
| TD-UNK | type==99 | None | 是 | TD6 |
| TD-ID | setShapeID | 与 drawable.id | 是 | TD7 |
| TD-OUT | drawing Shape | 产物类型可 set/get ID | 是 | TD8 |

### 3.3 可测试性
喂 `KmoShowShape { shape_type, id }`。不跑 zip。TD8 可在 Split 绿（drawing crate）。TD1–TD7 `#[ignore = "spec: ConvertTool 未进默认编译面"]`。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| TD1 | TD-SP | — | type=1 id=5 | Some，getShapeID=5 | 走错 group 分支 |
| TD2 | TD-PIC | — | type=2 | Some + id | 当 sp 丢 blip 槽 |
| TD3 | TD-CXN | — | type=3 | Some + id | panic dg load |
| TD4 | TD-GRP | children>0 | type=0 | Some；不要求 children 已 convert | 递归爆栈 |
| TD5 | TD-GF | type=4 | — | Some 壳 | Chart todo panic |
| TD6 | TD-UNK | type=99 | — | None | 空 Shape |
| TD7 | TD-ID | 与 TD1 | id=5 | convert 内外两次 set 仍为 5 | 被覆盖成 -1 |
| TD8 | TD-OUT | — | `Shape_new(None); setShapeID(9)` | getShapeID=9 | drawing 产物不可用 |

### 3.5 位置
```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/convert/ConvertTool.rs
test:  tests/cn/wps/show/convert/02_function_type_dispatch.rs
```

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_type_dispatch

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_type_dispatch \
  --src src/cn/wps/show/convert/ConvertTool.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/02_function_type_dispatch/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_type_dispatch --json`)
date: 2026-09-10 18:57

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/convert/ConvertTool.rs` | 237/594 | 39.9% | 568/1180 | 48.1% |
| **function total** | **237/594** | **39.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/convert/ConvertTool.rs \
  --java koffice/presentation/core/cn/wps/show/convert/ConvertTool.java
```
ConvertTool.java >700 行：loop 仍一次改完本 function 列表内方法；列表外 DEFERRED 停车。禁止 `--force` `PptxrSlide.rs`。

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/convert/ConvertTool.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/ConvertTool.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/02_function_type_dispatch/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-10 10:57

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ConvertTool` | `../../koffice/presentation/core/cn/wps/show/convert/ConvertTool.java` | `src/cn/wps/show/convert/ConvertTool.rs` | **PASS** | `convertShapeToPublicShape`, `shapeInkToPublicShape`, `shapePropertiesToPublicShape`, `hasShape3D`, `getOverlayFill`, `groupShapePropertiesToPublicShape`, `shapeStyleToPublicShape`, `connectionShapeToPublicShape`, `groupShapeToPublicShape`, `pictureShapeToPublicShape`, `shapeToPublicShape`, `shapeToGraphicFrameShape`, `filterFill` | `hasTransformTextIn3D`, `isIgnoreExtrusionShape`, `mergeColorForAlpha`, `getGradMainColor`, `readExtrusionClr`, `readExtrusionPos`, `readMeterial`, `readScene3dCamera`, `read3dRotation`, `readScene3dLightRig`, `convertSp3d`, `adjustShapePropertyFor3d`, `checkFlatTx`, `isLegacy3D`, `convertGlow`, `convertSoftEdge`, `convertReflection`, `convertOuterShadow`, `convertInnerShadow`, `convertPresetShadow`, `lightenColor`, `transBlendMode`, `cellToPublicShape`, `backgroudToPublicShape`, `shapeToOleObjectShape`, `shapeToModel3DShape`, `getClipPic`, `saveBitmap`, `getAllRect`, `getClipRect`, `getGraphicsPaths`, `getPath`, `convertPath`, `getPaths`, `getShapePath` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
