# 11_function_headings_words — getHeadings / getPLCWords

| 项 | 值 |
|----|----|
| feature | wc-10-subdocument-pipeline |
| order | 11 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | 懒创建 `new KPLC_Headings(this)`（仅 type 0）与 `new KPLC_Words(this)`；`Document` 同名门面 |
| 独立性 | 不实现 papx 全扫描 `rebuild_all` / WordNode 树 |
| 测试状态 | pass（HW1–HW6；§3.8 SubDocument+Document PASS；§3.9 overall PASS） |
| 阶段 | layout |

## 1. 功能说明

### 1.1 一句话

`getHeadings()` 仅主文档懒创建 `KPLC_Headings`；`getPLCWords()` 各故事双检懒创建 `KPLC_Words`。去掉 Copy 影子。

### 1.2 输入

已构造的 `SubDocument`；首次 getter。

### 1.3 输出

- 主文档 getHeadings Some；header None
- 所有故事 getPLCWords Some
- 两次 getter 同一实例
- 固有与 trait 同址
- hello-world parse 后主文档两者非空

### 1.6 硬约束

使用 `core/heading/KPLC_Headings.rs` 与 `core/words/KPLC_Words.rs`。禁止删除 `SubDocument`/`Document` 的 `// java`。

**改写列表：** `KPLC_Headings`/`KPLC_Words` ctor + `size`；`SubDocument` 懒 getter；`Document` 门面去 `*mut ()`。

### 1.7.1

| # | Java | 替代 |
|---|------|------|
| 1 | `rebuild_all()` papx 扫描 | rust: no-stmt |
| 2 | `t.add(this.mHeadings)` TextRopeListeners | rust: no-stmt |
| 3 | `readLock()` / `synchronized` | rust: no-stmt |
| 4 | KPLC 全方法模板 | 编译面 ctor/`size`；§3.8 不含 KPLC_Headings.rs / KPLC_Words.rs |

## 3. 测试方案

| id | 期望 |
|----|------|
| HW1 | 主文档 getHeadings Some |
| HW2 | header getHeadings None |
| HW3 | 主/页眉 getPLCWords Some |
| HW4 | headings 两次同指针 |
| HW5 | trait 与固有同址 |
| HW6 | hello-world parse 后两者非空 |

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 11_function_headings_words

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 11_function_headings_words \
  --src src/cn/wps/moffice/writer/core/SubDocument.rs \
  --src src/cn/wps/moffice/writer/core/Document.rs \
  --src src/cn/wps/moffice/writer/core/heading/KPLC_Headings.rs \
  --src src/cn/wps/moffice/writer/core/words/KPLC_Words.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/11_function_headings_words/FUNCTION.md
```

### 3.7

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 11_function_headings_words --json`)
date: 2026-09-30 22:09

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/core/Document.rs` | 8/253 | 3.2% | 6/269 | 2.2% |
| `src/cn/wps/moffice/writer/core/SubDocument.rs` | 320/829 | 38.6% | 353/971 | 36.4% |
| `src/cn/wps/moffice/writer/core/heading/KPLC_Headings.rs` | 11/22 | 50.0% | 13/22 | 59.1% |
| `src/cn/wps/moffice/writer/core/words/KPLC_Words.rs` | 3/10 | 30.0% | 3/12 | 25.0% |
| **function total** | **342/1114** | **30.7%** |  |  |
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

`KPLC_Headings.rs` / `KPLC_Words.rs` 本轮为 ctor/`size` 编译面。§3.8/§3.9 不含这两文件。

### 3.9

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/Document.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/Document.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/11_function_headings_words/FUNCTION.md
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
| 1 | type==0 懒 `new KPLC_Headings(this)` | `KPLC_Headings::KPLC_Headings(self)` | getHeadings |
| 2 | 双检 `new KPLC_Words(this)` | `KPLC_Words::KPLC_Words(self)` | getPLCWords |
| 3 | Document 门面 | `*mut dyn PLC_Headings` / `IPLC_Words` | layout 读目录/词表 |

### 3.11

| 项 | 内容 |
|----|------|
| 阶段 | layout |
| 原始文档 | hello-world.docx `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` |
| 步骤对照 | §3.10 |
| 产品接线 | `Document::getHeadings` / `getPLCWords` 懒创建 |
| 关键路径真实实现 | pass（真实 `KPLC_*` 模块，非 Copy 影子） |
| 管线测试 test_id | HW6 |
| 测试结果符合预期 | pass |
| 判定 | pass |
