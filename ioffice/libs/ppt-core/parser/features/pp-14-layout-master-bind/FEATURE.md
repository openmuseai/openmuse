# pp-14-layout-master-bind — slide→layout→master 产品绑定

> 层：parser ｜ 状态：**pass** ｜ depends_on: pp-13; pp-07; pp-01:03  
> 产品 I/O：把 **版式/母版 XML 和 nvPr** 接到 `PptxDocument`，不重写 PlaceHolderMerger。

## 目标

`open_pptx_document_bytes` 产出的每页 `KmoSlide` 带 `layout`/`master` 树，且每个 shape 有 `is_placeholder` + `ph.type/idx`。缺失时 pl-03 查找永远 miss，画面只剩 slide 自带 xfrm 空框。

## I/O

| 输入 | 输出 |
|---|---|
| zip 内 `ppt/slides/_rels/slideN.xml.rels` | slideLayout target partName |
| `ppt/slideLayouts/*.xml` + layout rels 的 slideMaster | `KmoSlideLayout` + `KmoSlideMaster` 形状树（复用 pp-13:02 walker） |
| `p:nvPr` / `p:ph` | `KmoShowShape.is_placeholder` + `Placeholder` |

不解析 pic/blip、不解析 theme。
