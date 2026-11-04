# 03_function_textrect_inset — bodyPr inset 后文本原点

> feature: pr-10-text-span-emit ｜ 状态：**spec** ｜ Java 管线：S9 TextRender

## 0. 管线能力解锁

Java covertTextBodyProperties:227 默认 inset 91440/45720 EMU。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | bodyPr lIns/tIns |
| **输出** | 文本 x/y ≠ 形状外框；默认 inset Java 91440/45720 EMU |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| covertTextBodyProperties | DataTransform:220 | inset | 需接线 | autofit | 禁止改 Java 默认 EMU |

## 2. 核心代码与函数改写列表

**本 function：** DrawText 原点用 getTextRect/insets，不是 shape AABB。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java DataTransform:227-232 defaultInset；emit_text 用 getShapeRect。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| IN-D | 无 lIns | 有字 sp | text.x > shape.left | 公开入口 |
| IN-C | lIns 更大 | 显式 inset | x 更大 | 关键分支 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 默认 inset | 无 lIns | 公开入口 | 是 | IN-D |
| 自定义 lIns | lIns 更大 | 关键分支 | 是 | IN-C |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| IN-D | 默认 inset | — | 有字 sp | text.x > shape.left | 贴外框 |
| IN-C | 自定义 lIns | — | 显式 inset | x 更大 | 忽略 bodyPr |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/mvp_pipeline.rs, src/cn/wps/show/layout/dataaction/DataTransform.rs
test:  tests/cn/wps/show/render/03_function_textrect_inset.rs
[[test]] name = "03_function_textrect_inset"
```

`#[ignore = "spec: 03_function_textrect_inset"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_textrect_inset -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_textrect_inset \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --src src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --report render/features/pr-10-text-span-emit/functions/03_function_textrect_inset/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

Loop L5b 必须 PASS。不删 `// java`；列表外方法 `todo!("DEFERRED_NON_FUNCTION")` + 普通 `/* */`。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/DataTransform.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/DataTransform.java --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --report render/features/pr-10-text-span-emit/functions/03_function_textrect_inset/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | IN-D, IN-C | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
