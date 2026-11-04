# 01_function_follow_fill — follow fill 惰性

> feature: pl-11-shapepr-follow ｜ 状态：**spec** ｜ Java 管线：S6 ShapePr.follow

## 0. 管线能力解锁

`ShapePr.follow:655` 已挂 parent 指针；`shapepr_hosts.rs` Fill/Outline `follow` **空实现**。Java fill() 在 parent 且本地无 fill 时 follow parent.fill()。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | slide ShapePr 无 fill + parent 有 solid |
| **输出** | fill() 色 = parent；改 parent 后未覆盖槽可见 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| ShapePr.follow | ShapePr.rs:655 | 挂 parent+discard | 指针在 | effect/3D | 禁止深拷贝冒充 |
| Fill.follow | shapepr_hosts.rs:222 | 惰性槽 | 空 fn | — | 禁止只测 mergeShapePr |

## 2. 核心代码与函数改写列表

**本 function：** Fill.follow + ShapePr.fill()/hasFill 走 parent。断言 **改 parent 后未覆盖槽变化可见**（禁止 merge 深拷贝冒充）。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java ShapePr.follow:360、fill 段与 xfrm:415-425 同类；Rust follow:655 有指针；hosts follow 空；xfrm() 内 `todo!("xfrm follow parent")`。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| FL-F | fill()+parent solid | slide 空 fill | 色=parent | 公开入口 |
| FL-LIVE | parent 换色 | follow 后改 parent | 再读 fill 新色 | 状态转换 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 无本地 fill | fill()+parent solid | 公开入口 | 是 | FL-F |
| 改 parent | parent 换色 | 状态转换 | 是 | FL-LIVE |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| FL-F | 无本地 fill | — | slide 空 fill | 色=parent | None |
| FL-LIVE | 改 parent | — | follow 后改 parent | 再读 fill 新色 | 拷贝冻结 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/drawing/data/shape/ShapePr.rs, src/cn/wps/drawing/data/shape/shapepr_hosts.rs
test:  tests/cn/wps/drawing/data/shape/01_function_follow_fill.rs
[[test]] name = "01_function_follow_fill"
```

`#[ignore = "spec: 01_function_follow_fill"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_follow_fill -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_follow_fill \
  --src src/cn/wps/drawing/data/shape/ShapePr.rs \
  --src src/cn/wps/drawing/data/shape/shapepr_hosts.rs \
  --report layout/features/pl-11-shapepr-follow/functions/01_function_follow_fill/FUNCTION.md
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
  --report layout/features/pl-11-shapepr-follow/functions/01_function_follow_fill/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | FL-F, FL-LIVE | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
