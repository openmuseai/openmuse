# PPT 骨架管线：布局正确 + 基础元素

> 日期：2026-09-10（loop **pass**；`samples/ppt-render` Sample_ppt 端到端已跑）  
> 对照：[VISUAL_CONTENT_PIPELINE.md](./VISUAL_CONTENT_PIPELINE.md) 叶 pass ≠ Sample 骨架对齐。产品桥：`pptx_mvp.rs` + `page_inheritance.rs` + `mvp_pipeline::render_slide` + Flutter `RenderPainter`。

## 0. 这一波只盯两件事

1. **布局正确**：组内子级落到 slide 绝对 EMU；`layer_order` 真的 film 到子 shape；负坐标不画进宿主侧栏。  
2. **基础元素能画**：实色矩形、线、椭圆、五边形、`p:sp` 上的 JPEG/PNG、页底实色；纯文本框不再变成空 AABB 黑线框。

已 pass 的 pp-14/15/16、pl-07、pr-05 **不重拆**。本波切在产品 I/O。

### 0.1 产品桥上已查证的断点（写方案所依，不是标题）

| 断点 | 代码 | 画面 |
|---|---|---|
| `element_id` 无 `chOff`/`chExt` | `pptx_mvp.rs` 只映射 `off=1114200`、`ext=1114197`；Java `GroupTransform2DHandler` 为 `chOff=1114204`、`chExt=1114205` | 组局部坐标当幻灯片绝对坐标 |
| `parse_one_grp` 走 `ShapePropertiesHandler` | 不用 `GroupShapePropertiesHandler`；即便 handler 接到 chOff，`parse_one_sp` 只拷 `xfrm.x/y/cx/cy`，不写 `child_off`/`child_ext` | `ShapeUtil::calcChildRelativeRect` 因 `hasChildPoint()==false` 整段跳过 |
| `resolve_pages` 算了 `abs` 不写回 | `page_inheritance.rs`：`calcRelativeRect` 只推进 `ResolvedShape.abs`；`apply_xfrm` 写的是 placeholder merge，不是子绝对框 | `film_shapes` 仍读 `transform2D*` |
| `film_shapes` 只扫顶层 | `SlideRender.rs`：`shapeCount()` = `sp_tree.children.len()`，**不递归** `grpSp.children` | 组内色块/五边形根本不入层，或只画组框 |
| `PresetGeometryHandler` 改 clone | `ShapePropertiesHandler` `enterSubElement(1114429)` 传入 `geom_obj.clone()`；`commit()` 只写 `xfrm`/`fill_obj` | `geom_obj.prst` 丢失；`DGMap` 产品表只有 rect/roundRect/ellipse |
| `geometry_commands` | 无 `graphics_path` → 空 fill/stroke 的 `DrawRect` | 线/圆/五边形变套叠方框 |
| `has_background: false` 写死 | `open_pptx_document_bytes` | `drawUnderSlide` 永不入 `Background` |
| `p:sp` 上 `blipFill` | walker 能进 `fill_obj.blip_embed`，`render_slide` 已对 `media_bytes` emit DrawImage；**产品 parse 需把 bytes 挂上非 pic** | 第 7 页城市图是 `p:sp`+`blipFill`，不是 `p:pic` |
| Flutter `_paintPair` | fill 与 stroke 皆空 → 默认黑描边 | 每个文本框都是线框 |

Sample：`slide2.xml` 有 `p:bg`、7 个 `grpSp`（chOff≠off）、`vert="eaVert"`、`image1.jpg`；`slide7.xml` 无 `p:bg`、全幅 `blipFill` 城市图、`pentagon`/`ellipse`/`line`。

## 1. Feature 清单（I/O 切，不按类切）

```text
.pptx
  → [pl-08] 组 chOff/chExt → 子 slide 绝对框 → film 子级     ★ 布局
  → [pr-06] prst 落盘 → 线/椭圆/多边形 Path；文本不发射空框  ★ 基础形状
  → [pp-18] p:sp blipFill → media_bytes → DrawImage           ★ 基础图
  → [pp-17] p:bg / bgRef → has_background → Background 层     ★ 基础底
  → [pr-07] clip 到 slide；bodyPr vert；painter 不默认黑描边  ★ 布局收口 + 宿主
```

