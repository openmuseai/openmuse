# pp-15-pic-blip-media — 图片 blip 与媒体字节

> 层：parser ｜ 状态：**pass** ｜ depends_on: pp-02:03; pp-10:02; pp-13:02  
> `p:blipFill` 在 `p:pic` 上是 **spPr 的兄弟**，当前 `parse_one_sp` 抽不到。

## 目标

产品树上 `shape_type==2` 的节点带 `media_id` 与 zip 内图片字节（或明确 miss）。不解码位图。

## I/O

| 输入 | 输出 |
|---|---|
| `<p:pic>…<p:blipFill><a:blip r:embed="rIdN"/>` | embed rId |
| `ppt/slides/_rels/slideN.xml.rels` image Relationship | `ppt/media/imageK.jpg` |
| zip entry bytes | 挂到 shape 的 media 句柄（禁止当实色 fill 冒充） |
