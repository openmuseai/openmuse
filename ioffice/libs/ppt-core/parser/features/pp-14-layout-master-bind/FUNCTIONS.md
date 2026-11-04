# pp-14-layout-master-bind — Function 组合图

> 状态：**pass** ｜ 管线：[VISUAL_CONTENT_PIPELINE.md](../../../VISUAL_CONTENT_PIPELINE.md)

## §0 管线位置

```text
pp-13:01 ordered slide names
  → [01] slide.xml.rels → layout partName
  → [02] layout XML + master XML → KmoSlideLayout/KmoSlideMaster.spTree
  → [03] nvPr/ph 写入产品 shape
  → [04] slide.setLayout + resolve_pages 有树可查
  → pl-07 应用 mergeShapePr / 继承 txBody
```

## §1 清单

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_slide_layout_rel` | slide rels XML | layout partName（规范化） | pp-13:01 | 02 | pass |
| 2 | `02_function_layout_master_graphs` | layout+master XML 字节 | 两层 `KmoShapeTree` | 01；pp-13:02 walker | 04 | pass |
| 3 | `03_function_nvpr_ph_bind` | `p:nvPr`/`p:ph` | ph type/idx + is_placeholder | pp-01:03 | pl-03;pl-07 | pass |
| 4 | `04_function_bind_resolve_pages` | slides + layout/master | `KmoSlide.layout` 非空后 `resolve_pages` | 02;03;pl-03 | pl-07 | pass |

## §2 数据流

```text
slideN.xml.rels
  Relationship Type=slideLayout Target=../slideLayouts/slideLayoutK.xml
layoutK.xml.rels
  Relationship Type=slideMaster Target=../slideMasters/slideMasterM.xml
parse_tree_children(layout XML) / parse_tree_children(master XML)
slide.layout = Rc(layout); layout.master = Rc(master)
```

## §3 测试面

crate：`libs/ppt-core`。镜像：`tests/cn/wps/show/pptxr/pp14_*.rs`（L2）。fixture：最小 zip（presentation + 1 slide + 1 layout + 1 master + rels）。不跑 Flutter。

## §4 验收

1. Sample_ppt slide1 rels → `ppt/slideLayouts/slideLayout12.xml`。  
2. 绑定后 `slide.layout.is_some()`；master 可达。  
3. `p:ph type="ctrTitle"` → placeholder type 与 Java `placeholderTypeMap` 一致。  
4. `resolve_pages` 后至少一处 ph 的 xfrm 来自 layout（相对仅 slide XML 有差或相等但 layout 树非空）。

## §5 loop

`01 → 02 → 03 → 04`

## §6 类级

| Java | 本 function | 说明 |
|---|---|---|
| `PptxrSlide` rels 取 layout | 01 | 产品桥；禁止 `--force` 重生 1200 行模板 |
| `PptxrLayout.open` / `PptxrMaster.open` | 02 | 可复用 walker；正式 open 仍归 pp-07 |
| `SwNvPrHandler` | 03 | 复用已 pass 叶；产品 walker 调用 |
| `PlaceHolderMerger` | 04 只调用 | 算法归 pl-03 |

## 文档

- [functions.csv](./functions.csv)
