# 04_function_common_slide_data — p:cSld film + protocol

> feature: pp-01-pptx-parse ｜ 状态：**pass**（12 tests；line 91.6%；layout/alignment PASS）

## 0. 管线能力解锁

已实现并验收：p:cSld 建立 film root（根 `KmoShowShape` + `spTree().initRoot`），name/背景/custData/shape tree/control list 进 CommonSlideData 并 save protocol。缺失时断在哪：slide 树无根，sld/layout/master 的子树分发无处挂载。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

把 `p:cSld` 的 name 属性、背景/自定义数据/shape tree/control list 子元素写入既有 film：
构造根 `KmoShowShape`，在 `endElement` 上 `spTree().initRoot` 并 `updateProtocol(commonSlideData.save())`。
未知子元素不构造 handler；`3211295` 显式 no-op。

### 1.1 I/O 契约

```text
(KmoFilm, DgStore, HandlerCallback, cSld start/sub/end events)
  -> film.root + CommonSlideData name/bg/custData + child ElementHandler
```

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `HandlerCallback` | 02，D/call-only | 传给 bg/custData/group/control | **pass** | 无 | 不改 callback 语义 |
| `SwGroupShapeHandler` | pp-02/05，D/constructor | `new(shapeTree, callback)` | **pass**（默认面用 constructor fixture 校验 kind） | 不在本 function 重写 spTree | 不把 group 解析塞进 cSld |
| `CustDataLstHandler` | C/constructor | `new(custDataLst, callback)` | 测试 fixture | 业务体后续 | 不用 `None` 假装支持 |
| `FillPropertiesHandler`/`EffectPropertiesHandler`/`StyleMatrixReferenceHandler` | DML，C | bgPr 子元素转派 | 测试 fixture | DML feature | 不实现 fill/effect 解析 |
| `KmoFilm`/`KmoShowShape`/`DgStore`/`CommonSlideData`/`Background` | model，E/fixture | parent/create/putName/background/save/initRoot/updateProtocol/addChild | 默认编译面 `full` 才进 crate | 测试 contract fixture | 不恢复 KoBin 全图 |
| `XmlRoAttr`/`ElementHandler` | kfc，E | getByName/count 不需要；start/enter/end | 测试内存 attrs | 不实现 reader | |
| `DGMap.blackWhiteModeMap` | drawing map，E | bg bwMode token | 测试 Java 字面量 | 不恢复 DGMap 全表 | 不改 bw 常量 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.java` |
| Java bg | `.../handler/slides/BackgroundHandler.java` |
| Java ctl | `.../handler/slides/ControlListHandler.java` |
| Rust | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs` |
| Rust bg | `.../slides/BackgroundHandler.rs` |
| Rust ctl | `.../slides/ControlListHandler.rs` |
| caller | `SlideHandler`、`SlideLayoutHandler`、`SlideMasterHandler` |

改写列表：

- `CommonSlideDataHandler`：ctor、`startElement`、`enterSubElement`、`endElement`
- `BackgroundHandler`：ctor、`startElement`、`enterSubElement`；nested `BackgroundPropertiesHandler` ctor/`startElement`/`enterSubElement`
- `ControlListHandler`：ctor、`enterSubElement`；nested `ControllerHandler` ctor/`startElement`/`enterSubElement`；`AlternateContentHandler`/`FallBackHandler` `enterSubElement`

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `CommonSlideDataHandler` ctor | CommonSlideDataHandler.java | film+store+callback | 新建 shapeTree type=0；`CommonSlideData.create()` | 无 |
| `startElement` | 同上 | attr 3211449 有/无 | `putName` | present/absent |
| `enterSubElement` | 同上 | 3211308/3211519/3211606/3211607/3211295/unknown | 对应 handler 或 null | 每个 case |
| `endElement` | 同上 | 任意 | initRoot(shapeTree)；updateProtocol(save) | 必发生一次 |
| `BackgroundHandler.startElement` | BackgroundHandler.java | 1114443 或 3211605 | `putBwMode` | 两 name；known/unknown token |
| `BackgroundHandler.enterSubElement` | 同上 | 3211603/3211604/unknown | bgPr nested / StyleMatrixReference / None | |
| `BackgroundPropertiesHandler` | nested | 3211602；fill ids；effect ids | shadeToTitle；转派 fill/effect | 各 id |
| `ControlListHandler.enterSubElement` | ControlListHandler.java | 2162692/3211472/unknown | AlternateContent / Controller / None | |
| `ControllerHandler` | nested | 3211464/3211463；child 3211469 | imageH/W；addChild type=2 + SwPictureHandler | 有/无 pic |
| AlternateContent Fallback | nested | 2162695 → 3211472 | 回到 ControllerHandler | Choice 路径 default 不进 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| C01 | cSld name present/absent | 分支 | 是 | T01–T02 |
| C02 | cSld children bg/cust/spTree/ctrl/ext/unknown | 分支 | 是 | T03–T04 |
| C03 | endElement initRoot+protocol | 状态转换 | 是 | T05 |
| C04 | bwMode 两 attr name + known/unknown | 后端契约 | 是 | T06–T07 |
| C05 | bgPr shadeToTitle + fill/effect dispatch | 分支 | 是 | T08 |
| C06 | bgRef StyleMatrixReference | 交互 | 是 | T09 |
| C07 | control pic child addChild+handler | 交互 | 是 | T10 |
| C08 | AlternateContent fallback → controller | 分支 | 是 | T11 |
| C09 | unknown/ext no-op | 失败/边界 | 是 | T12 |

