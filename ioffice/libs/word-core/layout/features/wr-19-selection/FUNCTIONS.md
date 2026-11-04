# wr-19-selection — function 拆分

> 日期：2026-08-27  
> Feature：[FEATURE.md](./FEATURE.md)

## 0. 管线

```text
(start,end) + 故事可见长度
  → 01 sel_clamp     有序半开 [lo,hi) 或空
  → 02 sel_rects     LayoutFrame + [lo,hi) → 几何框
  → 03 sel_fill      框 → kind=selection 的 DrawRect（编排：overlay）
```

## 1. 清单

| order | function_id | 一句话 | I | O | 上游 | 下游 | status |
|------|-------------|--------|---|---|------|------|--------|
| 1 | [01_function_sel_clamp](functions/01_function_sel_clamp/FUNCTION.md) | 夹紧选区 cp | start,end,len | Option [lo,hi) | wr-18 cp | 02;03 | pass |
| 2 | [02_function_sel_rects](functions/02_function_sel_rects/FUNCTION.md) | xpos 切片成框 | Frame + 区间 | DrawRect 几何 | 01; wr-18 | 03 | pass |
| 3 | [03_function_sel_fill](functions/03_function_sel_fill/FUNCTION.md) | 发 selection 指令 | 框或 Frame+区间 | Rect 指令 | 01;02; wr-07 | painter | pass |

## 2. 数据流

01 不打开 zip。02 只吃 Frame。03 可 overlay 到原 commands（Rect 在 Text 前）。

## 3. 编译面

crate：`libs/word-core`  
src：`api/{SelectionRange,SelectionRects,SelectionFill}.rs`

## 4. 验收顺序

01 → 02 → 03。

## 5. 明确不做

LocateTool；编辑；IME composing 高亮。
