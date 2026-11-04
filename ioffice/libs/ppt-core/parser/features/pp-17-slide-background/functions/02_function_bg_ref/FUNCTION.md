# 02_function_bg_ref — p:bgRef + theme → 页 fill ARGB

> feature: pp-17-slide-background ｜ 状态：**pass**

## 1. I/O 契约

| | 契约 |
|---|---|
| **输入** | `p:bg/p:bgRef idx="…"` + theme 实色槽（母版常见）。本波只认能落到 ARGB 的 idx，不解析 theme 填充列表全表。 |
| **输出** | 与 01 相同：`has_background=true` + fill ARGB。 |
| **错误** | 无法解析的 idx → 不置 true，不 panic。有 bgPr 时 01 优先（不双重覆盖成错色）。 |

### 1.7 依赖

| 依赖 | 需要 | 已实现 | 禁止改 |
|---|---|---|---|
| pp-16 scheme / style matrix 实色 | idx→ARGB | 产品未绑 bgRef | 不重写 theme parse |
| 01 | 无 bgPr 才走 02 | spec | — |

Java：`BackgroundHandler` `StyleMatrixReferenceHandler`（3211604）。Rust：`BackgroundHandler.rs` 已有 bgRef 分支，**open 不调用**。

## 3. 测试方案

已读：`BackgroundHandler.rs` 97-99。

| coverage_id | 必测? | test_id |
|---|---|---|
| C01 合法 bgRef+theme | 是 | T01 |
| C02 坏 idx | 是 | T02 |
| C03 同时有 bgPr：01 色优先 | 是 | T03 |

`test: tests/cn/wps/show/pptxr/02_function_bg_ref.rs`

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_bg_ref

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_bg_ref \
  --src src/cn/wps/show/pptxr/pptx_mvp.rs \
  --report libs/ppt-core/parser/features/pp-17-slide-background/functions/02_function_bg_ref/FUNCTION.md
```

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_bg_ref --json`)
date: 2026-09-09 21:24

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/pptx_mvp.rs` | 424/900 | 47.1% | 749/1617 | 46.3% |
| **function total** | **424/900** | **47.1%** |  |  |
<!-- coverage-report:end -->

<!-- java-rust-align:begin -->
tool: n/a (product I/O; BackgroundHandler not on open path)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (open 编排 `parse_bg_ref_fill`) | N/A | T01–T03 | theme 填充列表全表 | — | **PASS** |

**overall: PASS** — product I/O; bgRef idx/scheme → ARGB; bgPr wins.
<!-- java-rust-align:end -->
