# wr-24-live-stage2

> status: **pass** · **本波出口 M15**

`layout_document` **当次实排** 的 Stage2 行表 vs Java dump。复用 wr-16 比较器；**忽略 xpos**。corpus = wr-16 五类 + sample2。

## I/O

| I | .docx + 同 TTF + Java Stage2 JSON |
| O | CI pass/fail（首失败 fixture + 字段路径） |

禁止用预导出 Rust JSON 冒充实排。
