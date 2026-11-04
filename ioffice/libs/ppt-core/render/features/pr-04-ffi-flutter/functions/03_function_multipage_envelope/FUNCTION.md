# 03_function_multipage_envelope — 多页文档协议

> feature: pr-04-ffi-flutter ｜ 状态：**pass**

## 0. 管线能力解锁

把 `PptxDocument.slides[]` 逐页渲染为 `pages[]`，显式交付 `page_count`，消除原 envelope 只能表示第一页的问题。

## 1. 功能说明

### 1.1 I/O 契约

输入 `PptxDocument`；输出 `MvpEnvelope { page_count, selected_page, pages[] }`。为旧 consumer 保留顶层 width/height/commands，值严格等于 pages[0]。

### 1.2–1.6 边界

每页独立命令数组，顺序等于 pp-13 页序；本层不补 shape、不改 geometry、不二次断行。

### 1.7 依赖与实现状态

| 依赖 | 需要 | 状态 |
|---|---|---|
| pp-13:04 | ordered pages | wired |
| pr-01..03 | `render_slide` | pass |
| serde | document JSON | pass |

## 2. 核心代码

`mvp_pipeline.rs::{MvpPageEnvelope,envelope_from_document,render_pptx_pages,render_pptx_envelope_json}`。

## 3. 测试方案

### 3.1–3.5 覆盖

双页 fixture：page_count=2、pages.len=2、selected_page=0、每页含白底/shape 命令；Sample_ppt=23。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_pptx_reader t04_document_uses_presentation_relationship_order
```

### 3.7 覆盖率报告

行为回归 PASS；独立 coverage 将在测试重命名为 `03_function_multipage_envelope` 后生成。

### 3.8 模板布局校验

协议为 Rust/FFI 新增，无 Java 移植目标；Java Canvas 仍非移植对象。

### 3.9 Java↔Rust 结构对齐报告

N/A；以 JSON 字段与 pages/commands invariants 作为对齐门，结果 PASS。
