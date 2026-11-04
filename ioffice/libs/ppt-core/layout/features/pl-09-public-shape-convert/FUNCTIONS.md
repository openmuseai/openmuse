# pl-09-public-shape-convert — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)  
> 切在 **I/O**，不切 `ConvertTool` 整类。Java S5 步骤 5.1–5.6。状态全部 **pass**。

## §0 管线位置

```text
S4 Kmo 树（pp-10 ShapePr / pp-11 txBody / pl-03 follow 已折入）
  → S8 快照（pl-06 ShowTypoShape）—— 本层不重做 flags/rect 快照
      │
      └─[S5 convert，惰性，getShape 首次]
           01 getshape_cache     SlideCache hit? miss→convert+addShape
           02 type_dispatch      type 0–4 + setShapeID
           03 xfrm_pos           toDegree + flip + property(733) px + calcRelativeRect
           04 geom_path          prst/cust/ph-default → Geometry/path
           05 fill_base          fill 六类 / useBgFill → FillBase
           06 line_property      ln → LineProperty（min 12700 EMU；type 5 → null）
           07 shape_text_bind    KmoTextBody → ShapeText 挂到公开 Shape
                    │
         pr-02 / pr-03 / 命中 / 测量  ◄── 同一份公开 Shape
```

Java 对照：`ShowTypoShape.getShape:180` → `ConvertTool.convertShapeToPublicShape:100` → `shapePropertiesToPublicShape:140` → `GeomtryConvert` / `FillConvert` / `LineStyleConvert` → `shapeToPublicShape:1247` ShapeText。

S6（pl-02 `calcRelativeRect`）在 Java **被 S5 内部调用**（`:170`），不是 S5 之后才算框。本 feature 03 **只调用** pl-02，不重写算法。S8 快照（pl-06）在 Java 文档顺序上画在 S5 之后，但 `getShape` 活在快照对象上——Rust 已 pass 的 pl-06 继续拥有 flags/rect；**convert 从 pl-06 的 `DEFERRED getShape` 接手**。

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_getshape_cache](./functions/01_function_getshape_cache/FUNCTION.md) | 惰性缓存入口 | ShowTypoShape + SlideCache + filter | 公开 Shape（命中即返 / miss convert） | pl-06; 02 | pr-02; 命中 | pass |
| 2 | [02_function_type_dispatch](./functions/02_function_type_dispatch/FUNCTION.md) | type 0–4 分派 | KmoShowShape.type + InterpretCtx | 公开 Shape 壳 + shapeID | pp-10; 03–07 | 01 | pass |
| 3 | [03_function_xfrm_pos](./functions/03_function_xfrm_pos/FUNCTION.md) | 坐标/角归一化 | ShapeInfo + xfrm | rotation° / flip / property(733) px | pl-01; pl-02 | 04; pr-02 | pass |
| 4 | [04_function_geom_path](./functions/04_function_geom_path/FUNCTION.md) | 几何→path | prstGeom/custGeom/ph | Geometry + GraphicsPath | PresetShapeTypeMap; 03 | pr-02 | pass |
| 5 | [05_function_fill_base](./functions/05_function_fill_base/FUNCTION.md) | fill→FillBase | Fill + useBgFill + theme | FillBase 挂 Shape | pp-10; pp-16 | pr-02 | pass |
| 6 | [06_function_line_property](./functions/06_function_line_property/FUNCTION.md) | ln→LineProperty | Outline | LineProperty 或 null | pp-10; 05 | pr-02 | pass |
| 7 | [07_function_shape_text_bind](./functions/07_function_shape_text_bind/FUNCTION.md) | 文本挂到 Shape | 公开 Shape + KmoTextBody | ShapeText | pp-11; 02 | pr-03; pl-04 | pass |

## §2 数据流

```text
getShape (01):
  mShape != null → 返回
  noFilter = !ctx.filter.hasFilter()
  cached = noFilter ? SlideCache.getShape(base) : null
  if cached == null:
      mShape = convertShapeToPublicShape(base, ctx)   // 02
      mShape.setShapeID(base.id())
      if noFilter: SlideCache.addShape(base, mShape)
  return mShape

convertShapeToPublicShape (02):
  switch type:
    0 groupShapeToPublicShape
    1 shapeToPublicShape      → style + 03–06 + ink(park) + 07
    2 pictureShapeToPublicShape
    3 connectionShapeToPublicShape
    4 shapeToGraphicFrameShape  // 壳
  if shape != null: setShapeID

shapePropertiesToPublicShape (03–06):
  ShapeInfoCache.shapeInfo → rotation/flip
  calcRelativeRect → anchor WxH（04 用）
  fromEmuX/Y → property(733) + setShapeRect
  geom (04) → fill (05) → ln (06)
  effect/3D → DEFERRED
```

