# wr-05-page-flow — 分页/节/网格

> 分析日期：2026-08-22
> Java 底稿：`writer/core/layout/base/MainLayouter.java` + `writer/core/layout/FlowLayout.java` + `writer/core/layout/base/smallerscale/IPageGrid.java` + `TextLayouter.processPagination`
> 输入：折行流（wr-04）+ 节属性（PLCSection）
> 输出：页序列（每页行集合 + 已用高）→ wr-06 快照 / TypoPage
> 切分依据：I/O 边界（页满判定 → 页网格/节 → 重排缓存）

## 1. Java 管线分析

### 1.1 FlowLayout（writer/core/layout/FlowLayout.java）—— 顶层驱动
方法（钉死）：`flowPage(Rect)`（流到某视图矩形页）· `flowCP(cp, endCp, docType, rect)` · `flowToPage(pageIndex)` · `flowViewRect(rect)` · `flowPageInScrollDirection(rect)` · `flowToDocumentEnd(rect)` · `prepareFlowFromDocumentBegin` · `acceptCacheSnapShot(rect, hasFinishedIO)` · `update(DocumentEvent)`。持有 `TypoDocument`/`ViewEnv`/`LayoutServiceCache`；`changeSize` 触发重排。

### 1.2 MainLayouter（layout/base/MainLayouter.java）—— 文本页排版
`flowDocument(maxPageCount)`/`flowPage()/flowCP/flowToPage`；持有 `LocksHelper`/`KList<Event>`/`EventHandler`/`MaxCp`/`LayoutState`。页面/分节/脚注事件经 EventHandler 回调（MVP 只挂分节事件）。

### 1.3 processPagination（TextLayouter）—— 页满判定
行累计高 ≥ 页内容高 → 页终止（`FLAG_LASTLINEOFPAGE`），换新页；分页符 `\u000c`（wc-04）直接断页；节 PageSetup 变化（PLCSection）切换页尺寸。

### 1.4 IPageGrid（smallerscale）—— 页网格
`pageIndexFromChar(cp)` / 页内偏移 / 网格自动变化（doGridAutoResize，MVP 延后）；MVP 用「页对象列表 + cp 区间二分」等价实现。

### 1.5 Java↔Rust 现状对照
Rust 无 FlowLayout/MainLayouter；`wc-01` PLCSection 已实现（节节点/cp），`wc-05 06` 默认节已就绪（wr-02 01 消费）。

## 2. Function 清单
| order | function_id | 一句话 | 输入 | 输出 | 上游 | 下游 |
|------|-------------|--------|------|------|------|------|
| 1 | [01_function_page_break](./functions/01_function_page_break/FUNCTION.md) | 页满判定与断页 | 行流 + contentHeight + 分页符 | 页序列（页末行/已用高） | wr-04; wc-04 `\f` | 02; wr-06 |
| 2 | [02_function_section_grid](./functions/02_function_section_grid/FUNCTION.md) | 节页网格（尺寸切换 + cp→页） | PLCSection 节序列 + PageSetup | 每页尺寸 + `page_index_from_cp` | wc-01 PLCSection; wr-02 | wr-06/07 |
| 3 | [03_function_relayout_cache](./functions/03_function_relayout_cache/FUNCTION.md) | 重排范围/增量状态 | 编辑事件 (cp,rm,ins) | 需重排区间（MVP 全量，占位） | wc-02 编辑 | wr-06 |
| 4 | [03_function_layout_cache](./functions/03_function_layout_cache/FUNCTION.md) | 全量重排占位（invalidate 不可绘） | dirty | 丢弃 snapshot | 01 | wr-06 |

## 3. 依赖与编译面
src（loop 新建）：`libs/word-core/src/cn/wps/moffice/writer/layout/{FlowLayout,MainLayouter}.rs`（编排）+ `layout/base/PageGrid.rs`。crate `libs/word-core`。

## 4. 验收顺序
```text
01 page_break（满页断；分页符断；末页不足高）
02 section_grid（默认节单页尺寸；多节切换；cp→页二分）
03 relayout_cache（MVP 全量重排：区间=整文档；占位保接口）
```