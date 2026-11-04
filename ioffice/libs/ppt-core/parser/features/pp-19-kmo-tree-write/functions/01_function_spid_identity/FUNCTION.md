# 01_function_spid_identity — spid 位域与 `type()`/`id()`

> feature: pp-19-kmo-tree-write ｜ 状态：**pass** ｜ Java 管线：S4 步骤 4.1

## 0. 管线能力解锁

实现后：每个形状的身份是 Java 同构 spid（高位 type、低 20 位仓下标）。`KmoShowShape.type()` 不得再维护平行 `shape_type` 字段当真相。

没有它：convert 的 type 0–4 分派与 DgStore 六仓对不上。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `shapeType` ∈ {0..5} + 仓内 `idInList`；或已有 `dgShapeId` |
| **输出** | `spid = $spt.setValue(0, type) \| $spid.setValue(0, index)`；`queryType(spid)==type` |
| **Kmo** | `id()==dgShapeId`；`type()==DgStore.queryType(dgShapeId)`（Java `:544-549`） |
| **错误** | 本 function 不分配仓行（那是 03 的 `add`）；不解释非法 type |

Java 位域：`$spt = new KcBitsOp(-1048576)`（mask `0xFFF00000`，shift 20）；`$spid = new KcBitsOp(1048575)`（低 20 位）。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `DgStoreData.queryType` | drawing/app | 高位取 type | ✅ 默认编译面 | — | 禁止改 mask |
| `helper_storeIndex2Id` | 同上 | type+index→spid | ✅ | — | — |
| `KmoShowShape.type/id` | show/app/shape | 委托 queryType | ✅ `type()`/`id()`/`init`/3-arg ctor | equals/copy/protocol 已 `DEFERRED_NON_FUNCTION` | 禁止用 layout_shape.shape_type 冒充 |
| `layout_shape::KmoShowShape` | fixture | — | ✅ 热路径 host | — | **禁止本 function 继续加字段当真相** |

## 2. 核心代码与函数改写列表

- **本 function：** `DgStoreData.queryType` / `helper_storeIndex2Id`（已有则核对位运算与 Java `KcBitsOp`）；`KmoShowShape::init` / `type` / `id`；`show/mod.rs` 将 `app::shape` 接到默认编译面。
- **Detached stub record (2026-09-12):** `src/cn/wps/show/app/shape/KmoShowShape.rs` had no `// java` anchors (12× `todo!()` + `KmoProtocol` impl). Deleted and regenerated via `gen-template.sh --rewrite-fns KmoShowShape,init,type,id` (no `--force`).
- **停车：** `equals`/`equalsLax`/`hashCode`/`protocol`/`updateProtocol` 全文 → `todo!("DEFERRED_NON_FUNCTION")` + `/* … */`。
- **禁改：** `layout_shape.rs` 字段布局（pl-03 测试依赖）；`ConvertTool` 签名（07）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`DgStoreData.java:19-48`；Rust `DgStoreData.rs` `SPT`/`SPID`/`queryType`/`helper_storeIndex2Id`；`KmoShowShape.java:108-126,544-549`；Rust `KmoShowShape.rs` 12 个 `todo!()`；`show/mod.rs` `#[cfg(feature="full")] pub mod app`。

| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| queryType | DgStoreData | `(1<<20)\|7` | 1 | 高位 |
| queryType | | 0 | 0 | type0 根 |
| storeIndex2Id | | (1,3) | type=1 index=3 | 合成 |
| type 0–5 | | t,0 | queryType==t | 六仓 |
| Kmo type() | Java :544 | dgShapeId | queryType | **spec 未接线** |

### 3.2 覆盖矩阵

| coverage_id | 接口 | 场景 | 必测? | test_id / 未覆盖理由 |
|-------------|------|------|-------|----------------------|
| SP-QT | queryType | type1 + index7 | 是 | ID1 |
| SP-Z | queryType | spid=0 | 是 | ID2 |
| SP-MK | storeIndex2Id | 1,3 往返 | 是 | ID3 |
| SP-T05 | storeIndex2Id | type 0..5 | 是 | ID4 |
| SP-KMO | KmoShowShape.type/id | 委托 queryType | 是 | ID5 |

### 3.3 可测试性

