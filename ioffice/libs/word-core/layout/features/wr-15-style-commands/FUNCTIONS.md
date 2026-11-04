# wr-15-style-commands — function 拆分

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_style_runs` | styled fragments | coalesced runs | wr-10;11;14 | 02 | pass |
| 2 | `02_function_style_commands` | styled line | `DrawText[]` | 01 | 03 | pass |
| 3 | `03_function_page_commands` | styled pages | v2 `CommandDocument` | 02 | painter | pass |

Heading size、bold、ascii/eastAsia font id 都是合并边界；不以 xpos 作为本 feature 门闩。
