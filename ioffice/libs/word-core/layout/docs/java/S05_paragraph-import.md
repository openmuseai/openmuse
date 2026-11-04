# S05 — 正文事件 → 文档落库（rope/CHPX/PAPX/节）

## 1. 职责

把段落生命周期事件变成对 `CoreHelper` 的有序调用，落到 TextDocument 的低层存储：

```text
importParagraphStart(paraId)   → ParagraphContext(cp0 = document.getLength())
importRunTextSimple(text, chpx) → CoreHelper.appendText
importParagraphEnd(paragraph, layerTable) → correctProp → appendParagraph(…, mPositionInserted, …)
```

管线位置：上游 S03/S04；下游 S06（收尾）。

## 2. 核心 Java 代码

| 类 | 职责 |
|----|------|
| `ParagraphContext` | 段会话：`mPositionInserted`（段首 CP）、`mParaId`、`mEntry`（PAPX Entry） |
| `SubDocumentImporter` | Start/Run/End/Table/Field 回调（段落写入状态机） |
| `MainDocumentImporter` | 主文档 guard + `\f` 节 CP 记忆 + `doImportSectionPropertiesSimple` |
| `CoreHelper.appendText/appendParagraph` | rope 追加 + CHPX/PAPX insert |

`importParagraphEnd` 8 步（Java `SubDocumentImporter.java:370–417`，钉死）：

```java
char endMark = paragraph.getEndMark();                       // '\r' | '\f'
MutablePropertySet dstParaProps = paragraph.getParaProp();   // null → new
MutablePropertySet dstRunProps  = paragraph.getRunProp();
if (mCurrentParagraph.mParaId != null) {
    dstParaProps.setValue(282, mParaId);
    getKParagraphsID().addParaphId(mParaId);
}
ParaPropsConverterSimple.correctProp(dstParaProps);          // 缺191 → 0
if (layerTable >= 1) ImporterHelper.amendTableProps(dstParaProps, layerTable);
mEntry = CoreHelper.appendParagraph(mDocument, endMark, mPositionInserted,
             freeze(runProps|empty), freeze(paraProps|empty));   // ★ PAPX@段首 cp0
if (mParaId != null) getKParagraphsID().putId(mParaId, getType(), mEntry);
if (endMark == '\f') getPLCSection().setLastSectionEndCP(getLength());
```

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| `ParagraphContext` | `mPositionInserted: Option<i32>`、`mParaId: Option<String>`、`mEntry: Option<Box<dyn IPapxEntry>>` |
| rope | 字符流；`length()`；CHPX insert CP = append 前 length |
| CHPX | run 属性区间（`[cp, next)`） |
| PAPX | 段属性区间；**insert CP = 段首 `mPositionInserted`** |
| `KParagraphsID` | `paraId ↔ PAPX entry` 映射（282 / addParaphId / putId） |
| `PLCSection` | `\f` 分节锚点（`setLastSectionEndCP`） |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | `importParagraphStart(paraId)`、`importRunText(text\|char[], chpx)`、`importParagraphEnd(paragraph, layerTable)`… |
| **O** | main rope 追加文本；`CHPX`@run 首 CP；`PAPX`@段首 cp0；特殊字符 `\r/\f/0x07/19,20,21`；`KParagraphsID`（paraId↔PAPX）；`PLCSection`（`\f` 记截止 CP） |

## 5. 上下游

- **上游**：S03 分发、S04 属性桶。
- **下游**：S06 收尾（supplyEndPara、默认节兜底）；layout S08–S12 消费 rope/CHPX/PAPX/节。

## 6. 边界 / 失败语义 / 不变量

- **PAPX cp = Start 时 `getLength()`**，不是 End 时 length（否则段属性错位）。
- **CHPX insert CP = 当次 append 前 length**。
- 特殊字符码：`\r`=0x0D、`\f`=0x0C、`\u0007`=0x07（单元格）、19/20/21（域 begin/sep/end）。
- propId：282=paraId、191=pStyle、223=inTable、224=tableLevel、226=cellEnd、234/235=numPr、28=特殊字符。
- 空 run 文本 no-op；`correctProp` 不可省略。

## 7. Rust 承接与现状

wc-04：`01_function_paragraph_context`（pass）、`02_function_run_text`（pass）、`03_function_paragraph_end`（Start/End/isContainNumbering 已实现，paraId 建档与 `\f` 副作用暂 defer）、`04/05/06/07` 后续。