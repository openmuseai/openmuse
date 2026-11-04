# pr-10-text-span-emit — 行/run → DrawText（产品路径）

> 层：render ｜ 状态：**spec** ｜ depends_on: pl-12-textbox-live-paths; pl-05-text-typo; pl-09-public-shape-convert
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S9 TextRender**
> 跟踪：[06-next-features.md](../../../rewrite-review/06-next-features.md) · [PIXEL_PIPELINE.md](../../../PIXEL_PIPELINE.md)

## 0. 目前管线阶段的情况

产品 emit_text：eaVert 骨架竖排；横排常把首 run 样式打整行。

## 1. 缺乏的功能与逻辑

getTextRect inset；每 run 一条 DrawText；ph 提示走 pr-09 flags。

## 2. 完成验收的功能子集列表

| order | function_id | 验收口径 |
|---|---|---|
| 1 | `01_function_line_drawtext` | 一行一字 → 1 条 DrawText，原点=文本区+行偏移 |
| 2 | `02_function_multi_run_style` | 同行红+黑两 run → 两条不同色 DrawText |
| 3 | `03_function_textrect_inset` | bodyPr inset 后文本 x/y ≠ 形状外框 |

清单与 [functions.csv](./functions.csv) / [FUNCTIONS.md](./FUNCTIONS.md) 同步。`status=spec` 直到 loop 非 ignore 全绿。**禁止**把 `#[ignore]` 标成 pass。

### 跟踪

| date | status | note |
|---|---|---|
| 2026-09-15 | spec | 计划落盘 + function 拆分；未 loop |

## 3. I/O

| **I** | typo 行 + runs |
| **O** | DrawText 列表 |

crate：`libs/ppt-core`

## 4. Java 管线对照

| Java | Rust 归属 |
|---|---|
| IShowTextLayout.renderText | 01/02 |
| DataTransform inset:227 | 03 |

## 5. 必须 / 壳 / 延后

- **must：** 三测；回归 pr-07 vert。
- **壳：** 精确 glyph advance。
- **defer：** 与 Java 断行像素一致。

## 6. Feature 验收

三 function 绿。

## 7. 依赖

- **upstream：** pl-12-textbox-live-paths; pl-05-text-typo; pl-09-public-shape-convert
- **loop：** 有 `// java` 的叶走 **function-java2rust**；禁止 `--force` `PptxrSlide.rs` / 健康模板。
- **不依赖：** wpslayout。
