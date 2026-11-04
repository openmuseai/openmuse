# 01_function_parent_alias — SubDocument.mParent → 真实 TextDocument

| 项 | 值 |
|----|----|
| feature | wc-10-subdocument-pipeline |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SubDocument::{SubDocument(parent,type), getTextDocument, getMainDocument, getStyles}` / `TextDocument::{createAllSubDocument, getTextDocument}` / `impl SoftTextDocument for core::TextDocument` |
| 独立性 | 叶子（消费 wc-02 09/10 已建成的 7 槽；不重写 rope/PLC 语义） |
| 测试状态 | pass（PA1–PA10；§3.8 SubDocument+TextDocument PASS；§3.9 overall PASS） |
| 阶段 | parse + layout 读门面 |

**本 function 按 I/O 切，不按类切。** 不要全叶改写 `SubDocument.java` / `TextDocument.java`。不要把 `KStyles` 的 parent 类型、事务 `start`/`commit` 引擎、`data/table` 当本 function 的改写范围。

## 1. 功能说明

### 1.1 一句话

`new SubDocument(this, type)` 的 `mParent` 是 **`core::TextDocument`**（Java `cn.wps.moffice.writer.core.TextDocument`），`getTextDocument()` 返回该引用，`getMainDocument()` 委托 `mParent.getMainDocument()`（主故事槽 0），layout 经 `&dyn Document` 拿到同一顶层文档。

### 1.2 输入

`SubDocument(TextDocument parent, int nType)`。`parent` 必须是已经跑过 `_create`（或至少有 tx/pool/styles）的 **真实** `TextDocument`，不是 `KStyle.rs` 里的 stand-in。

产品入口：`TextDocument()` → `_create` → `createAllSubDocument`：`new SubDocument(this, i)`（`this` = 同一 `TextDocument`）。

### 1.3 输出

- `sd.getTextDocument()` 非空，与拥有该 `sd` 的 `TextDocument` 同一指针（Java `return this.mParent`）
- `Document::getTextDocument()` 在 `SubDocument` 上返回 `Some(&dyn SoftTextDocument)`，且该对象就是 `core::TextDocument`
- `Document::getTextDocument()` 在 `TextDocument` 上返回 `Some(self)`（Java `return this`），不得再 `None`
- type≠0 的故事：`getMainDocument()` 是槽 0，**不是** `self`（Java `return this.mParent.getMainDocument()`）
- `getStyles()` 与 `parent.getStyles()` 同一 `KStyles` 指针
- `createAllSubDocument` / `createSubDocument` 不再经 `subdoc_parent: Box<KStyle::TextDocument>` 间接构造

不在本 function 测：rope/CHPX/PAPX 装配（09）、七槽循环本身（10）、`TextDocument.start/commit` 事务引擎、`KStyles` 仍 clone 轻量 parent。

### 1.4 上游

wc-02 `09_function_subdocument_own`、`10_function_textdocument_create`。

### 1.5 下游

layout：`RangeTool` / `HelperHeaderFooter` / `FontStyleCacheFactory` / `GridSizeLayouter` / `DrawingLayouter` 经 `document.getTextDocument()`。

### 1.6 硬约束

| 允许 | 禁止 |
|------|------|
| 改 `SubDocument.mParent` 类型为 `*mut core::TextDocument`；`createAllSubDocument` 传 `self` | 发明 `SoftParent` / 第二套 `TextDocument` 生产名 |
| `impl SoftTextDocument for core::TextDocument` 接线 getter | 删/改 `// java`；瘦 `SubDocument.rs` |
| 09/10 测试改为产品构造 `TextDocument::new()` + `getSubdocument` | 为编译保留 `KStyle::TextDocument` 当 `SubDocument` ctor 形参 |
| `KStyles::KStyles` 仍吃 stand-in（05 债） | 本 function 全叶改写 `KStyles` |
| `TextDocument.start/commit` 保持 DEFERRED（事务引擎） | 把 SoftTextDocument.start 接到会 `todo!` 的 inherent 上（会炸 HelperHeaderFooter） |

