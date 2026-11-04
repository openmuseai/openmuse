# 01_function_compress_env — LineCompressEnv 可压空白记账

| 项 | 值 |
|----|----|
| feature | wr-25-line-compress |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `LineCompressEnv::{addSpaceCompressableSpace,addPunctCompressableSpace,addOtherCompressableSpace,consumeCompressableSpaces,getLeftedCompressableSpaceSum}` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
按 graph 登记可压空白（space/punct/other），consume 后给出 used/lefted 与 before/after。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| addSpaceCompressableSpace | graphIdx, space | LineCompressEnv.java:78–84 |
| addPunctCompressableSpace | graphIdx, before, after, punct | :86–96 |
| addOtherCompressableSpace | graphIdx, other | :98–104 |
| consumeCompressableSpaces | compressGraphIdx, needSpace | :143–176 |
| restoreCompressState | lastGraphIdx | :106–141 |

### 1.3 输出
`getLeftedCompressableSpaceSum` / `getUsedCompressableSpaceSum` / `getCompressBefore` / `getCompressAfter` / `isFirstCompress` / `getFirstCompressedGraphIndex` / `getFinalCompress`。

### 1.4 上游
无。不跑 LineLayouter / TextLayouter。

### 1.5 下游
`02_function_compress` 读同一 env。

### 1.6 不变量与函数改写列表
- Java `Integer.MIN_VALUE` 作为 firstExcess* 复位值。
- 三类可压空间按 space→punct→other 顺序 consume。
- `needSpace<=0` 仍记录 compressed graph，used 三槽写 0。
- **改写列表（本文件全部方法）：** `reset,dispose,doAddCompressableSpace,addSpaceCompressableSpace,addPunctCompressableSpace,addOtherCompressableSpace,restoreCompressState,consumeCompressableSpaces,getUsedCompressableSpaceSum,getLeftedCompressableSpaceSum,isFirstCompress,getFirstCompressedGraphIndex,getUsedSpaceCompressableSpaceSum,getUsedPunctCompressableSpaceSum,getUsedOtherCompressableSpaceSum,getMaxSpaceCompressableSpaceSum,getMaxPunctCompressableSpaceSum,getMaxOtherCompressableSpaceSum,getCompressBefore,doAddCompressBefore,getCompressAfter,doAddCompressAfter,setFinalCompress,getFinalCompress`
- 列表外方法：无（本类即本 function I/O）。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓（保留 Java 注释+签名） | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `WIntArrayList` | kfc util | `new/resetQuick/binarySearch/insert/set/get/add/isEmpty/setQuick/size` | 是 | — | 禁止改 kfc |
| `LineCompressTool` | wr-25 02 | 无 | 否 | 本 function 不生成 | 禁止改 Tool |
| `LineLayouter` | wr-12 | 无 | j2r_ty 壳 | 不接入 | 禁止改 |

## 2. 核心代码
`koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineCompressEnv.java`  
Rust：`libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineCompressEnv.rs`  
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `reset` | LineCompressEnv.java:25–43 | 任意脏状态 | firstExcess=MIN；sums=0；lists empty | 复位后 isFirstCompress |
| `addSpaceCompressableSpace` | :78–84 | space=0 / ≠0 | space=0 不写 after；仍 doAdd | 0 vs N |
| `addPunctCompressableSpace` | :86–96 | before/after 0/非0 | before/after 索引表 | 两路 if |
| `addOtherCompressableSpace` | :98–104 | other=0 / ≠0 | after + other 桶 | 0 vs N |
| `doAddCompressableSpace` | :55–76 | 同 idx 二次 / 新 idx | set vs insert×3 | binarySearch >=0 |
| `consumeCompressableSpaces` | :143–176 | needSpace<=0 / 跨三类 | used 累加；compressed 记录 | 三 if 耗尽 |
| `restoreCompressState` | :106–141 | 无记录 / 命中 idx | used+max 回退 | usedIdx<0 |
| `getCompressBefore/After` | :220–245 | 未知 idx | 0 | miss |
| `isFirstCompress` / `getFirstCompressedGraphIndex` | :188–194 | 未 consume / 已 consume | true+MIN / false+first idx | |
| `setFinalCompress` / `getFinalCompress` | :264–269 | 任意 int | 回读 | |
| `dispose` | :45–53 | after reset | lists null/None | cleanup |
| `getUsed*` / `getMax*` / `getLefted*` | :178–218 | after add+consume | 算术和 | |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| CE-reset | reset | 状态转换 | 是 | CE1 |
| CE-space-zero | addSpace space=0 | 边界 | 是 | CE2 |
| CE-space-n | addSpace space>0 | 公开入口 | 是 | CE3 |
| CE-punct | addPunct before+after | 公开入口 | 是 | CE4 |
| CE-other | addOther 0 与 N | 边界 | 是 | CE5 |
| CE-dup-idx | 同 graph 二次 add | 关键分支 | 是 | CE6 |
| CE-consume-zero | needSpace<=0 | 失败/边界 | 是 | CE7 |
| CE-consume-cross | needSpace 跨 space/punct/other | 关键分支 | 是 | CE8 |
| CE-restore-empty | restore 无 compressed | 失败 | 是 | CE9 |
| CE-restore-hit | restore 命中 | 状态转换 | 是 | CE10 |
| CE-before-miss | getCompressBefore 未知 | 失败 | 是 | CE11 |
| CE-first | isFirstCompress + first idx | 状态转换 | 是 | CE12 |
| CE-final | set/getFinalCompress | 公开入口 | 是 | CE13 |
| CE-dispose | dispose | cleanup | 是 | CE14 |
| CE-lefted | getLefted after consume | 公开入口 | 是 | CE15 |

