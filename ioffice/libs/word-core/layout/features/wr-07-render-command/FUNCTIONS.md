# wr-07-render-command — RenderCommand 指令生成与序列化

> 分析日期：2026-08-22
> Java 底稿：`writer-logic/render/LayoutRender.java`（消费 TypoDocument → RenderEnv → CanvasController/RenderDrawer）+ wpslayout `DecoLineCache`（装饰画线）
> 输入：TypoSnapshot（wr-06）+ run 样式
> 输出：跨端 RenderCommand 序列（DrawText/DrawRect/DrawLine）→ JSON（wr-08 消费 / Stage3 对比）
> 切分依据：I/O 边界（文本指令 → 装饰/背景指令 → 序列化）

## 1. Java 管线分析
`LayoutRender` 持 `TypoDocument + LayoutExtraStatus + Selection`；`RenderEnv` 组装 GraphicsCanvas/CanvasController/RenderDrawer；每页/行/run 投影为 canvas 调用（drawText/drawRect/画线）。**Rust 替换点**：不再依赖 Canvas——产出**平台无关指令**（`RenderCommand`），其 JSON 即 Stage3 对比物。`DecoLineCache` 的下划线/波浪线等装饰语义指令化。

指令集（MVP 稳定版，各指令含 page/x/y/尺寸/样式）：
```
DrawText   { page, x, y, text, font_id, size, color, style(bold/italic) }
DrawRect   { page, x, y, w, h, color, kind(bg/border) }
DrawLine   { page, x1, y1, x2, y2, kind(underline/wave/…), color, width }
```

## 2. Function 清单
| order | function_id | 一句话 | 输入 | 输出 | 上游 | 下游 |
|------|-------------|--------|------|------|------|------|
| 1 | [01_function_text_command](./functions/01_function_text_command/FUNCTION.md) | run→DrawText（字体/字号/色/坐标） | TypoRun + 页坐标 | DrawText 序列 | wr-01/06; wr-04 对齐 | 03; wr-08 |
| 2 | [02_function_deco_command](./functions/02_function_deco_command/FUNCTION.md) | 装饰/背景→DrawRect/DrawLine | run 样式（下划线/背景色）+ DecoLineCache 语义 | 装饰指令 | wr-01; wpslayout DecoLineCache | 03; wr-08 |
| 3 | [03_function_serialize](./functions/03_function_serialize/FUNCTION.md) | 指令 JSON 序列化/反序列化 | RenderCommand 序列 | JSON（serde，Stage3 对比格式） | 01;02 | wr-08 |

## 3. 依赖与编译面
src（loop 新建）：`libs/word-core/src/cn/wps/moffice/writer/rendercommand/{RenderCommand,TextCommand,DecoCommand,Serialize}.rs`。依赖：`serde/serde_json`（word-core 已有）。
Java 对标：`writer-logic/render/LayoutRender.java` 的绘制分支逐个指令化。

## 4. 验收顺序
```text
01 text_command（坐标=页偏移+行 x/y；文本/样式正确）
02 deco_command（underline 直线/双线/波浪；背景矩形）
03 serialize（roundtrip；Stage3 稳定格式）
```