# 03_function_sample_slide_n — 选定 Sample 页门闩

> feature: pr-13-pixel-gate ｜ 状态：**spec** ｜ Java 管线：S10 验收 · 不是新渲染器

## 0. 管线能力解锁

总闸第 5 条。未完成结构对齐时禁止把截图差当调 painter。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | 选定页（建议 slide2 组色块或自制金页） |
| **输出** | RMSE/边缘 IoU；失败打印 diff；富文本/图表页 ignore 到后波 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 02 金页工具 | 本 feature | RMSE | spec | wpslayout 长文 | 禁止全 23 页唯一用例 |

## 2. 核心代码与函数改写列表

一页几何向 Sample。Chart/Table ignore。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

e2e 只断言指令形态。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| SN-GEO | 选定 slide | 组色块页 | RMSE/IoU 过阈值 | 公开入口 |
| SN-IGN | 后波 | 长文 Sample | #[ignore spec: wpslayout] | 边界 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 几何页 | 选定 slide | 公开入口 | 是 | SN-GEO |
| 富文本页 | 后波 | 边界 | 是 | SN-IGN |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| SN-GEO | 几何页 | — | 组色块页 | RMSE/IoU 过阈值 | 用长文页当唯一 |
| SN-IGN | 富文本页 | — | 长文 Sample | #[ignore spec: wpslayout] | 强行标 pass |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/mvp_pipeline.rs
test:  tests/cn/wps/show/render/03_function_sample_slide_n.rs
[[test]] name = "03_function_sample_slide_n"
```

`#[ignore = "spec: 03_function_sample_slide_n"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_sample_slide_n -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_sample_slide_n \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --report render/features/pr-13-pixel-gate/functions/03_function_sample_slide_n/FUNCTION.md
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
  --report render/features/pr-13-pixel-gate/functions/03_function_sample_slide_n/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | SN-GEO, SN-IGN | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
