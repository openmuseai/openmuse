# wr-09-run-stream

> status: **pass**（4/4 function；见 [FUNCTIONS.md](./FUNCTIONS.md)）  
> 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)

## 目标

布局热路径改吃 **已 parse 的 `TextDocument` 主故事**，产出带 cp 的纯文本 run 流。不再从 zip 里拼接 `<w:t>`。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | `TextDocument`：rope 字符、CHPX 表、PAPX 表。只读主文档范围（通常 `0 .. length-1` 的 body）。 |
| **O** | 有序 `LayoutSourceRun[]`：`start_cp`, `end_cp`, `text`, `chpx` 入口（或 PropertySet 引用）, `papx` 入口（段级，run 继承段首）。段边界可识别（`\r` 段末）。 |

下游只许依赖 **O**，不许再打开 docx zip。

## Java 对照

`TextLayouter` 的源列表 / CHPX 游标走 rope；不是 `DocumentImporter`。

## 必须 / 壳 / 延后

- **must：** 主故事连续 run；空段；CHPX 变化切开 run；PAPX 变化即新段；保留 `\r` / `\f` 字符。
- **stub：** 字段、超链接在 rope 里已展开成字则当正文；未展开则当普通字或不出现（fixture 不含域）。
- **defer：** 表单元格子文档、textbox story、页眉页脚 story。

## 范围内 / 范围外

- **in：** 标题段落（pStyle 在 PAPX 上，本 feature **只传递引用**，不解析 Heading 字号）。
- **out：** 改 DocxReader；补 OOXML 属性；度量；断行；FontStyleFactory。

## 验收

给定 Stage1 已对齐的 TextDocument fixture：run 的 cp 区间覆盖主故事且无重叠；拼接 `text` 等于 rope；CHPX/PAPX 入口与 Java 按 cp seek 一致。

## 依赖

- **depends：** wc-05 / wc-02（TextDocument 已存在）。
- **does not depend：** wr-03 度量、wr-04 wrap、FontHost。

## 不过扩展

禁止在本 feature 里解析 `w:rPr` XML 或计算字号。那是 wr-10。