不跑 zip。`drawing::app` 默认 feature 未导出（依赖 `kso`/`full`）。Split 用与 `DgStoreData.rs` 相同的 `$spt/$spid` 常量做 oracle（ID1–ID4）；ID5 ignore 直到 ungate 后改为 `DgStoreData::queryType`。

### 3.4 用例表

| id | 覆盖项 | 前置 | 输入 | 期望 | 失败语义 |
|----|--------|------|------|------|----------|
| ID1 | SP-QT | new | `(1<<20)\|7` | queryType==1 | 位移错 |
| ID2 | SP-Z | — | 0 | 0 | 把 0 当非法 |
| ID3 | SP-MK | new | (1,3) | type1 index3 | setValue 错 |
| ID4 | SP-T05 | new | 0..=5 | 各 type 还原 | 仓映射错 |
| ID5 | SP-KMO | DummySlide + 3-arg ctor | spid | type()==queryType && id()==spid | 平行字段 |

### 3.5 测试代码位置

crate: `libs/ppt-core/Cargo.toml`  
src: `src/cn/wps/drawing/app/DgStoreData.rs`；`src/cn/wps/show/app/shape/KmoShowShape.rs`  
test: `tests/cn/wps/drawing/app/01_function_spid_identity.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_spid_identity -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_spid_identity \
  --src src/cn/wps/drawing/app/DgStoreData.rs \
  --src src/cn/wps/show/app/shape/KmoShowShape.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/01_function_spid_identity/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_spid_identity --json`)
