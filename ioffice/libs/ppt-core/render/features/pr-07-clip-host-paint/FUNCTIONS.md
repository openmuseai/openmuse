# pr-07-clip-host-paint — function 清单

> 状态：**pass** ｜ 管线：[SKELETON_PIPELINE.md](../../../SKELETON_PIPELINE.md)

## §1 清单

| order | function_id | I | O | crate | status |
|---|---|---|---|---|---|
| 1 | [01_function_clip_slide_bounds](./functions/01_function_clip_slide_bounds/FUNCTION.md) | cmds + slide size | 可视在 slide 内 | libs/ppt-core | pass |
| 2 | [02_function_bodypr_vert](./functions/02_function_bodypr_vert/FUNCTION.md) | bodyPr vert | 竖排 DrawText | libs/ppt-core | pass |
| 3 | [03_function_flutter_no_default_stroke](./functions/03_function_flutter_no_default_stroke/FUNCTION.md) | 空 fill/stroke | 不画黑框 | samples/libs/ppt/ppt_render | pass |

## §5 loop

`01 → 02 → 03`

## 文档

- [functions.csv](./functions.csv)
