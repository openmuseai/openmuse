# 04_function_graph_factory

| | |
|---|---|
| Java 权威 | `koffice/writer/core/.../layout/base/graph/GraphFactory.java`（1160 行）+ `languages/WestLayout.java`（158）+ `languages/LangLayout.java`（539）+ `env/FontStyleFactory.java`（421）+ `env/FontStyleCache.java`（204）+ `cache/style/TypoFontStyle.java`（335） |
| 入口 | `GraphFactory.createNodes`（经 `GraphBuffFactory` → `doPrepareGraphs` 之后的图节点量宽） |
| 状态 | hello 子集已通过；2026-10-04 补东亚路径，`04_function_graph_factory` 6/6；全脚本/全门面尚未验收，见 `rewrite-review/milestones/layout-java-pipeline/05-sample2-device-parity.md` |

## 1. I / O

- **I**：`GraphBuff.prepareGraphs` 后的 `charsList`/`typoPara`（含 chars 平面）；`FontHost` 静态字体表（Rust `FontHost::set_font_table`，Java `FontHost.getFontTable()` 对等）。
- **O**：`graphList` 图节点（`runType`/`graphLength`/`mWidth` 已量宽）＋ `typoPara.widths` 平面（Java `fillWidths`/逐字 advance 的 snapshot 语义等价）。

## 2. 本 function 落地记录（wr-29 04）

**真实（hello 主路径）**：
- `graph/GraphFactory.rs`：Java 1160 行全量方法面（ctor/dispose/createNodes/addSpaceGraph/
  getSpaceEmSize/addZeroWidthSpaceGraph/commitVanishBreak/getFontStyleCache/parseChar/
  parseDefault/parseAscii/needTagForTransToSpace/isRTL/isArabic/isEmoji/setWordAlignFont/
  scaleChar/setMathUnderline/setCalcGraphHeight/setFootEndNoteRefType/commitAll/commitGraphNode/
  updateGraphOperationEnv）；createNodes 的 runType 分派 + 空格合并 + 页断(runType5) 全真实；
  数学/符号/绘图/脚注/评论/双向 runType（6/15/30-37）GR-D3 停车。
- `languages/WestLayout.rs` + `LangLayoutFactory`（lang 0/1/9 → WestLayout）：ASCII 字母/数字
  run 合并 + `addGraphAfterParse`（字符图合并 + 阶段提交）真实；capsHandler None（首字下沉非 hello）。
- `languages/FarEastLayout.rs` + `SharedFarEastLayout.rs` + `LangLayoutFactory`（lang 2/3）：
  补 Java 同名子类、连续汉字合图、全角默认宽与 `roundError`；`LangLayout.calcWidths` 按 Java
  虚分派转入对应子类。`sample2` 由缺路径时的 3 页/88 行回到 8 页/228 行。
- `languages/LangLayout.rs`：`addGraphNode`（池分配+字段）/`addGraph`/`addCommonGraph`/
  `addLangGraph`/`commitLangGraphNode`/`commitCommonGraphNode`/`calcGraphNode`/`calcWidths`
  （roundError 逐字累积、hmtx×scale、boldAddWidth）/`calcHeight`（05 行高前空安全）真实；
  泰文/emoji/Non-BMP 停车。
- `cache/style/TypoFontStyle.rs`：真实移植（fields/flag 位方法/池 obtain/reset/clear/hashCode/equals）。
- `env/FontStyleFactory.rs`：`create/createFontStyle/calcAscentAndDescent/createTypeface/
  getFontSize/getScriptFontsizeScale/getSubposition/updateStyleResult` 真实（编号/修订停车）。
- `env/FontStyleCache.rs`：无 mark(finalRunProp) → Java `create` 直通分支真实；带 mark 缓存路径停车。
- `env/FontStyleCacheFactory.rs`：按 docType 建/复用 FontStyleCache 真实。
- `tools/TypefaceTool.rs`：`WriterTypeface`＝Rust `cn.wps.font.ITypeface` trait 真实实现
  （EM＝hmtx font units、srcEmSize＝upe；`FontStack.glyph_hmtx_units/units_per_em/hmtx_ascent_descent` 新增）。
- `core/CharParse.rs`/`CharMap.rs`：按 Java 原构造顺序移植 217 个 Item、ASCII 类表、二分查找
  和五项缓存；Han 被分类为 FarEast。镜像符号及复杂脚本分支仍需另里程碑覆盖。
- `LayoutState`：`isAllowTransForSpace` 解 DEFERRED；`getGraphList` 真；`roundError` 累积。
- `TypoPara`：`getWidthBlock/getChars/fillWidths/putWidths` 新增。
- 悬垂修复：GraphFactory ctor 不捕获 `this` 内指针（Rust 构造后整结构移动）→
  `updateGraphOperationEnv` 于 createNodes 入口以最终位置刷新（Java 对象引用语义等价）。

**测试**（`04_function_graph_factory`，6/6 绿）：

| 用例 | 断言 |
|------|------|
| GF1 | ctor/dispose/opEnv 初值（runType=1、mathUnderlineColor、SurrogateType） |
| GF2 | `"Hello World!"` → 4 图（WestLayout 合并）；runType 1/2/18；`Hello` 宽＝逐字 hmtx×200/upe 和（与 wr-11 同式）；`!` 缺字形 0 宽 |
| GF3 | `"A  B"` 双空格合并图 graphLength=2 |
| GF4 | setCalcGraphHeight/setMathUnderline/setFootEndNoteRefType/scaleChar/setMathContent/setIgnoreLeading |
| GF5 | createNodes 后 lang/commonGraphNode 无残留；typoPara.widths 已填充 |
| GF6 | Java CharMap 将汉字分为 FarEast，连续汉字合为一图，每字默认宽等于缩放字号 |

## 3. 门禁对照

- **G-L2（05 前不可达）**：本 function 产出"每图 mWidth＋typoPara.widths"，行几何在 05 `flowTextLine` 编排。
- **G-fake**：`Graphs.mLangLayout` 影子 struct 删除 → 语言 id（`Option<i32>`，05 经 Factory 重建）；
  `DocumentEnv.FontStyleCacheFactory`/`LayoutState.TypoFontStyle`（Cache 系）去影子。
- **G-java**：全部 `// java` 注释保留；DEFERRED 停车体含 Java 原文。
- **未覆盖**：Emoji/Thai/Complex 工具与对应 Java 分支；不能用 hello/CJK 子集测试宣称全函数逻辑 100% 对齐。
