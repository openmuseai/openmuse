# 05_function_flow_text_line

| | |
|---|---|
| Java 权威 | `layout/base/TextLayouter.java`（1389 行，flowTextLine/doFlowTextLine）＋ `line/LineLayouter.java`（1029，flow）＋ `line/SubLineLayouter.java`（508）＋ `line/SubLineLayouterBase.java`（486）＋ `line/LineHeightLayouter.java`（467） |
| 入口 | `TextLayouter.flow`（已接线）→ `flowTextLine` → `LineLayouter.flow` → `SubLineLayouter.flow` → snapshot 行写入 |
| 状态 | **pass**（2026-10，wr-29 05 完成） |

## 1. I / O

- **I**：非空 `graphList`（有量宽的 Graphs，wr-29 04 产出）＋ `layoutState`（page/para env、linePos 等）。
- **O**：`TypoSnapshot` 中的 ≥1 `TypoTextLine`（hello）；`layoutState` 行推进（cp/linePos/curLayoutHeight）。

## 2. 本 function 落地记录（wr-29 05，loop）

**真实（第七批收官：TextLayouter 外层编排）**：
- `TextLayouter.flowTextLine` 真实（prepareGraphs → hasGraphs（autoNum 清图）→ 空段/行段分支）：
  空段分支（修订 237 收集（hello 无）、`onEmptyGraphSize` 真实：pageEndLimit 判/段尾推进/
  initGraphBuff vs setFinishPage+clear）；行段分支 → `doFlowTextLine`。
- `TextLayouter.doFlowTextLine` 真实：mLineLayouter.flow（已备）→ isParaEnd（mParaLayouter.end
  wr-21，MainLayouter 接线前 None 安全）→ validLine 判 → **行读取回读**（mirror：
  curPage.lines[linePos]：top/height/lineAfter/lines 数字 → curLayoutHeight = 行底+lineAfter、
  startCP/endCP）→ lineFitsPageLimit（cp≤pageEndLimitCP）→ linePos++ → needFinishPageForFootNotes
  （false）/cp==pageEndLimit → finishPage__int（isLastLineOfPage && finishPage(false)）。
- `prepareGraphs` 真实（wordAlingFont + mGraphBuff.prepareGraphs + prepareEmbeds（GR-D3））；
  `flowFootEndNotes`/`needFinishPageForFootNotes` 真实（恒 false）；ctor 装配 mLineLayouter/mGraphBuff
  （默认 env 实例，MainLayouter 全链路 06 重建）。

**测试**（05 目标 14/14 绿，TL1 为收官新增）：

| 用例 | 断言 |
|------|------|
| **TL1** | **05 外层编排**：doFlowTextLine → LineLayouter.flow → 行回读 —— linePos 0→1、
      行宽/高推进（curLayoutHeight>0）、line startCP=0/endCP=12、不 finishPage、validLine |
| LL1 | 05 验收行写入（hello ≥1 TypoTextLine） |
| SL1/SL2/LH1-3/TL1-2/OB1/LSB1-2/WH1/AL1 | 全链叶子（行高/数据面/消费/handler/subline） |

**真实（第六批：LineLayouter.flow 端到端 —— hello ≥1 TypoTextLine）**：
- `line/LineLayouter.rs` 真实重写（替换 1893 行影子岛）：`flow(layoutState)` 全编排——
  field 复位 → `preCalcLineHeight`（calcParaBefore + LineHeightLayouter 回调（ILineHeightCallback 实现））
  → `isOverflowBeforeFlow`（pageEnv 高度门槛 + TypoPage.lines 对等）→ `prepareSubLineEnvs`
  （无绘绕排 → 单 subline：left=sectionMarginOffset、right=coreWidth+offset）→ wrap 循环
  （SubLineLayouter.flow + valid/计数/firstValid/lastGraphIndex）→ `postCalcLineHeight`
  （updateLineHeight + calcParaAfter）→ `regulateToTextLine`（行对象：para/textRectTop·Height/
  ascDesPos/首末行旗标/graphIndexStart·End）→ **`regulateToCache`**：curLayoutHeight 推进、
  line.setTop/Height/LineBefore·After、**镜像行推入 `TypoPage.lines`**（Java curPage.insertLine
  对等）+ start/endCP 换算（getNextCP）。
- 支撑：`KPList.get_at`（索引迭代）、`TypoPage.lines_len/insert_line/lines_first`、`TypoPara.getPointer`
  （镜像 para 序号）、TypoTextLine setTop/setHeight/setLineBefore·AfterVal/setCardBegin·End mirror；
  LineLayouter 默认装配真实 WhiteSpaceHandler + AlignLeft（jc=0）。
- mTextWrapLayouter None（hello 无绘绕排）；card/paraSHD/自动编号 GR-D3。

**测试**（05 目标 13/13 绿，LL1 为本周新增）：