`pp-17` 与 `pl-08` 无数据依赖，可并行。`pp-18`/`pr-06` 吃绝对框。`pr-07` 吃指令列表。

| order | feature_id | 层 | I | O | 不放进本层 |
|---|---|---|---|---|---|
| 1 | [pl-08-group-abs-geom](layout/features/pl-08-group-abs-geom/FEATURE.md) | layout | grpSpPr xfrm（off/ext/**chOff/chExt**）+ 子树 | 子 `transform2D*` 为 slide 绝对 EMU；`layer_order` 含组内 shape | 3D、任意旋转矩阵；不复制 `ShapeUtil` 算法 |
| 2 | [pr-06-prstgeom-skeleton](render/features/pr-06-prstgeom-skeleton/FEATURE.md) | parse+render | `prstGeom@prst` + 绝对框 | 线/椭圆/多边形 `DrawPath`；纯文本无 fill → 0 条几何 | custGeom 全路径、图标 SVG、按「形状种类」再拆 crate |
| 3 | [pp-18-sp-picture-fill](parser/features/pp-18-sp-picture-fill/FEATURE.md) | parser+render | `p:sp`/`p:pic` 的 `a:blip r:embed` + rels + zip | `media_bytes`；`render_slide` DrawImage（不限 type==2） | SVG/WDP 解码、srcRect 裁切 |
| 4 | [pp-17-slide-background](parser/features/pp-17-slide-background/FEATURE.md) | parser | `p:cSld/p:bg`（bgPr 实色 / bgRef）+ theme | `has_background` + 底 ARGB + `LayerItem::Background` | 渐变底、bg 上的图（图走 pp-18） |
| 5 | [pr-07-clip-host-paint](render/features/pr-07-clip-host-paint/FEATURE.md) | render+Flutter | 指令 + `bodyPr vert` + slide 尺寸 | clip；竖排字；空 fill **不**默认黑描边 | Zoom 控件、阴影、半透明像素级 |

## 2. 与已 pass 叶

| 已 pass | 本波怎么用 |
|---|---|
| pl-02 `calcChildRelativeRect` | pl-08 **只调用**；Java 结果在组局部渲染空间，产品指令无 canvas translate → writeback 时 **加上组绝对 origin** 得到 slide 绝对 EMU |
| pl-07 `mergeShapePr` / `resolve_pages` parent 链 | 组子级仍走 `resolve_pages`；补 chOff + abs 写回 + film 递归 |
| pr-02 `DrawPath` / pr-05 fill+DrawImage | pr-06 填 path；pp-18 让 `p:sp` 走进现有 emit |
| pp-15/16 | 同一套 rels/zip 与 scheme 表；禁止第二套 media/颜色句柄 |
| SlideRender Background | pp-17 把 `background()` 变成 Some |

## 3. 验收（Sample_ppt）

1. **布局（第 2 页）**：组内蓝/黄/橙块 AABB 在 slide 内（允许 1px）；不再出现大块负 x 实心矩形当装饰（CONTENTS 负 x 归 pr-07 clip）。`layer_order` 的 Shape 层数 ≥ 组内可见子级数。  
2. **基础形状（第 7 页）**：五边形是多边形 `DrawPath`（≥5 点），不是层层方框；椭圆/线是 path。  
3. **基础图（第 7 页）**：全幅城市 JPEG 为 `DrawImage`（或明确 media miss 矩形）。  
4. **基础底（第 2 页）**：`has_background==true`；第 7 页无 `p:bg` 保持 false。  
5. **线框**：无 fill 文本框不因 painter 默认黑描边铺满方框。页数仍 23。

## 4. 明确不做

SVG/WDP 解码；Chart/Table/SmartArt；渐变/阴影/`alphaModFix`/`srcRect`；Java `RichTextLayout` 像素断行；`--force` 重生 `PptxrSlide.rs` / `PptxrSlides.rs`。

## 5. loop 序（布局 → 形状 → 图 → 底 → 宿主）

```text
pl-08:01 → 02 → 03
pr-06:01 → 02 → 03
pp-18:01 → 02
pp-17:01 → 02 → 03
pr-07:01 → 02 → 03
```

有 Java 叶走 **function-java2rust**；产品桥无一对一 Java 类时与 pp-13:02 相同（行为门禁 + N/A 布局）。`#[ignore]` 的 spec **不得**标 `pass`。
