# wr-18-session-caret — function 拆分

> 日期：2026-08-27  
> Feature：[FEATURE.md](./FEATURE.md) · 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)  
> 切在 **I/O**，不切 `TextDocument` 整类。

## 0. 管线

```text
.docx 字节
  → 01 session_open          Box<TextDocument> / DocumentSession
  → 02 session_append        主故事长度 + PAPX（CoreHelper）
  → 04 session_relayout      LayoutFrame（编排：01+02+layout）
  → 03 caret_hit             LayoutFrame + 点 → Option<cp>
```

03 不跑 zip；吃 04 的 Frame 形状或内存里构造的 `DrawText` fixture。  
04 是 feature 出口（可画编辑）。caret 是光标出口，loop 前为 spec。

## 1. 清单

| order | function_id | 一句话 | I | O | 上游 | 下游 | status |
|------|-------------|--------|---|---|------|------|--------|
| 1 | [01_function_session_open](functions/01_function_session_open/FUNCTION.md) | 字节 → 堆会话 | `.docx` | `DocumentSession` | wc-05; DocumentOpen | 02;04 | pass |
| 2 | [02_function_session_append](functions/02_function_session_append/FUNCTION.md) | 末尾加段 | session + text | 新 length | 01; CoreHelper | 04 | pass |
| 3 | [03_function_caret_hit](functions/03_function_caret_hit/FUNCTION.md) | Frame 点 → cp | LayoutFrame + (page,x,y) | `Option<cp>` | wr-17; 04 | cursor | pass |
| 4 | [04_function_session_relayout](functions/04_function_session_relayout/FUNCTION.md) | 改文档后全量排 | session + fonts + view | LayoutFrame | 01;02; wr-09..15 | painter | pass |

## 2. 数据流

01 禁止在 open 后把 `TextDocument` 从堆上移走。02 只断言存储长度，不调 `layout()`。04 只断言可见 `DrawText.text`。03 只断言 cp，不改 rope。

## 3. 编译面

crate：`libs/word-core`  
src：`src/cn/wps/moffice/writer/api/{DocumentSession,SessionCaret}.rs`（open/append/layout 在 DocumentSession；caret 在 SessionCaret）  
test：`tests/` 镜像入口；`cargo test --test <function_id>`

| function_id | `cargo test --test` |
|-------------|---------------------|
| 01_function_session_open | `01_function_session_open` |
| 02_function_session_append | `02_function_session_append` |
| 03_function_caret_hit | `03_function_caret_hit` |
| 04_function_session_relayout | `04_function_session_relayout` |

## 4. 验收顺序

01 → 02 → 04 → 03。04 是可画出口；03 是光标出口。

## 5. 明确不做

选区 FillRect；IME；任意 cp 插入；改 DocxReader；从零重写 CoreHelper。
