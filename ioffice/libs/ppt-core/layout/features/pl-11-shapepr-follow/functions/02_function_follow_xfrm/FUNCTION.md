# 02_function_follow_xfrm — follow xfrm

> feature: pl-11-shapepr-follow ｜ 状态：**spec** ｜ Java 管线：S6 ShapePr.follow

## 0. 管线能力解锁

`ShapePr.xfrm:758` `todo!("xfrm follow parent")`。Java:421-423 `xfrm.follow(parent.xfrm())`。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | slide 无 xfrm + parent 有框 |
| **输出** | xfrm() 用 parent 框；hosts Transform2D.follow 真实现 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| Transform2D.follow | shapepr_hosts.rs:70 | 挂 parent | 空 | — | 禁止发明第二套 xfrm |
| ShapePr.xfrm | ShapePr.rs:746 | 解码+follow | todo | — | — |

## 2. 核心代码与函数改写列表

**本 function：** 解开 xfrm follow；Transform2D.follow 非空。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java xfrm():415-425；Rust :758 todo。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| FL-X | xfrm()+parent | parent off/ext | off/ext 相同 | 公开入口 |
| FL-XH | hasXfrm 经 parent | 仅 parent 有 key2 | true | 关键分支 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 无本地 xfrm | xfrm()+parent | 公开入口 | 是 | FL-X |
| hasXfrm | hasXfrm 经 parent | 关键分支 | 是 | FL-XH |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| FL-X | 无本地 xfrm | — | parent off/ext | off/ext 相同 | todo panic |
| FL-XH | hasXfrm | — | 仅 parent 有 key2 | true | false |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/drawing/data/shape/ShapePr.rs, src/cn/wps/drawing/data/shape/shapepr_hosts.rs
test:  tests/cn/wps/drawing/data/shape/02_function_follow_xfrm.rs
[[test]] name = "02_function_follow_xfrm"
```

`#[ignore = "spec: 02_function_follow_xfrm"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_follow_xfrm -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_follow_xfrm \
  --src src/cn/wps/drawing/data/shape/ShapePr.rs \
  --src src/cn/wps/drawing/data/shape/shapepr_hosts.rs \
  --report layout/features/pl-11-shapepr-follow/functions/02_function_follow_xfrm/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

Loop L5b 必须 PASS。不删 `// java`；列表外方法 `todo!("DEFERRED_NON_FUNCTION")` + 普通 `/* */`。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/data/shape/ShapePr.rs \
  --java koffice/presentation/core/cn/wps/drawing/data/shape/ShapePr.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/data/shape/ShapePr.java --rs libs/ppt-core/src/cn/wps/drawing/data/shape/ShapePr.rs \
  --report layout/features/pl-11-shapepr-follow/functions/02_function_follow_xfrm/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | FL-X, FL-XH | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
