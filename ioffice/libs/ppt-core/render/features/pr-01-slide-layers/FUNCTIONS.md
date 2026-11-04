# pr-01-slide-layers — Function 组合图

> depends_on: `pl-06` typo_shape_snapshot；`pl-01` units-view  
> 状态：**pass**  
> 输出是**有序层列表**（白底尺寸 → 背景 → 三层胶片），不是像素指令；指令在 pr-02 才产生。

## §0 管线位置

S3 遍历（见 [PIPELINE_LAYOUT_RENDER_PLAN.md](../../../PIPELINE_LAYOUT_RENDER_PLAN.md) 热路径 S3 段）。上游 pl-06 给 `LayeredTypoShape`；下游 pr-02 消费层列表产出 DrawRect/DrawPath。本 feature 禁止触碰 Canvas。

## §1 function 清单

| order | function_id | I | O | upstream | downstream |
|---|---|---|---|---|---|
| 1 | `01_function_layer_order` | slide+layout+master+slideDrawFlags/followerDrawFlags | `Vec<LayerItem>` = WhiteBackground{w,h} → Background（条件）→ master film → layout film → slide film | pl-06 | 02 |
| 2 | `02_function_begin_filter` | LayerItem::Shape（LayeredTypoShape）+ drawFlags | Pass/Skip（hidden / &1 ph / &32 miner 4/5/6/7） | 01 | pr-02-shape-command |

顺序钉死：白底尺寸 → slide 背景 → master film（条件）→ layout film（条件）→ slide film。

## §2 数据流

```text
draw(canvas, slide)         Java: SlideRender.draw
  !isPartChange → drawUnderSlide
    width  = UnitsConverter.render().fromEmuX(slide.getPresentation().slideWidth())
    height = UnitsConverter.render().fromEmuY(slide.getPresentation().slideHeight())
    → WhiteBackground{w, h}
    → reset(slide, slideDrawFlags)
    → drawBackgroundShape：slide.background()!=null && (drawFlags&8)==0 → Background 层
    → drawShapesOnLayout
        layout!=null && layout.isShowMasterShape() && slide.isShowMasterShape() → master film（followerDrawFlags）
        slide.isShowMasterShape() → layout film（followerDrawFlags）
  → reset(slide, slideDrawFlags) → drawFilm(slide, slideDrawFlags)
```

场景常量（`DrawFlags` 位与取值全部查证）：read slide=21505154 / follower=532609；thumb slide=29909122 / follower=548993；edit=15474692/794753；master=180/129；back=450/385；print=533122/533121。相关位：`1`=NOT_DRAW_PLACEHOLDER、`8`=NOT_DRAW_BACKGROUND_SHAPE、`32`=NOT_DRAW_PLACEHOLDER_MINOR。

## §3 测试面

- fixture：手工 `KmoShowShape`/`LayeredTypoShape`（含 hidden/ph/miner 各一条）+ Recording 顺序断言器（记录 `WhiteBackground→Background→film...` 事件序）。
- 不跑真实 `PptxReader`（pl-06 快照可 fixture）；不依赖 pp-10/11。

## §4 验收

1. 5 段层序 + 两条 master/layout 条件与 Java `drawUnderSlide`/`drawShapesOnLayout` 逐一对应。
2. read/thumb 场景常量锁定（21505154/29909122 与 follower 532609/548993）。
3. 02 四条过滤：hidden→Skip、&1+ph→Skip、&32+miner4/5/6/7→Skip、其余 Pass。
4. 背景层：`background()==null` 或 `&8` 置位 → 不输出 Background 层。

## §5 loop

`01 → 02`（02 用 01 的流水做过滤决策；01 可用 pl-06 fixture 独立测）。pr-01 → pr-02 → pr-03 → pr-04。

## §6 Java/Rust 范围

| Java 类 | Rust 目标 | 裁决 |
|---|---|---|
| `render/SlideRender.java` | `src/cn/wps/show/render/SlideRender.rs`（新建） | `draw`/`drawUnderSlide`/`drawShapesOnLayout` 必写；getOrAllocXxx 场景工厂只锁 read/thumb 两组常量，其余场景 `#[ignore = "spec: post-MVP"]` |
| `render/SlideRenderBase.java` | `src/cn/wps/show/render/SlideRenderBase.rs`（新建） | 字段 + `reset`/`setDrawFlags` 必写 |
| `render/SlideRenderContext.java` | `src/cn/wps/show/render/SlideRenderContext.rs`（新建） | `reset`/`clear`/`isPartChange` 必写 |
| `render/util/SlideRenderTool.java` | `src/cn/wps/show/render/SlideRenderTool.rs`（新建） | `drawSlide`/`drawFilm`/`drawSlideWhiteBackground`/`drawBackgroundShape` 语义必写；`setFilterFlag` 滤镜语义停车 |
| `render/ShapeRender.java`（`_begin` 部分） | `src/cn/wps/show/render/ShapeRender.rs`（新建，含 `_begin` 过滤） | 四条过滤必写；`_render` 与装饰方法留 pr-02-01 边界 |
| `show/util/DrawFlags.java` | `src/cn/wps/show/util/DrawFlags.rs`（新建，仅常量 + 位测） | read/thumb + 三个位常量必写；其余位待 loop 确认 |
| `render/util/ShowShapeRenderFactory.java` | —（语义源） | type==4 → graphicFrame 分支只保顺序语义（壳），不整文件移植 |

## 文档

- [functions.csv](./functions.csv)