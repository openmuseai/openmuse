# 02_function_graph_buff — GraphBuff.prepareGraphs 入口

| 项 | 值 |
|----|----|
| feature | wr-29-java-pipeline-cutover |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `GraphBuff`（`prepareGraphs` / `clear` / `addMap` / `getCP`） |
| status | **pass** |
| 阶段 | **layout**（prepareGraphs 控制流；`doPrepareGraphs` 仍由 03 GraphReader 填绳） |

## 1. 功能说明

### 1.1 一句话
把 `GraphBuff` 从自动生成 stub 换成与 Java 同名的完整交错模板：`prepareGraphs` 按 `hasContents` / 列表空满 / `hasReadEnd` 分支走真实控制流并调用抽象 `doPrepareGraphs`；`addMap`/`getCP`/`getGraphIndex`/`clear`/`doClear`/`EqArg` 逻辑与 koffice 一致。

### 1.2 输入
`LayoutState`（`srcList` 空或非空）+ `wordAlignFont`；CP/graph 映射四元组。

### 1.3 输出
- `graph/GraphBuff.rs`、`graph/GraphArrayDeque.rs` 登记进 crate
- `prepareGraphs` 空内容 → `false` 且不调 `doPrepareGraphs`
- 有内容且 graph/chars 空 → `doClear` + `doPrepareGraphs`（02 基类空钩）+ 清 map
- `TextLayouter.mGraphBuff` 类型为真实 `graph::GraphBuff`（去掉文件内影子）

### 1.4 上游
`01_function_graph_nodes`（`SrcNode`/`CharsNode`）；假 `KPList` 仍 PhantomData+`empty`。

### 1.5 下游
`03_function_graph_reader` 覆盖 `doPrepareGraphs` 填 `srcList`/`charsList`/`graphList`。

### 1.6 不变量与函数改写列表

**GraphBuff.rs（本 function 改写列表）**

- 字段：`mMainState` `mDocEnv` `mViewEnv` `mGraphsForEmbed` `mArgsForEmbed` `mMapData` `mMapSize` `mLastReadMapItem` `mRecommendCPRef` `mTypoFontStylePool`
- `GraphBuff(MainState, DocumentEnv, ViewEnv)` `dispose`
- `clear` `doClear` `prepareGraphs`
- `setRecommendCPRef` `getRecommendCPRef`
- `addBidi` `resetForInit`
- `ensureGraphsForEmbed` `addEmbed` `addMath` `addEqation`
- `getArgsForEmbedSize` `getEmbedSize` `getGraphsForEmbed` `getArgsForEmbed` `removeEmbedArgsAt`
- `addMap` `getGraphIndex` `getCP` `removeGraph`
- 嵌套 `EqArg`
- 抽象方法（Java 无方法体）：`setMaxGraphSize` `doCleanGraphReader` `doPrepareGraphs` `setCalcGraphHeight` `setIsInMath` `setIgnoreHpsAndVAlignRun` `setIgnoreCapsAndSmallCaps` `addFunctionSepField` `setIgnoreLeading` `setMathContent` `initFilter` → 空钩（03/04 子类填体）

**GraphArrayDeque.rs（全部在列表上）**

- 字段 `elements` `head` `tail`；`size` `get` `indexOf` `replace` `removeAt` `add` `doubleCapacity` `clear`

**TextLayouter.rs / LayoutState.rs：** 只换 `use` / 去掉本文件 GraphBuff·GraphArrayDeque 影子；假 `KPList.clean`；解 `clearPredictSpaces`/`resetDrawPosOffset`/`resetMathPara`/`clearWitchCache`/`resetGraphsIt` 以便 `clear`/`prepareGraphs` 不 panic。**不**解 `TextLayouter.prepareGraphs` / `flowTextLine`（05）。

**GraphBuffFactory.rs：** 不在本 function 改写列表（Java `new GraphFactory`，04）。

