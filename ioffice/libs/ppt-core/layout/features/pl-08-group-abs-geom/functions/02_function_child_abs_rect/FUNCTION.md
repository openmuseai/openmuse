# 02_function_child_abs_rect — 组内子级 slide 绝对框写回

> feature: pl-08-group-abs-geom ｜ 状态：**pass** ｜ 管线：[SKELETON_PIPELINE.md](../../../../../SKELETON_PIPELINE.md)

## 0. 管线能力解锁

01 只给组坐标系。本 function 把子 xfrm 从「组局部」变成 **slide 绝对 EMU** 并写回 `transform2D*`，否则 `film_shapes` / `getShapeRect` 仍用局部 off。

## 1. 功能说明

Java `ShapeUtil.calcRelativeRect`：子先取自身 off/ext；若 `parentGroup()!=null`，用 `calcShape2TopMatrix(parent)` 得组框，再 `calcChildRelativeRect`。该结果在 **组局部渲染空间**（缩放用 `rcGroup.width/height`，**不加**组 slide 原点——pl-02 T02）。产品 `RenderCommand` 无 canvas translate → writeback 必须把局部框 **offset 组绝对 origin**。

`page_inheritance::resolve_shape` 已 `child.parent = group` 并调用 `calcRelativeRect`，但只把结果放进 `ResolvedShape.abs`，**不写回** `transform2D*`。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | 组（已有 `child_off`/`child_ext` + 自身绝对或 slide 级 off/ext）+ 子（局部 off/ext）。嵌套组：先算父组绝对框再算孙。 |
| **输出** | 每个非组 child：`transform2DX/Y/W/H` = slide 绝对 EMU。组节点自身保持组 off/ext。 |
| **状态** | 写 `transform2D*`；不改 `child_off`。 |
| **错误** | 组缺 chOff 或 chExt → Java 不映射，子保持局部坐标（与现网一致，断言「不 panic」）；`chExt.cx==0` → scaleX=1.0。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `ShapeUtil::calcChildRelativeRect` | pl-02:02 | 组局部 RectF | ✅ 叶测 pass | 旋转象限（可读 0） | **禁止复制公式**到 page_inheritance |
| `calcRelativeRect` | pl-02:01 | 有 parent 时调 02 | ✅ | — | — |
| `resolve_pages` | pl-07 / `page_inheritance.rs` | parent 链 + **abs 写回 transform2D*** | ✅ 非组 child 写回 slide 绝对；组保持 off/ext | — | 不改 child XML 序 |
| 01 chOff | 本 feature | `hasChildPoint/Extents` | ✅ 01 pass | — | 不在 02 里解析 XML |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/util/ShapeUtil.java` 151-209 |
| Rust | `ShapeUtil.rs`（只读调用）；`page_inheritance.rs` `resolve_shape` |

### 2.2 函数改写列表

- **本 function：** `resolve_shape` 在 `calcRelativeRect` 后把 **slide 绝对** 写回 child（局部 + 组 origin）。两级组：先父后子（已有递归）。
- **停车：** `calcShapeMatrix` 非 null Matrix 路径。
- **禁改：** `ShapeUtil` 数值；`PptxrSlide.rs`。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`ShapeUtil.java` 151-209；`ShapeUtil.rs` `calcChildRelativeRect` 273+；`page_inheritance.rs` 47-63（`ResolvedShape.abs` 不写回）；`SlideRender.rs` `film_shapes` 读 `transform2D*`。

| symbol | 路径 | 输入 | 输出 | 分支 |
|---|---|---|---|---|
| `calcChildRelativeRect` | ShapeUtil | 1:1 组 chExt=组 ext，子 (0,0,300,300) | 局部 (0,0,300,300) | 不加组 off |
| writeback | page_inheritance | 组 off=(1000,2000) + 上式 | 子 transform=(1000,2000,300,300) | 漏加 origin |
| 缺 chOff | hasChildPoint false | 子 off=5000 | 子仍 5000（不映射） | 与 Java 一致 |
| cx==0 | Java 161 | scaleX=1 | 不 panic | |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 1:1 组 + writeback 含组 origin | 公开入口 | 是 | T01 |
| C02 | 无 chOff → 不映射 | 失败 | 是 | T02 |
| C03 | 两级组 | 分支 | 是 | T03 |
| C04 | chExt cx=0 → scale 1 | 边界 | 是 | T04 |
| C05 | `resolve_pages` 后 `transform2D*`==abs 而非仅 `ResolvedShape.abs` | 状态转换 | 是 | T05 |

### 3.3 可测试性

fixture 构造 `KmoShowShape`（组+子），调 `resolve_page` / `calcRelativeRect`。不打开 Sample zip。

### 3.4 用例表

| id | 覆盖项 | 前置 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|---|
| T01 | C01 | 01 字段已填 | 组 off=(2447933,1269734) ext=(E) chOff=(5143500,2476500) chExt=(S)；子局部 (5143500,2476500,w,h) 对齐 chOff 原点 | 子 abs origin ≈ 组 off（1 EMU） | 子仍停在 5143500 |
| T02 | C02 | child_off=None | 子 off=5000 | transform2DX 仍 5000 | 伪造映射 |
| T03 | C03 | 组套组 | 孙相对内组 | 与 Java 两层 fixture 同方向 | 只映射一层 |
| T04 | C04 | chExt=(0,100) | scaleX=1 | 不 panic，宽不除零 | |
| T05 | C05 | 同 T01 | `resolve_page` 后 shape.transform2DX | 等于子 abs.left | 只写 `out.abs` |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/layout/page_inheritance.rs
       src/cn/wps/show/util/ShapeUtil.rs（只读）
test:  tests/cn/wps/show/layout/02_function_child_abs_rect.rs
[[test]] name = "02_function_child_abs_rect"
```

L2 落 `[[test]]` name=`02_function_child_abs_rect`。Loop 后非 ignore。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_child_abs_rect

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_child_abs_rect \
  --src src/cn/wps/show/layout/page_inheritance.rs \
  --report libs/ppt-core/layout/features/pl-08-group-abs-geom/functions/02_function_child_abs_rect/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_child_abs_rect --json`)
date: 2026-09-09 20:57

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/layout/page_inheritance.rs` | 58/83 | 69.9% | 79/114 | 69.3% |
| **function total** | **58/83** | **69.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

`page_inheritance.rs` 无一对一 Java 类（编排）。`ShapeUtil.rs` 已 pass，本 function **禁止改**其交错模板。

```bash
# 不跑 check-template-layout 替代 ShapeUtil；禁 --force PptxrSlide.rs
```

结果：**PASS（N/A 产品编排）** — 前提是未改 `ShapeUtil.rs`。

### 3.9 Java↔Rust 结构对齐报告

<!-- java-rust-align:begin -->
tool: n/a (orchestrator / product I/O; ShapeUtil.rs is call-only D, not edited)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (none owned; writeback in page_inheritance) | N/A | `resolve_shape` writeback T01–T05 | — | — | **PASS** |

**overall: PASS** — product orchestrator; ShapeUtil not edited; I/O matrix green.
<!-- java-rust-align:end -->
