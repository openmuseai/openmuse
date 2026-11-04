# 02_function_slide_graph_assembly — 每页结构树装配

> feature: pp-13-document-assembly ｜ 状态：**pass**

## 0. 管线能力解锁

逐个消费 01 的 slide partName，输出等长、等序的 `Vec<KmoSlide>`；每页至少形成白底、尺寸和 shape tree 骨架。

## 1. 功能说明

### 1.1 I/O 契约

输入 ordered part names + XML bytes；输出 ordered `KmoSlide.spTree`。单页损坏返回含 partName 的错误，不静默缩短页数。

### 1.2–1.6 边界

上游 pp-07:03、pp-02:05；下游 03。当前可执行桥接已遍历全部 `<p:sp>`；`pic/grpSp/cxnSp/graphicFrame` 必须通过 pp-02 handler 接回，不能在 renderer 猜。

### 1.7 依赖与实现状态

| 依赖 | 需要 | 状态 | 暂缓 |
|---|---|---|---|
| pp-02 | 五类 shape dispatch | leaf pass；产品桥 `pptx_mvp` 按 tag 分发 | `Sw*Handler` 编译面仍未接入 crate |
| pp-07 | part stream → handler | spec | 正式 UnifiedXmlReader |
| 01 | ordered names | pass | — |

## 2. 核心代码

Rust：`PptxReader.rs::open_pptx_document_bytes`。Java 权威：`PptxrSlides.open`、`PptxrSlide.open`；正式改写必须继续使用 function-java2rust 模板与原注释。

## 3. 测试方案

### 3.1–3.5 覆盖

双页顺序/文本区分；Sample_ppt 23 页；缺失 slide part 报错；页数与名称/slide 数组等长。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_slide_graph_assembly

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_slide_graph_assembly \
  --src src/cn/wps/show/pptxr/pptx_mvp.rs \
  --report libs/ppt-core/parser/features/pp-13-document-assembly/functions/02_function_slide_graph_assembly/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_slide_graph_assembly --json`)
date: 2026-09-02 09:33

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/pptx_mvp.rs` | 407/477 | 85.3% | 738/865 | 85.3% |
| **function total** | **407/477** | **85.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

组合装配在 `pptx_mvp.rs`（无一对一 Java class）。`PptxrSlide.rs` / `PptxrSlides.rs` 已有交错模板，本 function **禁止 `--force` 重生**；类级布局由 pp-08/09 上游 loop 负责。

```bash
# 无本 function 独占 Java 叶；不跑 check-template-layout 替代上游类。
```

结果：**PASS（N/A 组合桥）**

### 3.9 Java↔Rust 结构对齐报告

```bash
# 无本 function Java/rs 对；行为门禁见 tests/cn/wps/show/pptxr/02_function_slide_graph_assembly.rs
```

<!-- java-rust-align:begin -->
tool: n/a (orchestrator)
date: 2026-09-02

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (none; I/O via `open_pptx_document_bytes`) | N/A | ordered pages + sp/pic/cxn/grp/graphic dispatch | — | — | **PASS** |

**overall: PASS** — behavior tests T01–T04; no Java leaf owned by this function.
<!-- java-rust-align:end -->
