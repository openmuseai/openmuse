# wr-04-line-break — 断行/行高/对齐

> 分析日期：2026-08-22
> Java 底稿：`writer/core/layout/base/TextLayouter.java`（flow 主循环）· `LineLayouter`/`ParaLayouter` · `LayoutState`
> 输入：TextDocument 段落流 + ViewEnv 内容宽 + 度量（wr-03）
> 输出：TypoTextLine 序列（textRect/基线/标志）→ wr-05 分页 / wr-07 渲染
> 切分依据：I/O 边界（度量缓存 → 断行 → 对齐 → 行高）

## 1. Java 管线分析

### 1.1 TextLayouter（layout/base/TextLayouter.java）流程（钉死）
```text
flow(LayoutState)：preFlow → doFlowLine → flowLine
   flowLine → doFlowTextLine（断行/写 TypoTextLine）
            → flowTable / flowTextFrame（MVP 延后）
processPagination（页满判定）→ postFlow
```
字段：`mStartCP/mRef`、`mPageLayouter/mParaLayouter/mLineLayouter/mTableLayouter/mDrawingLayouter/mTextFrameLayouter/mFootEndNoteLayouter`、`mPageRangesCollector`、`mStateSnapshot`（增量排版状态，MVP 全量）。

### 1.2 断行（LineLayouter 最小集）
对每个 run 取度量（wr-03）→ 按内容宽折行：空格/自然断点处折行；超宽单字强制断（Java 语义：`\r`=段末、双空格等规则 MVP 简化）。0/1/N run 折行、行尾空格剪裁。

### 1.3 行高（Java PropertySet 语义）
行高 = 字号/行格 + 段前段后（pPr）。MVP：`lineHeight = roundUp(baseLine * lineSpacing)`；`\r` 段末行、段前段后来自 wc-04 PAPX（paragraph property）。

### 1.4 对齐（jc，wc-03 已转换 3385 等）
`left/center/right/justify`：justify 时行内空白扩展（MVP：仅最后一个空格规则——先只做左/中/右，justify 延后）。

### 1.5 Java↔Rust 现状对照
Rust 无 TextLayouter；`layout/` 空骨架。度量（wr-03）是唯一前置。

## 2. Function 清单
| order | function_id | 一句话 | 输入 | 输出 | 上游 | 下游 |
|------|-------------|--------|------|------|------|------|
| 1 | [01_function_run_width_cache](./functions/01_function_run_width_cache/FUNCTION.md) | run 度量缓存（CP→宽） | run 文本+fontId（wr-03） | run→width 缓存表 | wr-03 01/02 | 02;03;04 |
| 2 | [02_function_wrap](./functions/02_function_wrap/FUNCTION.md) | 行内换行（0/1/N run + 超宽） | 段落 run 流 + contentWidth | 折行序列（每行 run 子区间） | 01 | 03;04; wr-05 |
| 3 | [03_function_alignment](./functions/03_function_alignment/FUNCTION.md) | 对齐（左/中/右） | 折行 + jc | 行内 x 偏移 | 02 | wr-07 |
| 4 | [04_function_line_height](./functions/04_function_line_height/FUNCTION.md) | 行高/段前段后换算 | 字号/行格 + pPr（191/236 等） | lineHeight/top | 02; wc-04 PAPX | wr-05/07 |
| 5 | [01_function_run_width](./functions/01_function_run_width/FUNCTION.md) | 逐 run 批测宽度（wrap 热循环） | run 文本列表 | widths[] 等长 | wr-03 | 02 |

## 3. 依赖与编译面
src（loop 新建）：`libs/word-core/src/cn/wps/moffice/writer/layout/base/text/{RunWidthCache,Wrap,Alignment,LineHeight}.rs`（或 line/ 镜像 Java 包）。crate `libs/word-core`。

## 4. 验收顺序
```text
01 run_width_cache（命中/未命中/CP 平移）
02 wrap        （单 run 超宽断、多 run 折行、空段）
03 alignment   （left/center/right 偏移 = (contentW−lineW)/2 等）
04 line_height （字号→行高；段前/段后累加）
```