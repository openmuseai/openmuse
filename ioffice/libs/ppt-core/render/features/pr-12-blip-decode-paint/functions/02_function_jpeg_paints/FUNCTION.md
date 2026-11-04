# 02_function_jpeg_paints — JPEG 画出

> feature: pr-12-blip-decode-paint ｜ 状态：**spec** ｜ Java 管线：S10 宿主 · Java drawBitmap

## 0. 管线能力解锁

城市页 Sample 多为 JPEG。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | 最小 JPEG b64 |
| **输出** | 非占位灰 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 01 PNG 路径 | 同文件 | decode | spec | — | — |

## 2. 核心代码与函数改写列表

同一 decode 路径覆盖 JPEG。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

同 _drawImage，未分支 mime。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| JPG-PX | decode | 1x1 或 8x8 jpeg | 非灰 | 公开入口 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 最小 jpeg | decode | 公开入口 | 是 | JPG-PX |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| JPG-PX | 最小 jpeg | — | 1x1 或 8x8 jpeg | 非灰 | 仍灰/crash |

### 3.5 测试代码位置

```text
crate: samples/libs/ppt/ppt_render
src:   samples/libs/ppt/ppt_render/lib/src/render_painter.dart
test:  samples/libs/ppt/ppt_render/test/02_function_jpeg_paints.dart
[[test]] name = "02_function_jpeg_paints"
```

`#[ignore = "spec: 02_function_jpeg_paints"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cd samples/libs/ppt/ppt_render && flutter test test/02_function_jpeg_paints.dart

# Dart：见 01。
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
  --report render/features/pr-12-blip-decode-paint/functions/02_function_jpeg_paints/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | JPG-PX | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
