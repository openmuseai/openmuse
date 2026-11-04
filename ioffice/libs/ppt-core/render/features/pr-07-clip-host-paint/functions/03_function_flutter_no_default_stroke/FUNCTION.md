# 03_function_flutter_no_default_stroke — 空样式不默认黑描边

> feature: pr-07-clip-host-paint ｜ 状态：**pass** ｜ crate：`samples/libs/ppt/ppt_render`

## 1. I/O 契约

| | 契约 |
|---|---|
| **输入** | `DrawRect`/`DrawPath` JSON：`fill==null && stroke==null`。 |
| **输出** | `_paintPair` **return**（不 `draw` 黑 stroke）。有 fill 或 stroke 时行为不变。DrawImage 不走 _paintPair。 |
| **错误** | 不得用「总是填白 fill」冒充。 |

已读：`samples/libs/ppt/ppt_render/lib/src/render_painter.dart` 73-90。

## 3. 测试

Dart：`samples/libs/ppt/ppt_render/test/no_default_stroke_test.dart`（已有 ignore 骨架）。无 ppt-core `[[test]]`。

```bash
cd samples/libs/ppt/ppt_render && flutter test test/no_default_stroke_test.dart
```

| coverage_id | test_id |
|---|---|
| C01 空 fill/stroke 不 stroke 黑 | T01 |
| C02 有 fill 仍 fill | T02 |
| C03 有 stroke 仍 stroke | T03 |

Loop 后非 ignore。`flutter test test/no_default_stroke_test.dart` 全绿。

<!-- coverage-report:begin -->
tool: flutter test (Dart painter; not cargo-llvm-cov)
date: 2026-09-09 21:24

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `lib/src/render_painter.dart` | n/a | **PASS via flutter test** |  |  |
| **function total** | **n/a** | **PASS via flutter test** |  |  |
<!-- coverage-report:end -->

<!-- java-rust-align:begin -->
tool: n/a (Flutter)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (none) | N/A | T01–T03 | — | — | **PASS** |

**overall: PASS** — `_paintPair` returns on empty fill+stroke; flutter test green.
<!-- java-rust-align:end -->
