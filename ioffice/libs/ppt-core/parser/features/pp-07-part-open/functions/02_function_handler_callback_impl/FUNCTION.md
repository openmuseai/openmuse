# 02_function_handler_callback_impl — HandlerCallbackImpl.load

> feature: pp-07-part-open ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：XML 期副作用在 `load()` 落定——blip rId→mediaId、hyperlink 改写、connector 重映射与 chart/diagram 收集，`openMedia` 真正打开媒体 part。01 的 parser 事件由此补上语义收尾。缺失时断在哪：真实 pptx 的 part 打开后引用/媒体对不上，产品闭环断；捷径可用 fixture 绕过。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(rels, mediaLib, mode, XML 期副作用) -> load() 解析 maps

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | `OpcrRelationships`（rels）+ `KmoMediaLib`/`KmoInkLib` + `DgStore` + `PptxrMedias`/`PptxrSlides`（均可为 null）+ mode（0=PPTX / 1=PPT14Slides / 2=GVml / 3=ArtTable，落在基类判定）。 |
| **输出** | `load()` 完成 connector 重映射；XML 期副作用落定：`mRidPicMediaIds`（rId→mediaId）、blip `putEmbedMediaID`/`putLinkMediaID`、hyperlink target/slideId 改写、`chartMap`/`diagrams` 收集就绪。 |
| **状态与副作用** | 实例字段：`shapeIdMap`（origin→core）、`connectorShapeIdList`、`mRidPicMediaIds`、`chartMap`、`diagrams`；`mMedias.openMedia(mediaLib, rel)` 会真正打开媒体 part。 |
| **错误语义** | rel 缺失（blip/hyperlink）→ `KmoLog.logPoint` 后静默，不回抛给 XML 引擎；`onTags` 的 IOException → IOLog 吞掉；`onInk` 的 IOException → `RuntimeException` 上抛。 |

### 1.2 方法清单（查证自 `HandlerCallbackImpl.java` + `SlideHandlerCallbackImpl.java`）

