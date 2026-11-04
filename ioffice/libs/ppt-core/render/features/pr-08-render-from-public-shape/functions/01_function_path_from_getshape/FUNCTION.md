# 01_function_path_from_getshape — path 来自公开 Shape

> feature: pr-08-render-from-public-shape ｜ 状态：**spec** ｜ Java 管线：S9 几何/填线（输入必须是 S5 公开 Shape）

## 0. 管线能力解锁

`geometry_commands` 仍可走 layout_shape 框。Java GemoRender 读 `shape.getGraphicsPaths`。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | type1 + prst rect/ellipse 的 typo |
| **输出** | DrawPath/Rect 顶点与 getShape() GraphicsPath 一致 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| ShowTypoShape.getShape | pl-09 | 公开 Shape + GraphicsPath | pass | effect | 禁止第二套 prst 公式当 pass |
| geometry_commands | rendercommand.rs | path 指令 | 可走 layout geom | — | 禁止 preset_path_in_rect 冒充 |

## 2. 核心代码与函数改写列表

**本 function：** `geometry_commands`/`render_slide` 以 `typo.getShape()` path/733 框为准。`preset_path_in_rect` 仅 getShape miss 夹具，不得标 pass。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 `mvp_pipeline::render_slide:290`：pic 捷径读 `s.media_bytes`；`geo = geometry_commands(item)`。`rendercommand.rs:142 geometry_commands`。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| GP-RECT | getShape+geometry_commands | prst rect | 指令框==property(733) | 公开入口 |
| GP-ELL | prst ellipse | prstGeom=ellipse | DrawPath 点数与 GraphicsPath 一致 | 关键分支 |
| GP-MISS | geometry_commands | 无法 convert | 0 条几何或显式 skip，不 panic | 失败 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| rect path | getShape+geometry_commands | 公开入口 | 是 | GP-RECT |
| ellipse | prst ellipse | 关键分支 | 是 | GP-ELL |
| getShape None | geometry_commands | 失败 | 是 | GP-MISS |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| GP-RECT | rect path | — | prst rect | 指令框==property(733) | 捷径框 |
| GP-ELL | ellipse | — | prstGeom=ellipse | DrawPath 点数与 GraphicsPath 一致 | 仍走 rect 近似 |
| GP-MISS | getShape None | — | 无法 convert | 0 条几何或显式 skip，不 panic | 假 path |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/mvp_pipeline.rs, src/cn/wps/show/render/rendercommand.rs
test:  tests/cn/wps/show/render/01_function_path_from_getshape.rs
[[test]] name = "01_function_path_from_getshape"
```

`#[ignore = "spec: 01_function_path_from_getshape"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_path_from_getshape -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_path_from_getshape \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --src src/cn/wps/show/render/rendercommand.rs \
  --report render/features/pr-08-render-from-public-shape/functions/01_function_path_from_getshape/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

§3.8 N/A（mvp_pipeline / rendercommand 产品编排）。

```bash
# 产品编排 / Flutter：无一对一 Java 叶。§3.8 标 N/A；不得 --force 邻近健康模板。
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  # n/a product I/O \
  --report render/features/pr-08-render-from-public-shape/functions/01_function_path_from_getshape/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | GP-RECT, GP-ELL, GP-MISS | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
