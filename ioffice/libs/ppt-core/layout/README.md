# PPT Layout → 几何与文本框排版

> 对照：[word-core/layout/README.md](../../word-core/layout/README.md)  
> 前置：至少 pp-01/pp-02 + **可查询的 xfrm/文本**（闭环还要 pp-10/pp-11）  
> 内核禁止 `android.graphics.Canvas`。本层输出是 **几何 + typo 结果**，不是像素。

## 一句话

```text
KmoSlide / KmoShowShape（parse 模型）
  → [pl-01] EMU + slideSize → layout/render 坐标系
  → [pl-02] Transform2D → 绝对 RectF（含 group 子级）
  → [pl-03] PlaceHolderMerger 继承母版/版式 xfrm/fill/style
  → [pl-04] shape → TextBoxData
  → [pl-05] 框内断行（MVP 贪心，≠ Java ShowRichTextLayout）
  → [pl-06] ShowTypoShape 快照（hidden/ph/geometry/children）
```

Writer 把「页/行/run」放进 TypoSnapshot；PPT 的「页」是 **一张固定尺寸的 slide**，没有正文流分页。  
PPT layout 的核心 I/O 是 **每个 shape 的框** 和 **框内文字**，外加 **master/layout/slide 三层谁画**（层序在 render pr-01）。

## feature 清单

见 [features/README.md](./features/README.md) 与 [features/manifest.csv](./features/manifest.csv)。

## 与 parse 的命名冲突

| 词 | 层 | Java |
|---|---|---|
| SlideLayout / `p:sldLayout` | **parse** | 版式 part（pp-01 06 / pp-07） |
| ShowTextLayout / typo | **layout** | 文本在形状内排版 |
| LayoutRender | **render** | 把 KmoSlideLayout 画到 Canvas（预览），不是本目录算法 |
