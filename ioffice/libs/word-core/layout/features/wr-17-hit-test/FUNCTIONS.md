# wr-17-hit-test — function 拆分

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_page_hit` | page + y + rows | clamped line | wr-14;16 | 03 | pass |
| 2 | `02_function_cp_hit` | line + x + advances | cp | wr-11;14 | 03 | pass |
| 3 | `03_function_hit_test` | snapshot rows + point | page/line/cp | 01;02 | cursor | pass |

只实现只读点命中；屏幕 zoom 反变换、选区、编辑、IME 均在边界外。
