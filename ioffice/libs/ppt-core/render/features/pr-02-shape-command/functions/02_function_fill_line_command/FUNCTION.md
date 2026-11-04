# 02_function_fill_line_command — 实色 fill/line 附着指令样式

> feature: pr-02-shape-command ｜ 状态：**pass** ｜ Java 管线环节：S1+S4（`GemoRender.drawFill/drawStroke` + `FillConvert` 语义子集）

## 0. 管线能力解锁

实现后：01 的几何指令获得**可消费的样式**——`SolidFill`（fillType==0）实色填充、`LineProperty` 线色/线宽、背景整幅实色。没有它，管线断在 S4 最后一步：DrawRect/DrawPath 是「无色几何」，painter 无从下笔（本 function 与 01 共同构成 pr-02 的最小可画出口）。

## 1. 功能说明

`GeometryCommand + RenderModel（getFill()/getLineProperty()）+ 背景 fill → RenderCommand`。决策完全对照 `GemoRender.drawFill/drawStroke`：`path.getNeedFill()` 且 `fill.getFillOn()` → 附着 fill；`path.getNeedSroke()` 且 `line.getfLine()` → 附着 stroke。MVP 只解析 `SolidFill`，其余 fill 类型跳过（不许画成实色冒充）。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | 01 的 `GeometryCommand`（或同源 `GraphicsPath[]` 的 needFill/needSroke 决策）+ `RenderModel.getFill()`（`FillBase`）+ `getLineProperty()`（`LineProperty`）；背景层走 `ConvertTool.backgroudToPublicShape` 的 fill（实例为 `FillBase`）。 |
| **输出** | `RenderCommand`：`DrawRect/DrawPath` 携带 `fill: Rgba255(fillType==0 时) \| None`、`stroke: Stroke{color,width} \| None`。fill/stroke 的 on/off 对照：`fill.getFillOn()`、`line.getfLine()`；颜色 = `getColor()`（ARGB int → RGBA255），宽度 = `getWidth()`（EMU 语义待 loop 确认换算）。 |
| **状态与副作用** | `GemoRender.drawFill` 中 `defualtFill.setIsFillNoLine(stroke == null \|\| !stroke.getfLine())` 的联动（无线形状 fill 判定）——Rust 侧同语义算出；`backupAndAdjustFillRect`/`useBgFill` 背景整幅扩展只在背景层与 `getUseBgFill` 时生效（post-MVP）。 |
| **错误语义** | 无异常路径（`GemoRender` exceptionHandler 仅 BlipFill 回调事件）；Rust 侧 fill 为 None → 无 fill 字段，不许 panic。 |

### 1.2-1.6 已并入 1.1（样式解析纯函数）

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| FillBase/SolidFill | core_libs_drawing | `getFillType()`/`getFillOn()`/`getColor()`/`getTransparency()` | keep（`graphics/FillBase.rs` 已有） | 渐变/图案/blip | 不改库语义 |
| LineProperty | core_libs_drawing | `getfLine()`/`getColor()`/`getWidth()` | keep（`line/LineProperty.rs` 已有） | dashing/lineStyle | — |
| 背景 fill | convert | `ConvertTool.backgroudToPublicShape` 语义 | convert MISSING | 背景实色先行 | 不实现整 convert |
| 样式来源（主题色） | pp-03/pp-04 | scheme 色解析 | spec | — | — |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `show/moffice/drawing/render/GemoRender.java`（`drawFill`/`drawStroke`）+ `core-libs/drawing` `FillBase`/`SolidFill`/`LineProperty` + `render/ShowTypoBackground.java`（背景 fill，`getShape()`→`backgroudToPublicShape`） |
| Rust | `libs/ppt-core/src/cn/wps/show/render/rendercommand/RenderCommand.rs`（样式字段）+ `src/cn/wps/show/render/ShapeRender.rs`（02 的 fill/line 分支） |

关键方法（对照 Java，均已查证）：

| symbol | 语义（Java） | Rust 目标 |
|---|---|---|
| `GemoRender.drawFill(IGraphics, RenderModel, RectF, rotation)` | `FillBase defaultFill = shape.getFill()`；`setIsFillNoLine(stroke==null \|\| !stroke.getfLine())`；逐 path：`path.getNeedFill()`→`fill = path.getFill()!=null?path.getFill():defaultFill`→`fill!=null && fill.getFillOn()`→ 画 | 决策：fill 来源（path 优先）+ on/off |
| `GemoRender.drawStroke` | `LineProperty defaultStroke = shape.getLineProperty()`；逐 path `getNeedSroke()`→`stroke = path.getLineProperty()!=null?…:defaultStroke`→`stroke!=null && stroke.getfLine()`→ 画 | 决策：stroke 来源 + on/off |
| `FillBase.getFillType()` | SolidFill=0（`SolidFill` 调 `super(0)`） | 仅 type==0 实色解析 |
| `FillBase.getColor()` | ARGB int | → RGBA255 输出 |
| `FillBase.getFillOn()` | 是否画 fill | on/off 位 |
| `LineProperty.getfLine()/getColor()/getWidth()` | 是否画线/线色/线宽 | stroke 字段 |
| `ShowTypoBackground.getShape()` | `ConvertTool.backgroudToPublicShape(...)` + `getFill()` | 背景整幅 fill 入口（实色 MVP） |
| `BackgroundShapeRender._begin` | `hasBackground()` && `(drawFlags & 8)==0` | pr-01-01 已在层生成期判断，02 不重复 |

