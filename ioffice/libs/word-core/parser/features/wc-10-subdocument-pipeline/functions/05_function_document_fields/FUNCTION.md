# 05_function_document_fields — createPLCOfDocumentFiels

| 项 | 值 |
|----|----|
| feature | wc-10-subdocument-pipeline |
| order | 5 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SubDocument::{createPLCOfDocumentFiels,getDocumentFileds}`；`createPLC` 末尾调用 |
| 独立性 | 不实现 `addDocumentField` 节点语义 |
| 测试状态 | pass（DF1–DF6；§3.8 SubDocument+TextDocument PASS；§3.9 overall PASS） |
| 阶段 | parse |

## 1. 功能说明

### 1.1 一句话

`createPLCOfDocumentFiels()` 在 `mDocumentFieles == null` 时 `new PLCDocumentFields(this)`；`getDocumentFileds()` 返回同一实例（可空），禁止 `default()` 假对象。

### 1.2 输入

`SubDocument` ctor → `createPLC` → `createPLCOfDocumentFiels`（Java 每个故事都会调）。

### 1.3 输出

- ctor 后 `getDocumentFileds()` 非空
- 二次 `createPLCOfDocumentFiels` 指针不变
- 新表 `isEmpty()` 为 true（无节点）
- `TextDocument.hasDocField()` 在空表上为 false（Java 还要求 `!= null`）

不测：`doAddTextListener`；`addDocumentField`。

### 1.4–1.5

上游 03。下游 `hasDocField` / DocFieldImporter / LineLayouter。

### 1.6 硬约束

去掉 Copy 影子；`use data::PLCDocumentFields`。禁止假 `isEmpty→true` 常数。

**改写列表：** `SubDocument.rs` create/get + `createPLC` 调用 + `Document` impl；`PLCDocumentFields.rs` 只 ctor/`isEmpty`（Deref PLC）。`TextDocument.hasDocField` 按 Java 空检查接线。

### 1.7.1

| # | Java | 替代 |
|---|------|------|
| 1 | `doAddTextListener(fields, false)` | DEFERRED |
| 2 | `addDocumentField` | 后续 importer |

## 3. 测试方案

| id | 期望 |
|----|------|
| DF1 | ctor 后 getter Some |
| DF2 | 二次 create 同指针 |
| DF3 | isEmpty true |
| DF4 | trait 与固有同址 |
| DF5 | hasDocField false |
| DF6 | hello-world parse 后 Some |

### 3.5–3.11

crate `libs/word-core`；src `SubDocument.rs` `PLCDocumentFields.rs`；test `04` 同目录 `05_function_document_fields.rs`。

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 05_function_document_fields

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 05_function_document_fields \
  --src src/cn/wps/moffice/writer/core/SubDocument.rs \
  --src src/cn/wps/moffice/writer/core/TextDocument.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/05_function_document_fields/FUNCTION.md
```

### 3.7

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 05_function_document_fields --json`)
date: 2026-09-30 16:02

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/core/SubDocument.rs` | 284/692 | 41.0% | 308/797 | 38.6% |
| `src/cn/wps/moffice/writer/core/TextDocument.rs` | 240/1181 | 20.3% | 318/1405 | 22.6% |
| **function total** | **524/1873** | **28.0%** |  |  |
<!-- coverage-report:end -->

