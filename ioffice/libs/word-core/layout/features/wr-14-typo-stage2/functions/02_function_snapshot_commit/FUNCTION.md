# 02_function_snapshot_commit — owned Snapshot 提交

| 项 | 值 |
|---|---|
| feature | wr-14-typo-stage2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `snapshot_from_pages` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
全量复制 wr-13 页/行到现有 `TypoPage`/`TypoSnapshot` owned bridge；页 range 取首/末 cp；版本设 `VER_VALIDBEGIN`；重复 page id 返回错误。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| SnapshotBuild | wr-01 | duplicate check/version | 是 | — | arena |
| TypoSnapshot/TypoPage | cache | add/get page, push line | 是 | incremental dirty | Java anchors |

## 2. 核心代码
每页构造 owned `TypoPage`，复制产品行，再由 `build_snapshot` 一次提交。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| snapshot_from_pages | empty/2 pages/range | snapshot | empty, ownership, range/version |
### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| SC-EMPTY | empty+version | 是 | SC1 |
| SC-OWN | multi page/line | 是 | SC2 |
| SC-RANGE | page cp range | 是 | SC3 |
### 3.3 可测试性
直接使用 wr-13 页 fixture。
### 3.4 用例表
| test | 期望 |
|---|---|
| SC1 | 0 page, version=1 |
| SC2 | 2 page, owned line preserved |
| SC3 | [0,2) range |
### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/cache/SnapshotCommit.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/cache/SnapshotCommit.rs`
### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_snapshot_commit
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 02_function_snapshot_commit --src src/cn/wps/moffice/writer/cache/SnapshotCommit.rs --report libs/word-core/layout/features/wr-14-typo-stage2/functions/02_function_snapshot_commit/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_snapshot_commit --json`)
date: 2026-08-25 01:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/cache/SnapshotCommit.rs` | 14/14 | 100.0% | 32/33 | 97.0% |
| **function total** | **14/14** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`TypoSnapshot.java ↔ TypoSnapshot.rs`、`TypoPage.java ↔ TypoPage.rs`：**PASS**。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoPage.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoPage.rs --report libs/word-core/layout/features/wr-14-typo-stage2/functions/02_function_snapshot_commit/FUNCTION.md
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
