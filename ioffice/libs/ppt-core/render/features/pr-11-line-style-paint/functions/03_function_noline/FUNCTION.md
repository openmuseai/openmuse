# 03_function_noline — noFill 线无 stroke

> feature: pr-11-line-style-paint ｜ 状态：**spec** ｜ Java 管线：S9 drawStroke

## 0. 管线能力解锁

加宽度后不得把 noline 画成 hairline。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | a:ln/a:noFill |
| **输出** | 无 stroke；回归 pr-07 空样式 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| pr-07:03 | Flutter | 空样式 | pass | — | 禁止默认黑线 |

## 2. 核心代码与函数改写列表

回归 pr-07:03；Flutter 可 `flutter test`。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

pr-07 Flutter 无默认描边已 pass。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| NL-NO | noFill ln | 有几何无 ln 色 | stroke None 且无宽 | 公开入口 |
| NL-EMPTY | 无 fill 无 ln | pr-07 fixture | 无描边 | 回归 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| noline | noFill ln | 公开入口 | 是 | NL-NO |
| 空样式 | 无 fill 无 ln | 回归 | 是 | NL-EMPTY |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| NL-NO | noline | — | 有几何无 ln 色 | stroke None 且无宽 | hairline |
| NL-EMPTY | 空样式 | — | pr-07 fixture | 无描边 | 回归红 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/mvp_pipeline.rs
test:  tests/cn/wps/show/render/03_function_noline.rs
[[test]] name = "03_function_noline"
```

`#[ignore = "spec: 03_function_noline"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_noline -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_noline \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --report render/features/pr-11-line-style-paint/functions/03_function_noline/FUNCTION.md
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
  --report render/features/pr-11-line-style-paint/functions/03_function_noline/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | NL-NO, NL-EMPTY | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
