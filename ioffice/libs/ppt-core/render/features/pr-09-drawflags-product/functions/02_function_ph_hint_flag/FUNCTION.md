# 02_function_ph_hint_flag — placeholder 提示字受 flags

> feature: pr-09-drawflags-product ｜ 状态：**spec** ｜ Java 管线：S9 ShapeRender._begin

## 0. 管线能力解锁

Java `DrawFlags.drawPHolderText` = `(flags & 16)==0`。`READ_NORMAL_SLIDE=21505154` 含 bit 16。产品 flags=0 会画提示字。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | ph 形状 + NOT_DRAW_PLACEHOLDER_TEXT(16) |
| **输出** | 未置 drawPHolderText 时不发射提示字 DrawText |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| DrawFlags | show/util | bit 16 / READ_NORMAL_SLIDE | 常量在 | 全套 helper | 禁止改字面量 |
| emit_text | mvp_pipeline:142 | DrawText | 不看 ph flags | 多 run 是 pr-10 | — |

## 2. 核心代码与函数改写列表

**本 function：** `emit_text` 按 `DrawFlags.drawPHolderText`；产品阅读模式传入 `READ_NORMAL_SLIDE`（或与 Java 阅读等价）。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java DrawFlags.java:8,68-70；Rust DrawFlags.rs:18 常量在；emit_text 未读该 bit。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| PH-OFF | drawPHolderText false | flags|=16 空 ph | 无提示 DrawText | 关键分支 |
| PH-ON | flags 不含 16 | 编辑态 flags | 可有提示字 | 对照 |
| PH-BODY | 非空 txBody | ph 但有用户字 | 仍发射正文 | 边界 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| bit16 | drawPHolderText false | 关键分支 | 是 | PH-OFF |
| 允许提示 | flags 不含 16 | 对照 | 是 | PH-ON |
| 真正文 | 非空 txBody | 边界 | 是 | PH-BODY |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| PH-OFF | bit16 | — | flags|=16 空 ph | 无提示 DrawText | 仍画 Click to add |
| PH-ON | 允许提示 | — | 编辑态 flags | 可有提示字 | 全禁文本 |
| PH-BODY | 真正文 | — | ph 但有用户字 | 仍发射正文 | 正文当提示丢掉 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/mvp_pipeline.rs, src/cn/wps/show/util/DrawFlags.rs
test:  tests/cn/wps/show/render/02_function_ph_hint_flag.rs
[[test]] name = "02_function_ph_hint_flag"
```

`#[ignore = "spec: 02_function_ph_hint_flag"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_ph_hint_flag -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_ph_hint_flag \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --src src/cn/wps/show/util/DrawFlags.rs \
  --report render/features/pr-09-drawflags-product/functions/02_function_ph_hint_flag/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

Loop L5b 必须 PASS。不删 `// java`；列表外方法 `todo!("DEFERRED_NON_FUNCTION")` + 普通 `/* */`。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/util/DrawFlags.rs \
  --java koffice/presentation/core/cn/wps/show/util/DrawFlags.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/util/DrawFlags.java --rs libs/ppt-core/src/cn/wps/show/util/DrawFlags.rs \
  --report render/features/pr-09-drawflags-product/functions/02_function_ph_hint_flag/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | PH-OFF, PH-ON, PH-BODY | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
