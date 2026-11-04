# 01_function_para_direction — getParaDirection

> feature: pl-12-textbox-live-paths ｜ 状态：**spec** ｜ Java 管线：S7 DataTransform

## 0. 管线能力解锁

`getParaDirection:410` `todo!("DEFERRED_NON_FUNCTION")`。`convertBullet` 会调用它。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | paraProp rtl 有/无 + parent 链 |
| **输出** | bool RTL；无属性沿 parent 直到 hasRightToLeft |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| TextParagraphProperties.options | drawing/data/text | hasRightToLeft/isRightToLeft | 需可用 | — | 禁止改 options bit |
| getParaDirection | DataTransform.rs:410 | bool | todo | bidi 整页 | 禁止 dummy true |

## 2. 核心代码与函数改写列表

**本 function：** 解开 Java `:200-217` 循环。function-java2rust，保留 // java。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java getParaDirection:200：先 options.isRightToLeft，若 !hasRightToLeft 则 while parent。Rust :410 parked。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| PD-L | hasRtl=true value=false | 显式 ltr | false | 对照 |
| PD-R | isRightToLeft=true | 显式 rtl | true | 公开入口 |
| PD-PAR | child 无 has，parent rtl | follow 链 | true | 关键分支 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| ltr | hasRtl=true value=false | 对照 | 是 | PD-L |
| rtl | isRightToLeft=true | 公开入口 | 是 | PD-R |
| 无本地沿 parent | child 无 has，parent rtl | 关键分支 | 是 | PD-PAR |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| PD-L | ltr | — | 显式 ltr | false | panic |
| PD-R | rtl | — | 显式 rtl | true | false |
| PD-PAR | 无本地沿 parent | — | follow 链 | true | 只看本地默认 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/layout/dataaction/DataTransform.rs
test:  tests/cn/wps/show/layout/dataaction/01_function_para_direction.rs
[[test]] name = "01_function_para_direction"
```

`#[ignore = "spec: 01_function_para_direction"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_para_direction -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_para_direction \
  --src src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --report layout/features/pl-12-textbox-live-paths/functions/01_function_para_direction/FUNCTION.md
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
  --report layout/features/pl-12-textbox-live-paths/functions/01_function_para_direction/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | PD-L, PD-R, PD-PAR | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
