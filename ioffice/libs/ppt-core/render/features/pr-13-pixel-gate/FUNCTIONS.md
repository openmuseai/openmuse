# pr-13-pixel-gate — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**。Java **S10 验收 · 不是新渲染器**。状态：**spec**（跟踪用，未 loop）。

## §0 管线位置

```text
结构对齐完成后的验收闸，不是新管线阶段
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_raster_size](./functions/01_function_raster_size/FUNCTION.md) | 栅格尺寸对齐 Java 单位 | slideSize EMU + 视口 | 页 w×h 与 pl-01 / Java render 单位一致 | pl-01;Batch A+B | 02 | spec |
| 2 | [02_function_slide_solid_rmse](./functions/02_function_slide_solid_rmse/FUNCTION.md) | 白底+实色矩形 RMSE | 自制金页：白底+一个实色矩形 + Java 参考 PNG | RMSE < 约定阈值（如 2.0） | 01;pr-08;pr-11 | 03 | spec |
| 3 | [03_function_sample_slide_n](./functions/03_function_sample_slide_n/FUNCTION.md) | 选定 Sample 页门闩 | 选定页（建议 slide2 组色块或自制金页） | RMSE/边缘 IoU；失败打印 diff；富文本/图表页 ignore 到后波 | 02;pr-12;pp-21;pl-10 | — | spec |

## §2 数据流

```text
lock raster size
  solid-rect RMSE
  one sample slide RMSE/IoU
```

## §3 编译面

| function_id | CRATE_DIR | src | `cargo test --test` | tests/ |
|---|---|---|---|---|
| 01_function_raster_size | libs/ppt-core | `src/cn/wps/show/layout/slide_viewport.rs` | `01_function_raster_size` | `tests/cn/wps/show/layout/01_function_raster_size.rs` |
| 02_function_slide_solid_rmse | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs` | `02_function_slide_solid_rmse` | `tests/cn/wps/show/render/02_function_slide_solid_rmse.rs` |
| 03_function_sample_slide_n | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs` | `03_function_sample_slide_n` | `tests/cn/wps/show/render/03_function_sample_slide_n.rs` |

## §4 串联

按 csv `order`。不要跳过上游 I/O 假装绿。测 N 只用声明的上游输出或最小 fixture。

## §5 验收顺序

```text
01 → 02 → 03
```

## §6 类级对照

| Java 类 | Rust 目标 | 本 feature |
|---|---|---|
| 验收脚本 | tests + fixture | 全部 |
