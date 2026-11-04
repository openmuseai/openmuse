# 01_function_run_width — 逐 run 宽度缓存

| 项 | 值 |
|----|----|
| feature | wr-04-line-break |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | cache_run_widths(runs) -> Vec<i32> |
| 独立性 | 上游宽度可 fixture |
| 测试状态 | pass |

## 1. 功能说明

### 1.1 一句话

对每个 run 调度量一次并缓存，避免 wrap 循环重复测。

### 1.2 输入

| 入口 | 输入 | Java 路径 |
|------|------|------|
| cache | run 文本列表 | LineLayouter 热循环 |

### 1.3 输出

| 字段/副作用 | 含义 |
|------|------|
| widths | 与 runs 等长 |

### 1.4 上游

wr-03 或假度量。

### 1.5 下游

02 wrap。

### 1.6 关键不变量（钉死）

1. len(widths)==len(runs)。
2. 空列表 → 空 widths。

### 1.7 依赖与实现状态（必写，着重）

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------|------|------|------|
| RunMetrics | wr-03 | measure_run | **pass**（本 function 用闭包注入） | — | 不改 font crate |

## 2. 核心代码与流程

Java：`koffice/writer/core/cn/wps/moffice/writer/layout/base/line/LineLayouter.java`
Rust：`libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs`

```mermaid
flowchart LR
  A[runs] --> B[widths[]]
```

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|------|------|------|------|------|
| len | — | 0/1/N | 等长 | — |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|------|------|------|------|------|
| RW-0 | 空 | 边界 | 是 | RW1 |
| RW-N | N | 主路径 | 是 | RW2 |

### 3.3 可测试性

假度量闭包。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|------|------|------|------|------|------|
| RW1 | RW-0 | — | [] | [] | 非空 |
| RW2 | RW-N | 假度量 10 | 2 runs | [10,10] | 长度错 |

### 3.5 测试代码位置

crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_run_width

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_run_width \
  --src src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --report libs/word-core/layout/features/wr-04-line-break/functions/01_function_run_width/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_run_width --json`)
date: 2026-08-22 10:12

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs` | 6/6 | 100.0% | 8/8 | 100.0% |
| **function total** | **6/6** | **100.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs
```

2026-08-22：**PASS**（Rust 胶水；勿与 `LineLayouter.java` 1+1 配对）。
