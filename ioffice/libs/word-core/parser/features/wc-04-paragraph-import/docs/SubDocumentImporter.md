# SubDocumentImporter

**Java**: `cn/wps/moffice/writer/io/reader/docxReader/importer/document/SubDocumentImporter.java`（~717 行）  
**基类**: `ImporterBase`  
**Wave**: 4（写入状态机核心）  
**相关**: [MainDocumentImporter](./MainDocumentImporter.md) · [ParagraphContext](./ParagraphContext.md) · [CoreHelper](../../wc-01-storage-write/docs/STORAGE.md#写入热路径钉死) · [ParaPropsConverterSimple](.../wc-03-property-convert/docs/ParaPropsConverterSimple.md)

---

## 1. 职责

针对**一个** `SubDocument` 的段落级写入器：把 XWPF 抛出的「段开始 / run 文本 / 段结束 / 表单元格」变成 `CoreHelper.appendText/appendParagraph`。

是 parse MVP 热路径中**最重要**的业务类之一。

---

## 2. 关键字段

| 字段 | 含义 |
|------|------|
| `mDocument` | 目标 `SubDocument` |
| `mStyleId2Sti` / `mNumIdRedirection` | 样式 id、编号 id 重定向 |
| `mCurrentParagraph` | 当前段会话 `ParagraphContext` |
| `mComplexFieldNodeBegin` / `mFieldsNotEnded` | 复杂域状态（MVP 可浅实现） |
| `mIoListener` | 进度 |
| `mWriteLock` | 写锁（主文档域相关） |
| `mDmlImporter` 等 | 绘图（MVP stub） |

---

## 3. 构造

```text
SubDocumentImporter(document, styleId2Sti, numIdRedirection,
                    ioListener, initDrawingContainer, mode, xml07, xml03)
  → super(mode, …)
  → 断言 document / styleId2Sti
  → 若 initDrawingContainer → document.getDrawingContainer().beginRead()
  → 清域状态计数
```

---

## 4. MVP 热路径三件套

### 4.1 `importParagraphStart(paraId)`

```text
mCurrentParagraph = new ParagraphContext(mDocument.getLength())
若 paraId != null → mCurrentParagraph.mParaId = paraId
```

**关键**: 此时抓取的 length 即本段文本起始 CP，之后 PAPX 插入要用它。

### 4.2 `importRunText` / `importRunTextSimple`

```text
若 len > 0:
  CoreHelper.appendText(mDocument, text|buffer, runProps)

若复杂域 begin 仍为 FieldEmpty:
  mComplexFieldNodeBegin.mData = toFieldType(runText)  // 用指令文本猜类型
```

重载：`String`、`char[]+start+len`、`importRunText4InlineChart`（拷贝 props 再调）。

### 4.3 `importParagraphEnd(paragraph, layerTable)` — 逐步

```text
1. endMark = paragraph.getEndMark()          // '\r' / '\f' / …
2. dstParaProps = paragraph.getParaProp()    // 可 null → new Mutable
3. dstRunProps  = paragraph.getRunProp()     // 段标字符属性
4. 若有 paraId:
     dstParaProps.setValue(282, paraId)
     textDocument.getKParagraphsID().addParaphId(paraId)
5. ParaPropsConverterSimple.correctProp(dstParaProps)
6. 若 layerTable >= 1 → ImporterHelper.amendTableProps(...)
7. mEntry = CoreHelper.appendParagraph(
       mDocument, endMark,
       mCurrentParagraph.mPositionInserted,   // ★ 段首 CP
       freeze(runProps), freeze(paraProps))
8. 若有 paraId → putId(paraId, type, mEntry)
9. 若 endMark == '\f':
     getPLCSection().setLastSectionEndCP(getLength())
10. 若 isContainNumbering(mEntry):
      可能触发 numbering part 懒解析（xml07）
11. return mEntry
```

---

## 5. 表格相关（MVP 次优先）

| 方法 | 逻辑摘要 |
|------|----------|
| `importTableStart(layerTable)` | 若当前层与 last papx 的 224 相同，可能补空段（223/224） |
| `importTableCellStartSimple` | `resetParagraph()`（清 mEntry） |
| `importTableCellEndSimple` | 无 entry 则 append 空段；设 223/224/226；`CoreHelper.setCellMark` |
| `doImportTableRowEndSimple` | `appendParagraph(rowMark, runProp, tableProps)` |

---

## 6. 编号探测 `isContainNumbering`

```text
paraBreak.property 含 234 或 235 → true
否则若含 191（样式）→ 取 KStyle.property，再看 234/235
```

用于懒加载 numbering.xml，避免无编号文档也解析。

---

## 7. MVP 可 stub 方法族

- Hyperlink / FldSimple / ComplexField begin-end  
- Drawing / DML / Math / Bookmark / Comment  
- 保持方法存在且不抛 NPE  

---

## 8. 读懂检查

- [ ] 为何 PAPX 用 `mPositionInserted` 而不是段末字符的 CP？对照 `CoreHelper.appendParagraph(ch, cp, …)`  
- [ ] `correctProp` 缺省会把样式写成 0（Normal）——对无样式段意味着什么？  
- [ ] `'\f'` 只 `setLastSectionEndCP`，真正 `addSection` 在何处？
