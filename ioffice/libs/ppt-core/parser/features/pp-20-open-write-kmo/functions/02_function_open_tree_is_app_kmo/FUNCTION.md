# 02_function_open_tree_is_app_kmo — open 产物是 app Kmo 树

> feature: pp-20-open-write-kmo ｜ 状态：**pass** ｜ Java 管线：S4 产线写入

## 0. 管线能力解锁

pp-19 测试面绿 ≠ `open_pptx_document_bytes` 返回 app 树。今日 `parse_one_sp:803` 仍构造 `layout_shape::KmoShowShape`。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | 最小 pptx zip（一页一 sp） |
| **输出** | app::KmoShapeTree.shapeCount()==1 且 type()==DgStore.queryType(id) |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| open_pptx_document_bytes | pptx_mvp | zip→PptxDocument.films | **本 function**：`CommonSlideDataHandler` + `SwGroupShapeHandler` 写 `KmoFilm.spTree()` | PptxReader 全编排；layout_shape 仍喂 render | 禁止 --force PptxrSlide |
| CommonSlideDataHandler | pptxr/handler/slides | ctor / 3211606 / endElement.initRoot | **本 function** 进默认 `handler/slides` | bg/custData/ctl 内容 handler | 禁止删 `// java` |
| KmoFilm | show/app/slide | ctor / parent / spTree / updateProtocol | **本 function** 交错模板 | copy/media/equals | — |
| KmoShapeTree | show/app/shape | ctor `new KmoShowShape(parent,0)` / initRoot / shapeCount | ctor 已按 Java 2 参；initRoot/shapeCount 可用 | groupUp 等 | 禁止 layout_shape 冒充 pass |
| layout_shape::KmoSlide | fixture | 过渡夹具 | 仍在 `PptxDocument.slides` 供 render | 测试夹具可留 | 禁止当 app 树真相 |

## 2. 核心代码与函数改写列表

**本 function：** `open_pptx_document_bytes` 经 Java 类名 `CommonSlideDataHandler` / `SwGroupShapeHandler` / `KmoFilm.spTree().initRoot` 写入 `app::KmoShowShape` 树。
**§3.8：** CommonSlideDataHandler、KmoFilm、KmoShapeTree。pptx_mvp 无一对一 Java 叶。
**禁改：** 发明第二套 spid；`--force` PptxrSlide。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 `pptx_mvp.rs parse_one_sp:803`：DataShapePr handler 后拷到 `layout_shape::ShapePr`，再 `KmoShowShape { … }` fixture。`open_pptx_document_bytes:978` 返回 `PptxDocument` 内 slide 同族。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| OT-CNT | open_pptx_document_bytes | 最小 zip 1 sp | shapeCount==1 | 公开入口 |
| OT-TYP | KmoShowShape.type | type1 sp | type()==DgStore.queryType(id) | 关键分支 |
| OT-GRP | parse_one_grp | grp+1 子 | root.childCount()==1 | 关键分支 |
| OT-GREP | pptx_mvp.rs | rg layout_shape::KmoSlide 于 open 返回路径 | 零命中 | 门闩 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| shapeCount | open_pptx_document_bytes | 公开入口 | 是 | OT-CNT |
| type==queryType | KmoShowShape.type | 关键分支 | 是 | OT-TYP |
| 组树 | parse_one_grp | 关键分支 | 是 | OT-GRP |
| 返回类型 | pptx_mvp.rs | 门闩 | 是 | OT-GREP |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| OT-CNT | shapeCount | — | 最小 zip 1 sp | shapeCount==1 | 仍 0 或 fixture 树 |
| OT-TYP | type==queryType | — | type1 sp | type()==DgStore.queryType(id) | 平行 shape_type |
| OT-GRP | 组树 | — | grp+1 子 | root.childCount()==1 | 子没 addChild |
| OT-GREP | 返回类型 | — | rg layout_shape::KmoSlide 于 open 返回路径 | 零命中 | 仍返回 fixture slide |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/pptxr/pptx_mvp.rs, src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs, src/cn/wps/show/app/slide/KmoFilm.rs, src/cn/wps/show/app/shape/KmoShapeTree.rs
test:  tests/cn/wps/show/pptxr/02_function_open_tree_is_app_kmo.rs
[[test]] name = "02_function_open_tree_is_app_kmo"
```

Loop 已落地：OT-CNT/TYP/GRP/GREP 四条非 ignore。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_open_tree_is_app_kmo -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_open_tree_is_app_kmo \
  --src src/cn/wps/show/pptxr/pptx_mvp.rs \
  --src src/cn/wps/show/app/shape/KmoShapeTree.rs \
  --src src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs \
  --src src/cn/wps/show/app/slide/KmoFilm.rs \
  --report libs/ppt-core/parser/features/pp-20-open-write-kmo/functions/02_function_open_tree_is_app_kmo/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_open_tree_is_app_kmo --json`)
