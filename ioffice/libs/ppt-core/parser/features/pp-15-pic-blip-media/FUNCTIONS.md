# pp-15-pic-blip-media — Function 组合图

> 状态：**pass** ｜ 管线：[VISUAL_CONTENT_PIPELINE.md](../../../VISUAL_CONTENT_PIPELINE.md)

## §0 管线位置

```text
pp-13:02 parse_tree_children 识别 p:pic
  → [01] 抽取 blipFill r:embed（不走 spPr 子树）
  → [02] slide rels rId → media partName
  → [03] zip 读字节挂到 KmoShowShape
  → pr-05 emit DrawImage
```

## §1 清单

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_pic_blip_embed` | pic XML chunk | r:embed rId | pp-13:02 | 02 | pass |
| 2 | `02_function_slide_image_rel` | slide rels + rId | media partName | 01 | 03 | pass |
| 3 | `03_function_media_bytes_bind` | partName + zip | shape media bytes/id | 02 | pr-05 | pass |

## §2 数据流

```text
p:pic
  p:blipFill / a:blip @r:embed
  p:spPr / a:xfrm          ← 几何仍走现有 spPr
slide rels Type=image Target=../media/image1.jpg
zip_read(ppt/media/image1.jpg)
```

## §3 测试面

最小 zip：1 slide + 1 pic + 1 jpeg 字节。断言 type=2、rId、bytes len。无图 rel → media miss，**不是**实色矩形。

## §4 验收

Sample_ppt slide1：`rId3` → `ppt/media/image1.jpg`，shape 上有非空 bytes。

## §5 loop

`01 → 02 → 03`

## §6 类级

| Java | function |
|---|---|
| `SwPictureHandler` blip 子树 | 01（产品桥可抽标签；正式 handler 接线优先于手写第二套） |
| `HandlerCallback` media map | 02/03 |
| 解码 / Flutter | pr-05 |
