# pp-06-presentation-xml — p:presentation 根 XML

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（`01`–`04` 均 `pass`）
> 依赖：pp-01-pptx-parse（`SWReaderMap` 查表、`PresentationHandlerCallback`、`XmlRoAttr` 事件面）；pp-03-dml-style（`defaultTextStyle` → `TextListStyleHandler`）
> 管线：[PIPELINE.md](../PIPELINE.md) / [README.md](../README.md)

## 目标

`PptxrPresentation.openGlobalData()`（`pptxr/ppt/PptxrPresentation.java`）中
`UnifiedXmlReader.read(presPart 流, PresentationHandler)` —— 解析 `p:presentation` 根元素：

- Presentation **根属性**：count+getAttrAt 循环把 `serverZoom`/`firstSlideNum`/`conformance` 等 12 个
  attr 写入 `Presentation`（put*），`endElement` 时 `mPres.updateProtocol(presData.save())`；
- **尺寸**：`p:sldSz`（type/cx/cy，16:9 魔数 → type 15）与 `p:notesSz`（交 `PositiveSize2DHandler`）；
- **rId 列表回调**：`p:sldIdLst`/`p:sldMasterIdLst`/`p:notesMasterIdLst`/`p:handoutMasterIdLst` 的 item
  rId 收集成 `List<String>`，在列表 `endElement` 时经 `PresentationHandlerCallback` 回调。

**本 feature 不创建 `KmoSlide`** —— pp-08 `PptxrSlides.onSlideIds` 消费回调后建壳。本 feature 只
保证回调参数（rId 列表与顺序）正确。

## I/O（锁死）

| function | I | O | 契约 |
|---|---|---|---|
| 01 presentation_attrs | `p:presentation`(3211535) attrs（count+getAttrAt）+ endElement | `Presentation` flags + `updateProtocol` | 12 个 attr 名 3211523–3211534 → put*；conformance map miss → 1；`mPres == null` 不 `updateProtocol`；空 attrs 不改 create 默认 |
| 02 id_lists | sldIdLst(3211512)/sldMasterIdLst(3211509)/notesMasterIdLst(3211510)/handoutMasterIdLst(3211511) 的 item `r:id`(4259841) | `onSlideIds`/`onSlideMasterIds`/`onNotesMasterIds`/`onHandoutMasterIds` 各收 `List<String>` | 列表 endElement 才回调；`enterSubElement` 返回 this；notes/handout 缺 r:id 复刻 Java NPE 判空语义（测试用合法 attrs） |
| 03 slide_notes_size | sldSz(3211513) attrs type(3211302)/cx(3211488)/cy(3211489)；notesSz(3211514) 子元素 | `SlideSize` cx/cy/type；notesSize cx/cy；16:9 魔数 → type 15 | notesSz 交 `PositiveSize2DHandler`（a:/p: 双命名空间 cx/cy） |
| 04 photo_kinsoku_custshow | photoAlbum(3211518)/kinsoku(3211520)/custShowLst(3211517) 子树 | photoAlbum 字段、kinsoku 三字符串、custShow 经 `onCustomShow` + `customShowList().write` | EmbeddedFontList 可随 04 或暂缓 fixture；ModifyVerifier 移出（加密） |

## Java 对照

| Java 类 | Rust 路径 | 裁决 |
|---|---|---|
| `PresentationHandler`（01 根属性） | `src/cn/wps/show/pptxr/handler/PresentationHandler.rs` | 尚不存在（`handler/mod.rs` 当前仅声明 `custdatas`/`shape`） |
| nested `SlideAndMasterIdListHandler` + `NotesAndHandoutMasterIdListHandler`（02） | 同文件 nested | 待 spec→loop |
| nested `SlideSizeHandler`（03） | 同文件 nested | 待 spec→loop |
| `PositiveSize2DHandler`（DML transform，03 用） | `src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/PositiveSize2DHandler.rs` | 待建；若无则本轮以 contract fixture 吸收 |
| nested `PhotoAlbumHandler`/`KinsokuHandler`/`CustomShowListHandler` 树（04） | 同文件 nested | 待 spec→loop |
| nested `EmbeddedFontListHandler` 树 | 同文件 nested | 可随 04 或延后 constructor fixture |
| nested `CustDataLstHandler`（空类） | `src/cn/wps/show/pptxr/handler/custdatas/CustDataLstHandler.rs`（已存在，独立包） | 仅空类接线 |
| nested `ModifyVerifierHandler` | — | **移出**（加密，不在改写列表） |
| `UnifiedXmlReader.read` | pp-07 `01_function_unified_xml_reader` | 归 pp-07 |
| `TextListStyleHandler`（3211521 defaultTextStyle） | pp-03 `02_function_text_list_style` | 归 pp-03 |

