# 02_function_group_child_rect — group 子级坐标映射

> feature: pl-02-xfrm-geometry ｜ 状态：**pass** ｜ Java 管线环节：S2 布局

## 0. 管线能力解锁

实现后管线新增「group 子级映射」：把子形状的相对 EMU 框按父组 `chOff/chExt` 比例 + 子级 rotation（象限宽高互换）
映进父组的渲染尺寸空间；`calcShape2TopMatrix` 自顶向下串联后，**嵌套组可画**。
没有它，组内形状坐标错乱/不可算——**group 形状断**（PIPELINE_LAYOUT_RENDER_PLAN §2.2 pl-02 `group_child_rect` 行）。

## 1. 功能说明

两个入参级别的能力（Java 查证）：

- `calcChildRelativeRect(KmoShowShape group, RectF rcGroup, RectF rcChild, float childRotation)`：
  `group` 读 `Shape.groupXfrm()`（`GroupTransform2D`）；`hasChildPoint && hasChildExtents` 时——
  ① `rcChild.offset(-chOff.x, -chOff.y)`；② `scaleX = cx==0 ? 1.0 : rcGroup.width()/cx`（Y 同）；
  ③ 中心缩放 `centerX = rcChild.centerX()*scaleX`；④ rotation 归一化到 `[0,360)` 后若在 `[45,135)∪[225,315)`
  则 `width*=scaleY; height*=scaleX`，否则 `width*=scaleX; height*=scaleY`；
  ⑤ `rcChild.set(centerX-width/2, centerY-height/2, …)`。
