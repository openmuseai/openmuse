# wr-03-measure-shape — 文本度量（字体栈 + 行宽/字形）

> 分析日期：2026-08-22
> Java 底稿：`core-libs/wpslayout`（`PaintFontHelper`/`SpanInfo` 度量语义）+ `core-libs/thirdparty/font`（FreeTypeJNI）
> 输入：run 样式（字体名/字号）+ 文本片段
> 输出：字符 advance / run 总宽 / ascent-descent —— wr-04 断行与 wr-07 坐标的度量输入
> 切分依据：I/O 边界（字体句柄 → 单字符宽 → run 行宽 → 审计）

## 1. Java 管线分析

- **度量原语**：Java 经 `PaintFontHelper.getCharWidth(char[], SpanInfo)` + `SpanInfo.fontSize/ascent`（wpslayout data 包）取字符宽；底子是 Android `Paint` + `libfreetype-jni.so`。
- **run 样式**：`SpanInfo`（fontSize/fontSizeOri/endPos/resetAscent）＝ run 级样式，对应我们 Rust 的 run（CHPX 派生 + 字体/字号/色）。
- **关键缺口（2026-08-22 复核）**：`core_libs_font` 的 Rust `FreeTypeJNI` 是**壳**（Cargo 零依赖、`LISBARY_NAME="freetype-jni"`、`sIsLoadLibSucceed=false`）——没有 Android 的 `.so`，**量不出任何字形**。
- **FontHost**：Java 先 `Platform` 扫目录 → `FontHost`/`FontTool.addFontMapScheme` 建表 → `TypefaceFactory` → `AndroidTypeface`。Rust 由宿主注入 `FontBlob`，`04_function_font_host` 建表（**不探盘**）。

## 2. Function 清单

| order | function_id | 一句话 | 输入 | 输出 | 上游 | 下游 |
|------|-------------|--------|------|------|------|------|
| 1 | [01_function_font_stack](./functions/01_function_font_stack/FUNCTION.md) | 字体句柄：pt↔twip + 字体名→ttf-parser Face | 字号/字体名（rPrDefault） | Face/Err | ttf-parser 0.24（已入 Cargo） | 02 |
| 2 | [02_function_run_metrics](./functions/02_function_run_metrics/FUNCTION.md) | run 字符度量：单字 advance + 总宽 | 文本+Face+字号 | 逐字 advance + runWidth（twip） | 01 | wr-04 |
| 3 | [03_function_rich_text_layout](./functions/03_function_rich_text_layout/FUNCTION.md) | 审计：正文不接 shape 排版引擎 + `LineMetrics` 契约 | wpslayout 壳 | 审计测试 + LineMetrics | wpslayout（defer） | 后续形状文本; wr-04/07 |
| 4 | [04_function_font_host](./functions/04_function_font_host/FUNCTION.md) | Java FontHost：注入表 + SchemaItem 显示名→实际族 → FontStack | FontBlob[] / scheme | FontStack + default_id | 01；宿主注入 | layout_document |

## 3. 依赖与编译面

src：`libs/word-core/src/cn/wps/moffice/writer/layout/text/{FontStack,FontHost,BaseTextLayout}.rs`
依赖：**`ttf-parser 0.24`（已加入 word-core Cargo.toml，MIT，纯 Rust 无 so）**；`core_libs_font` 仅 Backend B 占位；`rustybuzz`（后续 shaping 用）。
crate：`libs/word-core`。
Fixture：`libs/word-core/tests/fixtures/01_function_font_stack/*.ttf`（测试字体，零平台依赖）。

## 4. 验收顺序

```text
01 font_stack（pt_to_twip 钉死 10.5→210；fixture 加载；advance 黄金值=hmtx；未知名回落）
02 run_metrics（"A"/CJK/总宽=Σ；12 vs 24pt 成比例；控制字符跳过）
03 审计（wpslayout 壳分类表；"热路径不接 RichTextLayout" grep 0 hits；LineMetrics 契约字段钉死）
04 font_host（空表 Err；scheme 显示名命中；未知回落默认；layout_document 不经 FontStack::default 探盘）
```