### 1.7 依赖与实现状态（必写，着重）

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `SrcNode`/`CharsNode` | 01 | 类型面 | 是 | 假 KPList 仍不存节点 | 不改编排 |
| `LayoutState.hasContents` | env | `!srcList.isEmpty()` | 是 | — | 不重写 LayoutState 叶 |
| `LayoutState.clearPredictSpaces` 等 | env | `clear`/`prepareGraphs` 调用 | 本 function 解开无依赖体 | `addBidi`/`mGraphsIt` 真迭代器 | 不解 consumeGraphsBefore |
| `KPList.clean` | 假列表 | `empty=true` | 本 function 补 `clean` | 真 KPList 03 | 不引进 util KPList 叶 |
| `TypoPara.alloc/commit/clear/removeGraphs` | cache | `prepareGraphs`/`removeGraph` | 编译面补 Java 签名体 | `TypoSnapshot.alloc` 仍 DEFERRED（GB-D3） | 不改写 TypoSnapshot 叶 |
| `TypoFontStyle.TypoFontStylePool` | cache.style | `reset`/`mSize=0` | 本 function 嵌套池 | obtain/grow 03 | 不改写 cache.style 整叶 |
| `FontStyleFactory.setMaxFontSize` | env | ctor 电话模式 520 | 是 | — | — |
| `GraphReader.doPrepareGraphs` | 03 | 抽象钩 | 空钩 | 整叶 03 | 禁止本 function 改 GraphReader |
| `GraphFactory` / `GraphBuffFactory` | 04 | `createGraphBuff` | stub | `new GraphFactory` | 禁止本 function 假装 GraphFactory |
| `Shape` / `PLC.Node` | drawing/data | `addEmbed`/`addMath` | `Option<()>` 槽 | drawing 闭包 | 不引进 drawing |
| `TextLayouter.initGraphBuff` | wr-22 | `clear`+`initFilter` | 已接线 | `init()` 仍 DEFERRED 故 `mGraphBuff` 默认 None | 不解 flowTextLine |

#### 1.7.1 编号条目

- **GB-D1**：`doPrepareGraphs` Java 在 `GraphReader`。02 基类空钩；有内容路径仍**调用**该钩（与 Java `this.doPrepareGraphs` 一致）。owner：03。
- **GB-D2**：假 `KPList.clean()` 只置 `empty=true`，与 Java `clean` 清空节点等价于当前 PhantomData 列表。owner：03 换真列表。
- **GB-D3**：`typoPara.alloc(snapshot)` Java 调 `snapshot.alloc(5)`。`TypoSnapshot.alloc` 仍 `DEFERRED_NON_FUNCTION`。TypoPara 编译面 `alloc` 置 `mGraphSize=0` 并保留 `// java` 快照语句；不另起第二套 snapshot。owner：cache/TypoSnapshot。
- **GB-D4**：Java `GraphBuff.mMainState` 与 `TextLayouter.mMainState` 同实例。Rust 字段 `Option<*mut MainState>`（dispose→None）。依据：别名/回指。owner：06。
- **GB-D5**：`mArgsForEmbed` Java `GraphArrayDeque<Object>`（Shape / PLC.Node / EqArg）。02 用 `GraphArrayDeque<EmbedArg>` 判别式对应三路 `add*`；`EmbedArg` 不是第二套业务名，是 Object 槽。owner：03。
- **GB-D6**：`GraphBuffFactory.createGraphBuff` → `new GraphFactory`。04 前不实现工厂。测试直接 `GraphBuff(...)`。
- **GB-D7**：`addBidi` 调 `layoutState.addBidi`（仍 DEFERRED）。02 不测 bidi。owner：05。
- **GB-D8**：`CharsNode.srcRef` 01 已是 `Option<*mut SrcNode>`。

## 2. 核心代码

