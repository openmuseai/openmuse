# pp-06-presentation-xml — p:presentation 根 XML

> depends_on: `pp-01-pptx-parse`；`pp-03-dml-style`（defaultTextStyle → TextListStyleHandler）  
> 状态：**01–04 pass**

## §0 管线位置

```text
PptxrPresentation.openGlobalData
  UnifiedXmlReader.read(presPart stream, PresentationHandler)
       ↓ 本 feature
Presentation 根属性 + sldSz/notesSz + sldIdLst/sldMasterIdLst 回调
PresentationHandlerCallback.onSlideIds / onSlideMasterIds / …
       ↓
pp-08 PptxrSlides.onSlideIds（本层只验回调参数，不创建 KmoSlide）
```

不打开 zip。`ModifyVerifierHandler` 不进改写列表（加密）。

## §1 function 清单

| order | function_id | I | O | upstream | downstream |
|---|---|---|---|---|---|
| 1 | `01_function_presentation_attrs` | p:presentation attrs + endElement | serverZoom/firstSlideNum/…/conformance + updateProtocol | SWReaderMap.conformanceClassMap | 02；pp-09 |
| 2 | `02_function_id_lists` | sldIdLst/sldMasterIdLst/notes/handout 子元素 | `onSlideIds` 等 rId `List<String>` | PresentationHandlerCallback | pp-08 |
| 3 | `03_function_slide_notes_size` | sldSz attrs；notesSz → PositiveSize2DHandler | cx/cy/type；16:9 魔数改 type=15 | SWReaderMap.slideSizeMap | pp-09 addDefaultProperty |
| 4 | `04_function_photo_kinsoku_custshow` | photoAlbum/kinsoku/custShow 子树 | photoAlbum 字段；kinsoku 字符串；custShow 经 callback.onCustomShow | SWReaderMap photo 表 | pp-08 getSlideIdByFileNameNotAdd |

Embedded font list + font data rId 回调可随 04 或暂缓 constructor fixture。

## §2 数据流

```text
attrs loop (count+getAttrAt) → Presentation.put*
3211509/3211512 → SlideAndMasterIdListHandler → onSlideMasterIds / onSlideIds
3211510/3211511 → NotesAndHandoutMasterIdListHandler
endElement → mPres.updateProtocol(presData.save()) 当 mPres != null
```

## §3 测试面

- src：`show/pptxr/handler/PresentationHandler.rs`（含 nested）
- Recording `PresentationHandlerCallback`

## §4 验收

1. 空 attrs 不改 create 默认。
2. sldIdLst end 才回调；enterSub 返回 this。
3. notes/handout id 缺 4259841：Java NPE 语义在测试中用合法 attrs（不发明更安全的 Rust）。
4. 16:9 12192000×6858000 → type 15。

## §5 loop

`01 → 02 → 03 → 04`

## 文档

- [functions.csv](./functions.csv)
