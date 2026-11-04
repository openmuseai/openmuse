# 01_function_rope_dump — 主故事 rope dump

| 项 | 值 |
|----|----|
| feature | wc-06-stage1-live |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `RopeDump::dump(doc) -> String` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
从已 open 的 TextDocument 主故事取出与 Java `getText(0, length)` 相同的字符流。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `RopeDump::dump` | `&TextDocument` | `SubDocument.getText(0, getLength(), buf, 0)` via `CpText::getText` |

### 1.3 输出
全文 `String`，保留 `\r`/`\f`。无主文档或 `start==end` → `""`。字符个数与 `Document::getLength()` 一致（rope 标量，与现有 `CpText` 相同）。

### 1.4 上游
wc-05 `open_text_document`（本入口不 open、不处理坏 zip）。

### 1.5 下游
03 的 `text` 字段。

### 1.6 不变量与函数改写列表
- 禁止 zip 抽 `<w:t>`；只走 `getTextRope` → `ITextRope.getText`。
- 改写列表：仅 `RopeDump::dump`。
- 无 Java 类 `RopeDump`（Stage1 dump API）。Java 语义源是 `SubDocument.getText` / `ITextRope.getText`。
- 不改写 `SubDocument.getText`（仍 DEFERRED）；产品路径用已实现的 `CpText::getText`。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `CpText::getText` | wr-09 | `getText(doc, start, end) -> String` | 是 | — | 禁止改 RunStream / CpText |
| `Document::getLength` / `getTextRope` | wc-02 SubDocument | 主故事长度与 rope 指针 | 是 | `SubDocument.getText` buffer API | 禁止改 SAX |
| `TextDocument::getMainDocument` | wc-02 | `Option<&SubDocument>` | 是 | — | — |
| `open_text_document` | wc-05 DocumentOpen | bytes → `Box<TextDocument>` | 是 | RecXwpf 测试路径 | 禁止改 SAX / PHandler |

**F1 分类：** A=`RopeDump.rs`（无 Java 模板，允许新建）；D=`CpText`/`DocumentOpen`（只调用）；E=SAX/`RunStream`；C=`SubDocument.rs` 本轮不编辑（布局已 PASS）。

## 2. 核心代码
`CpText::getText`：clamp `start/end` 到 `[0, length]`；`start==end` 或 rope 空指针 → `""`；否则 `ITextRope.getText` 填 `Vec<char>`。

`RopeDump::dump`：`getMainDocument()` 为 None → `""`；否则 `CpText::getText(main, 0, main.getLength())`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|--------|----------------|-----------|-------------|---------------|
| `ITextRope.getText` | SubDocument.java:679 / rope | [start,end) | 写入 buffer | start/end clamp 到 length |
| `CpText::getText` | CpText.rs:13 | `&dyn Document`, start, end | `String` | start&lt;0；end&gt;len；start==end；rope None |
| `RopeDump::dump` | api/stage1/RopeDump.rs | `&TextDocument` | 主故事全文 | 无 main → ""；open 失败不在本入口 |
| `open_text_document` | LayoutDocument.rs:528 | docx 字节 | `Box<TextDocument>` | 调用方处理；dump 不 catch |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|-------------|---------------|----------|-------|---------|
| RD-HELLO | dump + open hello.docx | 正常主路径 | 是 | RD1 |
| RD-CR | dump + two-para.docx | 多段 `\r` | 是 | RD2 |
| RD-EMPTY | dump after `_create` length=0 | 边界 | 是 | RD3 |
| RD-SAMPLE2 | dump + sample2.docx | CJK | 是 | RD4 |
| RD-NO-MAIN | dump ctor 无 `_create` | 失败/缺主文档 | 是 | RD5 |
| RD-LEN | dump.len vs getLength | 交互契约 | 是 | RD6 |

