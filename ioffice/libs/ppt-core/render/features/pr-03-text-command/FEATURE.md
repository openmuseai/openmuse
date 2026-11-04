# pr-03-text-command — 文本 RenderCommand

> 层：render ｜ 状态：**pass** ｜ Java：`ShapeTextRender._render` + `TextRender.render`（typo 行 → canvas 文本）

## 目标

把 pl-05 的排版行变成**平台无关文本指令**：`DrawText{text, x, y, font_id, size, color, style}`。实现后，文本从「typo 结果」变成「可上屏指令」——pr-03 之前文本只存在于 typo 结果中，不上屏（主文档：「**渲染断点**：pr-03 之前文本只存在于 typo 结果中，不上屏」）。

Java 文本绘制依赖 `ITypeface`/Canvas Paint 度量；Rust MVP 走 `DrawText` 指令 + Flutter 字形（对齐口径同 writer Stage3，不做像素 CI 门闩）。

## I/O（锁死）

| function | I | O | 契约 |
|---|---|---|---|
| 01 text_line_command | pl-05 行列表（`{text,x,y,w,h}`，布局单位）+ `renderModel.getTextRect(w,h)`（文本区 inset）+ `ShapeTextRender._render` 过滤（drawPHolderText） | `DrawText{text,x,y}` 序列（布局坐标 → render px 换算） | 行 → 一个 DrawText；文本区左上为行几何偏移；flipH/V 先记命令元数据（镜像在 painter 层） |
| 02 run_style_command | `SpanInfo` 样式（fontName/fontSize/color/bold/italic）+ 行定位 | 带样式的 `DrawText` | 每个 run/span 一个 DrawText；颜色/字号/字重附着；bullet/下划线/动画 timing 不做 |

MVP 子集：纯文本行（每行/每 run 一个 DrawText）；**不做** bullet、动画 timing、编辑态选区（startPara/endPara/offset）、竖排/warp/autofit、下划线装饰、artTransform 文本框旋转（`rotateCanvas`）。

## Java 管线对照

| Java 类/方法 | 语义 | Rust 归属 |
|---|---|---|
| `ShapeTextRender._render(ITypoShape)` | `DrawFlags.drawPHolderText` 过滤；`renderModel.getTextRect(w,h)`；flip 计算（`ShapeUtil.calcShapeFlipEx`，bFlip=flipH xor flipV）；分派 `TextRender.render` | 01 入口 |
| `ShapeText` | `hasTypoText()`（= hasRealText() \|\| hasBullets()）、`hasRealText()`、`hasBullets()`、`getTextContext().getPageNumber()` | 复用 `ShapeText.rs`（已 keep_refactor） |
| `RenderModel.getTextRect(float,float)` | `Shape.getTextRect(w,h)`→`DataCal.getTextRect`：预置几何→`pathData.getTextBoxRect()`；自定义几何→`geometry.getGeoTextRect()` 缩放；否则全框 | 01 文本区（MVP：全框内缩，预置几何文本框待 loop） |
| `TextRender.render(canvas, rcShape, typoRect, typoShape, shapeText, drawFlags)` | scale=layoutRenderScale；`typoText()` 取/排版 `IShowTextLayout`；`drawText` = translate(typoRect.left/top) + `textLayout.renderText(canvas)` | 行几何 → DrawText 序列 |
| `TextRender.typoText(...)`/`_layout(...)` | `ShowTextLayout.layout(...)` → `IShowTextLayout`（pl-05 已管） | pl-05 输入复用 |
| `IShowTextLayout.renderText(canvas)` | 依行/Span 几何画字（`ShowRichTextLayout.renderText`） | 行/Span 枚举 |
| `ShowTextLayout.layout(RectF, KmoShowShape, KmoContext, boolean forceTypo, IResizeCallback)` | `exToInternal`→`TextBoxData`；`new ShowRichTextLayout().typo(boxData, w, h, forceTypo)` | pl-05（MVP 贪心断行） |
| `TextBoxData`/`TextData`/`LineInfo`/`SpanInfo` | 段落/行/Span 几何与样式 | pl-04/pl-05 数据模型 |
| `ShapeUtil.calcShapeFlipEx` | flipH/V 修正 | flip 元数据（painter 层消费） |
| `UnitsConverter.layoutRenderScaleX/Y` | 布局单位 → render px | DrawText 坐标换算 |

