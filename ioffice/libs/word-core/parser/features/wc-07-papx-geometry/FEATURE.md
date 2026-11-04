# wc-07-papx-geometry

> status: **pass** · M11 · [NEXT_PLAN.md](../../../state-java-l2-parity/NEXT_PLAN.md)

把 **影响行宽/行高** 的 pPr 写入 PAPX，并让产品行宽扣左右缩进。converter 叶子已部分存在；本 feature 补 firstLine/right/spacing 对象与 `ContentWidth::paraMaxWidth`。

## I/O

| I | OOXML ind/spacing attrs 或已 open 的 PAPX |
| O | 208–213、214/217/220 在桶中；内容宽公式含 210/212 |

Java：`ParaPropsConverterSimple` + `ParaLayouter` 缩进。不排表。
