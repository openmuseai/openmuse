# 05_function_pptx_master_open — PptxrMaster.open

> feature: pp-07-part-open ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：master part 的流填成 KmoSlideMaster（SlideMasterHandler → film + `load()`），`onSlideLayoutIds` 为每个 layout part 建 reader 入 `mLayoutsReader` 并 `_addLayout`，master→layout 血缘闭合。缺 part 时 `buildDefaultSlideMaster` 兜底。缺失时断在哪：真实 pptx 的 master 打不开、layout 挂不上 master，产品闭环断；捷径可用 fixture 绕过。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(KmoSlideMaster, part) -> master XML + layout id map

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | 构造 `(KmoPresentation, KmoSlideMaster, OpcrPackagePart part, PptxrMedias, PptxrThemes, PptxrSlides, int mode)`；`open()` 消费 `mPart`；`onSlideLayoutIds(Map<String, Long>)` 由 `MasterHandlerCallbackImpl` 回调进入。 |
| **输出** | XML 填 master（`SlideMasterHandler` 事件 → `mMaster` protocol/film）+ `callback.load()`；`onSlideLayoutIds` 为每个非空 part 建 `PptxrLayout` reader 入 `mLayoutsReader`（key=rel partName）并 `mMaster._addLayout`。 |
| **状态与副作用** | `bOpen` 幂等；缺 part → `mMaster` **整体替换**为 `buildDefaultSlideMaster` 产物；theme rel → `mThemes.open(...)` → `mMaster.film().setTheme`；无 theme rel → `PptxrTheme.buildDefaultTheme()` + `mPresentation._addTheme`；`mMaster.setIOReading(false)` 在 if/else 两个分支之后。 |
| **错误语义** | 缺 part → `buildDefaultSlideMaster`（资源缺 → `FileNotFoundException` 上抛）；XML Throwable → `IOLog.d` 吞掉继续（vml/diagram 移出后只剩 `setIOReading(false)`）；`onSlideLayoutIds` 内部 IOException → `MasterHandlerCallbackImpl` 捕获（Java `printStackTrace`，Rust 以日志等价）。 |

### 1.2 open 流程（查证自 `PptxrMaster.java`，逐行）

```text
1. !bOpen ──> bOpen = true
2. mPart == null ──> mMaster = buildDefaultSlideMaster(presentation, medias, mode)   （替换对象）
   else：
   a. relses = mPart.getRelationships()（可能 null）
      themeRels = relses.getRelationshipByType(theme)（relses null 时为 null）
   b. themeRels != null ──> theme = mThemes.open(...) ──> mMaster.film().setTheme(theme)
      else                ──> theme = PptxrTheme.buildDefaultTheme()
                              mMaster.film().setTheme(theme)；mPresentation._addTheme(theme)
   c. try  mPart.getInputStream()
           → new MasterHandlerCallbackImpl(drawingStore, mediaLib, inkLib, mPart.getRelationships(), this, medias, slides, mode)
           → new SlideMasterHandler(mMaster, drawingStore, callback)
           → UnifiedXmlReader.read(stream, handler)
           → masterHandlerCallback.load()          （load 在 try 内）
      catch Throwable ──> IOLog.d（继续）
   d.（移出）vml / diagram
3. mMaster.setIOReading(false)
```

`onSlideLayoutIds(Map<String, Long> rids)`（IdListHandler 在 `p:sldLayoutIdLst` 结束事件回调）：

