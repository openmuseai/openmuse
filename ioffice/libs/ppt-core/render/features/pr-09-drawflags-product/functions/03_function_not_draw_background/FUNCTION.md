# 03_function_not_draw_background — NOT_DRAW_BACKGROUND_SHAPE

> feature: pr-09-drawflags-product ｜ 状态：**spec** ｜ Java 管线：S9 ShapeRender._begin

## 0. 管线能力解锁

`DrawFlags.NOT_DRAW_BACKGROUND_SHAPE=8`。产品 flags=0 总画底。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | flags & 8 |
| **输出** | 无 Background 层指令 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| layer_order | SlideRender | Background 层 | pp-17 pass | — | 禁止拆掉真底色 |
| DrawFlags bit8 | DrawFlags.rs:16 | 常量 8 | 在 | — | 禁止改值 |

## 2. 核心代码与函数改写列表

**本 function：** `layer_order`/`render_slide` 尊重 bit 8。回归 pp-17 flags=0 仍有底。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 SlideRender 背景层；Java ShapeRender/SlideRender 用 flags。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| BG-OFF | render_slide(flags=8) | 有 bgPr 的页 | 无 Background 指令 | 公开入口 |
| BG-ON | 回归 pp-17 | bgPr 实色 | 有 Background | 回归 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| bit8 | render_slide(flags=8) | 公开入口 | 是 | BG-OFF |
| flags=0 | 回归 pp-17 | 回归 | 是 | BG-ON |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| BG-OFF | bit8 | — | 有 bgPr 的页 | 无 Background 指令 | 仍画底 |
| BG-ON | flags=0 | — | bgPr 实色 | 有 Background | 误杀底 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/SlideRender.rs, src/cn/wps/show/render/mvp_pipeline.rs
test:  tests/cn/wps/show/render/03_function_not_draw_background.rs
[[test]] name = "03_function_not_draw_background"
```

`#[ignore = "spec: 03_function_not_draw_background"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_not_draw_background -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_not_draw_background \
  --src src/cn/wps/show/render/SlideRender.rs \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --report render/features/pr-09-drawflags-product/functions/03_function_not_draw_background/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

Loop L5b 必须 PASS。不删 `// java`；列表外方法 `todo!("DEFERRED_NON_FUNCTION")` + 普通 `/* */`。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/util/DrawFlags.rs \
  --java koffice/presentation/core/cn/wps/show/util/DrawFlags.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/util/DrawFlags.java --rs libs/ppt-core/src/cn/wps/show/util/DrawFlags.rs \
  --report render/features/pr-09-drawflags-product/functions/03_function_not_draw_background/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | BG-OFF, BG-ON | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