- `calcShape2TopMatrix(KmoShowShape shape, Matrix matrix)`：自顶向下逐层读取 `transform2D` 框，
  上一层是组（`type()==0`）时对本层做 `calcChildRelativeRect`；`matrix==null` 时只走矩形路径（MVP）。

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | `group`（`KmoShowShape type()==0`，经 `Shape.open(drawingShape()).groupXfrm()` 读 `GroupTransform2D`）、`rcGroup`（父组 slide 绝对框，只取宽高）、`rcChild`（子相对框，EMU）、`childRotation`（**度**，f32；来源 `calcRelativeRect` 的 `(int)toDegree(shapeInfoCache.rotation(shape))`）。`calcShape2TopMatrix`：`shape` + 可空 `matrix`。 |
| **输出** | `rcChild` 原地改写为组局部渲染尺寸空间的矩形；`calcShape2TopMatrix` 返回层层映射后的矩形（顶层祖先组坐标系）。EMU。 |
| **状态与副作用** | Java 用 `SlideCache.getDShape()` 借临时 `Shape`（pool，读完 `dgGroup.close()` 归还）；Rust 直读 `Shape.groupXfrm()` 不建临时对象。`matrix != null` 时走 `calcShapeMatrix`（`preTranslate/preRotate/preScale` 组合）——MVP 壳。 |
| **错误语义** | `hasChildPoint/hasChildExtents` 任一为 false → **rcChild 原样不动**（Java 查证，158-160 行条件外直接返回）；`cx==0||cy==0` → 对应 scale=1.0；rotation 先 `%360`、`<0 → +360`（Java 165-171 行）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓（保留 Java 注释+签名） | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `GroupTransform2D` | drawing/data/basic | `hasChildPoint/hasChildExtents`、`childPoint().x()/y()`、`childExtents().cx()/cy()` | ✅ `GroupTransform2D.rs`（keep，510 行已实现；`childPoint/childExtents` 懒加载分支为 `todo!`，读路径以 `&self` 已实现） | 可变写路径 | 不改 `has*` 语义 |
| `Shape.groupXfrm()` | drawing/data/shape | 组变换读取 | ✅ `Shape.rs`（keep，`groupXfrm()` 已实现） | — | 不引 `SlideCache`/pool |
| `Point2D/PositiveSize2D` | drawing/data/basic | `x()/y()`、`cx()/cy()` | ✅ keep | — | — |
| 01（shape_absolute_rect） | 本 feature | `calc_relative_rect`/`to_degree` 被调 | 01 先行/同步 | — | 不反向依赖 |
| `ShapeUtil.calcShapeMatrix` | — | `preTranslate/preRotate/preScale`（Matrix） | 壳：MVP 只走 `matrix==None` | matrix 路径延后 | 不实现 android Matrix 全量 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/presentation/core/cn/wps/show/util/ShapeUtil.java`（`calcShape2TopMatrix` 66-125、`calcShapeMatrix` 127-149、`calcChildRelativeRect` 152-185）；`drawing/data/basic/GroupTransform2D.java`（childPoint/childExtents/has*） |
| Rust 现状 | `libs/ppt-core/src/cn/wps/drawing/data/basic/GroupTransform2D.rs`（keep）、`drawing/data/shape/Shape.rs`（keep）、`show/util/`（`ShapeUtil.rs` 待新建） |

### 2.1 关键方法/字段表（Java 源码查证 → Rust 目标）

| Java 成员 | 签名/行为 | Rust 目标 | 归属 |
|---|---|---|---|
| `calcChildRelativeRect(group, rcGroup, rcChild, childRotation)` | 见 §1.1 五步 | `pub fn calc_child_relative_rect(...)` | 本 function |
| `calcShape2TopMatrix(shape, matrix)` | `shapeLink` 收集 → 自顶向下逐层映射 | `pub fn calc_shape2_top_matrix(shape, matrix: Option<...>) -> RectF` | 本 function |
| `calcShape2TopMatrix(shape, matrix, rotate, isText)` | rotate/isText 变体（文本 flip 修正） | 签名保留，`isText` 逻辑延后 | 本 function（壳变体） |
| `calcShapeMatrix(matrix, rc, rotation, flipH, flipV)` | preTranslate 半宽半高 → preRotate → preScale | MVP 不实现（`matrix==None`） | 壳 |
| `toDegree/toRotation` | `÷/×60000` | 01 内实现复用 | 01（复用） |
| `GroupTransform2D.hasChildPoint/hasChildExtents/childPoint/childExtents` | `childOffset=childPoint()`、`childSize=childExtents()` | 同名（keep） | 本 function（只读） |
| `Shape.drawingShape()/groupXfrm()` | 借临时 Shape 读组变换 | `KmoShowShape` 直读 `Shape`/`groupXfrm` | 本 function |

### 2.2 函数改写列表

- **本 function 改写：** `ShapeUtil.rs` 新建 —— `calc_child_relative_rect`、`calc_shape2_top_matrix`（`matrix=None` 路径）、rotation 归一化助手（`%360` 与负数修正）；`KmoShowShape.rs` 补 `drawingShape()/type()` 等已列 accessor。
- **停车（`todo!("DEFERRED_NON_FUNCTION")`）：** `calcShapeMatrix`（Matrix 组合，pr-02 需要旋转绘制时再开）、`calcShape2TopMatrix` 的 `isText` 分支、`calcShapeTextRect/calcRelativeTextRect/calcShapeRect`（pl-05）、`shapeSnapshot` 族（render）。
- **禁改：** `GroupTransform2D.rs`/`Shape.rs` 语义；`SlideCache`（原样不移植）；Java 锚点。

## 3. 测试方案

### 3.1 代码审计与测试面

必须通读 Java：`ShapeUtil.calcShape2TopMatrix`（66-125）+ `calcChildRelativeRect`（152-185）+ `calcShapeMatrix`（127-149）方法体；
`GroupTransform2D.java` 的 `childPoint/childExtents/hasChild*` 四方法；`SlideCache.getDShape` 借还与 `Shape.open/load/close` 生命周期（确认 Rust 可免池直读）。

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `calcChildRelativeRect` | 152-185 | rcGroup/rcChild/rotation 组合 | rcChild 改写 | 无 chOff/chExt → 原样 |
| 缩放 | 158-174 | cx=0 / 比例 | scale=1.0 / w*h 缩放 | 除零保护 |
| 象限互换 | 170-178 | 15°/45°/135°/225°/315° | 换/不换 | 半开区间 `[45,135)`、`[225,315)` |
| 负/超量 rotation | 165-171 | -45°/400° | 归一化 | `%360; <0 → +360` |
| `calcShape2TopMatrix` | 66-125 | 单层/两层/三层组 | 顶层坐标系矩形 | `isParentGroup` 首层=false |
| `calcShapeMatrix` | 127-149 | matrix 非空 | —（壳） | MVP 不测 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| C01 | 无 chOff/chExt | 失败语义 | 是 | T01 |
| C02 | 1:1 比例 | 正常 | 是 | T02 |
| C03 | 非 1:1 比例 + 非零 chOff | 正常 | 是 | T03 |
| C04 | rotation 象限互换（45/135/225/315 边界） | 分支 | 是 | T04–T07 |
| C05 | rotation 归一化（负/超 360） | 边界 | 是 | T08 |
| C06 | 两级组 `calcShape2TopMatrix` 对拍 01 | 分支 | 是 | T09 |
| C07 | `matrix=None` 路径 | 正常 | 是 | T10 |
| C08 | `calcShapeMatrix`/isText | 壳 | 否 | matrix 路径未开，不测 |
| C09 | `cx==0` scale=1.0 | 边界 | 是 | T11 |

### 3.3 可测试性

纯数值函数；fixture 只构造 `Transform2D`（off/ext/rotation）与 `GroupTransform2D`（chPoint/chExtents/has*）；
无需 `KmoShowShape` 全模型即可测 `calc_child_relative_rect`（入参即矩形）；`calc_shape2_top_matrix` 用最小父链桩。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 无 chPoint/chExtents | rcChild 不变 | 被改写 |
| T02 | C02 | chExt=(300,300)、rcGroup=300×300、子(0,0,300,300)、rot=0 | (0,0,300,300) | 缩放无中生有 |
| T03 | C03 | chOff=(10,20)、chExt=(600,600)、rcGroup=300×300 | 子(10,20,310,320)→offset→(0,0,300,300)→scale0.5→(0,0,150,150)，center 验证 | 公式错 |
| T04 | C04 | rot=15°（900000 分之一度换算） | 宽*=scaleX 高*=scaleY | 提前换挡 |
| T05 | C04 | rot=45° | 宽*=scaleY 高*=scaleX | 边界含 45→换 |
| T06 | C04 | rot=135° | 不换 | 边界 135 不含→不换 |
| T07 | C04 | rot=315° | 不换（`<315` 半开） | 边界错 |
| T08 | C05 | rot=-45° → 315°；rot=400° → 40° | 归一化后按象限判 | 取模错 |
| T09 | C06 | 两层组 fixture | 与 Java 同名 fixture 数值一致 | 层级错 |
| T10 | C07 | `matrix=None` | 只出矩形、无 Matrix 副作用 | 混入 matrix |
| T11 | C09 | chExt cx=0, cy=300 | scaleX=1.0，rcChild 宽高不变 | 除零/错误缩放 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/util/ShapeUtil.rs（loop 新建）
test:  libs/ppt-core/tests/cn/wps/show/util/ShapeUtil_group.rs
[[test]] name = "02_function_group_child_rect"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_group_child_rect

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_group_child_rect \
  --src src/cn/wps/show/util/ShapeUtil.rs \
  --report libs/ppt-core/layout/features/pl-02-xfrm-geometry/functions/02_function_group_child_rect/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_group_child_rect --json`)
