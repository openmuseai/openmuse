# 03_function_pptx_slide_open — PptxrSlide.open

> feature: pp-07-part-open ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：slide part 的流经 01/02 填成 KmoSlide（SlideHandler → protocol/film），随后 `callback.load()` + `updateFieldText()` 收尾，单张真实 slide 可开。themeOverride rel 可挂成 slide 的继承 theme。缺失时断在哪：真实 pptx 的 slide 打不开，产品闭环断；捷径可用 fixture 绕过。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(KmoSlide, part) -> XML+load；不 open notes/chart/vml

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | 构造 `(KmoSlide, KmoPresentation, OpcrPackagePart part, PptxrMedias, PptxrThemes, PptxrSlides, int mode, CommentIdContext)`；`open()` 消费 `mPart`。 |
| **输出** | XML 填 slide（`SlideHandler` 事件 → `mSlide` protocol/film）+ `callback.load()` + `mSlide.updateFieldText()`；`getLayoutRels()` 等 rel 查询可复用。 |
| **状态与副作用** | themeOverride rel → `mThemes.open(...)` → `mSlide.setThemeOverride(theme)`；`load()` 与 `updateFieldText()` 只在 XML try 成功之后执行（catch 分支直接 return）。 |
| **错误语义** | `mPart == null` → `open()` 空转返回（不抛）；`getRelationships() == null` → 返回；XML Throwable → `IOLog.d` + return，**不抛给调用方**。 |

### 1.2 open 流程（查证自 `PptxrSlide.java`，逐行）

```text
1. mPart == null            → return（空转）
2. relationships == null    → return
3. theme：mThemes 非空 → themeOverrideRels != null → mThemes.open(..., themeOverrideRels, ...)
                             → mSlide.setThemeOverride(theme)
         mThemes 为空       → theme = mPresentation.selection().activedSlide().getThemeOverride()
                             （依赖 pp-06/pp-09 selection，stub）
4. try:  mPart.getInputStream()
         → new SlideHandlerCallbackImpl(drawingStore, mediaLib, inkLib, relationships, medias, slides, mode)
         → new SlideHandler(mSlide, drawingStore, callback)
         → UnifiedXmlReader.read(stream, handler)
   catch Throwable          → IOLog.d; return
5. slideHandlerCallback.load()                 （必在 XML 之后）
6. mSlide.updateFieldText()
7.（移出）notes / comment / chart / vml / diagram
```

