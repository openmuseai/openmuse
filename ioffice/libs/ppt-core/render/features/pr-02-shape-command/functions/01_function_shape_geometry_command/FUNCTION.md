# 01_function_shape_geometry_command — 几何到 DrawRect/DrawPath

> feature: pr-02-shape-command ｜ 状态：**pass** ｜ Java 管线环节：S4（几何），对照 `GemoRender.drawShape`

## 0. 管线能力解锁

实现后：pr-01 的有序层列表第一次变成**几何指令**——白底 → 全幅 `DrawRect`、矩形几何 → `DrawRect`、预置几何 → `DrawPath`，坐标经 `UnitsConverter.render().fromEmuX/Y` 换算成 render px。没有它，管线断在 S4：`LayeredTypoShape` 只有对象，没有指令（主文档：**pr-02 是最小可画出口**；断点表「形状层无指令 → 最小可画断」）。

## 1. 功能说明

`LayerItem（白底/背景/形状）→ Vec<GeometryCommand>`（DrawRect/DrawPath，样式字段待 02 附着）。几何源：`LayeredTypoShape.getShapeRect()`（pl-02 绝对 rect）与 `Shape.getGraphicsPaths(w,h)`（预置几何 `GraphicsPath[]`）。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `LayerItem`（`WhiteBackground{w,h}` / `Background` / `Shape{LayeredTypoShape}`）；pl-02 已给 `Shape.getGraphicsPaths` 的 MVP 子集（矩形 + 预置几何）。 |
| **输出** | `Vec<GeometryCommand>`：`DrawRect{x,y,w,h,fill:None,stroke:None}`（白底/矩形/背景整幅）、`DrawPath{path, fill:None, stroke:None}`（预置几何）。坐标统一 `UnitsConverter.render().fromEmuX/Y(emu)`（Java `SlideRenderTool.drawSlideWhiteBackground` 与 `GemoRender` 均以 render px 绘制）。 |
| **状态与副作用** | 纯转换无副作用。`GemoRender._render` 的 rect = `(0,0,rectOld.width,rectOld.height)`（绝对 rect 平移量由调用方 canvas translate 承担）——Rust 侧指令直接携带绝对坐标，不再有 canvas translate 语义。 |
| **错误语义** | 无 Java 异常路径（`drawDiagram`/`drawInk` 的 catch 吞掉均返回 false 走 drawShape）；Rust 侧 `getShape()`/path 为 None → 无几何指令（跳过形），不许 panic。 |

