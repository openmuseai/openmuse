# 05_function_flutter_pager — Flutter 页栏与当前页

> feature: pr-04-ffi-flutter ｜ 状态：**pass**

## 0. 管线能力解锁

Flutter 能看到文档总页数、选择任意页，并只把当前页 commands 交给既有 `RenderPainter`。

## 1. 功能说明

### 1.1 I/O 契约

输入 document envelope；输出 `RenderPage` 列表、`pageIndex/currentPage` 状态、页栏和前后按钮。旧单页 golden 自动提升为一页。

### 1.2–1.6 边界

页索引 clamp；打开文档重置到 selected_page；painter 不感知文档协议。

### 1.7 依赖与实现状态

| 依赖 | 状态 | 禁止修改 |
|---|---|---|
| 03 pages JSON | pass | 不扁平合并 commands |
| RenderPainter | pass | 不在 UI 修 geometry |
| controller | pass | 不重复调用 parser 翻页 |

## 2. 核心代码

`ppt_render/lib/src/envelope.dart`、`ppt_show/lib/src/show_controller.dart`、`show_workspace.dart`。

## 3. 测试方案

### 3.1–3.5 覆盖

旧 envelope→1 page；2 pages parse；setPage clamp；next/previous；UI selected tile/current commands。

### 3.6 测试运行脚本

```bash
cd samples/libs/ppt/ppt_show && flutter test test/pager_test.dart
cd samples/ppt-render/flutter && flutter test
```

### 3.7 覆盖率报告

Flutter 无 `cargo-llvm-cov`。门禁：`pager_test.dart` 全绿（legacy 1 page、clamp、rail tap）。

<!-- coverage-report:begin -->
tool: flutter test (no llvm-cov)
date: 2026-09-02

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `samples/libs/ppt/ppt_show/lib/src/show_controller.dart` | dart tests | — | — | — |
| `samples/libs/ppt/ppt_show/lib/src/show_workspace.dart` | widget rail | — | — | — |
| **function total** | **n/a** | **n/a** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

Flutter 产品协议无 Java 移植目标。结果：**PASS（N/A）**

### 3.9 Java↔Rust 结构对齐报告

<!-- java-rust-align:begin -->
tool: n/a (Dart)
date: 2026-09-02

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (none) | N/A | setPage/next/previous + page rail | — | — | **PASS** |

**overall: PASS**
<!-- java-rust-align:end -->
