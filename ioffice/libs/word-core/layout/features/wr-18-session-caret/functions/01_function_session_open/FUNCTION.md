# 01_function_session_open — 堆上打开会话

| 项 | 值 |
|----|----|
| feature | wr-18-session-caret |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `DocumentSession::open(&[u8]) -> Result<Self, LayoutError>` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
把 `.docx` 解析进 **堆上** `Box<TextDocument>`，会话生命周期内不得把文档值移回栈（SAX `SubDocument.mParent`）。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `open` | OPC zip 字节 | 无 `DocumentSession` 类；产品 `open_text_document` / `DocumentOpen` |

### 1.3 输出
`DocumentSession`；`main_length() > 0`（Hello fixture）。失败：`LayoutErrorKind::Zip`。

### 1.4 上游
wc-05 SAX / `DocumentOpen`（已实现）。本 function **不改** importer。

### 1.5 下游
02 append；04 layout。

### 1.6 不变量与函数改写列表
1. 合法 zip 且含 `word/document.xml` → `Ok`。
2. 非 PK / 截断 zip / 空缓冲 → `Err(Zip)`。
3. 两次 `open` 得到不同 `TextDocument` 地址。
4. **改写列表（本文件）：** `DocumentSession::open`、`document`、`main_length`。`append_paragraph` / `layout` 不在本 function。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `open_text_document` | LayoutDocument | bytes → `Box<TextDocument>` | 是 | RecXwpf 测试路径 | 禁止改 DocxReader |
| `DocumentOpen` | api | SAX fill | 是 | CoreThread | 禁止从零重写 |
| `TextDocument` | wc-02 | getMainDocument/getLength | 是 | 全 API | 禁止为 session 削模板 |
| `LayoutError` | LayoutDocument | Zip kind | 是 | Font/Measure 本 function 不测 | — |

### 1.8 函数改写列表
- `DocumentSession::open`
- `DocumentSession::document` / `document_mut`（只读暴露，不测 mutate）
- `DocumentSession::main_length`

## 2. 核心代码

Rust：`libs/word-core/src/cn/wps/moffice/writer/api/DocumentSession.rs`  
打开实现：`LayoutDocument::open_text_document` → `DocumentOpen`。  
Java：无对等类。结构对齐用 `TextDocument.java`（会话持有的模型）。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| open | DocumentSession.rs | Hello zip | Ok + length>0 | 主路径 |
| open | 两次 | 两份字节 | 两堆地址 | 独立 |
| open | 非 PK | `not-a-docx` | Zip | 非法 |
| open | `PK\x03\x04` | 截断 | Zip | 截断 |
| open | `[]` | 空 | Zip | 空 |
| main_length | 无 main | — | 0 | 本 fixture 达不到；不测 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| SO-OK | open | 合法 Hello | 是 | SO1 |
| SO-INDEP | open | 两次独立 | 是 | SO2 |
| SO-MAGIC | open | 非 zip | 是 | SO3 |
| SO-TRUNC | open | 截断 PK | 是 | SO4 |
| SO-EMPTY | open | 空字节 | 是 | SO5 |