**函数改写列表（本文件 / 本轮）**

- `SubDocument.rs`：ctor 形参类型；`getMainDocument`；`getTextDocument`；`getStyles`；`Document::{getMainDocument,getTextDocument,getStyles}`
- `TextDocument.rs`：`_create`/`createAllSubDocument`/`createSubDocument` 去掉 `subdoc_parent` glue；`Document::getTextDocument` 返回 `self`；`impl SoftTextDocument for TextDocument`（transaction / main / path / name / metadata / property / isInIOState / getSubdocument）
- 不在列表：`SubDocument.start/commit/submit` 体（Java 委托 `mParent.start()`；inherent `TextDocument.start` 仍 DEFERRED，见 §1.7.1）；其余 PLC getter

### 1.7 依赖与实现状态（着重）

**本 function 入口（leaf）**

| 类型 | 01 需要的接口 | 实现情况 |
|------|----------------|----------|
| `SubDocument` | ctor `(parent: &core::TextDocument, nType)`；`getTextDocument`→parent；`getMainDocument`→`mParent.getMainDocument()`；`getStyles` 经 parent | 模板保留。上列 **必须真写**。 |
| `TextDocument` | `createAllSubDocument`：`new SubDocument(this, i)`；`Document::getTextDocument`→`this`；`impl SoftTextDocument` | 模板保留。`subdoc_parent` glue **删除**（Java 无此字段）。 |
| `Document` trait | `getTextDocument` / `getMainDocument` / `getStyles` | 签名已有；SubDocument/TextDocument impl 必须对齐 Java。 |
| `SoftTextDocument` | 供 layout 在 `getTextDocument()` 之后立刻调用的成员 | 默认空值保留给未接线方法；本轮接线见上表。 |

**已实现、01 只调用**

| 依赖 | 归属 | 需要 | 禁止 01 去做 |
|------|------|------|----------------|
| 7 槽 `_create` | wc-02 **10** | `TextDocument::new()` 得到非空槽 | 重测 TD1–TD9 语义（回归即可） |
| rope/CHPX/PAPX | wc-02 **09** | ctor 仍装配 | 重写表语义 |
| `KStyles` | wc-02 **05** | `getStyles()` 非空指针 | 改 `KStyles.mTextDocument` 类型 |
| `KStyle.rs` stand-in `TextDocument` | 编译胶 | **仅** `KStyles::KStyles(&stand-in)` | 再当 `SubDocument` parent |

### 1.7.1 Java 有、本 function 不实现（编号）

| # | Java | Rust 现状 | 替代 | owner |
|---|------|-----------|------|-------|
| 1 | `TextDocument.start()` → `start(true)` → 事务锁 + `Transaction.start` | inherent `todo!("DEFERRED_NON_FUNCTION")` | `SoftTextDocument::start` **保持 trait 默认空实现**（与换 parent 前 stand-in 行为相同，避免 HelperHeaderFooter 踩 `todo!`） | 后续事务 function |
| 2 | `TextDocument.commit(String)` 链 | 同上 | `SoftTextDocument::commit` 默认空 | 同上 |
| 3 | `SubDocument.start/commit/submit` 体 `this.mParent.start()` | 仍 `todo!("DEFERRED_NON_FUNCTION")` + 原 `// java` 语句 | 01 不测调用 start；注释保留 | 同上 |
| 4 | `KStyles(TextDocument this)` 持有真实文档 | `KStyles` 仍 clone `KStyle::TextDocument` stand-in | `_create` 继续为 KStyles 填 stand-in 的 pool/styles/tx 指针 | wc-02 05 债 |
| 5 | `getMetadata()` 可空引用 | 本轮 un-park 为返回 `mMetadata` 字段（Java `return this.mMetadata`）；null→`Default` | — | 本 function |

## 2. 核心代码与流程

