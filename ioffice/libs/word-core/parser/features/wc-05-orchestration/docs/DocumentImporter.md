# DocumentImporter

**Java**: `cn/wps/moffice/writer/io/reader/docxReader/importer/DocumentImporter.java`（~1285 行）  
**接口**: `IDocumentImporter`（core-libs `cn.wps.io.xwpf`）  
**基类**: 经 `ImporterBase` 体系持有 styles / mode  
**Wave**: 5  
**相关**: [DocxReader](./DocxReader.md) · [SubDocumentImporter](.../wc-04-paragraph-import/docs/SubDocumentImporter.md) · [StylesImporter](./StylesImporter.md)

---

## 1. 职责

XWPF SAX 回调的**总线**：按 `SubDocType` 把事件路由到对应 `*DocumentImporter`；把 styles/numbering/settings/theme/font 事件路由到专用 importer。

自身几乎不写 rope；真正落库在子 importer + helpers。

---

## 2. 构造（主构造重载）

```text
DocumentImporter(TextDocument document, IOListener, IDLProvider,
                 initDrawingContainer, ThemeContext, mode, xml07, xml03)
```

关键步骤：

1. 若 `mainDocument.length == 0` → `document.setProperty(PropertySet.empty)`  
2. `mainDocument = createSubDocument(0)`  
3. 创建：
   - `SettingsImporter` / `FontTableImporter` / `NumberingImporter` / `ThemeImporter`
   - `MainDocumentImporter(main)`
   - `MainTextboxDocumentImporter(createSubDocument(5))`
   - `HeaderTextboxDocumentImporter(createSubDocument(6))`
   - `HdrFtrDocumentImporter(createSubDocument(2))`
   - `CommentsDocumentImporter(createSubDocument(3))`
   - `FootnoteDocumentImporter` / `EndnoteDocumentImporter`
4. 共享：`StyleId2Sti`、`NumIdRedirection`、`mIoListener`、`mIDLProvider`

另有只持 `KStyles` 的较轻构造，供样式相关路径。

---

## 3. 子文档路由

```text
getSubDocumentImporter(SubDocType):
  MAIN_DOCUMENT        → mMainDocumentImporter
  TEXTBOX_DOCUMENT     → mTextboxDocumentImporter
  HEADER_DOCUMENT      → mHdrFtrDocumentImporter
  COMMENT_DOCUMENT     → mCommentsDocumentImporter
  ENDNOTE_DOCUMENT     → mEndnoteDocumentImporter
  FOOTNOTE_DOCUMENT    → mFootnoteDocumentImporter
  HEADERTEXTBOX_DOCUMENT → mHdrFtrTextboxDocumentImporter
```

**MVP**: 必须实现 `MAIN_DOCUMENT`；其余返回可空操作的 stub，避免 NPE。

---

## 4. 段落事件转发模式

几乎所有 `onImport*` 都是同一模式：

```text
onImportParagraphStart(subDocType, attributes)
  paraId = ConvertHelper.getString(attributes, "paraId")
  getSubDocumentImporter(subDocType).importParagraphStart(paraId)
  // COMMENT 额外 putParaId

onImportParagraphEnd(paragraph, subDocType, layerTable)
  getSubDocumentImporter(...).importParagraphEnd(paragraph, layerTable)

onImportTableStart / Hyperlink* / FldSimple* / RunText* …
  → 同理转发
```

---

## 5. 样式 / 编号 / 设置转发

| 回调族 | 目标 |
|--------|------|
| `importDocDefaults` / `importStyleStart|Child|End` / `importStylesEnd` / `importTblStylePrEnd` | `StylesImporter` |
| `importNumbering*` | `NumberingImporter` |
| `importSettingsStart|SimpleChild|End` | `SettingsImporter` |
| Theme / FontTable 相关 | `ThemeImporter` / `FontTableImporter` |

---

## 6. 主文档生命周期钩子

| 方法 | 逻辑 |
|------|------|
| `onMainDocumentStart(hasHeaderFooter)` | 若页眉子文档仍空且 hasHeaderFooter → 记标志；`IOHelper.addNormalStyle(styles)` |
| `onMainDocumentEnd()` | 清 Vml shape types；footnote/endnote importer `clear()` |
| `onHeaderDocumentEnd` / `onFooterDocumentEnd` | 补段末 + `reviseNodeHeader/Footer*` |
| `dispose` / `dispose4StyleXml` | 释放子 importer |

---

## 7. MVP 改写要点

- 实现构造中 **Main** 路径 + Styles +（最小）Settings/Numbering。  
- 其它 SubDocType importer：签名完整、方法空实现。  
- 保持 `IDocumentImporter` 方法面与 XWPF 回调一致（否则 parse 驱动对不上）。

---

## 8. 读懂检查

- [ ] `createSubDocument(0)` 与 `getMainDocument()` 是否同一实例？  
- [ ] 样式回调发生在正文段落回调之前还是交错？  
- [ ] `addNormalStyle` 在 start 与 `FinishReadingWork` 各调一次，分别防什么？