```text
mPart != null ──> 遍历 rids key：
  rels.getRelationship(rid) != null 且 part != null
    ──> new KmoSlideLayout(presentation) + setIOReading(true)
    ──> createPptxrLayout(...) → mLayoutsReader.put(rels.getPartName(), layoutReader)
    ──> mMaster._addLayout(layout)；layout.setMaster(mMaster)
```

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `SlideMasterHandler` | pp-01:07（已 pass） | p:sldMaster 事件 + `onSlideLayoutIds` 回调 | 是 | — | 不改已 pass 的 pp-01/pp-02 语义 |
| `UnifiedXmlReader.read` | 01（本 feature） | part stream → handler 事件 | 未接线 | — | — |
| `HandlerCallbackImpl` / `MasterHandlerCallbackImpl` | 02（本 feature） | load()/onBlipEmbed 等 + onSlideLayoutIds 转发 | 未接线 | — | — |
| `PptxrThemes.open` / `PptxrTheme.buildDefaultTheme` | pp-04 | theme rel → KmoTheme；默认 theme | spec | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不内联 theme 解析 |
| `PptResourceFactory.getSlideMaster`；`PptxrLayout.buildDefaultLayout` | 资源层 / 04 | 默认 master/layout 资源 | 未接线 | — | 不在本 function 内造资源 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/ppt/slidemasters/PptxrMaster.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slidemasters/PptxrMaster.rs`（目录/文件尚未创建） |
| `koffice/presentation/core/cn/wps/show/pptxr/ppt/slidemasters/MasterHandlerCallbackImpl.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slidemasters/MasterHandlerCallbackImpl.rs`（目录/文件尚未创建） |

### 关键方法 / 字段

| Java | Rust 目标 | 契约要点 |
|---|---|---|
| 构造 `PptxrMaster(KmoPresentation, KmoSlideMaster, OpcrPackagePart, PptxrMedias, PptxrThemes, PptxrSlides, int)` | `PptxrMaster::new` | `mLayoutsReader` 初值为空 map；字段 `mMaster/mPart/mMedias/mThemes/mSlides/mMode` + `bOpen=false` |
| `open()` | `open(&mut self)` | §1.2 流程 1–3 |
| `onSlideLayoutIds(Map<String, Long>)` | 同名 | 为每个 rel 建 `PptxrLayout` reader（见 §1.2） |
| `createPptxrLayout(...)` | 构造 helper | protected 覆盖点 → Rust 如需子类覆盖则提 trait 方法（**待 loop 确认**） |
| `openLayout(OpcrRelationship)` | 同 | `!bOpen → open()`；`mLayoutsReader.get(partName)` hit → `layoutReader.open()` + `getLayout()`；miss → `PptxrLayout.buildDefaultLayout` + `layout.setMaster(mMaster)` 兜底 |
| `openAllDatas()` | 同 | `mLayoutsReader` 空 → `buildDefaultLayout` + `_addLayout` + `setMaster`；否则逐个 `open()` |
| 静态 `buildDefaultSlideMaster(KmoPresentation, PptxrMedias, int)` | 静态 | `new KmoSlideMaster` + `buildDefaultTheme` + `setTheme` + `_addTheme`；`getSlideMaster` 资源流（null → FileNotFoundException）；`MasterHandlerCallbackImpl(null rels, null master)` + `SlideMasterHandler` + `read`（finally close）+ `load()`；`PptxrLayout.buildDefaultLayout` + `_addLayout` + `setMaster` |
| 静态 `open(KmoPresentation, SlideMaster, OpcrPackagePart, PptxrMedias)` | 延后 | `TextStylesHandler` 路径（defer） |
| `getVmlDrawingRels()` / `getRels(OpcRelation)` | 停车 | vml 移出 |
| `getMaster()` | 同 | `mMaster` 访问器 |
| `MasterHandlerCallbackImpl` | 转发层 | extends `SlideHandlerCallbackImpl`；`onSlideLayoutIds` 转发到 masterReader，IOException → 日志 |

### 改写列表

`open()`、`onSlideLayoutIds`、`openLayout`、`openAllDatas`、静态 `buildDefaultSlideMaster`；
`MasterHandlerCallbackImpl::onSlideLayoutIds` 转发。静态 `open(pres, SlideMaster, ...)`（TextStylesHandler）`defer` 不写。

### 停车列表

- vml（`PptxrVmlDrawing`）/ diagram（`PptxrDiagramParts`）整块 + `getVmlDrawingRels`/`getRels`
- 静态 `open(pres, SlideMaster, part, medias)`（延后，TextStylesHandler 路径）
- `PptResourceFactory` 资源流供应（**待 loop 确认**，非本 function 造资源）

## 3. 测试方案

### 3.1 代码审计与测试面

- Java 侧：`PptxrMaster.java` 全部（open 的 null-part 分支、theme rel 双分支、XML catch 不 return、onSlideLayoutIds/createPptxrLayout/openLayout/openAllDatas/buildDefaultSlideMaster）；`MasterHandlerCallbackImpl.java` 的 onSlideLayoutIds 转发。
- 当前 Rust：`src/cn/wps/show/pptxr/ppt/slidemasters/` 整树未创建。
- fixture：最小 XML `p:sldMaster` 根 + fake part；`p:sldLayoutIdLst` 事件驱动 `onSlideLayoutIds`。

禁止只按标题冒烟：null part 替换 mMaster、theme rel 双分支（有/无）、onSlideLayoutIds 的 rel null/part null 跳过、bOpen 幂等必须独立覆盖。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | null part → default master | 边界 | 是 | T01 |
| C02 | theme rel vs default theme | 分支 | 是 | T02 |
| C03 | onSlideLayoutIds 建 layout readers | 交互 | 是 | T03 |
| C04 | bOpen 幂等 | 状态 | 是 | T04 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels：fake part 记录 getInputStream/getRelationships；T01 用 fake 资源工厂返回内存流；
T03 让 fake rels 返回带 package part 的 rel，断言 `mLayoutsReader` 落键与 `_addLayout`/`setMaster` 副作用。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 最小内存输入覆盖 `null part → default master`：part=None + fake 资源流 | `mMaster` 被替换；默认 theme 已 `_addTheme`；`setIOReading(false)` 执行 | 红测停在本 function |
| T02 | C02 | 最小内存输入覆盖 `theme rel vs default theme`：part 带/不带 theme rel | 带 rel → `mThemes.open` 恰一次；不带 → `buildDefaultTheme` + `_addTheme` | 红测停在本 function |
| T03 | C03 | 最小内存输入覆盖 `onSlideLayoutIds 建 layout readers`:`p:sldLayoutIdLst` 事件 + fake rels | `mLayoutsReader` 键=partName；`_addLayout`/`setMaster` 落值；rel null 跳过 | 红测停在本 function |
| T04 | C04 | 最小内存输入覆盖 `bOpen 幂等`：连续两次 `open()` | 第二次不再 parse（断言计数 ≤ 1） | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/ppt/slidemasters/PptxrMaster.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "05_function_pptx_master_open"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 05_function_pptx_master_open

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 05_function_pptx_master_open \
  --src src/cn/wps/show/pptxr/ppt/slidemasters/PptxrMaster.rs \
  --report libs/ppt-core/parser/features/pp-07-part-open/functions/05_function_pptx_master_open/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 05_function_pptx_master_open --json`)
