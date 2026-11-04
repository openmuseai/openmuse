# 03_function_background_layer — has_background → LayerItem::Background

> feature: pp-17-slide-background ｜ 状态：**pass**

## 1. I/O 契约

| | 契约 |
|---|---|
| **输入** | `KmoSlide.has_background==true` 且 `background().is_some()`；`slideDrawFlags` 未置 `NOT_DRAW_BACKGROUND_SHAPE`。 |
| **输出** | `layer_order` 含 `WhiteBackground` 然后 `Background{w,h}`。`geometry_commands(Background)` 的 fill 由页 ARGB 附着（现 Background 分支 fill=None，需接 01/02 色）。 |
| **错误** | has_background=false → 无 Background 层（slide7）。 |

### 1.7 依赖

| 依赖 | 已实现 | 禁止改 |
|---|---|---|
| `drawUnderSlide` 204-206 | 分支已在；缺 has_background | 不改白底 |
| `geometry_commands` Background | fill None | 不在 03 解析 XML |

## 3. 测试

已读：`SlideRender.rs` 192-206；`rendercommand.rs` 65-72。

| coverage_id | test_id |
|---|---|
| C01 has_background → 含 Background | T01 |
| C02 false → 无 Background | T02 |
| C03 Background 指令 fill=页 ARGB | T03 |

`test: tests/cn/wps/show/render/03_function_background_layer.rs`

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_background_layer

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_background_layer \
  --src src/cn/wps/show/render/SlideRender.rs \
  --src src/cn/wps/show/render/rendercommand.rs \
  --report libs/ppt-core/parser/features/pp-17-slide-background/functions/03_function_background_layer/FUNCTION.md
```

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_background_layer --json`)
date: 2026-09-09 21:25

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/SlideRender.rs` | 54/130 | 41.5% | 79/164 | 48.2% |
| `src/cn/wps/show/render/rendercommand.rs` | 30/183 | 16.4% | 28/242 | 11.6% |
| **function total** | **84/313** | **26.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

`SlideRender.rs` vs `SlideRender.java`：**PASS**。`rendercommand.rs` 为产品指令表（无一对一 Java）。

<!-- java-rust-align:begin -->
tool: n/a (drawUnderSlide fill 附着；layout PASS)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| SlideRender.drawUnderSlide | **PASS** | T01–T03 | — | — | **PASS** |

**overall: PASS** — Background layer fill = page ARGB; layout PASS.
<!-- java-rust-align:end -->
