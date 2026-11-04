# wr-21-para-layouter — function 拆分

切在 ParaLayouter **I/O**，不切整份 LayoutState。

| order | function_id | I | O | status |
|------|-------------|---|---|--------|
| 1 | [01_function_para_spacing](functions/01_function_para_spacing/FUNCTION.md) | 217/214 + 邻段 | before/after | pass |
| 2 | [02_function_line_spacing](functions/02_function_line_spacing/FUNCTION.md) | 220 | LineSpacing | pass |
| 3 | [03_function_para_indent](functions/03_function_para_indent/FUNCTION.md) | 208–213 | indent twip | pass |
| 4 | [04_function_para_start](functions/04_function_para_start/FUNCTION.md) | envoy | 填满 ParaEnv | pass |

src：`src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs`（loop 按 Java 模板生成后改写）。
