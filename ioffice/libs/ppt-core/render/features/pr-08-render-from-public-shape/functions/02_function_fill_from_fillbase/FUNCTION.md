# 02_function_fill_from_fillbase — fill 来自 FillBase

> feature: pr-08-render-from-public-shape ｜ 状态：**spec** ｜ Java 管线：S9 几何/填线（输入必须是 S5 公开 Shape）

## 0. 管线能力解锁

今日 `render_slide:320-337` 先 getFill 再 **or_else layout_shape.sp_pr.fill()**。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | SolidFill / noFill 公开 Shape |
| **输出** | DrawRect/Path.fill 色 = FillBase；noFill 不产 fill |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| FillConvert | pl-09:05 | FillBase 色 | pass | 渐变 | 禁止渐变本 function |
| render_slide fill | mvp_pipeline:320 | 指令 fill | 仍 or_else layout | — | 禁止 layout 回退当 pass |

## 2. 核心代码与函数改写列表

**本 function：** 产品热路径只读 FillBase；禁止 layout_shape fill 回退标 pass。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 `render_slide` fill 分支：`typo.getShape().getFill()` downcast FillBase，失败则 `s.sp_pr.fill()`。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| FF-SOL | FillBase.getColor | srgb 红 | 指令 fill==ARGB | 公开入口 |
| FF-NO | fill.type 空 | a:noFill | fill=None | 关键分支 |
| FF-BG | LayerItem::Background | 页底实色 | 仍发 Background fill（回归 pp-17） | 边界 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 实色 | FillBase.getColor | 公开入口 | 是 | FF-SOL |
| noFill | fill.type 空 | 关键分支 | 是 | FF-NO |
| Background 层 | LayerItem::Background | 边界 | 是 | FF-BG |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| FF-SOL | 实色 | — | srgb 红 | 指令 fill==ARGB | 仍读 layout fill |
| FF-NO | noFill | — | a:noFill | fill=None | 画成黑/白 |
| FF-BG | Background 层 | — | 页底实色 | 仍发 Background fill（回归 pp-17） | 背景丢 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/mvp_pipeline.rs
test:  tests/cn/wps/show/render/02_function_fill_from_fillbase.rs
[[test]] name = "02_function_fill_from_fillbase"
```

`#[ignore = "spec: 02_function_fill_from_fillbase"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_fill_from_fillbase -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_fill_from_fillbase \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --report render/features/pr-08-render-from-public-shape/functions/02_function_fill_from_fillbase/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

§3.8 **N/A**（产品编排）。I/O 测试绿 + 禁止第二套类型名。

```bash
# 产品编排 / Flutter：无一对一 Java 叶。§3.8 标 N/A；不得 --force 邻近健康模板。
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  # n/a product I/O \
  --report render/features/pr-08-render-from-public-shape/functions/02_function_fill_from_fillbase/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | FF-SOL, FF-NO, FF-BG | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
