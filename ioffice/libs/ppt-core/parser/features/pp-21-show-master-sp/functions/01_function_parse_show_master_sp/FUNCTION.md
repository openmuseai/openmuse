# 01_function_parse_show_master_sp — 解析 showMasterSp

> feature: pp-21-show-master-sp ｜ 状态：**pass** ｜ Java 管线：S1 属性 + S9 drawShapesOnLayout

## 0. 管线能力解锁

`open_pptx_document_bytes` 读 `p:sld` / `p:sldLayout` 的 `showMasterSp`（PMap 3211600），经 Java 同名 `toBoolean` 写入产品 `KmoSlide` / `KmoSlideLayout.isShowMasterShape`。缺省 **true**（`Slide.java:345-347` parent==null）。`data::Slide` / `app::KmoSlide` 仍在 `full` / `cfg(any())` 编译面外，产品 I/O 走 `pptx_mvp` 调用与 Java `SlideHandler`/`SlideLayoutHandler` 相同的 attr 循环。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | p:sld / sldLayout 属性 true/false/缺省 |
| **输出** | isShowMasterSp：缺省 **true**（Slide.java:345-347 parent==null ? true） |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名；不启用 `#[cfg(any())]` app KmoSlide。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| SlideHandler.startElement case 3211600 | pptxr/handler/slides | putShowMasterSp | 交错模板已在；默认 compile 面因 app KmoSlide 未接入 | 全 handler 编排 | 禁止 --force PptxrSlide |
| SlideLayoutHandler.startElement case 3211600 | pptxr/handler/slidelayouts | putShowMasterSp | 同上 | — | — |
| Slide.isShowMasterSp | data/slides | 缺省 true | 交错模板已在；`show::data` 需 feature=full | bit index 3 | 禁止改 map key 3 |
| 产品 slide 字段 | layout_shape::KmoSlide | isShowMasterShape | **本 function** 从 attr 注入，缺省 true | — | 禁止只改 handler 测试 struct |

## 2. 核心代码与函数改写列表

**本 function：** `attr_id("showMasterSp")=3211600`；`parse_slide_show_master_sp` / `parse_layout_show_master_sp` 按 Java handler 循环 `getAttrAt` + case 3211600 + `toBoolean`；无属性 → true。
**§3.8：** SlideHandler、SlideLayoutHandler。pptx_mvp 无一对一 Java 叶。
**禁改：** `--force` PptxrSlide；启用 `#[cfg(any())]` app KmoSlide。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Slide.java:345 缺省 true；SlideHandler.java:36 case 3211600；SlideLayoutHandler.java:68；layout_shape.rs:331 读 bool 字段。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| SM-DEF | 无 showMasterSp 属性 | 最小 sld | isShowMasterShape==true | 公开入口 |
| SM-T | showMasterSp=1 | 属性 true | true | 对照 |
| SM-F | showMasterSp=0 | 属性 false | false | 关键分支 |
| SM-LAYOUT-DEF | p:sldLayout 无属性 | 带 layout zip | layout.isShowMasterShape==true | 输入后端 |
| SM-LAYOUT-F | p:sldLayout showMasterSp=0 | 带 layout zip | layout false | 关键分支 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 缺省 | 无 showMasterSp 属性 | 公开入口 | 是 | SM-DEF |
| true | showMasterSp=1 | 对照 | 是 | SM-T |
| false | showMasterSp=0 | 关键分支 | 是 | SM-F |
| layout 缺省 | p:sldLayout 无属性 | 输入后端 | 是 | SM-LAYOUT-DEF |
| layout false | p:sldLayout=0 | 关键分支 | 是 | SM-LAYOUT-F |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| SM-DEF | 缺省 | — | 最小 sld | isShowMasterShape==true | false |
| SM-T | true | — | 属性 true | true | — |
| SM-F | false | — | 属性 false | false | 仍 true |
| SM-LAYOUT-DEF | layout 缺省 | — | sldLayout 无属性 | layout true | false |
| SM-LAYOUT-F | layout false | — | sldLayout=0 | layout false | 仍 true |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/pptxr/pptx_mvp.rs, src/cn/wps/show/pptxr/handler/slides/SlideHandler.rs, src/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs
test:  tests/cn/wps/show/pptxr/01_function_parse_show_master_sp.rs
[[test]] name = "01_function_parse_show_master_sp"
```

Loop 已落地：SM-DEF/T/F + SM-LAYOUT-DEF/F 非 ignore。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_parse_show_master_sp -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_parse_show_master_sp \
  --src src/cn/wps/show/pptxr/pptx_mvp.rs \
  --report libs/ppt-core/parser/features/pp-21-show-master-sp/functions/01_function_parse_show_master_sp/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_parse_show_master_sp --json`)
date: 2026-09-15 20:21

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/pptx_mvp.rs` | 725/1099 | 66.0% | 1201/1941 | 61.9% |
| **function total** | **725/1099** | **66.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

**PASS**（2026-09-15）：SlideHandler / SlideLayoutHandler 布局校验均 PASS。pptx_mvp 无一对一 Java 叶。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/SlideHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/SlideHandler.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/SlideHandler.java --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/SlideHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.java --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs \
  --report libs/ppt-core/parser/features/pp-21-show-master-sp/functions/01_function_parse_show_master_sp/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-15 12:20

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SlideHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/slides/SlideHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/SlideHandler.rs` | **PASS** | `SlideHandler`, `startElement`, `endElement`, `enterSubElement` | — | — | **PASS** |
| `SlideLayoutHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs` | **PASS** | `SlideLayoutHandler`, `enterSubElement`, `startElement`, `endElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
