# 02_function_prst_path — prst + 绝对框 → DrawPath / DrawRect

> feature: pr-06-prstgeom-skeleton ｜ 状态：**pass**

## 0. 管线能力解锁

01 给出 prst 整数。本 function 是基础元素的几何出口：线/椭圆/多边形用 `DrawPath`（与 pr-02 同类型名），rect 用已有 `DrawRect`。

## 1. 功能说明

`geometry_commands` 今日：有 `graphics_path` 则 Path，否则空 fill 的 AABB `DrawRect`。目标：读 `sp_pr.geom_obj.prst`（或 `graphics_path` 若已由 convert 填好），在 `getShapeRect()` 内生成点列。

同一 I/O，矩阵覆盖三种 prst（不按类拆 function）。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `LayerItem::Shape`：绝对框（pl-08）+ `geom_obj.prst`。 |
| **输出** | prst=1 rect → `DrawRect`（fill 仍由 `apply_fill_line`/pr-05 附着）；prst=20 line → `DrawPath` 2 点（框左右中点或上下，与 OOXML line 约定：连接框对边）；prst=3 ellipse → 闭合点列 ≥8；prst=56 pentagon → ≥5 点闭合。坐标为 render px（`UnitsConverter.render().fromEmu*`，与现 DrawRect 一致）。 |
| **错误** | prst None / 未知 → AABB DrawRect，无 panic。空框 w/h=0 → 空指令或退化点，不 panic。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| `RenderCommand::DrawPath` | rendercommand.rs | path: Vec<(f32,f32)> | ✅ | — | 不改 serde tag |
| `geometry_commands` | 同文件 | 按 prst 分支 | 一律 DrawRect | custGeom | 不发明 DrawEllipse 类型 |
| `UnitsConverter` | pl-01 | fromEmuX/Y | keep | — | — |
| 01 prst | 本 feature | Some(i32) | ✅ 01 pass | — | 不在 02 解析 XML |
| Java `getGraphicsPaths` | drawing Shape | 语义源 | convert 未接线 | 全量 preset | 不搬 PreDefData 全表 |

## 2. 核心代码

Java：`GemoRender.drawShape` → `shape.getGraphicsPaths(w,h)`。Rust：`rendercommand.rs` `geometry_commands`。

改写列表：prst→path 辅助（line/ellipse/pentagon 内接 abs rect）。停车：starN、roundRect 圆角、custGeom。禁改：DrawText。

## 3. 测试方案

### 3.1 审计

已读：`rendercommand.rs` 55-96；`mvp_pipeline.rs` apply_fill_line；Java DGMap line=20 ellipse=3 pentagon=56。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 必测? | test_id |
|---|---|---|---|
| C01 | line → DrawPath 2 点 | 是 | T01 |
| C02 | ellipse → Path 点数>2 且闭合意图 | 是 | T02 |
| C03 | pentagon → Path ≥5 | 是 | T03 |
| C04 | rect / prst=1 → DrawRect | 是 | T04 |
| C05 | prst None → DrawRect AABB | 是 | T05 |
| C06 | 零尺寸框不 panic | 是 | T06 |

### 3.3 可测试性

fixture `KmoShowShape` + `ShowTypoShape::typo_shape_snapshot` + `geometry_commands`。不打开 zip。

### 3.4 用例表

| id | 覆盖项 | 输入 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | prst=20，框 (0,0,100,0) 或非零高 | DrawPath len=2 | 仍 DrawRect |
| T02 | C02 | prst=3 | Path len≥8 | AABB |
| T03 | C03 | prst=56 | Path len≥5 | 套方框 |
| T04 | C04 | prst=1 + hasFill | DrawRect | 误改成 Path |
| T05 | C05 | prst=None | DrawRect | panic |
| T06 | C06 | w=0 h=0 | 不 panic | |

### 3.5 位置

```text
src: src/cn/wps/show/render/rendercommand.rs
test: tests/cn/wps/show/render/02_function_prst_path.rs
```

`#[ignore = "spec: pr-06-02 prst path"]`

### 3.6 脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_prst_path

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_prst_path \
  --src src/cn/wps/show/render/rendercommand.rs \
  --report libs/ppt-core/render/features/pr-06-prstgeom-skeleton/functions/02_function_prst_path/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_prst_path --json`)
date: 2026-09-09 21:05

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/rendercommand.rs` | 62/183 | 33.9% | 101/241 | 41.9% |
| **function total** | **62/183** | **33.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

`rendercommand.rs` 为产品指令模型（无一对一 Java 类）。N/A。禁 `--force` PptxrSlide。

### 3.9 Java↔Rust 结构对齐

<!-- java-rust-align:begin -->
tool: n/a (product I/O)

| Java class | layout | implemented | deferred | missing | result |
|---|---|---|---|---|---|
| (GemoRender 语义源，不整文件移植) | N/A | T01–T06 `preset_path_in_rect` | custGeom | — | **PASS** |

**overall: PASS** — product I/O; geometry_commands prst branches green.
<!-- java-rust-align:end -->