Rust 根路径前缀：`libs/ppt-core/src/cn/wps/show/pptxr/handler/`。数据模型
`libs/ppt-core/src/cn/wps/show/data/Presentation.rs` 已存在（put* / `save()` 齐备），handler 层待建。
元素/attr id 以 `koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/xsd/pml/P.java` 与
`xsd/shared/R.java`（`r:id` = 4259841）为准。

## 必须 / 壳 / 延后

- **must：** 01 的根属性循环（12 case 全部落 `Presentation.put*`，conformance map miss → 1）与
  `endElement` 的 `mPres.updateProtocol(presData.save())`（`mPres != null` 判定）。02 的 rId 列表收集 +
  endElement 一次性回调 + `enterSubElement` 返回 this；notes/handout 复刻 Java 对缺 `r:id` 的 NPE
  判空语义。03 的 sldSz type/cx/cy 写入与 16:9（12192000×6858000）→ type 15 魔数覆盖。04 的
  photoAlbum/kinsoku/custShow 子树与 `onCustomShow` 回调、`customShowList().write`。
- **stub：** `PositiveSize2DHandler`（DML transform，本轮无 Rust 则以 contract fixture 吸收双命名
  空间 cx/cy）；embeddedFontLst（随 04 或 constructor fixture，业务体不展开）；`KmoPresentation`/
  `updateProtocol` 的协议引擎（libs/ppt-core 当前无 `KmoPresentation`，01 交付 save 结果/回调契约）。
- **defer：** `ModifyVerifierHandler`（加密移出）；`TextListStyleHandler`（pp-03）；真实 XML reader
  事件分发（pp-07 接线）；`KmoSlide` 创建（pp-08）。

## 范围内 / 范围外

- **in：** 根 attr→put* 全 12 项；rId 列表四回调；`PositiveSize2DHandler` 的 a:/p: 双命名空间
  cx/cy；同文档同时出现 `sldIdLst`+`sldMasterIdLst` 与 `notesMasterIdLst`+`handoutMasterIdLst`
  （各列表独立实例、各回调各的）。rId 字符串保序（XML 顺序）。
- **out：** ZIP/OPC/`OpcrPackagePart` 打开；KmoSlide 创建与 slide 装配（pp-08）；`KmoPresentation`
  protocol 引擎实例化语义（pp-09 编排）；渲染、加密校验、`defaultTextStyle` 业务（pp-03）。

## 验收

FUNCTIONS.md §4 四点：

1. 空 attrs 不改 create 默认。
2. sldIdLst `endElement` 才回调；`enterSubElement` 返回 this。
3. notes/handout id 缺 `r:id`(4259841)：Java NPE 语义在测试中用合法 attrs（不发明更安全的 Rust）。
4. 16:9 12192000×6858000 → `putType(15)`。

回归：`cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_presentation_attrs` 等四个
`[[test]]`（与 [functions.csv](./functions.csv) 对齐，Loop L2 才创建）。

## 依赖

- **depends：** pp-01（`SWReaderMap` conformance/slideSize/photoAlbum 查表、`PresentationHandlerCallback`、
  `XmlRoAttr` 事件面、`R.id`=4259841）；pp-03（`TextListStyleHandler` 处理 `defaultTextStyle`，本 feature
  只传 `presData.defaultTextStyle()`）。
- **does not depend：** pp-07/08/09（`UnifiedXmlReader.read` 分发、`PptxrSlides` 消费回调、`PptxrPresentation`
  编排仅作为测试入参骨架）。

## 不过扩展

禁止打开 zip/`OpcrPackagePart`（FUNCTIONS.md §0）。禁止 `ModifyVerifierHandler` 密码循环（加密移出）。
禁止创建 `KmoSlide`。禁止把 `EmbeddedFontListHandler`/`TextListStyleHandler` 业务塞进本 feature 冒名
归属。禁止为 notes/handout 缺 `r:id` 发明比 Java 更安全的实现。16:9 魔数必须覆盖 map 值
（`putType(15)` 在 cx/cy 判断之后执行）；不许改用 `slideSizeMap.get("screen16x9")` 之外的等价写法。