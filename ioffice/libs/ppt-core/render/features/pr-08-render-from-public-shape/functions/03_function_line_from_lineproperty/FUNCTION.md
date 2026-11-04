# 03_function_line_from_lineproperty — stroke 来自 LineProperty

> feature: pr-08-render-from-public-shape ｜ 状态：**spec** ｜ Java 管线：S9 几何/填线（输入必须是 S5 公开 Shape）

## 0. 管线能力解锁

`render_slide:341-348` 只取 LineProperty 色；宽尚未进指令。本 function 钉 **有/无 + 色**。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | ln w EMU + getfLine |
| **输出** | 有线：色来自 LineProperty；`getfLine()==false` 无 stroke；宽 min 12700 留给 pr-11 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| LineStyleConvert | pl-09:06 | LineProperty | pass | dash/compound | 宽度交给 pr-11 |
| apply_fill_line | mvp_pipeline | 写入指令 | 色 or 旧值 | — | 禁止 layout ln 回退 |

## 2. 核心代码与函数改写列表

**本 function：** stroke 有无与色只来自 LineProperty；`fill.type==5`/noFill 线 → 无 stroke。宽度 px 是 pr-11。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 stroke 分支只 map `lp.getColor()`；`apply_fill_line` 把 fill/stroke 填进 DrawRect/Path。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| LP-COL | LineProperty.getColor | srgb 线 | 指令 stroke==色 | 公开入口 |
| LP-OFF | noFill 线 | noline | stroke=None | 关键分支 |
| LP-REG | 空样式 | 无 fill 无 ln | 无默认 stroke | 回归 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 线色 | LineProperty.getColor | 公开入口 | 是 | LP-COL |
| getfLine false | noFill 线 | 关键分支 | 是 | LP-OFF |
| 回归 pr-07:03 | 空样式 | 回归 | 是 | LP-REG |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| LP-COL | 线色 | — | srgb 线 | 指令 stroke==色 | stroke None |
| LP-OFF | getfLine false | — | noline | stroke=None | 默认黑描边 |
| LP-REG | 回归 pr-07:03 | — | 无 fill 无 ln | 无默认 stroke | Flutter 又描黑 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/mvp_pipeline.rs
test:  tests/cn/wps/show/render/03_function_line_from_lineproperty.rs
[[test]] name = "03_function_line_from_lineproperty"
```

`#[ignore = "spec: 03_function_line_from_lineproperty"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_line_from_lineproperty -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_line_from_lineproperty \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --report render/features/pr-08-render-from-public-shape/functions/03_function_line_from_lineproperty/FUNCTION.md
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
  --report render/features/pr-08-render-from-public-shape/functions/03_function_line_from_lineproperty/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | LP-COL, LP-OFF, LP-REG | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
