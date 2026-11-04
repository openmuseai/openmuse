# 01_function_line_drawtext — 一行一字 → 1 条 DrawText

> feature: pr-10-text-span-emit ｜ 状态：**spec** ｜ Java 管线：S9 TextRender

## 0. 管线能力解锁

`emit_text:142` 用首 run 样式打整行；原点常为形状外框而非 textRect。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | GreedyWrap 一行 + 单 run |
| **输出** | 1 条 DrawText，原点=文本区+行偏移 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| GreedyWrap 行 | pl-05 | ShowTypoLine | pass | WpsLayout | 本波允许行宽差 |
| emit_text | mvp_pipeline:142 | DrawText | 首 run 整行 | — | 禁止只测 eaVert |

## 2. 核心代码与函数改写列表

**本 function：** 按 typo 行 emit 一条 DrawText；xy= textRect + line offset。不要求与 Java 断行位置像素一致。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 emit_text:142-163 取 paras.first().runs.first()；eaVert 骨架在同函数。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| TD-1 | 一行一字 | Hello | DrawText 数==1 且 xy 在 textRect | 公开入口 |
| TD-V | eaVert=4 | pr-07:02 | 仍竖排 | 回归 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 单行 | 一行一字 | 公开入口 | 是 | TD-1 |
| 回归 vert | eaVert=4 | 回归 | 是 | TD-V |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| TD-1 | 单行 | — | Hello | DrawText 数==1 且 xy 在 textRect | 0 条或用外框原点 |
| TD-V | 回归 vert | — | pr-07:02 | 仍竖排 | 横排回归红 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/mvp_pipeline.rs
test:  tests/cn/wps/show/render/01_function_line_drawtext.rs
[[test]] name = "01_function_line_drawtext"
```

`#[ignore = "spec: 01_function_line_drawtext"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_line_drawtext -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_line_drawtext \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --report render/features/pr-10-text-span-emit/functions/01_function_line_drawtext/FUNCTION.md
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
  --report render/features/pr-10-text-span-emit/functions/01_function_line_drawtext/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | TD-1, TD-V | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
