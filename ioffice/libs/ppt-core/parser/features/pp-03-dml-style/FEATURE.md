# pp-03-dml-style — DML clrMap / lstStyle

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（`01`–`03` 均 `pass`）
> 依赖：pp-01-pptx-parse（`HandlerCallback`、`SWReaderMap` 间接）

## 目标

实现 DML 层三个原子样式槽：`ColorMapHandler` 把 `a:clrMap` 的配色槽名
（accent1-6 / bg1-2 / tx1-2 / hlink / folHlink）写入 `ColorMapping`；`TextListStyleHandler` 把
`a:lstStyle` 的 defPPr + lvl1-9pPr 子元素分发到段落属性 handler；`SchemeColorHandler` 把
`a:schemeClr val` 写成 `Color` 的 scheme 索引并回调 `onAdjustThemeColor`。前两者使 pp-01 05/07
用真实类型替换 fixture；第三者解锁真实 pptx 的主题色引用（pp-10 solidFill）。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | 01：`clrMap` 元素 attrs + `ColorMapping`。02：`lstStyle` 子元素 id + `TextListStyle` 槽。03：`schemeClr` attrs + `Color` + `HandlerCallback` + belonger。 |
| **O** | 01：12 个 scheme 槽写入。02：10 槽 handler 或 None。03：`Color.putMode(4)` + `putScheme` + 0/1 次 `onAdjustThemeColor`。 |

**attr name id 与子元素 id 必须取 kfc `dml/A.java` 常量并与 Java switch 精确一致。**

## Java 对照

| Java | Rust 目标 | 裁决 |
|---|---|---|
| `drawing.dmlr.handler.style.ColorMapHandler` | `src/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.rs`（尚无 Rust） | 01 |
| `drawing.dmlr.handler.shape.textshape.text.TextListStyleHandler` | `.../text/TextListStyleHandler.rs`（尚无 Rust） | 02 |
| `drawing.dmlr.handler.color.SchemeColorHandler` / `ColorHandlerFactory` | `…/handler/color/SchemeColorHandler.rs` | 03 |
| `ThemeHandler` | pp-04 | 移出 |
| `DGMap` | 本 feature 只读 `schemeColorMap` 字面量 / 既有表 | 不改 DGMap 全文件 |

## 必须 / 壳 / 延后

- **must：** 01 的 12 attr case；02 的 10 id case；03 `val`→putScheme + callback 判空 + factory 1114219。
- **stub：** `TextParagraphPropertiesHandler` 方法体；`ColorTransferReader` 变换链。
- **defer：** theme.xml 的 clrScheme RGB（pp-04）。

## 范围内 / 范围外

- **in：** `ColorMapHandler` / `TextListStyleHandler` / `SchemeColorHandler`（+ factory 的 1114219 分支）。
- **out：** theme part 打开（pp-04）；spTree；`TextParagraphPropertiesHandler` 方法体；hsl/sys/prst 色模型。

## 验收

FUNCTIONS.md §4 四条串联验收。四条全绿才可 `spec → pass`。

## 依赖

- **depends：** pp-01-pptx-parse（pp-01:02 `HandlerCallback` 透传；pp-01:05 slide handler、pp-01:07
  `TextStylesHandler` 为构造方）；`ColorMapping` / `TextListStyle` Rust 已存在。
- **does not depend：** theme part（pp-04）；spTree（pp-02）；`XmlRoAttr` reader 实现。

## 不过扩展

禁止打开 theme part（pp-04 职责）。禁止解析 spTree（pp-02 职责）。禁止把 `ThemeHandler`、
`ColorMapOverrideHandler`、`TextParagraphPropertiesHandler` 方法体塞进本 feature 顺便做。禁止改
DGMap 全文件。禁止改 functions.csv / FUNCTIONS.md / manifest.csv 状态列；loop 前不落可执行测试、
不填覆盖率与对齐报告数字（保持占位）。