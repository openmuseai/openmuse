# 09_function_table_infos — InfoFactory.createTableInfos

| 项 | 值 |
|----|----|
| feature | wc-10-subdocument-pipeline |
| order | 9 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | ctor `InfoFactory.createTableInfos(this)`；`SubDocument.getTableInfos`；`Document` 同名门面 |
| 独立性 | 不实现 `TreeTableCache` / `getTableInfoByLevel` 全表缓存 |
| 测试状态 | pass（TI1–TI6；§3.8 SubDocument+Document+InfoFactory PASS；§3.9 overall PASS） |
| 阶段 | layout |

## 1. 功能说明

### 1.1 一句话

Java ctor `this.mTableInfos = InfoFactory.createTableInfos(this)` 即 `new TableInfosImpl(document)`；getter 返回同一接口引用。Rust 去掉 Copy 影子。

### 1.2 输入

`new SubDocument(parent, type)`（所有故事）。

### 1.3 输出

- ctor 后 `getTableInfos()` 非空
- `size()` 为 0（TreeTableCache off-surface）
- 固有方法与 `Document` trait 同址
- header 故事同样挂上
- hello-world parse 后 trait getter 非空

### 1.6 硬约束

禁止第二套 `SoftTableInfos`。使用 `data/table/TableInfos.rs` trait 与 `TableInfosImpl.rs`。禁止删除 `// java`。

**改写列表：** `InfoFactory.rs` `createTableInfos`；`TableInfosImpl.rs` ctor；`SubDocument.rs` 字段 + getter + `impl Document`；`Document.rs` `getTableInfos` 去 `*mut ()`。

### 1.7.1

| # | Java | 替代 |
|---|------|------|
| 1 | `this.mTableCache = new TreeTableCache(document)` | rust: no-stmt；`size()` 返回 0 |
| 2 | `listeners.add(this.mTableInfos.getTextListener())` | getTextListener 默认 None |
| 3 | hasTable / getTableInfoByLevel 等 | 仍 DEFERRED；§3.8 不含 TableInfos.rs / TableInfosImpl.rs 全接口（同 05 对 PLCDocumentFields） |

## 3. 测试方案

| id | 期望 |
|----|------|
| TI1 | ctor 后 getter Some |
| TI2 | size == 0 |
| TI3 | trait 与固有同址 |
| TI4 | header 故事 Some |
| TI5 | 两次 getter 同指针 |
| TI6 | hello-world parse 后 trait getter 非空 |

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 09_function_table_infos

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 09_function_table_infos \
  --src src/cn/wps/moffice/writer/core/SubDocument.rs \
  --src src/cn/wps/moffice/writer/core/Document.rs \
  --src src/cn/wps/moffice/writer/data/table/InfoFactory.rs \
  --src src/cn/wps/moffice/writer/data/table/TableInfos.rs \
  --src src/cn/wps/moffice/writer/data/table/TableInfosImpl.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/09_function_table_infos/FUNCTION.md
```

### 3.7

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 09_function_table_infos --json`)
date: 2026-09-30 22:09

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/core/Document.rs` | 8/253 | 3.2% | 6/269 | 2.2% |
| `src/cn/wps/moffice/writer/core/SubDocument.rs` | 284/829 | 34.3% | 317/971 | 32.6% |
| `src/cn/wps/moffice/writer/data/table/InfoFactory.rs` | 3/3 | 100.0% | 4/4 | 100.0% |
| `src/cn/wps/moffice/writer/data/table/TableInfos.rs` | 0/6 | 0.0% | 0/6 | 0.0% |
| `src/cn/wps/moffice/writer/data/table/TableInfosImpl.rs` | 15/18 | 83.3% | 7/11 | 63.6% |
| **function total** | **310/1109** | **28.0%** |  |  |
<!-- coverage-report:end -->

### 3.8

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/Document.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/Document.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/data/table/InfoFactory.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/data/table/InfoFactory.java
```

