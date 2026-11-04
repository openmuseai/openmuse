# 02_function_pres_open_global — openGlobalData

> feature: pp-09-pptx-open ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：pres part 的全局数据前置处理——theme rel → `mThemes.open` + `setTheme`、presentation.xml 事件 + `load()`、缺省 sldSize/notesSize 经 `addDefaultProperty` 补齐。缺失时断在哪：打开后无 theme/全局尺寸上下文，产品闭环断；捷径可用 fixture 补齐 sldSz。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(pres part rels+stream) -> theme + presentation xml + default sizes

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `mPresPart.getRelationships()`（pres part rels：按 `OxfPptRelations.theme().getRelsType()` 找 theme rel、`tableStyles().getRelsType()` 找 tableStyles rel）+ `mPresPart.getInputStream()`（presentation.xml stream）；dTarget drawingStore / mediaLib / inkLib / fontLib / tmpPath 经 `PresentationHandlerCallbackImpl` 传入。 |
| **输出** | theme rel 非 null → `mThemes.open(mPres, themeRels, mMedias)`（pp-04）→ `mPres.setTheme(kmoTheme)`；`PresentationHandler` 消费 presentation.xml（pp-06）+ `handlerCallback.load()`（pp-07）；`addDefaultProperty`：`!hasSlideSize()` → `SlideSize{12192000, 6858000}`（DEFAULT_SLDSIZE_CX/CY）、`!hasNotesSize()` → `PositiveSize2D{6858000, 9144000}`（DEFAULT_NODESIZE_CX/CY）；末尾 `mPres.updateProtocol(presData.save())`。 |
| **状态与副作用** | 无 theme rel → **不** setTheme（theme 保持默认；Java 无 else 分支）。theme 先于 presentation.xml 解析（PIPELINE.md §4：styles 先于正文）。tableStyles rel 非 null 且 part 非 null 才开 `PptxrTableStyles(...).open()`（本 feature fixture skip，业务体后置）。addDefaultProperty 对 sldSz / notesSz **独立判定**（其一缺失只写其一）。 |
| **错误语义** | `throws IOException` 沿 `mPresPart.getInputStream()` / `UnifiedXmlReader.read`（pp-07）向上冒泡给 03；theme part 缺失不是错误；tableStyles rel 非 null 但 `getPackagePart()==null` 时跳过不开。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `PptxrThemes.open` → `KmoTheme` | pp-04/02、03 | `setTheme`；theme partName 缓存 | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |
| `PresentationHandler` + callback.load | pp-06/01；pp-07/02 | presentation.xml 事件面 + load | 未接线 | 上游 | 本 function 只管调用顺序，不实现 handler |
| `PptxrTableStyles` | PIPELINE.md §3 移出项 | 有 rel 时 fixture skip | 不做 | post-MVP | 不 fake 业务体 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/pptxr/ppt/PptxrPresentation.java` |
| Rust | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs`（尚不存在，loop 用模板生成） |

关键方法/字段（对照 Java）：

| symbol | 语义（Java） | Rust 目标 |
|---|---|---|
| `openGlobalData()` private throws IOException | rels.getRelationshipByType(theme) 非 null → `mThemes.open` + `mPres.setTheme`；tableStyles rel + part 非 null → `PptxrTableStyles(...).open()`（停车）；presentation.xml stream → `PresentationHandler` + `UnifiedXmlReader.read` + `handlerCallback.load()`；最后 `addDefaultProperty()` | 顺序保持：theme → tableStyles(skip) → presentation.xml → defaults |
| `addDefaultProperty()` private | `!hasSlideSize()` → SlideSize cx=12192000 cy=6858000 → writeSlideSize；`!hasNotesSize()` → PositiveSize2D cx=6858000 cy=9144000 → writeNotesSize；`mPres.updateProtocol(presData.save())` | 两个独立判定 |
| 常量 | `DEFAULT_SLDSIZE_CX=12192000`、`DEFAULT_SLDSIZE_CY=6858000`、`DEFAULT_NODESIZE_CX=6858000`、`DEFAULT_NODESIZE_CY=9144000` | const 同值 |

改写列表：`openGlobalData`、`addDefaultProperty`。

停车列表：`PptxrTableStyles` 全量（有 rel → fixture skip；业务体 post-MVP）；`PresentationHandlerCallbackImpl`
构造细节（pp-07 接线，本 function 只保顺序）。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读上表 Java 方法体与当前 Rust（若尚无 Rust 则只读 Java）。禁止只按标题冒烟。
审计要点：

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| theme rel | 有 / 无 | setTheme 有 / 无 | 无 theme rel 不 setTheme（C01） |
| tableStyles rel | 有 rel+part / 有 rel 无 part / 无 rel | skip 或 fixture | part==null 跳过 |
| presentation.xml stream | 有效 XML（内存 fixture） | handler 事件 + load 调用 | IOException 冒泡给 03 |
| addDefaultProperty | sldSz / notesSz 有 / 无 独立四态 | SlideSize / PositiveSize2D 默认写入 | 有则不写、无则写 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 无 theme rel | 边界 | 是 | T01 |
| C02 | 有 theme 则 setTheme | 正常 | 是 | T02 |
| C03 | 缺 sldSz/notesSz 写默认 | 状态 | 是 | T03 |
| C04 | tableStyles rel skip/fixture | 分支 | 是 | T04 |
| C05 | 打开顺序：theme → presentation handler → defaults | 顺序 | 是 | T05 |

### 3.3 可测试性

Fake `OpcrRelationships.getRelationshipByType` 返回有 / 无 rel；theme 依赖 pp-04 contract fixture
（`mThemes.open` 可打桩返回 KmoTheme）；presentation.xml 经内存 stream 喂 `PresentationHandler`（pp-06 契约），
Recording 记录 `setTheme → handler.load → addDefaultProperty(updateProtocol)` 顺序。不跑完整 PptxReader。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | rels 无 theme 类型 | 不调用 setTheme；后续 presentation.xml 处理不受影响 | 红测停在本 function |
| T02 | C02 | rels 有 theme rel | 调用 setTheme（theme 来自 pp-04 fixture） | 红测停在本 function |
| T03 | C03 | 无 sldSz 且无 notesSz；反向只缺其一 | 写 SlideSize(12192000, 6858000) + PositiveSize2D(6858000, 9144000)，且独立判定 | 红测停在本 function |
| T04 | C04 | 有 tableStyles rel 且 part 非 null；rel 有但 part==null；无 rel | fixture skip 不炸；skip 语义与 Java 一致 | 红测停在本 function |
| T05 | C05 | 有 theme + 有效 presentation.xml | Recording 命中 theme→handler→defaults 顺序 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "02_function_pres_open_global"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_pres_open_global

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_pres_open_global \
  --src src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs \
  --report libs/ppt-core/parser/features/pp-09-pptx-open/functions/02_function_pres_open_global/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_pres_open_global --json`)
date: 2026-09-01 19:22

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs` | 177/238 | 74.4% | 320/448 | 71.4% |
| **function total** | **177/238** | **74.4%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/PptxrPresentation.java
```

Loop 结果：（未跑）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/PptxrPresentation.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs \
  --report libs/ppt-core/parser/features/pp-09-pptx-open/functions/02_function_pres_open_global/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 11:22

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxrPresentation` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/PptxrPresentation.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs` | **PASS** | `PptxrPresentation`, `open`, `openGlobalData`, `addDefaultProperty` | `openOtherGlobalData`, `getPresentationSlideSizeType` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->