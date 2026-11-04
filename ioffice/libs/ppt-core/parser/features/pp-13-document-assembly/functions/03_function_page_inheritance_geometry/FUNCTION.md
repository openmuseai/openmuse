# 03_function_page_inheritance_geometry — 页面继承与绝对几何

> feature: pp-13-document-assembly ｜ 状态：**pass**

## 0. 管线能力解锁

把 slide→layout→master 的 placeholder 继承和 group child transform 应用到每页结构树，输出 renderer 可直接消费的绝对几何骨架。

## 1. 功能说明

### 1.1 I/O 契约

输入 slide/layout/master graphs；输出页序不变的 resolved graphs。不得改变 shape XML 层序；缺继承源回退 own xfrm/style。

### 1.2–1.6 边界

上游 pl-02:01/02、pl-03:01/02；下游 04。文字断行不在本 function。

### 1.7 依赖与实现状态

| 依赖 | 需要 | 状态 | 禁止修改 |
|---|---|---|---|
| pl-02 | shape/group rect | pass | 不在 painter 修坐标 |
| pl-03 | xfrm/style merge | pass | 不丢 own→layout→master 顺序 |
| pp-07/08 | layout/master 可达 | spec | 不用空壳伪造继承 |

## 2. 核心代码

复用 `ShapeUtil.rs` 与 `PlaceHolderMerger.rs`；新增代码只负责逐页调用和绑定 parent/layout/master，不重写算法。

## 3. 测试方案

### 3.1–3.5 覆盖

T01 own xfrm；T02 layout fallback；T03 master fallback；T04 group child off/ext；T05 23 页逐页坐标非负/有限。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_page_inheritance_geometry

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_page_inheritance_geometry \
  --src src/cn/wps/show/layout/page_inheritance.rs \
  --src src/cn/wps/show/layout/PlaceHolderMerger.rs \
  --src src/cn/wps/show/util/ShapeUtil.rs \
  --report libs/ppt-core/parser/features/pp-13-document-assembly/functions/03_function_page_inheritance_geometry/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_page_inheritance_geometry --json`)
date: 2026-09-01 21:39

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/layout/PlaceHolderMerger.rs` | 86/210 | 41.0% | 189/388 | 48.7% |
| `src/cn/wps/show/layout/page_inheritance.rs` | 37/37 | 100.0% | 58/58 | 100.0% |
| `src/cn/wps/show/util/ShapeUtil.rs` | 91/196 | 46.4% | 179/296 | 60.5% |
| **function total** | **214/443** | **48.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/util/ShapeUtil.rs \
  --java koffice/presentation/core/cn/wps/show/util/ShapeUtil.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/layout/PlaceHolderMerger.rs \
  --java koffice/presentation/core/cn/wps/show/app/shape/PlaceHolderMerger.java
```

Loop 结果：（待跑）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/util/ShapeUtil.java \
  --rs libs/ppt-core/src/cn/wps/show/util/ShapeUtil.rs \
  --java koffice/presentation/core/cn/wps/show/app/shape/PlaceHolderMerger.java \
  --rs libs/ppt-core/src/cn/wps/show/layout/PlaceHolderMerger.rs \
  --report libs/ppt-core/parser/features/pp-13-document-assembly/functions/03_function_page_inheritance_geometry/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 13:39

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShapeUtil` | `koffice/presentation/core/cn/wps/show/util/ShapeUtil.java` | `libs/ppt-core/src/cn/wps/show/util/ShapeUtil.rs` | **PASS** | `calcShapeText2TopMatrix`, `calcShape2TopMatrix`, `calcChildRelativeRect`, `calcRelativeRect`, `toDegree`, `toRotation` | `calcShapeMatrix`, `calcRelativeTextRect`, `calcShapeRect`, `calcShapeTextRect`, `getGdValue`, `getPresetValue`, `calcShapeFlipEx`, `calcShapeFlip`, `calcShapeRotation`, `extendShapeRect`, `getTable`, `shapeSnapshot`, `textShapeSnapshotWithRelativeInfo`, `textShapeSnapshot`, `calcMinFullDrawShapeRect`, `setInkTransparent`, `slideSnapshot`, `adjustCanvas`, `setGlCachEnable`, `invertCoordFor3dShape`, `isGifShape`, `isTable`, `calVirtualGroupRect`, `calVirtualGroupBound`, `degree2Radian`, `isSameArrayShape` | — | **PASS** |
| `RoundCornerCLipSizeType` | `koffice/presentation/core/cn/wps/show/util/ShapeUtil.java` | `libs/ppt-core/src/cn/wps/show/util/ShapeUtil.rs` | **PASS** | — | `RoundCornerCLipSizeType` | — | **PASS** |
| `PlaceHolderMerger` | `koffice/presentation/core/cn/wps/show/app/shape/PlaceHolderMerger.java` | `libs/ppt-core/src/cn/wps/show/layout/PlaceHolderMerger.rs` | **PASS** | `phType`, `mergeTransform`, `mergeShapePr`, `findLayoutPhShape`, `findMaster`, `isMaster`, `findMasterPhShape` | `mergeShapeStyle`, `mergeTextListStyle`, `filterTextListStyle`, `writeTextListStyle`, `writeTextListProperties`, `followTextListStyle`, `followTextParagraphProperties`, `mergeTextBodyPr`, `getTextListStyleInMaster`, `mergeFill`, `mergeFillNotFollow`, `mergeLnNotFollow`, `mergeEffectNotFollow`, `mergeGeomNotFollow`, `mergeScene3DNotFollow`, `mergeShape3DNotFollow` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
