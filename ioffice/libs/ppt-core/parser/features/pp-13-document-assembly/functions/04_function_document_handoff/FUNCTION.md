# 04_function_document_handoff — `PptxDocument` 交付

> feature: pp-13-document-assembly ｜ 状态：**pass**

## 0. 管线能力解锁

以一个文档对象同时交付 slide dimensions、ordered part names 与 pages，终止 `open_pptx_bytes().next()` 的单页截断。

## 1. 功能说明

### 1.1 I/O 契约

输入 01–03 输出；输出 `PptxDocument { slide_width, slide_height, slide_names, slides }`。三个页级数组必须等长，空页集返回 SuffixError。

### 1.2–1.6 边界

下游仅 pr-04:03；旧 `open_pptx_bytes` 保留为兼容首屏 adapter，不能再作为文档协议。

### 1.7 依赖与实现状态

| 依赖 | 状态 | 本 function 处理 |
|---|---|---|
| 01 | pass | 页序/名称 |
| 02 | pass | pages |
| 03 | pass | `resolve_pages` 在 `open_pptx_document_bytes` 内调用 |

## 2. 核心代码

`PptxReader.rs::PptxDocument`、`open_pptx_document_bytes/path`。Java 对标 `KmoPresentation + PptxrSlides` 的文档所有权，不新增文本语义。

## 3. 测试方案

### 3.1–3.5 覆盖

Sample_ppt 精确 23 页；双页 rId 反序；空 slide 列表错误；兼容 adapter 仍返回第一页。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_document_handoff

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_document_handoff \
  --src src/cn/wps/show/pptxr/pptx_mvp.rs \
  --src src/cn/wps/show/layout/page_inheritance.rs \
  --report libs/ppt-core/parser/features/pp-13-document-assembly/functions/04_function_document_handoff/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_document_handoff --json`)
date: 2026-09-02 09:33

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/layout/page_inheritance.rs` | 37/37 | 100.0% | 58/58 | 100.0% |
| `src/cn/wps/show/pptxr/pptx_mvp.rs` | 426/477 | 89.3% | 764/865 | 88.3% |
| **function total** | **463/514** | **90.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

`PptxDocument` 为 Rust 文档 DTO，无一对一 Java 文件。结果：**PASS（N/A）**

### 3.9 Java↔Rust 结构对齐报告

<!-- java-rust-align:begin -->
tool: n/a (document DTO)
date: 2026-09-02

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (none; `PptxDocument` + `open_pptx_document_bytes/path`) | N/A | dimensions + names + slides + resolve_pages | — | — | **PASS** |

**overall: PASS**
<!-- java-rust-align:end -->
