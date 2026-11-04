# pr-01-slide-layers — 三层胶片顺序与开始过滤

> 层：render ｜ 状态：**pass** ｜ Java：`SlideRender.draw/drawUnderSlide/drawShapesOnLayout` + `SlideRenderTool.drawFilm` + `ShapeRender._begin`

## 目标

把 S3 遍历钉死为**可测的有序输出**：`(KmoSlide + slideDrawFlags/followerDrawFlags) → 有序层列表`（白底尺寸 → slide 背景 → master film（条件）→ layout film（条件）→ slide film），并对 film 中每个形状执行 `ShapeRender._begin` 过滤（hidden / `drawFlags&1` 跳过 placeholder / `&32` 跳过 miner ph 4/5/6/7 / modular group 默认图条件）。

本 feature 只保证「顺序」与「画不画」两条语义，**不产出像素指令**（pr-02 才把层元素转成 DrawRect/DrawPath）。层序错了，母版/版式形状会盖错层（`MVP.md` 明确 `SlideRender.drawUnderSlide` 的层序必须钉死）。

## I/O（锁死）

| function | I | O | 契约 |
|---|---|---|---|
| 01 layer_order | `KmoSlide`（含 `layout()`/`master()`）、`slideDrawFlags`、`followerDrawFlags`、`UnitsConverter.render()` | `Vec<LayerItem>`：`WhiteBackground{w,h}` → `Background`（条件）→ `Shape[](master film)` → `Shape[](layout film)` → `Shape[](slide film)` | 白底尺寸 = `UnitsConverter.render().fromEmuX/Y(slide.getPresentation().slideWidth/Height)`；master film 条件 `layout != null && layout.isShowMasterShape() && slide.isShowMasterShape()`；layout film 条件 `slide.isShowMasterShape()`；背景条件 `slide.background() != null && (drawFlags&8)==0` |
| 02 begin_filter | `LayeredTypoShape`（`isHidden`/`isPlaceholder`/`placeholderType`）+ drawFlags | `Pass` / `Skip` | `isHidden()`→Skip；`drawFlags&1!=0 && isPlaceholder()`→Skip；`drawFlags&32!=0 && isPlaceholder() && placeholderType∈{4,5,6,7}`→Skip；`!(env.isDrawModularDefaultPic() || !isModularGroupDefaultPic())`→Skip |

错误语义：本 feature 无 Java 异常路径（`drawSlide` 对 `mSlide==null` 直接返回，Rust 侧对 `None` 返回空列表；`SlideRender.draw` 对 slide==null 直接 return）。

## Java 管线对照

| Java 类/方法 | 语义 | Rust 归属 |
|---|---|---|
| `SlideRender.getOrAllocRead/Thumb/Edit/Master/Back/Print`（含 `createPrintSlideRender`） | 静态工厂设定场景常量：read=21505154/follower=532609；thumb=29909122/548993；edit=15474692/794753；master=180/129；back=450/385；print=533122/533121 | `DrawFlags` 常量 + 场景构造（MVP 只锁 read/thumb 两个） |
| `SlideRenderBase`（`slideDrawFlags`/`followerDrawFlags`/`setDrawFlags`） | 状态与 setter | `SlideRender` 基态 |
| `SlideRenderContext.reset/clear/isPartChange` | `reset(slide,drawFlags)`；`isPartChange` = `mChangeIndex!=-1 && (mSlide==null \|\| mSlide.type()==0)` | `SlideRenderContext` |
| `SlideRender.draw` | `!isPartChange()` → `drawUnderSlide`；`mSlideContext.reset(slide,slideDrawFlags)` → `renderTool.drawSlide` | 01 编排 |
| `SlideRender.drawUnderSlide` | 白底尺寸 → `drawSlideWhiteBackground` → `reset` → `drawBackgroundShape` → `drawShapesOnLayout` | 01 |
| `SlideRender.drawShapesOnLayout` | master film（双条件）→ layout film（`slide.isShowMasterShape()`） | 01 |
| `SlideRenderTool.drawSlide/drawFilm` | film 遍历 `spTree.shapeAt(i)`；`start = isPartChange ? mChangeIndex : 0`；type==4 → `CreateGraphicFrameRender` 分支，否则 `ShapeRender` | 01（type==4 分支 = 壳） |
| `SlideRenderTool.drawSlideWhiteBackground` | `canvas.drawRect(0,0,w,h, WHITE_PAINT)`（白色 FILL） | 01 输出 `WhiteBackground` 层 |
| `SlideRenderTool.drawBackgroundShape` | `slide.background()` 非空 → `ShowTypoBackground` + `BackgroundShapeRender.render` | 01 输出 `Background` 层（fill 语义在 pr-02-02） |
| `BackgroundShapeRender._begin` | `hasBackground()` 为假 → false；`drawFlags&8 != 0`（`NOT_DRAW_BACKGROUND_SHAPE`）→ false | 01 背景层条件 |
| `ShapeRender._begin` | 四条过滤（hidden / &1 / &32 / modular Group 默认图） | 02 |

