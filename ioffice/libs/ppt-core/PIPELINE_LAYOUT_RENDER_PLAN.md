# PPT 渲染管线：Java 阶段分析、Rust 重写计划与 MVP 可行性

> 日期：2026-08-29
> 对照：writer 版 [PIPELINE_LAYOUT_RENDER_PLAN.md](../word-core/layout/PIPELINE_LAYOUT_RENDER_PLAN.md)（`.docx → TypoSnapshot → RenderCommand[] → Flutter`）
> 本文件取代 [RENDERING_PIPELINE_TRACE.md](./RENDERING_PIPELINE_TRACE.md) 的分析部分（该文件是旧粗粒度 10 段；本文按真机代码逐层拆）
> 三层定义见 [MVP.md](./MVP.md)；feature/function 清单在各层 `features/` 目录。

> 下一波（骨架 → 色/字/图）：[VISUAL_CONTENT_PIPELINE.md](./VISUAL_CONTENT_PIPELINE.md)（pp-14/15/16 + pl-07 + pr-05）

```text
.pptx
  → [S0 parse]      PptxReader/open 链 → KmoPresentation / KmoSlide / KmoShowShape / Theme
  → [S1 convert]    ConvertTool（KmoShowShape → 公开 Shape：xfrm/geom/fill/ln/effect）
  → [S2 layout]     ShapeUtil.calcRelativeRect（EMU rect）+ ShowTextLayout（TextBoxData → RichTextLayout.typo）
  → [S3 遍历]       SlideRender.draw → SlideRenderTool.drawFilm（master → layout → slide 三层胶片）
  → [S4 形状绘制]   ShapeRender._begin/_render → DrawingRender（fill/line/geom → canvas path）
  → [S5 文本绘制]   ShapeTextRender → TextRender（typo 行 → canvas 文本）
  → [S6 paint]      android.graphics.Canvas →（Rust：RenderCommand[] → Flutter）
```

与 writer 的对应：writer 的 `LayoutRender → IRenderDrawer → Canvas` 在 PPT 侧是 `SlideRender → IShowShapeRender → Canvas`；writer 的 `TextLayouter.flow` 在 PPT 侧是 **core-libs/wpslayout 的 `ShowRichTextLayout.typo`**（文本框排版，非 word 的 LineLayouter）。「SlideLayout」是 OOXML 版式 part（parse 词），不是排版引擎。

---

## 1. Java 渲染管线阶段分析

### S0 打开与解析（parse）

| | 内容 |
|--|------|
| **I** | `.pptx` 文件（zip/OPC）；`[Content_Types].xml`、`presentation.xml`、slide/layout/master/theme part |
| **O** | `KmoPresentation`：`KmoSlide[]`、每 slide 的 `KmoShapeTree`、`KmoSlideLayout`/`KmoSlideMaster`（三层继承）、`KmoTheme`（ColorMapping/FontScheme/FillStyleLst 等） |
| **功能** | `PptxReader.open` → `PptxrPresentation.open`（theme 先于 presentation.xml）→ `PptxrSlides` 装配 → DML handler 把 `spPr`→`ShapePr`、`txBody`→`KmoTextBody`、`spTree`→`KmoShapeTree`、`xfrm`→`Transform2D` 写进 `KmoShowShape` |
| **数据模型** | `KmoShowShape`（type 0-4：group/shape/picture/connector/graphicFrame；持有 `ShapePr`/`KmoTextBody`/`Transform2D`/`ph`/graphic）、`ShapePr`（xfrm/geom/fill/ln/effect）、`KmoTextBody`/`TextBody`（per-para/run 富文本）、`KmoTheme`、`ColorMapping` |
| **关键 Java 类** | `PptxReader`、`PptxrPresentation`、`PptxrSlides`、`ShowShapeHandler` 族（sppr/txbody/spTree）、`ShapePropertiesHandler`、`TextBodyHandler`、`ThemeHandler` |
| **Rust 归属** | parser `pp-01..pp-09`（已有）+ **新增 `pp-10-dml-sppr`**（spPr：xfrm/geom/fill/ln，渲染阻塞）+ **新增 `pp-11-dml-txbody`**（txBody→KmoTextBody，有字才必要） |

