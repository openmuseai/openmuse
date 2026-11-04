# 02_function_on_ids — onSlideIds / onSlideMasterIds

> feature: pp-08-slides-walk ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：presentation.xml 的 rId 列表装配成 KmoSlide 壳与 `mSlidesReader`（master 入 `mMastersReader` + `_addMaster`），并完成 selection init；只建壳不解析，阅读循环的输入容器就绪。缺失时断在哪：打开后无 slide 列表，产品闭环断；捷径可用 fixture 绕过。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

Java 入口 `PresentationHandlerCallbackImpl.onSlideIds`（`mSlidesReader` 非 null 才转发）→
`PptxrSlides.onSlideIds`：把 presentation.xml `p:sldIdLst` 的 rId 列表装配成 KmoSlide 壳与
`mSlidesReader`；`onSlideMasterIds` 把 master rId 装配成 `mMastersReader` 与 `_addMaster`。
本 function 只建壳与容器装配，不解析任何 XML（解析归 pp-07），不进入 open 阅读循环（03）。

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | `onSlideIds(List<String> rids)`：`p:sldIdLst` rId 序列（来自 pp-06 的 `PresentationHandlerCallback`）；`onSlideMasterIds(List<String> rids)`：`p:sldMasterIdLst` rId 序列。两者均通过 `mPresRelses.getRelationship(rId)` 解析，测试侧用 fake rels/parts 模拟。 |
| **输出** | `mSlidesReader`（`List<PptxrSlide>`）添加 ≤ `createSlideCount` 个 slideReader；`mPres.addSlide(KmoSlide)`；`mSlideIdMap.put(getRelationShipTargetFileName(rels), slide.id())`；`mMastersReader`（`Map<String, PptxrMaster>`，key = `rels.getPartName()`）与 `mPres._addMaster(KmoSlideMaster)`；`mPres.slideCount() > 0` 时 `mPres.selection().init()`，且 `getHistorySlideIndex() > 0 && < slideCount` 时 `mPres.selection().selectSlide(historySlideIndex)`。 |
| **状态与副作用** | `slideRids = rids`（供 03 的 `delayCreateSlides` 消费）；`createSlideCount = UnReadSlides.getCreateSlideCount(mPres, rids.size())`：Java 默认 `count = min(10, slideCount)`，active/thumbnail/history 索引落在 `[count, slideCount)` 时把 count 向后推（每步判 `sldIdx + 1 <= slideCount`，最多 +10）；新建 `KmoSlide.setIOReading(true)`；`onSlideMasterIds` 建的 master 也 `setIOReading(true)`。 |
| **错误语义** | 缺 rel（`getRelationship` 返回 null）或 `getPackagePart()` 返回 null 的 rId **不 addSlide、不写 map、不记 `mMastersReader`**，且该 idx 不回溯（实际建壳数 ≤ `createSlideCount`）；无受检异常，静默跳过。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| rId 列表（sldIdLst/sldMasterIdLst 回调） | pp-06-presentation-xml | `PresentationHandlerCallback.onSlideIds/onSlideMasterIds` 回调触发 | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不解析 p:presentation 属性 |
| rels/part（`getRelationship`、`getPackagePart`、`getPartName`） | pp-07-part-open | fake rels/parts | loop 前未接线 | pp-07 接线后换真 rels | 不改已 pass 的 pp-01/pp-02 语义 |
| selection（`init`/`getHistorySlideIndex`/`selectSlide`） | KmoPresentation 模型 | 建壳后初始化 | loop 前未接线 | 随 KmoPresentation 状态接线 | 不伪造 selection 历史 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java` |
| Rust 现状 | 目标 `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs`；当前 `libs/ppt-core/src/cn/wps/show/pptxr/ppt/` 目录尚未生成 |
| 直接调用方 | `PresentationHandlerCallbackImpl.onSlideIds/onSlideMasterIds/onNotesMasterIds/onHandoutMasterIds`（`mSlidesReader` 非 null 才转发）；回调由 `PptxrPresentation.open` 的 presentation.xml 解析触发 |

关键方法/字段表（方法名与行号以 Java 源码为准）：

| Java symbol | Java 位置 | 语义要点 | Rust 目标 |
|---|---|---|---|
| `public void onSlideIds(List<String> rids)` | 第 225–255 行 | 见 §1.1：截断 + 建壳 + map + selection.init | `fn on_slide_ids` |
| `public void onSlideMasterIds(List<String> rids)` | 第 258–271 行 | 逐 rid：rel/part 非 null → `new KmoSlideMaster` + `setIOReading(true)` + `new PptxrMaster` + `mMastersReader.put(rels.getPartName(), masterReader)` + `mPres._addMaster(master)` | `fn on_slide_master_ids` |
| `public void onNotesMasterIds(List<String> ids)` | 第 273–290 行 | part null → `buildDefaultNotesMaster` + `setNoteMaster` | 延后（NotesMaster 全文延后） |
| `public void onHandoutMasterIds(List<String> ids)` | 第 292–303 行 | 同构于 masters（`mHandoutMastersReader`） | 延后（HandoutMaster 全文延后） |
| 字段 `slideRids` / `createSlideCount` | 第 44–45 行 | `onSlideIds` 写入，`open`/`delayCreateSlides`（03）消费 | 结构字段 |
| 字段 `mSlidesReader` / `mMastersReader` | 第 40–41 行 | `List<PptxrSlide>` / `Map<String, PptxrMaster>` | 结构字段 |
| `UnReadSlides.getCreateSlideCount(pres, slideCount)`（static） | 第 546–582 行 | 截断 + selection 索引向后推进；被 `onSlideIds` 第 228 行调用 | 实现归 03，本 function 只调用 |

改写列表：`onSlideIds`、`onSlideMasterIds`（notes/handout 可 ignore post-MVP）。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读上表 Java 方法体与当前 Rust（若尚无 Rust 则只读 Java）。禁止只按标题冒烟。

| symbol | Java 位置 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `onSlideIds` | 225–255 | 有效 rId / 缺 rel / part null | addSlide + map.put；跳过项 | rel/part 双 null 检查；idx 不回溯 |
| `getCreateSlideCount`（调用点 228） | 546–582 | slideCount ≤ 10 或 > 10；active/thumbnail/history 索引高/低 | `count` 原值或向后推进 | min-10；`count <= sldIdx && sldIdx < slideCount` |
| `onSlideIds` 收尾 | 246–254 | `slideCount() > 0` | `selection().init()`；history 边界 selectSlide | `> 0 && < slideCount` |
| `onSlideMasterIds` | 258–271 | 缺 rel / 有效 | 跳过 / put(partName)+`_addMaster` | partName 作 key |
| `onNotesMasterIds`/`onHandoutMasterIds` | 273–303 | — | MVP 不接（延后） | — |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | `onSlideIds` 有效 rId addSlide | 正常 | 是 | T01 |
| C02 | `onSlideIds` 缺 rel 不 add | 失败 | 是 | T02 |
| C03 | `getCreateSlideCount` 截断 | 边界 | 是 | T03 |
| C04 | `onSlideMasterIds` put map + `_addMaster` | 正常 | 是 | T04 |
| C05 | `onSlideIds` 后 `selection.init` 当 slideCount>0 | 状态 | 是 | T05 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels。fake relationship 提供 `getTarget()`/`getPartName()`/`getPackagePart()`
（可返回 None）；Recording `KmoPresOpenEventListener` 与 selection 探针验证收尾。不跑完整
PptxReader（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 3 个全部有效 rId（rids.size()=3 ≤ 10） | 3 次 addSlide、`mSlidesReader`=3、`mSlideIdMap` 3 个 key（basename→slide.id()）、`slideRids=rids` | 红测停在本 function |
| T02 | C02 | 2 个有效 + 1 个缺 rel + 1 个 part null | 只 addSlide 2 次，无 map 写入，createSlideCount 不因跳过而补位 | 红测停在本 function |
| T03 | C03 | slideCount=25、activeIndex=12（12 ∈ [10, 25)）；另测 slideCount=5 ≤ 10 | 前者 count 在 [12, 22] 区间（Java 逐 10 步 +=1 判 `sldIdx+1<=slideCount`）；后者 count=5 | 红测停在本 function |
| T04 | C04 | 2 个有效 master rId + 1 个缺 rel | `mMastersReader` 2 个 key=`partName`、`_addMaster` 2 次、master setIOReading(true) | 红测停在本 function |
| T05 | C05 | 3 个 slide；`getHistorySlideIndex()` 分别取 0 / 越界 / 合法中间值 | `selection.init()` 恰好一次；仅合法值时 `selectSlide(historySlideIndex)` | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "02_function_on_ids"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_on_ids

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_on_ids \
  --src src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs \
  --report libs/ppt-core/parser/features/pp-08-slides-walk/functions/02_function_on_ids/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_on_ids --json`)
date: 2026-09-01 15:16

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs` | 185/241 | 76.8% | 276/336 | 82.1% |
| **function total** | **185/241** | **76.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs \
  --report libs/ppt-core/parser/features/pp-08-slides-walk/functions/02_function_on_ids/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 07:16

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxrSlides` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs` | **PASS** | `initialize`, `getSlideIdByFileName`, `getRelationShipTargetFileName`, `PptxrSlides`, `onSlideIds`, `onSlideMasterIds` | `getSlideIdByFileNameNotAdd`, `open`, `delayCreateSlides`, `createSlidesByFileName`, `onNotesMasterIds`, `onHandoutMasterIds`, `getMasterReader`, `getNotesMasterReader`, `changeLayout`, `shapeNeedCopyFromLayout`, `shapeNeedClearContentText` | — | **PASS** |
| `UnReadSlides` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs` | **PASS** | `getCreateSlideCount` | `UnReadSlides`, `resetSlides`, `next` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->