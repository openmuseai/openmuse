# pr-12-blip-decode-paint — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**。Java **S10 宿主 · Java drawBitmap**。状态：**spec**（跟踪用，未 loop）。

## §0 管线位置

```text
S9 DrawImage 已有字节 → S10 解码（本 feature）
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_png_paints_pixels](./functions/01_function_png_paints_pixels/FUNCTION.md) | PNG 画出非灰块 | 1×1 PNG bytes_b64 | 中心像素不是占位灰 0xFFCCCCCC | pp-18;pr-05 | 02 | spec |
| 2 | [02_function_jpeg_paints](./functions/02_function_jpeg_paints/FUNCTION.md) | JPEG 画出 | 最小 JPEG b64 | 非占位灰 | 01 | — | spec |
| 3 | [03_function_missing_b64_placeholder](./functions/03_function_missing_b64_placeholder/FUNCTION.md) | 无字节占位不 crash | DrawImage 无 bytes_b64 | 灰框描边，不 crash | 01 | — | spec |

## §2 数据流

```text
bytes_b64 → decodeImageFromList → canvas.drawImage
empty/bad → placeholder
```

## §3 编译面

| function_id | CRATE_DIR | src | `cargo test --test` | tests/ |
|---|---|---|---|---|
| 01_function_png_paints_pixels | samples/libs/ppt/ppt_render | `samples/libs/ppt/ppt_render/lib/src/render_painter.dart` | `01_function_png_paints_pixels` | `samples/libs/ppt/ppt_render/test/01_function_png_paints_pixels.dart` |
| 02_function_jpeg_paints | samples/libs/ppt/ppt_render | `samples/libs/ppt/ppt_render/lib/src/render_painter.dart` | `02_function_jpeg_paints` | `samples/libs/ppt/ppt_render/test/02_function_jpeg_paints.dart` |
| 03_function_missing_b64_placeholder | samples/libs/ppt/ppt_render | `samples/libs/ppt/ppt_render/lib/src/render_painter.dart` | `03_function_missing_b64_placeholder` | `samples/libs/ppt/ppt_render/test/03_function_missing_b64_placeholder.dart` |

## §4 串联

按 csv `order`。不要跳过上游 I/O 假装绿。测 N 只用声明的上游输出或最小 fixture。

## §5 验收顺序

```text
01 → 02 → 03
```

## §6 类级对照

| Java 类 | Rust 目标 | 本 feature |
|---|---|---|
| RenderPainter._drawImage | render_painter.dart | 全部 |
