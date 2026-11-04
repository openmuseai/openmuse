# wr-25-line-compress — function 拆分

切在压缩 **I/O**，不切整份 LineLayouter。

| order | function_id | I | O | status |
|------|-------------|---|---|--------|
| 1 | [01_function_compress_env](functions/01_function_compress_env/FUNCTION.md) | graphIdx + space/punct/other | 可压余量 / used / before/after | pass |
| 2 | [02_function_compress](functions/02_function_compress/FUNCTION.md) | graphsPos + subLineWidth + LayoutState | compress() 是否整行 fit | pass |

src：`LineCompressEnv.rs`、`LineCompressTool.rs` 由 function-java2rust 生成完整交错模板。
