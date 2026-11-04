# 02_function_stroke_color — 线色 scheme/srgb

> feature: pr-11-line-style-paint ｜ 状态：**spec** ｜ Java 管线：S9 drawStroke

## 0. 管线能力解锁

pr-08:03 钉有无；本 function 钉解析色。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | schemeClr / srgbClr ln |
| **输出** | painter/指令色解析后 ARGB |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| pp-16 scheme | parser | ARGB | pass | — | 禁止第二套主题表 |

## 2. 核心代码与函数改写列表

走已有 scheme 解析，不要平行调色盘。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 apply_fill_line 写 stroke 色。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| SC-SRGB | ln 色 | FF0000 | stroke==红 | 公开入口 |
| SC-SCH | accent1 | theme | 解析后非 0 | 关键分支 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| srgb | ln 色 | 公开入口 | 是 | SC-SRGB |
| scheme | accent1 | 关键分支 | 是 | SC-SCH |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| SC-SRGB | srgb | — | FF0000 | stroke==红 | None |
| SC-SCH | scheme | — | theme | 解析后非 0 | 未解析 scheme 名 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/mvp_pipeline.rs
test:  tests/cn/wps/show/render/02_function_stroke_color.rs
[[test]] name = "02_function_stroke_color"
```

`#[ignore = "spec: 02_function_stroke_color"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_stroke_color -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_stroke_color \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --report render/features/pr-11-line-style-paint/functions/02_function_stroke_color/FUNCTION.md
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
  --report render/features/pr-11-line-style-paint/functions/02_function_stroke_color/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | SC-SRGB, SC-SCH | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
