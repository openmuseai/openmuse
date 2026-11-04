# 04_function_pptx_layout_open — PptxrLayout.open

> feature: pp-07-part-open ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：layout part 的流填成 KmoSlideLayout（bOpen 幂等门、themeOverride 挂载、`load()` 在 try 内 + `setIOReading(false)` 收尾），slide 的版式底稿可开。缺失时断在哪：真实 pptx 的 layout 打不开、slide 版式继承断，产品闭环断；捷径可用 fixture 绕过。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(KmoSlideLayout, part) -> bOpen + XML

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | 构造 `(KmoPresentation, KmoSlideLayout, OpcrPackagePart part, PptxrMedias, PptxrThemes, PptxrSlides, int mode)`；`open()` 消费 `mPart`。 |
| **输出** | `bOpen` 幂等门；XML 填 layout（`SlideLayoutHandler` 事件 → `mLayout` protocol/film）；`callback.load()`（在 XML try **内部**）；`mLayout.setIOReading(false)` 收尾。 |
| **状态与副作用** | `bOpen` **先置位**：rels Throwable 早退后第二次 `open()` 仍直接跳过（幂等）；themeOverride rel → `mThemes.open(...)` → `mLayout.film().setTheme(theme)`。 |
| **错误语义** | rels 获取 Throwable → `IOLog.d` + return（bOpen 已为 true）；XML Throwable → `IOLog.d` 吞掉**继续**走 `setIOReading(false)`（不 return，与 slide 不同）；静态 `buildDefaultLayout` 资源缺失 → `FileNotFoundException` 上抛。 |

### 1.2 open 流程（查证自 `PptxrLayout.java`，逐行）

```text
1. !bOpen ──> bOpen = true                 （先置位）
2. mPart != null：
   a. try  relses = mPart.getRelationships()
      catch Throwable ──> IOLog.d; return（bOpen 已 true）
   b. themeOverrideRels != null ──> themeOverride = mThemes.open(...)
                                 ──> mLayout.film().setTheme(themeOverride)
   c. try  mPart.getInputStream()
           → new SlideHandlerCallbackImpl(drawingStore, mediaLib, inkLib, mPart.getRelationships(), medias, slides, mode)
           → new SlideLayoutHandler(mLayout, drawingStore, callback)
           → UnifiedXmlReader.read(stream, handler)
           → slideHandlerCallback.load()          （load 在 try 内）
      catch Throwable ──> IOLog.d（继续）
   d.（移出）chart / vml / diagram
   e. mLayout.setIOReading(false)
```