**渲染断点**：没有 spPr，形状无坐标/无几何/无填充，画不出来；没有 txBody，文本框无字。这是当前 parse 只绿到 pp-09 仍然**不能画**的根本原因。

### S1 模型 → 公共渲染模型（convert）

| | 内容 |
|--|------|
| **I** | `KmoShowShape` + `InterpretCtx`（主题/颜色/过滤上下文）+ `ShapeInfoCache`（缓存 transform2D/rotation/flip） |
| **O** | `cn.wps.moffice.drawing.Shape`（公开渲染模型：geometry path、`FillBase`、`LineProperty`、effects、GRF/flip/rotation、shapeRect）（经 `SlideCache` 缓存，`noFilter` 时命中即返） |
| **功能** | `ConvertTool.convertShapeToPublicShape` 按 type 0-4 分发；`shapePropertiesToPublicShape` 把 `ShapePr` 译成公开 Shape：`ShapeInfoCache`→rotation/flip；`ShapeUtil.calcRelativeRect`→anchor rect；`GeomtryConvert`（prstGeom/custGeom/缺几何时 placeholder 默认矩形）→path；`FillConvert`（solid/grad/blip/pattern + bg-fill + overlay）→`FillBase`；`LineStyleConvert`→`LineProperty`；effect 转换（shadow/reflection/glow/softedge）；3D 转换（MVP 不做） |
| **数据模型** | 公开 `Shape`（含 `ShapePos`/`GRF`/`FillBase`/`LineProperty`/`GraphicsPath`/`RenderModel`）、`FillBase` 族、`LineProperty`、`GeometryCache` |
| **关键 Java 类** | `ConvertTool`、`GeomtryConvert`、`FillConvert`、`LineStyleConvert`、`PresetShapeTypeMap`、`ShapeUtil`、`ShapeInfoCache`、`SlideCache` |
| **Rust 归属** | 审计表标注 `show/convert/*` 为 `add, MISSING`。按 MVP.md 分配：transform/几何落到 **pl-02**（xfrm→RectF）、fill/line/几何→指令落到 **pr-02**（DrawRect/DrawPath）。`PresetShapeTypeMap.rs`（415 行）已 keep |

**渲染断点**：没有 convert，`ShowTypoShape.getShape()` 拿不到公开 Shape，`DrawingRender` 无物可画。

### S2 布局（几何 rect + 文本 typo）

| | 内容 |
|--|------|
| **I** | 已装配的 `KmoShowShape`（含 spPr/txBody）+ slide/layout/master 三层（placeholder 继承源）+ `UnitsConverter`（doc/layout/render 三档） |
| **O** | 每 shape 的**绝对 EMU rect**（`ShapeInfoCache.transform2D` + group 子级递归 `calcChildRelativeRect`）；合并后的 `Transform2D`/`ShapePr`（`PlaceHolderMerger`）；文本框排版结果 `IShowTextLayout`（行/Span 几何 + typo cache） |
| **功能** | ①几何：`ShapeUtil.calcRelativeRect(KmoShowShape, RectF)` — `ShapeInfoCache.transform2D(shape, rc)` 取相对 EMU 框；若有 `parentGroup` 递归 `calcShape2TopMatrix` + `calcChildRelativeRect`（含子级 rotation 60000 分之一度）合并成绝对框。②占位符继承：`PlaceHolderMerger.mergeTransform/mergeShapePr` — slide ph 无 `xfrm` 时按 `ph.idx`/`phType`（title=0/1）从 **layout 的 ph shape** 继承，layout 没有则从 **master** 的 title/body ph 继承。③文本排版：`ShowTextLayout.layout(typoRect, shape, ctx, forceTypo, cb)` → `ShapeDataTransform.exToInternal`（KmoTextBody 的 para/run → 内部 `TextBoxData`/`TextData`/`ParaInfo`/`LineInfo`/`SpanInfo`，含 autofit/vertical/warp 判定）→ `new ShowRichTextLayout().typo(boxData, w, h, forceTypo)` 断行产出行几何 |
| **数据模型** | `RectF`（EMU→render px 前仍 EMU）、`Transform2D`/`GroupTransform2D`/`ShapeInfo`、`TextBoxData`/`TextData`/`ParaInfo`/`LineInfo`/`SpanInfo`（**在 core-libs/wpslayout**）、`TextBodyProperties`（insets/marL/marR/autofit） |
| **关键 Java 类** | `ShapeUtil`（calcRelativeRect/calcShape2TopMatrix）、`ShapeInfoCache`、`PlaceHolderMerger`、`ShowTextLayout`、`ShapeDataTransform`、`ShowRichTextLayout`→`RichTextLayout`（wpslayout） |
| **Rust 归属** | layout `pl-01`（EMU+视口）→ `pl-02`（xfrm→绝对 RectF）→ `pl-03`（placeholder 继承合并）→ `pl-04`（shape→TextBoxData）→ `pl-05`（框内断行，MVP 贪心 ≠ Java wpslayout）→ `pl-06`（ShowTypoShape 快照） |

