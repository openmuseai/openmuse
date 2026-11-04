# PPTX parse MVP 管线裁决

> 审计日期：2026-08-29  
> 依据：`PptxReader.java`、`PptxrPresentation.java`、`PptxrSlides.java`、`PptxrSlide`/`PptxrLayout`/`PptxrMaster`、`PresentationHandler.java`、现有 pp-01/pp-02 FUNCTIONS.md

## 1. 已落地（不要重拆）

| feature | I | O | 缺口（故意） |
|---|---|---|---|
| pp-01 | 内存 XML 事件 + model fixture | handler 分发 + protocol 字段 | 不打开 `OpcrPackagePart`；timing/clrMap/txStyles 子树用 constructor fixture |
| pp-02 | spTree 事件 + preallocated shape | DgStore 行 + 有序树 | 不装配 slide；graphic-frame 业务体仍 fixture |

pp-01 的 03 与 pp-02 的 01 共用 `SwNvPrHandler` 源码，回归时跑两个 `[[test]]` name。

旧 README 把剩余工作叫 `pp-03-slide-parts` + `pp-04-package-orchestration`。那是占位名：真实 Java 在 **theme part、presentation.xml、part open、slides walk、PptxReader** 上有独立 I/O，不能并成两层。

## 2. MVP 必须新增的 I/O 层

| feature | 为什么是独立 feature | 不放进本层 |
|---|---|---|
| pp-03-dml-style | 01 的 master/slide 已 `return new ColorMapHandler` / `TextListStyleHandler`；测的是 DML 属性写入，不是 part | Theme part、fill 全图 |
| pp-04-theme-part | `openGlobalData` 在读 presentation.xml **之前**用 rel 打开 theme；cache key 是 `partName` | theme XML 内部 scheme 全表可随 ThemeHandler 一轮改写，但 **PptxrThemes 命中/未命中** 是另一 I/O |
| pp-05-timing-transition | 01 的 05–07 已按 element id 构造；缺了则永远 fixture | 27 个 timing 行为 handler（animate*）——打开文档不依赖写全动画树 |
| pp-06-presentation-xml | `presentation.xml` 事件写入 `Presentation` 并 **回调 rId 列表**；这是 08 的输入 | `ModifyVerifier` 写密码循环（加密 feature） |
| pp-07-part-open | `InputStream → UnifiedXmlReader.read → *Handler + callback.load()` | chart/vml/diagram/notes 后置 open |
| pp-08-slides-walk | rId → 创建 `KmoSlide`/`KmoSlideMaster`；`open()` 顺序 master→layout→slide；空列表默认 master | 不重读 XML |
| pp-09-pptx-open | package + `getPresPart` 后缀校验 + 打开顺序 + close | decrypt、OLE install、custom xml、comment authors |
| pp-12-opc-package | **真实文件 → part/流**的桥接：Java `OxfrPackage`（WPS 私有 OPC 封装）+ `PptxReader.getPresPart`（FileDamaged/SuffixError）没有 Rust 对应；OPC 本体（zip/rels/CT）已在 core-libs wave0 pass，本 feature 只做 PPT 侧封装与定位 | 不再抄一份 OPC、不碰 `PptxrFileDecrypt`（加密）、不改 wave0 语义 |
| pp-13-document-assembly | 跨 feature 的文档级输出：presentation rId 页序 → 每页结构树 → layout/master 继承与绝对几何 → `PptxDocument` | 不解析文字效果、不序列化 FFI；后者归 pr-04 |
| pp-14-layout-master-bind | **产品**把 slide.rels 的 layout/master XML 和 nvPr 接到 `KmoSlide`（叶级 open 已 pass 但 pptx_mvp 未绑） | 不重写 PlaceHolderMerger |
| pp-15-pic-blip-media | **产品** `p:pic` 兄弟节点 `blipFill` + zip media 字节 | 不解码 JPEG（pr-05） |
| pp-16-scheme-color-resolve | theme 表 + schemeClr/srgb 写成 ARGB | 渐变/效果层 |

## 3. 明确移出 MVP（附录，先不建 functions/）

这些在 Java `PptxrSlide.open` / `PptxrPresentation.openOtherGlobalData` 上存在，但 **空白/普通 pptx 首屏不依赖**：

- `PptxrNotesSlide` / `PptxrNotesMaster` / `PptxrHandoutMaster`
- `PptxrChart` / `PptxrDiagramParts` / `PptxrVmlDrawing`
- `PptxrComment` / `PptxrCommentAuthors`
- `PptxrItem` / `PptxrItemProps` / `PptxrCustom`
- `PptxrFileDecrypt` / `ModifyVerifierHandler`
- `PptxrTableStyles` 全量（`openGlobalData` 有 rel 则后续可 fixture skip）
- timing 子树：`Animate*BehaviorHandler` 等（pp-05 只锁列表 end 写入）

以后按同样 I/O 规则加附录 feature，不要塞进 pp-09。**渲染阻塞的 DML 已占用 pp-10/pp-11**（spPr / txBody），不要把 notes/chart 再编号成 pp-10。

## 3b. 渲染阻塞 vs 打开器（2026-08-29 修订）

现有 pp-01..09 把终点设成 `PptxReader.open`。对照 writer：那只相当于 **wc-05**，后面还有 **wr-01..08**。

PPT Java 绘制不读 timing，但 **必须** 有：

- `spPr.xfrm` / fill / ln（pp-02 目前只分发、未锁定写入）→ **pp-10**
- `txBody`（有字时）→ **pp-11**
- 排版：`PlaceHolderMerger` + `ShowTextLayout` → `layout/pl-*`
- 绘制：`SlideRender.drawUnderSlide` 三层胶片 → `render/pr-*`

`pp-05` 从「parse MVP 完整打开」可以留；从「MVP 跑通渲染」里 **降级为非关键路径**。


## 4. 与 word-core 的对应

| word | ppt |
|---|---|
| wave0 OPC/XML | core-libs + `UnifiedXmlReader`（pp-07） |
| wc-03/04 内容写入 | pp-01 + pp-02 + **pp-10 spPr + pp-11 txBody** |
| wc-05 `DocxReader.read` | pp-09 `PptxReader.open` |
| wr-01..06 layout | `layout/pl-*`（几何 + 框内文本，无正文分页） |
| wr-07..08 render | `render/pr-*` |
| styles 先于正文 | theme 先于 `presentation.xml` |

## 5. 现有文档要改的指针

- pp-01 `05/06/07` csv `downstream`：原 `pp-04-package` → `pp-07-part-open`
- pp-02 `05` csv `downstream`：原 `pp-03-slide-parts` → `pp-07-part-open`（part open 消费 spTree 结果）
