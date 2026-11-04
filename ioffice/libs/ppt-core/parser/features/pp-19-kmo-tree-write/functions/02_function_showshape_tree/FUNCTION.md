# 02_function_showshape_tree — addChild / initRoot / shapeCount

> feature: pp-19-kmo-tree-write ｜ 状态：**pass** ｜ Java 管线：S4 步骤 4.2

## 0. 管线能力解锁

实现后：spTree 是可计数的孩子列表，且 **sw 孩子 + dg group id** 双写。`addChild` 写 `children` + `DgGroup.add(child.id())`；`shapeCount` 委托 `root.childCount()`。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `KmoShowShape(parent, 0)` 根；`addChild`/`addShape` 的 type1 子节点 |
| **输出** | `children` 含 child；`getDgGroup().add(child.id())`；`shapeCount()==root.childCount()` |
| **initRoot** | 替换 ctor 预建的占位根（Java `KmoShapeTree:29-32` 先 `new KmoShowShape(parent,0)`） |
| **错误** | type≠0 的节点 `addChild`：Java 仍往 `this.children` 写（ctor 仅 type==0 分配 list）。Rust 对齐：非组 `addChild` 与 Java 同（NPE 若 children 未分配） |

Java `addChild:451-456`：`child.parentGroup=this`；`children.add`；`getDgGroup().add(child.id())`。  
`shapeCount:558`：`return this.root.childCount()`。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `KmoShowShape.addChild` | 本 function | 双写 | ✅ children + getDgGroup.add | addChilds/insert/remove | 禁止只改 layout_shape.children |
| `KmoShapeTree.initRoot/shapeCount` | 本 function | 换根+计数 | ✅ | groupUp/ungroup/disconnect | — |
| `DgGroup.add` | drawing | 子 id 列表 | ✅ size/childAt/find | adjustGroupXfrm / Shape blob roundtrip | 禁止空 add；禁止项目 stub |
| spid | 01 | child.id() | ✅ | — | — |

## 2. 核心代码与函数改写列表

- **本 function：** `addChild`；`childAt`/`childCount`/`indexOfChild`；`KmoShapeTree` ctor / `initRoot` / `getRoot` / `shapeCount` / `addShape`。
- **停车：** `groupUp`/`ungroup`/`disconnectShape`/`doCopy`/`initSpTreeGroupTransform`/`findPhByType`/`getAllPh`。
- **禁改：** PlaceHolderMerger；layout_shape 测试夹具字段名。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`KmoShowShape.java:451-456`；`KmoShapeTree.java:29-44,550-564`；Rust tree 14 行 `shapeCount=0`；layout_shape `KmoShapeTree.children: Vec` 是当前热路径。

| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| addChild | :451 | type0 父 + type1 子 | len=1 + dg add | 双写 |
| shapeCount | :558 | 两子 | 2 | 委托 childCount |
| initRoot | :42 | 新根 | getRoot 换对象 | 替换占位 |
| 空树 | ctor | 无 add | shapeCount 0（占位根无子） | 0 次 |

### 3.2 覆盖矩阵

| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| TR-ADD | addChild | 1 child | 是 | TR1 |
| TR-N | addChild | N=2 | 是 | TR2 |
| TR-CNT | shapeCount | 委托 | 是 | TR3 |
| TR-ROOT | initRoot | 换根 | 是 | TR4 |
| TR-0 | 空 | 0 child | 是 | TR5 |

今日可编译的对照：layout_shape `children.len()`（TR0）——**不是** pass 口径。

### 3.3 可测试性

默认编译面 `show/app/shape` + `drawing/app/shape::DgGroup`。TR1–TR5 用真实 `KmoShowShape`/`KmoShapeTree`/`DgGroup`。

### 3.4 用例表

| id | 覆盖项 | 期望 | 失败语义 |
|----|--------|------|----------|
| TR0 | fixture 对照 | layout_shape vec len | 夹具坏 |
| TR1–TR5 | 见矩阵 | Java 树语义 | addChild/shapeCount 错 |

### 3.5 测试代码位置

test: `tests/cn/wps/show/app/shape/02_function_showshape_tree.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_showshape_tree -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_showshape_tree \
  --src src/cn/wps/show/app/shape/KmoShowShape.rs \
  --src src/cn/wps/show/app/shape/KmoShapeTree.rs \
  --src src/cn/wps/drawing/app/shape/DgGroup.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/02_function_showshape_tree/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_showshape_tree --json`)