**渲染断点**：没有框就没有绘制区域；没有 typo 就没有文本行几何。**最大风险**：Java `RichTextLayout`（wpslayout）在 Rust 侧 `core_libs_wpslayout` 有 3352 个 `todo!`——pl-05 若走「与 Java 一致」会撞上整块未移植引擎，第一波明确 MVP 用贪心断行绕开（同 writer M1 捷径）。

### S3 层序与形状遍历（render traversal）

| | 内容 |
|--|------|
| **I** | `KmoSlide` + `slideDrawFlags`/`followerDrawFlags`（按 read/edit/thumb/print/master/back 场景由 `SlideRender` 静态工厂设定：如 read=21505154、follower=532609） |
| **O** | 依序发出的每 shape `IShowShapeRender`（`ShowTypoShape` + `ShowShapeEnv`） |
| **功能** | `SlideRender.draw(canvas, slide)`：①`drawUnderSlide`（非 partChange 时）：白底 `drawSlideWhiteBackground(w,h)`（`UnitsConverter.render().fromEmu` 把 slide EMU 换成渲染 px）→ `drawBackgroundShape`（slide.background → `ShowTypoBackground`+`BackgroundShapeRender`）→ `drawShapesOnLayout`：`layout.isShowMasterShape && slide.isShowMasterShape` 先画 **master film**，`slide.isShowMasterShape` 再画 **layout film**（followerDrawFlags）。②`mSlideContext.reset(slide, slideDrawFlags)` → `renderTool.drawSlide` → `drawFilm`：遍历 `spTree.shapeAt(i)` → `ShowShapeRenderFactory.CreateShowShapeRender(shape, ctx, env, rcGeometry)`（type==4 走 graphicFrame 分支 Chart/Table/SmartArt/OLE/Model3D/Unknown；否则 `ShapeRender`）→ `render(canvas)` |
| **数据模型** | `SlideRenderContext`（mSlide/mDrawFlags/mChangeIndex）、`InterpretCtx`、`ShowShapeEnv`（上下文+canvas）、`KmoShapeTree` |
| **关键 Java 类** | `SlideRender`、`SlideRenderBase`、`SlideRenderContext`、`SlideRenderTool`、`ShowShapeRenderFactory`、`ShowTypoShape`、`BackgroundShapeRender` |
| **Rust 归属** | render `pr-01-slide-layers`（三层胶片顺序 + drawFlags 过滤） |

**渲染断点**：层序错了，母版/版式形状会盖错层。`MVP.md` 明确「Java `SlideRender.drawUnderSlide` 的层序必须钉死」。

### S4 形状绘制（shape command）

| | 内容 |
|--|------|
| **I** | `ShowTypoShape`（公开 `Shape` + render 单位 rect）+ `ShowShapeEnv`（canvas + drawFlags） |
| **O** | canvas 绘制调用（drawRect/drawPath/drawLine + Paint） |
| **功能** | `ShapeRender._begin`：hidden、`drawFlags&1` 跳过 placeholder、`&32` 跳过 miner ph（type 4/5/6/7）；`_render`：委托 `DrawingRender.render(typoShape)` 画 fill/line/预置几何 path；随后按 drawFlags 叠加编辑装饰（占位虚线框/布局边框/图片 ph 图标）；group（type 0/modular）递归子 ph 装饰 |
| **数据模型** | 公开 `Shape`（FillBase/LineProperty/RenderModel）、`DrawingRender`、`ShapeEnv` |
| **关键 Java 类** | `ShapeRender`、`DrawingRender`（moffice/drawing/render，深层绘图引擎）、`ShowShapeEnv` |
| **Rust 归属** | render `pr-02-shape-command`：几何 fill/line → `DrawRect`/`DrawPath`（不做 `DrawingRender` 全部，只做 MVP：白底/实色 fill/线/预置几何） |

