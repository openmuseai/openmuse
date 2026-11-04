# 03_function_false_skips_master — false 不 film master

> feature: pp-21-show-master-sp ｜ 状态：**pass** ｜ Java 管线：S1 属性 + S9 drawShapesOnLayout

## 0. 管线能力解锁

用户关母版装饰时 Java 不画 master，也不画 layout 树（`if (slide.isShowMasterShape())`）。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | slide showMasterSp=false |
| **输出** | 无 master 树形状 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| drawShapesOnLayout | SlideRender | false 分支 | 在 | — | — |
| 01 parse false | 本 feature | 输入 | **pass** | — | — |

## 2. 核心代码与函数改写列表

**本 function：** false 跳过 master；layout 是否画随 slide.isShowMasterShape。
**§3.8：** SlideRender。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java:110 `if (slide.isShowMasterShape())` 才 film layout 树。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| FS-M | showMasterSp=false | master 有装饰 | 无 master 指令 | 公开入口 |
| FS-L | slide false | layout 装饰 | 无 layout 装饰 | 关键分支 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| slide false | showMasterSp=false | 公开入口 | 是 | FS-M |
| layout 树 | slide false | 关键分支 | 是 | FS-L |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| FS-M | slide false | — | master 有装饰 | 无 master 指令 | 仍画 |
| FS-L | layout 树 | — | layout 装饰 | 无 layout 装饰 | 只跳 master 仍画 layout |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/SlideRender.rs
test:  tests/cn/wps/show/render/03_function_false_skips_master.rs
[[test]] name = "03_function_false_skips_master"
```

Loop 已落地：FS-M / FS-L 非 ignore。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_false_skips_master -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_false_skips_master \
  --src src/cn/wps/show/render/SlideRender.rs \
  --report libs/ppt-core/parser/features/pp-21-show-master-sp/functions/03_function_false_skips_master/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_false_skips_master --json`)
date: 2026-09-15 20:21

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/SlideRender.rs` | 69/130 | 53.1% | 104/165 | 63.0% |
| **function total** | **69/130** | **53.1%** |  |  |
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
  --report libs/ppt-core/parser/features/pp-21-show-master-sp/functions/03_function_false_skips_master/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-15 12:20

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SlideRender` | `koffice/presentation/core/cn/wps/show/render/SlideRender.java` | `libs/ppt-core/src/cn/wps/show/render/SlideRender.rs` | **PASS** | `getOrAllocThumbSlideRender`, `getOrAllocReadSlideRender`, `draw`, `drawUnderSlide`, `drawShapesOnLayout` | `reuseThumbSlideRender`, `reuseReadSlideRender`, `getOrAllocEditSlideRender`, `reuseEditSlideRender`, `getOrAllocMasterSlideRender`, `reuseMasterSlideRender`, `getOrAllocBackSlideRender`, `reuseBackSlideRender`, `createPrintSlideRender` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