Rust 根路径：`libs/ppt-core/src/cn/wps/show/render/`。

## 能力解锁

- `01_function_layer_order`：三层胶片顺序钉死 → 母版/版式不再盖错层；白底尺寸进入层列表 → 画布内容区确定（主文档 pr-01 `layer_order` 行）。
- `02_function_begin_filter`：过滤规则对齐 → 只画该画的形状；隐藏/ph/miner ph 不误上屏（主文档 pr-01 `begin_filter` 行）。

缺失断点：层序错/过滤错 → 画面错误（不断，但不一致）——本 feature 是 S3 的**一致性门闩**。

## 必须 / 壳 / 延后

- **must：** 01 的 5 段层序 + 两条 master/layout 条件 + 白底尺寸换算；02 的 4 条 `_begin` 过滤（hidden、&1、&32 + miner type 4/5/6/7、modular 默认图条件）；read=21505154/follower=532609 与 thumb=29909122/548993 两组场景常量；`SlideRender.draw` 的 `isPartChange` 分支。
- **壳：** `type()==4` graphicFrame 分支（graphic 为 null 时 → 仍按默认 `CreateShapeRender` 形状分支；Chart/Table/SmartArt/OLE/Model3D 业务体 post-MVP，只保 type==4 走 graphicFrame 的顺序语义）；`mChangeIndex` 部分重绘（isPartChange 判定可测，差量绘 dati后）。
- **defer：** edit/master/back/print 场景常量与装饰（dash 框、布局边框、图片 ph 图标）、`LayoutRender` 版式预览 flag、动画 InterpretCtx 滤镜（`USE_DRAW_FILTER`）、`SlideRenderEventListeners` 渲染事件、media delay。

## 范围

- **in：** 层序与顺序条件；白底/背景层的层元素产出；`_begin` 过滤决策；read/thumb 场景常量；背景跳过（`&8`）。
- **out：** 任何像素指令（pr-02）；背景 fill 的具体样式解析（pr-02-02）；ph 继承合并（pl-03）；装饰绘制（dash 框/布局边框/图标，edit 场景）；`renderTool.setFilterFlag` 的滤镜语义（动画，post-MVP）。

## 验收

FUNCTIONS.md §4：

1. `drawUnderSlide` 顺序：白底尺寸 → 背景 → master film（双条件）→ layout film → slide film（Recording fixture 断言顺序与条件组合）。
2. read/thumb 两组场景常量与位含义（`DrawFlags`：1=NOT_DRAW_PLACEHOLDER、8=NOT_DRAW_BACKGROUND_SHAPE、32=NOT_DRAW_PLACEHOLDER_MINOR）。
3. `_begin` 四条过滤各自独立可测（hidden 形状 / ph 形状 / miner ph 类型 4/5/6/7 / modular 默认图）。
4. 背景：`slide.background()==null` 或无 `&8` 时背景层跳过/输出正确。

**建议 MVP pass 口径（loop 采用）：** 01 必测 = 5 段层序 + master/layout 两条条件；02 必测 = hidden/&1/&32 三条；modular 默认图与 type==4 分支可 `#[ignore = "spec: post-MVP"]`。

## 依赖

- **depends：** pl-06（`LayeredTypoShape` 快照：isHidden/isPlaceholder/placeholderType/getShapeRect）；pl-01（`UnitsConverter.render().fromEmuX/Y` 与 slide 视口）；pl-02（绝对 EMU rect → render px 的 `getShapeRect`）。
- **does not depend：** pp-10/11（fixture 可先行）；pr-02/03/04；编辑态选择/动画。

## 不过扩展

禁止把 Canvas 装饰语义（dash frame、布局边框、图片 ph 图标）搬进本 feature 输出——装饰是 edit 场景 post-MVP。禁止实现 Chart/Table/SmartArt/OLE/Model3D 业务体（type==4 只保顺序语义）。禁止改动 pl-06 的 `LayeredTypoShape` 字段以迁就 pr-01。