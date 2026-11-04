# pl-07-ph-content-apply — 把继承结果写回产品 shape

> 层：layout ｜ 状态：**pass** ｜ depends_on: pp-14; pl-03; pp-11  
> pl-03 算法已 pass；产品 `resolve_pages` **丢弃** `mergeShapePr`，且不合并 layout txBody。

## 目标

继承后的 `ShapePr` 写回 `KmoShowShape.sp_pr`；slide 上空/仅 br 的 ph 文本从 layout 同 idx/type 的 txBody 补齐。

## I/O

| 输入 | 输出 |
|---|---|
| 已绑定 layout/master 的 `KmoSlide` | 每 shape `sp_pr` = mergeShapePr 结果 |
| slide ph `text_op` 空 + layout 匹配 ph 有文本 | 非空 `text_op` |