### 3.3 可测试性
直接 `LineCompressEnv` 默认构造 + `reset()`；不构造 LayoutState / Graphs。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| CE1 | CE-reset | add 后脏 | `reset()` | firstExcess=i32::MIN；lefted=0；isFirst=true | 残留 used |
| CE2 | CE-space-zero | reset | addSpace(3,0) | after(3)=0；maxSpace+=0 | 误写 after |
| CE3 | CE-space-n | reset | addSpace(3,10) | after(3)=10；lefted=10 | 未记账 |
| CE4 | CE-punct | reset | addPunct(1,4,5,9) | before=4 after=5 lefted=9 | 漏 before |
| CE5 | CE-other | reset | addOther(2,0) 再 addOther(2,7) | after(2)=7 lefted=7 | 0 误写 after |
| CE6 | CE-dup-idx | 已 addSpace(1,4) | 再 addSpace(1,6) | after(1)=10；一条 index | insert 重复 |
| CE7 | CE-consume-zero | addSpace(0,8) | consume(0,0) | isFirst=false；used=0；lefted=8 | 当耗尽 |
| CE8 | CE-consume-cross | space10+punct10+other10 | consume(5,25) | used=25 lefted=5；usedSpace=10 usedPunct=10 usedOther=5 | 顺序错 |
| CE9 | CE-restore-empty | 仅 add 未 consume | restore(99) | used=0 | 读越界 panic |
| CE10 | CE-restore-hit | consume 两档 | restore 第一档 idx | used 回到第一档 | 不回退 |
| CE11 | CE-before-miss | reset | getBefore(99) | 0 | 非0 |
| CE12 | CE-first | consume 后 | getFirstCompressedGraphIndex | 传入的 graphIdx | MIN |
| CE13 | CE-final | reset | setFinal(42) | get=42 | 0 |
| CE14 | CE-dispose | add 后 | dispose | 再 getAfter 不 panic（0 或 None 路径） | use-after-free |
| CE15 | CE-lefted | CE8 后 | getLefted | 5 | 用全 crate 覆盖率冒充 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineCompressEnv.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/line/LineCompressEnv.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_compress_env

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_compress_env \
  --src src/cn/wps/moffice/writer/layout/base/line/LineCompressEnv.rs \
  --report libs/word-core/layout/features/wr-25-line-compress/functions/01_function_compress_env/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_compress_env --json`)
date: 2026-09-05 17:39

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/line/LineCompressEnv.rs` | 267/294 | 90.8% | 396/432 | 91.7% |
| **function total** | **267/294** | **90.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineCompressEnv.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineCompressEnv.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineCompressEnv.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineCompressEnv.rs \
  --report libs/word-core/layout/features/wr-25-line-compress/functions/01_function_compress_env/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-05 09:39

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `LineCompressEnv` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineCompressEnv.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineCompressEnv.rs` | **PASS** | `reset`, `dispose`, `doAddCompressableSpace`, `addSpaceCompressableSpace`, `addPunctCompressableSpace`, `addOtherCompressableSpace`, `restoreCompressState`, `consumeCompressableSpaces`, `getUsedCompressableSpaceSum`, `getLeftedCompressableSpaceSum`, `isFirstCompress`, `getFirstCompressedGraphIndex`, `getUsedSpaceCompressableSpaceSum`, `getUsedPunctCompressableSpaceSum`, `getUsedOtherCompressableSpaceSum`, `getMaxSpaceCompressableSpaceSum`, `getMaxPunctCompressableSpaceSum`, `getMaxOtherCompressableSpaceSum`, `getCompressBefore`, `doAddCompressBefore`, `getCompressAfter`, `doAddCompressAfter`, `setFinalCompress`, `getFinalCompress` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
