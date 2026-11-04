# 01_function_stroke_width_px — 线宽 EMU→px

> feature: pr-11-line-style-paint ｜ 状态：**spec** ｜ Java 管线：S9 drawStroke

## 0. 管线能力解锁

RenderCommand 仅 `stroke: Option<u32>` 色，**无宽度字段**。Java min 12700 EMU。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | ln w=25400（2pt）；min 12700 |
| **输出** | 指令/ painter 宽度 = fromEmu；低于 min 提到 12700 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| LineProperty 宽 | pl-09:06 | EMU | 有 | compound | 禁止第二套 min 公式 |
| RenderCommand | rendercommand.rs | 宽字段 | 无 | — | 禁止只改 Flutter 硬编码 |

## 2. 核心代码与函数改写列表

**本 function：** 指令携带 stroke 宽（px）。dash 可实线 MVP。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 RenderCommand::DrawRect stroke 只有色。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| SW-2PT | fromEmu | w=25400 | 指令宽==fromEmu(25400) | 公开入口 |
| SW-MIN | w=0 或极小 | w<12700 但有线 | 按 Java min | 边界 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 25400 | fromEmu | 公开入口 | 是 | SW-2PT |
| min 12700 | w=0 或极小 | 边界 | 是 | SW-MIN |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| SW-2PT | 25400 | — | w=25400 | 指令宽==fromEmu(25400) | 仍只有色 |
| SW-MIN | min 12700 | — | w<12700 但有线 | 按 Java min | 0 宽看不见 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/rendercommand.rs, src/cn/wps/show/render/mvp_pipeline.rs
test:  tests/cn/wps/show/render/01_function_stroke_width_px.rs
[[test]] name = "01_function_stroke_width_px"
```

`#[ignore = "spec: 01_function_stroke_width_px"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_stroke_width_px -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_stroke_width_px \
  --src src/cn/wps/show/render/rendercommand.rs \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --report render/features/pr-11-line-style-paint/functions/01_function_stroke_width_px/FUNCTION.md
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
  --report render/features/pr-11-line-style-paint/functions/01_function_stroke_width_px/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | SW-2PT, SW-MIN | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
