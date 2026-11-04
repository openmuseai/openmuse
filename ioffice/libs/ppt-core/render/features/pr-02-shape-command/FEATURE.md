# pr-02-shape-command — 形状 RenderCommand

> 层：render ｜ 状态：**pass**（01–03）

## 目标

把 pr-01 的有序层列表变成**平台无关的形状指令**：`DrawRect`（白底/矩形几何/背景整幅）与 `DrawPath`（预置几何 path），指令携带坐标（EMU→render px，`UnitsConverter.render().fromEmuX/Y`）、实色 fill 与 line 样式。实现后 **`pptx → 像素` 链条首次具备最小可画出口**（主文档：pr-02 是最小可画的出口——没有它，形状层只出排序对象不出指令）。

内核只产指令，**不下任何 Canvas 调用**；Java `GemoRender`/`DrawingRender` 只作语义来源（`core_libs_drawing` 的 `Shape`/`FillBase`/`LineProperty`/`GraphicsPath` 已在本库，审计 keep）。

## I/O（锁死）

| function | I | O | 契约 |
|---|---|---|---|
| 01 shape_geometry_command | `LayerItem` + 几何 | `DrawRect`/`DrawPath` | 白底/矩形/预置 path；fromEmuX/Y |
| 02 fill_line_command | 几何指令 + FillBase + LineProperty | 实色 fill/stroke | fillType==0；noFill/noLine |
| 03 blip_image | 几何 rect + blip mediaId | `DrawImage` 或占位 DrawRect | fillType==2；无 media 禁止画成实色 |

MVP 子集：实色 fill、白底、背景实色、矩形/预置几何、简单 line；**图片 fill 单列为 `03_function_blip_image`（spec）**。仍不做效果层/3D/graphicFrame/渐变图案。

## Java 管线对照

| Java 类/方法 | 语义 | Rust 归属 |
|---|---|---|
| `ShapeRender._render` | `DrawingRender render; render.setShapeEnv(mEnv); render.render((ITypoShape)mTypoShape)` | 02 调用边界（01 已管 _begin） |
| `DrawingRender.render(ITypoShape)` | `setTextDrawn(false)`；renderMode 2→renderText；1→shape.render；默认→`shape.render` + 未画字则 renderText + renderDecor | 本 feature 只取 shape 分支（文本分支在 pr-03） |
| `ShowTypoShape.render(ShapeEnv)` | type!=4 → `ShapeRenderFactory.createShapeRender(getShape())`（GemoRender）；type==4 → ShowShapeRenderFactory | GemoRender 语义 |
| `ShapeRenderFactory.createShapeRender` | GRF isGroup→GroupRender；isArtText→WordArtRender；isChartShape→ChartRender；default→GemoRender | 只移植 GemoRender 子集 |
| `GemoRender._render` | rect=(0,0,w,h)；drawDiagram→drawInk→`drawShape(gc, shape, rect, rotation)` | 几何指令化 |
| `GemoRender.drawShape/drawFill/drawStroke` | `shape.getGraphicsPaths(w,h)`；`path.getNeedFill()`→`gc.setFill(fill)+drawPath`；`path.getNeedSroke()`→`gc.setStroke(stroke)+drawPath` | 指令决策：needFill/needSroke |
| `Shape.getGraphicsPaths` | `DataCal.getGraphicsPaths(w,h)`→`GraphicsPath[]`（prstGeom 预置几何） | pl-02 供 path；MVP 子集 |
| `FillBase`/`SolidFill`（fillType=0） | `getFillType()`/`getFillOn()`/`getColor()`/`getTransparency()` | 实色 fill |
| `LineProperty` | `getfLine()`/`getColor()`/`getWidth()`/`getTransparency()`/`getLineStyle()`/`getDashing()` | 线样式（MVP 只 color/width） |
| `UnitsConverter.render().fromEmuX/Y` | EMU→render px | 坐标换算 |
| `ShowShapeRenderFactory.CreateShowShapeRender` | type==4 → `CreateGraphicFrameRender`（graphicObj 0/1/2/3/4 → Chart/Table/SmartArt/OLE/Model3D） | type==4 分支 = 壳（post-MVP） |
| `ConvertTool.backgroudToPublicShape` | `Background → 公开 Shape`（含 fill） | 背景 fill 入口（02） |
| `SlideRenderTool.drawSlideWhiteBackground` | 白底 `WHITE_PAINT`（Style.FILL, color 0xFFFFFFFF） | 白底 DrawRect |

