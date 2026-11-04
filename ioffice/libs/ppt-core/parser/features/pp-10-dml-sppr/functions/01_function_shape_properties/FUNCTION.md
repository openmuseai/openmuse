# 01_function_shape_properties — p:spPr 顶层分发与 xfrm

> feature: pp-10-dml-sppr ｜ 状态：**pass** ｜ Java 管线环节：S0 parse → S1 convert 输入

## 0. 管线能力解锁

形状在 pp-02 已落 `DgStore` 行，但 spPr 子树是 deferred——没有本 function，`p:spPr` 事件不入
模型：**无 xfrm 坐标（off/ext/rot/flip），S1 convert 的 `ShapeInfoCache.transform2D` 无输入 → pl-02
无 rect → S4 无框可绘**。实现后 shapes 有坐标/几何/填充/线条的**入口** → S1 convert 有输入；
缺失则整条链断在「形状无坐标」。

## 1. 功能说明

两个 Java 顶层 handler 的分发 + xfrm 写入链：

- `ShapePropertiesHandler`（`p:spPr`）：`startElement` 读 bwMode（attr 1114443 `bwMode` /
  pml 3211605，值经 `DGMap.blackWhiteModeMap` 映射，miss 不写）→ `ShapePr.putBwMode`；
  `enterSubElement` 按子元素 id 分发：scene3d=1114144、sp3d=1114145（MVP 延后）、ln=1114148、
  fill=1114370–1114374+3211586、effectLst/effectDag=1114389/1114390（延后）、
  custGeom=1114428（`geom().putType(1)`）、prstGeom=1114429（`putType(0)`）、xfrm=3211589；
  `endElement` 对 scene3D/sp3d 做 save+forceWrite（MVP 不实现 3D，保留空语义）。
- `GroupShapePropertiesHandler`（`p:grpSpPr`，pp-02:05 group 路径）：bwMode 写
  `group.spPr().putBwMode`；子分发 fill 六 id + effect + `GroupTransform2DHandler`（group 形态）。
- xfrm 写入链：`Transform2DHandler` → flipH=1114202/flipV=1114203/rot=1114201（pml 3276881 备用）；
  `ext`=1114197 → `PositiveSize2DHandler`（cx=1114163/3211488、cy=1114164/3211489）；
  `off`=1114200 → `Point2DHandler`（x=1114161/3211405、y=1114162/3211406）。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `p:spPr`/`p:grpSpPr` 的 attrs + 子元素事件（`XmlRoAttr attrsNotCopy`、element id）+ 构造入参 `ShapePr shapePr`（或 group `Shape`）+ `HandlerCallback`（仅供子 handler 透传）。 |
| **输出** | `ShapePr.putBwMode`；`Transform2D` 的 putFlipHorizontal/putFlipVertical/putRotation + extents（cx/cy）+ point（x/y）；子分发返回的 `ElementHandler`（fill/ln/geom/xfrm/effect）。 |
| **状态与副作用** | 逐 attr `getByName` 判空；`blackWhiteModeMap` miss → null 不写。`enterSubElement` 的 default 分支返回 null（消费方按 null 停车）。`Transform2DHandler` 子元素 `ext`/`off` 均构造子 handler；未出现则 `Transform2D` 保持默认（ext/off 各自为空）。Scene3D/Shape3D 分支在 MVP 下不实例化（延后）。 |
| **错误语义** | 未知子元素 id → `enterSubElement` 返回 null，无副作用。bwMode 值 map miss → 不 put（与 Java `if (blackWhiteMode != null)` 一致）。灰度态：Rust table 写法与 Java switch-case 数值必须逐一核对（见 §2）；不一致项以 Java 为准并回写本文档。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止去改 |
|---|---|---|---|---|---|
| `HandlerCallback` | pp-01:02 | 透传给 fill/ln 子 handler；`onBlipEmbed/onBlipLink` 等回调签名 | **pass**（`model/HandlerCallback.rs` keep_refactor） | FileManager stub | **pass 语义**：回调顺序/缺省值不可改 |
| spTree 分发生成 spPr 事件 | pp-02:02/03/04/05 | 构造 `ShapePropertiesHandler`/`GroupShapePropertiesHandler` 的调用点 | pass（02/03/04/05 已验收） | — | 不重排 DgStore 提交顺序 |
| `ShapePr` / `Transform2D` / `PositiveSize2D` / `Point2D` | data 模型 | `putBwMode`、`putFlip*`、`putRotation`、`putCx/Cy/X/Y` | `data/shape/ShapePr.rs` 1300 行已存在；`data/basic/*` 齐备 | — | 不改模型字段语义 |
| `DGMap.blackWhiteModeMap` | util | `String → Option<i32>` | 审计 `delete` 空壳（12 行） | 以测试字面量/最小常量重建 | 不整表恢复 |
| sppr 族 handler `.rs` | 本 feature | Java 结构模板 | **重做**：审计 `ShapePropertiesHandler.rs` 275 行空壳、`GroupShapePropertiesHandler.rs` 183 行、`Transform2DHandler.rs` 121 行（本地 `_stub` 遮蔽真模型）；**现在模块接线已注释（`sppr/mod.rs`）**，loop 需 `template --recursive` 重新生成后整体改写 | — | 不在旧 stub 上增量补 |

