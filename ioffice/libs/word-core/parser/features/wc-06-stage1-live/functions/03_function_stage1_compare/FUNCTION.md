# 03_function_stage1_compare — Stage1 vs Java

| 项 | 值 |
|----|----|
| feature | wc-06-stage1-live |
| order | 3 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `Stage1Compare::diff(java, rust) -> Result<(), FirstDiff>` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
严格比较两份 Stage1 JSON；先文本后 sprm。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | java JSON + rust JSON | 无此类（双跑脚本） |

### 1.3 输出
Ok 或 FirstDiff.path（text / papx[i].193 / …）

### 1.4 上游
01;02。

### 1.5 下游
wc-07。

### 1.6 不变量与函数改写列表
- Stage1 无容差\n- 禁止改 Java dump 迁就 Rust\n- 改写列表：仅 diff

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| 01/02 schema | 本 feature | 字段名 | spec | — | 禁止改 Java 字段名 |

## 2. 核心代码
字段逐项。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| diff | Stage1Compare.rs | two JSON | Result | text/sprm/bad json |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|-------------|----------|-------|---------|
| SC-OK | 全等 | 是 | SC1 |
| SC-TEXT | 文本不同 | 是 | SC2 |
| SC-SPR | 193 不同 | 是 | SC3 |
| SC-BAD | 坏 JSON | 是 | SC4 |

### 3.3 可测试性
手写 JSON。

### 3.4 用例表
| id | 覆盖项 | 输入 | 期望 | 失败语义 |
|----|--------|------|------|----------|
| SC1 | SC-OK | 相同 | Ok | 误报 |
| SC2 | SC-TEXT | 差一字 | FirstDiff text | 吞掉 |
| SC3 | SC-SPR | 193 0 vs 1 | FirstDiff papx | 当默认 |
| SC4 | SC-BAD | 非法 JSON | parse err | panic |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src path: `libs/word-core/src/cn/wps/moffice/writer/api/stage1/Stage1Compare.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/api/stage1/Stage1Compare.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_stage1_compare

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_stage1_compare \
  --src src/cn/wps/moffice/writer/api/stage1/Stage1Compare.rs \
  --report libs/word-core/parser/features/wc-06-stage1-live/functions/03_function_stage1_compare/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_stage1_compare --json`)
