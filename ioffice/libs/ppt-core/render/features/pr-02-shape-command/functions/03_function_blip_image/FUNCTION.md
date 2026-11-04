# 03_function_blip_image — blipFill → DrawImage

> feature: pr-02-shape-command ｜ 状态：**pass** ｜ Java 管线环节：S4 `GemoRender.drawFill` 的 BlipFill 位图分支（指令化，无 Canvas）

## 0. 管线能力解锁

实现后：`p:pic` / 形状 `a:blipFill` / 母版 `bgPr` 的 **图片填充** 变成 `DrawImage{x,y,w,h,media}`，painter 可画母版背景图与内容图。缺失时 pr-02 只产实色 DrawRect（fillType==0），**图片被 FEATURE 明确延后** → Sample_ppt 第一页只剩白底+框+字。

本 function **不解析 zip 媒体流**（rId→bytes 归 pp-10:02 回调 + OPC/media）；只把「已有 mediaId + 几何 rect」编成指令。无 media 时产出占位 DrawRect（与 Java 缺图占位矩形对齐），**禁止**把 blip 画成实色冒充绿。

## 1. 功能说明

Java：`shape.getFill()` 为 `BlipFill` 且 `getFillType()==1`（graphics）时 `drawBitmap`。Rust 指令面新增：

```text
DrawImage { x, y, w, h, media_id: i32, r_id: Option<String> }
```

几何来自 01 已换算的绝对 rect（pic 的 xfrm 或背景整幅）。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | 01 的几何指令或 LayerItem 上的 fillType==2（DML blip）+ `mediaId`/`rId` fixture。 |
| **输出** | 恰好 1 条 `DrawImage`（有 media）；无 media → 1 条无 fill 的占位 `DrawRect`（stroke 可选），**不是**实色假图。 |
| **状态与副作用** | 不读磁盘。不改 Fill 模型。 |
| **错误语义** | fillType 非 blip → 本 function 不产出 DrawImage（交给 02 实色路径）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止去改 |
|---|---|---|---|---|---|
| 几何指令 | pr-02:01 | 绝对 rect | pass | — | 不重算 EMU |
| 实色 fill | pr-02:02 | fillType==0 | pass | — | 不把 blip 改成实色 |
| blip rId/mediaId | pp-10:02 | onBlipEmbed 写入 | spec | 位图解码 | 不在本 function 解 JPEG |
| Flutter painter | pr-04 | 画 DrawImage | 仅 DrawRect/Path/Text | Decode 图片 | 本 function 只扩指令枚举 |

## 2. 核心代码与函数改写列表

| Java 语义源 | Rust 目标 |
|---|---|
| `GemoRender.drawFill` BlipFill 分支 | `rendercommand.rs` 增 `DrawImage`；`apply_fill`/`geometry_commands` 旁路 |
| `ConvertTool.backgroudToPublicShape` 的 blip 背景 | Background 层 + blip → 整幅 DrawImage |

改写列表：

- `RenderCommand::DrawImage { … }` + serde tag。
- `fillType==2` → DrawImage；无 mediaId → 占位 DrawRect。
- `todo!("DEFERRED_NON_FUNCTION")`：tile/stretch/srcRect 裁剪、alpha、duotone。

## 3. 测试方案

### 3.1 代码审计与测试面

通读 `GemoRender.drawFill` 对 BlipFill 的调用（media id、dest rect）；`pr-02 FEATURE.md` 原「defer blip」条款改为本 function 解锁。Rust `RenderCommand` 现状无 DrawImage。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | blip + mediaId → 1×DrawImage 坐标=几何 rect | 主路径 | 是 | T01 |
| C02 | blip 无 mediaId → 占位 DrawRect 且 fill=None | 失败/占位 | 是 | T02 |
| C03 | fillType==0 实色 → 0 条 DrawImage | 负例 | 是 | T03 |
| C04 | Background 层 + blip → 整幅 DrawImage | 主路径 | 是 | T04 |

### 3.3 可测试性

fixture LayerItem + FillObj.fill_type=2 + media_id。不打开 pptx、不解码 JPEG。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 矩形 + media_id=7 | DrawImage 与 rect 一致 | 红测停在本 function |
| T02 | C02 | blip 无 id | DrawRect fill=None | 红测停在本 function |
| T03 | C03 | 实色 fill | 无 DrawImage | 红测停在本 function |
| T04 | C04 | White 后 Background+blip | 整幅 DrawImage | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/moffice/drawing/render/GemoRender.rs
       src/cn/wps/show/render/rendercommand.rs
test:  tests/cn/wps/show/moffice/drawing/render/GemoRender.rs
[[test]] name = "03_function_blip_image"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_blip_image

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_blip_image \
  --src src/cn/wps/show/moffice/drawing/render/GemoRender.rs \
  --src src/cn/wps/show/render/rendercommand.rs \
  --report libs/ppt-core/render/features/pr-02-shape-command/functions/03_function_blip_image/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_blip_image --json`)
date: 2026-09-01 15:03

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/moffice/drawing/render/GemoRender.rs` | 34/204 | 16.7% | 40/259 | 15.4% |
| `src/cn/wps/show/render/rendercommand.rs` | 32/114 | 28.1% | 29/107 | 27.1% |
| **function total** | **66/318** | **20.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .codex/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/moffice/drawing/render/GemoRender.rs \
  --java koffice/presentation/core/cn/wps/show/moffice/drawing/render/GemoRender.java
```

`rendercommand.rs` 是指令枚举（无 1:1 Java 类）；布局门闩对 `GemoRender` 交错模板。Layout FAIL 不得标 pass。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .codex/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/moffice/drawing/render/GemoRender.java \
  --rs libs/ppt-core/src/cn/wps/show/moffice/drawing/render/GemoRender.rs \
  --report libs/ppt-core/render/features/pr-02-shape-command/functions/03_function_blip_image/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 07:03

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `GemoRender` | `koffice/presentation/core/cn/wps/show/moffice/drawing/render/GemoRender.java` | `libs/ppt-core/src/cn/wps/show/moffice/drawing/render/GemoRender.rs` | **PASS** | `drawFill`, `drawFillPath`, `exceptionHandler` | `_render`, `_end`, `drawDiagram`, `drawInk`, `drawShape`, `getBaseDrawing`, `init`, `backupAndAdjustFillRect`, `isLineStyle`, `drawStroke`, `drawPicture`, `setClipBounds`, `drawNoClipPicture` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
