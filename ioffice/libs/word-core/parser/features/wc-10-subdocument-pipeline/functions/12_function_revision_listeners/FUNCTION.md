# 12_function_revision_listeners — add/removeRevisionListener

| 项 | 值 |
|----|----|
| feature | wc-10-subdocument-pipeline |
| order | 12 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `addRevisionListener` / `removeRevisionListener`；`fireBefore/AfterChange(All)` |
| 独立性 | 不实现 `KRevisionChange.changeRevision` 全文修订 |
| 测试状态 | pass（RL1–RL6；§3.8 SubDocument+Document+RevisionListener PASS；§3.9 overall PASS） |
| 阶段 | parse |

## 1. 功能说明

### 1.1 一句话

Java `ArrayList.add/remove` 监听器；fire 循环按索引回调。Rust 用指针身份匹配 `remove`（与引用语义一致）。

### 1.2 输入

已构造 `SubDocument` + `RevisionListener` 实现。

### 1.3 输出

- add 后 fireBeforeChangeAll 回调 1 次
- remove 后不再回调
- 两监听器都收到
- fireBefore/AfterChange 在 `isRevisionChangeAll==false` 时回调
- header 故事同样可 add
- hello-world parse 后对空列表 fire 不崩

### 1.6 硬约束

使用 `core/RevisionListener.rs`。禁止空 trait 就地影子。禁止删除 `// java`。`KRevisionChange.rs` 全文修订仍 off-surface。

**改写列表：** `RevisionListener.rs` 四方法 + 嵌套枚举类型；`SubDocument.rs` add/remove/fire。

### 1.7.1

| # | Java | 替代 |
|---|------|------|
| 1 | `KRevisionChange` 全文改写 | 未登记；枚举类型落在 `RevisionListener.rs` 以便接口编译 |
| 2 | `ArrayList.remove` equals | 指针身份（监听器无值相等） |

## 3. 测试方案

| id | 期望 |
|----|------|
| RL1 | add + fireBeforeChangeAll 计数 1 |
| RL2 | remove 后 fireAfterChangeAll 计数 0 |
| RL3 | 两监听器均收到 |
| RL4 | fireBefore/AfterChange 计数 1 |
| RL5 | header add 有效 |
| RL6 | parse 后空列表 fire 不崩 |

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 12_function_revision_listeners

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 12_function_revision_listeners \
  --src src/cn/wps/moffice/writer/core/SubDocument.rs \
  --src src/cn/wps/moffice/writer/core/Document.rs \
  --src src/cn/wps/moffice/writer/core/RevisionListener.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/12_function_revision_listeners/FUNCTION.md
```

### 3.7

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 12_function_revision_listeners --json`)
date: 2026-09-30 22:09

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/core/Document.rs` | 8/253 | 3.2% | 6/269 | 2.2% |
| `src/cn/wps/moffice/writer/core/SubDocument.rs` | 318/829 | 38.4% | 380/971 | 39.1% |
| `src/cn/wps/moffice/writer/core/RevisionListener.rs` | (not in report) | — | — | — |
| **function total** | **326/1082** | **30.1%** |  |  |
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
  --rs libs/word-core/src/cn/wps/moffice/writer/core/RevisionListener.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/RevisionListener.java
```

`KRevisionChange.rs` 仍 off-surface。§3.8/§3.9 不含该文件。

### 3.9

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/Document.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/Document.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/RevisionListener.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/RevisionListener.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/12_function_revision_listeners/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-30 14:08

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SubDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs` | **PASS** | `SubDocument`, `createPLCOfDocumentFiels`, `getDocumentFileds`, `getType`, `getMainDocument`, `getTextDocument`, `getPieceTable`, `getDrawingContainer`, `getTextRope`, `getChpxTable`, `getPapxTable`, `getTextStream`, `getName`, `getAutoNumCount`, `charAt`, `getLength`, `getStyles`, `getShapeAnchors`, `getTextboxText`, `getPLCField`, `getPLCSection`, `getPLCHeader`, `getFootnoteRef`, `getFootnoteTxt`, `getEndnoteRef`, `getEndnoteTxt`, `getTableInfos`, `writeLock`, `getMax`, `getLastParaStart`, `getHeadings`, `getPLCWords`, `getOMathStarts`, `getOMathEnds`, `getPLCMathFunction`, `createPLCMath`, `createPLCOMath`, `isContainMathEquations`, `isLocked`, `getAutoNumField`, `addRevisionListener`, `removeRevisionListener`, `getErrorCollector`, `getAutoWordStat` | `getContentThumb`, `dispose`, `createPLCOfBookmark`, `addTextListener`, `removeTextListener`, `getCommentStarts`, `getCommentEnds`, `getCommentRef`, `getBookmarkStarts`, `getBookmarkEnds`, `getTextboxDocument`, `copy`, `copyTextbox`, `insertTextboxNode`, `getRange`, `getTableRange`, `getParagraphRange`, `getText`, `getDispatcher`, `getCommentTxt`, `start`, `submit`, `commit`, `fireDocumentUpdate`, `toString`, `type_to_name`, `checkValid`, `readLockAll`, `readLock`, `tryReadLock`, `tryWriteLock`, `readLockMainDocument`, `writeLockMainDocument`, `getRangeLock`, `hashCode`, `getDocumentCorrector`, `exchangFootEndnoteTxt`, `exchangFootEndnoteRef`, `getShapes`, `hasHeadings`, `getDocParaIds` | — | **PASS** |
| `Document` | `koffice/writer/core/cn/wps/moffice/writer/core/Document.java` | `libs/word-core/src/cn/wps/moffice/writer/core/Document.rs` | **PASS** | — | — | — | **PASS** |
| `RevisionListener` | `koffice/writer/core/cn/wps/moffice/writer/core/RevisionListener.java` | `libs/word-core/src/cn/wps/moffice/writer/core/RevisionListener.rs` | **PASS** | — | — | — | **PASS** |

**overall: PASS** — basename aligned; every Java class has layout PASS and a Rust counterpart for each public method.
<!-- java-rust-align:end -->

### 3.10

| 步 | Java | Rust | 产品 |
|----|------|------|------|
| 1 | `revisionListeners.add(listener)` | `Vec` 存 `*mut dyn RevisionListener` | add |
| 2 | `remove(listener)` | 指针身份 | remove |
| 3 | fire 循环 get(i) 回调 | `(*listener).beforeChangeAll` | 修订通知 |

### 3.11

| 项 | 内容 |
|----|------|
| 阶段 | parse |
| 原始文档 | hello-world.docx `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` |
| 步骤对照 | §3.10 |
| 产品接线 | `SubDocument.addRevisionListener` / fire 循环 |
| 关键路径真实实现 | pass（真实 `RevisionListener` trait + ArrayList 语义） |
| 管线测试 test_id | RL6 |
| 测试结果符合预期 | pass |
| 判定 | pass |
