# pp-13-document-assembly — Function 组合图

> depends_on: pp-12 → pp-06 → pp-07/pp-02 → pp-08 → pl-02/pl-03
> 状态：01–04 **pass**

## §0 管线位置

```text
OxfrPackage
  → presentation.xml + rels
  → [01] rId 页序
  → [02] 每页 part → 有序 shape tree
  → [03] layout/master placeholder 继承 + group/shape absolute geometry
  → [04] PptxDocument(slides[])
  → pr-04 multipage envelope / indexed FFI / Flutter pager
```

## §1 function 清单

| order | function | I | O | 状态 |
|---|---|---|---|---|
| 1 | `01_function_presentation_page_order` | presentation XML + rels | 按 `p:sldIdLst` rId 排序的 slide partName | pass |
| 2 | `02_function_slide_graph_assembly` | ordered slide parts | 每页 `KmoSlide.spTree` | pass |
| 3 | `03_function_page_inheritance_geometry` | slide/layout/master | 继承完成的绝对几何骨架 | pass |
| 4 | `04_function_document_handoff` | dimensions + pages | `PptxDocument` | pass |

## §2 裁决

- 不把多页协议塞进 pp-09；pp-09 仍负责 Java `PptxReader.open` 生命周期。
- 不把 shape 解析塞进 pr-04；pr-04 只序列化/选择已装配页面。
- 01 必须按 rId，而不是按 `slideN.xml` 文件名猜序；缺 rels 时仅作兼容 fallback。
- 03 已接通：`open_pptx_document_bytes` 调用 `resolve_pages`；group transform 与 placeholder 继承不得在 painter 修补。

## §3 loop

`01 → 02 → 03 → 04 → pr-04:03 → 04 → 05`
