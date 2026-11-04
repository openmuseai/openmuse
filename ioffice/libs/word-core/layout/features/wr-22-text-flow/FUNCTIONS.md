# wr-22-text-flow — function 拆分

| order | function_id | I | O | status |
|------|-------------|---|---|--------|
| 1 | [01_function_pre_flow](functions/01_function_pre_flow/FUNCTION.md) | cp+papx | 段首/mRef | pass |
| 2 | [02_function_do_flow_line](functions/02_function_do_flow_line/FUNCTION.md) | state | 一行或 defer | pass |
| 3 | [03_function_flow_loop](functions/03_function_flow_loop/FUNCTION.md) | srcList | 页内行 | pass |
| 4 | [04_function_post_flow](functions/04_function_post_flow/FUNCTION.md) | state | end 回调 | pass |

src：`TextLayouter.rs` 完整交错模板；列表外方法停车。
