# pr-04-ffi-flutter — Function 组合图

> depends_on: `pr-02` shape-command；`pr-03` text-command  
> 状态：**01–05 pass**
> 对齐 [wr-08-ffi-flutter](../../../../word-core/layout/features/wr-08-ffi-flutter/FUNCTIONS.md)。emit 与 paint 分开，才能在没有 Flutter 时测指令 JSON。

## §0 管线位置

S6 像素输出（见 [PIPELINE_LAYOUT_RENDER_PLAN.md](../../../PIPELINE_LAYOUT_RENDER_PLAN.md) 热路径 S6 段）。上游 pr-02/pr-03 给 `RenderCommand[]`；本 feature 是「出屏入口」。**禁止** `android.graphics.Canvas` 语义外泄进内核输出。

## §1 function 清单

| order | function_id | I | O | upstream | downstream |
|---|---|---|---|---|---|
| 1 | `01_function_command_json` | `Vec<RenderCommand>` | JSON（serde roundtrip） | pr-02；pr-03 | 02 |
| 2 | `02_function_flutter_painter` | 单页指令 JSON | 屏幕像素 | 01（或 fixture） | 05 |
| 3 | `03_function_multipage_envelope` | `PptxDocument` | `page_count + pages[]`，并保留首屏兼容字段 | pp-13:04；01 | 04；05 |
| 4 | `04_function_indexed_page_ffi` | pptx bytes | 整份多页 JSON（后续可加按页查询） | 03 | 05 |
| 5 | `05_function_flutter_pager` | 多页 envelope | 页栏、当前页、前后翻页 | 03；04 | 用户 |

loop 序：`pr-01 → pr-02 → pr-03 → pr-04`。允许 pr-04 用**手工 fixture 指令**先画白底+一个矩形（不阻塞 pr-01）。

## §2 数据流

```text
Vec<RenderCommand>（DrawRect / DrawPath / DrawText）
  ─→ 01 serde → JSON（Stage3 对比物 / 跨端传输）
  ─→ 02 另仓 painter（解析 JSON）→ Flutter CustomPainter
       DrawRect → Canvas.drawRect（fill/stroke）
       DrawPath → Canvas.drawPath
       DrawText → TextPainter（按指令 x/y 逐 run 画；禁止二次断行）
  ─→ 像素
```

与 writer 对照：wr-07-03 `serialize` = pr-04-01；wr-08-01 `uniffi_export` = 发布形式（壳）；wr-08-02 `flutter_painter` = pr-04-02；wr-08-03 `interaction` = post-MVP。

## §3 测试面

- 01：指令 golden JSON + roundtrip 等值；空序列；未知字段/未知 variant 行为。
- 02：Flutter widget test + fixture JSON（DrawRect/DrawPath/DrawText 各一）；断言出现文本/矩形。
- 像素级对齐不做 CI 门闩（对齐 writer：几何对齐走指令 JSON，光栅差异不阻塞 pass）。

## §4 验收

1. 三指令 roundtrip 等值（01）。
2. 未知 variant：painter skip+log（钉死，不 assert）。
3. painter 三路径各一 fixture 用例通过。
4. 产品捷径：白底 + 矩形 + 一行字 = 第一屏像素。

## §5 loop

`01 → 02 → 03 → 04 → 05`。单页 painter 通过不再代表 feature 完成；必须由 pp-13 文档装配产出全部页面并能切换当前页。

## §6 Java/Rust 范围

| Java 类 | Rust 目标 | 裁决 |
|---|---|---|
| `android.graphics.Canvas`（语义源，不移植） | — | 只作语义参照：drawRect/drawPath/drawText 三条 → 指令语义；不建 Canvas 移植文件 |
| —（指令序列化） | `src/cn/wps/show/render/rendercommand/Serialize.rs`（新建，serde/serde_json） | `RenderCommand` 已有（pr-02-01）；to_json/from_json + roundtrip 必写 |
| —（单入口壳） | `src/cn/wps/show/render/mvp_pipeline.rs` 的 `render_slide` / `render_hello_slide` | 对齐 wr-08-01 `render_docx`；UniFFI 待 loop |
| —（painter） | `samples/ppt-mvp-render/flutter/lib/render_painter.dart` | DrawRect/DrawPath/DrawText 三条消费路径；不进 ppt-core 热路径 |

## 文档

- [functions.csv](./functions.csv)