`TableInfos.rs` / `TableInfosImpl.rs` 仍为 ctor/`size` 编译面；全接口方法 DEFERRED。§3.8/§3.9 不含这两文件。

### 3.9

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/Document.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/Document.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/data/table/InfoFactory.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/data/table/InfoFactory.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/09_function_table_infos/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-30 14:08

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SubDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs` | **PASS** | `SubDocument`, `createPLCOfDocumentFiels`, `getDocumentFileds`, `getType`, `getMainDocument`, `getTextDocument`, `getPieceTable`, `getDrawingContainer`, `getTextRope`, `getChpxTable`, `getPapxTable`, `getTextStream`, `getName`, `getAutoNumCount`, `charAt`, `getLength`, `getStyles`, `getShapeAnchors`, `getTextboxText`, `getPLCField`, `getPLCSection`, `getPLCHeader`, `getFootnoteRef`, `getFootnoteTxt`, `getEndnoteRef`, `getEndnoteTxt`, `getTableInfos`, `writeLock`, `getMax`, `getLastParaStart`, `getHeadings`, `getPLCWords`, `getOMathStarts`, `getOMathEnds`, `getPLCMathFunction`, `createPLCMath`, `createPLCOMath`, `isContainMathEquations`, `isLocked`, `getAutoNumField`, `addRevisionListener`, `removeRevisionListener`, `getErrorCollector`, `getAutoWordStat` | `getContentThumb`, `dispose`, `createPLCOfBookmark`, `addTextListener`, `removeTextListener`, `getCommentStarts`, `getCommentEnds`, `getCommentRef`, `getBookmarkStarts`, `getBookmarkEnds`, `getTextboxDocument`, `copy`, `copyTextbox`, `insertTextboxNode`, `getRange`, `getTableRange`, `getParagraphRange`, `getText`, `getDispatcher`, `getCommentTxt`, `start`, `submit`, `commit`, `fireDocumentUpdate`, `toString`, `type_to_name`, `checkValid`, `readLockAll`, `readLock`, `tryReadLock`, `tryWriteLock`, `readLockMainDocument`, `writeLockMainDocument`, `getRangeLock`, `hashCode`, `getDocumentCorrector`, `exchangFootEndnoteTxt`, `exchangFootEndnoteRef`, `getShapes`, `hasHeadings`, `getDocParaIds` | — | **PASS** |
| `Document` | `koffice/writer/core/cn/wps/moffice/writer/core/Document.java` | `libs/word-core/src/cn/wps/moffice/writer/core/Document.rs` | **PASS** | — | — | — | **PASS** |
| `InfoFactory` | `koffice/writer/core/cn/wps/moffice/writer/data/table/InfoFactory.java` | `libs/word-core/src/cn/wps/moffice/writer/data/table/InfoFactory.rs` | **PASS** | `createTableInfos` | — | — | **PASS** |

**overall: PASS** — basename aligned; every Java class has layout PASS and a Rust counterpart for each public method.
<!-- java-rust-align:end -->

### 3.10

| 步 | Java | Rust | 产品 |
|----|------|------|------|
| 1 | `InfoFactory.createTableInfos(this)` | `Box::new(TableInfosImpl::TableInfosImpl(document))` | ctor |
| 2 | getTableInfos 字段 | `Option<&dyn TableInfos>` / trait `*mut dyn TableInfos` | TableLayouter / MainLayouter |

### 3.11

| 项 | 内容 |
|----|------|
| 阶段 | layout |
| 原始文档 | hello-world.docx `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` |
| 步骤对照 | §3.10 |
| 产品接线 | ctor `InfoFactory.createTableInfos`；layout `document.getTableInfos()` |
| 关键路径真实实现 | pass（真实 `TableInfosImpl`，非 Copy 影子） |
| 管线测试 test_id | TI6 |
| 测试结果符合预期 | pass |
| 判定 | pass |
