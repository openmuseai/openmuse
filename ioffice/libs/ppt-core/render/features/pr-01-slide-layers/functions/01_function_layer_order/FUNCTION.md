# 01_function_layer_order — 三层胶片层序

> feature: pr-01-slide-layers ｜ 状态：**pass** ｜ Java 管线环节：S3

## 0. 管线能力解锁

实现后：一页 slide 的绘制内容变成**有序层列表**（白底尺寸 → 背景 → master film → layout film → slide film），母版/版式形状按 Java `SlideRender.drawUnderSlide` 的层序落到固定位置——不再盖错层。没有它，管线断在 S3：形状集存在但顺序未钉死，pr-02 画出的层是乱序的（主文档断点表「pr-01（层序）→ 画面错误（不断，但不一致）」）。

## 1. 功能说明

`(KmoSlide, slideDrawFlags, followerDrawFlags) → Vec<LayerItem>`；LayerItem = `WhiteBackground{w,h}`｜`Background`｜`Shape(LayeredTypoShape)`。顺序与条件完全对齐 Java，输出与像素无关。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `KmoSlide`（`slide.getPresentation().slideWidth()/slideHeight()`、`slide.background()`、`slide.layout()`、`slide.isShowMasterShape()`、`layout.isShowMasterShape()`、`layout.master()`）+ `slideDrawFlags`/`followerDrawFlags`。 |
| **输出** | `Vec<LayerItem>`：`WhiteBackground{w=fromEmuX(slideWidth), h=fromEmuY(slideHeight)}`（`SlideRenderTool.drawSlideWhiteBackground` 语义）→ `Background`（`drawBackgroundShape` 语义，`slide.background()!=null && (drawFlags&8)==0` 才输出）→ `Shape[](master film)`（条件 `layout!=null && layout.isShowMasterShape() && slide.isShowMasterShape()`，走 followerDrawFlags）→ `Shape[](layout film)`（条件 `slide.isShowMasterShape()`，走 followerDrawFlags）→ `Shape[](slide film)`（slideDrawFlags）。 |
| **状态与副作用** | film 遍历起止：`start = isPartChange() ? mChangeIndex : 0`（`SlideRenderContext`，快照差量重绘，MVP 固定 start=0）；film 生成 `ShapeRender` 语义的 `LayeredTypoShape` 阵（02 过滤后进入 pr-02）。 |
| **错误语义** | Java 无异常路径：`SlideRender.draw` 对 slide==null return；`SlideRenderTool.drawSlide` 对 mSlide==null return。Rust 侧 `Option`/空 Vec 等价，不许 panic。 |

### 1.2-1.6 已并入 1.1 与 §0（本功能契约集中，无额外接口）

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| LayeredTypoShape 快照 | pl-06 | isHidden/getShapeRect/ph 信息 | spec 未接线 | 先 fixture | 不改 pl-06 字段 |
| UnitsConverter.render | pl-01 | fromEmuX/fromEmuY | keep（`base/converter/UnitsConverter.rs` 486 行，render() 已在） | — | 不改换算系数 |
| KmoSlide 模型 | parser/pl | slideWidth/Height、layout、background | 部分 | 结构以 pl 为准 | — |
| DrawFlags 常量 | 本 feature/02 侧 | 21505154/29909122 等 | 无（`show/util/DrawFlags.rs` 不存在） | loop 确认位全集 | 不内联散落常量 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/render/SlideRender.java`（90 行）+ `SlideRenderBase.java` + `SlideRenderContext.java` + `render/util/SlideRenderTool.java` |
| Rust | `libs/ppt-core/src/cn/wps/show/render/{SlideRender,SlideRenderBase,SlideRenderContext,SlideRenderTool}.rs`（均尚不存在，loop 用模板生成） |

关键方法/字段（对照 Java，均已查证）：

| symbol | 语义（Java） | Rust 目标 |
|---|---|---|
| 字段 `slideDrawFlags`/`followerDrawFlags` | SlideRenderBase 受保护字段 + `setDrawFlags(int,int)` | 结构字段 + setter |
| `getOrAllocReadSlideRender` / `getOrAllocThumbSlideRender` | 场景工厂：read=21505154/532609；thumb=29909122/548993 | 两个场景构造（其余 edit/master/back/print 常量随表，先不落地） |
| `draw(Canvas, KmoSlide)` | `!mSlideContext.isPartChange()` → drawUnderSlide；`reset(slide, slideDrawFlags)` → `renderTool.drawSlide` | `draw(slide, ...)` 无 Canvas |
| `drawUnderSlide(Canvas, KmoSlide)` | 白底尺寸 → `setRenderContent` → `drawSlideWhiteBackground` → `reset` → `drawBackgroundShape` → `drawShapesOnLayout` | 5 段层序生成 |
| `drawShapesOnLayout(Canvas, fn)` | master film（双条件，用 layout.master()）；layout film（`slide.isShowMasterShape()`） | 两条条件分支 |
| `SlideRenderContext.reset/clear/isPartChange` | `reset(slide,drawFlags)`；isPartChange = `mChangeIndex != -1 && (mSlide == null \|\| mSlide.type()==0)` | 同 Semantics |
| `SlideRenderTool.drawSlide/drawFilm` | film 遍历 `spTree.shapeAt(i)` 生成 `CreateShowShapeRender`（type==4 → graphicFrame 分支） | 层元素生成；type==4 分支保留顺序语义壳 |
| `drawBackgroundShape` | `bg = mSlide.background()`；null 跳过；`BackgroundShapeRender._begin` 再查 `&8` | 背景层条件（&8 语义在 02/01 边界以 Java 为准） |

改写列表：`SlideRender`（draw/drawUnderSlide/drawShapesOnLayout）、`SlideRenderBase`（字段+reset+setDrawFlags）、`SlideRenderContext`（reset/clear/isPartChange）、`SlideRenderTool`（drawSlide/drawFilm/drawSlideWhiteBackground/drawBackgroundShape）。

停车列表：`SlideRender.getOrAllocEdit/Master/Back/Print` 场景装饰矩阵、`renderTool.setFilterFlag`（USE_DRAW_FILTER）、`SlideRenderEventListeners` 事件链、type==4 的 Chart/Table/SmartArt/OLE/Model3D 业务体。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读 `SlideRender.java` 全量 + `SlideRenderTool.drawFilm` 循环体（重点：`start = isPartChange() ? mChangeIndex : 0` 的起点语义、`type()==4` 分支）。当前 Rust 无对应文件，loop 前以 Java 为准。审计表：

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| draw | slide null | 直接 return | null |
| drawUnderSlide | slide+flags | WhiteBackground 尺寸正确 | fromEmu 系数 |
| drawShapesOnLayout | layout/master 存在性 + 两个 isShowMasterShape | master 先于 layout | 4 种条件组合 |
| drawFilm | shapeTree | 层元素序列 | type==4 分支 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| L01 | 5 段层序（白底→背景→slide film） | 顺序 | 是 | T01 |
| L02 | master 胶片双条件（layout!=null && layout.isShowMasterShape && slide.isShowMasterShape） | 条件 | 是 | T02 |
| L03 | layout 胶片条件（slide.isShowMasterShape） | 条件 | 是 | T03 |
| L04 | 白底尺寸 = fromEmuX/Y(slideWidth/Height) | 换算 | 是 | T04 |
| L05 | 背景层：background()==null 跳过；&8 置位跳过 | 条件 | 是 | T05 |
| L06 | read/thumb 两组场景常量值 | 常量 | 是 | T06 |
| L07 | type==4 graphicFrame 分支 | 壳 | 否 | `#[ignore = "spec: post-MVP"]` |