### 3.3 可测试性
只用 `plain-one-para.docx` 与内存坏字节；不调 `layout()`、不调 append。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| SO1 | SO-OK | fixture | hello docx | length>0 | 打开失败钉 SAX/zip |
| SO2 | SO-INDEP | fixture | 两次 open | 等长且指针不等 | 共享 Box |
| SO3 | SO-MAGIC | — | ASCII | Zip | 被当成 xml |
| SO4 | SO-TRUNC | — | PK 头 | Zip | panic |
| SO5 | SO-EMPTY | — | `[]` | Zip | panic |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/api/DocumentSession.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/api/DocumentSession.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_session_open

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_session_open \
  --src src/cn/wps/moffice/writer/api/DocumentSession.rs \
  --report libs/word-core/layout/features/wr-18-session-caret/functions/01_function_session_open/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_session_open --json`)
date: 2026-08-27 10:16

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/DocumentSession.rs` | 13/36 | 36.1% | 16/50 | 32.0% |
| **function total** | **13/36** | **36.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
`DocumentSession.rs` 无 Java 交错模板。Loop 校验会话持有的模型：

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java
```

本 function **禁止**为过校验从零重写 `TextDocument.rs`。本轮 `check-template-layout`：**PASS**。

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs \
  --report libs/word-core/layout/features/wr-18-session-caret/functions/01_function_session_open/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-27 02:18

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TextDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `TextDocument`, `_create`, `dispose`, `getSubdocument`, `createSubDocument`, `createAllSubDocument`, `setInIOState`, `isInIOState`, `onSaveSuccessed`, `getMainDocument`, `getProperty`, `getStyles`, `getCharBuffer`, `getTextPool`, `getUUSti`, `getUUNumberingId`, `type`, `getKParagraphsID` | `setStatAnalytics`, `getStatAnalytics`, `setDocumentCallback`, `getTmpRootDir`, `getFileManager`, `clearCachedValues`, `getShapeIDAlloc`, `getMediaLib`, `getOleManager`, `getCoreDataAcceptor`, `newDocumentFromTemplate`, `newDocumentFromAutoSave`, `newDocument`, `setDocumentProperties`, `generateName`, `openDocument`, `init`, `startLoading`, `endLoading`, `correctDocumentData`, `correctParaIds`, `correctDataSections`, `correctDataFields`, `correctOleFields`, `correctOleData`, `correctReadonlyTrackchanges`, `appendHeaderFooterNodeSections`, `finishIO`, `correctHeaderFooter`, `deleteRange`, `clearActionCorrectingDataSections`, `isLegalRange`, `getPLCSection`, `getFirstSectionNode`, `analyseNumberParagraphAppended`, `isParagraph`, `correctStructureError`, `deleteIllegalSection`, `fixAllError`, `clearError`, `onStartLoad`, `onEndLoad`, `setProperty`, `liteExchangeDocument`, `charCount`, `getGlobalInkUserID`, `setGloabalInkUserID`, `cryptInfo`, `isFinishDumpObjects`, `setIoError`, `getIoError`, `isInUserAction`, `startMonitor`, `stopMonitor`, `isLegal`, `loadDocument`, `doLoadDocument`, `loadHtmlDocument`, `loadMhtDocument`, `loadDocDocument`, `endDocReader`, `toFileFormat`, `loadRtfDocument`, `getRtfReader`, `loadDocxDocument`, `endDocxReader`, `loadXml03Document`, `loadXml07Document`, `loadEpubDocument`, `loadUot3Document`, `loadTxtDocument`, `endEpubReader`, `endTextReader`, `insert`, `isEncryption`, `isProEncryption`, `insertBefore`, `insertAfter`, `isLoadSuccess`, `setLoadSuccess`, `getListTemplates`, `getLists`, `getListParagraphs`, `getAutoNumTable`, `getName`, `getPath`, `getRaf`, `save`, `saveAs`, `saveFile`, `setSaveSuccessedListener`, `onEvidenceSuccessd`, `exportSecurityFile`, `saveFile4Normal`, `saveFile4Encryption`, `encryptFileByAgentClient`, `encryptFileByOnlineSecurity`, `getWriter`, `updateShape`, `backUp`, `qingSaveBackup`, `getFileFormat`, `saveToSrcFileFlag`, `close`, `updateModifys`, `getLeafCommands`, `updateActions_undo`, `updateActions_redo`, `fixCommand`, `setOnModifiedListener`, `setOnModifiedListener4Stat`, `addWriterOpenEventListener`, `removeWriterOpenEventListener`, `isModified`, `isAutoSave`, `setModified`, `doModified`, `notify_modified`, `submitModified`, `getEvidenceData`, `setEvidenceData`, `getMetadata`, `getAppVersion`, `setAppVersion`, `updateDocument`, `getContentThumb`, `getFontTable`, `getTheme`, `addBuiltinStyles`, `getRsids`, `_saveSelection`, `_restoreSelection`, `waitSync`, `addTransactionListener`, `removeTransactionListener`, `start`, `submit`, `commit`, `update`, `transaction_lock`, `transaction_unlock`, `startInner`, `getInterruptNext`, `setInterruptNext`, `rollback`, `clearRedo`, `canExecuteUndoOrRedo`, `undo`, `dumpForException`, `redo`, `canUndo`, `canRedo`, `clearUndoRedo`, `getViews`, `getAsiaTypography`, `getPassword`, `hasReadPassword`, `setPasswd`, `initPasswd`, `getEncryptType`, `getLength`, `getRange`, `getDigest`, `getSections`, `isEmbedBalloonMode`, `setIsEmbedBalloonMode`, `setTrackChange`, `getTrackChange`, `hasComments`, `hasRevisions`, `getRevisionsAuthor`, `setRevisionsAuthor`, `getUnknownDataMap`, `getVBAPath`, `setVBAPath`, `getUUCommentId`, `getUUPropertySetRegistryId`, `addBuiltinTableStyle`, `getUULsid`, `getSSClassLoader`, `getDrawingRIDMap`, `getShapeId2DrawingRID`, `getDiagramShapeId`, `isDisplayBackground`, `requestUpdateBkShape`, `getImmutableBkShape`, `getBackground`, `getShapeProprytyID`, `addShapeCommand`, `setBackground`, `setStopUpdateTextDocment`, `isStopUpdateTextDocment`, `onFinishDumpObjects`, `waitFinishDumpObjects`, `setIoThreadIdToRangeLock`, `setChartShapes`, `getChartShapes`, `beforeSectionChanged`, `afterSectionChanged`, `getEncoding`, `setEncoding`, `setLoadListener`, `isContainMathEquations`, `hasMainDocumentShape`, `hasDocField`, `setFileTime`, `setFileTimeSaved`, `setFileTimeSavedAs`, `backupFileTime`, `recoverFileTime`, `updateCore4Save`, `getDocumentVariables`, `getDocumentProtection`, `getOnlineSecurityTool`, `fillFirstPageFont`, `getMainFirstPageEndCp`, `getFirstPageFontList`, `hasAudioComment`, `getCompatibilityMode`, `isDirty`, `isSecurityFile`, `setSlimDirty`, `open`, `isEmptySlides`, `setWritePassword`, `hasWritePassword`, `getDocProtection`, `getBookProtection`, `getFileExtFormat`, `doFormatConvert`, `getShareObjIndexdMap`, `getFootnoteEndnoteExchange`, `getFootnoteEndnote`, `getSectionProperty`, `getSTHostApp`, `addOle`, `getKCustomIcv`, `isProtectOn_readOnly`, `isProtectOn_TrackChangeCtrl`, `isProtectOn`, `isThrowDocumentProtectionException`, `beginSectionChange`, `endSectionChange`, `createInitialSepx`, `createDefaultPageSetup`, `getMergedSepx`, `orientation_equal`, `normalizeOrientation`, `changeSepx`, `setPageSetup`, `getPageSetup`, `setPageOrientation`, `exportAsArray`, `advanceGVmlOffset`, `setGridResizeRequest`, `getGridResizeRequest`, `getSaveAccelerator`, `setDebugger`, `isDebugging`, `onDebug`, `collectOpenedDocumentInfo`, `hasHeadings`, `initPaperRecognition`, `isPaper`, `setScrolling`, `isOpenIdleScroll`, `getParaText`, `getSubDocParaIds` | — | **PASS** |
| `PapxTableListener` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | — | `PapxTableListener`, `beforeAddEntry`, `afterAddEntry`, `beforeRemoveEntry`, `afterRemoveEntry`, `beforeDataChange`, `afterDataChange` | — | **PASS** |
| `ChpxTableListener` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | — | `ChpxTableListener`, `beforeAddEntry`, `afterAddEntry`, `beforeRemoveEntry`, `afterRemoveEntry`, `beforeDataChange`, `afterDataChange` | — | **PASS** |
| `UUNumberingId` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `UUNumberingId`, `getNextId`, `getCurrentId` | — | — | **PASS** |
| `UULsid` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `UULsid`, `getNextId`, `getCurrentId` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
