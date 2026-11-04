# PPT layout features（pl-0X）

> 切 **I/O**，不按 `show/layout` 包切。每 feature 已拆 function（`functions.csv` + `functions/<id>/FUNCTION.md`，全部 spec，loop 未开始）。  
> 管线定位见 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S2；每 function 的 Java 管线对照与「能力解锁」见各自 FUNCTION.md §0。  
> MVP 可画：固定 slide 尺寸上的矩形 + 框内贪心断行。**不是** Java `ShowRichTextLayout` 对齐。

## 清单

| feature_id | I → O | 对齐 writer | Java 锚点 |
|---|---|---|---|
| pl-01-units-view | slideWidth/Height EMU + 缩放 → 布局/绘制单位 | wr-02 ViewEnv | `UnitsConverter.layout/render/doc` |
| pl-02-xfrm-geometry | `Transform2D` + 父 group → 绝对 `RectF` | wr-01 几何可查询 | `spPr.xfrm`；group child |
| pl-03-ph-merge | placeholder shape + layout/master 树 → 合并后的 xfrm/fill/ln/style/txLstStyle | 无直接对应（PPT 特有） | `PlaceHolderMerger` |
| pl-04-text-box | `KmoShowShape` + `KmoContext` → `TextBoxData` | wr-09 run 流的「取出可排内容」 | `ShapeDataTransform.exToInternal` |
| pl-05-text-typo | `TextBoxData` + 框宽高 → 行框列表（贪心） | wr-04 greedy wrap | `ShowTextLayout.layout` / `ShowRichTextLayout.typo`（MVP ≠ Java） |
| pl-06-typo-shape | shape + geometry + flags → `ShowTypoShape` | wr-06 snapshot 组装 | `ShowTypoShape` ctor |
| [pl-07-ph-content-apply](./pl-07-ph-content-apply/FEATURE.md) | 写回 mergeShapePr + 空 ph 文本 | PPT 特有 | `PlaceHolderMerger` 应用侧 |

并行：pl-01 ∥ pl-02（无 ph 时）。  
禁止：pl-05 用假宽度假装断行正确（可用固定宽度 **fixture**，FUNCTION.md 必须写明）。

## 延后（非「能画一页」）

- 表格单元格 `CellDataTransform`、SmartArt、3D、墨迹
- Java 精确 `ShowRichTextLayout` / autofit 连锁（第二波，对标 wr-12）
- 域：页码/日期 `resetSlideNumFieldTextRuns`（可先忽略，字画源文本）

## loop 序

`pl-01 → pl-02 → pl-03 → pl-04 → pl-05 → pl-06`（已 pass）  
下一波：`pl-07`（见 [VISUAL_CONTENT_PIPELINE.md](../../VISUAL_CONTENT_PIPELINE.md)）