```mermaid
flowchart LR
  TD["_create TextDocument"] -->|"new SubDocument(this, i)"| SD[SubDocument.mParent]
  SD -->|"getTextDocument()"| TD
  SD -->|"getMainDocument()"| MAIN["mSubdocuments[0]"]
  LAY["&dyn Document layout"] -->|"getTextDocument()"| STD[SoftTextDocument]
  STD --> TD
```

Java：`SubDocument.java:121-122,461-466,730-731`；`TextDocument.java:351+ createAllSubDocument`、`getMainDocument`、`getTextDocument` 在 Document 上 `return this`。

## 3. 测试方案

### 3.1 代码审计与测试面

Java：`koffice/writer/core/.../SubDocument.java` ctor / `getMainDocument` / `getTextDocument` / `getStyles`  
Java：`.../TextDocument.java` `_create` / `createAllSubDocument` / `getMainDocument`  
Rust：`libs/word-core/src/cn/wps/moffice/writer/core/{SubDocument,TextDocument,Document}.rs`  
产品调用：`RangeTool` `document.getTextDocument()`；`FontStyleCacheFactory`；`HelperHeaderFooter`；`open_text_document`

| symbol | 输入/状态 | 输出/副作用 | 关键分支 |
|--------|-----------|-------------|----------|
| `SubDocument(parent, nType)` | 真实 `&TextDocument` | `mParent` 存该指针 | 假 stand-in 必须编不过 |
| `getTextDocument` | 已装配 sd | 返回 `mParent` | Document trait Some |
| `TextDocument.getTextDocument` | `_create` 后 | `Some(self)` | 不得 None |
| `getMainDocument` type0 | 槽内 main | 与 `parent.getMainDocument()` 同一指针 | 不得 `Some(self)` 短路 |
| `getMainDocument` type2 | 页眉故事 | 槽 0，不是 self | |
| `getStyles` | `_create` 后 | 与 `td.getStyles()` 同址 | |
| 管线 | 同一 `.docx` parse | 主故事 `getTextDocument` 可达 | 产品 `open_text_document` |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|-------------|---------------|----------|-------|---------|
| C-CTOR-PTR | ctor `mParent` == parent | 主路径 | 是 | PA1 |
| C-GET-TD | 固有 `getTextDocument` 指针 | 主路径 | 是 | PA1 |
| C-TRAIT-TD | `Document::getTextDocument` Some | 主路径 | 是 | PA2 |
| C-TD-SELF | `TextDocument` 上 `getTextDocument` is self | 主路径 | 是 | PA3 |
| C-MAIN-SLOT | type0 `getMainDocument` 即槽 0 | 主路径 | 是 | PA4 |
| C-HDR-MAIN | type2 `getMainDocument` 是槽 0 不是 self | 分支 | 是 | PA5 |
| C-STYLES | `getStyles` 同 `td.getStyles` | 主路径 | 是 | PA6 |
| C-CREATE-THIS | `createAllSubDocument` 七槽 parent 同一 | 主路径 | 是 | PA7 |
| C-CREATE-SUB | `createSubDocument(0)` 不换 parent | 状态 | 是 | PA8 |
| C-MOCK | `TextDocument(true)` 无槽，不误造 parent | 边界 | 是 | PA9 |
| C-PIPE | 同一 docx → parse → 主故事 getTextDocument | 管线 | 是 | PA10 |

### 3.3 可测试性 / 管线测试

单元：`TextDocument::new()`（产品 `_create`）。  
管线：`open_text_document` 读 `tests/fixtures/p1_layout_document/hello-world.docx`（sha256 写入 PA10 运行时打印或本节冻结）。阶段 **parse**。步骤：`TextDocument._create` → `createAllSubDocument` → `SubDocument.getTextDocument`。禁止另造 Document。

### 3.4 用例表

