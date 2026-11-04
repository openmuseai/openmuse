# pp-01-pptx-parse — PPTX parser 基础契约与 slide XML 装配

> 本 feature 从 OOXML 枚举映射与 DML callback 出发，经 `p:nvPr` 写入 shape 元数据，
> 再把 `p:cSld` / `p:sld` / layout / master 的 XML 事件装配进 film/protocol。
> 它不打开 ZIP/OPC，也不把整个 PPTX 编排成 `Presentation`（那是后续 package feature）。

## §0 MVP 管线

```text
pp-01 parser
  ├─ 01 SWReaderMap: XML token -> enum i32
  ├─ 02 HandlerCallback: parser mode + 默认回调
  ├─ 03 SwNvPr: p:nvPr attrs/sub-events -> ShowShape metadata
  ├─ 04 CommonSlideData: p:cSld events -> film spTree + CommonSlideData
  ├─ 05 SlideHandler: p:sld attrs/sub-events -> Slide protocol
  ├─ 06 SlideLayoutHandler: p:sldLayout -> layout protocol
  └─ 07 SlideMasterHandler: p:sldMaster -> master protocol + layout id map
        ↓
pp-02 shape-tree-parse: p:spTree events -> KmoShowShape/DgStore（04 的 spTree 子 handler）
        ↓
pp-03 clrMap/lstStyle → pp-04 theme part → pp-05 transition/timing
        ↓
pp-06 presentation.xml → pp-07 part open → pp-08 rId walk → pp-09 PptxReader
```

`01`/`02` 是并行基础能力。`03` 被 leaf shape handler 复用（pp-02 已按同一源码验收）。
`04` 消费 pp-02 的 group/shape-tree dispatcher 作为子 handler 构造结果。`05`–`07` 并行消费 `04`。

## §1 function 清单与依赖

| order | function_id | I | O | feature 内 upstream | 下游 |
|---|---|---|---|---|---|
| 1 | `01_function_sw_reader_map` | OOXML 属性字符串 | `Option<i32>` 枚举查表 | 无 | 03;06;pp-02 |
| 2 | `02_function_handler_callback` | parser mode；typed callback | mode 判定；默认 fallback | 无 | 03–07;pp-02 |
| 3 | `03_function_sw_nvpr` | `p:nvPr` attrs/sub-events + `ShowShape` | flags、media mode、placeholder、custData | 01;02 | pp-02 leaf;04 |
| 4 | `04_function_common_slide_data` | `p:cSld` attrs/sub-events + film/store | name、bg/custData/control 子 handler、`initRoot`+`updateProtocol` | 02;pp-02/05 | 05;06;07 |
| 5 | `05_function_slide_handler` | `p:sld` attrs/sub-events + `KmoSlide` | Slide flags + 子 handler 分发 + protocol save | 02;04 | pp-07 part open |
| 6 | `06_function_slide_layout` | `p:sldLayout` attrs/sub-events | layout type/flags + `04`/`hf` 分发 + protocol | 01;02;04 | pp-07 part open |
| 7 | `07_function_slide_master` | `p:sldMaster` attrs/sub-events | preserve、clrMap/hf/sldLayoutIdLst/txStyles 分发 + protocol | 02;04 | pp-07 part open |

## §2 数据流与边界

```text
XML attribute text ── SWReaderMap ──> stable enum id
parser mode/callback ─ HandlerCallback ─> handler policy / default fallback
p:nvPr events ──────── SwNvPrHandler ──> ShowShape metadata
p:cSld events ──────── CommonSlideDataHandler ──> film root + CommonSlideData
p:sld / sldLayout / sldMaster events ── part handlers ──> film/protocol
```

- `SWReaderMap` 不负责 XML 遍历或模型写入。
- `HandlerCallback` 不是 XML 分发器。
- `SwNvPrHandler` 不解析 `p:spTree`。
- `CommonSlideDataHandler` 不解析 leaf shape 内部；`3211606` 只构造 `SwGroupShapeHandler`。
- part handler 不打开 `OpcrPackagePart`；timing/transition 子树由后续 function 实现，本层只锁定分发与属性写入。

## §3 编译与测试面

- crate：`libs/ppt-core/Cargo.toml`
- 测试镜像根：`libs/ppt-core/tests/cn/wps/`
- 单测：`cargo test --manifest-path libs/ppt-core/Cargo.toml --test <function_id>`
- 默认 Cargo 构建面按 MVP 队列收敛；完整模型树仍由 `full` opt-in。
- `03` 与 pp-02 `01_function_nonvisual_shape_props` 共用同一份 `SwNvPrHandler.rs` 源码与镜像测试。

## §4 串联验收

1. 01 锁定 54 张映射表。
2. 02 锁定四种 mode 与默认 callback；不依赖 01。
3. 03 用内存 attrs/events 写入 `ShowShape`；消费 01 的 placeholder 表与 02 的 callback。
4. 04 用内存 cSld 事件写入 film/protocol，并对已知子元素返回真实命名 handler。
5. 05–07 各自只消费 04 的构造契约和 01/02 输出，不启动 slide part reader。

## §5 推荐 loop 顺序

`01 → 02 → 03 → 04 → 05 → 06 → 07`

## §6 Java/Rust 范围裁决

| Java 类 | Rust 路径 | 裁决 |
|---|---|---|
| `show.pptxr.util.SWReaderMap` | `src/cn/wps/show/pptxr/util/SWReaderMap.rs` | 01；纯映射 |
| `drawing.dmlr.model.HandlerCallback` | `src/cn/wps/drawing/dmlr/model/HandlerCallback.rs` | 02 |
| `show.pptxr.handler.shape.SwNvPrHandler` | `src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs` | 03 |
| `CommonSlideDataHandler` + `BackgroundHandler` + `ControlListHandler` | `src/cn/wps/show/pptxr/handler/slides/*.rs` | 04 |
| `SlideHandler` + `ColorMapOverrideHandler` | 同上 | 05 |
| `SlideLayoutHandler` | `.../handler/slidelayouts/SlideLayoutHandler.rs` | 06 |
| `SlideMasterHandler` + `IdListHandler` + `TextStylesHandler` + `HeaderFooterHandler` | `.../handler/slidemasters/` | 07 |
| `PptxReader` | pp-09-pptx-open | 移出 |

## 文档

- 机器状态：[functions.csv](./functions.csv)
- function 方案：`functions/<function_id>/FUNCTION.md`