date: 2026-08-30 10:27

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/util/ShapeUtil.rs` | 107/196 | 54.6% | 197/296 | 66.6% |
| **function total** | **107/196** | **54.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/util/ShapeUtil.rs \
  --java koffice/presentation/core/cn/wps/show/util/ShapeUtil.java
```

Loop 结果：**PASS**（`check-template-layout.mjs`）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/util/ShapeUtil.java \
  --rs libs/ppt-core/src/cn/wps/show/util/ShapeUtil.rs \
  --report libs/ppt-core/layout/features/pl-02-xfrm-geometry/functions/02_function_group_child_rect/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-30 02:27

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShapeUtil` | `koffice/presentation/core/cn/wps/show/util/ShapeUtil.java` | `libs/ppt-core/src/cn/wps/show/util/ShapeUtil.rs` | **PASS** | `calcShapeText2TopMatrix`, `calcShape2TopMatrix`, `calcChildRelativeRect`, `calcRelativeRect`, `toDegree`, `toRotation` | `calcShapeMatrix`, `calcRelativeTextRect`, `calcShapeRect`, `calcShapeTextRect`, `getGdValue`, `getPresetValue`, `calcShapeFlipEx`, `calcShapeFlip`, `calcShapeRotation`, `extendShapeRect`, `getTable`, `shapeSnapshot`, `textShapeSnapshotWithRelativeInfo`, `textShapeSnapshot`, `calcMinFullDrawShapeRect`, `setInkTransparent`, `slideSnapshot`, `adjustCanvas`, `setGlCachEnable`, `invertCoordFor3dShape`, `isGifShape`, `isTable`, `calVirtualGroupRect`, `calVirtualGroupBound`, `degree2Radian`, `isSameArrayShape` | — | **PASS** |
| `RoundCornerCLipSizeType` | `koffice/presentation/core/cn/wps/show/util/ShapeUtil.java` | `libs/ppt-core/src/cn/wps/show/util/ShapeUtil.rs` | **PASS** | — | `RoundCornerCLipSizeType` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->