对照 `GraphBuff.java` / `GraphArrayDeque.java` 方法体移植。禁止 dummy `let X = ()`。禁止删 `// java`。抽象方法无 Java 语句 → 空 `fn` + `// rust: Java abstract, no body`。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|--------|----------------|-----------|-------------|---------------|
| ctor phone | GraphBuff.java:32–43 | `ViewMode` 2/3 | `setMaxFontSize(520)` | isPhoneReadMode |
| ctor 非 phone | :38–40 | mode=0 | MAX_VALUE + `setSmartFontSize(false)` | else |
| dispose | :45–49 | 已赋值 env | 三字段 null | — |
| recommend CP | :70–76 | set 7 | get==7 | — |
| addMap 首项 | :207–215 | mMapSize=0 | 四槽写入 size=1 | — |
| addMap 相邻合并 | :216–218 | cpLen==graphLen 且衔接 | 扩展 end，size 不变 | 合并条件 |
| addMap 新项 | :219–232 | 不衔接 | size++ | 扩容 length*2 |
| getGraphIndex | :237–284 | 命中/越界 | 线性+二分 | -1 |
| getCP | :287–338 | size≤0 / 命中 | -1 / cp | — |
| getEmbedSize | :190–192 | deque null | 0 | — |
| addEqation+embed | :180–184 | 一次 | size==1 EqArg | ensure |
| prepareGraphs 空 | :109–110 | srcList empty | false | 不 doClear |
| prepareGraphs 有内容 | :118–136 | empty=false, lists empty | map 被清；调 doPrepareGraphs | doClear false |
| prepareGraphs hasReadEnd | :112–116 | lists nonempty + hasReadEnd | false；map 保留 | resetGraphsIt |
| clear | :53–68 | 脏 flag + map | flags false；srcList empty | doClear true |
| EqArg | :400–411 | 四参 | 字段相等 | — |
| TextLayouter 类型 | TextLayouter.rs | 编译 | 无本地 `pub struct GraphBuff` | 产品类型 |
| 管线 | hello-world.docx | parse | prepareGraphs 空 srcList → false | 不停 render |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|-------------|---------------|----------|-------|----------------------|
| GB1 | ctor 非 phone | 正常主路径 | 是 | GB1 |
| GB2 | ctor phone 520 | 分支 | 是 | GB2 |
| GB3 | dispose 清空 | cleanup | 是 | GB3 |
| GB4 | recommend CP get/set | 正常 | 是 | GB4 |
| GB5 | addMap 首项 | 正常 | 是 | GB5 |
| GB6 | addMap 相邻合并 | 分支 | 是 | GB6 |
| GB7 | addMap 非衔接新项 | 分支 | 是 | GB7 |
| GB8 | addMap 扩容 | 边界 | 是 | GB8 |
| GB9 | getGraphIndex 命中 | 正常 | 是 | GB9 |
| GB10 | getGraphIndex miss | 失败 | 是 | GB10 |
| GB11 | getCP 空 map | 边界 | 是 | GB11 |
| GB12 | getCP 命中 | 正常 | 是 | GB12 |
| GB13 | getEmbedSize null→0 | 边界 | 是 | GB13 |
| GB14 | prepareGraphs 无内容 | 正常 | 是 | GB14 |
| GB15 | prepareGraphs 有内容 | 状态转换 | 是 | GB15 |
| GB16 | hasReadEnd 早退 | 分支 | 是 | GB16 |
| GB17 | clear 复位 | cleanup | 是 | GB17 |
| GB18 | EqArg + addEqation | 正常 | 是 | GB18 |
| GB19 | hello.docx → prepareGraphs false | 管线 layout | 是 | GB19 |
| GB20 | TextLayouter 无 GraphBuff 影子 | 产品接线 | 是 | GB20 |

### 3.3 单元测试 / 管线测试

单元：直接 `GraphBuff::GraphBuff` + `addMap`/`prepareGraphs`，不启动 GraphReader。  
管线：同一 `hello-world.docx` sha256 `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` → `DocumentOpen` 真实 parse → 默认 `LayoutState`（srcList 空）`prepareGraphs` → `false`。停在 **layout prepareGraphs 入口**，不要求 `flowToPage`。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|----|--------|----------|----------|-----------------|----------|
| GB1 | GB1 | ViewMode 0 | ctor | maxFontSize==i32::MAX | 走 phone 臂 |
| GB2 | GB2 | MODE_PHONE=2 | ctor | maxFontSize==520 | 未调 setMaxFontSize |
| GB3 | GB3 | ctor 后 | dispose | mMainState None | 仍持有 |
| GB4 | GB4 | 0 | set 9 / get | 9 | 未写字段 |
| GB5 | GB5 | 空 map | addMap(10,2,0,2) | getCP(0)==10 getCP(1)==11 | 未写 |
| GB6 | GB6 | 已有衔接项 | 再 addMap 相邻 | getGraphIndex 连续；一项 | 拆成两项 |
| GB7 | GB7 | 不衔接 cp | addMap | getCP 两段 | 错误合并 |
| GB8 | GB8 | 填满 16 槽后继续 | 5 项不合并 | 第 5 项可 get | 未扩容 panic |
| GB9 | GB9 | 一项 | getGraphIndex(cp) | graphIndex | 错映射 |
| GB10 | GB10 | 一项 | cp 越界 | -1 | 误命中 |
| GB11 | GB11 | 空 | getCP(0) | -1 | |
| GB12 | GB12 | GB5 | getCP(1) | 11 | |
| GB13 | GB13 | 未 addEmbed | getEmbedSize | 0 | |
| GB14 | GB14 | srcList empty + 已 addMap | prepareGraphs | false；getCP 仍有效 | 误清 map |
| GB15 | GB15 | srcList.empty=false + 已 addMap | prepareGraphs | false；getCP==-1（map 清） | 未走 doClear |
| GB16 | GB16 | lists nonempty + hasReadEnd + addMap | prepareGraphs | false；getCP 仍有效 | 误清 |
| GB17 | GB17 | 脏 flags + map | clear | srcList empty；getCP==-1 | |
| GB18 | GB18 | — | addEqation | embedSize==1 EqArg | |
| GB19 | GB19 | hello-world.docx | parse 后 prepareGraphs | false | parse 失败 |
| GB20 | GB20 | 源码 | 无 `TextLayouter` 内 `pub struct GraphBuff` | 仍影子 |

