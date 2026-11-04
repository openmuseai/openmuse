# pr-09-drawflags-product — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**。Java **S9 ShapeRender._begin**。状态：**spec**（跟踪用，未 loop）。

## §0 管线位置

```text
S9 film
  → _begin filter（本 feature 接到产品）
  → emit
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_hidden_skipped](./functions/01_function_hidden_skipped/FUNCTION.md) | hidden 形状不进指令 | isHidden typo + render_slide(flags) | 无该层 DrawRect/Path/Text | pr-01;pr-08 | 02 | spec |
| 2 | [02_function_ph_hint_flag](./functions/02_function_ph_hint_flag/FUNCTION.md) | placeholder 提示字受 flags | ph 形状 + NOT_DRAW_PLACEHOLDER_TEXT(16) | 未置 drawPHolderText 时不发射提示字 DrawText | 01 | 03 | spec |
| 3 | [03_function_not_draw_background](./functions/03_function_not_draw_background/FUNCTION.md) | NOT_DRAW_BACKGROUND_SHAPE | flags & 8 | 无 Background 层指令 | 01;pp-17 | — | spec |

## §2 数据流

```text
render_slide(slide, READ_NORMAL_SLIDE)
  begin_filter hidden/ph
  bit8 skip Background
```

## §3 编译面

| function_id | CRATE_DIR | src | `cargo test --test` | tests/ |
|---|---|---|---|---|
| 01_function_hidden_skipped | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs + ShapeRender.rs` | `01_function_hidden_skipped` | `tests/cn/wps/show/render/01_function_hidden_skipped.rs` |
| 02_function_ph_hint_flag | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs + DrawFlags.rs` | `02_function_ph_hint_flag` | `tests/cn/wps/show/render/02_function_ph_hint_flag.rs` |
| 03_function_not_draw_background | libs/ppt-core | `src/cn/wps/show/render/SlideRender.rs + mvp_pipeline.rs` | `03_function_not_draw_background` | `tests/cn/wps/show/render/03_function_not_draw_background.rs` |

## §4 串联

按 csv `order`。不要跳过上游 I/O 假装绿。测 N 只用声明的上游输出或最小 fixture。

## §5 验收顺序

```text
01 → 02 → 03
```

## §6 类级对照

| Java 类 | Rust 目标 | 本 feature |
|---|---|---|
| ShapeRender | ShapeRender.rs | 01 |
| DrawFlags | DrawFlags.rs | 02/03 |
| SlideRender | SlideRender.rs | 03 |
