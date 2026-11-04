# 01_function_bg_pr_solid — p:bgPr 实色 → 页 fill ARGB

> feature: pp-17-slide-background ｜ 状态：**pass**

## 0. 管线能力解锁

`open_pptx_document_bytes` 把 `has_background` 写死 `false`，`KmoSlide::background()` 永为 None，`drawUnderSlide` 不入 Background 层。本 function 只处理 **`p:bg/p:bgPr` 实色**（srgb / schemeClr）。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | slide XML 的 `p:cSld/p:bg/p:bgPr` + theme scheme 表（pp-16）。 |
| **输出** | `has_background=true`；页级 fill ARGB（scheme 走同一张 theme 表）。 |
| **错误** | 无 `p:bg` / 无 solidFill → 本 function 不置 true（bgRef 归 02）。不 panic。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| `parse_theme_scheme_table` | pp-16 | scheme→ARGB | pass | — | 第二套颜色表 |
| `KmoSlide::background` | layout_shape.rs | Some 当 has_background | 字段有；open 写死 false | 渐变 | — |
| `BackgroundHandler` | pptxr/handler/slides | Java 叶存在 | 产品 open **不调用** | bg 图 | `--force` PptxrSlide |

## 2. 核心代码

产品：`pptx_mvp.rs` `open_pptx_document_bytes`（约 889 行）。Java：`BackgroundHandler` / `BackgroundPropertiesHandler`。改写列表：从 slide XML 抽 bgPr solidFill。停车：gradFill。禁改：PptxrSlide。

## 3. 测试方案

### 3.1 审计

已读：`pptx_mvp.rs` has_background:false；`layout_shape.rs` `background()` 293+；`SlideRender::drawUnderSlide` 204-206；Sample `slide2.xml` `schemeClr val="tx1"`。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 必测? | test_id |
|---|---|---|---|
| C01 | bgPr + srgb | 是 | T01 |
| C02 | bgPr + schemeClr（theme 表） | 是 | T02 |
| C03 | 无 p:bg | 是 | T03 |
| C04 | 仅 bgRef（本 function 不置 true） | 是 | T04 |

### 3.3–3.4

`visual_pptx` 在 slide 的 `p:cSld` 插入 `<p:bg>…`。T01 期望 has_background && fill 匹配。T03 slide7 形态：false。T04 有 bgRef 无 bgPr → 01 不负责 true。

### 3.5

`test: tests/cn/wps/show/pptxr/01_function_bg_pr_solid.rs`

### 3.6

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_bg_pr_solid

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_bg_pr_solid \
  --src src/cn/wps/show/pptxr/pptx_mvp.rs \
  --report libs/ppt-core/parser/features/pp-17-slide-background/functions/01_function_bg_pr_solid/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_bg_pr_solid --json`)
date: 2026-09-09 21:24

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/pptx_mvp.rs` | 412/900 | 45.8% | 721/1617 | 44.6% |
| **function total** | **412/900** | **45.8%** |  |  |
<!-- coverage-report:end -->

### 3.8–3.9

产品编排 N/A。未改 BackgroundHandler.rs / PptxrSlide.rs。`pptx_mvp.rs` 无一对一 Java。

<!-- java-rust-align:begin -->
tool: n/a (product I/O)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (open 编排 `parse_bg_pr_solid_fill`) | N/A | T01–T04 | 渐变 bg | — | **PASS** |

**overall: PASS** — product I/O; bgPr solidFill → has_background + ARGB.
<!-- java-rust-align:end -->