date: 2026-09-01 18:42

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/ppt/slidemasters/MasterHandlerCallbackImpl.rs` | 6/16 | 37.5% | 14/21 | 66.7% |
| `src/cn/wps/show/pptxr/ppt/slidemasters/PptxrMaster.rs` | 189/272 | 69.5% | 410/546 | 75.1% |
| **function total** | **195/288** | **67.7%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slidemasters/PptxrMaster.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slidemasters/PptxrMaster.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slidemasters/PptxrMaster.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slidemasters/PptxrMaster.rs \
  --report libs/ppt-core/parser/features/pp-07-part-open/functions/05_function_pptx_master_open/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 10:42

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxrMaster` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slidemasters/PptxrMaster.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slidemasters/PptxrMaster.rs` | **PASS** | `PptxrMaster`, `open`, `getMaster`, `onSlideLayoutIds`, `createPptxrLayout`, `openLayout`, `openAllDatas`, `buildDefaultSlideMaster` | `getVmlDrawingRels`, `getRels` | — | **PASS** |
| `MasterHandlerCallbackImpl` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slidemasters/MasterHandlerCallbackImpl.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slidemasters/MasterHandlerCallbackImpl.rs` | **PASS** | `MasterHandlerCallbackImpl`, `onSlideLayoutIds` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->