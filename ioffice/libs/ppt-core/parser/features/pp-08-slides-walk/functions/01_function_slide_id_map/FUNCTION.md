# 01_function_slide_id_map — slideId map + target basename

> feature: pp-08-slides-walk ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：静态 rId target→slideId 反查表就位（`initialize` 复位 + `getSlideIdByFileName` 反查 + rel target→basename 归格），02/03 建壳装配时可按文件名挂接 slideId。缺失时断在哪：打开后无按文件名的 slide 反查依据，产品闭环断；捷径可用 fixture 绕过。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

Java `PptxrSlides` 的三个 **static** 工具：`initialize`（复位静态 `mSlideIdMap`）、
`getSlideIdByFileName`（反查 slideId）、`getRelationShipTargetFileName`（rel target → basename）。
`mSlideIdMap` 的写入入口在 `onSlideIds` / `delayCreateSlides` / `createSlidesByFileName`
（本 feature 02/03），本 function 只锁定「map 复位 + 读 + basename 计算」三步的确定性，
不拥有写入 hook。

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | `initialize()`（无参，静态）；`getSlideIdByFileName(fileName)`，`fileName: String`；`getRelationShipTargetFileName(relationship)`，Java `OpcrRelationship`，可 null（Rust 映射 `Option<&Relationship>`）。 |
| **输出** | `mSlideIdMap` 复位为「空 map」（首次 `new`，非首次 `clear`）；`getSlideIdByFileName`：hit 返回 Java `Long` 拆箱的 slideId，miss 返回 `0L`；`getRelationShipTargetFileName`：`relationship == null → ""`；否则 `target = relationship.getTarget()`，`index = target.lastIndexOf("/")`，当 `index >= 0 && index < target.length()` 时 `substring(index + 1)`（target 以 `/` 结尾 → `""`），无 `/`（`index == -1`）时走 `else index = 0` → `substring(0)` → **整段 target**。 |
| **状态与副作用** | `mSlideIdMap` 是 `static Map<String, Long>`，跨 `PptxrSlides` 实例共享；`PptxrPresentation.open()` 构造 `PptxrSlides` 后立即调用一次 `initialize()`（Java 第 59 行）；map 写入发生在 `onSlideIds`（Java 240–241）、`delayCreateSlides`（196–197）、`createSlidesByFileName`（217–218）：key = `getRelationShipTargetFileName(rels)`，value = `slide.id()`。 |
| **错误语义** | Java 三个方法均无受检异常；null rel → `""`、miss → `0L` 都是正常返回值。Rust 用与 Java 哨兵值一一对应的空值表达，禁止 panic 表达「查不到」，禁止额外抛错误。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| rel 元数据（`OpcrRelationships.getRelationship`、`OpcrRelationship.getTarget`） | pp-06-presentation-xml / pp-07-part-open | fake rels 即能提供 target 字符串（见 §3.3） | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不读 part XML 内容 |
| customShow 反查（`getSlideIdByFileNameNotAdd` miss 时触发 `createSlidesByFileName`） | pp-06 customShow（`PresentationHandlerCallbackImpl.onCustomShow`） | 读 map + 缺省补建 | 见 03 的 `createSlidesByFileName` | customShow 完整接线随 pp-06 延后 | 不在此处添加 map 写入 hook |
| slide.id() 来源 | KmoSlide 模型 | 建壳时分配 id | loop 前未接线 | 随 02 建壳接线 | 不改已 pass 的 pp-01/pp-02 语义 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java` |
| Rust 现状 | 目标 `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs`；当前 `libs/ppt-core/src/cn/wps/show/pptxr/ppt/` 目录尚未生成（无 PptxrSlides.rs） |
| 直接调用方 | `PptxrPresentation.open`（`PptxrSlides.initialize()`）；`PresentationHandlerCallbackImpl.onCustomShow`；`ViewPropertyHandlerCallback`（`preSlide.putId(getSlideIdByFileName(fileName))`） |

关键方法/字段表（方法名与行号以 Java 源码为准）：

| Java symbol | Java 位置 | 语义要点 | Rust 目标 |
|---|---|---|---|
| `public static void initialize()` | 第 54–60 行 | `mSlideIdMap == null → new HashMap()`，否则 `clear()` | 静态 map 复位（`OnceLock`/全局容器，按 crate 现有静态习惯） |
| `public static long getSlideIdByFileName(String fileName)` | 第 62–65 行 | `(Long) mSlideIdMap.get(fileName)`，null → `0L` | `fn get_slide_id_by_file_name(...) -> i64` |
| `public static String getRelationShipTargetFileName(OpcrRelationship relationship)` | 第 76–90 行 | null → `""`；`lastIndexOf("/")` 分支 + `substring` | `fn get_relation_ship_target_file_name(...) -> String` |
| `private static Map<String, Long> mSlideIdMap = null` | 第 46 行 | static，跨实例共享 | 静态容器，与 `initialize` 同归宿 |
| 协作者 `getSlideIdByFileNameNotAdd` / `createSlidesByFileName` | 第 67–72 / 203–224 行 | miss → 按文件名补建 → 重查（写 map 归 03 路径） | 本 function 只提供读接口 |

改写列表：`initialize`、`getSlideIdByFileName`、`getRelationShipTargetFileName`。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读上表 Java 方法体与当前 Rust（若尚无 Rust 则只读 Java）。禁止只按标题冒烟。

| symbol | Java 位置 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `initialize` | 54–60 | 首次（map==null）／已存在 | 新建空 map / `clear()` | null vs clear 二选一 |
| `getSlideIdByFileName` | 62–65 | key 存在 / miss | 原值 / `0L` | miss → 0 |
| `getRelationShipTargetFileName` | 76–90 | null rel；`ppt/slides/slide1.xml`；`slide1.xml`（无 `/`）；`a/b/` | `""` / `slide1.xml` / 整段 `slide1.xml` / `""`（substring 到末尾） | lastIndexOf 分支 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | `initialize` null vs clear（含二次调用幂等） | 状态 | 是 | T01 |
| C02 | `getSlideIdByFileName` miss → 0 | 边界 | 是 | T02 |
| C03 | `getRelationShipTargetFileName` target 有/无 slash；null rel → empty | 边界 | 是 | T03 |

### 3.3 可测试性

内存构造 fake relationship（只提供 target 字符串）；以内存 map 快照校验读写。`initialize` 改全局
静态状态，测试间必须复位。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 首次调 `initialize`（map==null）后 get 任意 key → 0；put 一项后再调 `initialize` → get 同 key → 0（已清空） | 与 Java 一致：可复位且幂等 | 红测停在本 function |
| T02 | C02 | miss：`getSlideIdByFileName("unknown.xml")` | 与 Java 一致：`0` | 红测停在本 function |
| T03 | C03 | null rel → `""`；`getTarget()="ppt/slides/slide1.xml"` → `"slide1.xml"`；`"slide1.xml"`（无 slash）→ 整段 `"slide1.xml"`；`"a/b/"` → `""` | 与 Java 一致：`lastIndexOf`+`substring` 三种分支 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "01_function_slide_id_map"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_slide_id_map

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_slide_id_map \
  --src src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs \
  --report libs/ppt-core/parser/features/pp-08-slides-walk/functions/01_function_slide_id_map/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_slide_id_map --json`)
date: 2026-09-01 15:10

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs` | 55/102 | 53.9% | 81/128 | 63.3% |
| **function total** | **55/102** | **53.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .codex/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs \
  --report libs/ppt-core/parser/features/pp-08-slides-walk/functions/01_function_slide_id_map/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 07:10

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxrSlides` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs` | **PASS** | `initialize`, `getSlideIdByFileName`, `getRelationShipTargetFileName` | `getSlideIdByFileNameNotAdd`, `PptxrSlides`, `open`, `delayCreateSlides`, `createSlidesByFileName`, `onSlideIds`, `onSlideMasterIds`, `onNotesMasterIds`, `onHandoutMasterIds`, `getMasterReader`, `getNotesMasterReader`, `changeLayout`, `shapeNeedCopyFromLayout`, `shapeNeedClearContentText` | — | **PASS** |
| `UnReadSlides` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs` | **PASS** | — | `UnReadSlides`, `resetSlides`, `next`, `getCreateSlideCount` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->