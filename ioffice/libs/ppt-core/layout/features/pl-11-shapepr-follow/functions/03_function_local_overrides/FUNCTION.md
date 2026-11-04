# 03_function_local_overrides — 本地写了不 follow

> feature: pl-11-shapepr-follow ｜ 状态：**spec** ｜ Java 管线：S6 ShapePr.follow

## 0. 管线能力解锁

Java map.has(key) 则不走 parent。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | slide 已 writeFill / writeXfrm |
| **输出** | 读本地；改 parent 不影响覆盖槽 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| writeFill/writeXfrm | ShapePr | 本地 bin | 有 API | — | 禁止 unFollow 误清 |

## 2. 核心代码与函数改写列表

**本 function：** 覆盖槽隔离；未覆盖槽仍 live。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java hasFill/hasXfrm：map.has 或 dirty 或 parent.has*。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| OV-F | writeFill 后改 parent | 两边都有 fill | 仍本地色 | 公开入口 |
| OV-X | writeXfrm | 本地框 | 仍本地框 | 对照 |
| OV-MIX | 有 fill 无 xfrm | parent 有 xfrm | fill 本地 + xfrm parent | 状态 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 本地 fill | writeFill 后改 parent | 公开入口 | 是 | OV-F |
| 本地 xfrm | writeXfrm | 对照 | 是 | OV-X |
| 只覆盖 fill | 有 fill 无 xfrm | 状态 | 是 | OV-MIX |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| OV-F | 本地 fill | — | 两边都有 fill | 仍本地色 | 被 parent 覆盖 |
| OV-X | 本地 xfrm | — | 本地框 | 仍本地框 | follow 覆盖 |
| OV-MIX | 只覆盖 fill | — | parent 有 xfrm | fill 本地 + xfrm parent | 全拷或全跟 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/drawing/data/shape/ShapePr.rs
test:  tests/cn/wps/drawing/data/shape/03_function_local_overrides.rs
[[test]] name = "03_function_local_overrides"
```

`#[ignore = "spec: 03_function_local_overrides"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_local_overrides -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_local_overrides \
  --src src/cn/wps/drawing/data/shape/ShapePr.rs \
  --report layout/features/pl-11-shapepr-follow/functions/03_function_local_overrides/FUNCTION.md
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
  --report layout/features/pl-11-shapepr-follow/functions/03_function_local_overrides/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | OV-F, OV-X, OV-MIX | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
