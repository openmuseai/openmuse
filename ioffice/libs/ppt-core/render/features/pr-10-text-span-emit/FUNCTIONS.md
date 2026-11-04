# pr-10-text-span-emit — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**。Java **S9 TextRender**。状态：**spec**（跟踪用，未 loop）。

## §0 管线位置

```text
S7 行几何 → S9 emit_text（本 feature）
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_line_drawtext](./functions/01_function_line_drawtext/FUNCTION.md) | 一行一字 → 1 条 DrawText | GreedyWrap 一行 + 单 run | 1 条 DrawText，原点=文本区+行偏移 | pl-12;pl-05;pl-09:07 | 02 | spec |
| 2 | [02_function_multi_run_style](./functions/02_function_multi_run_style/FUNCTION.md) | 同行多 run 分色 | 同行红+黑两 run | 两条 DrawText 颜色不同 | 01 | — | spec |
| 3 | [03_function_textrect_inset](./functions/03_function_textrect_inset/FUNCTION.md) | bodyPr inset 后文本原点 | bodyPr lIns/tIns | 文本 x/y ≠ 形状外框；默认 inset Java 91440/45720 EMU | 01;pl-04 | — | spec |

## §2 数据流

```text
textRect+inset → 行 y → 每 run DrawText
```

## §3 编译面

| function_id | CRATE_DIR | src | `cargo test --test` | tests/ |
|---|---|---|---|---|
| 01_function_line_drawtext | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs` | `01_function_line_drawtext` | `tests/cn/wps/show/render/01_function_line_drawtext.rs` |
| 02_function_multi_run_style | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs` | `02_function_multi_run_style` | `tests/cn/wps/show/render/02_function_multi_run_style.rs` |
| 03_function_textrect_inset | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs + DataTransform covertTextBodyProperties` | `03_function_textrect_inset` | `tests/cn/wps/show/render/03_function_textrect_inset.rs` |

## §4 串联

按 csv `order`。不要跳过上游 I/O 假装绿。测 N 只用声明的上游输出或最小 fixture。

## §5 验收顺序

```text
01 → 02 → 03
```

## §6 类级对照

| Java 类 | Rust 目标 | 本 feature |
|---|---|---|
| mvp_pipeline emit_text | mvp_pipeline.rs（N/A 布局） | 全部 |