无环：01 编排缓存；02 分派；03 只写 pos；04 只写 geometry；05 只写 fill；06 只写 line；07 只挂文本。测 N 只用声明的上游 fixture，不跑 `render_slide`。

## §3 测试面

crate：`libs/ppt-core`。fixture `KmoShowShape`（`layout_shape` host）+ 已落地的 `UnitsConverter` / `ShapeUtil::toDegree` / `drawing::Shape`。不跑 Flutter、不跑 23 页 Sample 作为本 feature 唯一用例。

| function_id | tests path | cargo |
|---|---|---|
| 01 | `tests/cn/wps/show/render/01_function_getshape_cache.rs` | `--test 01_function_getshape_cache` |
| 02 | `tests/cn/wps/show/convert/02_function_type_dispatch.rs` | `--test 02_function_type_dispatch` |
| 03 | `tests/cn/wps/show/convert/03_function_xfrm_pos.rs` | `--test 03_function_xfrm_pos` |
| 04 | `tests/cn/wps/show/convert/04_function_geom_path.rs` | `--test 04_function_geom_path` |
| 05 | `tests/cn/wps/show/convert/05_function_fill_base.rs` | `--test 05_function_fill_base` |
| 06 | `tests/cn/wps/show/convert/06_function_line_property.rs` | `--test 06_function_line_property` |
| 07 | `tests/cn/wps/show/render/07_function_shape_text_bind.rs` | `--test 07_function_shape_text_bind` |

默认 feature 下 `show/convert` 与 `show/radapter` 均 `cfg(feature="full")`，`ConvertTool.rs` **未进 mod**。Split 测试：已落地 helper 可绿；convert I/O 用 `#[ignore = "spec: …"]` 调 `ShowTypoShape.getShape`（现 `todo!("DEFERRED_NON_FUNCTION")`）。**禁止** `assert!(true)`。Loop 把 ConvertTool 接到默认编译面后再去掉 ignore。

## §4 串联

01 是唯一编排入口（对应 Java `getShape`）。02 是 convert 门面。03–07 是 `shapePropertiesToPublicShape` / `shapeToPublicShape` 的 I/O 切片。pr-02 今日从 `layout_shape` 读 path/fill；本 feature pass 后的消费契约：

```text
typo.getShape()  →  Shape.getGraphicsPaths(w,h) / getFill() / getLineProperty()
```

不要在 pl-09 里改 `rendercommand.rs` 发射逻辑。

## §5 验收顺序（loop）

```text
01_function_getshape_cache
02_function_type_dispatch
03_function_xfrm_pos
04_function_geom_path
05_function_fill_base
06_function_line_property
07_function_shape_text_bind
```

01 在 02 实现前，ignore 用例会因 `getShape` 仍 DEFERRED 而保持 ignore；loop 应先 02–06 叶子，再解除 01 的 miss 路径。入口 Java 行数不暂停。

## §6 类级对照

| 层 | Java | Rust（现状 → loop） |
|---|---|---|
| 入口 | `ShowTypoShape.getShape:180` | `ShowTypoShape.rs:361` 现 `DEFERRED_NON_FUNCTION` → 01 实现 |
| 缓存 | `SlideCache.java:78/83` | **无文件**；loop 在 `render/cache/SlideCache.rs` 最小 MRU（id→Shape），禁止项目 stub |
| 分派/装配 | `ConvertTool.java` 1671 行 | `convert/ConvertTool.rs` **脱离模板的空 stub** 且未进默认 `mod` → function-java2rust 重生交错模板 |
| 几何 | `GeomtryConvert.java` + `PresetShapeTypeMap.java` + `Vertices.java` | `PresetShapeTypeMap.rs` 已有（cfg full）；`GeomtryConvert.rs` 空 stub 未进 mod；Vertices 缺失 |
| 填充/线 | `FillConvert.java` / `LineStyleConvert.java` | **无 .rs 进编译面** |
| 产物 | `cn.wps.moffice.drawing.Shape` | `core_libs_drawing` `moffice/drawing/Shape.rs` 已有；`ShowTypoShape.rs` 本地空 `struct Shape {}` 必须换成 drawing Shape |
| 文本 | `ShapeText(publicShape, textBody)` | `render/ShapeText.rs` 现 `cfg(feature="full")` |

## 文档

- [functions.csv](./functions.csv)
- [FEATURE.md](./FEATURE.md)
