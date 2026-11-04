# 02_function_line_compare — Java/Rust 行表比较

| 项 | 值 |
|---|---|
| feature | wr-16-line-golden |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `compare_lines` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
page/line/start_cp/end_cp 精确；x/y/width/height 只允许显式 twip 容差；按字段顺序返回第一处差异；行数不同单独报告；忽略 xpos。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| 01 normalize | 本 feature | validated order | 是 | — | — |
| Java Stage2 dump | fixture | rows | comparator 接口完成 | 外部生成工具 | 不改金 cp |

## 2. 核心代码
先规范化两侧，再逐行按固定字段序比较。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| compare | equal/cp/geo/first/count | Result | exact/tolerance/order/count |
### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| LC-EQ | equal | 是 | LC1 |
| LC-CP | cp exact | 是 | LC2 |
| LC-TOL | geometry tol | 是 | LC3 |
| LC-FIRST | first field | 是 | LC4 |
| LC-COUNT | count | 是 | LC5 |
| LC-INVALID | invalid expected/actual | 是 | LC6;LC7 |
### 3.3 可测试性
两侧均用声明的 JSON/row fixture。
### 3.4 用例表
LC1 pass；LC2 end_cp；LC3 ±1 pass/±2 fail；LC4 page first；LC5 line-count。
### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/linegolden/LineCompare.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/linegolden/LineCompare.rs`
### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_line_compare
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 02_function_line_compare --src src/cn/wps/moffice/writer/linegolden/LineCompare.rs --report libs/word-core/layout/features/wr-16-line-golden/functions/02_function_line_compare/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_line_compare --json`)
date: 2026-08-25 02:04

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/linegolden/LineCompare.rs` | 44/44 | 100.0% | 73/73 | 100.0% |
| **function total** | **44/44** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`TypoSnapshot.java ↔ TypoSnapshot.rs`：**PASS**。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs --report libs/word-core/layout/features/wr-16-line-golden/functions/02_function_line_compare/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-24 18:02

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoSnapshot` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs` | **PASS** | `getVersion` | `newObject`, `reuseSnapshotAndCache`, `createTypoSnapshot`, `recycleTypoSnaptshot`, `TypoSnapshot`, `setEntranceObjAlloc`, `initCache`, `getData`, `assignmentData`, `cloneSnapshot`, `release`, `fork`, `cleanFork`, `reset`, `recycleNormalObjIndexs`, `switchUsingThread`, `isInSameThread`, `verifySameThread`, `getTypoTools`, `alloc`, `checkExtendSelfEnable`, `checkIn`, `checkMem`, `extendAlloc`, `hasFree`, `free`, `freeRanges`, `doFreeRanges`, `mergeFreeRange`, `addRange`, `getAllocSize`, `getModifyPageRange`, `setModify`, `setModifyFrom`, `setModifyRange`, `loadBlock`, `prepareWriteableBlock`, `get`, `getWriteBlockData`, `set`, `mutableSet`, `fill`, `setBoolBitSet`, `setIntBitSet`, `getBoolBitSet`, `getIntBitSet`, `compress`, `getKeyMap`, `decompress`, `enOrDescryp`, `intToZigZag`, `zigzagToInt`, `transToByteArray`, `transToIntsArray`, `extend`, `extendFloatRound`, `round`, `scale`, `copy`, `getSum`, `getAbsSum`, `processRTLWidths`, `addObj`, `getObjsIndexMap`, `beforeCommit`, `setFirstFrozenSize`, `getTypoType`, `getEntraceObj`, `addCPRef`, `addRefCP`, `getCPRef`, `getDocType`, `getRevision`, `getTableRevision`, `getCPRefs`, `getCPRefFreeLists`, `getSectionPageCount`, `getSectionPageIndex`, `getSectionStartPageIndex`, `getSectionMsoNfc`, `getLayoutPageSize`, `startSection`, `getLayoutPageList`, `setDocumentWidth`, `getDocumentWidth`, `setDocumentHeight`, `getDocumentHeight`, `setDocumentTop`, `getDocumentTop`, `setDocumentBottom`, `getDocumentBottom`, `getMaxPageHeight`, `setMaxPageHeight`, `getTemplete`, `getSectionIndex`, `setTemplete`, `isTemplateReachable`, `getSections`, `updateSections`, `getFirstInfo`, `updateLayoutPageList`, `getDocument`, `getTypoDocument`, `getPosVisitor`, `insertPage`, `getCrashRanges`, `getEditCrackRanges`, `getPagePosition`, `getContentRect`, `hasGap`, `getEditPagePosition`, `swapToLocal`, `cancleBlockSwapWait`, `replaceCrackRange`, `swapFromLocal`, `updateCrackRanges`, `checkInLocal`, `serializeJavaObjs`, `unserializeJavaObjs`, `dumpCPRef`, `dump`, `getBlockSize`, `getInLocalBlockSize`, `getSummaryInfo`, `checkFirstCacheSuitable`, `invalidFirstCache`, `isFirstCache` | — | **PASS** |
| `SafeTrace` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs` | **PASS** | — | `SafeTrace`, `changeUsingThread`, `dispose`, `getTypoDocument`, `onSnapshotCommit`, `release`, `get`, `isEmpty`, `detach`, `releaseAssociatedTraceInstance` | — | **PASS** |
| `IntArrayList` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs` | **PASS** | — | `IntArrayList`, `getData`, `addAll`, `offsetValues` | — | **PASS** |
| `Data` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs` | **PASS** | — | `newObject`, `Data`, `resetMaps`, `clear`, `init`, `getRefCount`, `incRef`, `decRef`, `fork`, `cleanFork` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
