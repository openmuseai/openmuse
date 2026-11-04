# 07_function_text_stream — TextData.createTextStream

| 项 | 值 |
|----|----|
| feature | wc-10-subdocument-pipeline |
| order | 7 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `TextData.createTextStream` → `new KTextStream(document)`；`SubDocument.getTextStream`；`Document` 同名门面 |
| 独立性 | 不实现 insert/remove/过滤器链；不实现 `setListener(AutoWordStat)`（function 10） |
| 测试状态 | pass（TS1–TS6；§3.8 SubDocument+Document+TextData PASS；§3.9 overall PASS） |
| 阶段 | parse |

## 1. 功能说明

### 1.1 一句话

Java ctor `this.mTextStream = TextData.createTextStream(this)` 即 `new KTextStream(document)`；getter 返回同一接口引用。Rust 去掉 Copy 影子与就地 `TextData`，字段为 `Box<dyn ITextStream>`（`KTextStream`）。

### 1.2 输入

`new SubDocument(parent, type)`（所有故事）。

### 1.3 输出

- ctor 后 `getTextStream()` 非空
- 两次 getter 同一实例
- 固有方法与 `Document` trait 同址
- `ITextStream.getLength()` 等于 `Document.getLength()`（经 `getTextRope().length()`）
- header 故事同样挂上 stream
- hello-world parse 后 trait getter 非空

不测：`insertText` / `removeText` / `pushFilter` 过滤器；`setListener`。

### 1.4–1.5

上游 01。下游修订过滤 / 写入；产品入口为本 ctor 赋值（parse 阶段创建 stream）。

### 1.6 硬约束

禁止第二套 `SoftITextStream` 类型名。使用已有 `data/ITextStream.rs` trait 与 `data/KTextStream.rs`。禁止删除 `// java`。

**改写列表：** `TextData.rs` `createTextStream`；`KTextStream.rs` ctor（`mDocument` 赋值）+ `getLength`/`getTextRope`/`getDocument`；`SubDocument.rs` 字段 + getter + `impl Document`；`Document.rs` `getTextStream` 去 `*mut ()`。过滤器叶仍 off-surface。

### 1.7.1

| # | Java | 替代 |
|---|------|------|
| 1 | `pushFilter(new TextStream_NormalFilter(this))` | 过滤器模板仍 off-surface；`// rust: no-stmt` |
| 2 | `mRevisionFilter = new TextStream_RevisionFilter(this)` + `pushFilter` | 同上 |
| 3 | insert/remove/modify/visit/seek 与 filter 转发 | `todo!("DEFERRED_NON_FUNCTION")` + 保留 `// java` |
| 4 | `this.mTextStream.setListener(this.mAutoWordStat)` | function 10 |
| 5 | `IRevisionModifyListener` / `autoWordStat` 登记 | function 10；避免拉入未编译 `AutoWordStat.rs` |

## 3. 测试方案

| id | 期望 |
|----|------|
| TS1 | ctor 后 getter Some |
| TS2 | 两次 getter 同指针 |
| TS3 | trait 与固有同址 |
| TS4 | stream.getLength == document.getLength |
| TS5 | header 故事 Some |
| TS6 | hello-world parse 后 trait getter 非空 |

### 3.5–3.11

crate `libs/word-core`；src `SubDocument.rs` `Document.rs` `TextData.rs` `KTextStream.rs` `ITextStream.rs`。

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 07_function_text_stream

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 07_function_text_stream \
  --src src/cn/wps/moffice/writer/core/SubDocument.rs \
  --src src/cn/wps/moffice/writer/core/Document.rs \
  --src src/cn/wps/moffice/writer/data/TextData.rs \
  --src src/cn/wps/moffice/writer/data/KTextStream.rs \
  --src src/cn/wps/moffice/writer/data/ITextStream.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/07_function_text_stream/FUNCTION.md
```

### 3.7

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 07_function_text_stream --json`)
date: 2026-09-30 16:29

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/core/Document.rs` | 8/253 | 3.2% | 6/269 | 2.2% |
| `src/cn/wps/moffice/writer/core/SubDocument.rs` | 282/717 | 39.3% | 307/821 | 37.4% |
| `src/cn/wps/moffice/writer/data/ITextStream.rs` | 0/42 | 0.0% | 0/49 | 0.0% |
| `src/cn/wps/moffice/writer/data/KTextStream.rs` | 23/114 | 20.2% | 23/113 | 20.4% |
| `src/cn/wps/moffice/writer/data/TextData.rs` | 3/3 | 100.0% | 4/4 | 100.0% |
| **function total** | **316/1129** | **28.0%** |  |  |
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
  --rs libs/word-core/src/cn/wps/moffice/writer/data/TextData.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/data/TextData.java
```