### 3.3 可测试性

`#[path]` 编译三份 production handler；film/shape/store/background/CommonSlideData 用合同 fixture。
`SwGroupShapeHandler`/`SwPictureHandler`/`CustDataLstHandler`/fill/effect/styleRef 用同名 constructor
fixture，`kind()` 锁定返回类型。不启动 XML reader 或 OPC。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| T01 | C01 | 新 handler | attr 3211449=`Title` | `name()=="Title"` | name 未写入 |
| T02 | C01 | 新 handler | 空 attrs | name 保持默认空 | 空 attrs 写脏 name |
| T03 | C02 | 新 handler | 3211308/3211519/3211606/3211607 | kind 分别为 Background/CustDataLst/SwGroup/ControlList | 错 handler |
| T04 | C02/C09 | 新 handler | 3211295 与 -1 | 均 `None` | ext 被当成成功子树 |
| T05 | C03 | 新 handler | endElement | film.root 即 ctor 的 shapeTree；protocol_save_count=1 | 未 initRoot 或未 save |
| T06 | C04 | bg handler | 1114443=`gray` | bwMode=2 | 查表失败 |
| T07 | C04 | bg handler | 3211605=`nope` 与 `clr` | unknown 保持默认；`clr`→0 | 未知 token 乱写 |
| T08 | C05 | bgPr nested | shade true；fill 1114370；effect 1114389 | shade 写入；返回 fill/effect fixture kind | 未转派 |
| T09 | C06 | bg handler | 3211604 | StyleMatrixReferenceHandler kind | 未构造 |
| T10 | C07 | control list | 3211472 后 3211469 | child type=2 加入 tree；SwPictureHandler | 未 addChild |
| T11 | C08 | control list | 2162692→2162695→3211472 | 得到 ControllerHandler | fallback 丢失 |
| T12 | C09 | bg/control | unknown child | `None` | 未知 id 误命中 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs
       libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/BackgroundHandler.rs
       libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/ControlListHandler.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs
[[test]] name = "04_function_common_slide_data"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_common_slide_data

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_common_slide_data \
  --src src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs \
  --src src/cn/wps/show/pptxr/handler/slides/BackgroundHandler.rs \
  --src src/cn/wps/show/pptxr/handler/slides/ControlListHandler.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/04_function_common_slide_data/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_common_slide_data --json`)
date: 2026-08-29 18:40

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/slides/BackgroundHandler.rs` | 104/114 | 91.2% | 91/105 | 86.7% |
| `src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs` | 83/95 | 87.4% | 67/86 | 77.9% |
| `src/cn/wps/show/pptxr/handler/slides/ControlListHandler.rs` | 149/158 | 94.3% | 126/137 | 92.0% |
| **function total** | **336/367** | **91.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/BackgroundHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/BackgroundHandler.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/ControlListHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/ControlListHandler.java
```

Loop 结果：**PASS**（CommonSlideDataHandler 50、BackgroundHandler 52、ControlListHandler 66 条 `// Java:` 锚点对齐）。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/BackgroundHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/BackgroundHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/ControlListHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/ControlListHandler.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/04_function_common_slide_data/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-29 10:40

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `CommonSlideDataHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs` | **PASS** | `CommonSlideDataHandler`, `startElement`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `BackgroundHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/slides/BackgroundHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/BackgroundHandler.rs` | **PASS** | `BackgroundHandler`, `startElement`, `enterSubElement` | — | — | **PASS** |
| `ControlListHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/slides/ControlListHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/ControlListHandler.rs` | **PASS** | `ControlListHandler`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
