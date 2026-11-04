# 02_function_prop_dump — PAPX/CHPX 几何 sprm dump

| 项 | 值 |
|----|----|
| feature | wc-06-stage1-live |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `PropDump::dump(doc) -> Stage1Props` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
按 cp 区间导出影响 L2 的 PAPX/CHPX，缺 key 为 `None` 而非默认 0。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `PropDump::dump` | `&TextDocument` | `IPapxTable.seek` / `IChpxTable.seek` + `PropertySet.contains` |

### 1.3 输出
`Stage1Props`：段 191/193/197/208/210/212/214/217/220（contains）；run `sz_twip`（prop 10 ×20）、`bold`(5)、`east_asia`(4)。

### 1.4 上游
wc-01 表；wc-03 写入。本 function 不 `convertProp`。

### 1.5 下游
03。

### 1.6 不变量与函数改写列表
- 改写列表：仅 `PropDump::dump`（无 Java 类）。
-  walker：`PapxParas::walkPapx` / `ChpxRuns::walkChpx`（call-only）。
- `contains` 假 → 字段 `None`。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `IPapxTable::seek` | wc-01 | 段区间 + getProperty | 是 | 完整 PapxTable 交错模板 | 禁止改 PapxTable.rs |
| `IChpxTable::seek` | wc-01 | run 区间 | 是 | — | — |
| `PropertySet.contains` | wave0 | 缺 key | 是 | — | — |
| converters | wc-03 | 桶已写入 | 部分 | wc-07 220 对象 | 禁止 dump 写桶 |

**F1：** A=`PropDump.rs`（无 Java）；D=walkers/IPapxTable；E=SAX。

## 3. 测试方案
### 3.2 覆盖矩阵
| coverage_id | 必测? | test_id |
|-------------|-------|---------|
| PD-JC 内存 193=1 | 是 | PD1 |
| PD-MISS 193=None | 是 | PD2 |
| PD-SZ 18pt→360 | 是 | PD3 |
| PD-IND 208 Some vs hello None | 是 | PD4 |
| PD-SAMPLE2 标题 193 | 是 | PD5 |
| PD-EMPTY | 是 | PD6 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src path: `libs/word-core/src/cn/wps/moffice/writer/api/stage1/PropDump.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/api/stage1/PropDump.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_prop_dump

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_prop_dump \
  --src src/cn/wps/moffice/writer/api/stage1/PropDump.rs \
  --report libs/word-core/parser/features/wc-06-stage1-live/functions/02_function_prop_dump/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_prop_dump --json`)
date: 2026-09-03 17:12

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/stage1/PropDump.rs` | 66/68 | 97.1% | 101/104 | 97.1% |
| **function total** | **66/68** | **97.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java
```

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java --rs libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs \
  --report libs/word-core/parser/features/wc-06-stage1-live/functions/02_function_prop_dump/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-03 09:13

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SubDocument` | `koffice/writer/core/cn/wps/moffice/writer/core/SubDocument.java` | `libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs` | **PASS** | `SubDocument`, `getType`, `getMainDocument`, `getTextDocument`, `getPieceTable`, `getDrawingContainer`, `getTextRope`, `getChpxTable`, `getPapxTable`, `getLength`, `getStyles`, `createPLC`, `getPLCSection`, `writeLock`, `getMax`, `getLastParaStart`, `isLocked` | `getContentThumb`, `dispose`, `createPLCOfBookmark`, `createPLCOfDocumentFiels`, `addTextListener`, `doAddTextListener`, `removeTextListener`, `createPLCOfComment`, `createPLCOfFootnote`, `createPLCOfEndnote`, `getCommentStarts`, `getCommentEnds`, `getCommentRef`, `getBookmarkStarts`, `getDocumentFileds`, `getBookmarkEnds`, `getTextboxDocument`, `copy`, `doCopy`, `copyTextbox`, `insertTextboxNode`, `getTextStream`, `getName`, `getRange`, `getTableRange`, `getParagraphRange`, `getAutoNumCount`, `getText`, `charAt`, `getDispatcher`, `getShapeAnchors`, `getTextboxText`, `getCommentTxt`, `getPLCField`, `getPLCHeader`, `getFootnoteRef`, `getFootnoteTxt`, `getEndnoteRef`, `getEndnoteTxt`, `start`, `submit`, `commit`, `getTableInfos`, `fireDocumentUpdate`, `toString`, `type_to_name`, `checkValid`, `check_shape`, `readLockAll`, `readLock`, `tryReadLock`, `tryWriteLock`, `readLockMainDocument`, `writeLockMainDocument`, `getHeadings`, `getPLCWords`, `getRangeLock`, `getOMathStarts`, `getOMathEnds`, `getPLCMathFunction`, `createPLCMath`, `createPLCOMath`, `isContainMathEquations`, `hashCode`, `getAutoNumField`, `addRevisionListener`, `removeRevisionListener`, `fireBeforeChangeAll`, `fireAfterChangeAll`, `fireBeforeChange`, `fireAfterChange`, `getErrorCollector`, `getDocumentCorrector`, `getAutoWordStat`, `exchangFootEndnoteTxt`, `exchangFootEndnoteRef`, `getShapes`, `hasHeadings`, `getDocParaIds` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