`KTextStream.rs` / `ITextStream.rs` 过滤器与 insert/remove 仍 DEFERRED；本轮 §3.8/§3.9 不含这两文件（同 05 对 `PLCDocumentFields` 的口径）。

### 3.9

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/Document.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/Document.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/data/TextData.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/data/TextData.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/07_function_text_stream/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-30 08:29

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SubDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs` | **PASS** | `SubDocument`, `createPLCOfDocumentFiels`, `getDocumentFileds`, `getType`, `getMainDocument`, `getTextDocument`, `getPieceTable`, `getDrawingContainer`, `getTextRope`, `getChpxTable`, `getPapxTable`, `getTextStream`, `getName`, `getAutoNumCount`, `charAt`, `getLength`, `getStyles`, `getTextboxText`, `getPLCField`, `getPLCSection`, `getPLCHeader`, `getFootnoteRef`, `getFootnoteTxt`, `getEndnoteRef`, `getEndnoteTxt`, `writeLock`, `getMax`, `getLastParaStart`, `getOMathStarts`, `getOMathEnds`, `getPLCMathFunction`, `createPLCMath`, `createPLCOMath`, `isContainMathEquations`, `isLocked`, `getAutoNumField`, `getErrorCollector` | `getContentThumb`, `dispose`, `createPLCOfBookmark`, `addTextListener`, `removeTextListener`, `getCommentStarts`, `getCommentEnds`, `getCommentRef`, `getBookmarkStarts`, `getBookmarkEnds`, `getTextboxDocument`, `copy`, `copyTextbox`, `insertTextboxNode`, `getRange`, `getTableRange`, `getParagraphRange`, `getText`, `getDispatcher`, `getShapeAnchors`, `getCommentTxt`, `start`, `submit`, `commit`, `getTableInfos`, `fireDocumentUpdate`, `toString`, `type_to_name`, `checkValid`, `readLockAll`, `readLock`, `tryReadLock`, `tryWriteLock`, `readLockMainDocument`, `writeLockMainDocument`, `getHeadings`, `getPLCWords`, `getRangeLock`, `hashCode`, `addRevisionListener`, `removeRevisionListener`, `getDocumentCorrector`, `getAutoWordStat`, `exchangFootEndnoteTxt`, `exchangFootEndnoteRef`, `getShapes`, `hasHeadings`, `getDocParaIds` | — | **PASS** |
| `Document` | `koffice/writer/core/cn/wps/moffice/writer/core/Document.java` | `libs/word-core/src/cn/wps/moffice/writer/core/Document.rs` | **PASS** | — | — | — | **PASS** |
| `TextData` | `koffice/writer/core/cn/wps/moffice/writer/data/TextData.java` | `libs/word-core/src/cn/wps/moffice/writer/data/TextData.rs` | **PASS** | `createTextStream` | — | — | **PASS** |

**overall: PASS** — basename aligned; every Java class has layout PASS and a Rust counterpart for each public method.
<!-- java-rust-align:end -->

### 3.10

| 步 | Java | Rust | 产品 |
|----|------|------|------|
| 1 | `TextData.createTextStream(this)` | `Box::new(KTextStream::KTextStream(document))` | ctor |
| 2 | `new KTextStream(document)` 赋 `mDocument` | 同 | getter |
| 3 | `getTextStream` 返回字段 | `Option<&dyn ITextStream>` / trait `*mut dyn ITextStream` | parse 后读 stream |

### 3.11

| 项 | 内容 |
|----|------|
| 阶段 | parse |
| 原始文档 | hello-world.docx `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` |
| 步骤对照 | §3.10 |
| 产品接线 | `SubDocument` ctor 调 `TextData.createTextStream` |
| 关键路径真实实现 | pass（真实 `KTextStream` via `TextData.createTextStream`，非 Copy 影子；`getLength` 走 `getTextRope().length()`） |
| 管线测试 test_id | TS6 |
| 测试结果符合预期 | pass |
| 判定 | pass |
