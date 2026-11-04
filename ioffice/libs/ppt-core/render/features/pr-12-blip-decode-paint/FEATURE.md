# pr-12-blip-decode-paint — G9：真的画出 JPEG/PNG

> 层：S10 宿主 ｜ 状态：**spec** ｜ depends_on: pp-15-pic-blip-media; pp-18-sp-picture-fill; pr-05-visual-emit-paint
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S10 宿主 · Java drawBitmap**
> 跟踪：[06-next-features.md](../../../rewrite-review/06-next-features.md) · [PIXEL_PIPELINE.md](../../../PIXEL_PIPELINE.md)

## 0. 目前管线阶段的情况

指令已有 bytes_b64；Flutter `_drawImage` 填灰块。像素对齐图时城市页永远是灰。

## 1. 缺乏的功能与逻辑

PNG/JPEG 解码画到位图；失败走现有灰/描边占位。

## 2. 完成验收的功能子集列表

| order | function_id | 验收口径 |
|---|---|---|
| 1 | `01_function_png_paints_pixels` | 1×1 PNG 不全是占位灰 |
| 2 | `02_function_jpeg_paints` | 最小 JPEG 画出 |
| 3 | `03_function_missing_b64_placeholder` | 无字节 → 灰框，不 crash |

清单与 [functions.csv](./functions.csv) / [FUNCTIONS.md](./FUNCTIONS.md) 同步。`status=spec` 直到 loop 非 ignore 全绿。**禁止**把 `#[ignore]` 标成 pass。

### 跟踪

| date | status | note |
|---|---|---|
| 2026-09-15 | spec | 计划落盘 + function 拆分；未 loop |

## 3. I/O

| **I** | DrawImage + bytes_b64 |
| **O** | 位图像素或占位 |

crate：`libs/ppt-core` ｜ 测试 crate：`samples/libs/ppt/ppt_render`（flutter test）

## 4. Java 管线对照

| Java | Rust 归属 |
|---|---|
| GraphicsCanvas.drawBitmap | Dart painter |

## 5. 必须 / 壳 / 延后

- **must：** flutter test 三测；可用 Picture.toImage。
- **壳：** Java 位图 LRU。
- **defer：** KBitmapDev。

## 6. Feature 验收

三 function flutter 绿。

## 7. 依赖

- **upstream：** pp-15-pic-blip-media; pp-18-sp-picture-fill; pr-05-visual-emit-paint
- **loop：** 有 `// java` 的叶走 **function-java2rust**；禁止 `--force` `PptxrSlide.rs` / 健康模板。
- **不依赖：** RMSE 金页（pr-13）。
