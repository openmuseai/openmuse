# 02_function_id_lists — sldIdLst / sldMasterIdLst

> feature: pp-06-presentation-xml ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：sldIdLst/sldMasterIdLst 等列表在 `endElement` 一次性回调 rId 列表（`onSlideIds`/`onSlideMasterIds` 等），slide/master/notes 的 part 引用装配有了输入。缺失时断在哪：rId 列表缺失则 pp-08 slides-walk 拿不到阅读顺序与 part 引用，pp-08/09 打开链断（闭环打开必要）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(list item elements, rId 4259841) -> onSlideIds / onSlideMasterIds

### 1.1 I/O 契约

| 方向 | 契约 |
|---|---|
| 输入 | 列表元素：sldMasterIdLst(3211509)/sldIdLst(3211512) → `SlideAndMasterIdListHandler`；notesMasterIdLst(3211510)/handoutMasterIdLst(3211511) → `NotesAndHandoutMasterIdListHandler`。item 子元素（id 均按 `kfc/xsd/pml/P.java` 查证）：sldId(3211473)/sldMasterId(3211474)/notesMasterId(3211475)/handoutMasterId(3211476)，取 `r:id` = `R.id`(4259841)（`kfc/xsd/shared/R.java`） |
| 输出 | 列表 `endElement` 一次性回调（幂等于收集的 rId 数量，保 XML 顺序）：3211512 → `onSlideIds(List<String>)`；3211509 → `onSlideMasterIds`；3211510 → `onNotesMasterIds`；3211511 → `onHandoutMasterIds`（方法签名按 `PresentationHandlerCallback.java`） |
| 状态与副作用 | handler 实例 ctor 建 `ArrayList<String>`；`startElement` 累积；`enterSubElement` 一律 `return this`（item 事件仍由同一实例处理）；回调发生在列表 `endElement` 而非每个 item；父级 `enterSubElement` 每进一个列表新建实例（fresh rids） |
| 错误语义 | Slide/Master：`getByName(4259841) != null` 才 add，缺 `r:id` 静默跳过该 item；notes/handout：`getByName(4259841).toString()` **无判空** —— 缺 `r:id` 即 Java NPE（FUNCTIONS.md §4 验收点 3：测试必须用合法 attrs，不发明更安全的 Rust）。空列表（0 item）→ 空 List 回调，无 NPE |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 见 FUNCTIONS.md §6 | | | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs`（尚不存在） |

### 2.1 Java nested handler → Rust 目标（按 Java 源码查证）

| Java nested（PresentationHandler.java） | 说明 | Rust 目标 |
|---|---|---|
| `SlideAndMasterIdListHandler` | 处理 sldIdLst(3211512)/sldMasterIdLst(3211509)；字段 `List<String> rids` | `PresentationHandler.rs` 内 nested；rids → `Vec<String>` |
| `NotesAndHandoutMasterIdListHandler` | 处理 notesMasterIdLst(3211510)/handoutMasterIdLst(3211511)；字段 `List<String> idList` | 同上；idList → `Vec<String>` |

关键方法（均已在源码中逐行查证）：

| Java 方法 | 行为 | 备注 |
|---|---|---|
| `SlideAndMasterIdListHandler.startElement` | 仅 `element == 3211473 \|\| element == 3211474` 时取 `getByName(4259841)`，`!= null` 才 `rids.add(toStr)` | item 判空，缺 r:id 跳过 |
| `enterSubElement` | `return this` | 任何子元素继续自食 |
| `endElement` | `element == 3211512` → `onSlideIds(rids)`；`element == 3211509` → `onSlideMasterIds(rids)` | 3211512=sldIdLst，3211509=sldMasterIdLst |
| `NotesAndHandoutMasterIdListHandler.startElement` | `element == 3211476 \|\| 3211475` 时 `idList.add(getByName(4259841).toString())` | **无判空**，Java NPE 复刻点 |
| `NotesAndHandoutMasterIdListHandler.endElement` | `element == 3211511` → `onHandoutMasterIds`；`element == 3211510` → `onNotesMasterIds` | 3211511=handoutMasterIdLst，3211510=notesMasterIdLst |
| 回调四方法 | `onSlideIds`/`onSlideMasterIds`/`onNotesMasterIds`/`onHandoutMasterIds(List<String>)` | `PresentationHandlerCallback.java` 空实现，pp-08 覆写 |

改写列表：两个 nested handler 的全部方法（ctor/startElement/enterSubElement/endElement）。
元素 id 按 `kfc/xsd/pml/P.java`：sldIdLst=3211512、sldMasterIdLst=3211509、notesMasterIdLst=3211510、
handoutMasterIdLst=3211511、sldId=3211473、sldMasterId=3211474、notesMasterId=3211475、
handoutMasterId=3211476；`r:id`=4259841 按 `kfc/xsd/shared/R.java`。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读上表 Java 方法体与当前 Rust（若尚无 Rust 则只读 Java）。审计面：

1. item 元素 id：sldId(3211473)/sldMasterId(3211474) 与 notesMasterId(3211475)/handoutMasterId(3211476)
   的收集分支；handler 共用实例的 `enterSubElement` 自食语义。
2. 回调时机：`endElement` 按**列表** id（3211509/3211512/3211510/3211511）分派，不是按 item。
3. 判空差异：Slide/Master `!= null` 才 add；notes/handout 无判空（NPE 复刻点）——测试夹具只喂合法 attrs。
4. 回调参数保序：`ArrayList.add` 顺序 = XML 顺序。
5. 父级 `enterSubElement` 每进一个列表 new 一个 handler 实例（`case 3211509/3211512` 与
   `case 3211510/3211511` 各自 new），同文档四列表并存互不串扰。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | sldIdLst 0/1/N 个 `r:id`；`endElement` 才回调，保序 | 状态 | 是 | T01 |
| C02 | sldMasterIdLst → `onSlideMasterIds`（含 N 个 r:id） | 分支 | 是 | T02 |
| C03 | `enterSubElement` 返回 this（item 不新建 handler） | 交互 | 是 | T03 |
| C04 | notesMasterIdLst/handoutMasterIdLst → `onNotesMasterIds`/`onHandoutMasterIds`（合法 r:id） | 分支 | 是 | T04 |
| C05 | sldId 缺 `r:id` 的 item 被静默跳过（Slide/Master 判空路径）；notes/handout 不判空（NPE 语义留档） | 边界 | 是 | T05 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。
Recording `PresentationHandlerCallback`：回调各方法只把收到的 `List<String>` 记录到测试侧，断言
幂等、顺序与时机。attrs 用内存 `XmlRoAttr`（`getByName("r:id")` 对应 4259841）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | sldIdLst 带 0 / 1 / N 个 `p:sldId[@r:id]`；enter 序列 + 列表 end | 仅列表 `endElement` 时 `onSlideIds` 收到对应长度且保序的 `List<String>`；item end 不回调 | 红测停在本 function |
| T02 | C02 | sldMasterIdLst 带 N 个 `p:sldMasterId[@r:id]` | 列表 end 时 `onSlideMasterIds` 收到 N 个 rId | 红测停在本 function |
| T03 | C03 | 列表内任意子元素 enter | 返回同一 handler（`this`），事件不丢 | 红测停在本 function |
| T04 | C04 | notesMasterIdLst + handoutMasterIdLst 各带合法 `r:id` | 各自在列表 end 回调 `onNotesMasterIds`/`onHandoutMasterIds`，参数正确 | 红测停在本 function |
| T05 | C05 | sldIdLst 含 1 个无 `r:id` 的 item + 1 个有 `r:id` 的 item | 无 `r:id` item 被跳过，回调只剩有 `r:id` 的项 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/handler/PresentationHandler.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "02_function_id_lists"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_id_lists

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_id_lists \
  --src src/cn/wps/show/pptxr/handler/PresentationHandler.rs \
  --report libs/ppt-core/parser/features/pp-06-presentation-xml/functions/02_function_id_lists/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_id_lists --json`)
date: 2026-09-01 17:33

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/PresentationHandler.rs` | 81/367 | 22.1% | 107/457 | 23.4% |
| **function total** | **81/367** | **22.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java
```

Loop 结果：**PASS**

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs \
  --report libs/ppt-core/parser/features/pp-06-presentation-xml/functions/02_function_id_lists/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 09:33

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PresentationHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs` | **PASS** | `PresentationHandler`, `startElement`, `endElement`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
