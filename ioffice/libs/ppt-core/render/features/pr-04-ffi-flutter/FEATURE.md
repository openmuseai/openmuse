# pr-04-ffi-flutter — 指令到 Flutter

> 层：render ｜ 状态：**pass** ｜ Java：语义源 `android.graphics.Canvas`（不移植）；对照 writer [wr-08](../../../../word-core/layout/features/wr-08-ffi-flutter/FUNCTIONS.md)

## 目标

让 `RenderCommand[]` 出内核、上屏幕。两个出口：**01** 指令 JSON 序列化/反序列化（serde，跨端可消费，Stage3 对比格式）；**02** Flutter painter 消费 JSON 画出**第一屏像素**（产品捷径出口）。可另包：`samples/ppt-mvp-render`（对照 `samples/docx-mvp-render`）；ppt-core 保证 JSON schema 稳定。

**禁止**把 `android.graphics.Canvas`/`Paint` 语义写进内核输出：Java Canvas 只作语义来源，指令 JSON 里只有平台无关字段（DrawRect/DrawPath/DrawText + 颜色/样式/坐标）。

## I/O（锁死）

| function | I | O | 契约 |
|---|---|---|---|
| 01 command_json | `Vec<RenderCommand>`（pr-02/pr-03 产物 或 fixture） | JSON 字符串（serde）+ 反序列化 roundtrip | 未知 variant 不落指令；字段名与 wr-07 对齐待 loop；JSON 即 Stage3 对比物 |
| 02 flutter_painter | 指令 JSON（fixture 或 01） | 屏幕像素（Flutter Canvas/TextPainter） | DrawRect→Flutter `Canvas.drawRect`、DrawPath→`Canvas.drawPath`、DrawText→TextPainter（有 xpos 时按指令坐标逐 run 画，禁止二次断行）；尺度 = 渲染 px 直用 |

错误语义：02 对未知 variant skip+log（钉死）；Rust 侧序列化失败 → Err 非 panic。

## 能力解锁

- `01_function_command_json`：指令出内核（JSON）→ **跨端可消费、可做 Stage3 对比**（主文档 pr-04 `command_json` 行）。
- `02_function_flutter_painter`：JSON → 像素 → **第一屏像素出现**（产品捷径出口；缺失 = **出屏断**，主文档断点表）。

## 必须 / 壳 / 延后

- **must：** 01 的 serde 序列化/反序列化 + roundtrip；02 的 DrawRect/DrawPath/DrawText 三条消费路径与未知 variant skip+log；`samples/ppt-mvp-render` 骨架。
- **壳：** UniFFI 导出入口（对齐 wr-08-01 `render_docx(bytes)` 单入口；MVP 可先用 JSON 文件/内存串，UniFFI 待 loop）。
- **defer：** 滚动/缩放/页间隙交互（对齐 wr-08-03，MVP 固定第一页）；DPR 密度换算细节（对齐口径与 writer `PIXEL_ALIGNMENT_ANDROID_VS_FLUTTER.md`）；flip 镜像；位图/纹理。

## 范围

- **in：** RenderCommand serde（含 roundtrip 测试）；JSON 消费（DrawRect/DrawPath/DrawText）；fixture 指令直画（白底+一个矩形+一行字，不阻塞 pr-01）。
- **out：** Canvas 语义进内核；真实图片解码；交互；像素级一致 CI 门闩（几何对齐走指令 JSON）。

## 验收

FUNCTIONS.md §4：

1. `Vec<RenderCommand>` → JSON → 反序列化 roundtrip 等值。
2. 未知 command variant：painter skip+log（较 assert 更稳）。
3. painter 三条路径（DrawRect/DrawPath/DrawText）各至少一个 fixture 用例。
4. MVP 场景：白底 + 一个矩形 + 一行字 = 第一屏像素（widget test 可测）。

**建议 MVP pass 口径（loop 采用）：** 01 必测 = 三指令 roundtrip + 空序列/未知字段；02 必测 = 三消费路径 fixture；DPR/交互 `#[ignore = "spec: post-MVP"]`。

## 依赖

- **depends：** pr-02（DrawRect/DrawPath shape 指令）；pr-03（DrawText）；`RenderCommand` 模型（pr-02-01 定义）。允许 fixture 指令先行（不阻塞 pr-01）。
- **does not depend：** pp/pl 全链路（fixture 指令即可画第一屏）。

## 不过扩展

禁止在内核 crate 出现 Flutter/Canvas 依赖（painter 只放 `samples/ppt-mvp-render/flutter`）；禁止用第二个排版引擎重排文本（DrawText 到像素只走字形，不二次断行）；禁止改 pr-02/03 指令字段迁就 Flutter 命名——schema 以内核为准，painter 适配。