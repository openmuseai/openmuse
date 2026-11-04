# PPT 下一波管线：视觉内容（框 → 色/字/图）

> 日期：2026-09-02  
> 模式：feature-function **Split**（本文件 + 下列 feature 目录）。未点名 loop，不改 src。  
> 对照：已绿的叶级 `functions.csv` **不等于** Sample 已画全。产品桥仍是 `pptx_mvp.rs` + `mvp_pipeline::render_slide`。

## 0. 现状（为什么只能看到页面框架）

`samples/ppt-render` 已能打开 `Sample_ppt.pptx`（23 页）。CLI 对 slide1 能吐出若干 `DrawRect`/`DrawText`，但产品语义仍是 **骨架**：

| 已绿（叶/编排） | 产品桥实际做了什么 | 画面后果 |
|---|---|---|
| pp-13 页序 + 每页 `p:sp` 树 | 只读 **slide XML**，不读 layout/master | `KmoSlide.layout == None`；`resolve_pages` 的 ph 查找空转 |
| pl-03 `mergeShapePr` | `page_inheritance.rs` 把返回值写成 `let _style = …` **丢弃** | 版式/母版 fill 永不落到 `sp_pr` |
| pp-02:03 `p:pic` 叶测 | 产品用 `parse_one_sp`，只抽 `spPr`/`txBody` | `p:blipFill` 是 pic 的 **兄弟节点**，从未解析；zip `ppt/media/*` 未绑定 |
| pp-10 fill / pp-03 scheme / pp-04 theme | 叶测内存 fixture | Sample 大量 `a:schemeClr`；无 theme 表 → `hasFill==false` → 只剩无 fill 的框线 |
| pp-11 `a:t` | 产品 `TextBodySurf` 不读 `rPr/solidFill`；`<a:br>` 当 run 推空串 | 有字也常是空 `DrawText` 或黑默认色 |
| pr-02:03 `DrawImage` 叶测 | `render_slide` 从不 emit；Flutter `RenderPainter` skip 未知 type | 图片完全不上屏 |

Sample_ppt 实证：`slide1.xml` 有 1 个 `p:pic`、12 个 `txBody`、11 个 `schemeClr`、5 个 `p:ph`；rels 指向 `slideLayout12` + `../media/image1.jpg`。不接下一段 I/O，画面只能是白底 + 空框。

**禁止**把本波做成「再 loop 一遍已 pass 的叶」：叶测试用 contract fixture，进不了 `open_pptx_document_bytes`。本波全部切在 **产品 I/O**（zip part / rels / 指令 / painter）。

## 1. 下一波 feature 清单（I/O 切，不按类切）

```text
.pptx zip
  → [pp-14] slide.rels → layout XML + master XML + nvPr/ph 绑到 KmoSlide
  → [pp-16] theme scheme 表 → schemeClr/srgb 写成 ARGB（fill + run）
  → [pp-15] p:pic.blipFill + r:embed → media bytes 挂到 type=2
  → [pl-07] 把 mergeShapePr 写回 shape；空 ph txBody 从 layout 继承
  → [pr-05] emit DrawImage + 实色 fill；Flutter 画 JPEG/PNG
  → samples/ppt-render 验收：封面有色块、标题字、图片位图
```

