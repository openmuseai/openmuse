# pp-01-pptx-parse — PPTX parser 基础契约与 slide XML 装配

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（7 个 function，均 pass）
> 依赖：无
> 管线：[PIPELINE.md](../PIPELINE.md) / [README.md](../README.md)

## 目标

PPTX 业务解析的词汇层与 part XML 装配层：把 OOXML 属性字符串映射为稳定枚举 id、锁定 parser
mode 与回调默认策略，再把 `p:nvPr` / `p:cSld` / `p:sld` / `p:sldLayout` / `p:sldMaster` 的内存
XML 事件分别写入 `ShowShape` 元数据与 film/protocol。它是 `pptx bytes → KmoPresentation` 管线的
第一层，被 pp-02 之后全部 feature 消费；**不打开 ZIP/OPC、不打开 part、不把整个 PPTX 编排成
`Presentation`**——那属于 pp-07 / pp-09。

## I/O（锁死）

| function | 输入 | 输出 | 契约说明 |
|---|---|---|---|
| `01` SWReaderMap | OOXML 属性字符串 | `Option<i32>` 枚举查表 | 纯查表，锁定 54 张映射表；不负责 XML 遍历或模型写入 |
| `02` HandlerCallback | parser mode；typed callback | mode 判定；identity/-1/None/no-op fallback | 锁定四种 mode 与默认 callback；不依赖 01；不是 XML 分发器 |
| `03` SwNvPrHandler | `p:nvPr` attrs/sub-events + `ShowShape` | flags、media mode、placeholder、custData 写入 | 消费 01 的 placeholder 表与 02 的 callback；不解析 `p:spTree`；被 pp-02 leaf 与 04 复用 |
| `04` CommonSlideDataHandler | `p:cSld` attrs/sub-events + film/DgStore | name、bg/custData/control 子 handler、`initRoot`+`updateProtocol` | `3211606` 只构造 `SwGroupShapeHandler`（消费 pp-02 group dispatcher 构造结果）；不解析 leaf shape 内部 |
| `05` SlideHandler | `p:sld` attrs/sub-events + `KmoSlide` | Slide flags + 子 handler 分发 + protocol save | 不启动 slide part reader |
| `06` SlideLayoutHandler | `p:sldLayout` attrs/sub-events | layout type/flags + cSld/hf 分发 + protocol | 同上 |
| `07` SlideMasterHandler | `p:sldMaster` attrs/sub-events | preserve、clrMap/hf/sldLayoutIdLst/txStyles 分发 + protocol | 同上 |

## Java 对照

| Java 类 | Rust 目标路径 | 裁决 |
|---|---|---|
| `show.pptxr.util.SWReaderMap` | `src/cn/wps/show/pptxr/util/SWReaderMap.rs` | 01；纯映射 |
| `drawing.dmlr.model.HandlerCallback` | `src/cn/wps/drawing/dmlr/model/HandlerCallback.rs` | 02 |
| `show.pptxr.handler.shape.SwNvPrHandler` | `src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs` | 03 |
| `CommonSlideDataHandler` + `BackgroundHandler` + `ControlListHandler` | `src/cn/wps/show/pptxr/handler/slides/*.rs` | 04 |
| `SlideHandler` + `ColorMapOverrideHandler` | 同上 | 05 |
| `SlideLayoutHandler` | `.../handler/slidelayouts/SlideLayoutHandler.rs` | 06 |
| `SlideMasterHandler` + `IdListHandler` + `TextStylesHandler` + `HeaderFooterHandler` | `.../handler/slidemasters/` | 07 |
| `PptxReader` | pp-09-pptx-open | 移出 |

## 必须 / 壳 / 延后

- **must：** 01 的 token→枚举查表；02 的 mode 判定与默认 fallback；03 用内存 attrs/events 写入
  `ShowShape` 元数据；04 用内存 cSld 事件写出 name/bg/custData/control 子 handler 与
  `initRoot`+`updateProtocol`；05–07 按 element id 分发真实命名子 handler 并写 protocol 字段。
- **stub：** timing/transition、clrMap/lstStyle/txStyles 子树——本层按 element id 构造 handler
  （constructor fixture 锁定 kind），属性写入留 pp-03/pp-05；`CustDataLst` 与 fill/effect/styleRef
  业务体保持测试 fixture。
- **defer：** part open（InputStream → `UnifiedXmlReader` → `callback.load`，pp-07）、theme part
  （pp-04）、presentation.xml（pp-06）、rId 装配与阅读顺序（pp-08）、`PptxReader` 编排（pp-09）。

## 范围内 / 范围外

- **in：** 内存 XML attrs/sub-events → handler 分发、属性写入与 protocol save；枚举查表与 callback
  策略；film/protocol/DgStore 的 model 写入（model 走 contract fixture，默认编译面按 MVP 队列收敛，
  完整模型树 `full` opt-in）。
- **out：** ZIP/OPC 与 part InputStream、`Presentation` 打开顺序编排、渲染、media 装载。

## 验收

1. 01 锁定 54 张映射表。
2. 02 锁定四种 mode 与默认 callback，且不依赖 01 的输出。
3. 03 只用内存 attrs/events 写入 `ShowShape`，消费 01 的 placeholder 表与 02 的 callback。
4. 04 用内存 cSld 事件写入 film/protocol，并对已知子元素返回真实命名 handler。
5. 05–07 各自只消费 04 的构造契约以及 01/02 输出完成分发与 protocol，不启动 slide part reader。

## 依赖

- **depends：** manifest 无前置 feature（`depends_on_feature` 为空）；function 层面：03 与 pp-02/01
  共用同一份 `SwNvPrHandler.rs` 源码与镜像测试，04 消费 pp-02/05 的 group/shape-tree dispatcher 作为
  spTree 子 handler 的构造结果（默认可 constructor fixture，`kind()` 锁定）。
- **does not depend：** pp-02 shape tree（leaf 解析在 pp-02，04 不展开）；pp-03 clrMap/lstStyle 与
  pp-05 timing/transition（子树仅构造）；pp-07 part open；pp-09 `PptxReader`。

## 不过扩展

- `SWReaderMap` 不负责 XML 遍历或模型写入；`HandlerCallback` 不是 XML 分发器。
- `SwNvPrHandler` 不解析 `p:spTree`；`CommonSlideDataHandler` 不解析 leaf shape 内部（`3211606` 只
  构造 `SwGroupShapeHandler`）。
- part handler 不打开 `OpcrPackagePart`；timing/transition 子树本层只锁定分发与属性写入入口，具体
  列表写入归 pp-05。
- 不实现 `PptxReader`（pp-09）；移出 MVP 附录的 notes/handout、chart/diagram/vml、comment/commentAuthors、
  custom、decrypt/`ModifyVerifier`、tableStyles 全量均不在本 feature 承担。