Rust 根路径：`libs/ppt-core/src/cn/wps/show/render/`（新增 `rendercommand/` 放指令模型）。

## 能力解锁

- `01_function_shape_geometry_command`：层元素 → 几何指令 → **形状能出指令**（主文档 pr-02 `shape_command` 行：最小可画 = 实色矩形/几何形状 → DrawRect/DrawPath）。
- `02_function_fill_line_command`：实色 fill/line 附着 → 指令可被 painter 直接消费；背景整幅 fill 覆盖 → 白底以外的 slide 背景可画。

缺失断点：pr-02 缺失 = 形状层无指令 = **最小可画断**（主文档 §3.2）。

## 必须 / 壳 / 延后

- **must：** 01 的白底 DrawRect、矩形几何 DrawRect、预置几何 DrawPath、EMU→px 换算；02 的实色 fill（fillType==0、`getFillOn`）、line（`getfLine`、color、width）、背景实色整幅 fill；指令模型（`RenderCommand`：DrawRect/DrawPath 带 fill/stroke）。
- **壳：** type==4 graphicFrame 分支（顺序语义已由 pr-01 管，此处输出透传或空，业务体 post-MVP）；`GemoRender.drawDiagram/drawInk` 空返回。
- **defer：** 效果层、3D、渐变/图案 fill、tile/srcRect 裁剪细节（03 只锁整幅 DrawImage）。

## 范围

- **in：** 白底 DrawRect；形状几何 → DrawRect/DrawPath；实色 fill 与简单 line；背景整幅实色 fill；指令模型的 serde 字段（颜色 RGBA、坐标、路径点阵）；`getTextObj`/文本分支只在 `DrawingRender.render` 的行文里保留调用判断（实际文本指令由 pr-03 产出）。
- **out：** 文本指令（pr-03）；指令 JSON/FFI（pr-04）；效果层/3D/graphicFrame/blip 位图；渐变/图案 fill；编辑装饰（dash 框等）；layout/master 的 `LayoutRender` 预览标志。

## 验收

FUNCTIONS.md §4：

1. 最小 fixture（xfrm 矩形 + 实色 fill + ln）→ 恰好 1 个 `DrawRect`（fill=实色，stroke=ln），坐标 = `fromEmuX/Y`。
2. 预置几何 fixture → `DrawPath`（path 点阵与 pl-02 的 `GraphicsPath` 一致）。
3. `getFillOn()==false`（noFill）/ `getfLine()==false` → 对应 fill/stroke 不产出。
4. 背景 fixture → 整幅 DrawRect + 背景实色。
5. 指令 JSON 字段在 pr-04 联测前 roundtrip（可先行在 kernel 内 serde 自测）。

**建议 MVP pass 口径（loop 采用）：** 01 必测 = 白底 + 矩形 DrawRect + 预置几何 DrawPath + 换算；02 必测 = 实色 fill/line + noFill/noLine 两分支；渐变/blip/效果层 `#[ignore = "spec: post-MVP"]`。

## 依赖

- **depends：** pr-01（有序层列表）；pl-02（绝对 EMU rect → 几何输入）；pl-01（`UnitsConverter.render()`，已 keep）；pl-06（TypoShape 快照）；**pl-09（公开 Shape：`getShape()` → FillBase/LineProperty/GraphicsPath）**。今日产品路径仍从 `layout_shape` 读 path/fill（指令捷径）；pl-09 pass 后本 feature 应改消费公开 Shape，禁止维持第二套几何。
- **does not depend：** pp-10/11（fixture 几何可先行）；pr-03/04；`DrawingRender` 全量（只取 GemoRender 子集语义）。

## 不过扩展

禁止把 Canvas/Paint 对象写进内核装备（`GeometryCache`、`GraphicsCanvas`、`gc.setFill/setStroke` 只作语义参照，不落 Rust）。禁止实现渐变/图案/blip fill 与效果层假装成实色（`fillType!=0` → MVP 跳过或占位，不许画成实色冒充绿）。禁止把 `DrawRect` 白底之外的"背景装饰"（页边距、母版背景图）并入本 feature。`card 图形（graphicFrame）的业务体`一律 `#[ignore = "spec: post-MVP"]`。