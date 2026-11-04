# 02_function_ph_merge_style — ph 样式继承（ShapePr 合并）

> feature: pl-03-ph-merge ｜ 状态：**pass** ｜ Java 管线环节：S2 布局

## 0. 管线能力解锁

实现后管线新增「ph 样式补齐」：placeholder 形状的 `ShapePr` 沿 自身→layout→master 的 `follow` 链解析缺失字段
（fill/ln/geom/effect 等），渲染侧（pr-02）拿到继承后的 fill/line。
没有它，ph 形状只有位置没有样子，`mergeShapePr` 缺失时继承链断裂——**样式断层**（PIPELINE_LAYOUT_RENDER_PLAN §2.2 pl-03 `ph_merge_style` 行）。

## 1. 功能说明

`PlaceHolderMerger.mergeShapePr(KmoShowShape)`（Java 全量查证）：
非 ph → 自身 `ShapePr`；ph 按 parent 类型与 idx 标志走 `ShapePr.follow` 链；
`ShapePr.follow` 的父引用解析（Rust `ShapePr.rs` 的 `follow(Option<Box<ShapePr>>)` 已 keep，字段 getter 的 follow 分支为 `todo!`——本 function 使用其 has*/取值语义，父链解析分支由 loop 完成或兜底）。

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | `KmoShowShape phShape`：`isPlaceHolder()`；`parent()`（type 0=slide/1=layout）；`ph().idx()`；可达 layout/master 的 `ShapePr`（`Shape.open(drawingShape()).spPr()`）。 |
| **输出** | 合并后 `ShapePr`（Java 返回同一对象，`follow` 链：缺失字段可经父引用解析）。
  分支（Java 87-127 行逐分支查证）：
  ① 非 ph → 返回自身 `spPr()`；
  ② slide（`parent.type()==0`）且 `idx()!=-1`：`layoutShape = findLayoutPhShape(phShape)`；null → 自身；否则 `own.spPr().follow(layoutSp.spPr())`；`masterShape = findMasterPhShape(layoutShape)`；非 null → `layoutSp.spPr().follow(masterSp.spPr())`；返回 own；
  ③ slide `idx()==-1`：`masterShape = findMasterPhShape(phShape)`；非 null → `own.spPr().follow(masterSp.spPr())`；返 own；
  ④ layout（`parent.type()==1`）：同 ③（findMasterPhShape(phShape)）；
  ⑤ 其它 → 自身。 |