改写列表：fill/stroke 决策与附着（含 path 级 fill/line 优先覆盖）、SolidFill→RGBA、noFill/noLine 两分支、背景整幅实色。

停车列表：`GradFill`/`PatternFill`/`BlipFill`/`Picture` 分支（`exceptionHandler` BlipFill 事件回调）、`useBgFill` 与 `adustFillToRect`（title 渐变调整）、`getTransparency` 透明度、虚线 `getDashing`、fill rot 跟随形状旋转。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读 `GemoRender.drawFill/drawStroke` 全量（重点：defaultFill/defaultStroke 的 path 级覆盖、`setIsFillNoLine` 联动）与 `SolidFill`/`LineProperty` 的字段默认值。Rust 侧 `FillBase.rs`/`LineProperty.rs` 已 keep，方法名以既有实现为准（差异待 loop 对照）。

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| drawFill | fill null/on/off | fill 附着 | path 级覆盖 |
| drawStroke | stroke null/fLine | stroke 附着 | noLine |
| 背景 | Background fill | 整幅色 | — |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 实色 fill（type 0）→ RGBA 附着 | 主路径 | 是 | T01 |
| C02 | noFill（getFillOn false → fill None） | 分支 | 是 | T02 |
| C03 | line（getfLine true）→ stroke{color,width} | 主路径 | 是 | T03 |
| C04 | noLine（getfLine false / line null）→ stroke None | 分支 | 是 | T04 |
| C05 | 背景整幅实色 | 主路径 | 是 | T05 |
| C06 | fillType!=0（渐变/blip）→ 跳过，不冒充实色 | 分支 | 是 | T06 |
| C07 | path 级 fill/line 优先覆盖 defaultFill/defaultStroke | 分支 | 否 | `#[ignore = "spec: post-MVP"]`（复杂几何常有） |
| C08 | useBgFill / transparency / dashing | defer | 否 | `#[ignore = "spec: post-MVP"]` |

### 3.3 可测试性

直接构造 `FillBase`/`LineProperty` fixture（drawing crate 已有）驱动决策函数；指令 golden JSON 断言；不进画布。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | SolidFill 0xFFFF0000 | fill=RGBA(255,0,0,255) | 红测停在本 function |
| T02 | C02 | fill.getFillOn()==false | fill=None | 红测停在本 function |
| T03 | C03 | LineProperty{color:0xFF000000, width:2emu} | stroke{color,width} | 红测停在本 function |
| T04 | C04 | getfLine()==false | stroke=None | 红测停在本 function |
| T05 | C05 | 背景 SolidFill | 整幅 DrawRect fill 附着 | 红测停在本 function |
| T06 | C06 | GradFill type!=0 | 跳过 fill（指令仍出，无色） | 红测停在本 function |
| T07 | C07 | path 带独立 fill/stroke | path 级覆盖 | ignore |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/render/rendercommand/RenderCommand.rs（样式字段）
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "02_function_fill_line_command"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_fill_line_command

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_fill_line_command \
  --src src/cn/wps/show/render/rendercommand/RenderCommand.rs \
  --report libs/ppt-core/render/features/pr-02-shape-command/functions/02_function_fill_line_command/FUNCTION.md
```

工具缺失先：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`。

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_fill_line_command --json`)
date: 2026-08-30 17:26

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/rendercommand.rs` | 17/113 | 15.0% | 17/106 | 16.0% |
| **function total** | **17/113** | **15.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/rendercommand/RenderCommand.rs \
  --java koffice/presentation/core/cn/wps/show/moffice/drawing/render/GemoRender.java
```

Loop 结果：（未跑，且 02 为语义移植，模板校验重点放在决策函数对齐）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/moffice/drawing/render/GemoRender.java \
  --rs libs/ppt-core/src/cn/wps/show/render/rendercommand/RenderCommand.rs \
  --report libs/ppt-core/render/features/pr-02-shape-command/functions/02_function_fill_line_command/FUNCTION.md
```

<!-- java-rust-align:begin -->
| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
| N/A (`rendercommand.rs`; Canvas not ported) | PASS (`--rs` only / no Java twin) | command I/O in this function | none | none | PASS |
| **overall** | | | | | **PASS** |
<!-- java-rust-align:end -->