### 3.3 可测试性
hello / two-para / sample2 字节 + 内存 `TextDocument`；不跑 layout。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|----|--------|----------|----------|-----------------|----------|
| RD1 | RD-HELLO | — | hello.docx | 含 `Hello` | 抽 XML 得到空串或无 Hello |
| RD2 | RD-CR | — | two-para.docx | 含 `\r` 且 Hello≥2 | 两段拼成一行无段标 |
| RD3 | RD-EMPTY | `_create` 未 append | 空主故事 | `""` 且 length=0 | panic |
| RD4 | RD-SAMPLE2 | — | sample2.docx | 非空且含 `大学生` 或 `概述` | 打开失败当空串 |
| RD5 | RD-NO-MAIN | bMock 未 `_create` | 无 main | `""` | panic |
| RD6 | RD-LEN | hello 已 open | 同上 | `chars().count()==getLength()` | UTF-8 字节当 cp |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src path: `libs/word-core/src/cn/wps/moffice/writer/api/stage1/RopeDump.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/api/stage1/RopeDump.rs`  
layout pair（无 `RopeDump.java`）：`SubDocument.rs` / `SubDocument.java`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_rope_dump

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_rope_dump \
  --src src/cn/wps/moffice/writer/api/stage1/RopeDump.rs \
  --report libs/word-core/parser/features/wc-06-stage1-live/functions/01_function_rope_dump/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_rope_dump --json`)
date: 2026-09-03 16:42

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/stage1/RopeDump.rs` | 5/5 | 100.0% | 9/9 | 100.0% |
| **function total** | **5/5** | **100.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
`RopeDump.rs` 无对应 `.java`（新建 dump API）。本 function 点名的 Java 语义源：

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java
```

本轮不编辑 `SubDocument.rs`。

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --report libs/word-core/parser/features/wc-06-stage1-live/functions/01_function_rope_dump/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-03 08:43

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SubDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs` | **PASS** | `SubDocument`, `getType`, `getMainDocument`, `getTextDocument`, `getPieceTable`, `getDrawingContainer`, `getTextRope`, `getChpxTable`, `getPapxTable`, `getLength`, `getStyles`, `createPLC`, `getPLCSection`, `writeLock`, `getMax`, `getLastParaStart`, `isLocked` | `getContentThumb`, `dispose`, `createPLCOfBookmark`, `createPLCOfDocumentFiels`, `addTextListener`, `doAddTextListener`, `removeTextListener`, `createPLCOfComment`, `createPLCOfFootnote`, `createPLCOfEndnote`, `getCommentStarts`, `getCommentEnds`, `getCommentRef`, `getBookmarkStarts`, `getDocumentFileds`, `getBookmarkEnds`, `getTextboxDocument`, `copy`, `doCopy`, `copyTextbox`, `insertTextboxNode`, `getTextStream`, `getName`, `getRange`, `getTableRange`, `getParagraphRange`, `getAutoNumCount`, `getText`, `charAt`, `getDispatcher`, `getShapeAnchors`, `getTextboxText`, `getCommentTxt`, `getPLCField`, `getPLCHeader`, `getFootnoteRef`, `getFootnoteTxt`, `getEndnoteRef`, `getEndnoteTxt`, `start`, `submit`, `commit`, `getTableInfos`, `fireDocumentUpdate`, `toString`, `type_to_name`, `checkValid`, `check_shape`, `readLockAll`, `readLock`, `tryReadLock`, `tryWriteLock`, `readLockMainDocument`, `writeLockMainDocument`, `getHeadings`, `getPLCWords`, `getRangeLock`, `getOMathStarts`, `getOMathEnds`, `getPLCMathFunction`, `createPLCMath`, `createPLCOMath`, `isContainMathEquations`, `hashCode`, `getAutoNumField`, `addRevisionListener`, `removeRevisionListener`, `fireBeforeChangeAll`, `fireAfterChangeAll`, `fireBeforeChange`, `fireAfterChange`, `getErrorCollector`, `getDocumentCorrector`, `getAutoWordStat`, `exchangFootEndnoteTxt`, `exchangFootEndnoteRef`, `getShapes`, `hasHeadings`, `getDocParaIds` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
