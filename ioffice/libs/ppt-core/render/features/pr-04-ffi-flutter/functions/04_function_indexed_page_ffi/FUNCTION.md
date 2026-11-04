# 04_function_indexed_page_ffi — FFI 多页交付

> feature: pr-04-ffi-flutter ｜ 状态：**pass**

## 0. 管线能力解锁

现有 `ppt_render_pptx(bytes)` 不改 ABI 即返回多页 envelope；后续大文档可增加 `ppt_render_page(index)`，无需再次改变 JSON 模型。

## 1. 功能说明

### 1.1 I/O 契约

输入 bytes pointer/len；输出 malloc UTF-8 document JSON。null/panic/parse error 仍返回 `{ok:false,error}`；内存由 `ppt_string_free` 回收。

### 1.2–1.6 边界

`ppt_render_pptx` 返回整份 document envelope；`ppt_render_page(index)` 返回 `pages[i]` JSON（clamp）。下游 05 读 pages，不重入解析器。

### 1.7 依赖与实现状态

| 依赖 | 状态 | 暂缓 |
|---|---|---|
| 03 document JSON | pass | — |
| C ABI ownership | pass | — |
| `ppt_render_page` | pass | 大文档 streaming handle |

## 2. 核心代码

`samples/ppt-render/native/src/lib.rs::{ppt_render_pptx,ppt_render_page,ppt_string_free}`；`mvp_pipeline::render_pptx_page_json`。

## 3. 测试方案

### 3.1–3.5 覆盖

Rust native crate 编译/测试；null input；双页 JSON；free；panic 转 error。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path samples/ppt-render/native/Cargo.toml

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate samples/ppt-render/native --test ffi \
  --src src/lib.rs \
  --report libs/ppt-core/render/features/pr-04-ffi-flutter/functions/04_function_indexed_page_ffi/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test ffi --json`)
date: 2026-09-02 09:33

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/lib.rs` | 43/53 | 81.1% | 77/110 | 70.0% |
| **function total** | **43/53** | **81.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

无 Java class；C ABI 为平台桥。结果：**PASS（N/A）**

### 3.9 Java↔Rust 结构对齐报告

<!-- java-rust-align:begin -->
tool: n/a (C ABI)
date: 2026-09-02

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (none) | N/A | `ppt_render_pptx` / `ppt_render_page` / `ppt_string_free` | — | — | **PASS** |

**overall: PASS**
<!-- java-rust-align:end -->