```text
构造 HandlerCallbackImpl(DgStore, KmoMediaLib, KmoInkLib, OpcrRelationships, PptxrMedias, PptxrSlides, int mode)
load() ──> updateConnectorSite()
XML 期 override：onTags / onBlipEmbed / onBlipLink / onAudioFileLink / onExtMediaFileLink /
  onQuickTimeFileLink / onVideoFileLink / onEmbeddedWAVAudioFileEmbed / onHyperlinkRid /
  onHyperlinkAction / onShapeId / onConnectorShape / onOleObject / onChartRelationShip /
  addDiagramPair / onInk / onModel3D
查询：getCoreShapeId / getChartMap / getDiagramPairs / getRidPicMediaIds
SlideHandlerCallbackImpl：上述全部逐方法转发到内部 HandlerCallbackImpl（含 load()）
```

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 基类 `HandlerCallback` | pp-01:02（已 pass） | mode 判定 isPPTX/isPPT14Slides/isGVml/isArtTable + 默认 fallback | 是 | — | 不改已 pass 的 pp-01/pp-02 语义 |
| `PptxrMedias.openMedia` | 媒体模块（Rust 侧接线**待 loop 确认**） | rId→mediaId 落点 | 未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不在本 function 内实现媒体打开 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/HandlerCallbackImpl.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/HandlerCallbackImpl.rs`（目录/文件尚未创建） |
| `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/SlideHandlerCallbackImpl.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/SlideHandlerCallbackImpl.rs`（目录/文件尚未创建） |

### 关键方法 / 字段

| Java | Rust 目标 | 契约要点 |
|---|---|---|
| 构造 `HandlerCallbackImpl(...)` | `HandlerCallbackImpl::new(...)` | `super(mode)` 落基类判定（不重复实现）；建空 `shapeIdMap`/`chartMap`/`diagrams` |
| `load()` | `load()` | 转发 `updateConnectorSite()` |
| `updateConnectorSite()` | 私有 | 遍历 `connectorShapeIdList`：`Shape.open(dgStore.query(id))`；`hasStCxn/hasEndCxn` → `putId(getCoreShapeId(originId))`；`dgStore.update(id, save())` |
| `onBlipEmbed(String embed, Blip)` | 同名 | rel null → logPoint；否则 `mMedias.openMedia(mediaLib, rel)` → `blip.putEmbedMediaID` + `mRidPicMediaIds.put(rel.getId(), mediaId)` |
| `onBlipLink(String link, Blip)` | 同名 | 同上但 `putLinkMediaID` |
| `onAudioFileLink` / `onQuickTimeFileLink` / `onVideoFileLink` / `onEmbeddedWAVAudioFileEmbed` | 同名 | `openMedia` + putMediaID；Java 侧无 null 判断，逐行照抄 |
| `onExtMediaFileLink(String link, Media)` | 同名 | audio：mediaData 空且 `mediaType∈(15,32)` → openMedia；video：`mediaType∈(33,46)` → openMedia；rel null → return |
| `onHyperlinkRid(String rId, Hyperlink)` | 同名 | INTERNAL slide relsType → fileName → slideId（`mSlides` 非空 → `getSlideIdByFileNameNotAdd`；空 → 静态 `PptxrSlides.getSlideIdByFileName` 查表）；EXTERNAL → `putTargetInternalMode(false)`；`putType(rel.getType())` |
| `onHyperlinkAction(String action, Hyperlink)` | 同名 | `hyperlink.putAction(action)` |
| `onShapeId(Integer, Integer)` / `getCoreShapeId(Integer)` | 同名 | `shapeIdMap` 首次写入；查询 identity 兜底 |
| `onConnectorShape(Integer)` | 同名 | 追加 `connectorShapeIdList` |
| `onOleObject(String rId)` | 同名 | `openMedia` → mediaId |
| `onChartRelationShip(String rId, Shape)` / `getChartMap()` | 同名 | `chartMap.put(rId, shape)` — 只收集不 open chart |
| `addDiagramPair(int, String×4)` / `getDiagramPairs()` | 同名 | `diagrams.put(smartArtId, DiagramPair)` — 只收集不 open |
| `onInk(String rId, FileManager)` | 同名（壳） | rel → `getPackagePart().getInputStream(true)` + `InkModel(stream, "ink", entry, fileManager)` → `inkLib.addInkModel`；IOException → RuntimeException |
| `onModel3D(String embed, Model3D)` | 同名 | rel 非空 → `openMedia` + `putEmbedMediaID` |
| `onTags(String id, CustDataLst.TagLst)` | 同名 | rel → `PptxrTags.open()`（IOException → IOLog） |
| `getRidPicMediaIds()` | 同名 | 返回 `mRidPicMediaIds` |
| `SlideHandlerCallbackImpl.*` | 转发层 | 每个 override（含 `load()`）委托内部 `HandlerCallbackImpl` 实例 |

### 改写列表

HandlerCallbackImpl 的全部 XML 回调 override + `load()`/`updateConnectorSite()`；SlideHandlerCallbackImpl 逐方法转发。
**不改基类** `HandlerCallback`（pp-01:02 已 pass）。

### 停车列表

- `PptxrTags.open()`（custData 关联 part，post-MVP 前可 todo!）
- `onInk` 的 `InkModel`/`FileManager`（壳）
- ppt14 模式分支（`Ppt14rSlides.getSlideIdByFileName`，非 MVP）

## 3. 测试方案

### 3.1 代码审计与测试面

- Java 侧：`HandlerCallbackImpl.java` 全部方法体（load/updateConnectorSite、blip/media/hyperlink/ink 族、shapeIdMap/connector 收集、chartMap/diagram 收集）；`SlideHandlerCallbackImpl.java` 转发逐一对照；基类 `HandlerCallback.java` 的默认返回值（`getChartMap()`→null、`onOleObject`→-1、identity `getCoreShapeId`）。
- 当前 Rust：`src/cn/wps/show/pptxr/ppt/` 整树未创建；`PptxrMedias.openMedia` 落点**待 loop 确认**。
- fixture：假 `OpcrRelationships`（记录 getRelationship 命中）+ Recording media lib；用真实 `SlideHandler`/`SlideLayoutHandler`（pp-01 pass）驱动最小 XML 触发回调。

禁止只按标题冒烟：每个 override 的 null 分支与 put* 落点必须逐一断言。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | load 空 maps | 边界 | 是 | T01 |
| C02 | blip/media rId 解析 | 正常 | 是 | T02 |
| C03 | chartMap/diagram 收集但不 open chart | 边界 | 是 | T03 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels。断言 `openMedia` 调用计数与 `putEmbedMediaID`/`putLinkMediaID`/`mRidPicMediaIds` 落值；chart/diagram 只进 map，不触发 PptxrChart/PptxrDiagramParts。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 最小内存输入覆盖 `load 空 maps`：无 connector 回调 | `load()` 空转，无 panic | 红测停在本 function |
| T02 | C02 | 最小内存输入覆盖 `blip/media rId 解析`：`a:blip r:embed` 事件 + 假 rels | `openMedia` 恰一次；`putEmbedMediaID`/`mRidPicMediaIds` 与 Java 一致；rel 缺失 → logPoint 静默 | 红测停在本 function |
| T03 | C03 | 最小内存输入覆盖 `chartMap/diagram 收集但不 open chart`：`onChartRelationShip`/`addDiagramPair`+`load()` | map 就绪且无 chart/vml/notes 副作用 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/ppt/slides/HandlerCallbackImpl.rs
       src/cn/wps/show/pptxr/ppt/slides/SlideHandlerCallbackImpl.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "02_function_handler_callback_impl"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_handler_callback_impl

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_handler_callback_impl \
  --src src/cn/wps/show/pptxr/ppt/slides/HandlerCallbackImpl.rs \
  --src src/cn/wps/show/pptxr/ppt/slides/SlideHandlerCallbackImpl.rs \
  --report libs/ppt-core/parser/features/pp-07-part-open/functions/02_function_handler_callback_impl/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_handler_callback_impl --json`)
