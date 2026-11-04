# 03_function_film_group_children — 层列表含组内 shape

> feature: pl-08-group-abs-geom ｜ 状态：**pass** ｜ 管线：[SKELETON_PIPELINE.md](../../../../../SKELETON_PIPELINE.md)

## 0. 管线能力解锁

02 写回绝对框之后，若 `film_shapes` 只扫顶层 `grpSp`，组内矩形/五边形/图仍不会变成 `LayerItem`。这是布局正确性的最后一跳 I/O（模型树 → 渲染层），不是「再算一次几何」。

## 1. 功能说明

Java `SlideRender` 画 shape 树时进入 group 画子级，组容器本身通常不单独铺一个空 AABB。产品 `KmoShapeTree::shapeCount` = 顶层 `children.len()`；`film_shapes` 对每个顶层节点 `typo_shape_snapshot(transform2D*)`，**不看** `KmoShowShape.children`。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `KmoSlide.sp_tree`：顶层可含 `shape_type==0` 且 `children` 非空的组（子级已是 02 的绝对框）。 |
| **输出** | `SlideRender::layer_order` 中，每个可见叶子（type≠0，或无子的 sp）一条 `LayerItem::Shape`；深度优先、保持 XML 兄弟序。组容器 **不** 额外发射与组 xfrm 等大的空 `DrawRect`（除非组自身有 fill——本波组无 fill，跳过容器）。 |
| **状态** | 只读树；不改 xfrm。 |
| **错误** | 空 children 的组 → 0 条 Shape（或仅白底）；不 panic。隐藏 `is_hidden` 跳过（若字段已有）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `film_shapes` | `SlideRender.rs` | 递归 `shape.children` | ✅ type==0 DFS 子级；空组 0 条 | master/layout 装饰树同规则 | 不改 DrawFlags 全集 |
| 02 abs | 本 feature | 子 `transform2D*` 已是 slide 绝对 | ✅ 02 pass | — | 不在 03 里算 chOff |
| `ShowTypoShape::typo_shape_snapshot` | pl-06 | 冻结 EMU 框 | keep | Java 每次 calcRelativeRect | 不改 UnitsConverter |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/render/SlideRender.java`（drawSlide / 组递归，语义源） |
| Rust | `libs/ppt-core/src/cn/wps/show/render/SlideRender.rs` `film_shapes` |

### 2.2 函数改写列表

- **本 function：** `film_shapes` DFS；`type==0 && !children.is_empty()` → 只 film 子级。
- **停车：** `drawShapesOnLayout` 母版树（`show_master_shape` 仍 false）；ink。
- **禁改：** `geometry_commands` 的 prst 决策（pr-06）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`SlideRender.rs` 69-86、174-189；`layout_shape.rs` `shapeCount` 429-434；`parse_one_grp` 把子放进 `group.children`。

| symbol | 路径 | 输入 | 输出 | 分支 |
|---|---|---|---|---|
| `film_shapes` | SlideRender | 1 组 + 2 子 rect | 2 条 Shape，0 条组 AABB | 现 1 条（组框） |
| 嵌套组 | 组⊃组⊃sp | 1 条叶子 | 漏孙 |
| 顶层非组 | 1 个 sp | 1 条 | 回归 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 一组两子 → Shape 层数=2 | 公开入口 | 是 | T01 |
| C02 | 空组 | 失败/空 | 是 | T02 |
| C03 | 两级组 → 只 film 叶子 | 分支 | 是 | T03 |
| C04 | 顶层普通 sp 仍 1 条 | 回归 | 是 | T04 |

### 3.3 可测试性

`SlideRender::layer_order(Some(&slide), 0, 0)`，fixture 内存 slide。过滤 `LayerItem::Shape`。不跑 Flutter。

### 3.4 用例表

| id | 覆盖项 | 前置 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|---|
| T01 | C01 | 02 已写 abs | 组+子 A/B | Shape 条数=2；snapshot 框=子 transform | 只有组框 |
| T02 | C02 | — | type=0 children=[] | Shape 0（白底可有） | panic |
| T03 | C03 | — | 外组⊃内组⊃1 sp | Shape=1（叶子） | 3 条套叠框 |
| T04 | C04 | — | 顶层 sp 无组 | Shape≥1 含该 sp | 误删顶层 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/SlideRender.rs
test:  tests/cn/wps/show/render/03_function_film_group_children.rs
[[test]] name = "03_function_film_group_children"
```

Split：`#[ignore = "spec: pl-08-03 film children"]`。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_film_group_children

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_film_group_children \
  --src src/cn/wps/show/render/SlideRender.rs \
  --report libs/ppt-core/layout/features/pl-08-group-abs-geom/functions/03_function_film_group_children/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_film_group_children --json`)
date: 2026-09-09 20:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/SlideRender.rs` | 72/126 | 57.1% | 104/163 | 63.8% |
| **function total** | **72/126** | **57.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/SlideRender.rs \
  --java koffice/presentation/core/cn/wps/show/render/SlideRender.java
```

Loop 后非 ignore。布局：**PASS**。禁止 `--force` PptxrSlide。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/SlideRender.java \
  --rs libs/ppt-core/src/cn/wps/show/render/SlideRender.rs \
  --report libs/ppt-core/layout/features/pl-08-group-abs-geom/functions/03_function_film_group_children/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-09 12:59

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SlideRender` | `koffice/presentation/core/cn/wps/show/render/SlideRender.java` | `libs/ppt-core/src/cn/wps/show/render/SlideRender.rs` | **PASS** | `getOrAllocThumbSlideRender`, `getOrAllocReadSlideRender`, `draw`, `drawUnderSlide`, `drawShapesOnLayout` | `reuseThumbSlideRender`, `reuseReadSlideRender`, `getOrAllocEditSlideRender`, `reuseEditSlideRender`, `getOrAllocMasterSlideRender`, `reuseMasterSlideRender`, `getOrAllocBackSlideRender`, `reuseBackSlideRender`, `createPrintSlideRender` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
