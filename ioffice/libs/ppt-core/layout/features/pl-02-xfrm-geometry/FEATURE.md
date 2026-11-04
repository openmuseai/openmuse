# pl-02-xfrm-geometry — 形状绝对几何

> 层：**layout** ｜ 状态：**pass** ｜ Java：`cn.wps.show.util.ShapeUtil`（`calcRelativeRect` / `calcShape2TopMatrix` / `calcChildRelativeRect`）＋ `cn.wps.show.render.cache.ShapeInfoCache.transform2D` ＋ `cn.wps.drawing.data.basic.Transform2D`

## 目标

把每个 `KmoShowShape` 的 `Transform2D`（`off`+`ext`，EMU）变成 **slide 上的绝对 EMU 矩形**（含 group 子级经
`chOff/chExt`+子级 rotation 的坐标变换）。这是「画在哪儿」的答案，S4 之前的唯一几何来源。

## I/O（锁死）

| 输入 | 输出 |
|---|---|
| `KmoShowShape` + 出参 `RectF`（`ShapeUtil.calcRelativeRect(shape, rcShape[, calcSelf])`） | shape 的 EMU `RectF`：单 shape 为自身 `Transform2D` 框（`left=transform2DX, top=transform2DY, right=left+transform2DW, bottom=top+transform2DH`，无父时即 slide 绝对；Java 查证 `calcRelativeRect`）。有 `parentGroup` 时：父组经 `calcShape2TopMatrix` 得 slide 绝对框（**只取宽高**），子经 `calcChildRelativeRect` 映射进**组局部坐标的渲染尺寸空间**（组位置不叠加——Java 语义，`calcChildRelativeRect` 只用到 `rcGroup.width()/height()`） |
| `KmoShowShape`（含父链 `parentGroup()`，`type()==0` 为组）+ 出参矩阵 | `calcShape2TopMatrix(shape, matrix)`：自顶向下逐层走 `shapeLink`——首层 rcShape=顶层形状自身 transform2D 框；`isParentGroup`（上一层为组）时 `calcChildRelativeRect(parentGroup(), rcGroupShape, rcShape, rotation)`，两层以上递归叠加；返回值为**顶层祖先组坐标系**下的矩形（Java 查证，`rotate=true` 走 `ShapeInfo.rotation/canRotate`） |
| `group` + `rcGroup` + `rcChild` + 子级 rotation（度） | `calcChildRelativeRect(group, rcGroup, rcChild, childRotation)`：`rcChild.offset(-chOff)` → 按 `chExt` 比例缩放（`scaleX=rcGroup.width()/cx`，`cx==0→1.0`）→ 中心缩放 → rotation 在 `[45,135)∪[225,315)` 时宽高缩放互换 → 重设 `rcChild`（Java 查证） |
| `Transform2D.rotation()` | 60000 分之一度（int）；`ShapeUtil.toDegree(r)=r/60000f`、`toRotation(deg)=deg*60000` |

输出全程 **EMU**（`RectF`），到 S6 前不换单位（pl-01 负责换算）；`calcShapeMatrix` 的 `Matrix` 组合（rotate/flip 到 Canvas）
不是输出主体——MVP 只出轴对齐框，rotation 以 `canRotate` 场景参与 `calcChildRelativeRect` 的宽高互换（Java 语义）。

## Java 管线对照

S2 布局的环节 ①「几何（绝对 rect）」：`ShapeInfoCache` 缓存每个 shape 的 `transform2DX/Y/W/H/rotation`（语义即 `Transform2D.point()/extents()/rotation()`，`ShapeInfo.reset` 查证），
`ShapeUtil.calcRelativeRect` → 单 shape 绝对框；父为 group 时递归 `calcShape2TopMatrix` + `calcChildRelativeRect`。

| Java 类 | 方法/字段 | 对应 Rust 目标 | 环节 |
|---|---|---|---|
| `ShapeUtil` | `calcRelativeRect(KmoShowShape, RectF[, boolean])`（`show/util/ShapeUtil.java` 186-209 行） | `src/cn/wps/show/util/ShapeUtil.rs`（审计行 665：`add, MISSING`，loop 新建） | pl-02 01 |
| `ShapeUtil` | `calcShape2TopMatrix(KmoShowShape, Matrix)`（66-125 行）、`calcChildRelativeRect`（152-185 行）、`calcShapeMatrix`（127-149 行）、`toDegree/toRotation`（455-462 行） | 同上 `ShapeUtil.rs` | pl-02 02 |
| `ShapeInfoCache` | `transform2D(KmoShowShape, RectF)`（render/cache/ShapeInfoCache.java 113-132 行）、`canRotate/rotation`（33-63 行） | Rust 无对应文件；语义与 `KmoShowShape.transform2D()` 直读等价（`ShapeInfo.reset` 即 point/extents/rotation），MRU 缓存为性能层，Rust 首版直读 | pl-02 01/02 |
| `ShapeInfo` | `transform2DX/Y/W/H`、`rotation`、`canRotate`（render/cache/ShapeInfo.java） | 同上（直读替代） | pl-02 01/02 |
| `Transform2D` | `point().x()/y()`、`extents().cx()/cy()`、`rotation()`（drawing/data/basic/Transform2D.java） | `drawing/data/basic/Transform2D.rs`（keep，687 行已实现）+ `Point2D.rs`/`PositiveSize2D.rs` | pl-02 01 |
| `GroupTransform2D` | `hasChildPoint/hasChildExtents`、`childPoint()`、`childExtents()` | `drawing/data/basic/GroupTransform2D.rs`（keep，510 行已实现） | pl-02 02 |
| `SlideCache.getDShape()` | 临时 `Shape`（组信息读取，pool） | Rust 直读 `Shape.groupXfrm()`（`Shape.rs` keep，已实现） | pl-02 02 |

