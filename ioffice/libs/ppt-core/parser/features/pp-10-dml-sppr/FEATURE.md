# pp-10-dml-sppr — 形状属性 spPr

> 状态：**pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（4 个 function，均 pass）
> 依赖：pp-01-pptx-parse; pp-02-shape-tree-parse
> 管线：[PIPELINE.md](../PIPELINE.md) / [README.md](../README.md) / [MVP.md](../../MVP.md)

## 目标

把 `p:spPr`（及 group 的 `p:grpSpPr`）子树从 pp-02 的 deferred 分发改成**写入 `ShapePr` 模型**：
xfrm（坐标/旋转/flip）、geom（预置/自定义几何）、fill、ln 四条子链。对齐 MVP.md：
**pp-10 是渲染阻塞**——没有 spPr，形状无坐标/无几何/无填充，S1 convert 无输入，画不出来
（[PIPELINE_LAYOUT_RENDER_PLAN.md](../../PIPELINE_LAYOUT_RENDER_PLAN.md) §1 S0 渲染断点、
§2.1 pp-10 行）。本 feature 只把 DML 属性写进 `ShapePr`（`data/shape/ShapePr.rs` 已存在），
不吃排版、不转换、不渲染。

## I/O（锁死）

| function | I | O | 契约 |
|---|---|---|---|
| 01 spPr 分发 + xfrm | `p:spPr`（`ShapePropertiesHandler`）/`p:grpSpPr`（`GroupShapePropertiesHandler`）attrs+子事件 + `ShapePr` | bwMode + `Transform2D`（off/ext/rot/flip）+ fill/ln/geometry 子分发 | 子元素 id（kfc dml `A.java`）：ln=1114148、fill=1114370–1114374+3211586、effectLst/Dag=1114389/1114390、prstGeom=1114429、custGeom=1114428、xfrm=3211589；grpSpPr 同 fill/effect 分发，xfrm 走 `GroupTransform2DHandler` |
| 02 fill | fill 子树事件 + `Fill` | fillType + solid/grad/patt/blip 最小集 | noFill=1114370→type 5、solidFill=1114371→0、gradFill=1114372→4、pattFill=1114373→1、grpFill=1114374→3、blipFill=3211586→2 |
| 03 outline | `a:ln` 子树事件 + `Outline` | w/cmpd/cap/algn + lineJoin + 最小 dash/end | w=1114304、cmpd=1114442、cap=1114441、algn=1114328；round/bevel/miter=1114433/1114434/1114435 → lineJoin 2/0/1；prstDash=1114437、custDash=1114438、headEnd=1114439、tailEnd=1114440 |
| 04 geometry | geometry 子树事件 + `Geometry2D` | prstGeom id + avLst / custGeom 最小子集 | prst=1114293 → `presetShapeTypeMap`；custGeom：avLst=1114422、gdLst=1114423、rect=1114426、pathLst=1114427 最小子集（moveTo/lnTo/close/arcTo 契约旁路） |

## Java 管线对照

Java 渲染管线 **S0 parse** 环节：`PptxrSlide/PptxrLayout/PptxrMaster.open` 的 DML handler 把
`spPr`→`ShapePr`、`xfrm`→`Transform2D` 写进 `KmoShowShape`（PLAN §1 S0「功能」「关键 Java 类」）。
**S1 convert** 环节 `ConvertTool.shapePropertiesToPublicShape` 消费 `ShapePr`：xfrm→
`ShapeInfoCache` rotation/flip、geom→`GeomtryConvert` path、fill→`FillBase`、ln→`LineProperty`。
本 feature 是 **S1 convert 的输入侧**：只补 S0 的 spPr 写入，convert/排版/绘制归 pl-*/pr-*。

## 能力解锁

- 实现并测试通过后：真实 pptx 的 spPr 进模型 → **S1 convert 有输入**（坐标/几何/填充/线条）→
  配合 layout `pl-*`/render `pr-*`，S4 能画出形状体（实色矩形/预置几何 + 线）。
- 缺失时：`p:spPr` 不解析，形状落在 DgStore 行但无坐标/无填充 → `shapePropertiesToPublicShape`
  拿到空 `ShapePr` → pl-02 无 rect、pr-02 无 fill/line → **整条链断在「形状无坐标/无几何/无填充」**。

## 必须 / 壳 / 延后

