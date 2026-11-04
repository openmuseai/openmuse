# 08_function_shape_anchor — createPLC new PLCShapeAnchor

| 项 | 值 |
|----|----|
| feature | wc-10-subdocument-pipeline |
| order | 8 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `createPLC` 末尾 `new PLCShapeAnchor(this)`；`getShapeAnchors`；`Document` 同名门面 |
| 独立性 | 不实现 `removeShapeInfo` 删图 / `ShapeTool` / `KDeleteCommand` |
| 测试状态 | pass（SA1–SA6；§3.8 SubDocument+Document PASS；§3.9 overall PASS） |
| 阶段 | layout |

## 1. 功能说明

### 1.1 一句话

`createPLC` 对每个故事 `this.mShapeAnchors = new PLCShapeAnchor(this)`；getter 返回同一实例。去掉 Copy 影子。

### 1.2 输入

`SubDocument` ctor → `createPLC`。

### 1.3 输出

- ctor 后 getter Some
- 二次 createPLC 仍 Some（Java 每次 `new`）
- 新表 `isEmpty()` true
- 固有与 trait 同址
- header 故事 Some
- hello-world parse 后 Some

### 1.6 硬约束

使用 `data/PLCShapeAnchor.rs`。禁止第二套类型名。

### 1.7.1

| # | Java | 替代 |
|---|------|------|
| 1 | ShapeTool / KDeleteCommand | core/shape 内联块未导出；removeShapeInfo rust: no-stmt |
| 2 | getDrawingContainer 真实容器 | 仍 `*mut ()` |
| 3 | ShapeTool.isShapeTriple | rust: no-stmt |

## 3. 测试方案

| id | 期望 |
|----|------|
| SA1 | ctor Some |
| SA2 | 二次 createPLC 仍 Some |
| SA3 | isEmpty true |
| SA4 | trait 与固有同址 |
| SA5 | header Some |
| SA6 | hello-world parse Some |

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 08_function_shape_anchor

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 08_function_shape_anchor \
  --src src/cn/wps/moffice/writer/core/SubDocument.rs \
  --src src/cn/wps/moffice/writer/core/Document.rs \
  --src src/cn/wps/moffice/writer/data/PLCShapeAnchor.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/08_function_shape_anchor/FUNCTION.md
```

### 3.7

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 08_function_shape_anchor --json`)
date: 2026-09-30 22:09

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/core/Document.rs` | 8/253 | 3.2% | 6/269 | 2.2% |
| `src/cn/wps/moffice/writer/core/SubDocument.rs` | 283/829 | 34.1% | 316/971 | 32.5% |
| `src/cn/wps/moffice/writer/data/PLCShapeAnchor.rs` | 35/440 | 8.0% | 34/534 | 6.4% |
| **function total** | **326/1522** | **21.4%** |  |  |
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

`PLCShapeAnchor.rs` 本轮修 ctor/`isEmpty` 编译面；ShapeTool 路径仍 DEFERRED。§3.8/§3.9 不含该文件。

### 3.9

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/Document.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/Document.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/08_function_shape_anchor/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-30 13:53

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SubDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs` | **PASS** | `SubDocument`, `createPLCOfDocumentFiels`, `getDocumentFileds`, `getType`, `getMainDocument`, `getTextDocument`, `getPieceTable`, `getDrawingContainer`, `getTextRope`, `getChpxTable`, `getPapxTable`, `getTextStream`, `getName`, `getAutoNumCount`, `charAt`, `getLength`, `getStyles`, `getShapeAnchors`, `getTextboxText`, `getPLCField`, `getPLCSection`, `getPLCHeader`, `getFootnoteRef`, `getFootnoteTxt`, `getEndnoteRef`, `getEndnoteTxt`, `writeLock`, `getMax`, `getLastParaStart`, `getOMathStarts`, `getOMathEnds`, `getPLCMathFunction`, `createPLCMath`, `createPLCOMath`, `isContainMathEquations`, `isLocked`, `getAutoNumField`, `getErrorCollector` | `getContentThumb`, `dispose`, `createPLCOfBookmark`, `addTextListener`, `removeTextListener`, `getCommentStarts`, `getCommentEnds`, `getCommentRef`, `getBookmarkStarts`, `getBookmarkEnds`, `getTextboxDocument`, `copy`, `copyTextbox`, `insertTextboxNode`, `getRange`, `getTableRange`, `getParagraphRange`, `getText`, `getDispatcher`, `getCommentTxt`, `start`, `submit`, `commit`, `getTableInfos`, `fireDocumentUpdate`, `toString`, `type_to_name`, `checkValid`, `readLockAll`, `readLock`, `tryReadLock`, `tryWriteLock`, `readLockMainDocument`, `writeLockMainDocument`, `getHeadings`, `getPLCWords`, `getRangeLock`, `hashCode`, `addRevisionListener`, `removeRevisionListener`, `getDocumentCorrector`, `getAutoWordStat`, `exchangFootEndnoteTxt`, `exchangFootEndnoteRef`, `getShapes`, `hasHeadings`, `getDocParaIds` | — | **PASS** |
| `Document` | `koffice/writer/core/cn/wps/moffice/writer/core/Document.java` | `libs/word-core/src/cn/wps/moffice/writer/core/Document.rs` | **PASS** | — | — | — | **PASS** |

**overall: PASS** — basename aligned; every Java class has layout PASS and a Rust counterpart for each public method.
<!-- java-rust-align:end -->

### 3.10

| 步 | Java | Rust | 产品 |
|----|------|------|------|
| 1 | createPLC `new PLCShapeAnchor(this)` | `PLCShapeAnchor::PLCShapeAnchor(self)` | ctor |
| 2 | getShapeAnchors 字段 | Option 同一实例 | DrawingLayouter |

### 3.11

| 项 | 内容 |
|----|------|
| 阶段 | layout |
| 原始文档 | hello-world.docx `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` |
| 步骤对照 | §3.10 |
| 产品接线 | `createPLC` 产品入口（每个故事 `new PLCShapeAnchor(this)`） |
| 关键路径真实实现 | pass（真实 `data::PLCShapeAnchor`，非 Copy 影子；`getShapeAnchors` 返回字段） |
| 管线测试 test_id | SA6 |
| 测试结果符合预期 | pass |
| 判定 | pass |
