# 06_function_autonum_cache — ctor AutoNumCount / AutoNumFieldCache

| 项 | 值 |
|----|----|
| feature | wc-10-subdocument-pipeline |
| order | 6 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SubDocument` ctor `new AutoNumCount(this)` / `new AutoNumFieldCache(this)`；`getAutoNumCount` / `getAutoNumField`；`Document` 同名门面 |
| 独立性 | 不实现 `getAutoNum(long)` 编号算法、`constructTo`、dispatcher 监听 |
| 测试状态 | pass（AN1–AN6；§3.8 SubDocument+Document PASS；§3.9 overall PASS） |
| 阶段 | parse+layout |

## 1. 功能说明

### 1.1 一句话

Java ctor 对每个故事执行 `this.mAutoNumCount = new AutoNumCount(this)` 与 `this.mAutoNumField = new AutoNumFieldCache(this)`；getter 返回同一字段实例。Rust 去掉 Copy 影子，字段为已登记 `data::autonum::AutoNumCount` / `data::field::AutoNumFieldCache`；`Document::getAutoNumCount` 不再是 `*mut ()`。

### 1.2 输入

`TextDocument::new` / parse → `new SubDocument(parent, type)`（所有 type，含 header）。

### 1.3 输出

- ctor 后 `getAutoNumCount()` / `getAutoNumField()` 非空
- 两次 getter 同一实例
- 固有方法与 `Document` trait 同址
- header 故事同样挂上 cache
- hello-world parse 后 trait getter 非空

不测：`getAutoNum(lfo,lvl,cp)` / `constructTo`；`listeners.add`；`getDispatcher().addDocumentListener`。

### 1.4–1.5

上游 01。下游 `AutoNumParse.getAutoNumField`（parse 产品）；`AutoNumLayouter.getAutoNumCount`（layout，`constructTo` 仍非本 function）。

### 1.6 硬约束

去掉 SubDocument.rs Copy 影子；在 `data/mod.rs` 的 inline `autonum` 块登记 `pub mod AutoNumCount`（path `data/autonum/AutoNumCount.rs`）。禁止第二套类型名。禁止为绿删除 `// java`。

**改写列表：** `SubDocument.rs` ctor 赋值 + inherent getters + `impl Document`；`Document.rs` `getAutoNumCount` 签名改为 `*mut AutoNumCount`。`AutoNumCount.rs` / `AutoNumFieldCache.rs` 只补 Document 形参 ctor（存 `mDocument`）；不全量 java2rust 叶。

### 1.7.1

| # | Java | 替代 |
|---|------|------|
| 1 | `this.mDocument.getDispatcher().addDocumentListener(this)`（两 ctor） | `getDispatcher` 仍 `todo!("DEFERRED_NON_FUNCTION")`；ctor 只赋 `mDocument` |
| 2 | `listeners.add(this.mAutoNumCount/Field)` | TextRopeListeners::add 仍 DEFERRED；`// rust: no-stmt` |
| 3 | `AutoNumCount` 字段默认 `mLsidCounts`/`mAutoNumCache`/`tmpInt` 等 | 本 function 只保证 `mDocument` 赋值；其余叶后续 |
| 4 | `AutoNumFieldCache.getAutoNum(long)` 全部分支 | 已有软实现返回 `n`；算法不在本 I/O |
| 5 | `AutoNumLayouter` `constructTo` / `getAutoNum(lfo,lvl,cp)` | rust: no-stmt 直至编号叶 |

## 3. 测试方案

| id | 期望 |
|----|------|
| AN1 | ctor 后两个 getter Some |
| AN2 | 两次 getter 同指针 |
| AN3 | trait 与固有 AutoNumCount 同址 |
| AN4 | trait 与固有 AutoNumFieldCache 同址 |
| AN5 | header 故事同样 Some |
| AN6 | hello-world parse 后 trait getter 非空 |

### 3.5–3.11

crate `libs/word-core`；src `SubDocument.rs` `Document.rs` `AutoNumCount.rs` `AutoNumFieldCache.rs`；test `06_function_autonum_cache.rs`。

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 06_function_autonum_cache

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 06_function_autonum_cache \
  --src src/cn/wps/moffice/writer/core/SubDocument.rs \
  --src src/cn/wps/moffice/writer/core/Document.rs \
  --src src/cn/wps/moffice/writer/data/autonum/AutoNumCount.rs \
  --src src/cn/wps/moffice/writer/data/field/AutoNumFieldCache.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/06_function_autonum_cache/FUNCTION.md
