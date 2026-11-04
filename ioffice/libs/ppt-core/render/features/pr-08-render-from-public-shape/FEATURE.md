# pr-08-render-from-public-shape — 指令来自公开 Shape（S5→S9）

> 层：render ｜ 状态：**spec** ｜ depends_on: pl-09-public-shape-convert; pp-20-open-write-kmo; pr-02-shape-command
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S9 几何/填线（输入必须是 S5 公开 Shape）**
> 跟踪：[06-next-features.md](../../../rewrite-review/06-next-features.md) · [PIXEL_PIPELINE.md](../../../PIXEL_PIPELINE.md)

## 0. 目前管线阶段的情况

Java：`GemoRender.drawFill/drawStroke` 读 `shape.getGraphicsPaths` + FillBase + LineProperty。

Rust：`mvp_pipeline::render_slide` 从 layout_shape 的 `sp_pr`/`media_bytes` 摊指令。`getShape()` 已能 convert，**产品出图混用**。

## 1. 缺乏的功能与逻辑

1. geometry 以 typo.getShape() 为准。
2. fill/stroke 来自 FillBase/LineProperty（noFill 无线）。
3. 禁止 `preset_path_in_rect` 第二套几何当 pass。

## 2. 完成验收的功能子集列表

| order | function_id | 验收口径 |
|---|---|---|
| 1 | `01_function_path_from_getshape` | type1+prst → DrawPath/Rect 与公开 Shape path 一致 |
| 2 | `02_function_fill_from_fillbase` | SolidFill 色=FillBase；noFill 不产 fill |
| 3 | `03_function_line_from_lineproperty` | ln 有色；getfLine==false 无 stroke |
| 4 | `04_function_no_layout_geom_in_render_slide` | render_slide 热路径不读 sp_pr.geom_obj |

清单与 [functions.csv](./functions.csv) / [FUNCTIONS.md](./FUNCTIONS.md) 同步。`status=spec` 直到 loop 非 ignore 全绿。**禁止**把 `#[ignore]` 标成 pass。

### 跟踪

| date | status | note |
|---|---|---|
| 2026-09-15 | spec | 计划落盘 + function 拆分；未 loop |

## 3. I/O

| **I** | ShowTypoShape（getShape 已热） |
| **O** | RenderCommand DrawRect/Path/Image |
| **错误** | getShape miss → skip 该层几何 |

crate：`libs/ppt-core`

## 4. Java 管线对照

| Java | Rust 归属 |
|---|---|
| GemoRender.drawFill/drawStroke | 01-03 |
| mvp_pipeline::render_slide:290 | 全部（产品 I/O） |

## 5. 必须 / 壳 / 延后

- **must：** 四 function；回归 pr-02/06/pp-18。
- **壳：** 渐变停靠。
- **defer：** effectLst、GraphicsCanvas。

## 6. Feature 验收

矩阵全绿；回归 DrawImage 仍绿。

## 7. 依赖

- **upstream：** pl-09-public-shape-convert; pp-20-open-write-kmo; pr-02-shape-command
- **loop：** 有 `// java` 的叶走 **function-java2rust**；禁止 `--force` `PptxrSlide.rs` / 健康模板。
- **不依赖：** Flutter 解码（pr-12）；线宽 px（pr-11）。
