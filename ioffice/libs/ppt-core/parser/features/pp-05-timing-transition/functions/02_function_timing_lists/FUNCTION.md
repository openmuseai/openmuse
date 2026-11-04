# 02_function_timing_lists — p:timing 列表容器

> feature: pp-05-timing-transition ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：`p:timing` 的时间节点列表（tnLst）与触发列表（bldLst）在 `endElement` 统一 save+write 进 `Timing` model，动画时间线数据落地。缺失时断在哪：timing 数据不解析则动画无从谈起，但时间线不在首屏绘制范围内，绘制不被阻塞（非关键路径）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

`TimingHandler` 消费 `p:timing` 元素，按子元素 3211422(tnLst)/3211437(bldLst) 实例化 `TimeNodeListHandler` / `BuildListHandler`，列表在调用方持有的 `List` 中就地累积；`TimingHandler.endElement` 为副作用点：逐项 `save()` 后 `timing.timeNodeLst().write(...)` / `timing.buildLst().write(...)`（Java 用 null 守卫判断是否写了列表）。

### 1.1 I/O 契约

| 项 | 契约 |
|---|---|
| **输入** | `p:timing` 子元素事件：3211422（tnLst）→ `TimeNodeListHandler(List<TimeNode>, callback)`、3211437（bldLst）→ `BuildListHandler(List<Build>, callback)`；`Timing` model + `HandlerCallback`（ctor 参数） |
| **输出** | `Timing.timeNodeLst().write(timeNodeList)`（tnLst 存在时）、`Timing.buildLst().write(buildList)`（bldLst 存在时）；每个 `TimeNode`/`Build` 先 `save()` |
| **状态与副作用** | `timeNodeList`/`buildList` 是 `TimingHandler` 字段，在 `enterSubElement` 分支内 `new ArrayList()` 后交给子 handler 就地累积；`endElement` 是唯一副作用点（write model）；Java 无 `startElement` 覆盖 |
| **错误语义** | 无子元素（两个 list 均为 null）→ `endElement` 双 null 守卫直接跳过，**不 write null 列表**（验收 §4-3）；未知子元素 id → `default: return null`（跳过子树）；`BuildListHandler` 分支内 build 项细节由停车 handler 承接（见 §2.3） |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `TimeNodeListHandler`（3211422 分发目标） | 03（本 feature 内 loop `01→02→03` 顺位） | Rust `TimeNodeListHandler` ctor | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |
| TimeNode/Build model（`save()`、`timeNodeLst()/buildLst()` 的 List write） | data/timing | Rust 等价 save/write 接口 | loop 前未接线 | 同上 | 同上 |
| HandlerCallback | pp-01:02 | 透传给子 handler | loop 前未接线 | 同上 | 同上 |
| pp-01 05/06/07 分发点（3211609） | pp-01 | `handler::timing` 模块路径（Rust `SlideHandler.rs` 等已 `use`，模块未建） | loop 前未接线 | 同上 | 同上 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/handler/timing/TimingHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/timing/TimingHandler.rs` |
| `koffice/presentation/core/cn/wps/show/pptxr/handler/timing/BuildListHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/timing/BuildListHandler.rs` |

### 2.1 关键方法/字段表（Java 源码查证）

**`TimingHandler`**

| 方法 | 元素 id | 行为 |
|---|---|---|
| `TimingHandler(Timing, HandlerCallback)` | — | 存 `timing`、`callback`；`timeNodeList`/`buildList` 字段初始 null |
| `enterSubElement(int)` | 3211422（tnLst） | `timeNodeList = new ArrayList()`；`handler = new TimeNodeListHandler(timeNodeList, callback)` |
| | 3211437（bldLst） | `buildList = new ArrayList()`；`handler = new BuildListHandler(buildList, callback)` |
| `endElement(int)` | — | `if (timeNodeList != null)`：每个 `TimeNode.save()`，`timing.timeNodeLst().write(timeNodeList)`；`if (buildList != null)`：每个 `Build.save()`，`timing.buildLst().write(buildList)` |

**`BuildListHandler`（容器层）**

| 方法 | 元素 id | 行为 |
|---|---|---|
| `BuildListHandler(List<Build>, HandlerCallback)` | — | 存 `buildList`、`callback` |
| `enterSubElement(int)` | 3211433（bldP） | `Build.create()` → putType(3) → add → `BuildParagraphHandler(build.buildParagraph())` |
| | 3211434（bldDgm） | `Build.create()` → putType(0) → add → `BuildDiagramHandler(build.buildDiagram())` |
| | 3211435（bldChart） | `Build.create()` → putType(2) → add → `OleChartBuildHandler(build.buildEmbeddedChart())` |
| | 3211436（bldGraphic） | `Build.create()` → putType(1) → add → `GraphicalObjectBuildHandler(build.buildGraphics())` |
| | default | 返回 null |

### 2.2 函数改写列表

- `TimingHandler`：ctor、enterSubElement（3211422/3211437）、endElement（双 null 守卫 + save + write）。
- `BuildListHandler` 容器层：ctor、enterSubElement（3211433/3211434/3211435/3211436 的 Build 创建/putType/add/分发）。

### 2.3 停车列表（非本 function 改写）

- `BuildListHandler` 内嵌 build 项细节解析 9 类：`BuildParagraphHandler`（+`TemplateListHandler`+`TemplateHanlder`，bldP attrs/tmplLst）、`BuildDiagramHandler`、`OleChartBuildHandler`、`GraphicalObjectBuildHandler`（+`AnimationGraphicalObjectBuildPropertiesHandler`+`AnimationDgmBuildPropertiesHandler`+`AnimationChartBuildPropertiesHandler`）——MVP 只保列表容器，子 build 项以 fixture/`todo!` 承接（涉及 paraBuildMap/diagramBuildMap/oleChartBuildMap/animationDgmBuildMap/animationChartBuildMap、`callback.getCoreShapeId` 等，均不在本 loop）。
- 动画行为树 21 类（`Animate*BehaviorHandler` 等，见 FUNCTIONS.md §0）：本 function 不引入。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读：

- `TimingHandler.java`：enterSubElement 两分支、endElement 的 null 守卫与 save/write 顺序（先逐项 save 后整表 write）。
- `BuildListHandler.java` 容器层：3211433–3211436 的 putType 字面量（3/0/2/1）与分发构造参数（buildParagraph/buildDiagram/buildEmbeddedChart/buildGraphics）。
- `Timing.java`（只读参照）：`timeNodeLst()`/`buildLst()` 是 `TimeNodeList`/`BuildList` 读写接口。
- 当前 Rust 同路径（若无 Rust 则只读 Java）。

禁止只按标题冒烟。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 无子元素 end：timeNodeList/buildList 均为 null | 边界 | 是 | T01 |
| C02 | 3211422 → TimeNodeListHandler（list 新建 + 资源转移） | 分支 | 是 | T02 |
| C03 | 3211437 → BuildListHandler（含 putType 3/0/2/1 的 4 个子分发） | 分支 | 是 | T03 |
| C04 | endElement 写 timing 列表（有列表：save 顺序 + write 内容） | 状态 | 是 | T04 |
| C05 | 未知子元素 id → null handler | 失败 | 是 | T05 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。`TimeNodeListHandler`/`BuildListHandler` 的列表由测试直接传入（03 未就绪时用真容器 + 空列表验证 write 路径）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | p:timing 空元素 → endElement | 两个 list 均 null，不 write（验收 §4-3） | 红测停在本 function |
| T02 | C02 | 3211422 进入 enterSubElement | 新建 List 并分发给 TimeNodeListHandler（列表地址一致，可往内累积） | 红测停在本 function |
| T03 | C03 | 3211433/3211434/3211435/3211436 逐个进入 | Build putType 3/0/2/1 + 对应子 handler 分发 | 红测停在本 function |
| T04 | C04 | 有 tnLst 内容（预填 TimeNode 列表）的 endElement | 先逐项 save() 再 timeNodeLst().write；bldLst 分支同构 | 红测停在本 function |
| T05 | C05 | 未知子元素 id（如 3211420） | null handler，子树跳过，无副作用 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/handler/timing/TimingHandler.rs
       src/cn/wps/show/pptxr/handler/timing/BuildListHandler.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "02_function_timing_lists"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_timing_lists

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_timing_lists \
  --src src/cn/wps/show/pptxr/handler/timing/TimingHandler.rs \
  --src src/cn/wps/show/pptxr/handler/timing/BuildListHandler.rs \
  --report libs/ppt-core/parser/features/pp-05-timing-transition/functions/02_function_timing_lists/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_timing_lists --json`)
date: 2026-09-01 22:17

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/timing/BuildListHandler.rs` | 64/180 | 35.6% | 76/215 | 35.3% |
| `src/cn/wps/show/pptxr/handler/timing/TimingHandler.rs` | 40/54 | 74.1% | 56/77 | 72.7% |
| **function total** | **104/234** | **44.4%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/timing/TimingHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/timing/TimingHandler.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/timing/BuildListHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/timing/BuildListHandler.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/timing/TimingHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/timing/TimingHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/timing/BuildListHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/timing/BuildListHandler.rs \
  --report libs/ppt-core/parser/features/pp-05-timing-transition/functions/02_function_timing_lists/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 14:18

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TimingHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/timing/TimingHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/timing/TimingHandler.rs` | **PASS** | `TimingHandler`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `BuildListHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/timing/BuildListHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/timing/BuildListHandler.rs` | **PASS** | `BuildListHandler`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->