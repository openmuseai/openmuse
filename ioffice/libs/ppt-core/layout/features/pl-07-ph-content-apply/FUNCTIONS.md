# pl-07-ph-content-apply — Function 组合图

> 状态：**pass** ｜ 管线：[VISUAL_CONTENT_PIPELINE.md](../../../VISUAL_CONTENT_PIPELINE.md)

## §0 管线位置

```text
pp-14 绑树 + nvPr
  → [01] mergeShapePr 写回 sp_pr（修 let _style）
  → [02] 空 ph 文本从 layout/master 同 ph 拷贝 text_op
  → render_slide 才有 fill 与字
```

## §1 清单

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_apply_merged_shape_pr` | shape+layout/master | 写回 sp_pr | pl-03:02;pp-14 | pr-05 | pass |
| 2 | `02_function_ph_text_inherit` | 空 slide ph + layout ph | text_op | pp-11;pp-14:03 | pr-03 | pass |

## §2 数据流

```text
resolve_shape:
  t = mergeTransform; apply_xfrm
  sp_pr = mergeShapePr   // 必须赋值，禁止 _
  if text_op 空 && is_placeholder → copy layout/master 匹配 ph 的 text_op
```

## §3 测试面

fixture：slide ph 无 fill、layout 同 idx 有 solidFill；slide txBody 空、layout 有 `a:t`。不跑 zip（可用内存 KmoSlide）。

## §4 验收

写回后 `hasFill()` true；空 ph 产生非空 DrawText（经现有 emit_text）。

## §5 loop

`01 → 02`

## §6 类级

| Java | function |
|---|---|
| `PlaceHolderMerger.mergeShapePr` | 01 只调用 |
| Java 文本跟随 / lstStyle follow | 02 MVP 只拷贝已解析 text_op；lstStyle 全量 DEFERRED |
