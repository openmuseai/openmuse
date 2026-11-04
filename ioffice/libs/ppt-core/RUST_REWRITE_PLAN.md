# PPT Rust 重写计划 — ppt-core

## 1. 概述

将 `koffice/presentation/core` (2201 Java 文件) 的演示文稿核心引擎移植到 Rust，产出 `libs/ppt-core` crate。

### 1.1 目标

- 100% 逻辑一致性：Rust 实现与 Java 原版行为完全相同
- 跨平台：Web (WASM) + Desktop + Mobile (via UniFFI)
- RenderCommand 架构：布局与渲染分离，输出平台无关的绘图指令

### 1.2 当前模块拆分


| Java 模块              | 文件数  | Rust 模块                                                          | 说明       |
| -------------------- | ---- | ---------------------------------------------------------------- | -------- |
| ppt-core-io          | 498  | `ppt-core::parse`                                                | 文件格式解析   |
| ppt-core-drawing     | 394  | `ppt-core::drawing` (future)                                     | 绘图引擎     |
| ppt-core-smartart    | 139  | `ppt-core::smartart` (future)                                    | SmartArt |
| ppt-core (remaining) | 1170 | `ppt-core::model` + `layout` + `cache` + `render_cmd` + `player` | 核心       |


---

## 2. 重写优先级与顺序

### Phase 1: 数据模型 + PPTX 解析（优先级 P0）

**目标**: 能够打开 PPTX 文件并构建内存模型


| 序号  | Java 源            | Rust 目标                 | 文件数  | 难度  | 预估工时  |
| --- | ----------------- | ----------------------- | ---- | --- | ----- |
| 1.1 | `show.data.`*     | `model/slide.rs`        | 69   | 中   | 2 周   |
| 1.2 | `show.model.*`    | `model/`                | 2    | 低   | 0.5 周 |
| 1.3 | `show.pptxr.*`    | `parse/pptx/`           | 143  | 高   | 4 周   |
| 1.4 | `show.app.*` (核心) | `model/presentation.rs` | ~100 | 中   | 3 周   |


**交付物**: `parse_pptx(bytes) -> Presentation`

**测试策略**: 对比 Java 和 Rust 解析同一 PPTX 后的 JSON 快照

### Phase 2: 绘图引擎（优先级 P0）

**目标**: 解析所有 DrawingML 和 DDF 绘图对象


| 序号  | Java 源               | Rust 目标            | 文件数  | 难度  | 预估工时 |
| --- | -------------------- | ------------------ | ---- | --- | ---- |
| 2.1 | `drawing.dml.`* (形状) | `drawing/dml/`     | ~200 | 高   | 5 周  |
| 2.2 | `drawing.dml.*` (文本) | `drawing/text/`    | ~50  | 中   | 2 周  |
| 2.3 | `drawing.dml.*` (效果) | `drawing/effects/` | ~30  | 中   | 2 周  |
| 2.4 | `drawing.ddf.*`      | `drawing/ddf/`     | ~100 | 高   | 3 周  |


**交付物**: `parse_shape(xml_node) -> Shape`

**注意**: 绘图引擎是 PPT 的核心，被 show 大量引用（1929 次），优先级应与 Phase 1 等同。

### Phase 3: 布局 + RenderCommand（优先级 P1）

**目标**: 生成幻灯片渲染指令


| 序号  | Java 源            | Rust 目标                  | 文件数 | 难度  | 预估工时 |
| --- | ----------------- | ------------------------ | --- | --- | ---- |
| 3.1 | `show.layout.`*   | `layout/`                | 9   | 中   | 1 周  |
| 3.2 | `show.render.*`   | `render_cmd/`            | 64  | 高   | 4 周  |
| 3.3 | `show.cache.*`    | `cache/`                 | 37  | 中   | 2 周  |
| 3.4 | `show.graphics.*` | `render_cmd/graphics.rs` | 47  | 中   | 2 周  |


**交付物**: `render_slide(slide, viewport) -> Vec<RenderCommand>`

### Phase 4: 放映播放器（优先级 P2）

**目标**: 幻灯片放映动画


