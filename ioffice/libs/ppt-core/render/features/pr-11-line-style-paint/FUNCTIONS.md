# pr-11-line-style-paint — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**。Java **S9 drawStroke**。状态：**spec**（跟踪用，未 loop）。

## §0 管线位置

```text
S9 LineProperty → 指令宽/色 → painter
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_stroke_width_px](./functions/01_function_stroke_width_px/FUNCTION.md) | 线宽 EMU→px | ln w=25400（2pt）；min 12700 | 指令/ painter 宽度 = fromEmu；低于 min 提到 12700 | pr-08:03 | 03 | spec |
| 2 | [02_function_stroke_color](./functions/02_function_stroke_color/FUNCTION.md) | 线色 scheme/srgb | schemeClr / srgbClr ln | painter/指令色解析后 ARGB | 01;pp-16 | — | spec |
| 3 | [03_function_noline](./functions/03_function_noline/FUNCTION.md) | noFill 线无 stroke | a:ln/a:noFill | 无 stroke；回归 pr-07 空样式 | 01;pr-07 | — | spec |

## §2 数据流

```text
ln w EMU → px (min 12700)
色 scheme/srgb
noline → 无 stroke
```

## §3 编译面

| function_id | CRATE_DIR | src | `cargo test --test` | tests/ |
|---|---|---|---|---|
| 01_function_stroke_width_px | libs/ppt-core | `src/cn/wps/show/render/rendercommand.rs + mvp_pipeline.rs` | `01_function_stroke_width_px` | `tests/cn/wps/show/render/01_function_stroke_width_px.rs` |
| 02_function_stroke_color | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs` | `02_function_stroke_color` | `tests/cn/wps/show/render/02_function_stroke_color.rs` |
| 03_function_noline | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs` | `03_function_noline` | `tests/cn/wps/show/render/03_function_noline.rs` |

## §4 串联

按 csv `order`。不要跳过上游 I/O 假装绿。测 N 只用声明的上游输出或最小 fixture。

## §5 验收顺序

```text
01 → 02 → 03
```

## §6 类级对照

| Java 类 | Rust 目标 | 本 feature |
|---|---|---|
| RenderCommand | rendercommand.rs | 01 |
| mvp_pipeline | mvp_pipeline.rs | 全部 |
