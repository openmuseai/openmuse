# 03_function_unread_walk — PptxrSlides.open 顺序

> feature: pp-08-slides-walk ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：`PptxrSlides.open()` 主循环按 master→layout→slide 顺序逐张 open，首屏 `delayCreateSlides` 补建；`UnReadSlides.next` 定阅读顺序、ForceQuit 可中断。缺失时断在哪：打开后 slide 不进读取循环、无阅读顺序，产品闭环断；捷径可用 fixture 绕过。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

`PptxrSlides.open()` 的主循环：由 `UnReadSlides` 决定下一张待读 slide，按
master→layout→slide 顺序打开，首屏补建延迟 slide（`delayCreateSlides`），支持
`mDlProvider.isOpenForceQuit()` 中断。`UnReadSlides` 是 `PptxrSlides` 的**静态内嵌类**
（Java 第 407 行，非独立文件），负责 `next`/`resetSlides`/`getCreateSlideCount`。
本 function 不解析 XML 内容（part XML 由 pp-07 的 `PptxrSlide.open` 处理）。

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | `open()`（无参实例方法，状态取 02 之后）：`mSlidesReader.size()` 构造 `UnReadSlides`；`mDlProvider`（ForceQuit 判据）、`mOpenListener`、`mMedias`、`mPres`；`getMasterReader(PptxrSlide)` 需要 slide 的 `getLayoutRels()` → layout part 的 `getRelationships()` → 其中 `rels.getType()` 以 `OxfPptRelations.slideMaster().getRelsType()` 结尾的 rel 的 `partName`。 |
| **输出** | 每张 slide：`masterReader.open()` → `masterReader.openLayout(slideReader.getLayoutRels())` → `slideReader.getSlide().setLayout(layout)`；随后 `slideReader.open()` → `slide.setIOReading(false)` → `mOpenListener.slideDidFinishReadingEverything(slideIndex)`；全部 master 再 `open()` + `openAllDatas()`；空 slide 列表 → `changeLayout(mPres.slideAt(0), 默认 layout)` + `slideDidFinishReadingEverything(0)`（默认 layout：无 master 时 `buildDefaultSlideMaster.layoutAt(0)`）。 |
| **状态与副作用** | 阅读顺序由 `UnReadSlides.next(pres)` 决定：priorityList 首个未读 → active 索引的 `sldIdx, +1, -1, +2, -2, +3, -3` → thumbnail 的 `sldIdx, +1, -1` → history 的 `sldIdx, +1, -1` → 从 `mTop` 顺序扫描，命中即 `mUnReadSlides[i]=false`。`bFirstSlide` 分支只执行一次：`delayCreateSlides()`（从 `createSlideCount` 顺序补建剩余 rId，缺 rel/part 的跳过、不 `++`）→ `slides.resetSlides(mSlidesReader.size())` → `mPres.eventListeners().onSlideListChanged()`；首张后若 `OPLogDumper_FirstSlideIOFinished` 为 true 直接 `return`；正常路径每张后 `mMedias.priorityReadMedia(mPres)`。 |
| **错误语义** | `mDlProvider.isOpenForceQuit()` 为 true 时在 `slideReader.open()` **之前**抛 `ForceQuitException`（中断剩余循环）；`getMasterReader` 返回 null（slide 无 layoutRels / layout part 无 relationships / 无 slideMaster 类型 rel）→ 走默认 master 路径（`buildDefaultSlideMaster` + `buildDefaultLayout` + `layout.setMaster(default)`，`indexOfMaster==-1` 才 `_addMaster`）；`next` 无未读返回 -1，正常结束循环。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 建壳容器（`mSlidesReader`/`mMastersReader`/`slideRids`/`createSlideCount`） | 02（本 feature） | open 循环的输入状态 | 已调用 `onSlideIds`/`onSlideMasterIds` | notes/handout 仍停车 | 不回填 02 的装配语义 |
| `PptxrSlide/Layout/Master.open` 与 part 读取 | pp-07-part-open | open 循环调用的 part open | 本 function 用事件探针记录调用顺序 | pp-07 接线后调真实现 | 不解析 part XML |
| selection（priorityList/activedSlideIndex/thumbnailSlideIndex/getHistorySlideIndex） | KmoPresentation 模型 | `UnReadSlides.next` 的分值来源 | `Selection` 字段可测 | — | 不伪造 selection 状态 |
| 不改已 pass 的 pp-01/pp-02 语义 | — | — | — | — | ph/形状解析不在此改写 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java`（`open` 第 104–183 行；`UnReadSlides` 第 406–582 行） |
| Rust 现状 | 目标 `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs`；当前 `libs/ppt-core/src/cn/wps/show/pptxr/ppt/` 目录尚未生成 |
| 直接调用方 | `PptxrPresentation.open` 装载完成后由 pp-09 编排调用 `mSlidesReader.open()` |

关键方法/字段表（方法名与行号以 Java 源码为准）：

| Java symbol | Java 位置 | 语义要点 | Rust 目标 |
|---|---|---|---|
| `public void open()` | 第 104–183 行 | 主循环 + scheme 后 master `openAllDatas` + 空列表 `changeLayout` + notes/handout 末尾打开 | `fn open` |
| `private void delayCreateSlides()` | 第 184–199 行 | 从 `createSlideCount` 顺序补建：rel/part null 跳过；`++createSlideCount` 后 addSlide + map.put | `fn delay_create_slides` |
| `private void createSlidesByFileName(String neededFileName)` | 第 203–224 行 | 按文件名补建，命中后 `break`（供 `getSlideIdByFileNameNotAdd`） | `fn create_slides_by_file_name` |
| `private PptxrMaster getMasterReader(PptxrSlide slideReader)` | 第 307–330 行 | layoutRels null → null；layout part rels 中 type endsWith slideMaster 类型 → `mMastersReader.get(partName)` | `fn get_master_reader` |
| `private PptxrNotesMaster getNotesMasterReader(PptxrNotesSlide)` | 第 332–335 行 | `notesSlideReader.getNotesMasterRels()` → map 查 | 延后（NotesMaster 全文延后） |
| 内嵌 `public static class UnReadSlides` | 第 407–582 行 | ctor 全 true；`next` 分值顺序；`resetSlides` 扩容补 true；static `getCreateSlideCount` | `struct UnReadSlides` 对应方法 |
| 字段 `mDlProvider`/`mOpenListener`/`mSlidesReader` | 第 37–43 行 | ForceQuit 判据 / 完成事件 / 读者容器 | 结构字段 |

改写列表：`open`、`UnReadSlides`、`getMasterReader`、`delayCreateSlides`。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读上表 Java 方法体与当前 Rust（若尚无 Rust 则只读 Java）。禁止只按标题冒烟。

| symbol | Java 位置 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `open` 主循环 | 106–149 | N 张 slide + M 个 master | master.open→openLayout→setLayout / default 分支；slide.open→setIOReading(false)→slideDidFinishReadingEverything | bFirstSlide 单次；OPLogDumper 短路；priorityReadMedia |
| `open` 收尾 | 146–179 | 全部打开后 | 各 master `openAllDatas`；空列表 → `changeLayout`+事件；notes/handout 末尾 | `slideCount()==0` |
| `getMasterReader` | 307–330 | layoutRels null / layoutRelses null / 无 slideMaster rel / 命中 | null 或 `mMastersReader.get(partName)` | `endsWith` 类型匹配 |
| `delayCreateSlides` | 184–199 | 补建区 rel/part null | 跳过不 `++`；否则建 slide+map.put | 顺序补建 |
| `UnReadSlides.next` | 437–543 | priorityList / active / thumbnail / history / 顺序扫描 | 首个未读索引返回并标 false | 每候选区间检查；无未读 → -1 |
| `UnReadSlides.resetSlides` | 420–435 | slideCount > 长度 | 扩容保留原值、新位 true | 仅当需要扩容 |
| `UnReadSlides.getCreateSlideCount` | 546–582 | 截断与推进（与 02 共用） | count 原值或后推 | `count <= sldIdx && sldIdx < slideCount` |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | `open` master.open 先于 slide.open | 状态 | 是 | T01 |
| C02 | `getMasterReader` 无 master → default layout | 分支 | 是 | T02 |
| C03 | 首屏 `delayCreateSlides` | 状态 | 是 | T03 |
| C04 | `mDlProvider.isOpenForceQuit` → 中断 | 失败 | 是 | T04 |
| C05 | `UnReadSlides.next` priorityList 决定下一张 | 分支 | 是 | T05 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels。用 Recording `KmoPresOpenEventListener` 记录
`slideDidFinishReadingEverything` 的索引顺序校验 master→layout→slide；假 `IDLProvider`
返回 ForceQuit 开关；`UnReadSlides` 用构造大小 + selection 探针直接单测。
`OPLogDumper_FirstSlideIOFinished` 短路可单测一次即 return（FUNCTIONS.md §3）。不跑完整
PptxReader（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 2 slides + 1 master（layoutRels 命中 slideMaster） | 事件顺序：每张先 master.open 后 slide.open；`openLayout(layoutRels)` → `slide.setLayout` | 红测停在本 function |
| T02 | C02 | slide 无 layoutRels，或 layout part 无 slideMaster rel | 走 `buildDefaultSlideMaster` + `buildDefaultLayout` + `layout.setMaster`；`indexOfMaster==-1` 才 `_addMaster` | 红测停在本 function |
| T03 | C03 | 首张打开后 `slideRids.size()` 大于 `createSlideCount` | 仅首屏一次：`delayCreateSlides` 补建剩余（缺 rel 跳过）+ `resetSlides(size)` + `onSlideListChanged` | 红测停在本 function |
| T04 | C04 | `mDlProvider.isOpenForceQuit()==true`（在 slide open 前） | `ForceQuitException` 语义：中断循环，后续 `slideDidFinishReadingEverything` 不再触发 | 红测停在本 function |
| T05 | C05 | priorityList=[4,1] 且 4 未读；空 priority 时 active 分值序列 | 先返回 4；空 priority 时按 `sldIdx, +1, -1, +2, -2, +3, -3` 首次未读命中 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "03_function_unread_walk"
```

