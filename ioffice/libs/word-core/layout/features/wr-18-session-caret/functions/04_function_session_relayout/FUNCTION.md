# 04_function_session_relayout — 改文档后全量重排

| 项 | 值 |
|----|----|
| feature | wr-18-session-caret |
| order | 4 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `DocumentSession::layout(fonts, view)` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
对 **已在堆上的** `TextDocument` 跑 wr-09..15，产出 `LayoutFrame`。编辑路径禁止 `layout_document(bytes)` 重开 zip。

### 1.2 输入
01 session（可先 02 append）+ `FontBlob[]` + `ViewParams`。

### 1.3 输出
`LayoutFrame`；`DrawText.text` 含原故事与新段（AAA 用 MvpLatin cmap）。

### 1.4 上游
01、02、`layout_text_document`。

### 1.5 下游
Painter / 03 caret（Frame 形状）。

### 1.6 不变量与函数改写列表
1. 两次 layout 无 mutate → 可见字符串相同。
2. append `"AAA"` 后同时含 Hello 与 AAA。
3. **改写列表：** `DocumentSession::layout`。禁止改 LineLayouter 冒充本 function。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `layout_text_document` | LayoutDocument | doc+fonts+view → Frame | 是 | wasm | 禁止改断行「修」AAA |
| FontHost / MvpLatin | wr-03 | cmap A–Z | 是 | 系统 Arial | 禁止 CJK append 当本测 |
| wr-09 RunStream | wr-09 | 主故事 runs | 是 | 表 story | — |

### 1.8 函数改写列表
- `DocumentSession::layout`

## 2. 核心代码

`layout_text_document(&self.doc, fonts, view)`。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| layout | DocumentSession.rs | 仅 open | Hello | 主路径 |
| layout | append AAA | Hello+AAA | 编排 |
| layout | 两次无 mutate | 字符串相等 | 幂等 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| SR-OPEN | layout | 无 mutate | 是 | SR1 |
| SR-APPEND | layout | AAA | 是 | SR2 |
| SR-IDEM | layout | 两次 | 是 | SR3 |