- **must：** 01 的 bwMode（attr 1114443/3211605 → `blackWhiteModeMap`）与 xfrm 四属性（flipH=1114202、
  flipV=1114203、rot=1114201/3276881、ext 的 cx=1114163/3211488、cy=1114164/3211489、off 的
  x=1114161/3211405、y=1114162/3211406）+ 子元素分发返回值精确。02 的 fillType 六分支 + 实色链
  （`SolidFillHandler`→`ColorHandlerFactory.createColorHandler(…, 1114371)`）+ blip 的 `r:embed`
  (4259842)/`r:link`(4259843) 回调。03 的 w/cmpd/cap/algn + lineJoin + prstDash。04 的 prstGeom id
  与 custGeom 最小编程集（rect + moveTo/lnTo path）。
- **壳：** gradFill 全量（`GradientFillHandler` 的 gsLst 排序可先 fixture/最小双停靠）、pattFill
  fg/bg 色、blip 的 14 种效果 handler、custGeom 的 ahLst/cxnLst/quadBezTo/cubicBezTo/arcTo——
  按「能画出矩形+实色+线」裁剪，先给 contract fixture。
- **延后：** effect 链（effectLst/effectDag）与 3D 三件套（scene3d/sp3d）——MVP 不做效果层/3D；
  `DGMap` 静态查表（审计 `delete` 空壳，按最小键值重建或以测试字面量代替）。

## 范围

- **in：** shape（`p:spPr`）与 group（`p:grpSpPr`）两种分发；xfrm 全属性（含 a:/p: 双命名空间 id）；
  solid/grad/patt/blip 四类 fill 的最小可画集；ln 的宽度/复合/端帽/对齐/连接/虚线；
  prstGeom 的 prst id + avLst；custGeom 的 rect/path 最小子集；`avLst/gdLst` 的 `GeomGuide`
  （fmla=1114391、name=1114136）。
- **out：** 效果层（shadow/reflection/glow 等）、3D、图片位图解码（blip 只回传 rId →
  `onBlipEmbed`/`onBlipLink`）、gradient 停靠全表、theme 槽回填（pp-03/04）、KmoShowShape 装配
  （pp-07 起）、convert/排版/绘制（pl-*/pr-*）。

## 验收

1. 四个 function 各自用内存 attrs/events 完成 spPr 子树写入，方法名/元素 id 与
   `koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/xsd/dml/A.java`（及 `pml/P.java`、`shared/R.java`）
   常数核对（见各 FUNCTION.md §2）。
2. 01 交付的 `Transform2D` 可被 pl-02 fixture 消费（off/ext/rot/flip 数值不变）；02/03/04 交付的
   fill/ln/prstGeom 可被 pr-02 fixture 消费。
3. 不打开 part、不装配 slide；`cargo test --manifest-path libs/ppt-core/Cargo.toml --test 0X_…` 绿。
4. loop 完成后 status 由 review 裁决（spec→pass），函数级 llvm-cov 与 Java↔Rust 结构门禁按
   FUNCTION.md §3.7/§3.9 补报告。

## 依赖

- **depends：** pp-01（`HandlerCallback`（pp-01:02，pass 语义不可改）、`XmlRoAttr`/元素 id 事件面、
  `SWReaderMap` 查表）；pp-02（spTree 分发生成 spPr 子事件；pp-02:02/03/04 构造
  `ShapePropertiesHandler`，pp-02:05 经 group 路径构造 `GroupShapePropertiesHandler`）；数据模型
  `ShapePr`/`Transform2D`/`Fill` 族/`Outline`/`Geometry2D`（`libs/ppt-core/src/cn/wps/drawing/data/`
  已存在）。
- **does not depend：** pp-07 起的 part open 与 slide 装配；渲染；`DGMap` 静态表全量（本层以测试
  字面量/最小常量代替）；theme/colorMapping（pp-03/04）。

## 不过扩展

- 禁止打开 `OpcrPackagePart`（PIPELINE.md §1 故意缺口）；禁止装配 slide；禁止渲染。
- 禁止把 effect/3D/图片位图解码混进本 feature（MVP 明确不做）；**禁止改 pp-01/pp-02 已 pass 的
  语义**（`HandlerCallback` 回调语义、DgStore 行提交顺序、spTree 编排）。
- 禁止绕开既有 `data/*.rs` 模型另造新模型；禁止未验收就置 status=pass；`DGMap` 不整表恢复。