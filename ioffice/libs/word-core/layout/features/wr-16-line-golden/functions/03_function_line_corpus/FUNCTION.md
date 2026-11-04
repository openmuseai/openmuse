# 03_function_line_corpus — corpus 门闩

| 项 | 值 |
|---|---|
| feature | wr-16-line-golden |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `run_corpus` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
按命名 fixture 顺序解析 Java/Rust Stage2 JSON，调用 02；失败携带 fixture 名和首差异。必含 latin/cjk/heading/bold/multipage 类别，无 ignore/skip。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| Stage2Json | wr-14 | parse | 是 | — | — |
| 02 comparator | 本 feature | first diff | 是 | — | — |
| Java dump producer | koffice harness | golden JSON | `LayoutTraceCollector.getStage2Json` + demo receiver；五类真实基线已冻结 | 新 fixture 需 ADB 重跑 | 禁止以 Rust 改 Java cp |

## 2. 核心代码
确定性逐 case fail-fast；坏 Java/Rust JSON 分别标识。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| run_corpus | 5 categories/failing/bad json | Result | all pass/name failure/parse failure |
### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| CP-CATS | five Android Java Stage2 categories | 是 | CP1 |
| CP-NAME | first failing name | 是 | CP2 |
| CP-BAD | bad Java JSON | 是 | CP3 |
| CP-BAD-RUST | bad Rust JSON | 是 | CP4 |
### 3.3 可测试性
比较器只读取上游 JSON，不跑全 feature 管线。
### 3.4 用例表
CP1 读取五类真实 Java Stage2 基线并验证 schema/corpus 编排；CP2 cjk/end_cp；CP3 java-json；CP4 rust-json。
### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/linegolden/LineCorpus.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/linegolden/LineCorpus.rs`
fixtures: `libs/word-core/tests/fixtures/03_function_line_corpus/{docx,java}`
### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_line_corpus
bash .cursor/skills/feature-function/scripts/function-coverage.sh --crate libs/word-core --test 03_function_line_corpus --src src/cn/wps/moffice/writer/linegolden/LineCorpus.rs --report libs/word-core/layout/features/wr-16-line-golden/functions/03_function_line_corpus/FUNCTION.md
```
### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_line_corpus --json`)
date: 2026-08-25 02:21

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/linegolden/LineCorpus.rs` | 19/19 | 100.0% | 33/33 | 100.0% |
| **function total** | **19/19** | **100.0%** |  |  |
<!-- coverage-report:end -->
### 3.8 模板布局校验
`TypoSnapshot.java ↔ TypoSnapshot.rs`：**PASS**。
### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs --report libs/word-core/layout/features/wr-16-line-golden/functions/03_function_line_corpus/FUNCTION.md
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