### 3.8

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java
```

`PLCDocumentFields.rs` 本轮只**调用**已登记 ctor/`isEmpty`（Deref `PLC`），不扩写 FieldProps 全叶；故 §3.8/§3.9 不含该文件。

### 3.9

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/05_function_document_fields/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-30 08:02

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SubDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs` | **PASS** | `SubDocument`, `createPLCOfDocumentFiels`, `getDocumentFileds`, `getType`, `getMainDocument`, `getTextDocument`, `getPieceTable`, `getDrawingContainer`, `getTextRope`, `getChpxTable`, `getPapxTable`, `getName`, `charAt`, `getLength`, `getStyles`, `getTextboxText`, `getPLCField`, `getPLCSection`, `getPLCHeader`, `getFootnoteRef`, `getFootnoteTxt`, `getEndnoteRef`, `getEndnoteTxt`, `writeLock`, `getMax`, `getLastParaStart`, `getOMathStarts`, `getOMathEnds`, `getPLCMathFunction`, `createPLCMath`, `createPLCOMath`, `isContainMathEquations`, `isLocked`, `getErrorCollector` | `getContentThumb`, `dispose`, `createPLCOfBookmark`, `addTextListener`, `removeTextListener`, `getCommentStarts`, `getCommentEnds`, `getCommentRef`, `getBookmarkStarts`, `getBookmarkEnds`, `getTextboxDocument`, `copy`, `copyTextbox`, `insertTextboxNode`, `getTextStream`, `getRange`, `getTableRange`, `getParagraphRange`, `getAutoNumCount`, `getText`, `getDispatcher`, `getShapeAnchors`, `getCommentTxt`, `start`, `submit`, `commit`, `getTableInfos`, `fireDocumentUpdate`, `toString`, `type_to_name`, `checkValid`, `readLockAll`, `readLock`, `tryReadLock`, `tryWriteLock`, `readLockMainDocument`, `writeLockMainDocument`, `getHeadings`, `getPLCWords`, `getRangeLock`, `hashCode`, `getAutoNumField`, `addRevisionListener`, `removeRevisionListener`, `getDocumentCorrector`, `getAutoWordStat`, `exchangFootEndnoteTxt`, `exchangFootEndnoteRef`, `getShapes`, `hasHeadings`, `getDocParaIds` | — | **PASS** |
| `TextDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `TextDocument`, `getSubdocument`, `createSubDocument`, `correctReadonlyTrackchanges`, `setProperty`, `setInIOState`, `isInIOState`, `getAutoNumTable`, `getName`, `getPath`, `getMetadata`, `getMainDocument`, `getProperty`, `getStyles`, `getFontTable`, `getTheme`, `getAsiaTypography`, `getCharBuffer`, `getTextPool`, `getLength`, `getRange`, `getTrackChange`, `getUUSti`, `getUUNumberingId`, `getUULsid`, `hasDocField`, `getCompatibilityMode`, `type`, `getKParagraphsID` | `setStatAnalytics`, `getStatAnalytics`, `setDocumentCallback`, `getTmpRootDir`, `getFileManager`, `dispose`, `getShapeIDAlloc`, `getMediaLib`, `getOleManager`, `getCoreDataAcceptor`, `newDocumentFromTemplate`, `newDocumentFromAutoSave`, `startLoading`, `onStartLoad`, `liteExchangeDocument`, `charCount`, `getGlobalInkUserID`, `setGloabalInkUserID`, `cryptInfo`, `isFinishDumpObjects`, `setIoError`, `getIoError`, `isInUserAction`, `isLegal`, `getRtfReader`, `openDocument`, `isEncryption`, `isProEncryption`, `insertBefore`, `insertAfter`, `isLoadSuccess`, `setLoadSuccess`, `getListTemplates`, `getLists`, `getListParagraphs`, `getRaf`, `save`, `saveAs`, `saveFile`, `setSaveSuccessedListener`, `onSaveSuccessed`, `onEvidenceSuccessd`, `exportSecurityFile`, `backUp`, `qingSaveBackup`, `getFileFormat`, `saveToSrcFileFlag`, `close`, `setOnModifiedListener`, `setOnModifiedListener4Stat`, `addWriterOpenEventListener`, `removeWriterOpenEventListener`, `isModified`, `isAutoSave`, `setModified`, `getEvidenceData`, `setEvidenceData`, `getAppVersion`, `setAppVersion`, `updateDocument`, `getContentThumb`, `addBuiltinStyles`, `getRsids`, `addTransactionListener`, `removeTransactionListener`, `start`, `submit`, `commit`, `update`, `startInner`, `getInterruptNext`, `setInterruptNext`, `rollback`, `clearRedo`, `canExecuteUndoOrRedo`, `undo`, `dumpForException`, `redo`, `canUndo`, `canRedo`, `clearUndoRedo`, `getViews`, `getPassword`, `hasReadPassword`, `setPasswd`, `getEncryptType`, `getDigest`, `getSections`, `isEmbedBalloonMode`, `setIsEmbedBalloonMode`, `setTrackChange`, `hasComments`, `hasRevisions`, `getRevisionsAuthor`, `setRevisionsAuthor`, `getUnknownDataMap`, `getVBAPath`, `setVBAPath`, `getUUCommentId`, `getUUPropertySetRegistryId`, `addBuiltinTableStyle`, `getSSClassLoader`, `getDrawingRIDMap`, `getShapeId2DrawingRID`, `getDiagramShapeId`, `isDisplayBackground`, `requestUpdateBkShape`, `getImmutableBkShape`, `getBackground`, `setBackground`, `setStopUpdateTextDocment`, `isStopUpdateTextDocment`, `onFinishDumpObjects`, `waitFinishDumpObjects`, `setIoThreadIdToRangeLock`, `setChartShapes`, `getChartShapes`, `beforeSectionChanged`, `afterSectionChanged`, `getEncoding`, `setEncoding`, `setLoadListener`, `isContainMathEquations`, `hasMainDocumentShape`, `setFileTime`, `recoverFileTime`, `updateCore4Save`, `getDocumentVariables`, `getDocumentProtection`, `getOnlineSecurityTool`, `getFirstPageFontList`, `hasAudioComment`, `isDirty`, `isSecurityFile`, `setSlimDirty`, `open`, `isEmptySlides`, `setWritePassword`, `hasWritePassword`, `getDocProtection`, `getBookProtection`, `getShareObjIndexdMap`, `getFootnoteEndnoteExchange`, `getFootnoteEndnote`, `getSectionProperty`, `getSTHostApp`, `addOle`, `getKCustomIcv`, `isProtectOn_readOnly`, `isProtectOn_TrackChangeCtrl`, `isProtectOn`, `isThrowDocumentProtectionException`, `beginSectionChange`, `endSectionChange`, `createDefaultPageSetup`, `setPageSetup`, `getPageSetup`, `setPageOrientation`, `exportAsArray`, `advanceGVmlOffset`, `setGridResizeRequest`, `getGridResizeRequest`, `getSaveAccelerator`, `setDebugger`, `isDebugging`, `onDebug`, `collectOpenedDocumentInfo`, `hasHeadings`, `initPaperRecognition`, `isPaper`, `setScrolling`, `isOpenIdleScroll`, `getParaText`, `getSubDocParaIds` | — | **PASS** |
| `PapxTableListener` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | — | `PapxTableListener`, `beforeAddEntry`, `afterAddEntry`, `beforeRemoveEntry`, `afterRemoveEntry`, `beforeDataChange`, `afterDataChange` | — | **PASS** |
| `ChpxTableListener` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | — | `ChpxTableListener`, `beforeAddEntry`, `afterAddEntry`, `beforeRemoveEntry`, `afterRemoveEntry`, `beforeDataChange`, `afterDataChange` | — | **PASS** |
| `UUNumberingId` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `getNextId`, `getCurrentId` | — | — | **PASS** |
| `UULsid` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `getNextId`, `getCurrentId` | — | — | **PASS** |

**overall: PASS** — basename aligned; every Java class has layout PASS and a Rust counterpart for each public method.
<!-- java-rust-align:end -->

### 3.10

| 步 | Java | Rust | 产品 |
|----|------|------|------|
| 1 | createPLC → createPLCOfDocumentFiels | 同 | ctor |
| 2 | getDocumentFileds 字段 | Option 同一实例 | hasDocField |

### 3.11

| 项 | 内容 |
|----|------|
| 阶段 | parse |
| 原始文档 | hello-world.docx `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` |
| 步骤对照 | §3.10 |
| 产品接线 | `TextDocument.hasDocField` |
| 关键路径真实实现 | pass（真实 `PLCDocumentFields` 实例，非 `default()`） |
| 管线测试 test_id | DF6 |
| 测试结果符合预期 | pass |
| 判定 | pass |
