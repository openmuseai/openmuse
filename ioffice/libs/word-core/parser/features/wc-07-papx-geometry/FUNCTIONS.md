# wc-07-papx-geometry — function 拆分

| order | function_id | I | O | status |
|------|-------------|---|---|--------|
| 1 | [01_function_indent_sprm](functions/01_function_indent_sprm/FUNCTION.md) | ind attrs | 208–213 | pass |
| 2 | [02_function_spacing_sprm](functions/02_function_spacing_sprm/FUNCTION.md) | spacing attrs | 214/217/220 | pass |
| 3 | [03_function_content_width](functions/03_function_content_width/FUNCTION.md) | 页内容宽+210/212 | 行可用宽 | pass |

crate：`libs/word-core`。01/02 改现有 `ParaPropsConverterSimple.rs`；03 扩 `ContentWidth.rs`。
