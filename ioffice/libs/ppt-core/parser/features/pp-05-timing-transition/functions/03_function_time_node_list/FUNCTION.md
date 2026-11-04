# 03_function_time_node_list — p:tnLst 收集

> feature: pp-05-timing-transition ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：tnLst 的 seq/par/excl/anim 等已知子节点按 putType 字面量就地累积进 `List<TimeNode>` 并分发子 handler，为 02 的 timing 列表提供元素。缺失时断在哪：时间节点缺失则 `p:timing` 列表为空，但动画不在首屏关键路径，绘制不被阻塞（非关键路径）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

`TimeNodeListHandler` 消费 tnLst 子节点事件，对每个已知子 id `TimeNode.create()` → `putType(字面量)` → add 进调用方传入的 `List<TimeNode>`，并分发给对应子 handler。seq/par/excl 分发目标用 constructor fixture（03 内壳）；Animate\* 行为类移出。列表的最终 write 不在本 handler（Java 无 `endElement` 覆盖），由 `TimingHandler.endElement` 统一 save+write（见 02）。

### 1.1 I/O 契约

| 项 | 契约 |
|---|---|
| **输入** | tnLst 子元素事件：3211323(par) / 3211324(seq) / 3211325(excl) / 3211326(anim) / 3211327(animClr) / 3211328(animEffect) / 3211329(animMotion) / 3211330(animRot) / 3211331(animScale) / 3211332(cmd) / 3211333(set) / 3211334(audio) / 3211335(video) / 3211295(extLst)；`List<TimeNode>` + `HandlerCallback`（ctor 参数，由 02 的 TimingHandler 传入） |
| **输出** | 调用方传入的 `List<TimeNode>` 就地累积：每节点 `TimeNode.create()` + `putType`（字面量 0/1/8/9/10/2/6/11/5/7/3/4/12）+ add；返回子 handler（seq/par/excl 壳、anim 族停车 fixture） |
| **状态与副作用** | 直接修改外部 List（TimingHandler 持有的 `timeNodeList`）；无 `endElement`（不做 save/write，models 由 02 的 endElement 统一处理）；`ExtLstHandler` 内嵌链（3211447 ext→ExtHandler→6619137→DataModelExt）也会往同一 List add |
| **错误语义** | unknown 子 id（3211296–3211322 及未列 id）→ `default` → 返回 null（跳过子树，无副作用）；putType 值直接取 Java 字面量，**不映射** SWReaderMap.timeNodeTypeMap；extLst 分支有「同一 ext 只建一次」守卫（`if (extHandler == null)`） |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `List<TimeNode>` 容器（由 02 TimingHandler 传入） | 02（本 feature 内 loop `01→02→03` 顺位） | Rust `TimeNodeList` 等价容器 | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |
| TimeNode model（`create`/`putType`/`parallelTimeNode`/`sequenceTimeNode`/`exclusive` 等） | data/timing/timenode | Rust 等价接口 | loop 前未接线 | 同上 | 同上 |
| TimeNodeParallelHandler / TimeNodeSequenceHandler / TimeNodeExclusiveHandler | timing（壳） | constructor fixture（本 function 内整理，见 §2.3） | loop 前未接线 | 同上 | 同上 |
| HandlerCallback | pp-01:02 | 透传 | loop 前未接线 | 同上 | 同上 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/handler/timing/TimeNodeListHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/timing/TimeNodeListHandler.rs` |

### 2.1 关键方法/字段表（Java 源码查证）

**`TimeNodeListHandler`**

| 方法 | 元素 id | putType 字面量 | 子 handler 构造 |
|---|---|---|---|
| `TimeNodeListHandler(List<TimeNode>, HandlerCallback)` | — | — | 存 `timeNodeList`、`callback` |
| `enterSubElement(int)` | 3211323（par） | 0 | `TimeNodeParallelHandler(timeNode.parallelTimeNode(), callback)` |
| | 3211324（seq） | 1 | `TimeNodeSequenceHandler(timeNode.sequenceTimeNode(), callback)` |
| | 3211325（excl） | 8 | `TimeNodeExclusiveHandler(timeNode.exclusive(), callback)` |
| | 3211326（anim） | 9 | `AnimateBehaviorHandler(timeNode.animate(), callback)` |
| | 3211327（animClr） | 10 | `AnimateColorBehaviorHandler(timeNode.animateColorBehavior(), callback)` |
| | 3211328（animEffect） | 2 | `AnimateEffectBehaviorHandler(timeNode.animateEffect(), callback)` |
| | 3211329（animMotion） | 6 | `AnimateMotionBehaviorHandler(timeNode.animateMotion(), callback)` |
| | 3211330（animRot） | 11 | `AnimateRotationBehaviorHandler(timeNode.animateRotation(), callback)` |
| | 3211331（animScale） | 5 | `AnimateScaleBehaviorHandler(timeNode.animateScale(), callback)` |
| | 3211332（cmd） | 7 | `CommandBehaviorHandler(timeNode.command(), callback)` |
| | 3211333（set） | 3 | `SetBehaviorHandler(timeNode.setTimeNodeBehavior(), callback)` |
| | 3211334（audio） | 4 | `MediaNodeAudioHandler(timeNode.audio(), callback)` |
| | 3211335（video） | 12 | `MediaNodeVideoHandler(timeNode.video(), callback)` |
| | 3211295（extLst） | — | 内嵌 `ExtLstHandler`（3211447 ext→`ExtHandler`→6619137→`DataModelExt`：内部 3211326 分支同样 create+add+putType(9)+`AnimateBehaviorHandler`） |
| | default（含 3211296–3211322） | — | 返回 null |
| — | 无 `endElement` 覆盖 | — | 列表 save/write 归 02 `TimingHandler.endElement` |

> 注：每个分支均先 `TimeNode timeNode = TimeNode.create(); this.timeNodeList.add(timeNode); timeNode.putType(N);` 再构造子 handler；except 3211335(video)/3211326(anim) 等无 break 的 case 通过末尾 `return handler` 生效。

### 2.2 函数改写列表

- `TimeNodeListHandler`：ctor、enterSubElement（13 个已知子 id 的 create/add/putType/分发 + default null；extLst 内嵌链守卫语义）。

### 2.3 停车列表（非本 function 改写）

- 壳（constructor fixture，03 内整理）：`TimeNodeParallelHandler`（par，type 0）、`TimeNodeSequenceHandler`（seq，type 1）、`TimeNodeExclusiveHandler`（excl，type 8）。
- 移出（post-MVP 动画树）：`AnimateBehaviorHandler`、`AnimateColorBehaviorHandler`、`AnimateEffectBehaviorHandler`、`AnimateMotionBehaviorHandler`、`AnimateRotationBehaviorHandler`、`AnimateScaleBehaviorHandler`、`CommandBehaviorHandler`、`SetBehaviorHandler`、`MediaNodeAudioHandler`、`MediaNodeVideoHandler`（10 个分发目标）+ 支撑类 `CommonBehaviorDataHandler`、`CommonMediaNodeDataHandler`、`CommonTimeNodeDataHandler`、`TimeConditionHandler`、`TimeConditionListHandler`、`TimeTargetElementHandler`、`ShapeTargetElementHandler`、`IteraterDataHandler`、`AnimVariantHandler`、`TimeAnimateValueListHandler`、`TLPointHandler`（11 类），共 21 类。
- 内嵌 `ExtLstHandler`/`ExtHandler`/`DataModelExt` 链：停车（fixture 或 `todo!` 承接），本 MVP 只在 3211295 分支保分发电位。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读：

- `TimeNodeListHandler.java`：13 个已知子 id 的 create/add/putType 字面量（0/1/8/9/10/2/6/11/5/7/3/4/12）与子 handler 构造参数（`parallelTimeNode()`/`sequenceTimeNode()`/`exclusive()`/`animate()`/…）。
- 3211295 extLst 内嵌链的「单例守卫」（`if (extHandler == null)` / `if (dataModelExt == null)`）与 DataModelExt 内部 3211326 的追加语义。
- `TimeNode.java`（只读参照）：`create`/`putType`/`parallelTimeNode`/`sequenceTimeNode`/`exclusive` 等 getter。
- 当前 Rust 同路径（若无 Rust 则只读 Java）。

禁止只按标题冒烟。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 已知 seq/par/excl 子 id（3211324/3211323/3211325） | 分支 | 是 | T01 |
| C02 | unknown（3211296–3211322 及未列 id）→ null | 失败 | 是 | T02 |
| C03 | 0/1/N 个子节点 → List 累积 | 循环 | 是 | T03 |
| C04 | anim 族分发点（3211326–3211335 → 停车 fixture） | 分支 | 否 | — |
| C05 | 3211295 extLst 内嵌链守卫 | 分支 | 否 | — |

### 3.3 可测试性

内存 attrs/events 或假 part/rels。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。seq/par/excl 用 constructor fixture（壳）；List 由测试直接构造传入校验 add/putType。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 3211324(seq)/3211323(par)/3211325(excl) 进入 enterSubElement | 各 TimeNode create+add+putType(1/0/8)，返回对应 fixture handler | 红测停在本 function |
| T02 | C02 | 3211320 等未知 id 进入 | null handler，List 不变 | 红测停在本 function |
| T03 | C03 | 0 / 1 / N（如 3 个）子节点 | List 长度与 Java 一致（0→空；1/N→逐个累积，顺序一致） | 红测停在本 function |
| T04 | C04 | 3211326 anim 进入 | create+add+putType(9)，AnimateBehaviorHandler 位置为停车 fixture | 红测停在本 function |
| T05 | C05 | 两次 3211447 ext 进入 ExtLstHandler | 第二个 ext 断言无新 handler（单例守卫对齐 Java） | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/handler/timing/TimeNodeListHandler.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "03_function_time_node_list"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_time_node_list

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_time_node_list \
  --src src/cn/wps/show/pptxr/handler/timing/TimeNodeListHandler.rs \
  --report libs/ppt-core/parser/features/pp-05-timing-transition/functions/03_function_time_node_list/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_time_node_list --json`)
date: 2026-09-01 22:18

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/timing/TimeNodeListHandler.rs` | 50/219 | 22.8% | 44/234 | 18.8% |
| **function total** | **50/219** | **22.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/timing/TimeNodeListHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/timing/TimeNodeListHandler.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/timing/TimeNodeListHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/timing/TimeNodeListHandler.rs \
  --report libs/ppt-core/parser/features/pp-05-timing-transition/functions/03_function_time_node_list/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 14:18

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TimeNodeListHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/timing/TimeNodeListHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/timing/TimeNodeListHandler.rs` | **PASS** | `TimeNodeListHandler`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->