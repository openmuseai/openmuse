# pl-08-group-abs-geom — function 清单

> 层：layout ｜ 状态：**pass** ｜ crate：`libs/ppt-core`  
> 管线：[SKELETON_PIPELINE.md](../../../SKELETON_PIPELINE.md)

## §0 管线位置

S2 布局：组坐标空间 → slide 绝对框 → 渲染层列表。数据流：

```text
p:grpSp / grpSpPr / a:xfrm
   ├─[01 grp_choff_chext]→ group.child_off/child_ext + 自身 off/ext
   │
   ├─[02 child_abs_rect]→ ShapeUtil.calcChildRelativeRect
   │                      + 组 origin → 子 transform2D*（slide 绝对 EMU）
   │
   └─[03 film_group_children]→ layer_order 递归 children
                                （跳过 type==0 容器本身的空 AABB）
                                      │
                   pr-06 / pp-18 / pr-02 ◄── 绝对框
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_grp_choff_chext](./functions/01_function_grp_choff_chext/FUNCTION.md) | XML chOff/chExt 落到组 | grpSpPr xfrm XML | `child_off`/`child_ext`；自身 `transform2D*` | pp-13:02 | 02 | pass |
| 2 | [02_function_child_abs_rect](./functions/02_function_child_abs_rect/FUNCTION.md) | 子级 slide 绝对框写回 | group+children（已有 chOff） | 子 `transform2D*` 为 slide 绝对 EMU | 01; pl-02:02; pl-07 | 03; pr-06 | pass |
| 3 | [03_function_film_group_children](./functions/03_function_film_group_children/FUNCTION.md) | 层列表含组内 shape | `KmoShapeTree` | `LayerItem::Shape` 按子级（深度优先） | 02 | pr-06; pp-18 | pass |

## §2 数据流

```text
Java GroupTransform2DHandler.enterSubElement:
  1114200 off → xfrm.point
  1114197 ext → xfrm.extents
  1114204 chOff → groupXfrm.childPoint
  1114205 chExt → groupXfrm.childExtents

产品现状：element_id 无 chOff/chExt；parse_one_grp 用 ShapePropertiesHandler；
commit 不写 child_*；film_shapes 不递归。

calcRelativeRect(child):
  rc = child off/ext
  parent != null → calcChildRelativeRect(group, rcGroupAbs, rc, rot)
  产品 writeback: child.transform2D* = rc offset(group.abs.left, group.abs.top)
```

无环：01 只解析；02 只布局写回；03 只 film。

## §3 测试面

crate：`libs/ppt-core`。内存 XML / fixture `KmoShowShape`，不跑 Flutter、不跑 23 页 Sample 作为唯一用例。

| function_id | tests path | cargo |
|---|---|---|
| 01 | `tests/cn/wps/show/pptxr/01_function_grp_choff_chext.rs` | `--test 01_function_grp_choff_chext` |
| 02 | `tests/cn/wps/show/layout/02_function_child_abs_rect.rs` | `--test 02_function_child_abs_rect` |
| 03 | `tests/cn/wps/show/render/03_function_film_group_children.rs` | `--test 03_function_film_group_children` |

## §4 验收

Sample 第 2 页 dump：装饰色块 `x>=0` 且 `x+w` 不超过 slide 宽（1px 误差）；Shape 指令数明显多于「每组一个框」。CONTENTS 负 x 归 pr-07。

## §5 loop

```text
01_function_grp_choff_chext
02_function_child_abs_rect
03_function_film_group_children
```

## §6 Java/Rust 范围

| 层 | Java | Rust |
|---|---|---|
| 01 | `drawing/dmlr/handler/shape/sppr/transform/GroupTransform2DHandler.java`；`GroupTransform2D.java` childPoint/childExtents | `Transform2DHandler.rs`（`GroupTransform2DHandler`）；`pptx_mvp.rs` `element_id`/`parse_one_grp`；`layout_shape.rs` `child_off` |
| 02 | `show/util/ShapeUtil.java` `calcChildRelativeRect` 152-185 | **只调用** `ShapeUtil.rs`；写回在 `page_inheritance.rs` |
| 03 | `show/render/SlideRender.java` 遍历 spTree（含组递归语义） | `SlideRender.rs` `film_shapes` |

## 文档

- [functions.csv](./functions.csv)