## 2. 核心代码与函数改写列表

| Java 类 | Rust 目标 | 关键方法/字段 |
|---|---|---|
| `ShapePropertiesHandler` | `…/sppr/ShapePropertiesHandler.rs` | 字段 `shapePr`、`callback`、`scene3DProperties`、`shape3DProperties`；`ctor(ShapePr, HandlerCallback)`、`startElement(int, XmlRoAttr)`、`enterSubElement(int)`、`endElement(int)` |
| `GroupShapePropertiesHandler` | `…/sppr/GroupShapePropertiesHandler.rs` | 字段 `group`（`Shape`）、`handlerCallback`；`ctor(Shape, HandlerCallback)`、`startElement`、`enterSubElement` |
| `Transform2DHandler` | `…/sppr/transform/Transform2DHandler.rs` | 字段 `transform2d`；`ctor(Transform2D)`、`startElement`（flipH/flipV/rot）、`enterSubElement`（ext/off） |
| `PositiveSize2DHandler` | `…/sppr/transform/PositiveSize2DHandler.rs` | 字段 `size2D`；`ctor/init(PositiveSize2D)`、`startElement`（cx/cy 双命名空间） |
| `Point2DHandler` | `…/sppr/transform/Point2DHandler.rs` | 字段 `point2D`；`ctor/init(Point2D)`、`startElement`（x/y 双命名空间） |
| `GroupTransform2DHandler` | `…/sppr/transform/GroupTransform2DHandler.rs` | group 子级比例/旋转；随 01 以 group 契约验收，业务归 pl-02 |

元素/attr id（已对照 `kfc/xsd/dml/A.java`、`pml/P.java` 查证）：bwMode=1114443/3211605、
xfrm=3211589、ln=1114148、noFill=1114370、solidFill=1114371、gradFill=1114372、pattFill=1114373、
grpFill=1114374、blipFill=3211586、effectLst=1114389、effectDag=1114390、custGeom=1114428、
prstGeom=1114429、scene3d=1114144、sp3d=1114145、flipH=1114202、flipV=1114203、rot=1114201/3276881、
ext=1114197、off=1114200、cx=1114163/3211488、cy=1114164/3211489、x=1114161/3211405、
y=1114162/3211406。

函数改写列表（模板 `// TODO_RUST_CODE` 全部在本 function 内实现）：

- 顶层两个 handler 的 `ctor`：只保存引用（`&mut` 等价物以模板为准），不做深拷贝。
- `startElement`（两处）：bwMode 双 id getByName + map miss 判空 → `putBwMode`。
- `enterSubElement`（两处）：switch 数值与 Java 完全一致；3D/effect 分支按「延后」返回 null 停车
  且不实例化（在 Rust 中以注释标明延后理由，不写 todo stub）。
- `Transform2DHandler.startElement`：flipH/flipV 用 `toBoolean`；rot 双 id。`enterSubElement` 构造
  ext/off 两个 size/point handler。