### 3.5 测试代码位置

crate: `libs/word-core/Cargo.toml`  
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphBuff.rs`  
       `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphArrayDeque.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/graph/GraphBuff.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_graph_buff

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_graph_buff \
  --src src/cn/wps/moffice/writer/layout/base/graph/GraphBuff.rs \
  --src src/cn/wps/moffice/writer/layout/base/graph/GraphArrayDeque.rs \
  --report libs/word-core/layout/features/wr-29-java-pipeline-cutover/functions/02_function_graph_buff/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_graph_buff --json`)
date: 2026-10-02 15:19

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/graph/GraphArrayDeque.rs` | 26/169 | 15.4% | 37/236 | 15.7% |
| `src/cn/wps/moffice/writer/layout/base/graph/GraphBuff.rs` | 247/452 | 54.6% | 326/602 | 54.2% |
| **function total** | **273/621** | **44.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphBuff.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/GraphBuff.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphArrayDeque.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/GraphArrayDeque.java
```

### 3.9 Java↔Rust 门面对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/GraphBuff.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphBuff.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/GraphArrayDeque.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphArrayDeque.rs \
  --report libs/word-core/layout/features/wr-29-java-pipeline-cutover/functions/02_function_graph_buff/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-10-02 07:20

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `GraphBuff` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/GraphBuff.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphBuff.rs` | **PASS** | `GraphBuff`, `dispose`, `setMaxGraphSize`, `clear`, `setRecommendCPRef`, `getRecommendCPRef`, `prepareGraphs`, `setCalcGraphHeight`, `setIsInMath`, `setIgnoreHpsAndVAlignRun`, `setIgnoreCapsAndSmallCaps`, `resetForInit`, `getArgsForEmbedSize`, `getEmbedSize`, `getGraphsForEmbed`, `getArgsForEmbed`, `removeEmbedArgsAt`, `getCP`, `addFunctionSepField`, `setIgnoreLeading`, `setMathContent`, `initFilter` | — | — | **PASS** |
| `EqArg` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/GraphBuff.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphBuff.rs` | **PASS** | `EqArg` | — | — | **PASS** |
| `GraphArrayDeque` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/graph/GraphArrayDeque.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/graph/GraphArrayDeque.rs` | **PASS** | `size`, `get`, `indexOf`, `replace`, `removeAt`, `add`, `clear` | — | — | **PASS** |

**overall: PASS** — basename aligned; every Java class has layout PASS and a Rust counterpart for each public method.
<!-- java-rust-align:end -->

### 3.10 Java↔Rust 管线对比（逻辑 + 产品接线）

| 步 | Java | Rust |
|----|------|------|
| 1 | `new GraphBuff(main, doc, view)` | `GraphBuff::GraphBuff` |
| 2 | `layoutState.hasContents()` | 同名；假 KPList `empty` |
| 3 | `graphBuff.prepareGraphs(wordAlignFont, layoutState)` | 同名方法，真实分支 |
| 4 | `doPrepareGraphs` | 基类空钩（GB-D1） |
| 5 | `addMap` / `getCP` | 同名 |

产品：`TextLayouter.mGraphBuff` 为 `graph::GraphBuff`；`initGraphBuff` 调真实 `clear`/`initFilter`。`GraphBuffFactory.createGraphBuff` 仍待 04。

### 3.11 管线阶段对齐报告

| 项 | 内容 |
|----|------|
| 阶段（parse / layout / render） | layout（prepareGraphs 入口） |
| 原始文档（同一份，sha256） | `tests/fixtures/p1_layout_document/hello-world.docx` `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e` |
| 步骤对照（Java 类.方法 ↔ Rust 类.方法 / 方案名） | 见 §3.10 |
| 产品接线（谁调用本阶段门面） | `TextLayouter.initGraphBuff` → `GraphBuff.clear`/`initFilter` |
| 关键路径真实实现（无 stub / 无另起实现） | **pass**（`prepareGraphs`/`clear`/`addMap`/`getCP` 无 DEFERRED；`doPrepareGraphs` 为 Java 抽象空钩 GB-D1） |
| 管线测试 test_id | GB19 |
| 测试结果符合预期 | **pass**（20/20 `cargo test --test 02_function_graph_buff`） |
| 判定 | **pass** |
