# 03_function_xfrm_pos — xfrm 归一化到 ShapePos / property(733)

> feature: pl-09-public-shape-convert ｜ 状态：**pass** ｜ Java 管线：S5 步骤 5.3 前半

## 0. 管线能力解锁

实现后：公开 Shape 带 **degree 旋转**、**GRF flip**、**render 单位绝对框**（property 733 + `pos.setShapeRect`）。渲染/命中不再读 EMU xfrm 或 `rotation/60000`。

`calcRelativeRect` 在 Java convert **内部**调用（`:170`）——组嵌套绝对框在转换时刻冻结。本层 **只调用** pl-02，不重写矩阵。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `Shape` 目标 + `KmoShowShape`（transform2D* / rotation / flip）+ `shapePr()!=null` |
| **输出** | 写回同一 `Shape`：`ShapePos.rotation = toDegree(shapeInfo.rotation)`；`GRF.flipH/V`；`property(733)=RectF(fromEmuX/Y(x,y,x+cx,y+cy))`；`pos.setShapeRect(同一 RectF)` |
| **副作用** | 调用 `ShapeInfoCache.shapeInfo`（读 transform，含 ph 折入后的有效 xfrm）；`ShapeUtil.calcRelativeRect(drawable, rcAnchor, false)` 得到 04 用的 anchorWidth/Height |
| **错误** | `shapePr==null` → Java 整段跳过（`:142`），不写 pos。Rust 同样 no-op，不 panic |

### 1.2 / 1.3 输入输出

Java `shapePropertiesToPublicShape:151-177`：

- `pos.setRotation(ShapeUtil.toDegree(shapeInfo.rotation))` — `toDegree = rotation/60000.0F`（`ShapeUtil.java` / Rust `:811-814` 已 pass）
- `grf.setFlipH(shapeInfo.isFlipHorz)` / `setFlipV`
- `rcAnchor` 先用 transform2D 局部框，再 `calcRelativeRect(..., false)`
- `coordBounds` 用 **shapeInfo 的 x,y,cx,cy** 做 `fromEmuX/Y`（注意：733 用的是 shapeInfo 原值，不是 calcRelativeRect 后的 rcAnchor——`:164-176` 实测）。anchorWidth/Height 来自 **rcAnchor**（给 geom）。

### 1.4 上游
pl-01 `UnitsConverter.render()`；pl-02 `calcRelativeRect` / `toDegree`；pl-03 有效 xfrm 已在 shape 上。

### 1.5 下游
04 用 anchor WxH；pr-02 / 命中用 733 框。

### 1.6 不变量
1. `toDegree(60000)==1.0`，`toDegree(0)==0`。
2. 733 四边 = `fromEmuX/Y` 作用在 shapeInfo 的 x/y/x+cx/y+cy。
3. 不在本层写 fill/line/path。
4. 禁止把 733 写成 EMU。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `ShapeUtil.toDegree` | pl-02 | `/60000` | ✅ `:812` | — | 禁止改系数 |
| `ShapeUtil.calcRelativeRect` | pl-02 | 绝对 EMU 框 | ✅ 热路径 | 3D 矩阵 | 禁止复制算法 |
| `UnitsConverter.render().fromEmuX/Y` | pl-01 | EMU→px | ✅ | — | 禁止改 dpi |
| `ShapeInfoCache.shapeInfo` | S6 | 填 rotation/flip/transform2D* | ⚠️ layout_shape 有空 `ShapeInfo` | 真 MRU 128 | 本层可直读 `KmoShowShape` 字段作 MVP 等价（Java 也是从 shape 填 ShapeInfo） |
| `drawing ShapePos/GRF/setProperty` | drawing | 733 槽 | ⚠️ `setPropertyValue` 走 PropertyMap | 需确认 733 写入 API | 禁止另起 HashMap 当 Shape |
| `shapePropertiesToPublicShape` xfrm 段 | ConvertTool | :151-177 | ❌ | 后半 fill/ln | 只改 xfrm 段，后半 DEFERRED 或由 04–06 解 |

## 2. 核心代码与函数改写列表

- **本 function：** `shapePropertiesToPublicShape` 的 xfrm/pos/733 段；`toDegree` **只调用**。
- **停车：** 同方法内 geom/fill/ln/effect/3D（04–06 与 defer）。
- **禁改：** `ShapeUtil.rs` 几何算法；`PptxrSlide.rs`。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`ConvertTool.java:140-177`；`ShapeUtil.rs:811-814` `toDegree`；pl-06 测试 `fromEmuX/Y` 公式；733 用 shapeInfo 原 xfrm 而非 rcAnchor。

| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| toDegree | ShapeUtil | 60000 / 0 / -60000 | 1 / 0 / -1 | 已落地 |
| fromEmu 733 | UnitsConverter | 框 (0,0,1219200,685800) | 与手算相等 | 单位 |
| shapePr null | :142 | sp_pr 缺 | 不写 rotation | early return |
| flip | shapeInfo | H/V true | GRF 同 | 翻转 |
| calcRelativeRect | :170 | 组子 | anchorW/H 给 04 | 只调用 |
| 733 vs rcAnchor | :164 vs :170 | 有 parent | 733 仍用 shapeInfo xfrm | 混用即错 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| XP-DEG | toDegree | 0/60000/-60000 | 是 | XP1 |
| XP-733 | property(733) | 已知 EMU 框 | 是 | XP2 |
| XP-FLIP | GRF | flipH/V | 是 | XP3 |
| XP-NOPR | shapePr null | no-op | 是 | XP4 |
| XP-ROT | pos.rotation | rotation=10800000 → 180° | 是 | XP5 |
| XP-ANCHOR | calcRelativeRect | 子框给 geom 宽高 | 是 | XP6 |

### 3.3 可测试性
XP1 Split 可绿。XP2–XP6 需 convert 接线后解 ignore：`#[ignore = "spec: shapePropertiesToPublicShape xfrm 段未接线"]`。不跑 film。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| XP1 | XP-DEG | metrics | toDegree(60000) | 1.0 | 当弧度 |
| XP2 | XP-733 | x=0 y=0 cx=1219200 cy=685800 | 733 l/t/r/b | fromEmu 手算 | 仍 EMU |
| XP3 | XP-FLIP | flipH=true flipV=false | GRF | H true V false | 对调 |
| XP4 | XP-NOPR | has_fill 等缺且 shapePr 视为 null | rotation 保持默认 | 被写成 0 冒充 | |
| XP5 | XP-ROT | rotation=10800000 | 180.0 | /60000 漏掉 | |
| XP6 | XP-ANCHOR | 组内子 | geom 用 rcAnchor.w/h | 用 733 宽高替代 | |

### 3.5 位置
```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/convert/ConvertTool.rs
       src/cn/wps/show/util/ShapeUtil.rs（只调用 toDegree/calcRelativeRect）
test:  tests/cn/wps/show/convert/03_function_xfrm_pos.rs
```

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_xfrm_pos

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_xfrm_pos \
  --src src/cn/wps/show/convert/ConvertTool.rs \
  --src src/cn/wps/show/util/ShapeUtil.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/03_function_xfrm_pos/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_xfrm_pos --json`)
date: 2026-09-10 19:07

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/convert/ConvertTool.rs` | 132/601 | 22.0% | 301/1199 | 25.1% |
| `src/cn/wps/show/render/cache/ShapeInfo.rs` | 23/29 | 79.3% | 32/38 | 84.2% |
| `src/cn/wps/show/render/cache/ShapeInfoCache.rs` | 15/49 | 30.6% | 34/72 | 47.2% |
| **function total** | **170/679** | **25.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/convert/ConvertTool.rs \
  --java koffice/presentation/core/cn/wps/show/convert/ConvertTool.java
```

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/convert/ConvertTool.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/ConvertTool.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/03_function_xfrm_pos/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-10 11:07

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ConvertTool` | `koffice/presentation/core/cn/wps/show/convert/ConvertTool.java` | `libs/ppt-core/src/cn/wps/show/convert/ConvertTool.rs` | **PASS** | `convertShapeToPublicShape`, `shapeInkToPublicShape`, `shapePropertiesToPublicShape`, `hasShape3D`, `getOverlayFill`, `groupShapePropertiesToPublicShape`, `shapeStyleToPublicShape`, `connectionShapeToPublicShape`, `groupShapeToPublicShape`, `pictureShapeToPublicShape`, `shapeToPublicShape`, `shapeToGraphicFrameShape`, `filterFill` | `hasTransformTextIn3D`, `isIgnoreExtrusionShape`, `mergeColorForAlpha`, `getGradMainColor`, `readExtrusionClr`, `readExtrusionPos`, `readMeterial`, `readScene3dCamera`, `read3dRotation`, `readScene3dLightRig`, `convertSp3d`, `adjustShapePropertyFor3d`, `checkFlatTx`, `isLegacy3D`, `convertGlow`, `convertSoftEdge`, `convertReflection`, `convertOuterShadow`, `convertInnerShadow`, `convertPresetShadow`, `lightenColor`, `transBlendMode`, `cellToPublicShape`, `backgroudToPublicShape`, `shapeToOleObjectShape`, `shapeToModel3DShape`, `getClipPic`, `saveBitmap`, `getAllRect`, `getClipRect`, `getGraphicsPaths`, `getPath`, `convertPath`, `getPaths`, `getShapePath` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
