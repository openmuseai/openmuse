# pp-08-slides-walk — rId 装配与阅读顺序

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（4 function，均 pass）
> 依赖：pp-06-presentation-xml；pp-07-part-open

## 目标

`PresentationHandlerCallback.onSlideIds` → `PptxrSlides` 装配与阅读顺序：按
`UnReadSlides.getCreateSlideCount` 预创建 KmoSlide 壳（不解析 slide XML），读 master/layout rel 建
`mMastersReader`，`open()` 按 master→layout→slide 顺序打开、首屏 `delayCreateSlides` + `resetSlides`，
空 slide 列表走默认 layout 的 `changeLayout(slideAt(0), layout)`。本 feature **不解析任何 XML 内容**
——presentation.xml 回调与 part open/解析分属 pp-06 / pp-07。

## I/O（锁死）

| function | I | O |
|---|---|---|
| `01_function_slide_id_map` | initialize；fileName；relationship target | static `mSlideIdMap`；target basename |
| `02_function_on_ids` | rId lists + fake rels/parts | mSlidesReader/mMastersReader；selection.init |
| `03_function_unread_walk` | UnReadSlides + open() 循环 | master→layout→slide 顺序；delayCreate；ForceQuit |
| `04_function_change_layout` | empty slide + layout phs | copy placeholder shapes；hf 4/5/6 条件 |

## Java 对照

`PptxrSlides.java`（583 行）+ 其静态内嵌类 `UnReadSlides`（第 407 行起，非独立文件）：

- 01：`initialize`、`getSlideIdByFileName`、`getRelationShipTargetFileName`（均 static）。
- 02：`onSlideIds`、`onSlideMasterIds`、`onNotesMasterIds`、`onHandoutMasterIds`。
- 03：`open`、`getMasterReader`、`delayCreateSlides`、`createSlidesByFileName`、`UnReadSlides.next`。
- 04：`changeLayout`、`shapeNeedCopyFromLayout`、`shapeNeedClearContentText`（均 static）。

入口链路：`PresentationHandlerCallback.onSlideIds(List<String>)`（默认空实现）→
`PresentationHandlerCallbackImpl.onSlideIds`（`mSlidesReader` 非 null 才转发）→
`PptxrSlides.onSlideIds`；`PptxrPresentation.open()` 构造 `PptxrSlides` 后先调一次
`PptxrSlides.initialize()`。

notes/handout master 的 open 在 Java `open()` **末尾**（无 notes master → `buildDefaultNotesMaster` +
`setNoteMaster`）；MVP 只保留「无 notes master → buildDefaultNotesMaster」一条默认测试，不实现
NotesMaster 全文。

## 必须 / 壳 / 延后

- **must：** onSlideIds 预创建（`getCreateSlideCount` 截断）+ 缺 rel 的 rId 不 addSlide；`mSlideIdMap`
  读/写与 basename 规则（含 `getRelationShipTargetFileName(null)→""`）；open 顺序
  master→layout→slide、无 master 走默认 master/layout；首屏 `delayCreateSlides` + `resetSlides` +
  `onSlideListChanged`；`selection.init`；空列表 `changeLayout(slideAt(0), default layout)`；
  changeLayout 的 `type==1 && isPlaceHolder` 与 hf 4/5/6 条件。
- **stub：** `UnReadSlides.next` 的 priorityList 首项优先（selection priority 未全接线时先行 active
  分值序列）；`mMedias.priorityReadMedia` 记桩。
- **defer：** NotesMaster/HandoutMaster 全文；`getSlideIdByFileNameNotAdd` 的 customShow 完整链路。

## 范围内 / 范围外

- **in：** KmoSlide 壳创建与 `mSlideIdMap` 装配；layout rels → master 解析与默认 fallback；阅读顺序。
- **out：** 解析 slide/layout/master 的 XML 内容（pp-07 职责）；NotesMaster 全文；修改已 pass 的
  pp-01/pp-02 语义。

## 验收

见 [FUNCTIONS.md §4](./FUNCTIONS.md)：

1. 缺 rel 的 rId 不 addSlide。
2. `getRelationShipTargetFileName(null)→""`；无 `/` 用整段 target。
3. open：layout 无 master rel → 默认 master 路径。
4. changeLayout：非 type=1 或不 isPlaceHolder 不 copy。

## 依赖

- **depends：** pp-06-presentation-xml（sldIdLst/sldMasterIdLst 的 rId 列表回调）；
  pp-07-part-open（PptxrSlide/Layout/Master.open 与 part/rels 读取）；pp-02（ph 元数据，changeLayout
  copy 判定）。
- **does not depend：** pp-05 timing/transition；pp-09 编排（本 feature 是 pp-09 的上游）。

## 不过扩展

禁止重读 XML（slide/layout/master 内容归 pp-07）；禁止实现 NotesMaster 全文；禁止改
functions.csv / FUNCTIONS.md / manifest.csv 的状态列；禁止把 `UnReadSlides.next` 的分值顺序
换成正序遍历冒充本 feature 通过；KmoSlide 的 XML 字段解析不塞进「预创建壳」路径。