# 01_function_anchor_1_2 — bodyPr anchor 1/2 对齐 Java

> feature: pl-10-g6-semantic-parity ｜ 状态：**spec** ｜ Java 管线：S6/S7 语义注记（G6）

## 0. 管线能力解锁

Rust `GreedyWrap.apply_anchor:148`：`1 => 底`，`2 => 居中`，与 Java **对调**。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | TextBoxData.anchor 0/1/2 + 行高 < 框高 |
| **输出** | 1=垂直居中，2=底（Java RichTextLayout.modifyTextBoxAlign:1050） |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| GreedyWrap.apply_anchor | pl-05 | 行 y 偏移 | 实现反 | WpsLayout | 禁止整页 wpslayout |
| TextBoxData.anchor | dataaction | 0/1/2 | 能读 | — | — |

## 2. 核心代码与函数改写列表

**本 function：** 把 match 改成 Java case 1 居中、case 2/3/4 底。不改 Java。getShapeRect 冻结政策只写口径。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java `RichTextLayout.modifyTextBoxAlign:1050` case 1 居中、case 2 底；Rust `GreedyWrap.rs:148-151` 对调。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| AN-0 | anchor=0 | 短文+高框 | 首行 y≈textRect.top | 对照 |
| AN-1 | anchor=1 | 短文+高框 | extra≈(avail-h)/2 | 公开入口 |
| AN-2 | anchor=2 | 短文+高框 | extra≈avail-h | 公开入口 |
| AN-EQ | 行高==框 | 填满 | extra≈0 | 边界 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 顶 | anchor=0 | 对照 | 是 | AN-0 |
| 居中 | anchor=1 | 公开入口 | 是 | AN-1 |
| 底 | anchor=2 | 公开入口 | 是 | AN-2 |
| 等高 | 行高==框 | 边界 | 是 | AN-EQ |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| AN-0 | 顶 | — | 短文+高框 | 首行 y≈textRect.top | 误加 extra |
| AN-1 | 居中 | — | 短文+高框 | extra≈(avail-h)/2 | 当成底 |
| AN-2 | 底 | — | 短文+高框 | extra≈avail-h | 当成居中 |
| AN-EQ | 等高 | — | 填满 | extra≈0 | NaN |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/layout/typo/GreedyWrap.rs
test:  tests/cn/wps/show/layout/typo/01_function_anchor_1_2.rs
[[test]] name = "01_function_anchor_1_2"
```

`#[ignore = "spec: 01_function_anchor_1_2"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_anchor_1_2 -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_anchor_1_2 \
  --src src/cn/wps/show/layout/typo/GreedyWrap.rs \
  --report layout/features/pl-10-g6-semantic-parity/functions/01_function_anchor_1_2/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

Loop L5b 必须 PASS。不删 `// java`；列表外方法 `todo!("DEFERRED_NON_FUNCTION")` + 普通 `/* */`。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/layout/typo/GreedyWrap.rs \
  --java koffice/core-libs/wpslayout/src/cn/wps/text/layout/typo/RichTextLayout.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/core-libs/wpslayout/src/cn/wps/text/layout/typo/RichTextLayout.java --rs libs/ppt-core/src/cn/wps/show/layout/typo/GreedyWrap.rs \
  --report layout/features/pl-10-g6-semantic-parity/functions/01_function_anchor_1_2/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | AN-0, AN-1, AN-2, AN-EQ | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