| id | 覆盖项 | 前置 | 输入摘要 | 期望 | 失败语义 |
|----|--------|------|----------|------|----------|
| PA1 | C-CTOR-PTR C-GET-TD | `TextDocument::new()` | 取槽 0 | `getTextDocument` 与 `td` 同址 | 仍指向 KStyle 壳 |
| PA2 | C-TRAIT-TD | PA1 | `Document::getTextDocument` | Some | None |
| PA3 | C-TD-SELF | `_create` | `Document::getTextDocument` on td | Some，与 self 同址 | 仍 None |
| PA4 | C-MAIN-SLOT | PA1 | `getMainDocument` | 与 `td.getMainDocument()` 同址 | 返回 self 短路 |
| PA5 | C-HDR-MAIN | 槽 2 | type2 `getMainDocument` | 槽 0 指针；≠ 槽 2 | 页眉当 main |
| PA6 | C-STYLES | `_create` | `sd.getStyles()` | 与 `td.getStyles()` 同址 | 空或另一表 |
| PA7 | C-CREATE-THIS | 七槽 | 槽 0 与槽 6 `getTextDocument` | 同一 parent | 各挂各的壳 |
| PA8 | C-CREATE-SUB | 已有槽 0 | `createSubDocument(0)` | parent 指针不变 | 重建换 parent |
| PA9 | C-MOCK | `TextDocument(true)` | `getMainDocument` | None | mock 误 `_create` |
| PA10 | C-PIPE | hello-world.docx | `open_text_document` | 主故事 `getTextDocument` Some；`getMainDocument` 为槽 0 | 产品链仍 None |

### 3.5 测试代码位置