date: 2026-09-01 18:29

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/ppt/slides/HandlerCallbackImpl.rs` | 67/178 | 37.6% | 133/357 | 37.3% |
| `src/cn/wps/show/pptxr/ppt/slides/SlideHandlerCallbackImpl.rs` | 18/57 | 31.6% | 36/78 | 46.2% |
| **function total** | **85/235** | **36.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/HandlerCallbackImpl.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/HandlerCallbackImpl.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/SlideHandlerCallbackImpl.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/SlideHandlerCallbackImpl.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/HandlerCallbackImpl.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/HandlerCallbackImpl.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/SlideHandlerCallbackImpl.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/SlideHandlerCallbackImpl.rs \
  --report libs/ppt-core/parser/features/pp-07-part-open/functions/02_function_handler_callback_impl/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 10:29

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `HandlerCallbackImpl` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/HandlerCallbackImpl.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/HandlerCallbackImpl.rs` | **PASS** | `HandlerCallbackImpl`, `load`, `updateConnectorSite`, `onBlipEmbed`, `onBlipLink`, `onShapeId`, `getCoreShapeId`, `onConnectorShape`, `onChartRelationShip`, `addDiagramPair` | `onTags`, `onAudioFileLink`, `onExtMediaFileLink`, `onQuickTimeFileLink`, `onVideoFileLink`, `onEmbeddedWAVAudioFileEmbed`, `onHyperlinkRid`, `onHyperlinkAction`, `onOleObject`, `onInk`, `onModel3D` | — | **PASS** |
| `SlideHandlerCallbackImpl` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/SlideHandlerCallbackImpl.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/SlideHandlerCallbackImpl.rs` | **PASS** | `SlideHandlerCallbackImpl`, `load`, `onBlipEmbed`, `onChartRelationShip`, `addDiagramPair` | `onTags`, `onBlipLink`, `onAudioFileLink`, `onExtMediaFileLink`, `onQuickTimeFileLink`, `onVideoFileLink`, `onEmbeddedWAVAudioFileEmbed`, `onHyperlinkRid`, `onHyperlinkAction`, `onShapeId`, `getCoreShapeId`, `onConnectorShape`, `onOleObject`, `onInk`, `onModel3D` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->