date: 2026-09-15 17:42

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/app/shape/KmoShapeTree.rs` | 16/138 | 11.6% | 21/147 | 14.3% |
| `src/cn/wps/show/app/slide/KmoFilm.rs` | 29/105 | 27.6% | 25/96 | 26.0% |
| `src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs` | 49/100 | 49.0% | 68/115 | 59.1% |
| `src/cn/wps/show/pptxr/pptx_mvp.rs` | 650/1040 | 62.5% | 1073/1835 | 58.5% |
| **function total** | **744/1383** | **53.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

**PASS**（2026-09-15）：CommonSlideDataHandler / KmoFilm / KmoShapeTree / SwGroupShapeHandler 布局校验均 PASS。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/app/slide/KmoFilm.rs \
  --java koffice/presentation/core/cn/wps/show/app/slide/KmoFilm.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/app/shape/KmoShapeTree.rs \
  --java koffice/presentation/core/cn/wps/show/app/shape/KmoShapeTree.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.java --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs \
  --java koffice/presentation/core/cn/wps/show/app/slide/KmoFilm.java --rs libs/ppt-core/src/cn/wps/show/app/slide/KmoFilm.rs \
  --java koffice/presentation/core/cn/wps/show/app/shape/KmoShapeTree.java --rs libs/ppt-core/src/cn/wps/show/app/shape/KmoShapeTree.rs \
  --report libs/ppt-core/parser/features/pp-20-open-write-kmo/functions/02_function_open_tree_is_app_kmo/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-15 09:44

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `CommonSlideDataHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs` | **PASS** | `CommonSlideDataHandler`, `startElement`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `KmoFilm` | `koffice/presentation/core/cn/wps/show/app/slide/KmoFilm.java` | `libs/ppt-core/src/cn/wps/show/app/slide/KmoFilm.rs` | **PASS** | `updateProtocol`, `spTree`, `eventListeners`, `parent` | `KmoFilm`, `copyFrom`, `copyFromEx`, `doCopyFrom`, `updatePicRef4Copy`, `protocol`, `name`, `hasName`, `setName`, `setBackgroundFill`, `setFilmBackground`, `clearFilmBackground`, `hasBackgroud`, `background`, `hasCusDataLst`, `custDataLst`, `setCustDataLst`, `ksoWMInfo`, `getTheme`, `setTheme`, `hashCode`, `equalsLax`, `equals`, `compareMedia`, `addTags`, `willSwap`, `didFinishSwapping` | — | **PASS** |
| `KmoShapeTree` | `koffice/presentation/core/cn/wps/show/app/shape/KmoShapeTree.java` | `libs/ppt-core/src/cn/wps/show/app/shape/KmoShapeTree.rs` | **PASS** | `KmoShapeTree`, `initRoot`, `getRoot`, `parent`, `shapeAt`, `indexOfShape`, `shapeCount`, `addShape` | `protocol`, `updateProtocol`, `copyFrom`, `copyFromEx`, `doCopy`, `initSpTreeGroupTransform`, `disconnectShape`, `groupUp`, `ungroup`, `ungroupself`, `addPicture`, `addPenKit`, `add3dObj`, `addCurveLine`, `addAudio`, `addNoteAudio`, `addVideo`, `addPictureData`, `addMediaData`, `depth`, `replacePicture2PlaceHolder`, `replaceAudio`, `addPresetShape`, `addInk`, `addTextBox`, `addConnector`, `addTable`, `addChart`, `addShapes`, `insertShapes`, `findPhByType`, `findPhByIndex`, `deleteShape`, `updateParentXfrm`, `getAllPh`, `getVmlDrawing`, `setVmlDrawing`, `hashCode`, `equals`, `equalsLax` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