crate: `libs/word-core`  
src:   `libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs`  
src:   `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs`  
src:   `libs/word-core/src/cn/wps/moffice/writer/core/Document.rs`（只读 trait，本轮若改签名须 layout）  
test:  `libs/word-core/tests/cn/wps/moffice/writer/core/01_function_parent_alias.rs`  
（09 已占用 `tests/.../SubDocument.rs` 镜像；本 function 是 I/O 切，入口 harness 单独文件。）

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_parent_alias

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_parent_alias \
  --src src/cn/wps/moffice/writer/core/SubDocument.rs \
  --src src/cn/wps/moffice/writer/core/TextDocument.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/01_function_parent_alias/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_parent_alias --json`)
date: 2026-09-30 11:52

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/core/SubDocument.rs` | 247/585 | 42.2% | 266/665 | 40.0% |
| `src/cn/wps/moffice/writer/core/TextDocument.rs` | 229/1175 | 19.5% | 301/1396 | 21.6% |
| **function total** | **476/1760** | **27.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java
```

### 3.9 Java↔Rust 门面对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/01_function_parent_alias/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-30 06:56

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SubDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs` | **PASS** | `SubDocument`, `getDocumentFileds`, `getType`, `getMainDocument`, `getTextDocument`, `getPieceTable`, `getDrawingContainer`, `getTextRope`, `getChpxTable`, `getPapxTable`, `getName`, `charAt`, `getLength`, `getStyles`, `getTextboxText`, `getPLCField`, `getPLCSection`, `getPLCHeader`, `getFootnoteRef`, `getFootnoteTxt`, `getEndnoteRef`, `getEndnoteTxt`, `writeLock`, `getMax`, `getLastParaStart`, `isLocked`, `getErrorCollector` | `getContentThumb`, `dispose`, `createPLCOfBookmark`, `createPLCOfDocumentFiels`, `addTextListener`, `removeTextListener`, `getCommentStarts`, `getCommentEnds`, `getCommentRef`, `getBookmarkStarts`, `getBookmarkEnds`, `getTextboxDocument`, `copy`, `copyTextbox`, `insertTextboxNode`, `getTextStream`, `getRange`, `getTableRange`, `getParagraphRange`, `getAutoNumCount`, `getText`, `getDispatcher`, `getShapeAnchors`, `getCommentTxt`, `start`, `submit`, `commit`, `getTableInfos`, `fireDocumentUpdate`, `toString`, `type_to_name`, `checkValid`, `readLockAll`, `readLock`, `tryReadLock`, `tryWriteLock`, `readLockMainDocument`, `writeLockMainDocument`, `getHeadings`, `getPLCWords`, `getRangeLock`, `getOMathStarts`, `getOMathEnds`, `getPLCMathFunction`, `createPLCMath`, `createPLCOMath`, `isContainMathEquations`, `hashCode`, `getAutoNumField`, `addRevisionListener`, `removeRevisionListener`, `getDocumentCorrector`, `getAutoWordStat`, `exchangFootEndnoteTxt`, `exchangFootEndnoteRef`, `getShapes`, `hasHeadings`, `getDocParaIds` | — | **PASS** |
| `TextDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `TextDocument`, `getSubdocument`, `createSubDocument`, `correctReadonlyTrackchanges`, `setProperty`, `setInIOState`, `isInIOState`, `getAutoNumTable`, `getName`, `getPath`, `getMetadata`, `getMainDocument`, `getProperty`, `getStyles`, `getFontTable`, `getTheme`, `getAsiaTypography`, `getCharBuffer`, `getTextPool`, `getLength`, `getRange`, `getTrackChange`, `getUUSti`, `getUUNumberingId`, `getUULsid`, `hasDocField`, `getCompatibilityMode`, `type`, `getKParagraphsID` | `setStatAnalytics`, `getStatAnalytics`, `setDocumentCallback`, `getTmpRootDir`, `getFileManager`, `dispose`, `getShapeIDAlloc`, `getMediaLib`, `getOleManager`, `getCoreDataAcceptor`, `newDocumentFromTemplate`, `newDocumentFromAutoSave`, `startLoading`, `onStartLoad`, `liteExchangeDocument`, `charCount`, `getGlobalInkUserID`, `setGloabalInkUserID`, `cryptInfo`, `isFinishDumpObjects`, `setIoError`, `getIoError`, `isInUserAction`, `isLegal`, `getRtfReader`, `openDocument`, `isEncryption`, `isProEncryption`, `insertBefore`, `insertAfter`, `isLoadSuccess`, `setLoadSuccess`, `getListTemplates`, `getLists`, `getListParagraphs`, `getRaf`, `save`, `saveAs`, `saveFile`, `setSaveSuccessedListener`, `onSaveSuccessed`, `onEvidenceSuccessd`, `exportSecurityFile`, `backUp`, `qingSaveBackup`, `getFileFormat`, `saveToSrcFileFlag`, `close`, `setOnModifiedListener`, `setOnModifiedListener4Stat`, `addWriterOpenEventListener`, `removeWriterOpenEventListener`, `isModified`, `isAutoSave`, `setModified`, `getEvidenceData`, `setEvidenceData`, `getAppVersion`, `setAppVersion`, `updateDocument`, `getContentThumb`, `addBuiltinStyles`, `getRsids`, `addTransactionListener`, `removeTransactionListener`, `start`, `submit`, `commit`, `update`, `startInner`, `getInterruptNext`, `setInterruptNext`, `rollback`, `clearRedo`, `canExecuteUndoOrRedo`, `undo`, `dumpForException`, `redo`, `canUndo`, `canRedo`, `clearUndoRedo`, `getViews`, `getPassword`, `hasReadPassword`, `setPasswd`, `getEncryptType`, `getDigest`, `getSections`, `isEmbedBalloonMode`, `setIsEmbedBalloonMode`, `setTrackChange`, `hasComments`, `hasRevisions`, `getRevisionsAuthor`, `setRevisionsAuthor`, `getUnknownDataMap`, `getVBAPath`, `setVBAPath`, `getUUCommentId`, `getUUPropertySetRegistryId`, `addBuiltinTableStyle`, `getSSClassLoader`, `getDrawingRIDMap`, `getShapeId2DrawingRID`, `getDiagramShapeId`, `isDisplayBackground`, `requestUpdateBkShape`, `getImmutableBkShape`, `getBackground`, `setBackground`, `setStopUpdateTextDocment`, `isStopUpdateTextDocment`, `onFinishDumpObjects`, `waitFinishDumpObjects`, `setIoThreadIdToRangeLock`, `setChartShapes`, `getChartShapes`, `beforeSectionChanged`, `afterSectionChanged`, `getEncoding`, `setEncoding`, `setLoadListener`, `isContainMathEquations`, `hasMainDocumentShape`, `setFileTime`, `recoverFileTime`, `updateCore4Save`, `getDocumentVariables`, `getDocumentProtection`, `getOnlineSecurityTool`, `getFirstPageFontList`, `hasAudioComment`, `isDirty`, `isSecurityFile`, `setSlimDirty`, `open`, `isEmptySlides`, `setWritePassword`, `hasWritePassword`, `getDocProtection`, `getBookProtection`, `getShareObjIndexdMap`, `getFootnoteEndnoteExchange`, `getFootnoteEndnote`, `getSectionProperty`, `getSTHostApp`, `addOle`, `getKCustomIcv`, `isProtectOn_readOnly`, `isProtectOn_TrackChangeCtrl`, `isProtectOn`, `isThrowDocumentProtectionException`, `beginSectionChange`, `endSectionChange`, `createDefaultPageSetup`, `setPageSetup`, `getPageSetup`, `setPageOrientation`, `exportAsArray`, `advanceGVmlOffset`, `setGridResizeRequest`, `getGridResizeRequest`, `getSaveAccelerator`, `setDebugger`, `isDebugging`, `onDebug`, `collectOpenedDocumentInfo`, `hasHeadings`, `initPaperRecognition`, `isPaper`, `setScrolling`, `isOpenIdleScroll`, `getParaText`, `getSubDocParaIds` | — | **PASS** |
| `PapxTableListener` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | — | `PapxTableListener`, `beforeAddEntry`, `afterAddEntry`, `beforeRemoveEntry`, `afterRemoveEntry`, `beforeDataChange`, `afterDataChange` | — | **PASS** |
| `ChpxTableListener` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | — | `ChpxTableListener`, `beforeAddEntry`, `afterAddEntry`, `beforeRemoveEntry`, `afterRemoveEntry`, `beforeDataChange`, `afterDataChange` | — | **PASS** |
| `UUNumberingId` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `getNextId`, `getCurrentId` | — | — | **PASS** |
| `UULsid` | `koffice/writer/core/cn/wps/moffice/writer/core/TextDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs` | **PASS** | `getNextId`, `getCurrentId` | — | — | **PASS** |

**overall: PASS** — basename aligned; every Java class has layout PASS and a Rust counterpart for each public method.
<!-- java-rust-align:end -->

### 3.10 Java↔Rust 管线对比（逻辑 + 产品接线）

| 步 | Java | Rust | 产品接线 |
|----|------|------|----------|
| 1 | `new TextDocument()` → `_create` | `TextDocument::new()` | `open_text_document` |
| 2 | `createAllSubDocument`：`new SubDocument(this, i)` | 同，`this` = `core::TextDocument` | `_create` |
| 3 | `doc.getTextDocument()` | `Document::getTextDocument` | layout `RangeTool` / `FontStyleCacheFactory` |
| 4 | `doc.getMainDocument()` | `mParent.getMainDocument()` | 页眉 `HelperHeaderFooter` |

### 3.11 管线阶段对齐报告

| 项 | 内容 |
|----|------|
| 阶段（parse / layout / render） | parse（getter 供 layout 读） |
| 原始文档（同一份，sha256） | `tests/fixtures/p1_layout_document/hello-world.docx` `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` |
| 步骤对照 | 见 §3.10 |
| 产品接线 | `open_text_document` → `TextDocument` → `SubDocument` |
| 关键路径真实实现 | pass（`mParent` 为 `core::TextDocument`；`Document::getTextDocument` 返回 `this`） |
| 管线测试 test_id | PA10 |
| 测试结果符合预期 | pass |
| 判定 | pass |