**渲染断点**：pr-02 是最小可画的出口——没有它，形状层只出排序对象不出指令。

### S5 文本绘制（text command）

| | 内容 |
|--|------|
| **I** | `ShowTypoShape` + `IShowTextLayout`（typo 结果）+ renderModel.getTextRect（shape 内 insets 文本区） |
| **O** | canvas 文本绘制（逐 run/span：字体/字号/颜色/下划线…，`TextRender.render`） |
| **功能** | `ShapeTextRender._render`：drawPHolderText 过滤；`renderModel.getTextRect(w,h)` 取文本 inset 区；`TextRender.render(canvas, rcShape, typoRect, shape, shapeText, flags)` 依 `IShowTextLayout` 的行/Span 几何画字；flip 时 canvas 镜像 |
| **数据模型** | `ShapeText`（textObj 缓冲）、`ITextRenderModel`、typo 行/Span 几何 |
| **关键 Java 类** | `ShapeTextRender`、`TextRender`、`ShapeText`、`ShowRichTextLayout.renderText` |
| **Rust 归属** | render `pr-03-text-command`：排版行 → `DrawText`（MVP：纯文本行；bullet/动画 timing 不做） |

**渲染断点**：pr-03 之前文本只存在于 typo 结果中，不上屏。**注意**：Java 文本绘制依赖 `ITypeface`/Canvas Paint 度量；Rust MVP 走 `DrawText` 指令 + Flutter TextPainter/字形（对齐口径同 writer Stage3，不做像素 CI 门闩）。

### S6 像素输出（paint）

| | 内容 |
|--|------|
| **I** | `RenderCommand[]`（Rust 侧） |
| **O** | Flutter/平台 painter 把指令画到屏幕上 |
| **功能** | 指令 JSON/UniFFI 出内核；内核禁止 `android.graphics.Canvas` 语义外泄 |
| **关键 Java 类** | `android.graphics.Canvas`（语义源，不做移植目标） |
| **Rust 归属** | render `pr-04-ffi-flutter` |

### 单位换算（贯穿 S1-S5）

`UnitsConverter.render().fromEmuX/Y`（EMU→渲染 px 系数）+ `doc`/`layout` 档。Rust 侧 `base/converter/UnitConverter.rs` + `UnitsConverter.rs`（审计 keep，486 行）已在内核——pl-01 就是把它接到 slide 视口。

### drawFlags（贯穿 S3-S5）

按场景（read/edit/thumb/print/master/back）的位掩码决定：ph 文本/占位框/布局框/批注/3D… 的开关。pr-01 必须锁定 read/thumb 场景用的两个常量（slideDrawFlags=21505154/29909122 等），否则 ph 形状过滤不一致。

---

## 2. Rust 重写计划

### 2.1 三 crate feature 总表（Java 管线环节 ↔ feature）

