# 02_function_slide_solid_rmse — 白底+实色矩形 RMSE

> feature: pr-13-pixel-gate ｜ 状态：**spec** ｜ Java 管线：S10 验收 · 不是新渲染器

## 0. 管线能力解锁

先锁几何页再谈富文本。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | 自制金页：白底+一个实色矩形 + Java 参考 PNG |
| **输出** | RMSE < 约定阈值（如 2.0） |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| render_slide | pr-08 | 指令 | spec | — | 禁止在 painter 上调色冒充 |
| 金图 fixture | tests/fixtures/02_function_slide_solid_rmse | PNG | 未落地 | — | 禁止用 Sample 23 页 |

## 2. 核心代码与函数改写列表

fixture 金图 + 栅格比较。阈值写入 FUNCTION 常量。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

现状无 RMSE 工具；Split 占位测试 ignore。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| RM-THR | 金页 vs 栅格 | 白+红矩形 | RMSE<2.0 | 公开入口 |
| RM-DIFF | 故意错色 | 阈值外 | 非静默 fail + diff 路径 | 失败 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| RMSE | 金页 vs 栅格 | 公开入口 | 是 | RM-THR |
| 失败打印 | 故意错色 | 失败 | 是 | RM-DIFF |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| RM-THR | RMSE | — | 白+红矩形 | RMSE<2.0 | 结构未齐就比图 |
| RM-DIFF | 失败打印 | — | 阈值外 | 非静默 fail + diff 路径 | 假绿 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/mvp_pipeline.rs
test:  tests/cn/wps/show/render/02_function_slide_solid_rmse.rs
[[test]] name = "02_function_slide_solid_rmse"
```

`#[ignore = "spec: 02_function_slide_solid_rmse"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_slide_solid_rmse -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_slide_solid_rmse \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --report render/features/pr-13-pixel-gate/functions/02_function_slide_solid_rmse/FUNCTION.md
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
  --report render/features/pr-13-pixel-gate/functions/02_function_slide_solid_rmse/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | RM-THR, RM-DIFF | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
