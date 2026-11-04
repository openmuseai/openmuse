# wr-14-typo-stage2 — function 拆分

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_stage2_rows` | wr-13 pages / Snapshot | `Stage2Line[]` | wr-13 | 03;wr-16;17 | pass |
| 2 | `02_function_snapshot_commit` | wr-13 pages | owned `TypoSnapshot` | wr-13 | 01 | pass |
| 3 | `03_function_stage2_json` | `Stage2Line[]` | stable JSON | 01 | wr-16 | pass |

crate：`libs/word-core/Cargo.toml`；产品结构门禁：`TypoSnapshot.java`、`TypoPage.java`、`TypoTextLine.java`。
