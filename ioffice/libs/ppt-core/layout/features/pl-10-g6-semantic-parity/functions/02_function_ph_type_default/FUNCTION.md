# 02_function_ph_type_default — ph type 缺省 8

> feature: pl-10-g6-semantic-parity ｜ 状态：**spec** ｜ Java 管线：S6/S7 语义注记（G6）

## 0. 管线能力解锁

`layout_shape::Placeholder` Default `type_=0`。Java 无 map key 且 parent==null 返回 **8**。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | Placeholder 无 type 属性、无 parent |
| **输出** | type()==8（ShowShape.Placeholder.java:865-867）不是 0 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| Placeholder.type | ShowShape / layout_shape | 缺省 8 | Default 0 | follow parent type | 禁止改 hasType 语义 |
| PlaceHolderMerger.phType | PlaceHolderMerger.rs:54 | 查找链 | 逻辑在 | — | 禁止改 findLayout 公式 |

## 2. 核心代码与函数改写列表

**本 function：** 缺省 type=8；`PlaceHolderMerger.phType` 末路 `ph.type()` 得 8。优先修 **写入端/app Placeholder**，不要平行字段。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java Placeholder.type:865 `parent==null ? 8`；Rust layout_shape.rs:235 Default type_=0；PlaceHolderMerger.rs:54 委托 ph.type()。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| PH8-DEF | Placeholder.type() | hasType=false parent=null | 8 | 公开入口 |
| PH8-HAS | putType(1) | body=1 | 1 | 对照 |
| PH8-MRG | PlaceHolderMerger.phType | 无 layout/master | 8 | 关键分支 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 无 type | Placeholder.type() | 公开入口 | 是 | PH8-DEF |
| 有 type | putType(1) | 对照 | 是 | PH8-HAS |
| phType 末路 | PlaceHolderMerger.phType | 关键分支 | 是 | PH8-MRG |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| PH8-DEF | 无 type | — | hasType=false parent=null | 8 | 返回 0 |
| PH8-HAS | 有 type | — | body=1 | 1 | 仍 8 |
| PH8-MRG | phType 末路 | — | 无 layout/master | 8 | merger 返回 0 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/layout/PlaceHolderMerger.rs
test:  tests/cn/wps/show/layout/02_function_ph_type_default.rs
[[test]] name = "02_function_ph_type_default"
```

`#[ignore = "spec: 02_function_ph_type_default"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_ph_type_default -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_ph_type_default \
  --src src/cn/wps/show/layout/PlaceHolderMerger.rs \
  --report layout/features/pl-10-g6-semantic-parity/functions/02_function_ph_type_default/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

Loop L5b 必须 PASS。不删 `// java`；列表外方法 `todo!("DEFERRED_NON_FUNCTION")` + 普通 `/* */`。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/util/layout_shape.rs \
  --java koffice/presentation/core/cn/wps/show/data/shape/ShowShape.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/data/shape/ShowShape.java --rs libs/ppt-core/src/cn/wps/show/util/layout_shape.rs \
  --report layout/features/pl-10-g6-semantic-parity/functions/02_function_ph_type_default/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | PH8-DEF, PH8-HAS, PH8-MRG | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