| 用例 | 断言 |
|------|------|
| **LL1** | **05 验收：hello ≥1 TypoTextLine** —— `validLine=true`、`curPage.lines==1`、
      `startCP=0`、`endCP=12`（getNextCP 换算）、行高>0 |
| SL1/SL2 | SubLineLayouter.flow fit/空游标（上周） |

**真实（第五批：SubLineLayouter.flow —— 子行 run 循环）**：
- `line/SubLineLayouter.rs` 真实（Java 508 行）：`flow(SubLineEnv, firstSubLine, layoutState)`
  —— indent/layoutLeft·Right 装配 → run 循环（runType 分派：空格(2/17)/页断(5)/段断(3,4)/
  标点(18)/软·不间断连字符(28,29)/自动编号(20-27 GR-D3)）→ 空格 WhiteSpaceHandler.handleSpace
  决策 → prevRun/addRun（位置累加 offsetXTo + mMaxWidth 溢出）→ 行尾 endLine 收尾；
  `doAddRun`/`addRun`/`tryTransForSpace`/`doAddTab`（tab GR-D3 停车面真实）/
  `needBreakLineOnShowCardInfo`（恒 false）/`flowAutoNum`（停车）。
- 支撑：`LayoutState` **GraphsIt 真实游标**（cursor + seekBegin/seek/value/advance）+ `get()`/
  `getNextSectionBreak()` 真实；`TypoPara.putWidth` 新增；`AlignLeftHandler` 实现 IHAlignHandler trait；
  Base 去 IHAlignHandler/TabHandler 影子（真实 trait/类型）；`TabHandler` 类型面真实（tab 非 hello
  停车）；`wordbreaker/HyphenFlag` 登记。

**测试**（05 目标 12/12 绿，SL 为本周新增）：

| 用例 | 断言 |
|------|------|
| SL1 | "Hello World!" 全图收入单子行：first=('Hello' rt1 gi0 len5)、last=('!' rt18)、
      位置累加 last right>0 且未溢出；mContinueFlow=true |
| SL2 | 空游标 → 空流返回、子行无图 |

**真实（第四批：handler 层 —— SubLineLayouter.flow 前置依赖）**：
- `charSpace/WhiteSpaceHandler`（Java 178 行）：handleSpace（diffSBMB/wordWrap 分支、半角/
  全角空格）、setCompactWidth（压半字宽+saveOrgWidth 快照+fillWidths）、setCompressableWidth
  （可压缩宽度写入 LineCompressEnv.slot）、isBetweenLetters/isLetterChar/isLetterGraphs；
  `getWordWrapWhiteSpaceMinWidth` 用真实字体空格比率（WriterTypeface.getWriterLayoutCharSpaceRatio
  = space hmtx/upe）。
- `align/AlignBaseHandler`（143）+ `AlignLeftHandler`（39）+ `IHAlignHandler` trait（28）：
  setSubLineEnv（layoutLeft/right、压缩和）、onAddRun/alignRightGraphs 追踪、adjustRun、
  adjustLine（行盒 left/width=lastRun.right-压缩和）、calcLineLeftAndRight。
- `LayoutState.OrgWidthCache` 真实（save/restore/clear，压缩前字宽快照）；`saveOrgWidth` 解除。
- `SubLineEnv` ctor real + `hasGraphs`/`addGraphs`（first/last 引用镜像）+ `alignRightGraphs` 指针化；
  `TypoRectCoreImpl` setLeft/setWidth/setHeight/getLeft/width 真实。
- 注册：lib.rs `line::align`、charSpace/iWhiteSpaceHandler。

**测试**（05 目标 10/10 绿，WH/AL 为本周新增）：

| 用例 | 断言 |
|------|------|
| WH1 | diffSBMB+!wordWrap 空格压半字宽（10pt→100twip 精确）；setEnv 读 paraEnv/docEnv |
| AL1 | AlignLeft.adjustLine：行盒 left=layoutLeft、width=lastRun.right（压缩和 0） |

**真实（第三批：图消费推进）**：
- `LayoutState.consumeGraphsBefore` 真实（Java LayoutState.java:513-577）：clearWitchCache →
  mFirstGraphIndex/cp（`typoPara.getNextCP`）→ graphList 裁剪（含 next 分支的 src/chars 联动与
  else 分支 srcNode 归并）→ mGraphsIt.seek/cleanDrawPosOffsetBefore；`hasDrawing`/`addBidi`/BidiTool 恢复。
- `TypoPara` **CP→graph 映射 mirror**：`cp_map` + `putMap`（Java GraphBuff.addMap 1:1 段合并语义）+
  `getNextCP(graphIndex)`（Java TypoPara.java:585-611 逐行语义，cpRef=0）；`GraphReader.commitReadText`
  经 putMap 同步维护（Java 存 para snapshot 对等）。