| **状态与副作用** | Java 对 `Shape.open()` 的临时 Shape 建 follow 链（写 `parent` 引用）；Rust 以 `ShapePr.rs` `follow` 等价实现；不修改 slide/layout/master 原始模型，不改 `KmoShowShape`。 |
| **错误语义** | `layoutShape==null` 且 `idx!=-1` → 返回自身（不查 master，Java 94-96 行）；`findMasterPhShape` 未中 → 返回已 follow 的自身；`ShapePr.follow` 父为 null → 等价自身（Java `follow(null)` 语义需 loop 对拍）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓（保留 Java 注释+签名） | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `ShapePr.follow` | drawing/data/shape | `follow(parent)` + 字段 has*/getter 的父解析 | ✅ `ShapePr.rs` `follow(&mut self, Option<Box<ShapePr>>)` 已实现（654 行；字段 getter 的「follow parent」分支仍有 `todo!`，loop 完成本 function 需要的最小父解析） | 父解析懒加载 | 不换 ShapePr 结构 |
| `findLayoutPhShape/findMasterPhShape/phType` | 01（本 feature 共享） | 01 已实现 | 01 先行 | — | 不复制 01 逻辑 |
| `ShapePr` 字段读取 | drawing/data/shape | `hasFill/fill/hasLn/ln/hasGeom/geom/hasEffect/effect` | ✅ 已实现（`todo!` 仅 follow 分支） | — | — |
| `merge*NotFollow` 取值族 | 本 function 壳 | `mergeFillNotFollow/mergeLnNotFollow/mergeEffectNotFollow/mergeGeomNotFollow` | 待 loop（pr-02 打开） | 先以 `mergeShapePr` 为准 | 不提前改 pr-02 契约 |
| `KmoSlideLayout/KmoSlideMaster` 可达 | 01 同款缺口 | `spTree()/master()` | ⚠ 同 01（layout 层重建/接线） | fixture 三层树 | — |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/presentation/core/cn/wps/show/app/shape/PlaceHolderMerger.java`（`mergeShapePr` 87-127 行） |
| Rust 现状 | `libs/ppt-core/src/cn/wps/show/layout/PlaceHolderMerger.rs`（`mergeShapePr` 已实现）；`layout_shape.rs` `ShapePr.follow` 父解析 |

### 2.1 关键方法/字段表（Java 源码查证 → Rust 目标）

| Java 成员 | 签名/行为 | Rust 目标 | 归属 |
|---|---|---|---|
| `mergeShapePr(KmoShowShape)` | 见 §1.1 五分支 | `merge_shape_pr(...) -> ShapePr`（own 语义：Java 返回合并链的链表头） | 本 function |
| `ShapePr.follow(ShapePr)` | 设父引用（`drawing/data/shape/ShapePr.java`） | `ShapePr::follow(Option<Box<ShapePr>>)`（已实现 ✅） | 本 function（复用） |
| `ShapePr.hasFill/hasLn/hasGeom/hasEffect/…` | 字段判定 | 已实现 ✅（follow 分支除外） | 本 function（复用） |
| `isPlaceHolder()/parent().type()/ph().idx()` | 分支判定 | 01/pl-02 补的 accessor | 本 function（复用） |
| `mergeFillNotFollow/mergeLnNotFollow/mergeEffectNotFollow/mergeGeomNotFollow` | 自身→layout→master 的「不 follow」取值（Java 493-597 行） | 壳：随 pr-02 打开 | 壳 |

### 2.2 函数改写列表

- **本 function 改写：** layout 层合并模块 —— `merge_shape_pr`（五分支）；与 01 共享 `find_layout_ph_shape/find_master_ph_shape/is_master/ph_type`（一处实现，01 loop 内完成）。
- **停车（`todo!("DEFERRED_NON_FUNCTION")`）：** `mergeShapeStyle`（ShapeStyle 继承，themeable 消费未开）、`mergeFill/mergeTextBodyPr/mergeTextListStyle`（分层 follow 变体）、`merge*NotFollow` 三件（effect/scene3D/sp3d：3D 不做）、`filterTextListStyle/writeTextListStyle/followTextListStyle`（pl-04/05）、`getTextListStyleInMaster`（master 样式表，pl-04 后）。
- **禁改：** `ShapePr.rs` 结构与非 follow 字段语义；01 的查找 helper；pp-10 的 spPr 解析写入。

## 3. 测试方案

### 3.1 代码审计与测试面

必须读 Java：`mergeShapePr` 87-127 方法体（含 follow 顺序：own.follow(layout) 后才 layout.follow(master)）、
`isPlaceHolder` 定义（`show/app/shape/KmoShowShape.java` 552 行）、`ShapePr.follow`（drawing/data/shape/ShapePr.java）。
审计表：

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `mergeShapePr` | 87-127 | 五分支 fixture | 合并 ShapePr | 非 ph 直通 |
| follow 顺序 | 97-102 | 三级链 | own→layout→master | follow 顺序反了 |
| `idx==-1` 分支 | 105-115 | 直连 master | own.follow(master) | 误查 layout |
| layout slide 分支 | 116-124 | parent.type==1 | own.follow(master) | 误跳 slide 分支 |
| `ShapePr.follow` | ShapePr.java/.rs | 字段缺失/指定 | 父解析 | 空父 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| C01 | 非 ph → 自身 spPr | 正常 | 是 | T01 |
| C02 | slide idx!=-1 → 三级 follow 链 | 正常 | 是 | T02 |
| C03 | slide idx!=-1 且 layout ph 缺失 → 自身 | 失败语义 | 是 | T03 |
| C04 | slide idx!=-1 且 master 缺失 | 失败语义 | 是 | T04 |
| C05 | slide idx==-1 → own.follow(master) | 分支 | 是 | T05 |
| C06 | layout（parent.type==1）→ own.follow(master) | 分支 | 是 | T06 |
| C07 | 其它 parent type | 边界 | 是 | T07 |
| C08 | follow 链缺失字段经父解析 | 状态 | 是 | T08 |

### 3.3 可测试性

同 01 的 fixture 三层树；`ShapePr` 用真实模型（keep 类）构造字段再 follow；断言「经 follow 解析后的字段值」而非内部指针。
`ShapePr.rs` 字段 getter 的 follow 分支 `todo!` 若挡路，本 loop 需先完成最小父解析（本 function 改写列表内）。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| T01 | C01 | 非 ph、自身有 fill | merge_shape_pr | 返回自身 spPr，字段原样 | 误建链 |
| T02 | C02 | slide idx=2、layout ph 有 fill、master 有 ln | 三级 | own→layout→master：fill 经 layout、ln 经 master | 链序错 |
| T03 | C03 | idx!=-1 但无 layout ph | merge_shape_pr | 自身原样 | 误走 master |
| T04 | C04 | layout 命中原样但无 master | 二级 | own.follow(layout)，master 不参与 | 越级 |
| T05 | C05 | idx=-1、master 有 fill | own.follow(master) | fill 经 master | idx 分支错 |
| T06 | C06 | layout slide、master 有 fill | own.follow(master) | 同 T05 语义 | parent.type 判定错 |
| T07 | C07 | parent.type=2/4/5 | merge_shape_pr | 自身直通 | 误建链 |
| T08 | C08 | own 无 fill / layout 有 | follow 解析 | 取到 layout fill | 父解析断 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/layout/PlaceHolderMerger.rs
       ＋ src/cn/wps/show/util/layout_shape.rs（`ShapePr.follow` / hasFill/fill/hasLn/ln 父解析）
test:  libs/ppt-core/tests/cn/wps/show/layout/PlaceHolderMerger_style.rs
[[test]] name = "02_function_ph_merge_style"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_ph_merge_style

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_ph_merge_style \
  --src src/cn/wps/show/layout/PlaceHolderMerger.rs \
  --src src/cn/wps/show/util/layout_shape.rs \
  --report libs/ppt-core/layout/features/pl-03-ph-merge/functions/02_function_ph_merge_style/FUNCTION.md
```

