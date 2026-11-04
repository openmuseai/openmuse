# wr-10-font-style

> status: **pass**（5/5 function；见 [FUNCTIONS.md](./FUNCTIONS.md)）  
> 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)

## 目标

把 **标题、加粗等核心字符/段落样式** 收成度量用的 `TypoFontStyle`。这是纯文本行级对齐的样式入口，不是绘制层「看起来粗一点」。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | 单个 run 的 CHPX `PropertySet` + 所属段 PAPX（含 `pStyle`）+ `KStyles` 最终样式 + 文档 `rPrDefault`（若已在模型中）。 |
| **O** | `TypoFontStyle`（或等价结构）：`ascii` / `eastAsia` 字体名、`size`（半磅→twip 与 Java 同一换算）、`bold`、`italic`。可选：underline kind、color ARGB（给 wr-15，**不**进 wr-12 断行输入）。 |

一次调用只处理 **一个 run**。段属性里影响行框的 jc/缩进/段距 **不在本 feature 输出**（随 PAPX 原样给 wr-12）。

## Java 对照

`layout/base/env/FontStyleFactory.java`：`PropertySet` → `TypoFontStyle` + 选 typeface 族名。不在这里做 hmtx。

## 必须 / 壳 / 延后

- **must：** Heading 经由 KStyle basedOn 得到的 size/bold；直接 `b`/`i`/`sz`；ascii vs eastAsia 名（按 Java hint/lang 规则的 **最小集**：正文中文走 eastAsia，拉丁走 ascii）。
- **stub：** 主题字体 theme、复杂脚本 cs、高亮 shd、字距。
- **defer：** 嵌入气球缩字、SmartFontSize、数学字体。

## 范围内 / 范围外

- **in：** 标题、加粗、斜体、字号、两套族名。
- **out：** `FontHost` 注入、advance、断行、Paint、改 KStyles parse。若 Stage1 缺 `pStyle`/bold，记 parse 缺口，不在本 feature 里读 XML。

## 验收

同一 PropertySet fixture：输出的 size/bold/italic/族名与 Java `FontStyleFactory` 字段相等（字符串名与半磅/twip 按 Java 单位）。

## 依赖

- **depends：** wr-09（有 PropertySet 引用）；wc-02 KStyles。
- **does not depend：** wr-11 度量实现、wr-12。

## 不过扩展

禁止在这里测字宽或选 glyph id。禁止引入 `RichTextLayout`。