| 序号  | Java 源               | Rust 目标                | 文件数 | 难度  | 预估工时 |
| --- | -------------------- | ---------------------- | --- | --- | ---- |
| 4.1 | `show.player.`* (切换) | `player/transition.rs` | ~40 | 高   | 3 周  |
| 4.2 | `show.player.*` (动画) | `player/animation.rs`  | ~60 | 极高  | 5 周  |
| 4.3 | `show.player.*` (控制) | `player/controller.rs` | ~40 | 中   | 2 周  |


**交付物**: `SlideShowPlayer` state machine

### Phase 5: SmartArt（优先级 P3）


| 序号  | Java 源              | Rust 目标            | 文件数 | 难度  | 预估工时 |
| --- | ------------------- | ------------------ | --- | --- | ---- |
| 5.1 | `smartart.model.`*  | `smartart/model/`  | ~40 | 中   | 2 周  |
| 5.2 | `smartart.layout.*` | `smartart/layout/` | ~60 | 高   | 3 周  |
| 5.3 | `smartart.io.*`     | `smartart/parse/`  | ~39 | 中   | 1 周  |


### Phase 6: 写出 + 其他格式（优先级 P4）


| 序号  | Java 源               | Rust 目标       | 文件数 | 难度  | 预估工时 |
| --- | -------------------- | ------------- | --- | --- | ---- |
| 6.1 | `show.pptxw.*`       | `write/pptx/` | 67  | 中   | 3 周  |
| 6.2 | `show.pptr/pptw.*`   | `parse/ppt/`  | 127 | 高   | 5 周  |
| 6.3 | `show.hslf.*`        | `parse/hslf/` | 98  | 高   | 4 周  |
| 6.4 | `show.htmlr/htmlw.*` | `parse/html/` | 36  | 中   | 2 周  |


---

## 3. 关键 Java→Rust 类映射


| Java 类                         | Rust 结构体                          | 说明          |
| ------------------------------ | --------------------------------- | ----------- |
| `show.data.PresentationData`   | `model::Presentation`             | 演示文稿        |
| `show.data.SlideData`          | `model::Slide`                    | 幻灯片         |
| `show.data.MasterData`         | `model::SlideMaster`              | 母版          |
| `show.data.LayoutData`         | `model::SlideLayout`              | 版式          |
| `show.pptxr.PptxReader`        | `parse::pptx::PptxParser`         | PPTX 解析器    |
| `drawing.dml.ShapeReader`      | `drawing::dml::ShapeParser`       | DML 形状      |
| `drawing.ddf.DdfReader`        | `drawing::ddf::DdfParser`         | DDF 解析      |
| `show.render.SlideRenderer`    | `render_cmd::SlideCommandBuilder` | 渲染指令        |
| `show.player.SlideShowPlayer`  | `player::SlideShowPlayer`         | 播放器         |
| `smartart.model.SmartArtModel` | `smartart::SmartArtModel`         | SmartArt 模型 |


---

## 4. 技术要点

### 4.1 依赖

- `quick-xml` — XML 解析（PPTX 核心）
- `zip` — PPTX 包解压
- `serde` — 序列化/反序列化
- EMU (English Metric Units) → 点/像素转换需要精确

### 4.2 注意事项

- PPT 形状几何：预设形状（>200 种）+ 自定义路径，需要精确的几何计算
- 动画引擎最复杂：涉及时间线、缓动函数、多属性并行动画
- 文本排版：段落/运行级别的富文本，需要字体度量
- 母版继承链：slide → slideLayout → slideMaster → theme，4 级查找

### 4.3 与 word-core 共享

- `drawing` 模块可与 `word-core` 共享 DML 解析代码
- `render_cmd` 的 `RenderCommand` 类型可统一

---

## 5. 总体工时估算


| Phase  | 描述           | 文件数      | 预估工时        |
| ------ | ------------ | -------- | ----------- |
| P0     | 模型 + PPTX 解析 | 314      | 9.5 周       |
| P0     | 绘图引擎         | 394      | 12 周        |
| P1     | 布局 + 渲染指令    | 157      | 9 周         |
| P2     | 放映播放器        | 140      | 10 周        |
| P3     | SmartArt     | 139      | 6 周         |
| P4     | 写出 + 其他格式    | 328      | 14 周        |
| **合计** |              | **1472** | **~60.5 周** |


---

## 文档日期

- **编写日期**: 2026-04-30
- **状态**: 等待确认后执行

