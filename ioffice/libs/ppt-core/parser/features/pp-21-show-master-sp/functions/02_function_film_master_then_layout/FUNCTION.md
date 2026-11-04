# 02_function_film_master_then_layout — true 时 master→layout→slide

> feature: pp-21-show-master-sp ｜ 状态：**pass** ｜ Java 管线：S1 属性 + S9 drawShapesOnLayout

## 0. 管线能力解锁

01 缺省 true 后，`SlideRender.drawShapesOnLayout` 按 Java:104-110 film master（若 layout∧slide show）再 layout（若 slide show），`draw()` 再 film slide 树。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | slide+layout+master 皆 showMasterSp true |
| **输出** | 层序含 master 形状再 layout 再 slide（SlideRender.java:104-110） |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| drawShapesOnLayout | SlideRender.rs | 胶片序 | Java 同序已在 | — | 禁止对调 master/layout |
| 01 缺省 true | 本 feature | 输入 | **pass** | — | — |

## 2. 核心代码与函数改写列表

**本 function：** 接 01 的 true 缺省后断言层序。不改 Java 顺序。
**§3.8：** SlideRender。
**禁改：** 对调 master/layout。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 SlideRender.rs:234-250 与 Java:104-110 同序：master 若 layout∧slide show；再 slide.show 则 layout 树。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| FM-ORD | 三层各 1 装饰 | 全 true | 指令序 master,layout,slide | 公开入口 |
| FM-LAY | layout.show=false | layout 不显 master | 无 master 层 | 关键分支 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 层序 | 三层各 1 装饰 | 公开入口 | 是 | FM-ORD |
| layout 关 master | layout.show=false | 关键分支 | 是 | FM-LAY |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| FM-ORD | 层序 | — | 全 true | 指令序 master,layout,slide | 只有 slide |
| FM-LAY | layout 关 master | — | layout 不显 master | 无 master 层 | 仍画 master |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/SlideRender.rs
test:  tests/cn/wps/show/render/02_function_film_master_then_layout.rs
[[test]] name = "02_function_film_master_then_layout"
```

Loop 已落地：FM-ORD / FM-LAY 非 ignore。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_film_master_then_layout -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_film_master_then_layout \
  --src src/cn/wps/show/render/SlideRender.rs \
  --report libs/ppt-core/parser/features/pp-21-show-master-sp/functions/02_function_film_master_then_layout/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_film_master_then_layout --json`)
date: 2026-09-15 20:21

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/SlideRender.rs` | 75/130 | 57.7% | 117/165 | 70.9% |
| **function total** | **75/130** | **57.7%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

**PASS**（2026-09-15）：SlideRender 布局校验 PASS。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/SlideRender.rs \
  --java koffice/presentation/core/cn/wps/show/render/SlideRender.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/SlideRender.java --rs libs/ppt-core/src/cn/wps/show/render/SlideRender.rs \
  --report libs/ppt-core/parser/features/pp-21-show-master-sp/functions/02_function_film_master_then_layout/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-15 12:20

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SlideRender` | `koffice/presentation/core/cn/wps/show/render/SlideRender.java` | `libs/ppt-core/src/cn/wps/show/render/SlideRender.rs` | **PASS** | `getOrAllocThumbSlideRender`, `getOrAllocReadSlideRender`, `draw`, `drawUnderSlide`, `drawShapesOnLayout` | `reuseThumbSlideRender`, `reuseReadSlideRender`, `getOrAllocEditSlideRender`, `reuseEditSlideRender`, `getOrAllocMasterSlideRender`, `reuseMasterSlideRender`, `getOrAllocBackSlideRender`, `reuseBackSlideRender`, `createPrintSlideRender` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