## 能力解锁

- **01 `shape_absolute_rect`**：形状在 slide 上的绝对 EMU 框 → 「画在哪儿」；缺失时 S4 无框可绘，**必断**（渲染断点表 §3.2）。
- **02 `group_child_rect`**：group 子级绝对坐标（含 `chOff/chExt` 比例与子级 rotation 的宽高互换）→ 嵌套组可画。

## 必须 / 壳 / 延后

### 必须
- `calcRelativeRect` 全语义（含 `calcSelf` 参数、`parentGroup` 分支、rotation 的 `toDegree` 换算）；
- `calcChildRelativeRect` 全语义（offset → scale → center → rotation 象限互换）；
- `rotation` 读取与 `canRotate` 判定（60000 分之一度 → 度）。

### 壳
- `calcShapeMatrix`（`Matrix` rotate/flip 组合）：轴对齐框先行；flip/旋转**画**仍按轴对齐，`calcShape2TopMatrix(matrix=null)` 不需要它；
- `calcRelativeTextRect`/`calcShapeTextRect`/`calcShapeRect`（文本区与预设几何内嵌 rect）：归 pl-05/pr-02，本 feature 只留签名。

### 延后
- `extendShapeRect`（线宽膨胀 101600 EMU 下限）：画线装饰用，pr-02 需要时再开；
- MRU 缓存/`ThreadLocal` 性能层：Rust 直读模型，不做缓存。

## 范围

本 feature 只算 **EMU 矩形**（shape 自身 + group 子级递归）。不包含：单位换算（pl-01）、placeholder 继承（pl-03）、
文本排版、任何 `android.graphics.Canvas`/`Paint` 语义、flip/rotation 的矩阵绘制。

## 验收

1. 无组 shape：`Transform2D(off=(100,200), ext=(300,400))` → `calcRelativeRect` 得 `(100,200,400,600)`；
2. 比例 1:1 单层组（`chExt=(300,300)`、`rcGroup` 宽高均为 300）：子 `(0,0,300,300)` → `(0,0,300,300)`（组局部缩放空间；组位置不叠加——Java `calcChildRelativeRect` 只取 rcGroup 宽高）；
3. 组比例缩放：`chExt=(600,600)` 而 `rcGroup` 宽高 300（scale 0.5）：子 `(0,0,300,300)` → `(0,0,150,150)`；子 `(300,300,300,300)` → `(150,150,300,300)`；
4. rotation（60000 分之一度）语义：`900000→15°` 不触发换挡；`2700000→45°` 进 `[45,135)` → 宽/高缩放互换（`width*=scaleY, height*=scaleX`）；`toDegree(r)=r/60000f`、`toRotation(deg)=deg*60000` 对拍；
5. 对拍 Java：同一 `KmoShowShape` fixture 下 `calcRelativeRect` 与 `calcShape2TopMatrix(parent, null)` 数值一致（含两层组嵌套）；
6. 全程 EMU，未过 pl-01 `render().fromEmu` 前不做任何单位换算。

## 依赖

- 前置 feature：`pl-01-units-view`（单位语义）；manifest 另列不强依赖其它（捷径轨可 fixture `Transform2D`）。
- 必读 Java：`…/show/util/ShapeUtil.java`（66-209、455-462 行）；`…/show/render/cache/ShapeInfoCache.java`（全量）；`…/show/render/cache/ShapeInfo.java`（全量）；`…/drawing/data/basic/Transform2D.java`（point/extents/rotation 段）；`…/drawing/data/basic/GroupTransform2D.java`（childPoint/childExtents/has*）；`…/drawing/app/graphic/GraphicObject.java` 不需要。
- Rust 现状：`drawing/data/basic/Transform2D.rs`（keep，`point()/extents()/rotation()/isFlipHorizontal…` 已实现）、`GroupTransform2D.rs`（keep，`childPoint()/childExtents()/hasChild*` 已实现）、`drawing/data/shape/Shape.rs`（keep，`groupXfrm()` 已实现）、`show/app/shape/KmoShowShape.rs`（**todo 桩**：`transform2D()/canRotate()/parentGroup()/type()` 缺失 → loop 补）。

## 不过扩展

- 不实现 `android.graphics.Matrix` 全语义（只做 `calcChildRelativeRect` 需要的数值）；
- 不把 flip/rotation 做成输出（MVP 轴对齐框）；
- 不搬 `ShapeInfoCache` 的 MRU/`ThreadLocal` 性能设施；
- 不做 group 的 `calcShapeText2TopMatrix`/文本矩阵（pl-06/pl-05）。