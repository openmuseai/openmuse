# PPT 像素对齐管线：下一波可验收 feature

> 日期：2026-09-15  
> 权威拆解：[rewrite-review/06-next-features.md](./rewrite-review/06-next-features.md)  
> 上一波（已 pass，不重拆）：[SKELETON_PIPELINE.md](./SKELETON_PIPELINE.md)

目标：产线结构对齐 Java **S4 Kmo 树 → S5 公开 Shape → S9 胶片 → S10 像素**，再用金页 RMSE 验收。

```text
.pptx
  → [pp-20] 打开写入 app::Kmo 树（替换 layout_shape 产线）     ★ S4 产线
  → [pr-08] getShape() → DrawPath/Fill/Line                     ★ S5→S9
  → [pr-09] DrawFlags 接到 render_slide                         ★ S9 过滤
  → [pl-10] G6 语义（anchor/ph/舍入/canRotate）                 ★ S6/S7
  → [pl-11] ShapePr.follow                                      ★ S6
  → [pp-21] showMasterSp 母版胶片                               ★ S9
  → [pl-12] DataTransform 活路径（bullet/行距/rtl）             ★ S7
  → [pr-10] 行/run DrawText                                     ★ S9 文本
  → [pr-11] 线宽线色                                            ★ S9 描边
  → [pr-12] Flutter 解码 JPEG/PNG                               ★ S10 G9
  → [pr-13] 金页像素门闩                                        ★ 验收
```

拆 function / loop：对单个 `feature_id` 说「拆 function」或 `loop <function_id>`，走 feature-function + function-java2rust。

2026-09-15：11 条 FEATURE.md / FUNCTIONS.md / functions.csv 已落盘（**spec**）。跟踪表见 [06-next-features.md §8](./rewrite-review/06-next-features.md)。
