# 10_function_autowordstat — new AutoWordStat + setListener

| 项 | 值 |
|----|----|
| feature | wc-10-subdocument-pipeline |
| order | 10 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | ctor `nType==0` 时 `new AutoWordStat(this)` + `mTextStream.setListener`；`getAutoWordStat` |
| 独立性 | 不实现字数统计回调 / `beforeRevisionModify` |
| 测试状态 | pass（AWS1–AWS6；§3.8 SubDocument+Document PASS；§3.9 overall PASS） |
| 阶段 | parse |

## 1. 功能说明

### 1.1 一句话

仅主文档（type 0）构造 `AutoWordStat` 并把其挂到 `ITextStream.setListener`；header 故事字段为 null。

### 1.2 输入

`new SubDocument(parent, type)`。

### 1.3 输出

- type 0：`getAutoWordStat()` Some；stream `getListener()` 非空
- type ≠ 0：getter None
- 固有与 trait 同址
- hello-world parse 后主文档 Some

### 1.6 硬约束

使用 `data/autoWordStat/AutoWordStat.rs`。禁止删除 `// java`。

**改写列表：** `AutoWordStat.rs` ctor（`mDoc`/`mCore`）；`KTextStream`/`ITextStream` `setListener`/`getListener`；`SubDocument.rs` type0 分支 + getter。

### 1.7.1

| # | Java | 替代 |
|---|------|------|
| 1 | `listeners.add(this.mAutoWordStat)` | rust: no-stmt |
| 2 | `beforeRevisionModify` 等统计回调 | DEFERRED；§3.8 不含 AutoWordStat.rs 全方法（同 05） |
| 3 | `this.mCore = this.mDoc.getTextDocument()` 字段 | 经 `mDoc` 指针 `core_is_some()` |

## 3. 测试方案

| id | 期望 |
|----|------|
| AWS1 | 主文档 Some |
| AWS2 | header None |
| AWS3 | trait 与固有同址 |
| AWS4 | 主文档 stream listener 非空 |
| AWS5 | core_is_some |
| AWS6 | hello-world parse 后 Some |

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 10_function_autowordstat

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 10_function_autowordstat \
  --src src/cn/wps/moffice/writer/core/SubDocument.rs \
  --src src/cn/wps/moffice/writer/core/Document.rs \
  --src src/cn/wps/moffice/writer/data/autoWordStat/AutoWordStat.rs \
  --src src/cn/wps/moffice/writer/data/KTextStream.rs \
  --src src/cn/wps/moffice/writer/data/ITextStream.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/10_function_autowordstat/FUNCTION.md
```

### 3.7

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 10_function_autowordstat --json`)
date: 2026-09-30 22:09

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/core/Document.rs` | 8/253 | 3.2% | 6/269 | 2.2% |
| `src/cn/wps/moffice/writer/core/SubDocument.rs` | 286/829 | 34.5% | 319/971 | 32.9% |
| `src/cn/wps/moffice/writer/data/ITextStream.rs` | 0/46 | 0.0% | 0/54 | 0.0% |
| `src/cn/wps/moffice/writer/data/KTextStream.rs` | 21/122 | 17.2% | 17/121 | 14.0% |
| `src/cn/wps/moffice/writer/data/autoWordStat/AutoWordStat.rs` | 18/24 | 75.0% | 11/17 | 64.7% |
| **function total** | **333/1274** | **26.1%** |  |  |
<!-- coverage-report:end -->

### 3.8

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/Document.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/Document.java
```

`AutoWordStat.rs` / `KTextStream.rs` / `ITextStream.rs` 统计回调与过滤器仍 DEFERRED。§3.8/§3.9 不含这三文件。

### 3.9

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/Document.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/Document.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/10_function_autowordstat/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-30 14:08

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SubDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs` | **PASS** | `SubDocument`, `createPLCOfDocumentFiels`, `getDocumentFileds`, `getType`, `getMainDocument`, `getTextDocument`, `getPieceTable`, `getDrawingContainer`, `getTextRope`, `getChpxTable`, `getPapxTable`, `getTextStream`, `getName`, `getAutoNumCount`, `charAt`, `getLength`, `getStyles`, `getShapeAnchors`, `getTextboxText`, `getPLCField`, `getPLCSection`, `getPLCHeader`, `getFootnoteRef`, `getFootnoteTxt`, `getEndnoteRef`, `getEndnoteTxt`, `getTableInfos`, `writeLock`, `getMax`, `getLastParaStart`, `getHeadings`, `getPLCWords`, `getOMathStarts`, `getOMathEnds`, `getPLCMathFunction`, `createPLCMath`, `createPLCOMath`, `isContainMathEquations`, `isLocked`, `getAutoNumField`, `addRevisionListener`, `removeRevisionListener`, `getErrorCollector`, `getAutoWordStat` | `getContentThumb`, `dispose`, `createPLCOfBookmark`, `addTextListener`, `removeTextListener`, `getCommentStarts`, `getCommentEnds`, `getCommentRef`, `getBookmarkStarts`, `getBookmarkEnds`, `getTextboxDocument`, `copy`, `copyTextbox`, `insertTextboxNode`, `getRange`, `getTableRange`, `getParagraphRange`, `getText`, `getDispatcher`, `getCommentTxt`, `start`, `submit`, `commit`, `fireDocumentUpdate`, `toString`, `type_to_name`, `checkValid`, `readLockAll`, `readLock`, `tryReadLock`, `tryWriteLock`, `readLockMainDocument`, `writeLockMainDocument`, `getRangeLock`, `hashCode`, `getDocumentCorrector`, `exchangFootEndnoteTxt`, `exchangFootEndnoteRef`, `getShapes`, `hasHeadings`, `getDocParaIds` | — | **PASS** |
| `Document` | `koffice/writer/core/cn/wps/moffice/writer/core/Document.java` | `libs/word-core/src/cn/wps/moffice/writer/core/Document.rs` | **PASS** | — | — | — | **PASS** |

**overall: PASS** — basename aligned; every Java class has layout PASS and a Rust counterpart for each public method.
<!-- java-rust-align:end -->

### 3.10

| 步 | Java | Rust | 产品 |
|----|------|------|------|
| 1 | type==0 `new AutoWordStat(this)` | `AutoWordStat::AutoWordStat(document)` | ctor |
| 2 | `mTextStream.setListener(this.mAutoWordStat)` | `ITextStream::setListener` | 修订过滤挂钩 |
| 3 | getAutoWordStat | 字段 / trait | parse 后读 |

### 3.11

| 项 | 内容 |
|----|------|
| 阶段 | parse |
| 原始文档 | hello-world.docx `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` |
| 步骤对照 | §3.10 |
| 产品接线 | ctor type0 分支 + stream.setListener |
| 关键路径真实实现 | pass（真实 `AutoWordStat` + listener 指针） |
| 管线测试 test_id | AWS6 |
| 测试结果符合预期 | pass |
| 判定 | pass |