| feature | 层 | 覆盖的 Java 管线环节 | 状态 | 依赖 | 渲染关键性 |
|---|---|---|---|---|---|
| pp-01-pptx-parse | parser | S0：SWReaderMap/HandlerCallback/nvPr/cSld/sld/layout/master XML handler | pass | — | 必要（树+协议） |
| pp-02-shape-tree-parse | parser | S0：spTree 事件 → KmoShowShape/DgStore 行 | pass | pp-01 | 必要（形状树）但不充分 |
| pp-03-dml-style | parser | S0：clrMap/lstStyle（颜色槽与文本样式） | spec | pp-01 | 画色必要 |
| pp-04-theme-part | parser | S0：theme part → KmoTheme | spec | pp-01;pp-03 | 画色必要（scheme 色） |
| pp-05-timing-transition | parser | S0：transition/timing 列表 | spec | pp-01 | **首屏不需要** |
| pp-06-presentation-xml | parser | S0：presentation.xml 属性 + rId 列表 | spec | pp-01;pp-03 | 闭环打开必要（至少 sldSz） |
| pp-07-part-open | parser | S0：part stream → UnifiedXmlReader + callback.load | spec | pp-01;pp-02;pp-04 | 闭环打开必要 |
| pp-08-slides-walk | parser | S0：rId → KmoSlide/Master + 阅读顺序 | spec | pp-06;pp-07 | 闭环打开必要 |
| pp-09-pptx-open | parser | S0：PptxReader.open 编排 | spec | pp-04;pp-06;pp-08 | 闭环打开必要 |
| **pp-10-dml-sppr（新）** | parser | S1：**spPr → ShapePr（xfrm/geom/fill/ln）** | 待建 | pp-01;pp-02 | **渲染阻塞** |
| **pp-11-dml-txbody（新）** | parser | S1：**txBody → KmoTextBody** | 待建 | pp-01;pp-02;pp-10 | **有字才必要** |
| pl-01-units-view | layout | S2：UnitsConverter + slide 视口 | spec | — | 必要（坐标基准） |
| pl-02-xfrm-geometry | layout | S2：ShapeInfoCache.transform2D + group 子级 → 绝对 RectF | spec | pl-01 | **必要**（画在哪儿） |
| pl-03-ph-merge | layout | S2：PlaceHolderMerger.mergeTransform/mergeShapePr | spec | pp-02;pl-02 | 必要（ph 形状补齐） |
| pl-04-text-box | layout | S2：ShapeDataTransform.exToInternal → TextBoxData | spec | pp-11;pl-03 | 有字才必要 |
| pl-05-text-typo | layout | S2：RichTextLayout.typo（MVP 贪心断行） | spec | pl-04;pl-01 | 有字才必要；（Java 一致版依赖 wpslayout） |
| pl-06-typo-shape | layout | S2：ShowTypoShape 快照（hidden/ph/geometry/children） | spec | pl-02;pl-05 | **必要**（pr 的输入） |
| pr-01-slide-layers | render | S3：drawUnderSlide 层序 + drawFlags 过滤 | spec | pl-06 | **必要**（层序） |
| pr-02-shape-command | render | S1+S4：fill/line/预置几何 → DrawRect/DrawPath | spec | pr-01 | **最小可画出口** |
| pr-03-text-command | render | S5：typo 行 → DrawText | spec | pl-05;pr-01 | 有字才必要 |
| pr-04-ffi-flutter | render | S6：指令 → Flutter | spec | pr-02;pr-03 | 出屏入口 |

### 2.2 function 拆解与能力解锁（新拆 feature 全量 + 既有 feature 关键项）

> 「能力解锁」= 该 function 实现并测试通过后，`pptx → 像素` 链条新增了哪一段能力；缺失时管线断在哪。

**parser 补齐（渲染阻塞）**

| function_id | Java 对照 | 实现后解锁的管线能力 |
|---|---|---|
| pp-10 `01_function_shape_properties` | `ShapeParametersHandler`/`ShapePropertiesHandler.startElement`（xfrm 1114238、geom、fill、ln 分发） | shape 有了坐标/几何/填充/线条 → S1 convert 有输入 |
| pp-10 `02_function_fill_outline_geom` | `FillPropertiesHandler`/`OutlineHandler`/`PresetGeometryHandler`（spPr 子树） | 形状可带实色/渐变/图片填、线型、预置几何 → S4 能画出形状体 |
| pp-11 `01_function_text_body` | `TextBodyHandler`/`ParagraphHandler`/`TextRun`（txBody 子树） | 文本框进模型 → pl-04 有排版原料 |
| pp-11 `02_function_text_body_props` | `TextBodyPropertiesHandler`（insets/anchor/autofit/vertical） | 文本区几何可算 → pr-03 的 `getTextRect` 有依据 |

**layout（pl-01..pl-06）**

