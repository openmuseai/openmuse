# pr-08-render-from-public-shape — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**。Java **S9 几何/填线（输入必须是 S5 公开 Shape）**。状态：**spec**（跟踪用，未 loop）。

## §0 管线位置

```text
S5 getShape
  → S9 render_slide（本 feature 改数据源）
  → JSON/Flutter
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_path_from_getshape](./functions/01_function_path_from_getshape/FUNCTION.md) | path 来自公开 Shape | type1 + prst rect/ellipse 的 typo | DrawPath/Rect 顶点与 getShape() GraphicsPath 一致 | pp-20:04;pl-09:04 | 02;04 | spec |
| 2 | [02_function_fill_from_fillbase](./functions/02_function_fill_from_fillbase/FUNCTION.md) | fill 来自 FillBase | SolidFill / noFill 公开 Shape | DrawRect/Path.fill 色 = FillBase；noFill 不产 fill | 01;pl-09:05 | 04 | spec |
| 3 | [03_function_line_from_lineproperty](./functions/03_function_line_from_lineproperty/FUNCTION.md) | stroke 来自 LineProperty | ln w EMU + getfLine | 有线：色来自 LineProperty；`getfLine()==false` 无 stroke；宽 min 12700 留给 pr-11 | 01;pl-09:06 | pr-11 | spec |
| 4 | [04_function_no_layout_geom_in_render_slide](./functions/04_function_no_layout_geom_in_render_slide/FUNCTION.md) | render_slide 热路径不读 layout geom | render_slide 源 | 不读 KmoShowShape.sp_pr.geom_obj（grep 门闩） | 01;02;03 | pr-09 | spec |

## §2 数据流

```text
typo.getShape()
  → GraphicsPath → DrawPath
  → FillBase → fill
  → LineProperty → stroke 有无
```

## §3 编译面

| function_id | CRATE_DIR | src | `cargo test --test` | tests/ |
|---|---|---|---|---|
| 01_function_path_from_getshape | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs + rendercommand.rs` | `01_function_path_from_getshape` | `tests/cn/wps/show/render/01_function_path_from_getshape.rs` |
| 02_function_fill_from_fillbase | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs` | `02_function_fill_from_fillbase` | `tests/cn/wps/show/render/02_function_fill_from_fillbase.rs` |
| 03_function_line_from_lineproperty | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs` | `03_function_line_from_lineproperty` | `tests/cn/wps/show/render/03_function_line_from_lineproperty.rs` |
| 04_function_no_layout_geom_in_render_slide | libs/ppt-core | `src/cn/wps/show/render/mvp_pipeline.rs + rendercommand.rs` | `04_function_no_layout_geom_in_render_slide` | `tests/cn/wps/show/render/04_function_no_layout_geom_in_render_slide.rs` |

## §4 串联

按 csv `order`。不要跳过上游 I/O 假装绿。测 N 只用声明的上游输出或最小 fixture。

## §5 验收顺序

```text
01 → 02 → 03 → 04
```

## §6 类级对照

| Java 类 | Rust 目标 | 本 feature |
|---|---|---|
| GemoRender | 产品 mvp_pipeline（无 1:1 叶） | 01-03 |
| ShowTypoShape | ShowTypoShape.rs | 输入 |
