# 03_function_stage2_json — Stage2 JSON

| 项 | 值 |
|---|---|
| feature | wr-14-typo-stage2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `to_stage2_json` / `from_stage2_json` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
稳定序列化八个 Stage2 行字段并严格反序列化；不含 xpos、颜色和 command 数。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| Stage2Line | 01 | serde fields | 是 | — | 重排 |
| serde_json | crate | pretty/parse | 是 | streaming | — |

## 2. 核心代码
`serde_json::to_string_pretty` / `from_str`，错误原样返回。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| to/from | row/bad json | JSON/Result | roundtrip/schema/error |
### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| SJ-RT | roundtrip | 是 | SJ1 |
| SJ-NAME | field names | 是 | SJ2 |
| SJ-BAD | invalid json | 是 | SJ3 |
### 3.3 可测试性
纯内存 JSON。
### 3.4 用例表
| test | 期望 |
|---|---|
| SJ1 | Vec 精确往返 |
| SJ2 | start_cp/height 存在 |
| SJ3 | Err |
### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/cache/Stage2Json.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/cache/Stage2Json.rs`
### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_stage2_json
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 03_function_stage2_json --src src/cn/wps/moffice/writer/cache/Stage2Json.rs --report libs/word-core/layout/features/wr-14-typo-stage2/functions/03_function_stage2_json/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_stage2_json --json`)
date: 2026-08-25 01:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/cache/Stage2Json.rs` | 6/6 | 100.0% | 8/8 | 100.0% |
| **function total** | **6/6** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`TypoSnapshot.java ↔ TypoSnapshot.rs`：**PASS**。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs --report libs/word-core/layout/features/wr-14-typo-stage2/functions/03_function_stage2_json/FUNCTION.md
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
