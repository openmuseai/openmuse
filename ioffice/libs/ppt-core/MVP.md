# PPT MVP：Parse → Layout → Render

> 对照：[word-core parser](../word-core/parser/features/README.md) + [word-core layout](../word-core/layout/README.md)  
> 日期：2026-08-29

Writer 的产品闭环不是「DocxReader 绿了就算 MVP」，而是：

```text
.docx → [parse wc-01..05] TextDocument
      → [layout wr-01..06] TypoSnapshot
      → [emit   wr-07]     RenderCommand[]
      → [paint  wr-08]     Flutter
```

PPT 现在只把 **parser** 拆到 `pptx bytes → KmoPresentation`（`parser/features/pp-01..09`）。  
Java 真画一页是另一条链：`PlaceHolderMerger` + `ShowTextLayout` + `ShowTypoShape` + `SlideRender.draw`。  
**parse 绿 ≠ 能渲染。** 名称里的 `SlideLayout` 是 OOXML **母版版式 part**，不是排版引擎。

## 一句话三层

```text
.pptx
  → [parse  pp-*]   KmoPresentation / KmoSlide / KmoShowShape / DgStore / Theme
  → [layout pl-*]   几何 RectF + TextBox 排版结果（ShowTypoShape / IShowTextLayout）
  → [emit   pr-*]   RenderCommand[]（禁止 android.graphics.Canvas 进内核）
  → [paint  pr-04] Flutter / 平台 painter
```

两条验收轨（对齐 writer）：

| 轨 | 像 writer 的 | PPT |
|---|---|---|
| **产品捷径** | zip 抽 `<w:t>` + greedy wrap 可画字 | **手工 `KmoShowShape` fixture**（xfrm+fill+一段字）→ layout → 指令 → 画一页。不跑 `PptxReader` |
| **产品闭环** | `render_docx` | `PptxReader.open` + 当前页 layout + 指令。需要 parse 把 **spPr/txBody/theme** 写进模型 |

Java 像素级一致 **不是** 第一波门闩（同 writer）。

## Java 热路径（绘制，不是 XML）

来源：`SlideRender.draw` / `drawUnderSlide`、`SlideRenderTool.drawFilm`、`ShowShapeRenderFactory`、`ShowTextLayout`、`PlaceHolderMerger`。

```text
SlideRender.draw(canvas, KmoSlide)
  → 若非 partChange：drawUnderSlide
       白底 rect（slideWidth/Height EMU → render 像素）
       drawBackgroundShape(slide.background)
       drawShapesOnLayout：
         若 layout.isShowMasterShape && slide.isShowMasterShape
           → drawFilm(master)     # 母版形状层
         若 slide.isShowMasterShape
           → drawFilm(layout)      # 版式形状层
  → mSlideContext.reset(slide) + drawFilm(slide)
       for i in spTree.shapeAt(i):
         ShowTypoShape(shape, InterpretCtx, geometry)
         CreateShowShapeRender → ShapeRender.render
           _begin：hidden / 按 drawFlags 跳过 placeholder
           几何：xfrm（缺则 PlaceHolderMerger.mergeTransform）
           填充/线/预设几何
           文本：ShowTextLayout.layout(typoRect, shape) → ShowRichTextLayout.typo
```

`LayoutRender` 是画 **KmoSlideLayout 预览**，不是排版算法。

## 当前 parser 划分：哪些挡住「能画」

| feature | 层 | 对渲染 | 说明 |
|---|---|---|---|
| pp-01 / pp-02 | parse | 必要但不充分 | 树和 nvPr 有了；**spPr/txBody 在 pp-02 里是 deferred 子 handler** |
| pp-03 clrMap / pp-04 theme | parse | **画色必要** | scheme 色；无 theme 则纯色/默认 |
| pp-05 timing | parse | **首屏可不做** | 打开/绘制不依赖动画树 |
| pp-06..09 | parse | **闭环打开必要**；捷径可用 fixture 跳过 | 无 spPr 时打开了也画不出形状 |
| **pp-10-dml-sppr（新）** | parse | **渲染阻塞** | `ShapePropertiesHandler`：xfrm/geom/fill/ln |
| **pp-11-dml-txbody（新）** | parse | **有字才必要** | `txBody` → KmoTextBody；无字可只画矩形 |

## Feature 前缀

| 前缀 | 目录 | 对齐 writer |
|---|---|---|
| `pp-*` | `parser/features/` | `wc-*` |
| `pl-*` | `layout/features/` | `wr-01..06` |
| `pr-*` | `render/features/` | `wr-07..08` |

## 推荐顺序（能画一页矩形+字）

```text
parse 捷径：pp-01, pp-02（已有）+ 手工 shape fixture
        或闭环：+ pp-10 spPr + pp-11 txBody + pp-03/04 色 + pp-07/09 打开
layout：pl-01 → pl-02 → pl-03 → pl-04 → pl-05 → pl-06
render：pr-01 层序 → pr-02 形状指令 → pr-03 文本指令 → pr-04 Flutter
```

样例（对照 `samples/docx-mvp-render`）：`samples/ppt-mvp-render`

- 捷径：fixture slide → `render_hello_slide`
- **闭环：** `PptxReader::open_pptx_bytes` / `open_pptx_path` → `render_pptx_bytes`（最小 pptx：sldSz + slide1 spPr xfrm/solidFill + txBody）



`pp-05` 不在「能画」关键路径上。`pp-06` 只需 `sldSz` 默认值（`addDefaultProperty`）即可先画。

明细：[parser/features](./parser/features/README.md) · [layout](./layout/README.md) · [render](./render/README.md)