### 3.3 可测试性
编排 function：允许 open+append+layout。字体仅 MvpLatin。不断言 ypos 金文件。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| SR1 | SR-OPEN | Hello | layout | 含 Hello | bytes-in 才有字 |
| SR2 | SR-APPEND | Hello | append AAA + layout | Hello 且 AAA | 只改 rope 未进 RunStream |
| SR3 | SR-IDEM | Hello | layout×2 | 字符串相等 | 第二次清空 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/api/DocumentSession.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/api/SessionRelayout.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_session_relayout

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 04_function_session_relayout \
  --src src/cn/wps/moffice/writer/api/DocumentSession.rs \
  --report libs/word-core/layout/features/wr-18-session-caret/functions/04_function_session_relayout/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_session_relayout --json`)
date: 2026-08-27 10:28

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/DocumentSession.rs` | 29/36 | 80.6% | 41/50 | 82.0% |
| **function total** | **29/36** | **80.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
本 function 不改 Java 叶。若 loop 要动 `layout_text_document` 所在文件，对该 `.rs` 与对应 Java（若有）跑 `check-template-layout.mjs`。`LayoutDocument.rs` 为产品编排，无整文件 Java 对等。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java
```

本轮 `TextDocument` 布局：**PASS**。

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs \
  --report libs/word-core/layout/features/wr-18-session-caret/functions/04_function_session_relayout/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-27 02:27

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TextDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `TextDocument`, `_create`, `dispose`, `getSubdocument`, `createSubDocument`, `createAllSubDocument`, `setInIOState`, `isInIOState`, `onSaveSuccessed`, `getMainDocument`, `getProperty`, `getStyles`, `getCharBuffer`, `getTextPool`, `getUUSti`, `getUUNumberingId`, `type`, `getKParagraphsID` | `setStatAnalytics`, `getStatAnalytics`, `setDocumentCallback`, `getTmpRootDir`, `getFileManager`, `clearCachedValues`, `getShapeIDAlloc`, `getMediaLib`, `getOleManager`, `getCoreDataAcceptor`, `newDocumentFromTemplate`, `newDocumentFromAutoSave`, `newDocument`, `setDocumentProperties`, `generateName`, `openDocument`, `init`, `startLoading`, `endLoading`, `correctDocumentData`, `correctParaIds`, `correctDataSections`, `correctDataFields`, `correctOleFields`, `correctOleData`, `correctReadonlyTrackchanges`, `appendHeaderFooterNodeSections`, `finishIO`, `correctHeaderFooter`, `deleteRange`, `clearActionCorrectingDataSections`, `isLegalRange`, `getPLCSection`, `getFirstSectionNode`, `analyseNumberParagraphAppended`, `isParagraph`, `correctStructureError`, `deleteIllegalSection`, `fixAllError`, `clearError`, `onStartLoad`, `onEndLoad`, `setProperty`, `liteExchangeDocument`, `charCount`, `getGlobalInkUserID`, `setGloabalInkUserID`, `cryptInfo`, `isFinishDumpObjects`, `setIoError`, `getIoError`, `isInUserAction`, `startMonitor`, `stopMonitor`, `isLegal`, `loadDocument`, `doLoadDocument`, `loadHtmlDocument`, `loadMhtDocument`, `loadDocDocument`, `endDocReader`, `toFileFormat`, `loadRtfDocument`, `getRtfReader`, `loadDocxDocument`, `endDocxReader`, `loadXml03Document`, `loadXml07Document`, `loadEpubDocument`, `loadUot3Document`, `loadTxtDocument`, `endEpubReader`, `endTextReader`, `insert`, `isEncryption`, `isProEncryption`, `insertBefore`, `insertAfter`, `isLoadSuccess`, `setLoadSuccess`, `getListTemplates`, `getLists`, `getListParagraphs`, `getAutoNumTable`, `getName`, `getPath`, `getRaf`, `save`, `saveAs`, `saveFile`, `setSaveSuccessedListener`, `onEvidenceSuccessd`, `exportSecurityFile`, `saveFile4Normal`, `saveFile4Encryption`, `encryptFileByAgentClient`, `encryptFileByOnlineSecurity`, `getWriter`, `updateShape`, `backUp`, `qingSaveBackup`, `getFileFormat`, `saveToSrcFileFlag`, `close`, `updateModifys`, `getLeafCommands`, `updateActions_undo`, `updateActions_redo`, `fixCommand`, `setOnModifiedListener`, `setOnModifiedListener4Stat`, `addWriterOpenEventListener`, `removeWriterOpenEventListener`, `isModified`, `isAutoSave`, `setModified`, `doModified`, `notify_modified`, `submitModified`, `getEvidenceData`, `setEvidenceData`, `getMetadata`, `getAppVersion`, `setAppVersion`, `updateDocument`, `getContentThumb`, `getFontTable`, `getTheme`, `addBuiltinStyles`, `getRsids`, `_saveSelection`, `_restoreSelection`, `waitSync`, `addTransactionListener`, `removeTransactionListener`, `start`, `submit`, `commit`, `update`, `transaction_lock`, `transaction_unlock`, `startInner`, `getInterruptNext`, `setInterruptNext`, `rollback`, `clearRedo`, `canExecuteUndoOrRedo`, `undo`, `dumpForException`, `redo`, `canUndo`, `canRedo`, `clearUndoRedo`, `getViews`, `getAsiaTypography`, `getPassword`, `hasReadPassword`, `setPasswd`, `initPasswd`, `getEncryptType`, `getLength`, `getRange`, `getDigest`, `getSections`, `isEmbedBalloonMode`, `setIsEmbedBalloonMode`, `setTrackChange`, `getTrackChange`, `hasComments`, `hasRevisions`, `getRevisionsAuthor`, `setRevisionsAuthor`, `getUnknownDataMap`, `getVBAPath`, `setVBAPath`, `getUUCommentId`, `getUUPropertySetRegistryId`, `addBuiltinTableStyle`, `getUULsid`, `getSSClassLoader`, `getDrawingRIDMap`, `getShapeId2DrawingRID`, `getDiagramShapeId`, `isDisplayBackground`, `requestUpdateBkShape`, `getImmutableBkShape`, `getBackground`, `getShapeProprytyID`, `addShapeCommand`, `setBackground`, `setStopUpdateTextDocment`, `isStopUpdateTextDocment`, `onFinishDumpObjects`, `waitFinishDumpObjects`, `setIoThreadIdToRangeLock`, `setChartShapes`, `getChartShapes`, `beforeSectionChanged`, `afterSectionChanged`, `getEncoding`, `setEncoding`, `setLoadListener`, `isContainMathEquations`, `hasMainDocumentShape`, `hasDocField`, `setFileTime`, `setFileTimeSaved`, `setFileTimeSavedAs`, `backupFileTime`, `recoverFileTime`, `updateCore4Save`, `getDocumentVariables`, `getDocumentProtection`, `getOnlineSecurityTool`, `fillFirstPageFont`, `getMainFirstPageEndCp`, `getFirstPageFontList`, `hasAudioComment`, `getCompatibilityMode`, `isDirty`, `isSecurityFile`, `setSlimDirty`, `open`, `isEmptySlides`, `setWritePassword`, `hasWritePassword`, `getDocProtection`, `getBookProtection`, `getFileExtFormat`, `doFormatConvert`, `getShareObjIndexdMap`, `getFootnoteEndnoteExchange`, `getFootnoteEndnote`, `getSectionProperty`, `getSTHostApp`, `addOle`, `getKCustomIcv`, `isProtectOn_readOnly`, `isProtectOn_TrackChangeCtrl`, `isProtectOn`, `isThrowDocumentProtectionException`, `beginSectionChange`, `endSectionChange`, `createInitialSepx`, `createDefaultPageSetup`, `getMergedSepx`, `orientation_equal`, `normalizeOrientation`, `changeSepx`, `setPageSetup`, `getPageSetup`, `setPageOrientation`, `exportAsArray`, `advanceGVmlOffset`, `setGridResizeRequest`, `getGridResizeRequest`, `getSaveAccelerator`, `setDebugger`, `isDebugging`, `onDebug`, `collectOpenedDocumentInfo`, `hasHeadings`, `initPaperRecognition`, `isPaper`, `setScrolling`, `isOpenIdleScroll`, `getParaText`, `getSubDocParaIds` | — | **PASS** |
| `PapxTableListener` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | — | `PapxTableListener`, `beforeAddEntry`, `afterAddEntry`, `beforeRemoveEntry`, `afterRemoveEntry`, `beforeDataChange`, `afterDataChange` | — | **PASS** |
| `ChpxTableListener` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | — | `ChpxTableListener`, `beforeAddEntry`, `afterAddEntry`, `beforeRemoveEntry`, `afterRemoveEntry`, `beforeDataChange`, `afterDataChange` | — | **PASS** |
| `UUNumberingId` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `UUNumberingId`, `getNextId`, `getCurrentId` | — | — | **PASS** |
| `UULsid` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `UULsid`, `getNextId`, `getCurrentId` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
