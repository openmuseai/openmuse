# pp-08-slides-walk — rId 装配与阅读顺序

> depends_on: `pp-06-presentation-xml`（rId 列表）；`pp-07-part-open`（open 实现）  
> 状态：**pass**

## §0 管线位置

```text
PresentationHandlerCallback.onSlideIds(rids)
       ↓ 本 feature
PptxrSlides.onSlideIds → 按 UnReadSlides.getCreateSlideCount 预创建 KmoSlide
onSlideMasterIds → mMastersReader + _addMaster
PptxrSlides.open:
  next(priorityList) → getMasterReader(layout rel → master partName)
  master.open + openLayout + slide.setLayout
  slide.open + slideDidFinishReadingEverything
  首屏 delayCreateSlides + resetSlides
  其余 master.openAllDatas
  空 slide 列表：changeLayout(slideAt(0), default layout)
```

不解析 XML。notes/handout master open 在 Java 末尾；MVP 可用「无 notes master → buildDefaultNotesMaster」一条测试，不实现 NotesMaster 全文。

## §1 function 清单

| order | function_id | I | O | upstream | downstream |
|---|---|---|---|---|---|
| 1 | `01_function_slide_id_map` | initialize；fileName；relationship target | static `mSlideIdMap`；target basename | — | 02；pp-06 customShow |
| 2 | `02_function_on_ids` | rId lists + fake rels/parts | mSlidesReader/mMastersReader；selection.init | 01 | 03 |
| 3 | `03_function_unread_walk` | UnReadSlides + open() 循环 | master→layout→slide 顺序；delayCreate；ForceQuit | 02；pp-07 | pp-09 |
| 4 | `04_function_change_layout` | empty slide + layout phs | copy placeholder shapes；hf 4/5/6 条件 | pp-02 ph 元数据 | 空文档路径 |

## §2 数据流

```text
slideRids ── create first N slides (N=getCreateSlideCount)
open loop ── getMasterReader: slide.layoutRels → layout.rels slideMaster type → mMastersReader.get(partName)
null master → buildDefaultSlideMaster + buildDefaultLayout
```

## §3 测试面

- Fake `OpcrRelationships.getRelationship`
- Recording `KmoPresOpenEventListener`（initializing / slideDidFinish）
- `OPLogDumper_FirstSlideIOFinished` 短路：测一次即 return

## §4 验收

1. 缺 rel 的 rId 不 addSlide。
2. `getRelationShipTargetFileName(null)→""`；无 `/` 用整段 target。
3. open：layout 无 master rel → 默认 master 路径。
4. changeLayout：非 type=1 或不 isPlaceHolder 不 copy。

## §5 loop

`01 → 02 → 03 → 04`

## 文档

- [functions.csv](./functions.csv)