注意：`updateFieldText()` 在 Java 中位于 XML+load 之后、notes 之前；
notes/comment/chart/vml/diagram 五块**不在本 feature 承接范围**（FUNCTIONS.md §0 post-MVP）。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `SlideHandler` | pp-01:05（已 pass） | p:sld 事件 → Slide protocol | 是 | — | 不改已 pass 的 pp-01/pp-02 语义 |
| `UnifiedXmlReader.read` | 01（本 feature） | part stream → handler 事件 | 未接线 | — | — |
| `HandlerCallbackImpl` | 02（本 feature） | load()/onBlipEmbed 等 | 未接线 | — | — |
| `PptxrThemes.open` | pp-04 | themeOverride rel → KmoTheme | spec | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不内联 theme 解析 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlide.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlide.rs`（目录/文件尚未创建） |

### 关键方法 / 字段

| Java | Rust 目标 | 契约要点 |
|---|---|---|
| 构造 `PptxrSlide(KmoSlide, KmoPresentation, OpcrPackagePart, PptxrMedias, PptxrThemes, PptxrSlides, int, CommentIdContext)` | `PptxrSlide::new` | 只存引用不解析；字段 `mSlide/mPart/mPresentation/mMedias/mThemes/mSlides/mMode/mCommentIdCxt` |
| `open()` | `open(&mut self)` | §1.2 流程 1–6 |
| `getRels(OpcRelation)` | 私有 helper | mPart null → null；relsCol null → null；遍历 `relsType.endsWith(type)` |
| `getLayoutRels()` | 同 | `getRels(OxfPptRelations.slideLayout())` — 保留（下游可消费） |
| `getNotesSlideRels()` / `getCommentRels()` / `getVmlDrawingRels()` | 停车 | 随 notes/comment/vml part 移出（post-MVP） |
| `getSlide()` | 同 | `mSlide` 访问器 |

### 改写列表

`open()` 主线至 `callback.load()` + `mSlide.updateFieldText()`；`getRels`/`getLayoutRels`。
theme 的 else 分支（`mPresentation.selection().activedSlide().getThemeOverride()`）→ **stub**。

### 停车列表

- notes（`PptxrNotesSlide`）/ comment（`PptxrComment`）/ chart（`PptxrChart`）/ vml（`PptxrVmlDrawing`）/ diagram（`PptxrDiagramParts`）整块移出，含对应 rel 查询
- themeOverride else 分支（stub，依赖 pp-06/pp-09 selection）

## 3. 测试方案

### 3.1 代码审计与测试面

- Java 侧：`PptxrSlide.java` 全部（构造、open 的 theme 分支、try/catch 边界、load/updateFieldText 位置、getRels 判定）。
- 当前 Rust：`src/cn/wps/show/pptxr/ppt/slides/` 整树未创建。
- fixture：最小 XML `p:sld` 根 + fake part（`getInputStream`/`getRelationships` 记录调用）。SlideHandler 已 pass（pp-01:05），可直接接管事件。

禁止只按标题冒烟：null part、relationships null、XML Throwable 三条早退路径与 themeOverride 分支必须独立覆盖。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | null part 早退 | 边界 | 是 | T01 |
| C02 | XML+load 成功 | 正常 | 是 | T02 |
| C03 | XML Throwable 吞掉 | 失败 | 是 | T03 |
| C04 | themeOverride rel | 分支 | 是 | T04 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels：fake part 可抛 IOException 模拟 C03；Recording handler 断言事件序（`callback.load()` 在 XML 之后）。断言 C04 时 themeOverride 走 fake `PptxrThemes` 记录。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 最小内存输入覆盖 `null part 早退`：part=None | `open()` 直接返回，无任何 XML/load 副作用 | 红测停在本 function |
| T02 | C02 | 最小内存输入覆盖 `XML+load 成功`：`<p:sld/>` + fake part | XML 填 slide；`load()` 恰一次；`updateFieldText()` 被调 | 红测停在本 function |
| T03 | C03 | 最小内存输入覆盖 `XML Throwable 吞掉`：fake part 抛 IOException | IOLog 等价记录；`load()`/`updateFieldText()` 不执行；不抛给调用方 | 红测停在本 function |
| T04 | C04 | 最小内存输入覆盖 `themeOverride rel`：part 带 themeOverride rel | `mThemes.open` 恰一次；`mSlide.setThemeOverride` 落值 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/ppt/slides/PptxrSlide.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "03_function_pptx_slide_open"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_pptx_slide_open

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_pptx_slide_open \
  --src src/cn/wps/show/pptxr/ppt/slides/PptxrSlide.rs \
  --report libs/ppt-core/parser/features/pp-07-part-open/functions/03_function_pptx_slide_open/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_pptx_slide_open --json`)
date: 2026-09-01 18:34

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/ppt/slides/PptxrSlide.rs` | 97/188 | 51.6% | 183/317 | 57.7% |
| **function total** | **97/188** | **51.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlide.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlide.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlide.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlide.rs \
  --report libs/ppt-core/parser/features/pp-07-part-open/functions/03_function_pptx_slide_open/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 10:34

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxrSlide` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlide.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlide.rs` | **PASS** | `PptxrSlide`, `open`, `getRels`, `getLayoutRels`, `getSlide` | `getNotesSlideRels`, `getCommentRels`, `getVmlDrawingRels` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->