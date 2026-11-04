# 02_function_sp_blip_drawimage — 有 bytes 即 DrawImage

> feature: pp-18-sp-picture-fill ｜ 状态：**pass**

## 1. I/O 契约

| | 契约 |
|---|---|
| **输入** | `LayerItem::Shape`：`media_bytes` 或 `media_r_id` + 绝对框（pl-08）。**不要求** `shape_type==2`。 |
| **输出** | `render_slide` 对该层发 `DrawImage{x,y,w,h,media_id,r_id,bytes_b64}`，**不再**对该层发覆盖全框的实色 `DrawRect`（避免色块盖住图）。 |
| **错误** | 有 r_id 无 bytes → 仍可 DrawImage media_id 或占位 Rect（与 pr-05 一致）；无 media 走普通几何。 |

已读：`mvp_pipeline.rs` 184-207 已对 `media_bytes.is_some() \|\| media_r_id` emit DrawImage 并 `continue`。若画面仍无图，红在 01 或绝对框为 0。本 function 门禁：type==1 的 sp 只要有 bytes 就必须 DrawImage。

### 1.7 依赖

pr-05 emit 复用；禁改 painter JPEG 解码（已有）。SVG 占位归 painter。

## 3. 测试

| coverage_id | test_id |
|---|---|
| C01 type=1 + bytes → 含 DrawImage | T01 |
| C02 无 media → 无 DrawImage | T02 |
| C03 DrawImage 框 = getShapeRect | T03 |

`test: tests/cn/wps/show/render/02_function_sp_blip_drawimage.rs`

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_sp_blip_drawimage

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_sp_blip_drawimage \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --report libs/ppt-core/parser/features/pp-18-sp-picture-fill/functions/02_function_sp_blip_drawimage/FUNCTION.md
```

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_sp_blip_drawimage --json`)
date: 2026-09-09 21:24

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/mvp_pipeline.rs` | 94/395 | 23.8% | 193/563 | 34.3% |
| **function total** | **94/395** | **23.8%** |  |  |
<!-- coverage-report:end -->

<!-- java-rust-align:begin -->
tool: n/a (product I/O; render_slide orchestrator)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (render_slide 编排) | N/A | T01–T03 | SVG | — | **PASS** |

**overall: PASS** — product I/O; type=1 + bytes emits DrawImage.
<!-- java-rust-align:end -->
