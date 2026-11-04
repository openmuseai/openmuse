# pp-10-dml-sppr — 形状属性 spPr

> status: **pass** ｜ depends_on: `pp-01-pptx-parse`; `pp-02-shape-tree-parse`
>
> Java 管线环节：**S0 parse（spPr → ShapePr）→ S1 convert 输入**。本 feature 是 S1 convert 的
> 输入侧：不排版、不转换、不渲染。

## §0 管线位置

```text
pp-01: token maps + HandlerCallback（pass）
  -> pp-02: p:spTree 事件 -> KmoShowShape/DgStore 行（pass；spPr 为 deferred）
       -> pp-10: p:spPr / p:grpSpPr 子树
            -> 01 spPr 分发 + xfrm（bwMode/Transform2D/子分发）
            -> 02 fill（fillType + solid/grad/patt/blip 最小集）
            -> 03 outline（w/cmpd/cap/algn + lineJoin + dash/end）
            -> 04 geometry（prstGeom + custGeom 最小子集）
            -> ShapePr 就绪
       -> place holder 合并（pl-03）/ convert 输入（pl-02）/ 形状指令（pr-02）
```

## §1 function 清单与依赖序

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_shape_properties` | `p:spPr`/`p:grpSpPr` attrs+子事件 + `ShapePr` | bwMode、`Transform2D`、fill/ln/geom/effect 子分发 | pp-02:02/03/04/05 | 02;03;04;pl-02-xfrm-geometry | pass |
| 2 | `02_function_fill` | solid/grad/patt/blip fill 子树事件 + `Fill` | fillType 六分支 + Solid/Gradient/Blip 最小集 | 01 | pl-03-ph-merge;pr-02-shape-command | pass |
| 3 | `03_function_outline` | `a:ln` 子树事件 + `Outline` | w/cmpd/cap/algn + lineJoin + 最小 dash/end | 01 | pr-02-shape-command | pass |
| 4 | `04_function_geometry` | geometry 子树事件 + `Geometry2D` | prstGeom prst id + avLst；custGeom rect/path | 01 | pr-02-shape-command | pass |

`02/03/04` 是 01 的三个并行子链；01 是唯一顶层分发。group 形态（`p:grpSpPr`）随 01 验收：
`GroupShapePropertiesHandler` 与 `SwGroupShapeHandler`（pp-02:05）的 group 路径共用契约。

## §2 数据流

```text
p:spPr(ShapePropertiesHandler)
  ├─ bwMode(1114443/3211605) ─────────────> ShapePr.putBwMode
  ├─ xfrm(3211589) ── Transform2DHandler ─> flipH/flipV/rot + ext(cx/cy) + off(x/y)
  ├─ ln(1114148) ──── OutlineHandler ─────> 02? 否：03 function
  ├─ noFill/solidFill/gradFill/pattFill/grpFill/blipFill ─> FillPropertiesHandler -> 02 function
  ├─ prstGeom/custGeom ───────────────────> 04 function
  └─ effectLst/effectDag / scene3d / sp3d ─> 延后（MVP 不做）
p:grpSpPr(GroupShapePropertiesHandler)
  ├─ fill 六分支（同 spPr）
  └─ xfrm(3211589) ── GroupTransform2DHandler（group 比例/旋转，供 pl-02）
