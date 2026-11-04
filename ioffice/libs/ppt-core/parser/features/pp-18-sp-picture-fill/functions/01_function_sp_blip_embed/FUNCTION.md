# 01_function_sp_blip_embed — p:sp 上 blipFill → media_bytes

> feature: pp-18-sp-picture-fill ｜ 状态：**pass**

## 0. 管线能力解锁

Sample 第 7 页城市图是 `p:sp` + `a:blipFill` + `r:embed`，不是 `p:pic`。pp-15 已对 pic 做 rels+zip。本 function 同一套 bytes 表，入口扩到 **sp / 组内 sp**。

## 1. I/O 契约

| | 契约 |
|---|---|
| **输入** | shape XML 内 `a:blipFill/a:blip/@r:embed`；slide rels Type=image；zip jpeg/png。 |
| **输出** | `media_r_id`；`media_bytes=Some(jpeg/png)`；`media_id>0`。`shape_type` 可仍为 1。 |
| **错误** | 缺 rel / 缺 part → r_id 可有、bytes None（不 panic）。SVG 可挂 bytes 但不在本 function 解码。 |

已读：`parse_one_sp` 687-736 已 walk blipFill 且 `blip_embed.or pic_blip_embed`；若 Sample 仍无图，缺口在 **images map 未收录该 rId** 或 `fill_obj.blip_embed` 未写入。禁止第二套 media 句柄。

### 1.7 依赖

| 依赖 | 已实现 | 禁止改 |
|---|---|---|
| pp-15 rels/zip | pass | 第二套 HashMap |
| BlipFillHandler | walk 已接 fill_obj | srcRect 精确 |

## 3. 测试

最小 zip：**仅 p:sp+blipFill+jpeg，没有 p:pic**。

| coverage_id | test_id |
|---|---|
| C01 sp+embed+png → bytes 非空 | T01 |
| C02 缺 rel → bytes None | T02 |
| C03 组内 sp 同样挂 bytes | T03 |

`test: tests/cn/wps/show/pptxr/01_function_sp_blip_embed.rs`

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_sp_blip_embed

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_sp_blip_embed \
  --src src/cn/wps/show/pptxr/pptx_mvp.rs \
  --report libs/ppt-core/parser/features/pp-18-sp-picture-fill/functions/01_function_sp_blip_embed/FUNCTION.md
```

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_sp_blip_embed --json`)
date: 2026-09-09 21:24

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/pptx_mvp.rs` | 580/900 | 64.4% | 974/1617 | 60.2% |
| **function total** | **580/900** | **64.4%** |  |  |
<!-- coverage-report:end -->

<!-- java-rust-align:begin -->
tool: n/a (product I/O; BlipFillHandler call-only, not rewritten this round)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (open 编排 `parse_one_sp` blipFill) | N/A | T01–T03 | SVG decode | — | **PASS** |

**overall: PASS** — product I/O; matrix green; `--force` PptxrSlide forbidden.
<!-- java-rust-align:end -->