注意：layout 的 XML catch 不 return（与 slide 的 return 语义不同），且 `load()` 在 try 内、
XML 失败时不执行；`mLayout.setIOReading(false)` 无论成败都会执行。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `SlideLayoutHandler` | pp-01:06（已 pass） | p:sldLayout 事件 → layout protocol | 是 | — | 不改已 pass 的 pp-01/pp-02 语义 |
| `UnifiedXmlReader.read` | 01（本 feature） | part stream → handler 事件 | 未接线 | — | — |
| `HandlerCallbackImpl` | 02（本 feature） | load()/onBlipEmbed 等 | 未接线 | — | — |
| `PptxrThemes.open` | pp-04 | themeOverride rel → KmoTheme | spec | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不内联 theme 解析 |
| `PptResourceFactory.getSlideLayout` | 资源层（Rust 侧**待 loop 确认**） | 默认 layout 资源流 | 未接线 | — | 不在本 function 内造资源 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/ppt/slidelayouts/PptxrLayout.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slidelayouts/PptxrLayout.rs`（目录/文件尚未创建） |

### 关键方法 / 字段

| Java | Rust 目标 | 契约要点 |
|---|---|---|
| 构造 `PptxrLayout(KmoPresentation, KmoSlideLayout, OpcrPackagePart, PptxrMedias, PptxrThemes, PptxrSlides, int)` | `PptxrLayout::new` | 字段 `mPresentation/mLayout/mPart/mMedias/mThemes/mSlides/mMode` + `bOpen=false` |
| `open()` | `open(&mut self)` | §1.2 流程 1–5 |
| 静态 `buildDefaultLayout(KmoPresentation, PptxrMedias, int)` | 静态 | `getPresentationSlideSizeType` → `PptResourceFactory.getSlideLayout(type)`（null → FileNotFoundException）；`new KmoSlideLayout` + `setIOReading(true)`；`SlideHandlerCallbackImpl(null rels, null slides, mode)`；`SlideLayoutHandler`；`read`（finally close）+ `load()` + `setIOReading(false)` |
| 静态 `open(KmoSlideLayout, String xmlpath, PptxrMedias)` / `open(..., InputStream, ...)` | 延后 | 无 part 场景（handler-only，defer） |
| `getVmlDrawingRels()` / `getRels(OpcRelation)` | 停车 | vml 移出 |
| `getLayout()` | 同 | `mLayout` 访问器 |

### 改写列表

`open()` 主线（bOpen/rels/themeOverride/XML+load/setIOReading(false)）；静态 `buildDefaultLayout`（Java 有 → 列入，
其中 `PptResourceFactory` 流供应**待 loop 确认**）；静态两个 `open` overload `defer` 不写。

### 停车列表

- chart（`PptxrChart`）/ vml（`PptxrVmlDrawing`）/ diagram（`PptxrDiagramParts`）整块 + `getVmlDrawingRels`/`getRels`
- 静态 `open(layout, xmlpath/xmlstream, medias)`（延后）

## 3. 测试方案

### 3.1 代码审计与测试面

- Java 侧：`PptxrLayout.java` 全部（open 的 bOpen 置位点、rels try/catch 早退、XML catch 不 return、themeOverride 分支、buildDefaultLayout 资源流 finally close、两个静态 open overload）。
- 当前 Rust：`src/cn/wps/show/pptxr/ppt/slidelayouts/` 整树未创建。
- fixture：最小 XML `p:sldLayout` 根 + fake part（`getRelationships` 可抛 Throwable 模拟早退）。

禁止只按标题冒烟：bOpen 幂等（第二次调用不重新 parse）、rels Throwable 早退、XML Throwable 继续三条路径必须独立覆盖。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | bOpen 第二次跳过 | 状态 | 是 | T01 |
| C02 | rels Throwable 早退 | 失败 | 是 | T02 |
| C03 | themeOverride | 分支 | 是 | T03 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels：fake part 可分两段抛错（getRelationships / getInputStream）区分 C02/C03；
Recording 断言 `parse 次数 ≤ 1`（C01）。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 最小内存输入覆盖 `bOpen 第二次跳过`：连续两次 `open()` | 第二次不再 getInputStream/parse（断言计数） | 红测停在本 function |
| T02 | C02 | 最小内存输入覆盖 `rels Throwable 早退`：`getRelationships` 抛错 | IOLog 等价记录；后续 theme/XML 不执行；`bOpen==true` | 红测停在本 function |
| T03 | C03 | 最小内存输入覆盖 `themeOverride`：part 带 themeOverride rel | `mThemes.open` 恰一次；`mLayout.film().setTheme` 落值 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/ppt/slidelayouts/PptxrLayout.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "04_function_pptx_layout_open"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_pptx_layout_open

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_pptx_layout_open \
  --src src/cn/wps/show/pptxr/ppt/slidelayouts/PptxrLayout.rs \
  --report libs/ppt-core/parser/features/pp-07-part-open/functions/04_function_pptx_layout_open/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_pptx_layout_open --json`)
date: 2026-09-01 18:37

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/ppt/slidelayouts/PptxrLayout.rs` | 101/183 | 55.2% | 180/317 | 56.8% |
| **function total** | **101/183** | **55.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slidelayouts/PptxrLayout.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slidelayouts/PptxrLayout.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slidelayouts/PptxrLayout.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slidelayouts/PptxrLayout.rs \
  --report libs/ppt-core/parser/features/pp-07-part-open/functions/04_function_pptx_layout_open/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 10:37

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxrLayout` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slidelayouts/PptxrLayout.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slidelayouts/PptxrLayout.rs` | **PASS** | `PptxrLayout`, `open`, `buildDefaultLayout`, `getLayout` | `getVmlDrawingRels`, `getRels` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->