date: 2026-09-03 17:13

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/stage1/Stage1Compare.rs` | 32/49 | 65.3% | 43/59 | 72.9% |
| **function total** | **32/49** | **65.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java --rs libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs \
  --report libs/word-core/parser/features/wc-06-stage1-live/functions/03_function_stage1_compare/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-03 09:13

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TextDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `TextDocument`, `_create`, `dispose`, `getSubdocument`, `createSubDocument`, `createAllSubDocument`, `setInIOState`, `isInIOState`, `onSaveSuccessed`, `getMainDocument`, `getProperty`, `getStyles`, `getCharBuffer`, `getTextPool`, `getUUSti`, `getUUNumberingId`, `type`, `getKParagraphsID` | `setStatAnalytics`, `getStatAnalytics`, `setDocumentCallback`, `getTmpRootDir`, `getFileManager`, `clearCachedValues`, `getShapeIDAlloc`, `getMediaLib`, `getOleManager`, `getCoreDataAcceptor`, `newDocumentFromTemplate`, `newDocumentFromAutoSave`, `newDocument`, `setDocumentProperties`, `generateName`, `openDocument`, `init`, `startLoading`, `endLoading`, `correctDocumentData`, `correctParaIds`, `correctDataSections`, `correctDataFields`, `correctOleFields`, `correctOleData`, `correctReadonlyTrackchanges`, `appendHeaderFooterNodeSections`, `finishIO`, `correctHeaderFooter`, `deleteRange`, `clearActionCorrectingDataSections`, `isLegalRange`, `getPLCSection`, `getFirstSectionNode`, `analyseNumberParagraphAppended`, `isParagraph`, `correctStructureError`, `deleteIllegalSection`, `fixAllError`, `clearError`, `onStartLoad`, `onEndLoad`, `setProperty`, `liteExchangeDocument`, `charCount`, `getGlobalInkUserID`, `setGloabalInkUserID`, `cryptInfo`, `isFinishDumpObjects`, `setIoError`, `getIoError`, `isInUserAction`, `startMonitor`, `stopMonitor`, `isLegal`, `loadDocument`, `doLoadDocument`, `loadHtmlDocument`, `loadMhtDocument`, `loadDocDocument`, `endDocReader`, `toFileFormat`, `loadRtfDocument`, `getRtfReader`, `loadDocxDocument`, `endDocxReader`, `loadXml03Document`, `loadXml07Document`, `loadEpubDocument`, `loadUot3Document`, `loadTxtDocument`, `endEpubReader`, `endTextReader`, `insert`, `isEncryption`, `isProEncryption`, `insertBefore`, `insertAfter`, `isLoadSuccess`, `setLoadSuccess`, `getListTemplates`, `getLists`, `getListParagraphs`, `getAutoNumTable`, `getName`, `getPath`, `getRaf`, `save`, `saveAs`, `saveFile`, `setSaveSuccessedListener`, `onEvidenceSuccessd`, `exportSecurityFile`, `saveFile4Normal`, `saveFile4Encryption`, `encryptFileByAgentClient`, `encryptFileByOnlineSecurity`, `getWriter`, `updateShape`, `backUp`, `qingSaveBackup`, `getFileFormat`, `saveToSrcFileFlag`, `close`, `updateModifys`, `getLeafCommands`, `updateActions_undo`, `updateActions_redo`, `fixCommand`, `setOnModifiedListener`, `setOnModifiedListener4Stat`, `addWriterOpenEventListener`, `removeWriterOpenEventListener`, `isModified`, `isAutoSave`, `setModified`, `doModified`, `notify_modified`, `submitModified`, `getEvidenceData`, `setEvidenceData`, `getMetadata`, `getAppVersion`, `setAppVersion`, `updateDocument`, `getContentThumb`, `getFontTable`, `getTheme`, `addBuiltinStyles`, `getRsids`, `_saveSelection`, `_restoreSelection`, `waitSync`, `addTransactionListener`, `removeTransactionListener`, `start`, `submit`, `commit`, `update`, `transaction_lock`, `transaction_unlock`, `startInner`, `getInterruptNext`, `setInterruptNext`, `rollback`, `clearRedo`, `canExecuteUndoOrRedo`, `undo`, `dumpForException`, `redo`, `canUndo`, `canRedo`, `clearUndoRedo`, `getViews`, `getAsiaTypography`, `getPassword`, `hasReadPassword`, `setPasswd`, `initPasswd`, `getEncryptType`, `getLength`, `getRange`, `getDigest`, `getSections`, `isEmbedBalloonMode`, `setIsEmbedBalloonMode`, `setTrackChange`, `getTrackChange`, `hasComments`, `hasRevisions`, `getRevisionsAuthor`, `setRevisionsAuthor`, `getUnknownDataMap`, `getVBAPath`, `setVBAPath`, `getUUCommentId`, `getUUPropertySetRegistryId`, `addBuiltinTableStyle`, `getUULsid`, `getSSClassLoader`, `getDrawingRIDMap`, `getShapeId2DrawingRID`, `getDiagramShapeId`, `isDisplayBackground`, `requestUpdateBkShape`, `getImmutableBkShape`, `getBackground`, `getShapeProprytyID`, `addShapeCommand`, `setBackground`, `setStopUpdateTextDocment`, `isStopUpdateTextDocment`, `onFinishDumpObjects`, `waitFinishDumpObjects`, `setIoThreadIdToRangeLock`, `setChartShapes`, `getChartShapes`, `beforeSectionChanged`, `afterSectionChanged`, `getEncoding`, `setEncoding`, `setLoadListener`, `isContainMathEquations`, `hasMainDocumentShape`, `hasDocField`, `setFileTime`, `setFileTimeSaved`, `setFileTimeSavedAs`, `backupFileTime`, `recoverFileTime`, `updateCore4Save`, `getDocumentVariables`, `getDocumentProtection`, `getOnlineSecurityTool`, `fillFirstPageFont`, `getMainFirstPageEndCp`, `getFirstPageFontList`, `hasAudioComment`, `getCompatibilityMode`, `isDirty`, `isSecurityFile`, `setSlimDirty`, `open`, `isEmptySlides`, `setWritePassword`, `hasWritePassword`, `getDocProtection`, `getBookProtection`, `getFileExtFormat`, `doFormatConvert`, `getShareObjIndexdMap`, `getFootnoteEndnoteExchange`, `getFootnoteEndnote`, `getSectionProperty`, `getSTHostApp`, `addOle`, `getKCustomIcv`, `isProtectOn_readOnly`, `isProtectOn_TrackChangeCtrl`, `isProtectOn`, `isThrowDocumentProtectionException`, `beginSectionChange`, `endSectionChange`, `createInitialSepx`, `createDefaultPageSetup`, `getMergedSepx`, `orientation_equal`, `normalizeOrientation`, `changeSepx`, `setPageSetup`, `getPageSetup`, `setPageOrientation`, `exportAsArray`, `advanceGVmlOffset`, `setGridResizeRequest`, `getGridResizeRequest`, `getSaveAccelerator`, `setDebugger`, `isDebugging`, `onDebug`, `collectOpenedDocumentInfo`, `hasHeadings`, `initPaperRecognition`, `isPaper`, `setScrolling`, `isOpenIdleScroll`, `getParaText`, `getSubDocParaIds` | — | **PASS** |
| `PapxTableListener` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | — | `PapxTableListener`, `beforeAddEntry`, `afterAddEntry`, `beforeRemoveEntry`, `afterRemoveEntry`, `beforeDataChange`, `afterDataChange` | — | **PASS** |
| `ChpxTableListener` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | — | `ChpxTableListener`, `beforeAddEntry`, `afterAddEntry`, `beforeRemoveEntry`, `afterRemoveEntry`, `beforeDataChange`, `afterDataChange` | — | **PASS** |
| `UUNumberingId` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `UUNumberingId`, `getNextId`, `getCurrentId` | — | — | **PASS** |
| `UULsid` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `UULsid`, `getNextId`, `getCurrentId` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
