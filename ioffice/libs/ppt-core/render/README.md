# PPT Render → RenderCommand → 绘制

> 对照 writer wr-07 / wr-08。  
> 前置：pl-06 `ShowTypoShape`（或 fixture 几何）。  
> **禁止** 把 `android.graphics.Canvas` / `Paint` 作为内核输出。Java `ShapeRender` / `IRenderDrawer` 只作语义来源。

## 一句话

```text
ShowTypoShape[]（按 master → layout → slide 层序）
  → [pr-01] 层过滤（drawFlags / hidden / miner placeholder）
  → [pr-02] 形状：白底、背景 fill、预设几何 fill/line → DrawRect/DrawPath
  → [pr-03] 文本：行框 → DrawText
  → [pr-04] JSON/UniFFI → Flutter Canvas
```

Java `SlideRender.drawUnderSlide` 的层序必须钉死，否则母版形状会画在幻灯片之上或之下。

## MVP 不做

- Chart / Table / SmartArt / OLE / Model3D（`CreateGraphicFrameRender`）
- 效果层、3D、GlCache
- 动画 InterpretCtx 滤镜
- `LayoutRender` 版式预览专用 flags（可复用 pr-01 层序测 layout 作为 ISlide）

图片：第一波可用 **占位矩形**；真实 blip 为 post-MVP（依赖 parse media）。
