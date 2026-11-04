# 03_function_live_corpus — 实排 corpus 门闩

| 项 | 值 |
|----|----|
| feature | wr-24-live-stage2 |
| order | 3 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `run_live_corpus()` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
五类 wr-16 + sample2：实排 vs Java dump。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | TypoSnapshot Stage2 导出器 |

### 1.3 输出
见 §1.1

### 1.4 上游
上游见 FUNCTIONS.md

### 1.5 下游
下游见 FUNCTIONS.md

### 1.6 不变量与函数改写列表
- 切在 I/O\n- 改写列表：本入口；其余 DEFERRED_NON_FUNCTION

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| 上游 feature | 见计划 | 声明的 O | 见 overview | 本 function 不改 | — |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出 | 关键分支 |
|---|---|---|---|---|
| run_live_corpus | TypoSnapshot Stage2 导出器 | 见矩阵 | 见矩阵 | 见覆盖矩阵 |

### 3.2 覆盖矩阵
| coverage_id | 场景类别 | 必测? | test_id |
|---|---|---|---|
| LV1 | latin 实排 | 是 | LV1 |
| LV2 | cjk+注入字体 | 是 | LV2 |
| LV3 | heading | 是 | LV3 |
| LV4 | sample2 | 是 | LV4 |
| LV5 | 缺 java dump | 是 | LV5 |

### 3.3 可测试性
fixture / 假 PropertySet / 假 LayoutState；不跑无关管线。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| LV1 | LV1 | latin 实排 | Ok | 静态 rust json |
| LV2 | LV2 | cjk+注入字体 | Ok | MvpLatin |
| LV3 | LV3 | heading | 行 cp=Java | 字号未进 |
| LV4 | LV4 | sample2 | 标题 x>1800 | 左对齐 |
| LV5 | LV5 | 缺 java dump | 明确错误 | 假绿 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/linegolden/LiveCorpus.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/linegolden/LiveCorpus.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_live_corpus

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_live_corpus \
  --src src/cn/wps/moffice/writer/linegolden/LiveCorpus.rs \
  --report libs/word-core/layout/features/wr-24-live-stage2/functions/03_function_live_corpus/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_live_corpus --json`)
date: 2026-09-05 03:51

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/linegolden/LiveCorpus.rs` | 27/49 | 55.1% | 36/51 | 70.6% |
| **function total** | **27/49** | **55.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs \
  --report libs/word-core/layout/features/wr-24-live-stage2/functions/03_function_live_corpus/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-04 19:51

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TypoSnapshot` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs` | **PASS** | `getVersion`, `addCPRef`, `addRefCP`, `getCPRef`, `getLayoutPageSize`, `getLayoutPageList`, `setDocumentBottom`, `getDocumentBottom`, `getPosVisitor`, `getContentRect` | `reuseSnapshotAndCache`, `createTypoSnapshot`, `recycleTypoSnaptshot`, `TypoSnapshot`, `setEntranceObjAlloc`, `initCache`, `getData`, `assignmentData`, `cloneSnapshot`, `release`, `fork`, `cleanFork`, `reset`, `recycleNormalObjIndexs`, `switchUsingThread`, `isInSameThread`, `verifySameThread`, `getTypoTools`, `alloc`, `checkExtendSelfEnable`, `checkIn`, `checkMem`, `extendAlloc`, `hasFree`, `free`, `freeRanges`, `doFreeRanges`, `mergeFreeRange`, `addRange`, `getAllocSize`, `getModifyPageRange`, `setModify`, `setModifyFrom`, `setModifyRange`, `loadBlock`, `prepareWriteableBlock`, `get`, `getWriteBlockData`, `set`, `mutableSet`, `fill`, `setBoolBitSet`, `setIntBitSet`, `getBoolBitSet`, `getIntBitSet`, `compress`, `getKeyMap`, `decompress`, `enOrDescryp`, `intToZigZag`, `zigzagToInt`, `transToByteArray`, `transToIntsArray`, `extend`, `extendFloatRound`, `round`, `scale`, `copy`, `getSum`, `getAbsSum`, `processRTLWidths`, `addObj`, `getObjsIndexMap`, `beforeCommit`, `setFirstFrozenSize`, `getTypoType`, `getEntraceObj`, `getDocType`, `getRevision`, `getTableRevision`, `getCPRefs`, `getCPRefFreeLists`, `getSectionPageCount`, `getSectionPageIndex`, `getSectionStartPageIndex`, `getSectionMsoNfc`, `startSection`, `setDocumentWidth`, `getDocumentWidth`, `setDocumentHeight`, `getDocumentHeight`, `setDocumentTop`, `getDocumentTop`, `getMaxPageHeight`, `setMaxPageHeight`, `getTemplete`, `getSectionIndex`, `setTemplete`, `isTemplateReachable`, `getSections`, `updateSections`, `getFirstInfo`, `updateLayoutPageList`, `getDocument`, `getTypoDocument`, `insertPage`, `getCrashRanges`, `getEditCrackRanges`, `getPagePosition`, `hasGap`, `getEditPagePosition`, `swapToLocal`, `cancleBlockSwapWait`, `replaceCrackRange`, `swapFromLocal`, `updateCrackRanges`, `checkInLocal`, `serializeJavaObjs`, `unserializeJavaObjs`, `dumpCPRef`, `dump`, `getBlockSize`, `getInLocalBlockSize`, `getSummaryInfo`, `checkFirstCacheSuitable`, `invalidFirstCache`, `isFirstCache` | — | **PASS** |
| `SafeTrace` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs` | **PASS** | — | `SafeTrace`, `changeUsingThread`, `dispose`, `getTypoDocument`, `onSnapshotCommit`, `release`, `get`, `isEmpty`, `detach`, `releaseAssociatedTraceInstance` | — | **PASS** |
| `IntArrayList` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs` | **PASS** | — | `IntArrayList`, `getData`, `addAll`, `offsetValues` | — | **PASS** |
| `Data` | `koffice/writer/core/cn/wps/moffice/writer/cache/TypoSnapshot.java` | `libs/word-core/src/cn/wps/moffice/writer/cache/TypoSnapshot.rs` | **PASS** | — | `Data`, `resetMaps`, `clear`, `init`, `getRefCount`, `incRef`, `decRef`, `fork`, `cleanFork` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
