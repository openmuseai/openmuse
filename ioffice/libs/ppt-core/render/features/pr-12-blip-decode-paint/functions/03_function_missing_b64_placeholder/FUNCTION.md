# 03_function_missing_b64_placeholder — 无字节占位不 crash

> feature: pr-12-blip-decode-paint ｜ 状态：**spec** ｜ Java 管线：S10 宿主 · Java drawBitmap

## 0. 管线能力解锁

现有空 b64 已描边灰框（:154-161）。解码后不得让空 b64 走 decode crash。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | DrawImage 无 bytes_b64 |
| **输出** | 灰框描边，不 crash |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 空 b64 分支 | _drawImage:154 | 占位 | 在 | — | 禁止 unwrap decode |

## 2. 核心代码与函数改写列表

保持占位；decode 失败同占位。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 :154 空 b64 描边返回。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| NB-EMPTY | bytes_b64 null | DrawImage | 不 crash 且占位 | 公开入口 |
| NB-BAD | 非法 base64 | !!!! | 占位不 crash | 失败 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 无 b64 | bytes_b64 null | 公开入口 | 是 | NB-EMPTY |
| 坏 b64 | 非法 base64 | 失败 | 是 | NB-BAD |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| NB-EMPTY | 无 b64 | — | DrawImage | 不 crash 且占位 | decode 抛 |
| NB-BAD | 坏 b64 | — | !!!! | 占位不 crash | panic |

### 3.5 测试代码位置

```text
crate: samples/libs/ppt/ppt_render
src:   samples/libs/ppt/ppt_render/lib/src/render_painter.dart
test:  samples/libs/ppt/ppt_render/test/03_function_missing_b64_placeholder.dart
[[test]] name = "03_function_missing_b64_placeholder"
```

`#[ignore = "spec: 03_function_missing_b64_placeholder"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cd samples/libs/ppt/ppt_render && flutter test test/03_function_missing_b64_placeholder.dart

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
  --report render/features/pr-12-blip-decode-paint/functions/03_function_missing_b64_placeholder/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | NB-EMPTY, NB-BAD | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
