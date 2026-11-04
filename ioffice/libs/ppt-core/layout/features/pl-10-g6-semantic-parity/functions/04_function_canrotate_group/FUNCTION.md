# 04_function_canrotate_group — 组 canRotate 递归

> feature: pl-10-g6-semantic-parity ｜ 状态：**spec** ｜ Java 管线：S6/S7 语义注记（G6）

## 0. 管线能力解锁

Java `KmoShowShape.canRotate:2261` case 0 递归。Rust `layout_shape` 是 bool 字段默认 true。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | type0 组 + 子 canRotate |
| **输出** | 任一子 false → 组 false；type4 永远 false；其它 true |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| KmoShowShape.canRotate | app/shape | 递归 | layout 是 bool | graphicFrame 内容 | 禁止只改 fixture 字段 |
| childAt/childCount | pp-19:02 | 遍历 | pass | — | — |

## 2. 核心代码与函数改写列表

**本 function：** 在 **app::KmoShowShape** 实现 Java switch；layout_shape 夹具可留但不作为 pass。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java canRotate:2261-2277；layout_shape.rs:103 直读 `can_rotate`。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| CR-SP | type=1 | 单 sp | true | 对照 |
| CR-GF | type=4 | 空 gf | false | 关键分支 |
| CR-GRP-OK | type0 + 两 sp | 子皆 true | true | 公开入口 |
| CR-GRP-NO | type0 + type4 子 | 一子 false | 组 false | 关键分支 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 普通 sp | type=1 | 对照 | 是 | CR-SP |
| graphicFrame | type=4 | 关键分支 | 是 | CR-GF |
| 组全可转 | type0 + 两 sp | 公开入口 | 是 | CR-GRP-OK |
| 组含 gf | type0 + type4 子 | 关键分支 | 是 | CR-GRP-NO |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| CR-SP | 普通 sp | — | 单 sp | true | false |
| CR-GF | graphicFrame | — | 空 gf | false | true |
| CR-GRP-OK | 组全可转 | — | 子皆 true | true | 快照 bool |
| CR-GRP-NO | 组含 gf | — | 一子 false | 组 false | 不递归 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/app/shape/KmoShowShape.rs
test:  tests/cn/wps/show/app/shape/04_function_canrotate_group.rs
[[test]] name = "04_function_canrotate_group"
```

`#[ignore = "spec: 04_function_canrotate_group"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_canrotate_group -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_canrotate_group \
  --src src/cn/wps/show/app/shape/KmoShowShape.rs \
  --report layout/features/pl-10-g6-semantic-parity/functions/04_function_canrotate_group/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

Loop L5b 必须 PASS。不删 `// java`；列表外方法 `todo!("DEFERRED_NON_FUNCTION")` + 普通 `/* */`。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/app/shape/KmoShowShape.rs \
  --java koffice/presentation/core/cn/wps/show/app/shape/KmoShowShape.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/app/shape/KmoShowShape.java --rs libs/ppt-core/src/cn/wps/show/app/shape/KmoShowShape.rs \
  --report layout/features/pl-10-g6-semantic-parity/functions/04_function_canrotate_group/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | CR-SP, CR-GF, CR-GRP-OK, CR-GRP-NO | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
