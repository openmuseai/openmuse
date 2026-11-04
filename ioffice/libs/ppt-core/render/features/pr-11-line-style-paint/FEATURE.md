# pr-11-line-style-paint — 线宽/线色进入 painter

> 层：render+Flutter ｜ 状态：**spec** ｜ depends_on: pr-08-render-from-public-shape
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S9 drawStroke**
> 跟踪：[06-next-features.md](../../../rewrite-review/06-next-features.md) · [PIXEL_PIPELINE.md](../../../PIXEL_PIPELINE.md)

## 0. 目前管线阶段的情况

指令 stroke 常只有有无色。Java 线宽 min 12700 EMU。

## 1. 缺乏的功能与逻辑

DrawRect/Path 的 stroke 宽 px 与色；dash MVP 实线。

## 2. 完成验收的功能子集列表

| order | function_id | 验收口径 |
|---|---|---|
| 1 | `01_function_stroke_width_px` | ln w=25400 → 宽度=fromEmu |
| 2 | `02_function_stroke_color` | scheme/srgb 线色 |
| 3 | `03_function_noline` | noFill 线 → 无 stroke |

清单与 [functions.csv](./functions.csv) / [FUNCTIONS.md](./FUNCTIONS.md) 同步。`status=spec` 直到 loop 非 ignore 全绿。**禁止**把 `#[ignore]` 标成 pass。

### 跟踪

| date | status | note |
|---|---|---|
| 2026-09-15 | spec | 计划落盘 + function 拆分；未 loop |

## 3. I/O

| **I** | LineProperty |
| **O** | 指令宽+色；noline 无 stroke |

crate：`libs/ppt-core`

## 4. Java 管线对照

| Java | Rust 归属 |
|---|---|
| GemoRender.drawStroke | 全部 |

## 5. 必须 / 壳 / 延后

- **must：** 三测；可 flutter test。
- **壳：** dash/compound。
- **defer：** 渐变线。

## 6. Feature 验收

三 function 绿。

## 7. 依赖

- **upstream：** pr-08-render-from-public-shape
- **loop：** 有 `// java` 的叶走 **function-java2rust**；禁止 `--force` `PptxrSlide.rs` / 健康模板。
- **不依赖：** 位图（pr-12）。