| function_id | Java 对照 | 实现后解锁的管线能力 |
|---|---|---|
| pl-01 `emu_convert` | `UnitsConverter.layout()/render()/doc()` | EMU↔px 三档换算可用 → 所有 rect 有单位语义 |
| pl-01 `slide_viewport` | `Presentation.slideSize`（默认 12192000×6858000） | slide 内容区 = 白底尺寸 → S3 白底/背景有画布 |
| pl-02 `shape_absolute_rect` | `ShapeInfoCache.transform2D` + `ShapeUtil.calcRelativeRect` | 形状在 slide 上的绝对 EMU 框 → 「画在哪儿」 |
| pl-02 `group_child_rect` | `ShapeUtil.calcShape2TopMatrix` + `calcChildRelativeRect`（含 rotation） | group 子级绝对坐标 → 嵌套组可画 |
| pl-03 `ph_merge_transform` | `PlaceHolderMerger.mergeTransform`（idx/type 查 layout→master） | 缺 xfrm 的 ph 形状拿到真实位置 → 文本框/占位可画 |
| pl-03 `ph_merge_style` | `PlaceHolderMerger.mergeShapePr` | ph 形状拿到继承 fill/style → 继承链完整 |
| pl-04 `shape_to_textbox` | `ShapeDataTransform.exToInternal`（para/run → TextBoxData） | 文本从 KmoTextBody 进入内部排版模型 → pl-05 输入就绪 |
| pl-05 `greedy_wrap` | 对照 `RichTextLayout.typo` 断点语义（MVP 贪心，Java 一致版另开） | 框内行几何（start/end cp、行高）→ 文本可排 |
| pl-05 `text_box_geometry` | `renderModel.getTextRect`（insets/anchor） | 文本区内缩几何 → 画的 x/y 正确 |
| pl-06 `typo_shape_snapshot` | `ShowTypoShape`（isHidden/isPlaceholder/placeholderType/getShapeRect） | pr-01 的过滤与 pr-02/03 的 rect 输入打包成 `LayeredTypoShape` |

**render（pr-01..pr-04）**

| function_id | Java 对照 | 实现后解锁的管线能力 |
|---|---|---|
| pr-01 `layer_order` | `SlideRender.drawUnderSlide`（白底→bg→master→layout→slide film） | 三层胶片顺序钉死 → 母版/版式不再盖错层 |
| pr-01 `begin_filter` | `ShapeRender._begin`（hidden / &1 ph / &32 miner 4/5/6/7） | 过滤规则对齐 → 只画该画的形状 |
| pr-02 `shape_command` | `DrawingRender.render`（fill/line/预置几何→path） | **最小可画**：实色矩形/几何形状 → `DrawRect`/`DrawPath` |
| pr-03 `text_command` | `ShapeTextRender._render`/`TextRender.render` | 文本行 → `DrawText`（字体/字号/颜色） |
| pr-04 `command_json` | —（指令序列化，语义源 Canvas） | 指令出内核（JSON）→ 跨端可消费 |
| pr-04 `flutter_painter` | —（Flutter CustomPainter 消费指令） | **第一屏像素出现**（产品捷径出口） |

### 2.3 推荐执行顺序

```text
捷径轨（不读文件也能画）：
  手工 KmoShowShape fixture → pl-01 → pl-02 → pl-06 → pr-01 → pr-02 → pr-03 → pr-04
闭环轨（真 pptx → 画）：
  + pp-10 / pp-11（spPr/txBody 进模型）→ pl-03/ph 合并（+pp-03/04 色）→ pl-04/pl-05（有字）
  + pp-06/07/08/09（打开链）→ pr-04 出口
```

`pp-05`（transition/timing）不在关键路径；`pp-06` 只需 `addDefaultProperty` 的 sldSz 默认即可先画。

---

## 3. MVP 可行性结论

### 3.1 两条验收轨（对齐 writer）

| 轨 | 路径 | 出口 |
|---|---|---|
| **产品捷径** | 手工 `KmoShowShape` fixture（xfrm+实色 fill+一段字）→ pl-01/02/06 → pr-01/02/03 → pr-04 | Flutter 画出一页（矩形+文字） |
| **产品闭环** | `PptxReader.open` + pp-10/11 真实 spPr/txBody + pp-03/04 色 → 同上 | 打开真实 pptx 并渲染当前页 |

