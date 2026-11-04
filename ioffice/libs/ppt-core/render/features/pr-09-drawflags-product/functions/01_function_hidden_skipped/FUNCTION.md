# 01_function_hidden_skipped — hidden 形状不进指令

> feature: pr-09-drawflags-product ｜ 状态：**spec** ｜ Java 管线：S9 ShapeRender._begin

## 0. 管线能力解锁

`begin_filter` 已测 hidden（pr-01），产品 `render_slide(..., 0)` 五处（:383/:417/:450/:455/:463）未把过滤接到出指令。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | isHidden typo + render_slide(flags) |
| **输出** | 无该层 DrawRect/Path/Text |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| begin_filter | pr-01 | Skip/Pass | 函数绿 | modular pic | 禁止改 DrawFlags 常量 |
| render_slide | mvp_pipeline | 产品 envelope | 不调 filter | — | — |

## 2. 核心代码与函数改写列表

**本 function：** `render_slide` 对每层调用 `ShapeRender::begin_filter`；Skip 不 emit。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 `ShapeRender.rs:126` `typo.isHidden()`；测试 `ShapeRender.rs` T hidden；产品入口恒 flags=0 且未调用 begin_filter。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| HD-SKIP | begin_filter+render_slide | isHidden=true | 0 条该 shape 指令 | 公开入口 |
| HD-SHOW | isHidden=false | 普通 sp | 有几何指令 | 对照 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| hidden | begin_filter+render_slide | 公开入口 | 是 | HD-SKIP |
| 可见 | isHidden=false | 对照 | 是 | HD-SHOW |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| HD-SKIP | hidden | — | isHidden=true | 0 条该 shape 指令 | 仍画出 |
| HD-SHOW | 可见 | — | 普通 sp | 有几何指令 | 全 skip |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/mvp_pipeline.rs, src/cn/wps/show/render/ShapeRender.rs
test:  tests/cn/wps/show/render/01_function_hidden_skipped.rs
[[test]] name = "01_function_hidden_skipped"
```

`#[ignore = "spec: 01_function_hidden_skipped"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_hidden_skipped -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_hidden_skipped \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --src src/cn/wps/show/render/ShapeRender.rs \
  --report render/features/pr-09-drawflags-product/functions/01_function_hidden_skipped/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

Loop L5b 必须 PASS。不删 `// java`；列表外方法 `todo!("DEFERRED_NON_FUNCTION")` + 普通 `/* */`。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/ShapeRender.rs \
  --java koffice/presentation/core/cn/wps/show/render/ShapeRender.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/ShapeRender.java --rs libs/ppt-core/src/cn/wps/show/render/ShapeRender.rs \
  --report render/features/pr-09-drawflags-product/functions/01_function_hidden_skipped/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | HD-SKIP, HD-SHOW | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
