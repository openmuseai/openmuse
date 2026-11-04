# pl-08-group-abs-geom — 组内子级绝对框 + film 子级

> 层：layout ｜ 状态：**pass** ｜ depends_on: pl-02; pl-07; pp-13:02  
> 产品 I/O：把 Java `GroupTransform2D` 的 **chOff/chExt** 写进产品树，`resolve_pages` 调用已 pass 的 `ShapeUtil::calcChildRelativeRect`，把 **slide 绝对 EMU** 写回 `transform2D*`，并且 `SlideRender::film_shapes` **递归 film 组内子 shape**。

## 目标

组内形状用组局部坐标；当前 `parse_one_grp` 不写 `child_off`/`child_ext`，`film_shapes` 只扫 `sp_tree` 顶层。Sample 第 2 页组：`off=(2447933,1269734)` 而 `chOff=(5143500,2476500)`。

## I/O

| 输入 | 输出 |
|---|---|
| `p:grpSpPr/a:xfrm` 的 `a:off/a:ext/a:chOff/a:chExt` | group.`transform2D*` + `child_off`/`child_ext` |
| `resolve_pages` 深度优先 + parent 链 | 每个 **非组** child 的 `transform2D*` = slide 绝对 EMU |
| `KmoShapeTree` 含 `shape_type==0` 的组 | `layer_order` 对每个可见子 shape 一条 `LayerItem::Shape`（不把组容器当唯一可画框） |

不重写 `ShapeUtil` 算法。Java `calcChildRelativeRect` 结果在组局部渲染空间（只用 `rcGroup` 宽高缩放）；产品指令无 canvas translate → writeback = 该局部框 **加上组绝对 origin**。不处理 3D / 任意旋转矩阵（轴对齐 MVP）。
