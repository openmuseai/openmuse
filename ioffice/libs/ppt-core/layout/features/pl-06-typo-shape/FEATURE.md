# pl-06-typo-shape — ShowTypoShape 快照

> 层：layout ｜ 状态：**pass** ｜ Java：`ShowTypoShape` ctor（`KmoShowShape.isHidden` / `isPlaceHolder` / `ph().type()` / `isTextBox`）+ `getShapeRect`（`ShapeUtil.calcRelativeRect` + `UnitsConverter.render()/fromEmuX/Y`）

## 目标

把「每个 shape 的**过滤 flags** + **绝对矩形** + **group 子级**」打包成一个可查询的快照对象 `ShowTypoShape`，作为 render 层（pr-01 过滤、pr-02/03 的 rect 输入）的统一输入。这是 Java S3 `ShowShapeRenderFactory.CreateShowShapeRender(shape, ctx, env, rcGeometry)` 依赖对象的 Rust 对应。**不做实际绘制。**

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | `KmoShowShape`（含 `ph`/`transform2D`/`type`）+ `InterpretCtx\|fixture`（主题/过滤上下文，MVP 可 None）+ `geometry: RectF`（pl-02 产出的**绝对 EMU 框**；传入即冻结，本层不再重算） |
| **O** | `ShowTypoShape`：`isHidden` / `isPlaceholder` / `placeholderType` / `isTextBox` / `getShapeRect()`（→ **render 单位**矩形，`UnitsConverter.render().fromEmuX/Y`）/ `getShowShape()` / children（惰性） |
| 约束 | 不调用 Canvas；`getShape()`（`ConvertTool` → 公开 `Shape`）**归属 pl-09-public-shape-convert**，本 feature 保持 `DEFERRED_NON_FUNCTION`，不在快照 loop 里做 convert |

## Java 管线对照

S3 遍历（`PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S3）：`SlideRender.draw` → `drawFilm` 遍历 `spTree.shapeAt(i)` → `ShowShapeRenderFactory.CreateShowShapeRender(shape, ctx, env, rcGeometry)`。快照源：

- ctor（`ShowTypoShape.java:43-58`）：`mIsHidden = shapeBase.isHidden()`；`mIsPlaceholder = shapeBase.isPlaceHolder()`；`mPlaceholderType = shapeBase.ph().type()`；`mIsTextBox = shapeBase.isTextBox()`。
- `getShapeRect()`（`ShowTypoShape.java:119-139`）：`ShapeUtil.calcRelativeRect(shapeBase, rcShape)`（EMU 绝对框，group 子级含旋转合并）→ `UnitsConverter.render().fromEmuX/Y(...)`（render px）；缓存于 `mGeometry`，`clearShapeRectCache()` 置脏。
- 子级（`ShowTypoShape.java:149-174`）：`getChildCount()` 仅 `type()==0`（group）返回 `shapeBase.childCount()`；`getChild(i)` 经 `childAt(i)` 取子 shape，并以 `mChildren` 缓存复用（内存地址相等复用，`typoShape.getShowShape() == shape`）。
- `KmoShowShape`：`type()`（0=group/1=shape/2=picture/3=connector/4=graphicFrame，`KmoShowShape.java:544`）、`childAt/childCount`（`:516/520`）、`isHidden()`（`:2284`）、`isPlaceHolder()`（`:552`）、`ph()`（`:566`）、`isTextBox()`（`:938`）。

## 能力解锁

| function | Java 对照 | 实现后管线新增 |
|---|---|---|
| `01_function_typo_shape_snapshot` | `ShowTypoShape` ctor 字段 + `getShapeRect` | 每 shape 的 hidden/ph/placeholderType/isTextBox + render 单位矩形可查询 → **pr-01 过滤（&1 ph、&32 miner 4/5/6/7）与 pr-02/03 rect 输入就绪** |
| `02_function_child_walk` | `ShowTypoShape.getChildCount`/`getChild`（type 0 递归 + `mChildren` 缓存） | group 子级产出子快照（惰性缓存）→ 层序遍历（master→layout→slide）每 film 拿到一致形状列表 |

缺失断点：没有 pl-06 → pr-01 无输入对象可过滤，管线在 S3 断（`PIPELINE_LAYOUT_RENDER_PLAN.md` §3.2「pl-06（TypoShape 快照）→ pr-01 无输入对象，**必断**」）。

## 必须 / 壳 / 延后

- **must：** snapshot 字段（`isHidden/isPlaceholder/placeholderType/isTextBox/getShapeRect`）；传入 `geometry` 冻结 + `UnitsConverter.render()` 换算；`type()==0` 的 `childCount/childAt` 走 `KmoShowShape`；children 惰性缓存（同一子 shape 复用同一子快照）。
- **stub：** `getShape()` 交给 **pl-09**（禁止在 pl-06 实现 ConvertTool）；`RenderModelOverride`；`TextRenderModel`（pr-03）；`isModularGroupDefaultPic` 恒 false。
- **defer：** ink 修正（`KmoShowShape.hasInk`，`ShowTypoShape.java:128`）、3D 矩阵、`calculateBounds`/effect、`SlideCache` 复用（convert 缓存）。

## 范围

- **in：** slide/film 上所有 type 0-4 形状的快照查询（含 group 递归）。
- **out：** 实际 `ConvertTool`/公开 `Shape`；绘制指令；hit-test。

## 验收

fixture `KmoShowShape` 集合：三层 flags 组合（hidden / ph 有无 / placeholderType 类型号 / isTextBox）与 Java ctor 逐项一致；`getShapeRect` 对给定 EMU 框的 render 单位换算与 Java 同公式一致；group 嵌套结构遍历与 `getChildCount/getChild` 一致（含复用语义：同一 KmoShowShape 只产生一个子快照）。全部通过才可标 **pass**（现为 **spec**）。

## 依赖

- **upstream：** pl-02（geometry 绝对 EMU 框）；pl-03（ph 继承后的 shape 属性，可选）；pl-05（行几何可选挂载，非必须）。
- **downstream：** pr-01（层序/过滤）、**pl-09**（`getShape` 宿主）、pr-02（shape command）、pr-03（text command）。
- **不依赖：** Canvas、wpslayout 引擎。

## 不过扩展

禁止在 pl-06 里做 `ConvertTool`/公开 `Shape` 适配冒充 pr-02；禁止在本层重算 geometry（那是 pl-02）；禁止把 Canvas 语义（`ShapeEnv`）放进快照对象。