date: 2026-09-12 15:47

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/app/shape/DgGroup.rs` | 20/59 | 33.9% | 27/66 | 40.9% |
| `src/cn/wps/show/app/shape/KmoShapeTree.rs` | 19/138 | 13.8% | 25/147 | 17.0% |
| `src/cn/wps/show/app/shape/KmoShowShape.rs` | 57/552 | 10.3% | 81/585 | 13.8% |
| **function total** | **96/749** | **12.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/app/shape/KmoShowShape.rs \
  --java koffice/presentation/core/cn/wps/show/app/shape/KmoShowShape.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/app/shape/KmoShapeTree.rs \
  --java koffice/presentation/core/cn/wps/show/app/shape/KmoShapeTree.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/app/shape/DgGroup.rs \
  --java koffice/presentation/core/cn/wps/drawing/app/shape/DgGroup.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/app/shape/KmoShowShape.java \
  --rs libs/ppt-core/src/cn/wps/show/app/shape/KmoShowShape.rs \
  --java koffice/presentation/core/cn/wps/show/app/shape/KmoShapeTree.java \
  --rs libs/ppt-core/src/cn/wps/show/app/shape/KmoShapeTree.rs \
  --java koffice/presentation/core/cn/wps/drawing/app/shape/DgGroup.java \
  --rs libs/ppt-core/src/cn/wps/drawing/app/shape/DgGroup.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/02_function_showshape_tree/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-12 07:47

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `KmoShowShape` | `koffice/presentation/core/cn/wps/show/app/shape/KmoShowShape.java` | `libs/ppt-core/src/cn/wps/show/app/shape/KmoShowShape.rs` | **PASS** | `KmoShowShape`, `init`, `addChild`, `indexOfChild`, `childAt`, `childCount`, `type`, `id`, `getDgGroup` | `isEditState`, `setEditState`, `hashCode`, `equals`, `equalsLax`, `compareDgShape`, `compareMedia`, `copy`, `copyEx`, `doCopy`, `updateModel3DRef4Copy`, `updateVideoRef4Copy`, `updateAudioRef4Copy`, `updateFillPicRef4Copy`, `updatePicRef4Copy`, `updateOleRef4Copy`, `copyOlePicture`, `updateChart4Copy`, `drawingShape`, `protocol`, `updateProtocol`, `parent`, `getSlideCache`, `getMediaLib`, `parentGroup`, `ancestor`, `isChildOfGroup`, `setParentGroup`, `addChilds`, `insertChilds`, `insertChild`, `removeChildAt`, `removeAllChild`, `graphic`, `graphicChoice`, `isPlaceHolder`, `writePh`, `deletePh`, `ph`, `phType`, `custDataLst`, `hasCustDataLst`, `setNoGrp`, `findParents`, `hasInk`, `inkId`, `putInkId`, `hasHlinkClick`, `hlinkClickAction`, `hlinkClickTarget`, `hlinkClickTargetIsInternal`, `deleteHlinkClick`, `setHyperlink`, `hasHlinkHover`, `hlinkHoverAction`, `hlinkHoverTarget`, `hlinkHoverTargetIsInternal`, `deleteHlinkHover`, `setHyperlinkOnHover`, `move`, `relocateModularGroup`, `resize`, `autoResize`, `resizeAfterTableInsertRow`, `resizeModularPicSelf`, `resizeCrop`, `groupUp`, `ungroup`, `rotate`, `adjustParentTransform`, `adjustModularGroupTransform`, `adjustGroupTransform`, `getAllChildrenShape`, `getAdjustTransformShapes`, `isTextBox`, `isNoTextEdit`, `layerOp`, `clearTextContents`, `rerouteConnectors`, `borderColor`, `borderFill`, `setBorderColor`, `setBorderGradientFillColor`, `getColor`, `getARGBColor`, `borderWidth`, `getOutlineInShapeStyle`, `getOutlinePhColor`, `setBorderWidth`, `setBorderWidthNoDirty`, `setBorderFillType`, `setBorderFillTypeNoEvent`, `canSetBorderType`, `borderStyleType`, `borderFillType`, `hasBorderFillInStyle`, `setBorderStyleType`, `setBorderStyleTypeNoEvent`, `fillType`, `getFill`, `getFillInShapeStyle`, `getFillPhColor`, `setFillType`, `setFillColor`, `setGradientFill`, `setFill`, `hasSelfFill`, `hasFill`, `hasSelfOutline`, `hasOutline`, `fillColor`, `fillARGBColor`, `fill`, `parentShapeFill`, `hasStyle`, `setQuickShapeStyle`, `setFontColor`, `shapeStyle`, `setShapeStyle`, `getTextOperator`, `getTableOperator`, `getChartOperator`, `setNoGroup`, `setTextContent`, `replaceTextContentByPara`, `getTextContent`, `hasTextBody`, `isTextEmpty`, `hasTextContext`, `setName`, `name`, `disconnect`, `hasUseBgFill`, `isUseBgFill`, `anchor`, `isAnchorCenter`, `putAnchor`, `getPicId`, `changePicMediaId`, `changePic`, `clipPic`, `clipModularGroupPic`, `clipResize`, `resizePic`, `resizePicForSquare`, `resizeCropPic`, `getClipType`, `isPresetClip`, `getOle`, `getPenkitOle`, `getPenkitOleForSpecBrand`, `getPenKitFileName`, `isPenKit`, `isPenKitForSpecBrand`, `getOleFile`, `isPicture`, `getPenKit`, `isEmptyPenkit`, `editPenKit`, `changeOleBlipFill`, `setFontSizeWidthScale`, `setFontSizeGroupScale`, `setFontName`, `setFontNameGroupScale`, `isTable`, `isOle`, `isModel3D`, `isChart`, `setFontScale`, `hasFontSizeFit`, `clearFontSizeFit`, `canRotate`, `rotation`, `isHidden`, `transform2D`, `shapePr`, `style`, `textDirection`, `setTextDirection`, `hasAudio`, `hasVideo`, `audioMediaId`, `mediaTrimStart`, `mediaTrimEnd`, `videoMediaId`, `delVideoFile`, `addVideoFile`, `delAudioFile`, `addAudioFile`, `getDgShape`, `getDgConn`, `getDgPicture`, `setTextRect`, `getTextRect`, `setBeautifyOrder`, `beautifyOrder`, `setLocked`, `isLocked`, `parentLocked`, `lock`, `lockChildrens`, `lockParent`, `unlock`, `unlockChildren`, `unlockParent`, `getParentShapeLocked`, `adjust`, `getPictureOriginSize`, `addCustTags`, `deleCustTags`, `isEditingText`, `saveOriginalProperty`, `getOriginalAutoFit`, `getOriginalAnchor`, `resetOriginalProperty`, `shapeTagMap`, `is3DObject`, `isModularGroup`, `modularGroupType`, `modularGroupIndex`, `putModularGroupExt`, `isModularGroupPicDefault`, `putModularGroupPicture`, `copyModularDgShapeAttrFrom`, `copyModularDgPictureAttrFrom`, `isTitlePh` | — | **PASS** |
| `KmoShapeTree` | `koffice/presentation/core/cn/wps/show/app/shape/KmoShapeTree.java` | `libs/ppt-core/src/cn/wps/show/app/shape/KmoShapeTree.rs` | **PASS** | `KmoShapeTree`, `initRoot`, `getRoot`, `parent`, `shapeAt`, `indexOfShape`, `shapeCount`, `addShape` | `protocol`, `updateProtocol`, `copyFrom`, `copyFromEx`, `doCopy`, `initSpTreeGroupTransform`, `disconnectShape`, `groupUp`, `ungroup`, `ungroupself`, `addPicture`, `addPenKit`, `add3dObj`, `addCurveLine`, `addAudio`, `addNoteAudio`, `addVideo`, `addPictureData`, `addMediaData`, `depth`, `replacePicture2PlaceHolder`, `replaceAudio`, `addPresetShape`, `addInk`, `addTextBox`, `addConnector`, `addTable`, `addChart`, `addShapes`, `insertShapes`, `findPhByType`, `findPhByIndex`, `deleteShape`, `updateParentXfrm`, `getAllPh`, `getVmlDrawing`, `setVmlDrawing`, `hashCode`, `equals`, `equalsLax` | — | **PASS** |
| `DgGroup` | `koffice/presentation/core/cn/wps/drawing/app/shape/DgGroup.java` | `libs/ppt-core/src/cn/wps/drawing/app/shape/DgGroup.rs` | **PASS** | `DgGroup`, `find`, `size`, `childAt`, `add` | `groupUp`, `unGroup`, `initGroupXfrmAfterGroup`, `layerOp`, `insertAll`, `addAll`, `insert`, `remove`, `removeAt`, `removeAllChild`, `childIndex`, `adjustGroupXfrm`, `setGrpTransform2D`, `setTransform2D`, `adjustChildTransform2D`, `disconnect`, `disconnectByGroup` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