测试入口：`libs/ppt-core/tests/cn/wps/show/pptxr/ppt/slides/PptxrSlides_unread_walk.rs`（`[[test]] name = "03_function_unread_walk"`）。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_unread_walk

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_unread_walk \
  --src src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs \
  --report libs/ppt-core/parser/features/pp-08-slides-walk/functions/03_function_unread_walk/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_unread_walk --json`)
date: 2026-09-01 16:17

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs` | 416/573 | 72.6% | 656/878 | 74.7% |
| **function total** | **416/573** | **72.6%** |  |  |
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
  --report libs/ppt-core/parser/features/pp-08-slides-walk/functions/03_function_unread_walk/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 08:16

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxrSlides` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs` | **PASS** | `initialize`, `getSlideIdByFileName`, `getRelationShipTargetFileName`, `PptxrSlides`, `open`, `delayCreateSlides`, `onSlideIds`, `onSlideMasterIds`, `getMasterReader` | `getSlideIdByFileNameNotAdd`, `createSlidesByFileName`, `onNotesMasterIds`, `onHandoutMasterIds`, `getNotesMasterReader`, `changeLayout`, `shapeNeedCopyFromLayout`, `shapeNeedClearContentText` | — | **PASS** |
| `UnReadSlides` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs` | **PASS** | `UnReadSlides`, `resetSlides`, `next`, `getCreateSlideCount` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->