| order | feature_id | 层 | I | O | 解锁 | 不放进本层 |
|---|---|---|---|---|---|---|
| 1 | [pp-14-layout-master-bind](parser/features/pp-14-layout-master-bind/FEATURE.md) | parser | `ppt/slides/_rels/*.xml.rels` + layout/master part 字节 | `KmoSlide.layout`/`master` + 每 shape `ph` | pl-03 查找有树可走 | 不重写 PlaceHolderMerger 算法 |
| 2 | [pp-16-scheme-color-resolve](parser/features/pp-16-scheme-color-resolve/FEATURE.md) | parser | theme XML + 形状/run 上的 `schemeClr`/`srgbClr` | `ShapePr.has_fill` + run `font_color` | 色块可见、字可见 | 渐变/主题覆盖全表、lstStyle 九级 |
| 3 | [pp-15-pic-blip-media](parser/features/pp-15-pic-blip-media/FEATURE.md) | parser | `p:pic` + `a:blip r:embed` + slide rels + zip media | `shape_type=2` + `media_id` + bytes | 有图可编指令 | JPEG 解码（归 pr-05 painter） |
| 4 | [pl-07-ph-content-apply](layout/features/pl-07-ph-content-apply/FEATURE.md) | layout | 已绑定的三层树 + 已解析 txBody | 写回的 `sp_pr` + 非空 `text_op` | 后页占位文字/填充不再空 | 不重做贪心断行 |
| 5 | [pr-05-visual-emit-paint](render/features/pr-05-visual-emit-paint/FEATURE.md) | render | 带 fill/media/text 的 `KmoSlide` | `DrawImage` + 有 fill 的 `DrawRect` + 有色 `DrawText`；Flutter 消费 | Sample 视觉内容上屏 | Chart/SVG/WDP/SmartArt |

`pp-14` 必须先于 pl-07；`pp-16` 可与 pp-14 后半并行；`pp-15` 不依赖 layout；`pr-05` 吃 15+16 的输出。

## 2. 与已 pass 叶的关系（复用，不重拆）

| 已 pass | 本波怎么用 |
|---|---|
| pp-01:03 `SwNvPrHandler` | pp-14:03 产品 walker 调同一套 ph 写入，禁止第二套 Placeholder 类型 |
| pp-02:03 `SwPictureHandler` | pp-15 产品路径接 blip；禁止再发明 pic 类型号 |
| pp-03:03 / pp-04 theme | pp-16 把 theme part **读进文档**并解析 scheme 槽；不重写 ColorMapping 叶测 |
| pp-07 layout/master open | pp-14 用 zip 字节 + 现有 slide XML walker 填 `KmoSlideLayout`；正式 `PptxrLayout.open` 仍可后置 |
| pl-03 merge* | pl-07 **应用**返回值；禁止复制 merge 算法 |
| pr-02:03 DrawImage 枚举 | pr-05 在 `render_slide` 真正 emit；painter 实现 `DrawImage` |
| pp-13 PptxDocument | 本波只加字段/绑定，不改页序契约 |

## 3. 验收顺序（Sample_ppt）

1. 首页标题「蓝色年终汇报PPT模板」可见且非黑框空字。  
2. 首页至少 1 条 `DrawImage`（或占位矩形明确标 media miss），Flutter 能画出 JPEG。  
3. 页栏仍 23；翻到非封面页仍有版式色块或继承文字（不再整页空框）。  
4. `cargo test --test <function_id>` 按各 FUNCTION.md；e2e：`ppt_render` 对该 pptx 的 JSON 含 `DrawImage` 且 `DrawText` 非空串比例上升。

## 4. 明确不做（本波）

- SVG / WDP / SVG 占位以外的矢量图解码  
- Chart / Table / SmartArt / OLE（graphicFrame 业务体）  
- Java `RichTextLayout` 像素级断行  
- 动画 / 切换（pp-05）  
- 把 `SwShapeHandler` 整棵编译进默认 crate（仍走产品 walker + 已落地 handler）

## 5. loop 序

```text
pp-14:01 → 02 → 03 → 04
pp-16:01 → 02 → 03
pp-15:01 → 02 → 03
pl-07:01 → 02
pr-05:01 → 02 → 03
```

入口 Java 行数不暂停。每 function 走 **function-java2rust**（交错模板 + `// java`）；产品桥 `pptx_mvp.rs` 无一对一 Java 类时按编排 function 做行为门禁 + N/A 布局（与 pp-13:02 相同），**禁止 `--force` 重生 PptxrSlide.rs**。

---

## 6. 下一波（骨架正确）

本波已 pass 后，Sample 仍可能组坐标错、sp 上的图当矩形、满屏黑线框。下一波 Split：**[SKELETON_PIPELINE.md](./SKELETON_PIPELINE.md)**（**布局 pl-08 → 基础形状 pr-06 → 图 pp-18 → 底 pp-17 → 宿主 pr-07**）。不要重拆本文件里的 function。
