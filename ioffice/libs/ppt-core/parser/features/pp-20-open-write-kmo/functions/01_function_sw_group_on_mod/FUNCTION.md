# 01_function_sw_group_on_mod — SwGroup 进默认 mod 并分发

> feature: pp-20-open-write-kmo ｜ 状态：**pass** ｜ Java 管线：S4 产线写入

## 0. 管线能力解锁

没有它：CommonSlideDataHandler 引用 SwGroup 但 `handler/shape/mod.rs` 仍注释掉 `pub mod SwGroupShapeHandler`，产线无法走 Java 组树。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | grpSp 事件 int-id（3211593/1594/1469/1592/1595/1596） |
| **输出** | enterSubElement 返回对应 handler；sp/grp/pic 已 addChild |
| **错误** | 未知 subElement：Java fall-through 返回 null；Rust 同 skip。3211596 缺 break 按 Java 落入 3211597。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| SwGroupShapeHandler | pptxr/handler/shape | enterSubElement + init | **本 function**：`mod.rs` 已 `pub mod`；ctor/`init`/`enterSubElement`/`endElement` 已按 Java 改写 | AlternateContent 深层内容；graphicFrame 内容 handler | 禁止 --force PptxrSlide |
| KmoShowShape.addChild | show/app/shape | 双写 children + dg id | pp-19:02 pass | — | 禁止改 spid 位域 |
| SwShapeHandler | 同上 | sp 子 handler | pp-19:04 pass | — | 本 function 只分发 |
| ISlide::KmoPresentation.drawingStore | show/app/slide | Java `canvas.store()` 别名 | 本 function 补 `*mut DgStore` 返回 | 完整 `KmoPresentation.rs` | 禁止 `--force` 产线 slide |

## 2. 核心代码与函数改写列表

**本 function：** `handler/shape/mod.rs` 解开 `pub mod SwGroupShapeHandler`；`enterSubElement` 常量 3211593 sp / 3211594 grpSp / 3211469 pic / 3211592 grpSpPr / 3211595 graphicFrame / 3211596 cxnSp。
**停车：** graphicFrame 内容、AlternateContent 深层。
**禁改：** `--force` PptxrSlide；pp-19 addChild 位域。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java `SwGroupShapeHandler.enterSubElement:43-127`（new KmoShowShape(parent,type) + dgGroup.children().add(id) + kmoGroup.addChild）；Rust `SwGroupShapeHandler.rs` 文件在，`mod.rs:5` 注释禁用；`CommonSlideDataHandler.rs:20` 仍 `use …SwGroupShapeHandler`。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| SG-MOD | handler/shape/mod.rs:5 | cargo check 引用 SwGroupShapeHandler | mod 不再注释 | 公开入口 |
| SG-SP | enterSubElement:76 | subElement=3211593 | 返回 SwShapeHandler 且 child type==1 | 关键分支 |
| SG-GRP | enterSubElement:88 | subElement=3211594 | 嵌套 SwGroup 且 child type==0 | 关键分支 |
| SG-PIC | enterSubElement:58 | subElement=3211469 | SwPictureHandler 且 type==2 | 关键分支 |
| SG-GF | enterSubElement:100 | graphicFrame | type==4 节点存在（内容可空） | 边界 |
| SG-UNK | enterSubElement 默认 | subElement=0 | handler=None，不 panic | 失败 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| 默认编译面 | handler/shape/mod.rs:5 | 公开入口 | 是 | SG-MOD |
| 3211593 | enterSubElement:76 | 关键分支 | 是 | SG-SP |
| 3211594 | enterSubElement:88 | 关键分支 | 是 | SG-GRP |
| 3211469 | enterSubElement:58 | 关键分支 | 是 | SG-PIC |
| 3211595 | enterSubElement:100 | 边界 | 是 | SG-GF |
| 未知 id | enterSubElement 默认 | 失败 | 是 | SG-UNK |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| SG-MOD | 默认编译面 | — | cargo check 引用 SwGroupShapeHandler | mod 不再注释 | 仍注释则产品链断 |
| SG-SP | 3211593 | — | subElement=3211593 | 返回 SwShapeHandler 且 child type==1 | 没 addChild |
| SG-GRP | 3211594 | — | subElement=3211594 | 嵌套 SwGroup 且 child type==0 | 没递归组 |
| SG-PIC | 3211469 | — | subElement=3211469 | SwPictureHandler 且 type==2 | pic 当 sp |
| SG-GF | 3211595 | — | graphicFrame | type==4 节点存在（内容可空） | 直接丢节点 |
| SG-UNK | 未知 id | — | subElement=0 | handler=None，不 panic | 未知当 fatal |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/pptxr/handler/shape/mod.rs, src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs
test:  tests/cn/wps/show/pptxr/handler/shape/01_function_sw_group_on_mod.rs
[[test]] name = "01_function_sw_group_on_mod"
```

Loop 已落地：`tests/cn/wps/show/pptxr/handler/shape/01_function_sw_group_on_mod.rs` 六条非 ignore（SG-MOD/SP/GRP/PIC/GF/UNK）。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_sw_group_on_mod -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_sw_group_on_mod \
  --src src/cn/wps/show/pptxr/handler/shape/mod.rs \
  --src src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs \
  --report libs/ppt-core/parser/features/pp-20-open-write-kmo/functions/01_function_sw_group_on_mod/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_sw_group_on_mod --json`)
date: 2026-09-15 16:49

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs` | 104/283 | 36.7% | 136/302 | 45.0% |
| `src/cn/wps/show/pptxr/handler/shape/mod.rs` | (not in report) | — | — | — |
| **function total** | **104/283** | **36.7%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

**PASS**（2026-09-15）：`check-template-layout.mjs` 对 SwGroup 130 条 `// java` 对齐。不删 `// java`；列表外方法 `todo!("DEFERRED_NON_FUNCTION")` + 普通 `/* */`。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.java --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs \
  --report libs/ppt-core/parser/features/pp-20-open-write-kmo/functions/01_function_sw_group_on_mod/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-15 08:49

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SwGroupShapeHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs` | **PASS** | `SwGroupShapeHandler`, `init`, `enterSubElement`, `endElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
