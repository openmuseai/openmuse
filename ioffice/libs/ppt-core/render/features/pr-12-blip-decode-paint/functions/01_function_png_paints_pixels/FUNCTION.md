# 01_function_png_paints_pixels — PNG 画出非灰块

> feature: pr-12-blip-decode-paint ｜ 状态：**spec** ｜ Java 管线：S10 宿主 · Java drawBitmap

## 0. 管线能力解锁

`_drawImage:147` 有 b64 仍 `fill 灰 0xFFCCCCCC`，不 `decodeImageFromList`。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | 1×1 PNG bytes_b64 |
| **输出** | 中心像素不是占位灰 0xFFCCCCCC |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| bytes_b64 | pp-18 | 指令字段 | pass | — | 禁止改回无字节 |
| _drawImage | render_painter.dart:147 | 解码 | 灰块 | LRU | 禁止 Java KBitmapDev |

## 2. 核心代码与函数改写列表

**本 function：** decode PNG 画到位图。§3.8 N/A（Dart）。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 render_painter.dart:147-168；draw_image_test.dart 只断言 op 名不采样像素。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| PNG-PX | decodeImageFromList | 红像素 PNG | 中心≠CCCCCC | 公开入口 |
| PNG-SZ | 绘制 rect | w/h | 画在 cmd 矩形内 | 边界 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 1x1 png | decodeImageFromList | 公开入口 | 是 | PNG-PX |
| 尺寸 | 绘制 rect | 边界 | 是 | PNG-SZ |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| PNG-PX | 1x1 png | — | 红像素 PNG | 中心≠CCCCCC | 仍灰块 |
| PNG-SZ | 尺寸 | — | w/h | 画在 cmd 矩形内 | crash |

### 3.5 测试代码位置

```text
crate: samples/libs/ppt/ppt_render
src:   samples/libs/ppt/ppt_render/lib/src/render_painter.dart
test:  samples/libs/ppt/ppt_render/test/01_function_png_paints_pixels.dart
[[test]] name = "01_function_png_paints_pixels"
```

`#[ignore = "spec: 01_function_png_paints_pixels"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cd samples/libs/ppt/ppt_render && flutter test test/01_function_png_paints_pixels.dart

# Dart：无 cargo-llvm-cov。Loop 用 Picture.toImage 采样写入 §3.7 人工表（仍禁止编造绿）。
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

§3.8 **N/A**（产品编排）。I/O 测试绿 + 禁止第二套类型名。

```bash
# 产品编排 / Flutter：无一对一 Java 叶。§3.8 标 N/A；不得 --force 邻近健康模板。
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  # n/a product I/O \
  --report render/features/pr-12-blip-decode-paint/functions/01_function_png_paints_pixels/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | PNG-PX, PNG-SZ | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
