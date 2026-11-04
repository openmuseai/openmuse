# wr-16-line-golden — function 拆分

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_line_normalize` | unordered rows | validated ordered rows | wr-14 | 02 | pass |
| 2 | `02_function_line_compare` | Java/Rust rows + tolerance | first exact diff | 01 | 03 | pass |
| 3 | `03_function_line_corpus` | named JSON pairs | pass / named first failure | 02 | release gate | pass |

比较器对 page/line/start_cp/end_cp 精确比较，几何只用显式 twip 容差，忽略 xpos。
