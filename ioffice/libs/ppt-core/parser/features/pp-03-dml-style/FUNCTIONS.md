# pp-03-dml-style — DML clrMap / lstStyle

> depends_on: `pp-01-pptx-parse`（`HandlerCallback`、`SWReaderMap` 间接）  
> 状态：**pass**（`01`/`02`/`03` 均已 loop）

## §0 管线位置

```text
pp-01 SlideMasterHandler / ColorMapOverrideHandler / TextStylesHandler
  构造 ColorMapHandler / TextListStyleHandler
       ↓ 本 feature
a:clrMap attrs → ColorMapping
a:lstStyle 子树 → 九级 TextListStyle
       ↓
pp-04 theme；pp-07 part open 不再用 kind fixture 假装 clrMap/txStyles
```

本层 **不打开 theme part**（那是 pp-04），不解析 spTree。

## §1 function 清单

| order | function_id | I | O | upstream | downstream |
|---|---|---|---|---|---|
| 1 | `01_function_color_map` | clrMap 元素 attrs + `ColorMapping` | scheme 槽写入（accent/bg/tx/hlink…） | DGMap.schemeColorMap；pp-01 05/07 构造 | pp-01 05–07；pp-07 |
| 2 | `02_function_text_list_style` | lstStyle 子元素 id + style 槽 | title/body/other 或 def 列表样式 | HandlerCallback | pp-01 07 TextStylesHandler；pp-06 defaultTextStyle |
| 3 | `03_function_scheme_color` | `a:schemeClr` attrs + `Color` + callback | `Color.putScheme` + `onAdjustThemeColor` | 01（clrMap 槽名）；pp-01:02 | pp-10:02 solidFill；pp-04 RGB |

## §2 数据流

```text
XmlRoAttr tokens ── ColorMapHandler.startElement ──> ColorMapping.put*
element id ──── TextListStyleHandler.enterSubElement ──> 对应 lvl 的 paragraph style handler
schemeClr val ── SchemeColorHandler.addElementAttr ──> Color.putScheme + onAdjustThemeColor
```

## §3 编译与测试面

- crate：`libs/ppt-core/Cargo.toml`
- 测试镜像：`libs/ppt-core/tests/cn/wps/drawing/dmlr/handler/...`
- loop 前无 `[[test]]`；L2 再接线。
- 单测：`cargo test --manifest-path libs/ppt-core/Cargo.toml --test <function_id>`

## §4 串联验收

1. 01 用内存 attrs 写入全部 Java switch 槽；未知 token 保持默认。
2. 02 对已知 lstStyle 子 id 返回真实 handler kind；unknown → None。
3. 03 `schemeClr val` 写入 scheme 索引；callback 判空；factory 1114219 命中。
4. 回归 pp-01 05/07：constructor fixture 可换成真实类型后仍绿。

## §5 推荐 loop 顺序

`01 → 03 → 02`

## §6 Java/Rust 范围

| Java | Rust 目标 | 裁决 |
|---|---|---|
| `drawing.dmlr.handler.style.ColorMapHandler` | `src/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.rs` | 01 |
| `drawing.dmlr.handler.shape.textshape.text.TextListStyleHandler` | `.../text/TextListStyleHandler.rs` | 02 |
| `drawing.dmlr.handler.color.SchemeColorHandler` + `ColorHandlerFactory`（1114219） | `…/handler/color/SchemeColorHandler.rs` | 03 |
| `ThemeHandler` | pp-04 | 移出 |
| `DGMap` | 本 function 只读 schemeColorMap 字面量/既有表 | 不改 DGMap 全文件 |

## 文档

- [functions.csv](./functions.csv)
- `functions/<function_id>/FUNCTION.md`
