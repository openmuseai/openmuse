# 01_function_ph_merge_transform — ph 位置继承（xfrm 合并）

> feature: pl-03-ph-merge ｜ 状态：**pass** ｜ Java 管线环节：S2 布局

## 0. 管线能力解锁

实现后管线新增「ph 位置补齐」：缺 `xfrm` 的 placeholder 形状能从 layout（按 `ph.idx` 匹配）或 master（按 `ph.type` 匹配
title/body）继承真实 EMU 位置。
没有它，缺 xfrm 的 ph 形状位置为 `(0,0,0,0)`，文本框/占位画错位置——**必断**（PIPELINE_LAYOUT_RENDER_PLAN §3.2 断点表 pl-03 行）。

## 1. 功能说明

`PlaceHolderMerger.mergeTransform(KmoShowShape)` 的三级查找（Java 全量查证），配 `phType`（自身→layout→master 的 type 判定）、
`findLayoutPhShape`（按 idx）、`findMasterPhShape`（按 type + 转换表）。非 ph/有自身 xfrm 时 identity 直通。

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | `KmoShowShape phShape`：`ph()`（`ShowShape.Placeholder`：`hasType()/type()/idx()/hasIdx()`）、`isPlaceHolder()`、`parent()`（`type()==0`=slide、`1`=layout、`2/4/5`=master 系 JSON 母版）、`drawingShape()`（读 `Shape.spPr()`）。可达：`((KmoSlide)parent).layout()` → `KmoSlideLayout.spTree()` → `layout.master().spTree()`。 |
| **输出** | 合并后的 `Transform2D`（off/ext，EMU）。优先级链：①自身 `spPr().hasXfrm()` → 自身；②`parent.type()==0 && ph.idx()!=-1`：layout ph（`findLayoutPhShape` 命中且 `spPr().hasXfrm()`）→ layout；layout ph 缺失 → master `spTree` 扫 `asTitle=(phType()==2||phType()==0)` 匹配 `masterShape.phType()==0`（title）/`==1`（body）→ 该 master `shapePr().xfrm()`；③`findMasterPhShape(phShape)` → master xfrm；④兜底自身 `spPr().xfrm()`。（Java 53-83 行逐分支查证） |
| **状态与副作用** | 只读三层树；不写 ph、不改 ShapePr。Java 用 `Shape.open(drawingShape())` 临时对象（`Shape.java` pool），Rust 直读模型。 |
| **错误语义** | `parent().type()!=0`（如 layout 上的 ph）→ 跳过 ② 直入 ③；`slide.layout()==null` → 跳过 ②；`findMasterPhShape` 全程未中 → 自身兜底（可能空 xfrm）；`idx` 语义：解析侧 `"4294967295"` → `putIdx(-1)`（`SwNvPrHandler.java` 98-104），`idx()==-1` 不入 ②；无 idx（`hasIdx=false`）→ `findLayoutPhShape` 走 `findPhByIndex(0)`（`PlaceHolderMerger.findLayoutPhShape` 676-679 查证）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓（保留 Java 注释+签名） | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `KmoShapeTree.findPhByIndex/findPhByType` | show/app/shape | 按 idx/type 找 ph（`idx<0/type<0 → null`） | ⚠ `KmoShapeTree.rs` no-op shell（`shapeCount()=0`）——loop 补 | — | 不改 `spTree` 祖先容器语义 |
| `ShowShape.Placeholder` | show/data/shape | `type()/idx()/hasType()/hasIdx()` | ⚠ Rust 核心侧未接线（parse 以 test-local stub 过 pp-01/02） | loop 提供最小 accessor | 不定义第二套 ph 存储 |
| `ShapePr`/`Transform2D` | drawing/data | `hasXfrm()/xfrm()`（含 follow 父解析 `todo!` 分支） | ✅ `ShapePr.rs`/`Transform2D.rs` keep（`hasXfrm/xfrm` 已实现；xfrm 的 follow-parent 懒加载为 `todo!`，本 function 用 `hasXfrm` 判定不触懒加载） | follow 懒加载 | 不提前修 ShapePr 内懒加载 |
| `KmoSlideLayout/KmoSlideMaster` | show/app/slide | `spTree()/master()` 可达 | ⚠ Rust 无对应（audit delete，layout 层重建/接线） | fixture 三层最小树 | 不重建编辑容器 |
| `phType` helper | 本 function | 自身 type → layout → master | 一并改写 | — | 不移动归属 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/presentation/core/cn/wps/show/app/shape/PlaceHolderMerger.java`（`mergeTransform` 53-83、`phType` 25-50、`findLayoutPhShape` 654-686、`findMasterPhShape` 724-754、`findMaster` 691-711、`isMaster` 714-722）；`KmoShapeTree.java`（574-605） |
| Rust 现状 | `libs/ppt-core/src/cn/wps/show/layout/PlaceHolderMerger.rs`（交错模板；本 function 活路径已实现）；`util/layout_shape.rs` 三层 fixture 树 |

### 2.1 关键方法/字段表（Java 源码查证 → Rust 目标）

| Java 成员 | 签名/行为 | Rust 目标 | 归属 |
|---|---|---|---|
| `mergeTransform(KmoShowShape)` | `Transform2D` 返回；四级查找（§1.1） | `merge_transform(&self/…, ph: &KmoShowShape) -> Transform2D` | 本 function |
| `phType(KmoShowShape)` | 自身 type → layout ph type → master ph type（把 `phShape.phType()` 语义钉死） | `ph_type(...) -> i32` | 本 function |
| `findLayoutPhShape(KmoShowShape)` | `isPlaceHolder && parent.type==0 && slide.layout()!=None && layout.spTree()!=None`；`hasIdx? findPhByIndex(idx): findPhByIndex(0)` | `find_layout_ph_shape(...) -> Option<&KmoShowShape>` | 本 function |
| `findMasterPhShape(KmoShowShape)` | `isMaster(2/4/5)→self`；`findMaster`（slide→layout.master / layout→master / notes→noteMaster）；`phConvertType`（无 type→8；2→0；3/8→1）；`findPhByType(realType/8)` 优先、未中 `findPhByType(convertType)` | `find_master_ph_shape(...)` | 本 function |
| `findMaster(ISlide)` | 按 parent type 0/1/3 分派 | `find_master(...)` | 本 function |
| `isMaster(KmoShowShape)` | `parent.type()==2/4/5` | `is_master(...)` | 本 function |
| `findPhByIndex/findPhByType` | `root.childAt(i)` 首匹配；`idx<0/type<0→null` | `KmoShapeTree` 补齐 | 本 function |
| `ShowShape.Placeholder` | `hasType/type/idx/hasIdx` | 最小 accessor（loop 接线） | 本 function |

### 2.2 函数改写列表

- **本 function 改写：** layout 层合并模块 —— `merge_transform`、`ph_type`、`find_layout_ph_shape`、`find_master_ph_shape`、`find_master`、`is_master`；`KmoShapeTree.rs` 补 `find_ph_by_index/find_ph_by_type`；`KmoShowShape.rs` 补 `ph()/isPlaceHolder()/parent()/parentGroup()` 等（与 pl-02 共用，一处实现）。
- **停车（`todo!("DEFERRED_NON_FUNCTION")`）：** `mergeShapePr` 及所有 `merge*NotFollow`（02 改写）、`mergeShapeStyle/mergeTextListStyle/mergeTextBodyPr/mergeFill`（壳/延后）、`getTextListStyleInMaster`（pl-04/05）、`KmoPropertiesMerger`（不可在此实现）。
- **禁改：** `ShapePr.rs`/`Transform2D.rs` keep 语义；pp-01/pp-02 解析语义；`KmoSlide.rs` 现有桩结构。

## 3. 测试方案

### 3.1 代码审计与测试面

必须通读 Java：`PlaceHolderMerger.java` 全量（53-83、25-50、654-754 方法体）＋ `KmoShapeTree.java` 574-605 ＋
`SwNvPrHandler.java` 75-128（ph 解析与 `"4294967295"` 语义）＋ `SWReaderMap.placeholderTypeMap`（type 常量表）。
禁止只按标题冒烟。

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `mergeTransform` | 53-83 | 各查找分支 fixture | Transform2D | 四级兜底顺序 |
| `phType` | 25-50 | 自身/仅 layout/仅 master type | 最终 type | 无 type 时的链传递 |
| `findLayoutPhShape` | 654-686 | hasIdx idx=N / 无 idx | layout ph / None | 非 slide、layout 缺失 |
| `findMasterPhShape` | 724-754 | type 转换表入参 | master ph / None | 2→0、3/8→1、无 type→8 |
| `findPhByIndex/findPhByType` | KmoShapeTree 574-605 | idx/type 列表 | 首个 ph | idx<0/type<0 → null |
| Placeholder idx | SwNvPrHandler 98-104 | "4294967295" / 缺省 | -1 / 0（未 put） | hasIdx 区分 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| C01 | 非 ph → identity | 正常 | 是 | T01 |
| C02 | 自身有 xfrm | 正常 | 是 | T02 |
| C03 | layout ph 命中（hasIdx / 无 idx 两条路） | 正常 | 是 | T03、T04 |
| C04 | layout ph 缺失 → master title/body 扫描 | 分支 | 是 | T05（title）、T06（body） |
| C05 | asTitle 边界：phType 0/2 → title；1/3 → body | 分支 | 是 | T07 |
| C06 | idx=-1 → 跳 ② 直查 master（findMasterPhShape） | 边界 | 是 | T08 |
| C07 | findMasterPhShape type 转换（2→0；3/8→1；无 type→8 优先） | 分支 | 是 | T09 |
| C08 | 全程兜底自身空 xfrm | 失败语义 | 是 | T10 |
| C09 | layout()==null / master 缺失 | 失败语义 | 是 | T11 |

### 3.3 可测试性

fixture 构造最小三层树（slide.spTree → layout.spTree → master.spTree，各挂 ph 的 KmoShowShape 桩）与 `Placeholder{type,idx,hasType,hasIdx}`；
断言只到返回的 xfrm 数值/None 值，不触发完整 parse。`ShapePr.hasXfrm` 判定避免触碰 `xfrm` 的 follow 懒加载 `todo!`。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| T01 | C01 | 非 ph shape | merge_transform | 自身 xfrm | 误查三层 |
| T02 | C02 | 自身 spPr 有 xfrm | off=(100,200) ext=(300,400) | (100,200,400,600) | 覆盖自身 |
| T03 | C03 | layout ph idx=3 有 xfrm | findLayoutPhShape 命中 | layout xfrm | 查找序错 |
| T04 | C03 | 无 idx | 走 findPhByIndex(0) | 命中 idx=0 的 layout ph | 缺省语义错 |
| T05 | C04 | layout 无 ph、master title(0) 有 xfrm、phType=0 | 扫 master spTree | master title xfrm | 扫错了层 |
| T06 | C04 | 同上 phType=1 | asTitle=false → body(1) | master body xfrm | 匹配错 |
| T07 | C05 | phType=2(ctrTitle)/3(subTitle) | asTitle=true/false | 分别落 title/body | 边界错 |
| T08 | C06 | idx=-1 | 跳过 ② | `findMasterPhShape` 结果/兜底 | idx 语义错 |
| T09 | C07 | 无 type / type=2 / type=3 | 转换表 | 8→(先 8 后 1)；0；1 | 转换表漂移 |
| T10 | C08 | 全程未中 | 空 Transform2D | 自身空 xfrm | 伪造位置 |
| T11 | C09 | layout()==null / master 缺失 | merge_transform | 跳过对应分支 → 兜底 | panic |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/layout/PlaceHolderMerger.rs
       ＋ `layout_shape.rs` 三层树 + `findPhByIndex`/`findPhByType`（默认面；`app/shape/KmoShapeTree.rs` 仍 gated）
test:  libs/ppt-core/tests/cn/wps/show/layout/PlaceHolderMerger.rs
[[test]] name = "01_function_ph_merge_transform"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_ph_merge_transform

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_ph_merge_transform \
  --src src/cn/wps/show/layout/PlaceHolderMerger.rs \
  --report libs/ppt-core/layout/features/pl-03-ph-merge/functions/01_function_ph_merge_transform/FUNCTION.md
```

