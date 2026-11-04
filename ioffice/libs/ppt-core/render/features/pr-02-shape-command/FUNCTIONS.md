# pr-02-shape-command — Function 组合图

> depends_on: `pr-01` slide-layers；`pl-02` xfrm-geometry；`pl-01` units-view  
> 状态：**pass**  
> 输出平台无关形状指令（DrawRect/DrawPath/DrawImage），无 Canvas。**这是最小可画出口**。

## §0 管线位置

S1+S4（见 [PIPELINE_LAYOUT_RENDER_PLAN.md](../../../PIPELINE_LAYOUT_RENDER_PLAN.md) 热路径）。上游 pr-01 给有序层列表，pl-02 给绝对 EMU rect 与 `GraphicsPath`；下游 pr-04 消费指令。对齐 writer wr-07 的 DrawRect/Path 语义。

## §1 function 清单

| order | function_id | I | O | upstream | downstream |
|---|---|---|---|---|---|
| 1 | `01_function_shape_geometry_command` | LayerItem（白底/背景/形状）+ 几何 | `DrawRect`/`DrawPath` 几何指令（样式待 02） | pr-01；pl-02 | 02 |
| 2 | `02_function_fill_line_command` | 几何指令 + `FillBase` + `LineProperty` | 完成样式的 `RenderCommand` | 01 | pr-04-ffi-flutter |
| 3 | `03_function_blip_image` | 几何 rect + blip mediaId/rId | `DrawImage` 或占位 DrawRect | 01；pp-10:02 | pr-04 painter |

指令模型（MVP 稳定版，字段待 loop 与 wr-07 对齐后锁定）：

```text
DrawRect { x, y, w, h, fill, stroke }
DrawPath { path, fill, stroke }
DrawImage { x, y, w, h, media_id, r_id }
```

## §2 数据流

```text
LayerItem
  WhiteBackground{w,h}  ─→ DrawRect{0,0,w,h}（fill=白 0xFFFFFFFF）
  Background            ─→ DrawRect{0,0,w,h}（fill=背景实色，<02>）
  Shape{LayeredTypoShape} ─→ 几何指令化：
    getShapeRect()（pl-02 绝对 rect，EMU→px 经 UnitsConverter.render().fromEmuX/Y）
    getShape().getGraphicsPaths(w,h)（预置几何）
      path.getNeedFill() && fill.getFillOn()  → fill 附着（<02>，SolidFill type 0 → getColor）
      path.getNeedSroke() && line.getfLine()  → stroke 附着（<02>，color+width）
    rect 型几何 → DrawRect；path 型 → DrawPath
```

`GemoRender.drawShape → drawFill → drawStroke` 是 Java 语义锚点；Rust 侧产出的是指令数组，不接触 IGraphics/GraphicsCanvas。

## §3 测试面

- fixture：手工 `LayeredTypoShape`（矩形 + 预置几何各一）+ 实色 fill/line；白底与背景整幅。
- 断言：坐标换算、fill 颜色、noFill/noLine 分支、path 点阵。
- 不跑真实 `PptxReader`；不进画布（指令 JSON 即对比物）。

## §4 验收

1. 白底 → 全幅 DrawRect（fill 白）。
2. 矩形几何+实色 fill → DrawRect（fill=实色）；带 ln → stroke 附着。
3. 预置几何 → DrawPath（path 与 pl-02 `GraphicsPath` 一致）。
4. `getFillOn()==false`（noFill）→ 不产 fill；`getfLine()==false` → 不产 stroke。
5. 背景 → 整幅 DrawRect + 背景实色（BackgroundShapeRender/`ConvertTool.backgroudToPublicShape` 语义）。

## §5 loop

`01 → 02`（02 依赖 01 的几何指令；01 可用 fixture 独立测）。**图片：`03_function_blip_image`（spec）**。pr-01 → pr-02 → pr-03 → pr-04。

## §6 Java/Rust 范围

| Java 类 | Rust 目标 | 裁决 |
|---|---|---|
| `render/ShapeRender.java`（`_render`） | `src/cn/wps/show/render/ShapeRender.rs` | `_render` 的 DrawingRender 调用边界必写；装饰方法停车 |
| `show/moffice/drawing/render/DrawingRender.java` | —（语义源，不整文件移植） | `render(ITypoShape)` 分支语义（shape/text/decor 三分法）取 shape 分支；audit 标 `show/moffice/drawing/render/*` delete |
| `show/moffice/drawing/render/GemoRender.java` | —（语义源） | `drawFill`/`drawStroke` 决策指令化；drawDiagram/drawInk 空返回 |
| `show/moffice/drawing/render/ShapeRenderFactory.java` | —（语义源） | 只取 GemoRender 分支；Group/WordArt/Chart 停车 |
| `render/util/ShowShapeRenderFactory.java` | —（语义源） | type==4 → `CreateGraphicFrameRender` 分支壳 |
| `core-libs/drawing` `Shape.getGraphicsPaths` | `drawing::cn::wps::moffice::drawing::Shape`（已有） | `GraphicsPath.rs`/`FillBase.rs`/`LineProperty.rs` 已有（keep） |
| `base/converter/UnitsConverter.java` | `src/cn/wps/base/converter/UnitsConverter.rs`（486 行，keep） | `render().fromEmuX/Y` 复用 |
| —（指令模型） | `src/cn/wps/show/render/rendercommand/RenderCommand.rs`（新建） | DrawRect/DrawPath + serde 字段；字段名待 loop 与 wr-07 对齐 |

## 文档

- [functions.csv](./functions.csv)