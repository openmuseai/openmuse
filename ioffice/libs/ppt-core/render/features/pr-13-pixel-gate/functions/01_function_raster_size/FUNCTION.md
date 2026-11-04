# 01_function_raster_size — 栅格尺寸对齐 Java 单位

> feature: pr-13-pixel-gate ｜ 状态：**spec** ｜ Java 管线：S10 验收 · 不是新渲染器

## 0. 管线能力解锁

无固定视口则 RMSE 无意义。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | slideSize EMU + 视口 |
| **输出** | 页 w×h 与 pl-01 / Java render 单位一致 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| slide_viewport | pl-01 | px 尺寸 | pass | density | 禁止另开一套 dpi |

## 2. 核心代码与函数改写列表

**本 function：** 锁定栅格宽高常量（与 Java 参考图同）。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 pl-01 视口函数；S10 文档 width≠IDevice.w 风险。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| RS-WH | 9144000×6858000 EMU | 标准 10×7.5in | 栅格 w/h 与 Java fixture 相同 | 公开入口 |
| RS-0 | 0 EMU | 空 sldSz | 错误不 panic 成 Inf RMSE | 失败 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 默认页 | 9144000×6858000 EMU | 公开入口 | 是 | RS-WH |
| 坏尺寸 | 0 EMU | 失败 | 是 | RS-0 |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| RS-WH | 默认页 | — | 标准 10×7.5in | 栅格 w/h 与 Java fixture 相同 | 错单位 |
| RS-0 | 坏尺寸 | — | 空 sldSz | 错误不 panic 成 Inf RMSE | 除零 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/layout/slide_viewport.rs
test:  tests/cn/wps/show/layout/01_function_raster_size.rs
[[test]] name = "01_function_raster_size"
```

`#[ignore = "spec: 01_function_raster_size"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_raster_size -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_raster_size \
  --src src/cn/wps/show/layout/slide_viewport.rs \
  --report render/features/pr-13-pixel-gate/functions/01_function_raster_size/FUNCTION.md
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
  --report render/features/pr-13-pixel-gate/functions/01_function_raster_size/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | RS-WH, RS-0 | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