```

写入目标模型：`data/shape/ShapePr.rs`（1300 行，已存在）、`data/basic/Transform2D.rs`、
`data/basic/Point2D.rs`、`data/basic/PositiveSize2D.rs`、`data/fill/*`、`data/outline/Outline.rs`、
`data/geometry/*`。

## §3 测试面

- crate：`libs/ppt-core/Cargo.toml`；测试镜像根 `libs/ppt-core/tests/cn/wps/drawing/dmlr/handler/shape/sppr/`。
- 每个 function 一个 `[[test]]` name（`01_function_shape_properties` / `02_function_fill` /
  `03_function_outline` / `04_function_geometry`），与 [functions.csv](./functions.csv) 对齐。
- 测试输入全部用内存 attrs/events（contract fixture），不打开 reader/part。
- **Rust 现状（审计表 `ppt_core_mvp_audit.csv`）**：sppr 族 handler 全是 `rewrite` 级垃圾模板——
  `ShapePropertiesHandler.rs` 275 行空壳（0 todo/71 stub，「local stub ShapePr/Fill; all empty」）、
  `FillPropertiesHandler.rs` 110 行（34 stub）、`BlipFillHandler.rs` 318 行（1 todo/81 stub，
  本地 stub BlipFill/FillMode）、`OutlineHandler.rs` 428 行（140 stub，handler shells）、
  `PresetGeometryHandler.rs` 129 行、`CustomGeometryHandler.rs` 1191 行（8 todo/381 stub，
  dispatch empty）、`Transform2DHandler.rs` 121 行（本地 `_stub` 遮蔽真 Transform2D）。
  **当前工作树这些 handler 已从模块接线中注释**（`sppr/mod.rs` 等仅保留 `// pub mod …` 占位），
  默认 crate 不编译。**loop 需重做而非增量实现**：重新 `template --recursive` 生成 Java 结构模板
  后整体改写，本地 stub 全部删除，改用 `data/*.rs` 真实模型。
- 覆盖/对齐数字：FUNCTION.md §3.7/§3.9 保持占位，由 loop 后脚本写入，本 split 不填。

## §4 串联验收

1. 01 用内存 attrs/events 完成 bwMode + xfrm 四属性 + 六类子元素分发；分发返回值与 Java
   `enterSubElement` 一一对应（不返回 handler 的分支保持 null/None 语义）。
2. 02 用 fill 六 id 的 fixture 验证 fillType 与最小子 handler 写入；solidFill 走
   `ColorHandlerFactory` 契约（`1114371` 上下文 id）。
3. 03 用 ln fixture 验证 w/cmpd/cap/algn/lineJoin/dash 写入与 endElement flush。
4. 04 用 prstGeom + custGeom 最小 fixture 验证 prst id、avLst、rect、pathLst（moveTo/lnTo/close）。
5. 四个 function 串读一段真实 spPr XML 事件（内存 fixture），结果体现为一个完整 `ShapePr`，
   可被 pl-02/pr-02 的 contract 消费。

## §5 推荐 loop 顺序

`01_function_shape_properties → 02_function_fill → 03_function_outline → 04_function_geometry`

（01 先行：顶层分发与 xfrm 是阻塞主链；02/03/04 各自由 01 的 dispatch 契约驱动，可并行。）

## §6 类级范围

| Java 类 | function | Rust 现状（审计表） |
|---|---|---|
| `ShapePropertiesHandler`（`…/sppr/ShapePropertiesHandler.java`） | 01 | `sppr/ShapePropertiesHandler.rs` 275 行空壳（rewrite）；现模块接线已注释 → **重做** |
| `GroupShapePropertiesHandler`（`…/sppr/GroupShapePropertiesHandler.java`） | 01 | `sppr/GroupShapePropertiesHandler.rs` 183 行空壳（rewrite）；empty handlers |
| `Transform2DHandler` / `PositiveSize2DHandler` / `Point2DHandler`（`…/sppr/transform/`） | 01 | `Transform2DHandler.rs` 121 行（`_stub` 遮蔽）；Point2D/PositiveSize2D 各 72 行 stub |
| `GroupTransform2DHandler`（`…/sppr/transform/GroupTransform2DHandler.java`） | 01（group 契约） | `GroupTransform2DHandler.rs` 203 行（53 stub） |
| `FillPropertiesHandler` / `SolidFillHandler` / `GradientFillHandler` / `PatternFillHandler` / `BlipFillHandler` / `BlipHandler` / `RelativeRectHandler`（`…/sppr/fill/`） | 02 | `FillPropertiesHandler.rs` 110 行、`BlipFillHandler.rs` 318 行、`GradientFillHandler.rs` 等均 rewrite 级 → **重做** |
| `OutlineHandler` + nested `MiterHandler`/`LineEndHandler`/`PrstDashHandler`/`CustomDashHandler`（`…/sppr/outline/`） | 03 | `OutlineHandler.rs` 428 行（140 stub，handler shells）→ **重做** |
| `PresetGeometryHandler` / `CustomGeometryHandler` + nested path/segment 族（`…/sppr/geometry/`） | 04 | `PresetGeometryHandler.rs` 129 行、`CustomGeometryHandler.rs` 1191 行（dispatch empty）→ **重做** |
| `ColorHandler` / `ColorHandlerFactory`（`…/handler/color/`） | 02 依赖 | 色链走已有 `handler/color` 契约；最小集内复用，不在此 feature 展开全色表 |

Rust 根路径前缀：`libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/`。元素/attr id 以
`koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/xsd/dml/A.java` 与 `pml/P.java`、`shared/R.java`
（`r:embed`=4259842、`r:link`=4259843）为准（已逐项查证，见各 FUNCTION.md §2）。

## 文档

- 机器状态：[functions.csv](./functions.csv)
- function 方案：`functions/<function_id>/FUNCTION.md`
- 上游/下游：pp-02 见 [pp-02 FEATURE.md](../pp-02-shape-tree-parse/FEATURE.md)；消费方见
  layout [pl-02](../../../layout/features/pl-02-xfrm-geometry/FEATURE.md) 与 render
  [pr-02](../../../render/features/pr-02-shape-command/FEATURE.md)。