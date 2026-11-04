# wr-13-page-layouter — function 拆分

切在分页 I/O：装页判定 → 段边界回滚 → 页流编排。输入是 wr-12 已切好的行，禁止重新断行。

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_page_fit` | used/line/content height | break-before | wr-12 | 03 | pass |
| 2 | `02_function_widow_control` | 相邻页的段行 | 回滚后的页边界 | 01 | 03 | pass |
| 3 | `03_function_page_layouter` | `PageLine[]` + content height + `\f` | `PaginatedPage[]` | 01;02 | wr-14 | pass |

crate：`libs/word-core/Cargo.toml`；测试镜像 `src/cn/wps/moffice/writer/layout/base/env/*.rs` 到 `tests/.../env/*.rs`。

Java 逻辑权威：`TextLayouter.processPagination` / `finishPage`；产品结构门禁：`TypoTextLine.java`、`TypoPage.java`。
