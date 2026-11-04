# wr-29-java-pipeline-cutover

> status: **wired**（`01`-`07` pass；`08` wrap 9/9；`09` docGrid 6/6，WPS p1 分页对齐进行中）  
> 里程碑：[rewrite-review/milestones/layout-java-pipeline](../../rewrite-review/milestones/layout-java-pipeline/README.md)  
> 上游：**P0 pass**（`w:br` → rope `'\f'`，`multipage` `page_count=2`）  
> 切分：[FUNCTIONS.md](./FUNCTIONS.md) · [functions.csv](./functions.csv)

**I：** 已装配的 `TextDocument` 主故事（rope/CHPX/PAPX）+ 真实 `'\f'`。

**进展（09 wired）**：`w:docGrid type=lines linePitch=312` 经 `SectPrHandler`/`convertDocumentGrid` 进 sepx，
`SectionTool.calcGrid` 写入 `PageEnv`；正文行高 468；产品 `page_count=8`（与 WPS 金图页数一致）；
WPS p1 的「三、」已到第 2 页。页眉「毕业论文」仍 STAGE6。

**进展（07 pass）**：真实 docx 端到端 — hello / App hello.docx / two-para / multipage / sample2 产品 Frame 绿；
`consumeGraphsBefore` 推进 CP（不再重复标题行）；PAPX `getInt(193)` + `AlignCenterHandler.adjustLine` 居中标题；
显式 `'\f'` 不做寡妇合并。FFI `p2_ffi`/`p5_hash`；Flutter `screenshot_hello_test` 3/3（sample2 标题 x>1800）；
`build-macos.sh` 产出 `docx_mvp_render.app`。

**进展（06 pass）**：产品 `layout_document` / `layout_text_document` 默认 Java 栈（J1–J8 8/8）：
`GraphBuffFactory.createGraphBuff` → prepareGraphs → createNodes → doFlowTextLine → LineLayouter.flow
→ TypoTextLine → paginate（`'\f'` → force_page_after）→ DrawText；`attach_pageview_hot_path` 已删；
默认路径不调 `env::layout_lines`（Stage2 仅 `layout_document_stage2`）。hello x=1800 size=240；
multipage `page_count≥2`。

**进展（05 pass）**：`05_function_flow_text_line` 完成——LineLayouter.flow（preCalcLineHeight→
prepareSubLineEnvs→wrap→SubLineLayouter.flow→regulateToCache 行写入 curPage.lines）+ TextLayouter.
flowTextLine/doFlowTextLine（prepareGraphs/空段/行回读/finishPage）全真实；验收：hello ≥1
TypoTextLine（LL1 行写入、TL1 行回读推进，startCP/endCP 精确）。06 产品切线（layout_document
默认 Java 栈）：子 layouters env 以 MainLayouter 真实装配重建 + Java 行 → PageLine/TypoTextLine
→ PaginatedPage → snapshot_from_pages。

**进展（05 loop）**：`LineHeightLayouter` 真实（467 行）+ `GraphAlignTool` 对齐度量 + `ILineHeightCallback`
真实 trait（LH1-3 绿，行高=Java 公式精确）；**snapshot 数据面真实**（flat heap alloc/get/set、
`TypoList`/`TypoObjBase`/`TypoTextLine.alloc`+行字段 statics，TL1-2/OB1 精确断言）——05 行写入
与 doFlowTextLine 行读取的奠基层已备；flow 编排链（LineLayouter/SubLineLayouter/flowTextLine）下一批。

**进展（04 pass）**：`GraphFactory` 真实移植（createNodes/parseChar/parseAscii/addSpaceGraph +
WestLayout 字母合并 + LangLayout calcWidths 量宽）；字宽走 FontHost/FontStack（wr-11 度量同式：
宽 = hmtx×fontSize/srcEmSize，roundError 逐字累积）；`cache/style/TypoFontStyle`、env
`FontStyleFactory.create/createTypeface`、`FontStyleCache/Factory` 真实接线；`WriterTypeface`
实现 `cn.wps.font.ITypeface` trait；hello `"Hello World!"` → 4 图（WestLayout 合并）且宽度
= 逐字 hmtx×200/upe 和（GF2 证明）；`CharParse/CharMap` 登记 + CHARMAP_DEFERRED 文档化。

**进展（03 pass）**：`GraphReader` 真实移植（doPrepareGraphs/readCoreNode/readRun/readBreak/commitReadText 等）；
`LayoutState` 的 KPList 由假 PhantomData 换成真实节点存储（GR-D1，Vec 池 + Java 同方法面）；
`TypoPara` 补 allocGraphs/putChars/putChar/hasAlloc/getCharBlock；env `PropertyTool`+`CachedPropertySet`+
`PropertyToolFactory` 真实接线（纯文本合并级联：defaults→paraStyle→para→runStyle→run，编号/修订 GR-D3 DEFERRED）；
`IPropertyTool` 门面收尾（Entry 重命名 + &mut self）；`LayoutState` 去 StyleResult/BidiTool/hasDrawing/
hasBidiGraph/hasCoreCharsContent 影子；`initFilter` 对 hello 不 panic；`gvml/enml` 不动。
hello `prepareGraphs` → charsList≥1 且每节点带 finalRunProp、`mCurCP` 达正文尾（GR3 单测证明）。  
**O：** 产品 `layout_document` 默认走 Java  
`FlowLayout.createDocLayouter(PAGEVIEW)` → `PageViewLayouter.flowToPage` → `TextLayouter.flow` → `LineLayouter.flow` → `TypoSnapshot` → `RenderCommand`；hello / sample2 / multipage 端到端可画。

不重做：wr-12 叶子折行公式、wr-21 `ParaLayouter.start`、P0 `BrHandler`。  
不解：`flowTable` / `flowTextFrame` / drawing / hdrftr（STAGE4–6）。

叶子 wr-21..24 csv `pass` **不算本 feature 完成**：产品入口仍丢掉 `flowToPage`。本 feature 以 G-cut / G-L2 / G-app 为准。
