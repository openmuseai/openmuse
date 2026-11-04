# wr-20-edit-relayout — function 拆分

> 日期：2026-08-27  
> Feature：[FEATURE.md](./FEATURE.md)

## 0. 管线

```text
session + (cp, text) 或 [start,end)
  → 01 event_insert     DocumentEvent INSERT + 新 length
  → 02 event_remove     DocumentEvent REMOVE + 新 length
  → 03 event_relayout   全量 LayoutFrame（编排：01/02 + layout）
```

## 1. 清单

| order | function_id | 一句话 | I | O | 上游 | 下游 | status |
|------|-------------|--------|---|---|------|------|--------|
| 1 | [01_function_event_insert](functions/01_function_event_insert/FUNCTION.md) | 任意 cp 插入 | session+cp+text | INSERT 事件 | wr-18 | 03 | pass |
| 2 | [02_function_event_remove](functions/02_function_event_remove/FUNCTION.md) | 半开删除 | session+[lo,hi) | REMOVE 事件 | wr-18 | 03 | pass |
| 3 | [03_function_event_relayout](functions/03_function_event_relayout/FUNCTION.md) | 改后全量排 | session+fonts | LayoutFrame | 01;02; wr-18-04 | painter | pass |

## 2. 数据流

01/02 不调用 `layout()`。03 只断言可见 `DrawText.text`。不跑 EventCollector。

## 3. 编译面

crate：`libs/word-core`  
src：`api/SessionEdit.rs`；`core/DocumentEvent.rs`  
test：`tests/cn/wps/moffice/writer/api/{SessionInsert,SessionRemove,SessionEventRelayout}.rs`

| function_id | `cargo test --test` |
|-------------|---------------------|
| 01_function_event_insert | `01_function_event_insert` |
| 02_function_event_remove | `02_function_event_remove` |
| 03_function_event_relayout | `03_function_event_relayout` |

## 4. 验收顺序

01 → 02 → 03。

## 5. 明确不做

CoreThread；增量 Typo 失效；选区 FillRect；在 Frame 上改字。
