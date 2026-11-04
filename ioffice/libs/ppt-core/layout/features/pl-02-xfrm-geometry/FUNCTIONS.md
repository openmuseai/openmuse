# pl-02-xfrm-geometry — function 清单

> 层：layout ｜ 状态：**pass** ｜ crate：`libs/ppt-core`

## §0 管线位置（S2 内数据流）

S2 布局的环节①「几何（绝对 rect）」。数据流：

```text
KmoShowShape（Transform2D.off/ext + rotation，EMU）
   │  ShapeInfo 语义直读（ShapeInfoCache.transform2D = Transform2D.point/extents，Java 查证）
   ├─[01 shape_absolute_rect]→ 单 shape RectF
   │        │（parentGroup 非空）
   │        └─[02 group_child_rect]← calcShape2TopMatrix(parent) 取组宽高
   └─[02] recursion → group 局部渲染尺寸空间的子级 RectF
                                            │
                   pl-06（ShowTypoShape 快照）/ pr-02（rect→指令）◄──┘
```

单位全程 EMU；`kOffice S1 convert` 的 `ShapeInfoCache` 是性能缓存，Rust 首版直读模型（语义等价，`ShapeInfo.reset` 即
`transform2D().point()/extents()/rotation()` + `shape.canRotate()`，Java 查证）。

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream |
|---|---|---|---|---|---|---|
| 1 | [01_function_shape_absolute_rect](./functions/01_function_shape_absolute_rect/FUNCTION.md) | 单 shape 绝对 EMU 框 | `KmoShowShape`（`Transform2D` off/ext + `canRotate`/rotation） | `calcRelativeRect` 语义 RectF（无父=slide 绝对；有父=组局部） | pl-01（单位语义）；pp-02（模型）；pp-10（xfrm，或 fixture） | 02；pl-06；pr-02 |
| 2 | [02_function_group_child_rect](./functions/02_function_group_child_rect/FUNCTION.md) | group 子级坐标映射 | `group` + `rcGroup` + `rcChild` + childRotation（度） | `calcChildRelativeRect` 语义 rcChild（offset→scale→center→象限互换） | 01（父组 rect） | 01（递归）；pl-06；pr-02 |

## §2 数据流

```text
calcRelativeRect(shape, rc):
  1. rc = transform2D()（ShapeInfo 语义直读）
  2. parentGroup != None?
       rotation = canRotate? toDegree(rotation) : 0        （60000 分之一度 → 度）
       rcGroup  = calcShape2TopMatrix(parent, null)        （[02]）
       calcChildRelativeRect(parent, rcGroup, rc, rotation)（[02]）
  3. 返回 rc

calcShape2TopMatrix(shape, matrix=None):
  shapeLink = [shape, parentGroup, …, top]
  顶层 → 底层逐层: rc = 该层 transform2D 框
    isParentGroup(上一层 type==0) → calcChildRelativeRect(其 parentGroup, rcGroup, rc, rotation)
  matrix==None 时只走矩形路径（MVP 轴对齐）
```

无环；02 向上层递归经 01 读取父组 rect，向下（多级组）靠 `calcShape2TopMatrix` 自顶向下串联。

## §3 测试面

- **01**：无组单 shape；组直子；两级组嵌套；`calcSelf=false`（`rcShape` 已非空时跳过自身计算）；rotation 读取与 `canRotate=false` 路径；fixture `Transform2D` 无 xfrm（全 0）输入。
- **02**：`hasChildPoint/hasChildExtents` 缺失 → rcChild 原样；`cx==0/cy==0` → 缩放取 1.0（Java `cx==0 ? 1.0F : …`）；比例缩放；四象限 rotation（45/135/225/315 边界，Java 用 `>=/<` 半开区间）；负 rotation 归一化（`%360` 后 `<0 → +360`）。

## §4 验收

```text
01：无组 (100,200,300,400)→(100,200,400,600)；fixture 对拍 Java calcRelativeRect
02：1:1 组→(0,0,300,300)；scale 0.5→(0,0,150,150)；15° 不换挡/45° 换挡；无 chExt→原样
+ calcShape2TopMatrix(parent, null) 与 calcRelativeRect 数值一致（两层组）
```

## §5 loop 顺序

```text
01_function_shape_absolute_rect（单 shape 框 + transform2D 直读 + KmoShowShape 缺失 accessor 补桩）
02_function_group_child_rect（依赖 01 父组 rect + GroupTransform2D/Shape.groupXfrm 只读）
```

## §6 Java/Rust 范围

| 层 | Java | Rust |
|---|---|---|
| 01 | `show/util/ShapeUtil.java` 186-209(`calcRelativeRect`)、455-462(`toDegree/toRotation`)；`render/cache/ShapeInfoCache.java`(transform2D 113-132 / canRotate&rotation 33-63)；`render/cache/ShapeInfo.java`（reset） | **新建** `src/cn/wps/show/util/ShapeUtil.rs`（审计行 665 `add, MISSING`，loop 模板起自 Java 全量）；只读 `show/app/shape/KmoShowShape.rs`（现 12 个 todo 桩，`transform2D/canRotate/parentGroup/type` 需 loop 补）；`drawing/data/basic/Transform2D.rs`/`Point2D.rs`/`PositiveSize2D.rs`（keep） |
| 02 | `show/util/ShapeUtil.java` 66-125(`calcShape2TopMatrix`)、127-149(`calcShapeMatrix`)、152-185(`calcChildRelativeRect`)；`drawing/data/basic/GroupTransform2D.java` | 同上 `ShapeUtil.rs`；只读 `drawing/data/basic/GroupTransform2D.rs`（keep，childPoint/childExtents/has* 已实现）、`drawing/data/shape/Shape.rs`（keep，`groupXfrm()` 已实现） |

> 注：Rust 无 `android.graphics.RectF`——layout 层定义最小 `RectF`（f32 left/top/right/bottom；EMU 整数上也以 f32 承载，与 Java `RectF(float)` 一致），放 `ShapeUtil.rs` 同模块；`calcShapeMatrix` 的 `Matrix` 组合为壳，MVP 只走 `matrix=None` 路径。