### 1.2-1.6 已并入 1.1（几何决策纯函数）

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 层列表 | pr-01-01 | LayerItem 枚举 | spec 未接线 | 先 fixture | — |
| 绝对 rect / GraphicsPath | pl-02 | `getShapeRect()`、`Shape.getGraphicsPaths` MVP 子集 | pl-02 spec | 复杂自定义几何 | 不在这里重算几何 |
| UnitsConverter | pl-01 | `render().fromEmuX/Y` | keep | — | 不改换算 |
| 公开 Shape | pl-06/convert | `LayeredTypoShape.getShape()` | convert MISSING | fixture 几何 | 不碰 ConvertTool |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/render/ShapeRender.java`（`_render`）+ `show/moffice/drawing/render/DrawingRender.java` + `GemoRender.java`（`_render`/`drawShape`） |
| Rust | `libs/ppt-core/src/cn/wps/show/render/rendercommand/RenderCommand.rs`（指令模型，新建）+ `src/cn/wps/show/render/ShapeRender.rs`（01 的几何分支） |

关键方法（对照 Java，均已查证）：

| symbol | 语义（Java） | Rust 目标 |
|---|---|---|
| `_render()`（ShapeRender） | `DrawingRender render; render.setShapeEnv(mEnv); render.render((ITypoShape)mTypoShape)` | 几何决策入口（renderMode 默认分支） |
| `DrawingRender.render(ITypoShape)` | `setTextDrawn(false)`；renderMode 2→renderText；1→shape.render；默认→shape.render+未画字则renderText+renderDecor | 只取 shape 分支（renderMode 2/1 与 renderText/renderDecor 转 pr-03/停车） |
| `GemoRender._render` | `rect=(0,0,w,h)`；`drawDiagram`→`drawInk`→`drawShape(env, shape, rect, shape.getRotation())` | diagram/ink 返回空；走几何 |
| `GemoRender.drawShape(IGraphics, RenderModel, RectF, float rotation)` | `drawFill` + `drawStroke`；`shape.getGraphicsPaths(rectF.width(), rectF.height())` | **路径头**：`paths==null` → 无指令 |
| `Shape.getGraphicsPaths(float,float)` | `DataCal.getGraphicsPaths(w,h)` → `GraphicsPath[]`（`getNeedFill()`/`getNeedSroke()` 每 path 判定） | pl-02 语义复用 |
| `GraphicsPath` | `getNeedFill`/`getNeedSroke`/`getFill`/`getLineProperty` | 只读字段 |
| `UnitsConverter.render().fromEmuX/Y` | EMU→render px | 一次换算 |
| `SlideRenderTool.drawSlideWhiteBackground` | `drawRect(0,0,w,h,WHITE)`（white=0xFFFFFFFF FILL） | 白底 → 全幅 DrawRect（fill 白实色） |

改写列表：01 的 `LayerItem→GeometryCommand` 映射（白底/背景/形状三条）、`getGraphicsPaths` 的 needFill/needSroke 读取（决策字段，颜色解析在 02）。

停车列表：`drawDiagram`（SmartArt 位图）、`drawInk`、GroupRender 递归、`Shape3dRender`/效果层、rotation 的 canvas rotate 语义（MVP 先记 `rotation` 为指令元数据，待在 02 侧确认是否交付 painter）。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读 `GemoRender._render`/`drawShape`/`drawFill`/`drawStroke` 与 `Shape.getGraphicsPaths`（重点：rect 原点语义、paths null 分支、needFill/needSroke 判定）。当前 Rust 无 `rendercommand/`，loop 前以 Java 语义为准。

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| 白底 | w,h | DrawRect 全幅 | 空尺寸 |
| 矩形几何 | LayeredTypoShape | DrawRect | paths null |
| 预置几何 | GraphicsPath[] | DrawPath | needSroke 单独形状 |
| 背景 | Background 层 | DrawRect 整幅 | — |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| G01 | 白底 → DrawRect(0,0,w,h) | 主路径 | 是 | T01 |
| G02 | 矩形几何 → DrawRect（坐标 fromEmuX/Y） | 主路径 | 是 | T02 |
| G03 | 预置几何 → DrawPath（path 点阵一致） | 主路径 | 是 | T03 |
| G04 | 背景层 → 整幅 DrawRect | 主路径 | 是 | T04 |
| G05 | paths==null / getShape()==None → 空指令 | 边界 | 是 | T05 |

### 3.3 可测试性

fixture `LayeredTypoShape`（矩形/预置几何）+ golden 指令 JSON；不连 canvas。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | G01 | WhiteBackground{1920,1080} | 1×DrawRect{0,0,1920,1080} | 红测停在本 function |
| T02 | G02 | 矩形 xfrm 1000×1000 EMU | DrawRect 坐标 = fromEmuX/Y(1000) | 红测停在本 function |
| T03 | G03 | prstGeom 椭圆 fixture | DrawPath 点阵与 pl-02 一致 | 红测停在本 function |
| T04 | G04 | 背景层 fixture | 整幅 DrawRect | 红测停在本 function |
| T05 | G05 | getShape() None | 零指令输出 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/render/rendercommand/RenderCommand.rs（+ ShapeRender.rs 几何分支）
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "01_function_shape_geometry_command"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_shape_geometry_command

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_shape_geometry_command \
  --src src/cn/wps/show/render/rendercommand/RenderCommand.rs \
  --report libs/ppt-core/render/features/pr-02-shape-command/functions/01_function_shape_geometry_command/FUNCTION.md
```

工具缺失先：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`。

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_shape_geometry_command --json`)
date: 2026-08-30 17:26

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/rendercommand.rs` | 41/113 | 36.3% | 39/106 | 36.8% |
| **function total** | **41/113** | **36.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/rendercommand/RenderCommand.rs \
  --java koffice/presentation/core/cn/wps/show/render/ShapeRender.java
```

Loop 结果：**PASS**（`rendercommand.rs` 无 Java 双子；`--rs` 不配对 ShapeRender.java）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/ShapeRender.java \
  --rs libs/ppt-core/src/cn/wps/show/render/rendercommand/RenderCommand.rs \
  --report libs/ppt-core/render/features/pr-02-shape-command/functions/01_function_shape_geometry_command/FUNCTION.md
```

<!-- java-rust-align:begin -->
| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
| N/A (`rendercommand.rs`; Canvas not ported) | PASS (`--rs` only / no Java twin) | command I/O in this function | none | none | PASS |
| **overall** | | | | | **PASS** |
<!-- java-rust-align:end -->