### 3.2 渲染断点对照（function → 断在哪）

| 缺失 | 断点 |
|---|---|
| pp-10（spPr） | shape 无坐标/几何/填充 → S1 convert 无输入，**必断** |
| pp-11（txBody） | 文本框无字 → pl-04 空，**有字必断** |
| pl-02（绝对 rect） | 不知道画在哪 → S4 无框可绘，**必断** |
| pl-06（TypoShape 快照） | pr-01 无输入对象，**必断** |
| pr-01（层序） | 层序错/过滤错 → 画面错误（不断，但不一致） |
| pr-02（shape command） | 形状层无指令 → **最小可画断** |
| pr-03（text command） | 文本不上屏（有字时断） |
| pr-04（ffi） | 指令出不了内核 → **出屏断** |

### 3.3 风险清单

1. **wpslayout 排版引擎（最高风险）**：Java `RichTextLayout` 在 `core_libs_wpslayout` Rust 侧 178 文件、3352 个 `todo!`。pl-05 若对标 Java 断行，需先移植该引擎；MVP 用贪心断行绕开（同 writer M1 捷径），Java 一致文本排版另开 feature（对齐 `LINE_PARITY_PLAN` 的「先画、后对齐」节奏）。
2. **convert 层缺失**：`show/convert/*` 全部 `add, MISSING`。转译工作量大且是 S1 核心；MVP 把它的「用途子集」拆进 pl-02（transform）与 pr-02（fill/line/几何→指令），不整文件移植。
3. **drawFlags 语义**：read/thumb 等场景常量与位含义需从 `SlideRender` 工厂方法锁定，否则 ph 过滤漂移。
4. **blip 图片**：第一波占位矩形；真实图片依赖 parse media + 平台解码（post-MVP）。
5. **效果层/3D/graphicFrame**：shadow/reflection/glow、3D、Chart/Table/SmartArt/OLE 明确不做（`pr-02` MVP 子集）。
6. **像素级一致不做 CI 门闩**（同 writer）：几何对齐走指令 JSON，光栅差异不阻塞 pass。

### 3.4 结论

**能。** 在所有列出的 function 实现并通过门闩后，可以构建 MVP 渲染管线并渲染出 PPT，但口径要钉死：

- **产品捷径轨**：`pl-01..pl-06 + pr-01..pr-04` 实现后即可用 fixture 画出第一页（矩形 + 文本行 + 三层胶片顺序）——这是「能渲染」的最小充分集，且不依赖 parser 闭环。
- **产品闭环轨**：再加 `pp-10/pp-11`（spPr/txBody）与 `pp-06..09`（打开链）+ `pp-03/04`（颜色）后，可打开**真实 pptx** 并渲染其当前页。
- **渲染范围（MVP 语义）**：白底、背景 fill、实色/预置几何形状（fill+line）、文本框纯文本行、ph 继承（layout/master）、母版/版式/slide 三层；图片=占位矩形；**不含** 图片位图、Chart/Table/SmartArt/OLE/3D、效果层、动画/切换、复杂自定义几何与 Java 一致的富文本断行。
- **验收口径**：`RenderCommand[]` JSON 序列对齐 + Flutter 出屏；像素级一致与 Java 文本排版一致**不是**第一波门闩（对齐 writer MVP 的 M1 阶段）。

一句话：**function 全绿后「能渲染出 ppt」成立，前提是接受 MVP 画质语义（矩形+文本+层序）而非 Java 像素级 clone；文本排版是唯一高风险链路，需在 pl-05 明确走贪心断行。**

---

## 文档索引

- 三层定义：[MVP.md](./MVP.md)
- parse 特征：[parser/features](./parser/features/README.md)（pp-10/11 见各自 FEATURE.md）
- layout 特征：[layout/features](./layout/features/README.md)（pl-01..06）
- render 特征：[render/features](./render/features/README.md)（pr-01..04）
- 审计表：[ppt_core_mvp_audit.csv](./ppt_core_mvp_audit.csv)
- 旧粗粒度追踪：`RENDERING_PIPELINE_TRACE.md`（分析部分由本文件取代）