### 3.3 可测试性

fixture `KmoSlide`（含 layout/master/background 三种组合）+ Recording 事件列表断言层序；不跑 `PptxReader`、不依赖 pp-10/11。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | L01 | 最小 fixture（无 bg，无 layout 显示） | WhiteBackground→Shape[](slide) 顺序 | 红测停在本 function |
| T02 | L02 | layout.isShowMasterShape=false 变体 | 不输出 master film | 红测停在本 function |
| T03 | L03 | slide.isShowMasterShape=false 变体 | 不输出 layout film | 红测停在本 function |
| T04 | L04 | slideWidth=12192000 | WhiteBackground.w = fromEmuX(12192000) | 红测停在本 function |
| T05 | L05 | bg==null / flags&8 | 跳过背景层 | 红测停在本 function |
| T06 | L06 | 工厂 | read=21505154；thumb=29909122；follower 532609/548993 | 红测停在本 function |
| T07 | L07 | type4+graphic 幻影 | 壳：顺序不崩（影象为 null → 走默认形状分支） | ignore |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/render/SlideRender.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "01_function_layer_order"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_layer_order

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_layer_order \
  --src src/cn/wps/show/render/SlideRender.rs \
  --report libs/ppt-core/render/features/pr-01-slide-layers/functions/01_function_layer_order/FUNCTION.md
```

工具缺失先：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`。

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_layer_order --json`)
date: 2026-08-30 17:26

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/SlideRender.rs` | 77/109 | 70.6% | 107/142 | 75.4% |
| **function total** | **77/109** | **70.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/SlideRender.rs \
  --java koffice/presentation/core/cn/wps/show/render/SlideRender.java
```

Loop 结果：**PASS**

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/SlideRender.java \
  --rs libs/ppt-core/src/cn/wps/show/render/SlideRender.rs \
  --report libs/ppt-core/render/features/pr-01-slide-layers/functions/01_function_layer_order/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-30 09:26

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SlideRender` | `koffice/presentation/core/cn/wps/show/render/SlideRender.java` | `libs/ppt-core/src/cn/wps/show/render/SlideRender.rs` | **PASS** | `getOrAllocThumbSlideRender`, `getOrAllocReadSlideRender`, `draw`, `drawUnderSlide`, `drawShapesOnLayout` | `reuseThumbSlideRender`, `reuseReadSlideRender`, `getOrAllocEditSlideRender`, `reuseEditSlideRender`, `getOrAllocMasterSlideRender`, `reuseMasterSlideRender`, `getOrAllocBackSlideRender`, `reuseBackSlideRender`, `createPrintSlideRender` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->