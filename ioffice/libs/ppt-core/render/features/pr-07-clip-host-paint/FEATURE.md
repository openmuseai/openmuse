# pr-07-clip-host-paint — clip、竖排、宿主不默认黑描边

> 层：render + Flutter ｜ 状态：**pass** ｜ depends_on: pr-04; pr-06  
> 布局收口：负坐标不画进宿主侧栏；文本在框内竖排。宿主：空 fill 不描黑框。

## I/O

| 输入 | 输出 |
|---|---|
| envelope + slide w/h | 绘制限制在 `[0,w]×[0,h]`（指令裁剪或 painter clipRect） |
| `a:bodyPr vert="eaVert"\|"vert"` | DrawText 在框内竖排（骨架级：逐字或旋转盒） |
| DrawRect/Path 无 fill 无 stroke | painter **不**画默认黑框 |

不修 Zoom 控件。SVG skip。