- `GraphsIt.seek`/`cleanDrawPosOffsetBefore`/`LayoutState.addBidi` 真实（hello 空/安全路径）。

**测试**（05 目标 8/8 绿，LSB 为本周新增）：

| 用例 | 断言 |
|------|------|
| LSB1 | consume 'Hello' 之前 → graphList 4→3、`mFirstGraphIndex=5`、`cp=5`（getNextCP 精确） |
| LSB2 | consume 最后图 → else 分支 clean graphList/srcList、cp=3、hasReadEnd=false |

**真实（第二批：snapshot 数据面 + 列表/矩形访问器）**：
- `cache/TypoSnapshot.rs`：**真实数据面**（Java TypoMemBlock 块管理降压 flat heap）——
  `alloc(size)`（0 保留 null、`getAllocSize` 同步）/`get(pointer)`（OOB→0）/
  `set(pointer,value)`（越界自动扩堆）/`set×3..5` 逐字写；Java 布局逐位等价。
- `cache/TypoList.rs`：`alloc`/`size`/`get`/`quickGet`/`first`/`last`/`insert` 真实
  （行指针数组布局 [pList+2]=size、[pList+3..]=元素）。
- `cache/TypoObjBase.rs`：`getLeft/Top/Right/Bottom` 真实（Java TypoRectMemoryImpl 布局
  pointer+3+0..3；替代 drawing 侧 0-stub）。
- `cache/TypoTextLine.rs`：`alloc`（snapshot.alloc(26)+类型头 3/26）真实；静态
  `getLines(ptr+19)`/`getLineAfter(ptr+24)`/实例 `getStartCP`（mirror）真实。
- PageViewLayouter 两处 TypoList 调用点改 `&mut snapshot`（数据面真读取）。

**测试**（05 目标 6/6 绿，TL/OB 为本周新增）：

| 用例 | 断言 |
|------|------|
| TL1 | heap 平面：alloc(4)→1（null 保留）、set/get 精确、OOB 读 0、越界写扩堆、`getAllocSize` 同步 |
| TL2 | TypoList：alloc 列表、insert×3 → size=3、first/last/get/quickGet 精确、OOB 0、pList==0→0 |
| OB1 | 行矩形 TypoObjBase 四向精确；TypoTextLine.alloc→行指针、getLines/getLineAfter 精确、实例 mirror roundtrip |

**真实（第一批：行高叶）**：
- `line/LineHeightLayouter.rs`：Java 467 行全量真实移植——`initLineHeightInfo`
  （graphList 遍历、embed/weak/text 通道 VMetrics 汇总、pushVMetricsDatas 栈）、
  `calcLineHeightAndAlignmentLine`/`updateLineHeightAndAlignmentLine`、
  `calcLineHeightInfoByLineSpacingRule`（规则 0/1/2 + 网格）、`calcBaseLinePos`、`updateLineInfo`；
  `nextGraphs` 游标（05 后半程）停车（hello 走 lastElement 等价）。
- `line/ILineHeightCallback.rs`：真实 trait（4 方法，LineLayouter 实现）。
- `graph/GraphAlignTool.rs`：`getGraphVMetrics`/`getEmbedVMetrics`＋4 个对齐度量 helper
  （top/middle/baseline/bottom）真实（hps sub/sup、bdrWidth/weak、CJK external 0.15）。
- `LayoutState.pageEnv` 升 pub（Java `public PageEnv pageEnv` 门面）。

**测试**（`05_function_flow_text_line`，3/3 绿）：

| 用例 | 断言 |
|------|------|
| LH1 | hello 单行（10pt）`lineHeight == (int)(a0+a1+0.5)`（a=GraphAlignTool.getGraphVMetrics(4)）；baseline=通道 0；asc/descPos 精确 |
| LH2 | 空 graphList → datas 全 0 → doCommit 直通（0 行高） |
| LH3 | 合并 Hello 图同 style → updateLineHeightAndAlignmentLine 不增补度量，行高不变 |

## 3. 剩余（下一批）

- `LayoutState.consumeGraphsBefore`（getNextCP/srcList 联动）解 DEFERRED。
- `LineLayouter.flow`：`preCalcLineHeight`（calcParaBefore/After、mLineHeightLayouter 接线）→
  `isOverflowBeforeFlow` → `prepareSubLineEnvs`（单 subline/coreWidth）→ wrap 循环 →
  `SubLineLayouter.flow`（508 行）→ 行终止/换行 → TypoTextLine 行写入（数据面已备）。
- `TextLayouter.flowTextLine`/`doFlowTextLine`/`onEmptyGraphSize`/`finishPage`/`rollbackLine` 编排真实
  （doFlowTextLine 行读取 TypoList/TypoObjBase/TypoTextLine statics 已备）。
- 06 产品桥：Java 行 → `PageLine/TypoTextLine` → `PaginatedPage → snapshot`。
