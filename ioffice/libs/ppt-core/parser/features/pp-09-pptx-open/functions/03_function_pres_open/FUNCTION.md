# 03_function_pres_open — PptxrPresentation.open

> feature: pp-09-pptx-open ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：打开流程的唯一编排点——PptxrSlides 构造 + `initialize` → openGlobalData → slideCount 两分支回调 → `setInitialized(true)` → `mSlidesReader.open()` → 媒体延载，顺序固定由 Recording 锁定。缺失时断在哪：打开中途无人续盘、slides 与媒体加载不执行，产品闭环断；捷径可用 fixture 绕过。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(KmoPresentation, presPart) -> init + slides.open + media delay

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `KmoPresentation` + pres part（`OpcrPackagePart`，来自 04 `getPresPart`）+ `KmoPresOpenEventListener` + passProvider + dlProvider + `PptxrMedia` + mode。ctor 内另建 `mMedias = new PptxrMedias(mMedia, mPres, 0)`、`mThemes = new PptxrThemes(mMode)`、`mCommentIdCxt = new CommentIdContext(mPres)`。 |
| **输出** | 顺序（Java `open()` 主体）：构造 `PptxrSlides`（传入 `mPresPart.getRelationships()` 等）→ `PptxrSlides.initialize()`（pp-08 静态）→ `openGlobalData()`（02）→ 空白 / 首屏回调 → `setInitialized(true)` → `mSlidesReader.open()`（pp-08：master→layout→slide 顺序）→ `openOtherGlobalData()`（全量停车）→ `mMedia.delayLoadMedias()`（`PptxrMedia.java:147`：delay 集合非空才逐条 `dumpTempFile`）→ `mMedias.priorityReadMedia(mPres)`。 |
| **状态与副作用** | `slideCount()==0`：`new KmoSlide` + `addSlide` + `selection().init` + `hasBlankSlide(true)` + `docDidFinishInitializing(pres, true)`；否则 `docDidFinishInitializing(pres, false)`。`setInitialized(true)` 在 `mSlidesReader.open()` **之前**。 |
| **错误语义** | `throws IOException`；`mSlidesReader.open()` 抛错则中断后续（delayLoadMedias 不执行）。Java 的 open 主体无 try/finally——package close 由 04 的 finally 兜底，本 function 不负责收尾 package。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `PptxrSlides.initialize/open` | pp-08 | slideIdMap、阅读顺序、initializing 回调 | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不重写 slides 内部 |
| `openGlobalData` | 02 | theme + defaults 前置 | 未接线 | 02 | — |
| `PptxrMedia/PptxrMedias` | media 层 | delayLoadMedias、priorityReadMedia | 未接线 | 上游 | 不实现媒体下载 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/pptxr/ppt/PptxrPresentation.java` |
| Rust | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs`（尚不存在，loop 用模板生成） |

关键方法/字段（对照 Java）：

| symbol | 语义（Java） | Rust 目标 |
|---|---|---|
| ctor | 设 mMode/mPres/mPresPart/mOpenListener/mPassProvider/mDlProvider/mMedia；建 mMedias、mThemes、mCommentIdCxt | 字段与构造顺序同 Java |
| `open()` | PptxrSlides 构造 + `PptxrSlides.initialize()` → `openGlobalData()` → slideCount 两分支回调 → `setInitialized(true)` → `mSlidesReader.open()` → `openOtherGlobalData()` → `delayLoadMedias()` → `priorityReadMedia()` | **唯一编排点**；顺序由 Recording 锁定 |
| `openOtherGlobalData()` private | presProps / viewProps（或无 → `buildDefaultViewProperty`）/ commentAuthors / ITEM / ITEMPROPS 五个 rel 分支 + updateProtocol | 全量停车 `#[ignore = "spec: post-MVP"]`，调用点顺序保留 |
| `getPresentationSlideSizeType(KmoPresentation)` static | `slideSize.hasType()` 用之；否则 cx*9==cy*16 → type 15 | 非 open 路径调用，可延后（loop 决定） |

改写列表：`open`（含内部顺序）；`openOtherGlobalData` 全量停车（非必测）。

停车列表：`PptxrPresProps`、`PptxrViewProps`（含 `buildDefaultViewProperty`）、`PptxrCommentAuthors`、
`PptxrItem`、`PptxrItemProps` 全量，矩阵项 `#[ignore = "spec: post-MVP"]`。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读上表 Java 方法体与当前 Rust（若尚无 Rust 则只读 Java）。禁止只按标题冒烟。
审计要点：

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| slides 构造 + initialize | presPart rels、mode | mSlidesReader 就绪 | initialize 静态幂等语义（pp-08） |
| slideCount==0 | 空 slide 清单 | 空白 slide + selection().init + hasBlankSlide(true) + initializing(true) | — |
| slideCount>0 | 非空清单 | initializing(false)；slides.open 执行 | slides.open IOException 冒泡，后续不执行 |
| setInitialized | — | true 在 slides.open 之前 | 顺序归 Recording |
| media | delay 集合空 / 非空 | delayLoadMedias 不动 / 动作 | — |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | slideCount==0 空白 slide + initializing true | 分支 | 是 | T01 |
| C02 | slideCount>0 initializing false 再 slides.open | 正常 | 是 | T02 |
| C03 | setInitialized true 在 slides.open 前 | 状态 | 是 | T03 |
| C04 | open 内部顺序 Recording（initialize→openGlobal→回调→setInitialized→slides.open→media） | 顺序 | 是 | T04 |
| C05 | openOtherGlobalData 全量停车 | ignore | 否 | T05 `#[ignore = "spec: post-MVP"]` |

### 3.3 可测试性

`OpcrRelationships` fake + Recording `KmoPresOpenEventListener`（docDidFinishInitializing 两态）；slides / media
用 pp-08 与媒体层 contract fixture 打桩；Recording 记录调用顺序。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | slideCount==0 | addSlide 1 张空白 slide + selection().init + hasBlankSlide(true) + initializing(true) | 红测停在本 function |
| T02 | C02 | slideCount>0 | initializing(false)；slides.open 被调用 | 红测停在本 function |
| T03 | C03 | 任意一条路径 | setInitialized(true) 先于 slides.open 调用 | 红测停在本 function |
| T04 | C04 | 有 theme + 有效 slides fixture | Recording 顺序与 Java open 主体一致 | 红测停在本 function |
| T05 | C05 | openOtherGlobalData 五个 rel 齐备 | 调用点保留不解析；测试标 ignore | — |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "03_function_pres_open"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_pres_open

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_pres_open \
  --src src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs \
  --report libs/ppt-core/parser/features/pp-09-pptx-open/functions/03_function_pres_open/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_pres_open --json`)
date: 2026-09-01 19:24

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs` | 190/274 | 69.3% | 365/534 | 68.4% |
| **function total** | **190/274** | **69.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/PptxrPresentation.java
```

Loop 结果：**PASS**

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/PptxrPresentation.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs \
  --report libs/ppt-core/parser/features/pp-09-pptx-open/functions/03_function_pres_open/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 11:24

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxrPresentation` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/PptxrPresentation.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs` | **PASS** | `PptxrPresentation`, `open`, `openGlobalData`, `addDefaultProperty`, `openOtherGlobalData` | `getPresentationSlideSizeType` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->