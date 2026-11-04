# PPT render features（pr-0X）

> emit 与 paint 分开，才能在没有 Flutter 时测指令 JSON。每 feature 已拆 function（`functions.csv` + `functions/<id>/FUNCTION.md`，全部 spec，loop 未开始）。  
> 管线定位见 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S3..S6；每 function 的 Java 管线对照与「能力解锁」见各自 FUNCTION.md §0。

## 清单

| feature_id | I → O | 对齐 writer | Java 锚点 |
|---|---|---|---|
| pr-01-slide-layers | slide + layout + master + flags → 有序 `ShowTypoShape` 列表 | 无（PPT 三层胶片） | `SlideRender.drawUnderSlide` + `drawFilm` |
| pr-02-shape-command | TypoShape 几何+fill/ln → RenderCommand | wr-07 DrawRect/Path | `ShapeRender._begin` + DrawingRender 语义 |
| pr-03-text-command | IShowTextLayout 行 → DrawText | wr-07 DrawText | `TextRender` / `ITextRenderModel` |
| pr-04-ffi-flutter | commands → 屏幕 | wr-08 | 不进 ppt-core 热路径也可另包 |
| [pr-05-visual-emit-paint](./pr-05-visual-emit-paint/FEATURE.md) | DrawImage + 实色 fill + Flutter 位图 | wr-07 DrawImage | `GemoRender.drawFill` BlipFill |

## loop 序

`pr-01 → pr-02 → pr-03 → pr-04`（已 pass）  
下一波：`pr-05`（见 [VISUAL_CONTENT_PIPELINE.md](../../VISUAL_CONTENT_PIPELINE.md)）

允许：pr-04 用 **手工 fixture 指令** 先画白底+一个矩形（不阻塞 pr-01）。  
不允许：pr-02 在 `_begin` 过滤未测时把 hidden 形状画出来当绿。
