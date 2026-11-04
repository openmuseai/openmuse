# wc-06-stage1-live — function 拆分

切在 **I/O**：dump 文本 → dump 属性桶 → 比较。不跑 layout。

## 0. 管线

```text
.docx → open_text_document
  → 01 rope_dump     getText(0, length)
  → 02 prop_dump     PAPX/CHPX 关键 sprm
  → 03 stage1_compare  vs Java JSON
```

## 1. 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|------|-------------|--------|---|---|----------|------------|--------|
| 1 | [01_function_rope_dump](functions/01_function_rope_dump/FUNCTION.md) | 主故事文本 | TextDocument | String | wc-05 | 03 | pass |
| 2 | [02_function_prop_dump](functions/02_function_prop_dump/FUNCTION.md) | 段/run 桶 | PAPX+CHPX | Stage1 props JSON | wc-01;wc-03 | 03 | pass |
| 3 | [03_function_stage1_compare](functions/03_function_stage1_compare/FUNCTION.md) | 双跑 Stage1 | rust+java JSON | firstDiff | 01;02 | wc-07 | pass |

crate：`libs/word-core`。src 拟：`src/cn/wps/moffice/writer/api/stage1/{RopeDump,PropDump,Stage1Compare}.rs`。  
`cargo test --test <function_id>` 在 loop L2 挂 `[[test]]`。
