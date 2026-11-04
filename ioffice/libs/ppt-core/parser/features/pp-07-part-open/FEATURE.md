# pp-07-part-open — slide / layout / master part 读入

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（`01`–`05` 均 `pass`）
> 依赖：pp-01（handler / callback，已 pass）；pp-02（spTree）；pp-04（theme cache）

## 目标

**把已有 XML handler 接到 part stream**：`PptxrSlide.open` / `PptxrLayout.open` / `PptxrMaster.open`
拿到 `OpcrPackagePart` 后，用 `UnifiedXmlReader.read(stream, handler)` 把 slide / layout / master XML
填进 Kmo 模型，随后 `callback.load()` 收尾（connector 重映射）与 `slide.updateFieldText()`。
**不创建 slide 列表**（pp-08），**不打开 package**（pp-09）；notes / comment / chart / vml / diagram
一律不读（post-MVP）。

## I/O（锁死）

| function | I | O |
|---|---|---|
| `01 unified_xml_reader` | `InputStream`/`path` + `ElementHandler` | `HLXmlReader.parser` 恰好一次（三种 overload 转发）；IOException 上抛 |
| `02 handler_callback_impl` | rels + mediaLib/inkLib + mode | `load()` 重映射 connector；blip/media rId→mediaId 落点；chartMap/diagram 只收集；`onInk` 等 XML 期副作用 |
| `03 pptx_slide_open` | `KmoSlide` + part | XML 填 slide + `callback.load()` + `slide.updateFieldText()`；null part / XML Throwable 不抛给调用方 |
| `04 pptx_layout_open` | `KmoSlideLayout` + part | `bOpen` 幂等 + themeOverride rel + XML 填 layout |
| `05 pptx_master_open` | `KmoSlideMaster` + part | 缺 part → `buildDefaultSlideMaster`；`onSlideLayoutIds` 为每个 rId 建 layout reader |

## Java 对照

| Java | function | 本 feature 承担 |
|---|---|---|
| `pptxr.util.UnifiedXmlReader` | 01 | 三个静态 `read` overload；每次 `new HLXmlReader(MapManager.getInstance(), handler)` 后 `parser` 一次 |
| `ppt.slides.HandlerCallbackImpl` | 02 | `load()`→`updateConnectorSite()`；XML 期回调 `onBlipEmbed/onBlipLink/onAudioFileLink/onExtMediaFileLink/onQuickTimeFileLink/onVideoFileLink/onEmbeddedWAVAudioFileEmbed/onHyperlinkRid/onHyperlinkAction/onShapeId/onConnectorShape/onOleObject/onChartRelationShip/addDiagramPair/onInk/onModel3D/onTags`；`SlideHandlerCallbackImpl` 逐方法转发 |
| `ppt.slides.PptxrSlide` | 03 | `open()`：themeOverride → XML → `callback.load()` → `mSlide.updateFieldText()` |
| `ppt.slidelayouts.PptxrLayout` | 04 | `open()`：`bOpen` 幂等 + rels/themeOverride + XML+load；`buildDefaultLayout` |
| `ppt.slidemasters.PptxrMaster` | 05 | `open()`：theme rel vs default theme + XML+load；`onSlideLayoutIds`/`openLayout`/`openAllDatas`/`buildDefaultSlideMaster` |
| `PptxrNotesSlide`/`PptxrComment`/`PptxrChart`/`PptxrVmlDrawing`/`PptxrDiagramParts` | — | **移出**（post-MVP，禁止调用） |

## 必须 / 壳 / 延后

- **must：**
  - 01：stream overload + path overload 忠于 Java；path+begin/end+isBeginAtRoot overload 可 ignore（FUNCTIONS.md §4 验收 1 允许）
  - 02：`load()`=connector 重映射（`updateConnectorSite`）；blip/media rId→mediaId（`mMedias.openMedia`）；chartMap/diagram 只收集不 open
  - 03：XML Throwable → IOLog + return（不抛）；null part 空转
  - 04：`bOpen` 先置位（rels Throwable 早退后仍为 true，幂等）；themeOverride rel 经 pp-04 写 layout
  - 05：缺 part → `buildDefaultSlideMaster`；`onSlideLayoutIds` 建 `PptxrLayout` reader 并 `mMaster._addLayout`
- **stub：** 03 的 `mPresentation.selection().activedSlide().getThemeOverride()` else 分支（依赖 pp-06/pp-09 selection）；02 的 `onInk`（ink 模型，先记 rId 或回 -1）；ppt14 模式分支
- **defer：** notes / comment / chart / vml / diagram；`PptxrLayout` 静态 `open(xmlpath/xmlstream)` 与 `PptxrMaster` 静态 `open(pres, SlideMaster, part, medias)`（TextStylesHandler 路径）

## 范围内 / 范围外

- **in：** themeOverride / theme rel 分支（03/04/05 各自经 pp-04 的 `PptxrThemes.open` cache）；connector 重映射；media rId→mediaId
- **out：** 创建 slide 列表（pp-08）；打开 package / ZIP（pp-09）；读 notes/comment/chart/vml/diagram；实现 XML 引擎（HLXmlReader 契约外部化）

## 验收（FUNCTIONS.md §4）

1. UnifiedXmlReader 三种 overload 都接到 HLXmlReader（path 切片 overload 可 ignore 直到需要）。
2. Slide open：XML 失败不抛给调用方。
3. Master `bOpen`：第二次 open 不重复 parse。
4. 不调用 chart/vml/notes（断言副作用计数为 0）。

## 依赖

- **depends：** pp-01 `05`/`06`/`07` 的 `SlideHandler` / `SlideLayoutHandler` / `SlideMasterHandler`（`ElementHandler` 实现）与 `02` `HandlerCallback` 基类（均 pass）；pp-02 spTree 子 handler（`CommonSlideDataHandler` 消费）；pp-04 `PptxrThemes.open`（03/04 themeOverride、05 theme rel）。
- **does not depend：** pp-08（slide 列表）、pp-09（package）、notes/chart/vml/comment/diagram part。

## 不过扩展

- 禁止调用 `PptxrNotesSlide` / `PptxrComment` / `PptxrChart` / `PptxrVmlDrawing` / `PptxrDiagramParts`。
- 禁止在 01 里实现 XML 引擎（HLXmlReader / HLElementReader 是外部契约，不改 pp-01 pass 的 handler 语义）。
- 禁止顺手创建 slide 列表或封装 package 打开（pp-08/pp-09 范畴），即使 `onHyperlinkRid` 需要 slideId 反查。