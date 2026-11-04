# pp-04-theme-part — theme / themeOverride part

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（3 function）
> 依赖：pp-01-pptx-parse；pp-03-dml-style

## 目标

与 Java `PptxrPresentation.openGlobalData` 对齐：在解析 `presentation.xml` **之前**，通过 presentation
rels 的 `theme` 类型关系（`rels.getRelationshipByType(theme)`）取 `themeRels`，经
`PptxrThemes.open(partName 缓存)` → miss 时 `PptxrTheme.open` 用 `UnifiedXmlReader` +
`ThemeHandler` 解析 theme.xml 填充 `KmoTheme`；解析回调收集的 `rid→mediaId` 图片映射经
`KmoTheme.setRid2MediaId` 写回。slide/layout 的 themeOverride 复用同一缓存（`mGlobalThemes`）。
测试本层只用假 `OpcrRelationship`/`part` 或内存 stream fixture + 注入式 reader，**不跑 `PptxrReader`**。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | theme XML 事件（元素 id + `XmlRoAttr`，驱动 `startElement/endElement/enterSubElement`）+ `KmoTheme`（01）；`OpcrPackagePart` stream + callback（02）；`partName` + `themeRels` + `pres` + `medias`（03）。 |
| **O** | theme 模型字段写入 + 子元素分发（01）；`rid→mediaId` map（02，null 表示无图/失败）；cache hit 返回同一 `KmoTheme` 实例 / miss 新开并入缓存（03）。 |

三条输入输出不可改口：

1. theme XML 事件 → `KmoTheme`（01 全部字段语义）
2. part stream → `rid→mediaId` map（02）
3. partName → cache hit（同一实例）/ miss（open 后 `_addTheme`）（03）

## Java 对照

| Java 类 | Rust 目标 | 裁决 |
|---|---|---|
| `drawing/dmlr/handler/style/ThemeHandler.java` | `src/cn/wps/drawing/dmlr/handler/style/ThemeHandler.rs` | 01 |
| `show/pptxr/ppt/theme/PptxrTheme.java` | `src/cn/wps/show/pptxr/ppt/theme/PptxrTheme.rs` | 02 |
| `show/pptxr/ppt/theme/PptxrThemes.java` | `src/cn/wps/show/pptxr/ppt/theme/PptxrThemes.rs` | 03 |
| `show/pptxr/ppt/slides/HandlerCallbackImpl.java` | `src/cn/wps/show/pptxr/ppt/slides/HandlerCallbackImpl.rs` | 移 pp-07（本层 fixture 只认 ctor 契约 + `load` + `getRidPicMediaIds`） |
| `show/pptxr/util/UnifiedXmlReader.java` | `src/cn/wps/show/pptxr/util/UnifiedXmlReader.rs` | 移 pp-07（02 用注入式 `read` fixture） |
| `show/pptxr/ppt/theme/PptxrThemeOverride.java` | 未规划 | 延后（03 的 themeOverride 类型分支使用；逻辑与 02 同构待裁决） |

`KmoTheme`（`show/app/theme/KmoTheme.rs`）、`Theme` 模型（`drawing/data/style/Theme.rs`）已在库内；
本 feature 只消费 `setRid2MediaId`/`setThemePath`/`updateProtocol`，不改模型。

## 必须 / 壳 / 延后

- **must：** `ThemeHandler` 分发表与 Java 逐一对应（`clrMap(1114459)`、`themeElements(1114461)`、
  `objectDefaults(1114462)`、`extraClrSchemeLst(1114463)`、`custClrLst(1114464)`）；`name` 属性(1114136)→`putName`；
  `endElement`→`themeReceiver.updateProtocol(themeData.save())`。`PptxrTheme.open`：null part→null；
  Throwable 吞掉→null（含 `handlerCallback.load()` 在 try 内）。`PptxrThemes.open`：cache hit 同一实例；
  miss open + put + `addTheme` 默认 true。`buildDefaultTheme()` 资源流 + callback=null。
- **stub：** `rid→mediaId` 的媒体搬运细节（`onBlipEmbed`/`onBlipLink`）依赖 pp-07 `HandlerCallbackImpl`，
  本层 fixture callback 只断言键值；theme XML scheme 全表（`clrScheme`/`fontScheme`/`fmtScheme` 内部子元素）
  随各子 handler，本 feature 只保障 `ThemeHandler` 分发命中。
- **defer：** `PptxrThemeOverride.open`（03 分支，与 02 同构）；slide/layout 侧 themeOverride 复用接线（pp-07）。

## 范围内 / 范围外

- **in：** theme 根属性 `name`；`objectDefaults` 的 `spDef/lnDef/txDef` 分发到
  `BodyPropertiesHandler`/`TextListStyleHandler`/`ShapePropertiesHandler`/`ShapeStyleHandler`
  （子 handler 实现可停车，分发必须通）；`extraClrSchemeLst`/`custClrLst` 列表积累 + `endElement` write-back；
  `dumps` 与 `mGlobalThemesPath` 路径缓存。
- **out：** `presentation.xml` 解析与 `openGlobalData` 编排（pp-06/pp-09）；slide/layout themeOverride
  复用（pp-07）；`HandlerCallbackImpl`/`UnifiedXmlReader` 实现（pp-07）；`PptxReader` 整链（pp-09）。

## 验收

FUNCTIONS.md §4 三条锁死：

1. ThemeHandler 对已知 theme 子元素返回非 None kind。
2. PptxrTheme.open：null part → null；parse 失败 → null（Java 吞 Throwable）。
3. PptxrThemes：同一 partName 两次 open 同一 theme；不同 partName 两次实例。

## 依赖

- **depends：** pp-01-pptx-parse（关系/part 抽象与解析器基础契约）；pp-03-dml-style（`ColorMapHandler`
  等分发目标已就位）；`kfc/opcr` 关系接口语义（本层以 fixture/内存 stream 替身）。
- **does not depend：** pp-07（`HandlerCallbackImpl`/`UnifiedXmlReader` 用 fixture）；pp-09（`PptxReader`）。

## 不过扩展

禁止跑 `PptxReader` 整链冒充本 feature。theme XML 的 scheme 全表（`clrScheme`/`fontScheme`/`fmtScheme`）
可随 `ThemeHandler` 一轮改写分发命中，但「命中/未命中」子元素语义是另一 I/O，禁止把未验收的子 handler
全实现塞进本 feature 顺手做。禁止改 pp-01/pp-02 已 pass 语义。