（src 参数按 loop 实际落位修正）

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_ph_merge_transform --json`)
date: 2026-08-30 11:00

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/layout/PlaceHolderMerger.rs` | 109/180 | 60.6% | 221/320 | 69.1% |
| **function total** | **109/180** | **60.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/layout/PlaceHolderMerger.rs \
  --java koffice/presentation/core/cn/wps/show/app/shape/PlaceHolderMerger.java
```

Loop 结果：**PASS**（`check-template-layout.mjs`）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/app/shape/PlaceHolderMerger.java \
  --rs libs/ppt-core/src/cn/wps/show/layout/PlaceHolderMerger.rs \
  --report libs/ppt-core/layout/features/pl-03-ph-merge/functions/01_function_ph_merge_transform/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-30 03:00

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PlaceHolderMerger` | `koffice/presentation/core/cn/wps/show/app/shape/PlaceHolderMerger.java` | `libs/ppt-core/src/cn/wps/show/layout/PlaceHolderMerger.rs` | **PASS** | `phType`, `mergeTransform`, `findLayoutPhShape`, `findMaster`, `isMaster`, `findMasterPhShape` | `mergeShapePr`, `mergeShapeStyle`, `mergeTextListStyle`, `filterTextListStyle`, `writeTextListStyle`, `writeTextListProperties`, `followTextListStyle`, `followTextParagraphProperties`, `mergeTextBodyPr`, `getTextListStyleInMaster`, `mergeFill`, `mergeFillNotFollow`, `mergeLnNotFollow`, `mergeEffectNotFollow`, `mergeGeomNotFollow`, `mergeScene3DNotFollow`, `mergeShape3DNotFollow` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->