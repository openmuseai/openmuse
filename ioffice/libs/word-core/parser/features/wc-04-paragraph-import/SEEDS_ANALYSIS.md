# wc-04-paragraph-import — Seeds 核心文件解析

对照 [seeds.csv](./seeds.csv)。

---

## 热路径状态机

```text
importParagraphStart(paraId)
  → ParagraphContext(cp0 = getLength())

importRunTextSimple(text, chpx)
  → CoreHelper.appendText

importParagraphEnd(paragraph, layerTable)
  → correctProp(paraProps) [+ amendTableProps]
  → CoreHelper.appendParagraph(endMark, cp0, runProps, paraProps)
  → 若 '\f': PLCSection.setLastSectionEndCP
```

---

## 1. ParagraphContext（order 500）

三字段会话：`mPositionInserted` / `mParaId` / `mEntry`。  
**钉死**: PAPX insert 用 Start 时的 cp0，不是段末 length。

文档：[docs/ParagraphContext.md](./docs/ParagraphContext.md)

---

## 2. SubDocumentImporter（order 501）— 本切片枢纽

depends：SubDocument、ParagraphContext、CoreHelper、Para/Run converters、StyleId2Sti、NumIdRedirection、ImporterHelper、PLCSection。

**Must**: Start / RunText* / End；Table 可次优先。  
**Stub**: Drawing / Math / Field / Bookmark / Comment 回调（防 NPE）。

`importParagraphEnd` 逐步分解见 [docs/SubDocumentImporter.md](./docs/SubDocumentImporter.md)。

---

## 3. MainDocumentImporter（order 502）

extends SubDocumentImporter：

- `checkQuit`（ForceQuit / ReadingLimit）  
- lock/unlock（域计数；无域恒 ended）  
- `\f` → 记 `mCpLastSectionEnd`  
- `doImportSectionPropertiesSimple` → `PLCSection.addSection`

文档：[docs/MainDocumentImporter.md](./docs/MainDocumentImporter.md)

---

## 验收

单段多 run + 段末 `\r`：rope / CHPX / PAPX 与 Java 对照；可选 `\f` + addSection。

## 改写顺序

ParagraphContext → SubDocumentImporter 热路径 → MainDocumentImporter 节增量
