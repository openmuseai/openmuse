# pr-09-drawflags-product — _begin 过滤接到 render_slide

> 层：render ｜ 状态：**spec** ｜ depends_on: pr-01-slide-layers; pr-08-render-from-public-shape
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S9 ShapeRender._begin**
> 跟踪：[06-next-features.md](../../../rewrite-review/06-next-features.md) · [PIXEL_PIPELINE.md](../../../PIXEL_PIPELINE.md)

## 0. 目前管线阶段的情况

pr-01 `begin_filter` 函数绿。产品 `render_slide(..., 0)` 五处，hidden/ph/背景过滤未进 envelope。

## 1. 缺乏的功能与逻辑

产品传入与 Java 阅读模式相当的 flags；hidden / ph 提示 / bit8 背景生效。

## 2. 完成验收的功能子集列表

| order | function_id | 验收口径 |
|---|---|---|
| 1 | `01_function_hidden_skipped` | isHidden → 无该层指令 |
| 2 | `02_function_ph_hint_flag` | 未置 drawPHolderText 时不发射提示字 |
| 3 | `03_function_not_draw_background` | flags 含 NOT_DRAW_BACKGROUND_SHAPE → 无 Background 层 |

清单与 [functions.csv](./functions.csv) / [FUNCTIONS.md](./FUNCTIONS.md) 同步。`status=spec` 直到 loop 非 ignore 全绿。**禁止**把 `#[ignore]` 标成 pass。

### 跟踪

| date | status | note |
|---|---|---|
| 2026-09-15 | spec | 计划落盘 + function 拆分；未 loop |

## 3. I/O

| **I** | KmoSlide + slide_draw_flags |
| **O** | 过滤后的 RenderCommand |
| **错误** | Skip 层静默 |

crate：`libs/ppt-core`

## 4. Java 管线对照

| Java | Rust 归属 |
|---|---|
| ShapeRender._begin isHidden | 01 |
| DrawFlags.drawPHolderText:68 | 02 |
| NOT_DRAW_BACKGROUND_SHAPE=8 | 03 |

## 5. 必须 / 壳 / 延后

- **must：** 三测；默认阅读 flags 下指令数 ≤ 现捷径（允许少画）。
- **壳：** modular group pic bit 16777216。
- **defer：** 打印/缩略图全套 flags。

## 6. Feature 验收

三 function 绿。

## 7. 依赖

- **upstream：** pr-01-slide-layers; pr-08-render-from-public-shape
- **loop：** 有 `// java` 的叶走 **function-java2rust**；禁止 `--force` `PptxrSlide.rs` / 健康模板。
- **不依赖：** 文本 span（pr-10）。
