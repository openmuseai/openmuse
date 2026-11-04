# pp-17-slide-background — 幻灯片底色（cSld bg）

> 层：parser ｜ 状态：**pass** ｜ depends_on: pp-16; pp-13; pr-01  
> 产品 I/O：把 **`p:bg` 接到 `KmoSlide.has_background` + fill**，让 `SlideRender::drawUnderSlide` 的 Background 层真正入列。

## 目标

`open_pptx_document_bytes` 后，有 `p:bgPr/a:solidFill` 或 `p:bgRef` 的页 `has_background==true` 且底色 ARGB 可查询。缺失时永远先铺白底，Sample 第 2 页 `schemeClr tx1` 底丢失。

## I/O

| 输入 | 输出 |
|---|---|
| `p:cSld/p:bg/p:bgPr` 实色（srgb / schemeClr） | 页级 fill ARGB（scheme 走 pp-16 表） |
| `p:bgRef idx` + scheme（母版常见） | 同上；idx 本波只认实色槽，不解析 theme 填充列表全表 |
| `KmoSlide.background()` | `Some` 当且仅当解析到底 |

不解析 bg 上的 blipFill（图走 pp-18：Sample 第 7 页城市是 shape blip，不是 p:bg）。
