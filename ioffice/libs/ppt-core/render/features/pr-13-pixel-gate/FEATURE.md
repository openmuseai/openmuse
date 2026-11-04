# pr-13-pixel-gate — 与 Java 的页级像素门闩

> 层：S10 验收 ｜ 状态：**spec** ｜ depends_on: pp-20; pr-08; pr-09; pl-10; pl-11; pp-21; pl-12; pr-10; pr-11; pr-12
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S10 验收 · 不是新渲染器**
> 跟踪：[06-next-features.md](../../../rewrite-review/06-next-features.md) · [PIXEL_PIPELINE.md](../../../PIXEL_PIPELINE.md)

## 0. 目前管线阶段的情况

无 RMSE/哈希门闩；Sample e2e 只断言指令形态。

## 1. 缺乏的功能与逻辑

固定视口/字体/页：栅格 vs Java 金图。先 AABB 再 RMSE。

## 2. 完成验收的功能子集列表

| order | function_id | 验收口径 |
|---|---|---|
| 1 | `01_function_raster_size` | 页 w×h 与 Java render 单位一致 |
| 2 | `02_function_slide_solid_rmse` | 仅白底+一个实色矩形 RMSE < 阈值 |
| 3 | `03_function_sample_slide_n` | 选定 Sample 页 RMSE/IoU；不把全 23 页当唯一用例 |

清单与 [functions.csv](./functions.csv) / [FUNCTIONS.md](./FUNCTIONS.md) 同步。`status=spec` 直到 loop 非 ignore 全绿。**禁止**把 `#[ignore]` 标成 pass。

### 跟踪

| date | status | note |
|---|---|---|
| 2026-09-15 | spec | 计划落盘 + function 拆分；未 loop |

## 3. I/O

| **I** | 金图 PNG + 本侧栅格 |
| **O** | 尺寸一致 + RMSE/IoU |

crate：`libs/ppt-core`

## 4. Java 管线对照

| Java | Rust 归属 |
|---|---|
| KBitmapDev / 同页截图 fixture | 验收 |

## 5. 必须 / 壳 / 延后

- **must：** 金页稳定绿；不把 23 页当唯一用例。
- **壳：** 富文本/图表 ignore。
- **defer：** 在本 feature 修 wpslayout；Chart/Table。

## 6. Feature 验收

01–03 约定页绿。结构未齐不得 pass。

## 7. 依赖

- **upstream：** pp-20; pr-08; pr-09; pl-10; pl-11; pp-21; pl-12; pr-10; pr-11; pr-12
- **loop：** 有 `// java` 的叶走 **function-java2rust**；禁止 `--force` `PptxrSlide.rs` / 健康模板。
- **不依赖：** 新渲染器。
