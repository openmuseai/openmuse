# 02_function_flutter_painter — Flutter 消费指令

> feature: pr-04-ffi-flutter ｜ 状态：**pass** ｜ Java 管线环节：S6（语义源 `android.graphics.Canvas` 的 drawRect/drawPath/drawText，**不移植**）

## 0. 管线能力解锁

实现后：**第一屏像素出现**——白底 + 矩形 + 一行字可以由 Flutter 画出（产品捷径出口）。宿主：`samples/ppt-mvp-render`（对照 `samples/docx-mvp-render`，v0 无 FRB）。

## 1. 功能说明

`JSON → Flutter CustomPainter → Canvas/TextPainter → 像素`。三条消费路径对应指令三变体。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | 指令 JSON（01 输出、`hello.commands.json`、或手工 fixture）。 |
| **输出** | 屏幕像素：`DrawRect` → `Canvas.drawRect`（fill/stroke）；`DrawPath` → `Canvas.drawPath`；`DrawText` → `TextPainter` 按指令 x/y 绘制。 |
| **状态与副作用** | 未知 command variant → **skip + log**（钉死，不 assert）。 |
| **错误语义** | 解析失败 → 该帧不画该命令 + log，不崩整树。 |

### 1.2-1.6 已并入 1.1（paint 纯消费）

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 指令 JSON | pr-04-01 | 三命令字段 | pass | — | 不改模型迁就 Flutter 命名 |
| Flutter 环境 | samples/ppt-mvp-render/flutter | CustomPainter/TextPainter | pass | FRB | 不进 ppt-core |
| 字体度量 | Flutter TextPainter | 按指令字形绘制 | 平台自带 | xpos 细节 | 禁止二次断行重排 |
| 编排入口 | ppt-core `mvp_pipeline::render_slide` | fixture → 指令 | pass | PptxReader.open | 不把 Canvas 塞进内核 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java（语义源） | `android.graphics.Canvas` 的 `drawRect`/`drawPath`/`drawText`（不移植） |
| Dart | `samples/ppt-mvp-render/flutter/lib/render_painter.dart` |
| 内核编排 | `libs/ppt-core/src/cn/wps/show/render/mvp_pipeline.rs` |

改写列表：painter 三路径 + unknown skip+log；`RenderCommand` 的 JSON parse（Dart 侧）。

停车列表：滚动/缩放、图片位图、GPU 缓存、FRB。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| DrawRect | fill/stroke JSON | 画矩形 | 缺色默认描边 |
| DrawPath | path JSON | 画路径 | 空 path |
| DrawText | text+样式 JSON | 画字 | 字体缺失 fallback |
| unknown | 未知 variant | skip+log | 不崩 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| PA1 | DrawRect → 有矩形 | 主路径 | 是 | T01 |
| PA2 | DrawPath → 有路径 | 主路径 | 是 | T02 |
| PA3 | DrawText → 有文本 | 主路径 | 是 | T03 |
| PA4 | unknown variant → skip+log | 容错 | 是 | T04 |
| PA5 | 尺度/DPR 换算 | 对齐 | 否 | ignore post-MVP |

### 3.3 可测试性

Flutter unit test + fixture JSON；金文件 `assets/hello.commands.json` 由 `ppt_mvp_render` dump。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | PA1 | DrawRect fixture | ops 含 DrawRect | 空白 |
| T02 | PA2 | DrawPath fixture | 有 path | 空白 |
| T03 | PA3 | DrawText "Hello" | ops 含文本 | 空白 |
| T04 | PA4 | 未知 variant 混入 | 跳过其余照画 | 崩/缺画 |

### 3.5 测试代码位置

```text
crate: samples/ppt-mvp-render/flutter
src:   lib/render_painter.dart
test:  test/render_painter_test.dart
       test/hello_asset_test.dart
```

### 3.6 测试运行脚本

```bash
cd samples/ppt-mvp-render/flutter && flutter test

cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_mvp_hello_slide

cargo run --manifest-path samples/ppt-mvp-render/native/Cargo.toml -- \
  samples/ppt-mvp-render/flutter/assets/hello.commands.json
```

Dart 侧不以 cargo-llvm-cov 记。内核编排覆盖：

```bash
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_mvp_hello_slide \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --report libs/ppt-core/render/features/pr-04-ffi-flutter/functions/02_function_flutter_painter/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_mvp_hello_slide --json`)
date: 2026-08-30 20:52

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/mvp_pipeline.rs` | 171/177 | 96.6% | 223/230 | 97.0% |
| **function total** | **171/177** | **96.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

本 function 无 Java 文件（语义源 Canvas 不移植）。Dart painter 无 `// java` 交错模板。

Loop 结果：N/A（无 Java 叶）→ 不阻塞 pass。

### 3.9 Java↔Rust 结构对齐报告

无 Java 类可对齐。对照 wr-08-02 语义：DrawRect/DrawPath/DrawText + unknown skip。

```bash
# 无 --java/--rs 可跑；总体按 Dart 三路径 + T01–T04 记 PASS
```

<!-- java-rust-align:begin -->
| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
| (none — Canvas not ported) | N/A | RenderPainter.paint DrawRect/DrawPath/DrawText | — | — | PASS |
overall: PASS
<!-- java-rust-align:end -->
