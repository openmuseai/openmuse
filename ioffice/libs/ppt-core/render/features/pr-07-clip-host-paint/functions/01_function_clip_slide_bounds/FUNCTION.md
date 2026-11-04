# 01_function_clip_slide_bounds — 指令裁剪到 slide

> feature: pr-07-clip-host-paint ｜ 状态：**pass**

## 1. I/O 契约

| | 契约 |
|---|---|
| **输入** | `render_slide` 指令 + slide 宽高（render px，与 WhiteBackground 相同）。 |
| **输出** | 画在 slide 外的几何/文字被 clip（`intersect` 矩形或 painter `clipRect(0,0,w,h)`）。CONTENTS `xfrm x<0` 不得盖住宿主侧栏。 |
| **错误** | 完全在外的指令可丢弃或 clip 成空；不缩短页序。 |

已读：`mvp_pipeline.rs` 无 clip；负 x 原样进 JSON。Java canvas 有 clip 到 slide。

### 1.7

产品 `mvp_pipeline` / envelope。禁改 Zoom UI。

## 3. 测试

| coverage_id | test_id |
|---|---|
| C01 负 x DrawText clip 后 envelope 仍合法 | T01 |
| C02 完全在 slide 内不变 | T02 |
| C03 零尺寸 slide 不 panic | T03 |

`test: tests/cn/wps/show/render/01_function_clip_slide_bounds.rs`

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_clip_slide_bounds

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_clip_slide_bounds \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --report libs/ppt-core/render/features/pr-07-clip-host-paint/functions/01_function_clip_slide_bounds/FUNCTION.md
```

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_clip_slide_bounds --json`)
date: 2026-09-09 21:25

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/mvp_pipeline.rs` | 70/395 | 17.7% | 137/563 | 24.3% |
| **function total** | **70/395** | **17.7%** |  |  |
<!-- coverage-report:end -->

<!-- java-rust-align:begin -->
tool: n/a (product I/O; `clip_commands_to_slide` in mvp_pipeline)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (canvas clip 语义) | N/A | T01–T03 | — | — | **PASS** |

**overall: PASS** — product I/O; cmds clipped to WhiteBackground size.
<!-- java-rust-align:end -->
