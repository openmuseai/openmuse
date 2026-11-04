# FinishReadingWork

**Java**: `cn/wps/moffice/writer/io/reader/tools/FinishReadingWork.java`（~279 行）  
**Wave**: 5  
**相关**: [DocxReader](./DocxReader.md) · [CoreHelper](../../wc-01-storage-write/docs/STORAGE.md#写入热路径钉死) · [DocumentImporter](./DocumentImporter.md)

---

## 1. 职责

`XWPFDocument.parse()` 成功返回后的**收尾修补**：结束绘图容器读取、补主文档结束段、修正节/脚注/文本框链、确保 Normal 样式、触发 `onMainDocumentEnd`。

---

## 2. `go()` 主流程

```text
IOHelper.endAllSubdocumentsDrawingContainer(doc)
ShapeIDTool.reset()
correctCoreData()
processDmlLinkedTxbx()      // 各 SubDocType 的 DmlImporter.buildTxbxChain
TextboxInfoTool.run()
IOHelper.addNormalStyle(styles)
mImporter.onMainDocumentEnd()
```

---

## 3. `correctCoreData()`（MVP 相关）

```text
CoreHelper.supplyEndPara(mainDocument)   // ★ 保证文档至少有结束段
correctShapePropertyId_textNext()
setTextIdentifiers()
correctSectionData()
correctFootnoteDocument()
correctEndnoteDocument()
```

`supplyEndPara`：若长度 0 或末字符不是段标，则 `appendParagraph('\r', …)`。

---

## 4. MVP 改写要点

- Must 调用：`supplyEndPara`、`addNormalStyle`、`onMainDocumentEnd`。  
- Drawing / textbox / footnote 纠正：可空实现，但不要跳过整个 `go()`。  
- 顺序尽量与 Java 一致，避免节数据在 supplyEndPara 前后 CP 错位。

---

## 5. 读懂检查

- [ ] 为何 parse 后再 `addNormalStyle`？（与 `onMainDocumentStart` 重复调用的原因）  
- [ ] `supplyEndPara` 对「最后一段已是 `\r`」的文档是否 no-op？  
- [ ] `correctSectionData` 与 `PLCSection.onEndLoad` 关系？
