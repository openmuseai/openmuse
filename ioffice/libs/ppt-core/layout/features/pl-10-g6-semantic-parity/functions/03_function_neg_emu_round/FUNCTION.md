# 03_function_neg_emu_round — 负 EMU 舍入同 Java

> feature: pl-10-g6-semantic-parity ｜ 状态：**spec** ｜ Java 管线：S6/S7 语义注记（G6）

## 0. 管线能力解锁

Rust `UnitsConverter.rs:13` `v.round() as i32` = half away from zero；Java `Math.round` = floor(x+0.5)。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | 负 f32 emu→px |
| **输出** | Math.round 语义 floor(x+0.5) 对负值 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| UnitsConverter.round | base/converter | emu/px | 正值可用 | 全单位表 | 禁止改 UNIT_* 常量 |

## 2. 核心代码与函数改写列表

**本 function：** 对齐 Java `Math.round`（可用 `(v + 0.5).floor()` 对 f32）。回归 pl-01 正值。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 UnitsConverter.rs Math::round；Java UnitsConverter.java:131 Math.round(fromDip)。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| NR-POS | 1.5f | 1.5 | 2（两边同） | 对照 |
| NR-NEG | -1.5f | -1.5 | Java Math.round → -1 | 公开入口 |
| NR-N2 | -2.5f | -2.5 | Java -2 | 关键分支 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 正半数 | 1.5f | 对照 | 是 | NR-POS |
| 负半数 | -1.5f | 公开入口 | 是 | NR-NEG |
| 负过半 | -2.5f | 关键分支 | 是 | NR-N2 |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| NR-POS | 正半数 | — | 1.5 | 2（两边同） | 回归红 |
| NR-NEG | 负半数 | — | -1.5 | Java Math.round → -1 | Rust round → -2 |
| NR-N2 | 负过半 | — | -2.5 | Java -2 | 不一致 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/base/converter/UnitsConverter.rs
test:  tests/cn/wps/base/converter/03_function_neg_emu_round.rs
[[test]] name = "03_function_neg_emu_round"
```

`#[ignore = "spec: 03_function_neg_emu_round"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_neg_emu_round -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_neg_emu_round \
  --src src/cn/wps/base/converter/UnitsConverter.rs \
  --report layout/features/pl-10-g6-semantic-parity/functions/03_function_neg_emu_round/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

Loop L5b 必须 PASS。不删 `// java`；列表外方法 `todo!("DEFERRED_NON_FUNCTION")` + 普通 `/* */`。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/base/converter/UnitsConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/UnitsConverter.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/base/converter/UnitsConverter.java --rs libs/ppt-core/src/cn/wps/base/converter/UnitsConverter.rs \
  --report layout/features/pl-10-g6-semantic-parity/functions/03_function_neg_emu_round/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | NR-POS, NR-NEG, NR-N2 | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