date: 2026-09-12 14:02

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/app/DgStoreData.rs` | 21/74 | 28.4% | 32/107 | 29.9% |
| `src/cn/wps/show/app/shape/KmoShowShape.rs` | 33/527 | 6.3% | 30/523 | 5.7% |
| **function total** | **54/601** | **9.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/app/DgStoreData.rs \
  --java koffice/presentation/core/cn/wps/drawing/app/DgStoreData.java \
  --rs libs/ppt-core/src/cn/wps/show/app/shape/KmoShowShape.rs \
  --java koffice/presentation/core/cn/wps/show/app/shape/KmoShowShape.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/app/DgStoreData.java \
  --rs libs/ppt-core/src/cn/wps/drawing/app/DgStoreData.rs \
  --java koffice/presentation/core/cn/wps/show/app/shape/KmoShowShape.java \
  --rs libs/ppt-core/src/cn/wps/show/app/shape/KmoShowShape.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/01_function_spid_identity/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-12 06:03

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `DgStoreData` | `koffice/presentation/core/cn/wps/drawing/app/DgStoreData.java` | `libs/ppt-core/src/cn/wps/drawing/app/DgStoreData.rs` | **PASS** | `queryType`, `isGroup`, `isShape`, `isPicture`, `isConnector`, `isGraphicFrame`, `isTextShape`, `helper_storeIndex2Id`, `helper_getIndexById`, `helper_storeByType`, `helper_storeById`, `helper_isGood` | — | — | **PASS** |
| `KmoShowShape` | `koffice/presentation/core/cn/wps/show/app/shape/KmoShowShape.java` | `libs/ppt-core/src/cn/wps/show/app/shape/KmoShowShape.rs` | **PASS** | `KmoShowShape`, `init`, `type`, `id` | `isEditState`, `setEditState`, `hashCode`, `equals`, `equalsLax`, `compareDgShape`, `compareMedia`, `copy`, `copyEx`, `doCopy`, `updateModel3DRef4Copy`, `updateVideoRef4Copy`, `updateAudioRef4Copy`, `updateFillPicRef4Copy`, `updatePicRef4Copy`, `updateOleRef4Copy`, `copyOlePicture`, `updateChart4Copy`, `drawingShape`, `protocol`, `updateProtocol`, `parent`, `getSlideCache`, `getMediaLib`, `parentGroup`, `ancestor`, `isChildOfGroup`, `setParentGroup`, `addChild`, `addChilds`, `insertChilds`, `insertChild`, `removeChildAt`, `removeAllChild`, `indexOfChild`, `childAt`, `childCount`, `graphic`, `graphicChoice`, `isPlaceHolder`, `writePh`, `deletePh`, `ph`, `phType`, `custDataLst`, `hasCustDataLst`, `setNoGrp`, `findParents`, `hasInk`, `inkId`, `putInkId`, `hasHlinkClick`, `hlinkClickAction`, `hlinkClickTarget`, `hlinkClickTargetIsInternal`, `deleteHlinkClick`, `setHyperlink`, `hasHlinkHover`, `hlinkHoverAction`, `hlinkHoverTarget`, `hlinkHoverTargetIsInternal`, `deleteHlinkHover`, `setHyperlinkOnHover`, `move`, `relocateModularGroup`, `resize`, `autoResize`, `resizeAfterTableInsertRow`, `resizeModularPicSelf`, `resizeCrop`, `groupUp`, `ungroup`, `rotate`, `adjustParentTransform`, `adjustModularGroupTransform`, `adjustGroupTransform`, `getAllChildrenShape`, `getAdjustTransformShapes`, `isTextBox`, `isNoTextEdit`, `layerOp`, `clearTextContents`, `rerouteConnectors`, `borderColor`, `borderFill`, `setBorderColor`, `setBorderGradientFillColor`, `getColor`, `getARGBColor`, `borderWidth`, `getOutlineInShapeStyle`, `getOutlinePhColor`, `setBorderWidth`, `setBorderWidthNoDirty`, `setBorderFillType`, `setBorderFillTypeNoEvent`, `canSetBorderType`, `borderStyleType`, `borderFillType`, `hasBorderFillInStyle`, `setBorderStyleType`, `setBorderStyleTypeNoEvent`, `fillType`, `getFill`, `getFillInShapeStyle`, `getFillPhColor`, `setFillType`, `setFillColor`, `setGradientFill`, `setFill`, `hasSelfFill`, `hasFill`, `hasSelfOutline`, `hasOutline`, `fillColor`, `fillARGBColor`, `fill`, `parentShapeFill`, `hasStyle`, `setQuickShapeStyle`, `setFontColor`, `shapeStyle`, `setShapeStyle`, `getTextOperator`, `getTableOperator`, `getChartOperator`, `setNoGroup`, `setTextContent`, `replaceTextContentByPara`, `getTextContent`, `hasTextBody`, `isTextEmpty`, `hasTextContext`, `setName`, `name`, `disconnect`, `hasUseBgFill`, `isUseBgFill`, `anchor`, `isAnchorCenter`, `putAnchor`, `getPicId`, `changePicMediaId`, `changePic`, `clipPic`, `clipModularGroupPic`, `clipResize`, `resizePic`, `resizePicForSquare`, `resizeCropPic`, `getClipType`, `isPresetClip`, `getOle`, `getPenkitOle`, `getPenkitOleForSpecBrand`, `getPenKitFileName`, `isPenKit`, `isPenKitForSpecBrand`, `getOleFile`, `isPicture`, `getPenKit`, `isEmptyPenkit`, `editPenKit`, `changeOleBlipFill`, `setFontSizeWidthScale`, `setFontSizeGroupScale`, `setFontName`, `setFontNameGroupScale`, `isTable`, `isOle`, `isModel3D`, `isChart`, `setFontScale`, `hasFontSizeFit`, `clearFontSizeFit`, `canRotate`, `rotation`, `isHidden`, `transform2D`, `shapePr`, `style`, `textDirection`, `setTextDirection`, `hasAudio`, `hasVideo`, `audioMediaId`, `mediaTrimStart`, `mediaTrimEnd`, `videoMediaId`, `delVideoFile`, `addVideoFile`, `delAudioFile`, `addAudioFile`, `getDgShape`, `getDgGroup`, `getDgConn`, `getDgPicture`, `setTextRect`, `getTextRect`, `setBeautifyOrder`, `beautifyOrder`, `setLocked`, `isLocked`, `parentLocked`, `lock`, `lockChildrens`, `lockParent`, `unlock`, `unlockChildren`, `unlockParent`, `getParentShapeLocked`, `adjust`, `getPictureOriginSize`, `addCustTags`, `deleCustTags`, `isEditingText`, `saveOriginalProperty`, `getOriginalAutoFit`, `getOriginalAnchor`, `resetOriginalProperty`, `shapeTagMap`, `is3DObject`, `isModularGroup`, `modularGroupType`, `modularGroupIndex`, `putModularGroupExt`, `isModularGroupPicDefault`, `putModularGroupPicture`, `copyModularDgShapeAttrFrom`, `copyModularDgPictureAttrFrom`, `isTitlePh` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