```

### 3.7

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 06_function_autonum_cache --json`)
date: 2026-09-30 16:22

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/core/Document.rs` | 8/253 | 3.2% | 6/269 | 2.2% |
| `src/cn/wps/moffice/writer/core/SubDocument.rs` | 295/710 | 41.5% | 317/813 | 39.0% |
| `src/cn/wps/moffice/writer/data/autonum/AutoNumCount.rs` | 9/18 | 50.0% | 4/13 | 30.8% |
| `src/cn/wps/moffice/writer/data/field/AutoNumFieldCache.rs` | 16/25 | 64.0% | 4/13 | 30.8% |
| **function total** | **328/1006** | **32.6%** |  |  |
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

`AutoNumCount.rs` / `AutoNumFieldCache.rs` 本轮只补 Document 形参 ctor（dispatcher 仍 DEFERRED），不扩写全叶；故 §3.8/§3.9 不含这两文件。

### 3.9

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/Document.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/Document.rs \
  --report libs/word-core/parser/features/wc-10-subdocument-pipeline/functions/06_function_autonum_cache/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-30 08:21

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SubDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs` | **PASS** | `SubDocument`, `createPLCOfDocumentFiels`, `getDocumentFileds`, `getType`, `getMainDocument`, `getTextDocument`, `getPieceTable`, `getDrawingContainer`, `getTextRope`, `getChpxTable`, `getPapxTable`, `getName`, `getAutoNumCount`, `charAt`, `getLength`, `getStyles`, `getTextboxText`, `getPLCField`, `getPLCSection`, `getPLCHeader`, `getFootnoteRef`, `getFootnoteTxt`, `getEndnoteRef`, `getEndnoteTxt`, `writeLock`, `getMax`, `getLastParaStart`, `getOMathStarts`, `getOMathEnds`, `getPLCMathFunction`, `createPLCMath`, `createPLCOMath`, `isContainMathEquations`, `isLocked`, `getAutoNumField`, `getErrorCollector` | `getContentThumb`, `dispose`, `createPLCOfBookmark`, `addTextListener`, `removeTextListener`, `getCommentStarts`, `getCommentEnds`, `getCommentRef`, `getBookmarkStarts`, `getBookmarkEnds`, `getTextboxDocument`, `copy`, `copyTextbox`, `insertTextboxNode`, `getTextStream`, `getRange`, `getTableRange`, `getParagraphRange`, `getText`, `getDispatcher`, `getShapeAnchors`, `getCommentTxt`, `start`, `submit`, `commit`, `getTableInfos`, `fireDocumentUpdate`, `toString`, `type_to_name`, `checkValid`, `readLockAll`, `readLock`, `tryReadLock`, `tryWriteLock`, `readLockMainDocument`, `writeLockMainDocument`, `getHeadings`, `getPLCWords`, `getRangeLock`, `hashCode`, `addRevisionListener`, `removeRevisionListener`, `getDocumentCorrector`, `getAutoWordStat`, `exchangFootEndnoteTxt`, `exchangFootEndnoteRef`, `getShapes`, `hasHeadings`, `getDocParaIds` | — | **PASS** |
| `Document` | `koffice/writer/core/cn/wps/moffice/writer/core/Document.java` | `libs/word-core/src/cn/wps/moffice/writer/core/Document.rs` | **PASS** | — | — | — | **PASS** |

**overall: PASS** — basename aligned; every Java class has layout PASS and a Rust counterpart for each public method.
<!-- java-rust-align:end -->

### 3.10

| 步 | Java | Rust | 产品 |
|----|------|------|------|
| 1 | ctor `new AutoNumCount(this)` | `AutoNumCount::AutoNumCount(this)` 真实模块 | ctor |
| 2 | ctor `new AutoNumFieldCache(this)` | `AutoNumFieldCache::AutoNumFieldCache(this)` | `AutoNumParse` |
| 3 | `getAutoNumCount` / `getAutoNumField` 返回字段 | Option 同一实例；trait 去 `*mut ()` | `AutoNumParse.getAutoNumField` |

### 3.11

| 项 | 内容 |
|----|------|
| 阶段 | parse+layout |
| 原始文档 | hello-world.docx `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` |
| 步骤对照 | §3.10 |
| 产品接线 | `AutoNumParse` 调 `Document::getAutoNumField` |
| 关键路径真实实现 | pass（真实 `AutoNumCount`/`AutoNumFieldCache` 实例，非 Copy 影子；`Document::getAutoNumCount` 为 `*mut AutoNumCount`） |
| 管线测试 test_id | AN6 |
| 测试结果符合预期 | pass |
| 判定 | pass |
