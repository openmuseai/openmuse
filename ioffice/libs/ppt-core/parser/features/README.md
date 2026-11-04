# ppt-core Parser MVP Features

本目录 **只负责 parse**（模型写好）。能画一页见 [MVP.md](../../MVP.md)（parse + **layout pl-*** + **render pr-***）。

目标：按 I/O 把 PPTX 写进 `KmoPresentation`。OPC 仍走 core-libs。

**名称：** `SlideLayoutHandler` = 版式 **XML**，不是排版引擎（排版在 `layout/features/pl-*`）。

## 当前状态

| feature_id | 状态 | 对「能渲染」 |
|---|---|---|
| [pp-01](./pp-01-pptx-parse/FUNCTIONS.md) / [pp-02](./pp-02-shape-tree-parse/FUNCTIONS.md) | **pass** | 树 / pic 行；PptxReader 最小 sp 仍不走完整 handler |
| [pp-10-dml-sppr](./pp-10-dml-sppr/FUNCTIONS.md) | **01 pass**；02–04 spec | xfrm 有；fill 仅 srgb；blip 壳 |
| [pp-11-dml-txbody](./pp-11-dml-txbody/FUNCTIONS.md) | spec（最小 p/r/t 已接线） | DrawText |
| [pp-03-dml-style](./pp-03-dml-style/FUNCTIONS.md) | spec（01–03） | clrMap / lstStyle / **schemeClr** |
| [pp-04-theme-part](./pp-04-theme-part/FUNCTIONS.md) | spec | theme.xml → RGB |
| pp-05 | spec | 动画；不挡首屏 |
| [pp-06](./pp-06-presentation-xml/FUNCTIONS.md) … [pp-09](./pp-09-pptx-open/FUNCTIONS.md) / [pp-12](./pp-12-opc-package/FUNCTIONS.md) | spec | 打开器；**pp-08 母版胶片** |
| layout pl-01…06 / render pr-01…04 | 叶 **pass** | 产品仍骨架；下一波见 [VISUAL_CONTENT_PIPELINE.md](../../VISUAL_CONTENT_PIPELINE.md) |
| [pp-14](./pp-14-layout-master-bind/FUNCTIONS.md) / [pp-15](./pp-15-pic-blip-media/FUNCTIONS.md) / [pp-16](./pp-16-scheme-color-resolve/FUNCTIONS.md) | **spec** | layout 绑定、图片字节、主题色 |

## Parse 依赖链

```text
pp-01 / pp-02（已 pass）
  ├─ pp-10 spPr、pp-11 txBody     ← 画形状/字的模型输入
  ├─ pp-03 clrMap、pp-04 theme    ← 画 scheme 色
  ├─ pp-05 timing                 ← 非绘制
  └─ pp-06 → pp-07 → pp-08 → pp-09  ← 打开 pptx（产品闭环；捷径可 fixture 跳过）
       └─ pp-12 OPC 包桥接（真实文件 → part/流；OPC 本体在 core-libs wave0）
```

## 推荐 loop（骨架之后：色 / 字 / 图）

产品下一波见 [VISUAL_CONTENT_PIPELINE.md](../../VISUAL_CONTENT_PIPELINE.md)：

```text
pp-14 layout/master/nvPr 绑定
  → pp-16 scheme/srgb ARGB
  → pp-15 pic blip + media bytes
  → pl-07 写回 mergeShapePr + 空 ph 文本
  → pr-05 DrawImage + Flutter 位图
```

叶级 pp-03/04/10/11 **不要再当「下一条」重 loop**；缺口在 `pptx_mvp` 产品 I/O。

细则：[PIPELINE.md](./PIPELINE.md)
