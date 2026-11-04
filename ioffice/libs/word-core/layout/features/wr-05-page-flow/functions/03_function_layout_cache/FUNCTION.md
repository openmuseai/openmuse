# 03_function_layout_cache — 全量重排占位

| 项 | 值 |
|----|----|
| feature | wr-05-page-flow |
| order | 3 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | invalidate() / layout() 全量 |
| 独立性 | 增量算法 defer |
| 测试状态 | pass |

## 1. 功能说明

### 1.1 一句话

任何文档变更 → 丢弃 snapshot 全量 layout。保留 Java 增量 API 签名为 DEFERRED。

### 1.2 输入

| 入口 | 输入 | Java 路径 |
|------|------|------|
| invalidate | 脏标记 | FlowLayout / TypoDocument |

### 1.3 输出

| 字段/副作用 | 含义 |
|------|------|
| next layout 全量 | version++ |

### 1.4 上游

01。

### 1.5 下游

wr-06。

### 1.6 关键不变量（钉死）

1. invalidate 后 current snapshot 不可用于绘制（None 或 version 旧）。
2. 不实现局部行 dirty。

### 1.7 依赖与实现状态（必写，着重）

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------|------|------|------|
| FlowLayout 增量 | layout | — | 暂缓 | 局部重排 | 禁止假增量绿 |

## 2. 核心代码与流程

Java：`koffice/writer/core/cn/wps/moffice/writer/layout/FlowLayout.java`
Rust：`libs/word-core/src/cn/wps/moffice/writer/layout/FlowLayout.rs`

```mermaid
flowchart LR
  A[dirty] --> B[drop snapshot]
  B --> C[full layout]
```

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|------|------|------|------|------|
| invalidate | — | 有 snapshot | None | 仍持有 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|------|------|------|------|------|
| LC-INV | invalidate | 状态 | 是 | LC1 |

### 3.3 可测试性

内存标志。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|------|------|------|------|------|------|
| LC1 | LC-INV | 已 layout | invalidate | 不可绘制 | 仍可绘 |

### 3.5 测试代码位置

crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/FlowLayout.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/FlowLayout.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_layout_cache

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_layout_cache \
  --src src/cn/wps/moffice/writer/layout/FlowLayout.rs \
  --report libs/word-core/layout/features/wr-05-page-flow/functions/03_function_layout_cache/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_layout_cache --json`)
date: 2026-08-22 10:12

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/FlowLayout.rs` | 22/22 | 100.0% | 19/19 | 100.0% |
| **function total** | **22/22** | **100.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/FlowLayout.rs
```

2026-08-22：**PASS**（Rust 胶水；勿与 `FlowLayout.java` 1+1 配对）。
