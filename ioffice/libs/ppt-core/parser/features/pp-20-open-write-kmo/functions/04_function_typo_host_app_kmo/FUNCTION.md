# 04_function_typo_host_app_kmo — ShowTypoShape 宿主切 app Kmo

> feature: pp-20-open-write-kmo ｜ 状态：**pass** ｜ Java 管线：S4 产线写入

## 0. 管线能力解锁

今日 `ShowTypoShape` 宿主为 `app::KmoShowShape`（`Rc` 别名，对齐 Java `mShapeBase`）。layout_shape 仅测试夹具，经 `ConvertKmo::from_layout` 接入。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | app::KmoShowShape |
| **输出** | ShowTypoShape.mShapeBase 类型非 layout_shape |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| ShowTypoShape | show/render | 宿主 app Kmo | **pass**（`app::KmoShowShape`） | getShapeRect 政策；ph() | 禁止删 // java |
| ConvertTool | show/convert | getShape | pl-09 pass | effect/3D | 禁止改回 layout 别名 |
| SlideRender | show/render | film 树类型 | ConvertKmo 胶片宿主 | 胶片序 pr-09/pp-21 | 禁止删 // java |

## 2. 核心代码与函数改写列表

**本 function：** 字段/ctor 改 `app::KmoShowShape`（可经 ConvertKmo 适配）。layout_shape 仅测试夹具。
**停车：** getShapeRect 冻结政策（G6，写入口径不改 Java）。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 `ShowTypoShape.rs` 宿主 `app::KmoShowShape`；`SlideRender::film_one` 经 `ConvertKmo::from_layout`；`mvp_pipeline` 不再从 `getShowShape()` 读 layout 字段。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| TH-IMP | ShowTypoShape.rs:13 | rg layout_shape::KmoShowShape | 生产 src 零命中 | 门闩 |
| TH-GET | ShowTypoShape.getShape | app type1 + prst | Some(公开 Shape) | 公开入口 |
| TH-REG | 01_function_getshape_cache | cache 命中 | 仍绿 | 回归 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| import 门闩 | ShowTypoShape.rs:13 | 门闩 | 是 | TH-IMP |
| getShape | ShowTypoShape.getShape | 公开入口 | 是 | TH-GET |
| 回归 pl-09:01 | 01_function_getshape_cache | 回归 | 是 | TH-REG |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| TH-IMP | import 门闩 | — | rg layout_shape::KmoShowShape | 生产 src 零命中 | 仍 import fixture |
| TH-GET | getShape | — | app type1 + prst | Some(公开 Shape) | 宿主切了 convert 红 |
| TH-REG | 回归 pl-09:01 | — | cache 命中 | 仍绿 | 切类型打爆 cache |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/ShowTypoShape.rs
test:  tests/cn/wps/show/render/04_function_typo_host_app_kmo.rs
[[test]] name = "04_function_typo_host_app_kmo"
```

`[[test]]` name = `04_function_typo_host_app_kmo`。Loop 已接线，无 `#[ignore]`。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_typo_host_app_kmo -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_typo_host_app_kmo \
  --src src/cn/wps/show/render/ShowTypoShape.rs \
  --report libs/ppt-core/parser/features/pp-20-open-write-kmo/functions/04_function_typo_host_app_kmo/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_typo_host_app_kmo --json`)
date: 2026-09-15 18:52

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/ShowTypoShape.rs` | 66/215 | 30.7% | 89/256 | 34.8% |
| **function total** | **66/215** | **30.7%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

**PASS**（2026-09-15）：`check-template-layout.mjs --rs ShowTypoShape.rs --java ShowTypoShape.java`。不删 `// java`；列表外方法 `todo!("DEFERRED_NON_FUNCTION")` + 普通 `/* */`。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs \
  --java koffice/presentation/core/cn/wps/show/render/ShowTypoShape.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/ShowTypoShape.java --rs libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs \
  --report libs/ppt-core/parser/features/pp-20-open-write-kmo/functions/04_function_typo_host_app_kmo/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-15 10:52

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShowTypoShape` | `koffice/presentation/core/cn/wps/show/render/ShowTypoShape.java` | `libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs` | **PASS** | `ShowTypoShape`, `isPlaceholder`, `placeholderType`, `isTextBox`, `getShowShape`, `isHidden`, `isModularGroupDefaultPic`, `getRect`, `getShapeRect`, `clearShapeRectCache`, `getChildCount`, `getChild`, `getShape`, `getShapeRenderRect` | `setRenderOverride`, `setTextRenderModel`, `getTextRenderModel`, `render`, `getRenderModel`, `getInterpretCtx`, `calculateBounds`, `update3dTransfromMatrix`, `setIsPlayer`, `hasPlayer` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
