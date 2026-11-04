# 01_function_shape_absolute_rect — 单 shape 绝对 EMU 框

> feature: pl-02-xfrm-geometry ｜ 状态：**pass** ｜ Java 管线环节：S2 布局

## 0. 管线能力解锁

实现后管线新增「单 shape 框查询」：给定 `KmoShowShape` 即得其 EMU 矩形（无父 = slide 绝对；有父 = 组局部渲染尺寸空间）。
没有它，「画在哪儿」无答案，S4（pr-02）无框可绘——**必断**（PIPELINE_LAYOUT_RENDER_PLAN §3.2 渲染断点 table）。

## 1. 功能说明

Java `ShapeUtil.calcRelativeRect(KmoShowShape, RectF)`（2 参 = calcSelf=true）：
① 经 `ShapeInfoCache.transform2D(shape, rc)` 读 `ShapeInfo.transform2DX/Y/W/H`（语义 = `Transform2D.point()/extents()`，`ShapeInfo.reset` 查证）；
② 若 `parentGroup() != null`：`rotation = shapeInfoCache.canRotate(shape) ? toDegree(shapeInfoCache.rotation(shape)) : 0`，
`rcGroup = calcShape2TopMatrix(parent, null)`（[02]），`calcChildRelativeRect(parent, rcGroup, rc, (int)rotation)`（[02]）。
Rust 首版直读模型（免 MRU 缓存）。

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | `KmoShowShape`（Java 查证字段来源：`Transform2D.point()→x/y`、`extents()→cx/cy`、`rotation()→60000 分之一度`、`canRotate()`、`parentGroup()`、`type()`）。`calcSelf` 参数（`calcRelativeRect(shape, rc, false)`：`rcShape` 非空时跳过自身计算，仅做父组合并——Java 191 行 `if (calcSelf || rcShape.isEmpty())`）。 |
| **输出** | `RectF`（EMU，f32）：自身框 `(x, y, x+cx, y+cy)`；有 `parentGroup` 时经 02 映射为组局部渲染尺寸空间（组 slide 位置不叠加——Java `calcChildRelativeRect` 只用 `rcGroup.width()/height()`）。 |
| **状态与副作用** | 只读；不写缓存、不建 shape、不改模型。`ShapeInfoCache`/`ShapeInfo` 在 Java 是查询/缓存层，Rust 以 `KmoShowShape` accessor 直读等价替换（`ShapeInfo.reset` 语义，Java 查证）。 |
| **错误语义** | Java 无显式异常：shape 无 xfrm → `spPr.xfrm()` 返回空 `Transform2D`（off/ext 全 0）→ rect `(0,0,0,0)`；`canRotate=false` → rotation=0，不进象限换挡；`parentGroup()` 链到顶层为止（`tmpShape.parentGroup()==null` 停，`calcShape2TopMatrix` 66-125 行查证）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓（保留 Java 注释+签名） | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `Transform2D` | drawing/data/basic | `point().x()/y()`、`extents().cx()/cy()`、`rotation()` | ✅ `Transform2D.rs`（keep，已实现） | — | 不删字段、不动 kproto 语义 |
| `KmoShowShape` accessor | show/app/shape | `transform2D()/canRotate()/parentGroup()/type()/id()` | ⚠ 默认面用 `layout_shape.rs` fixture（off/ext/rotation/parent/ch*）；全量 `KmoShowShape.rs` 仍 gated | 不展开 `show/app` | 不重构 KmoAtom 存储 |
| `ShapeInfo` 语义 | render/cache | `transform2DX/Y/W/H`、`rotation`、`canRotate`（`reset`） | 无 Rust 文件——以 `transform2D()` 直读等价 | MRU/ThreadLocal 缓存 | 不搬缓存设施 |
| `RectF` | 平台 | f32 left/top/right/bottom | 无——layout 层新建最小 RectF（同 `ShapeUtil.rs` 模块或新建） | — | 不依赖 android |
| 02（group_child_rect） | 本 feature | `calcShape2TopMatrix`/`calcChildRelativeRect` | 02 先行/同步 | — | 不把 02 逻辑内联进 01 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/presentation/core/cn/wps/show/util/ShapeUtil.java`（`calcRelativeRect` 186-209；`toDegree/toRotation` 455-462）；`render/cache/ShapeInfoCache.java`（`transform2D` 113-132、`canRotate` 49-63、`rotation` 33-47）；`render/cache/ShapeInfo.java`（`reset` 16-30） |
| Rust 现状 | `libs/ppt-core/src/cn/wps/show/util/ShapeUtil.rs`（交错模板 + 本 function 活路径）；`layout_shape.rs`（默认编译面 `KmoShowShape` fixture，`show/app` 仍 gated） |

### 2.1 关键方法/字段表（Java 源码查证 → Rust 目标）

| Java 成员 | 签名/行为 | Rust 目标 | 归属 |
|---|---|---|---|
| `calcRelativeRect(KmoShowShape, RectF)` | `calcRelativeRect(shape, rc, true)` | `pub fn calc_relative_rect(shape, rc)` | 本 function |
| `calcRelativeRect(KmoShowShape, RectF, boolean)` | `if (calcSelf || rc.isEmpty()) transform2D(shape, rc)`；`parent != null` → rotation + 02 | 同（带 calcSelf） | 本 function |
| `ShapeInfoCache.transform2D(shape, RectF)` | `left=transform2DX; top=transform2DY; right=left+W; bottom=top+H` | 直读 `shape.transform2D()` | 本 function |
| `ShapeInfo.reset(showShape)` | `transform2D=showShape.transform2D()`; `canRotate=showShape.canRotate()`; `rotation=transform2D.rotation()` | 内联读 accessor | 本 function |
| `ShapeInfoCache.canRotate/rotation` | 缓存查询 → `canRotate` / `rotation`（60000 分之一度） | `shape.canRotate()` / `transform2D().rotation()` | 本 function |
| `toDegree(int)` | `rotation / 60000.0F` | 同（f32） | 本 function |
| `toRotation(float)` | `degree * 60000.0F → int` | 同 | 壳（备用） |
| `parentGroup()` | 向上一个组（或 null） | Rust `KmoShowShape::parent_group()` | 本 function（补桩） |

### 2.2 函数改写列表

- **本 function 改写：** `ShapeUtil.rs` 新建 —— `calc_relative_rect`（2 参/3 参）、`to_degree/to_rotation`、`rect_from_transform` 私助手；`KmoShowShape.rs` 补 `transform2D()/canRotate()/parentGroup()/type()` 桩实现（仅直读 accessor，不重构模型）。
- **停车（`todo!("DEFERRED_NON_FUNCTION")`）：** `calcShape2TopMatrix`/`calcChildRelativeRect`/`calcShapeMatrix`（02 改写）、`calcRelativeTextRect`/`calcShapeTextRect`/`calcShapeRect`/`extendShapeRect`（pl-05/pr-02）、`shapeSnapshot` 族（render）、ink 处理（墨迹，MVP 不做）。
- **禁改：** `Transform2D.rs`/`GroupTransform2D.rs`/`Shape.rs` 语义；`KmoShapeTree.rs` 现有 shell；pp-01/pp-02 解析语义。

## 3. 测试方案

### 3.1 代码审计与测试面

必须通读 Java：`ShapeUtil.java` 66-209（`calcRelativeRect` 全量 + `calcShape2TopMatrix` 被调路径）+ `ShapeInfoCache.java` 33-132 + `ShapeInfo.java` 全量。禁止只按标题冒烟。

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `calcRelativeRect` (2/3 参) | ShapeUtil.java 186-209 | 无组/单组/两级组 fixture | RectF | `calcSelf=false` 且 rc 非空 → 跳过自身 |
| `transform2D` 读路径 | ShapeInfoCache.java 113-132 | off/ext 值 | left/top/right/bottom | 无 xfrm → `(0,0,0,0)` |
| rotation 路径 | ShapeInfo.java reset + 33-63 | `canRotate=true/false` | rotation 度值 | false → 0 |
| `toDegree/toRotation` | 455-462 | 0/±900000/60000000 | 0/15.0/1000.0… | 整数除法 vs float |
| 组分支 | calcChildRelativeRect 被调 | parent 组 | rc 被 02 映射 | 依赖 02 数值 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| C01 | 无组单 shape | 正常 | 是 | T01 |
| C02 | 组直子（1:1） | 正常 | 是 | T02 |
| C03 | 两级组嵌套 | 分支 | 是 | T03 |
| C04 | `calcSelf=false` 且 rc 非空 | 分支 | 是 | T04 |
| C05 | `canRotate=false` | 边界 | 是 | T05 |
| C06 | 无 xfrm（全 0） | 失败语义 | 是 | T06 |
| C07 | `toDegree/toRotation` 往返 | 边界 | 是 | T07 |

### 3.3 可测试性

fixture 直接构造 `KmoShowShape` 模型（或最小 accessor 桩），不启动解析器；数值可手算对拍（`off/ext→rect` 为线性）；组分支复用 02 的单测 fixture。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| T01 | C01 | 无父 | off=(100,200) ext=(300,400) | `(100,200,400,600)` | 框错 |
| T02 | C02 | 父=组 chExt=rcGroup 尺寸 | 子 (0,0,300,300) | `(0,0,300,300)` | 组位置被误加 |
| T03 | C03 | 两层组 | 嵌套 fixture | 与 Java 同 fixture 数值一致 | 层级错 |
| T04 | C04 | rc 预置 (1,2,3,4) | calcSelf=false | 跳过自身，只做父合并且 rc 变化符合 Java | calcSelf 语义错 |
| T05 | C05 | canRotate=false | 有 rotation 值 | 不入换挡、rotation=0 | rotation 泄漏 |
| T06 | C06 | 无 xfrm | 空 Transform2D | `(0,0,0,0)` | 伪造默认框 |
| T07 | C07 | — | rot=900000 | toDegree=15.0；toRotation(15)=900000 | 系数漂移 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/util/ShapeUtil.rs（loop 新建）
test:  libs/ppt-core/tests/cn/wps/show/util/ShapeUtil.rs
[[test]] name = "01_function_shape_absolute_rect"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_shape_absolute_rect

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_shape_absolute_rect \
  --src src/cn/wps/show/util/ShapeUtil.rs \
  --report libs/ppt-core/layout/features/pl-02-xfrm-geometry/functions/01_function_shape_absolute_rect/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_shape_absolute_rect --json`)
date: 2026-08-30 10:27

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/util/ShapeUtil.rs` | 102/196 | 52.0% | 198/296 | 66.9% |
| **function total** | **102/196** | **52.0%** |  |  |
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
  --report libs/ppt-core/layout/features/pl-02-xfrm-geometry/functions/01_function_shape_absolute_rect/FUNCTION.md
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