Rust 根路径：`libs/ppt-core/src/cn/wps/show/render/`（`ShapeTextRender.rs`、`TextRender.rs` 新建；`ShapeText.rs` 复用）。

## 能力解锁

- `01_function_text_line_command`：排版行 → `DrawText{text,x,y}` → **文本可上屏**（主文档 pr-03 `text_command` 行）。
- `02_function_run_style_command`：run 样式（字体/字号/颜色）附着 → Flutter 可按样式画字（对齐 writer wr-07 `DrawText{font_id,size,color,style}`）。

缺失断点：pr-03 缺失 = 文本不上屏（**有字时断**，主文档 §3.2）。

## 必须 / 壳 / 延后

- **must：** 01 的行 → DrawText（text/x/y）、`getTextRect` 文本区、坐标换算（layout→render px）、`drawPHolderText` 过滤；02 的 run 样式（fontName/fontSize/color/bold/italic）附着；指令 `DrawText` 模型（与 wr-07 字段对齐待 loop）。
- **壳：** flip 的 canvas 镜像（Rust 记 `flipH/flipV` 命令元数据，镜像归 painter 层，内核不执行 canvas scale）；`ShapeText.hashTypoText` 中 hasBullets 分支（MVP 含不带 bullet 的文本行）。
- **defer：** bullet 绘制（`BulletDrawer`）、动画 `TimingInfo`（`_createTextInfo`）、编辑态选区超载（`render(...firstPara,startOffset,…)` 与 `getTextRects`）、竖排/warp/autofit（textbox 属性，pl-04 测）、下划线装饰、提示文本（`PromptResource`/`layoutPromptText`）、modular 文本子形状。

## 范围

- **in：** 行/run → DrawText；文本区 rect；font/字号/颜色/字重；flip 元数据；`ShapeText` 判定（isPlaceHolder/isTextBox/hasTypoText）。
- **out：** Canvas Paint 度量（字体度量交由 painter/Flutter）；bullet/动画/提示文本；表格单元格文本（TableRender）；vertical/warp 文本；图文混排（blip 内文本）。

## 验收

FUNCTIONS.md §4：

1. fixture 文本框（一行纯文本）→ 恰好 1 个 `DrawText`，坐标 = 文本区左上 + 行偏移（layout→render px）。
2. `getTextRect(w,h)` 的 inset 生效（文本区 ≠ 形状框，全框 MVP 至少可测）。
3. run 样式：fontName/size/color/bold 附着正确。
4. `DrawFlags.drawPHolderText==false`（&16 置位）且是占位符 → 不产 DrawText。
5. flipH（xor flipV）→ 命令带 flip 元数据（painter 层镜像，不作为内核坐标变换）。

**建议 MVP pass 口径（loop 采用）：** 01 必测 = 单行 DrawText + 坐标换算 + ph 文本过滤；02 必测 = 字体/字号/颜色附着；bullet/timing/编辑选区 `#[ignore = "spec: post-MVP"]`。

## 依赖

- **depends：** pl-05（排版行几何，MVP 贪心断行）；pl-01（layoutRenderScale/fromEmu）；pr-01（层列表里文本形状）；pl-06（`LayeredTypoShape`）。
- **does not depend：** pp-11 完整 txbody（pl-04 的 TextBoxData fixture 可先行）；pr-02（文本与形状指令并行，最终由 pr-04 合流）。

## 不过扩展

禁止把 Canvas `drawText`/Paint 度量语义搬进内核输出（文本指令只有 text+几何+样式字段，字形渲染归 painter）。禁止实现 bullet/动画 timing 假装画字。禁止在缺少 pl-05 时用「flatten 整框字符串」冒充逐行指令——坐标对齐另做像素门闩（主文档：像素级一致与 Java 文本排版一致不是第一波门闩）。