- `PositiveSize2DHandler`/`Point2DHandler`：四 attr、双命名空间 getByName → `toInt` 写入。
- `todo!("DEFERRED_NON_FUNCTION")` 停车项：scene3d/sp3d/effect 分支（MVP 不做效果/3D）；`endElement`
  的 3D save/forceWrite 以空实现 + 注释说明（Java 语义已记录，恢复时对齐）。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读（禁止只按标题冒烟）：

- `ShapePropertiesHandler.java`/`GroupShapePropertiesHandler.java` 的 `startElement`/`enterSubElement`
  全部分支与 id 数值。
- `Transform2DHandler.java`/`PositiveSize2DHandler.java`/`Point2DHandler.java` 的 attr 名与双命名
  空间 id（a: 1114xxx vs p: 3211xxx/3276xxx）。
- `data/shape/ShapePr.java`、`data/basic/Transform2D.java` 的 put* 语义；对照 Rust
  `data/shape/ShapePr.rs`、`data/basic/Transform2D.rs`。
- Rust 现状：`sppr/mod.rs` 现在只有 `// pub mod …` 注释；旧 `ShapePropertiesHandler.rs`（275 行
  空壳）以审计表为准，**不读旧 stub 做增量**。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | bwMode 双 id + map miss（shape 与 group 两处） | 分支 | 是 | T01 |
| C02 | enterSubElement 六类 id（fill 全、ln、prst/custGeom、xfrm）返回对应 handler | 分支 | 是 | T02 |
| C03 | 未知子元素 → None | 失败/边界 | 是 | T03 |
| C04 | xfrm：flipH/flipV/rot 双 id | 分支 | 是 | T04 |
| C05 | ext/off → PositiveSize/Point 双命名空间 cx/cy/x/y | 数据 | 是 | T05 |

### 3.3 可测试性

内存 attrs/events（构造 `XmlRoAttr` 的 contract fixture）。`ShapePr`/`Transform2D` 用真实
`data/` 模型实例，不引入本地 stub。不打开 reader/part。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | spPr attrs 含 bwMode（合法值 + 非法值各一）；grpSpPr 同 | 合法值写 `putBwMode`；非法值不写 | 红测停在本 function |
| T02 | C02 | 逐个 enterSubElement：noFill/solidFill/…/ln/prstGeom/custGeom/xfrm | 每 id 返回对应 handler 类型；`geom().putType` 0/1 正确 | 红测停在本 function |
| T03 | C03 | 未知 id（如 1114199） | 返回 None，无副作用 | 红测停在本 function |
| T04 | C04 | xfrm attrs flipH/flipV/rot（a: 与 p: 各一组） | `Transform2D` 三个字段与 Java 一致 | 红测停在本 function |
| T05 | C05 | ext（cx/cy 双 id）+ off（x/y 双 id） | `extents`/`point` 数值正确 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs（含 group/transform 族目标文件）
test:  libs/ppt-core/tests/cn/wps/drawing/dmlr/handler/shape/sppr/01_function_shape_properties.rs
[[test]] name = "01_function_shape_properties"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_shape_properties

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_shape_properties \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs \
  --report libs/ppt-core/parser/features/pp-10-dml-sppr/functions/01_function_shape_properties/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_shape_properties --json`)
date: 2026-08-31 09:13

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs` | 64/95 | 67.4% | 83/122 | 68.0% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/Transform2DHandler.rs` | 57/125 | 45.6% | 73/172 | 42.4% |
| **function total** | **121/220** | **55.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.java
```

Loop 结果：PASS（`--rs` + `--java` ShapePropertiesHandler / Transform2DHandler）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs \
  --report libs/ppt-core/parser/features/pp-10-dml-sppr/functions/01_function_shape_properties/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-31 01:14

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShapePropertiesHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/ShapePropertiesHandler.rs` | **PASS** | `ShapePropertiesHandler`, `startElement`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `Transform2DHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/transform/Transform2DHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/Transform2DHandler.rs` | **PASS** | `Transform2DHandler`, `startElement`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->