（src 参数按 loop 实际落位修正）

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_ph_merge_style --json`)
date: 2026-08-30 11:22

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/layout/PlaceHolderMerger.rs` | 94/210 | 44.8% | 207/388 | 53.4% |
| `src/cn/wps/show/util/layout_shape.rs` | 127/187 | 67.9% | 135/209 | 64.6% |
| **function total** | **221/397** | **55.7%** |  |  |
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
  --report libs/ppt-core/layout/features/pl-03-ph-merge/functions/02_function_ph_merge_style/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-30 03:22

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PlaceHolderMerger` | `koffice/presentation/core/cn/wps/show/app/shape/PlaceHolderMerger.java` | `libs/ppt-core/src/cn/wps/show/layout/PlaceHolderMerger.rs` | **PASS** | `phType`, `mergeTransform`, `mergeShapePr`, `findLayoutPhShape`, `findMaster`, `isMaster`, `findMasterPhShape` | `mergeShapeStyle`, `mergeTextListStyle`, `filterTextListStyle`, `writeTextListStyle`, `writeTextListProperties`, `followTextListStyle`, `followTextParagraphProperties`, `mergeTextBodyPr`, `getTextListStyleInMaster`, `mergeFill`, `mergeFillNotFollow`, `mergeLnNotFollow`, `mergeEffectNotFollow`, `mergeGeomNotFollow`, `mergeScene3DNotFollow`, `mergeShape3DNotFollow` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->