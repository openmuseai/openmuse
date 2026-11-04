# pp-18-sp-picture-fill — 形状上的图片填充（非仅 p:pic）

> 层：parser ｜ 状态：**pass** ｜ depends_on: pp-15; pr-05  
> 产品 I/O：`a:blipFill` 在 **`p:sp` 上与 spPr 并列或在其内**，当前只把 pic 挂 media。

## 目标

全幅背景图、装饰贴图以 `DrawImage` 出现。Sample 第 7 页城市是 `prstGeom=rect` + `a:blip r:embed="rId3"` → `ppt/media/image2.jpg`，**不是** `<p:pic>`。

## I/O

| 输入 | 输出 |
|---|---|
| `p:sp` 内 `a:blipFill/a:blip/@r:embed` | 与 pp-15 相同的 rId |
| slide rels Type=image | media partName |
| zip bytes（jpeg/png） | `media_bytes` + `media_id`；`render_slide` 发 DrawImage（不要求 shape_type==2） |

SVG 字节：不解码，可挂 bytes 但 painter 画占位 Rect。禁止用实色 fill 冒充 JPEG。
