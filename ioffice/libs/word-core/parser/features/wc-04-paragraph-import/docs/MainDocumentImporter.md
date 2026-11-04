# MainDocumentImporter

**Java**: `cn/wps/moffice/writer/io/reader/docxReader/importer/document/MainDocumentImporter.java`（~216 行）  
**继承**: `SubDocumentImporter`  
**Wave**: 4  
**相关**: [SubDocumentImporter](./SubDocumentImporter.md) · [PLCSection](../../wc-01-storage-write/docs/STORAGE.md#52-plcsection排版最敏感的一种) · [DocumentImporter](.../wc-05-orchestration/docs/DocumentImporter.md)

---

## 1. 职责

主文档（`SubDocType.MAIN_DOCUMENT` / type=0）专用增量：

- 强制退出 / 阅读长度限制  
- 域结束后的 lock/unlock  
- 分节 CP 记忆 + 节属性写入 `PLCSection`  
- 批注引用 importer 壳  

正文段落落库仍走父类 `SubDocumentImporter`。

---

## 2. 额外字段

| 字段 | 含义 |
|------|------|
| `mCommentImporter` | `CommentStartEndRefImporter` |
| `mParaId2CmtRefNode` / `mMcdName2CmtRefNode` | 批注映射（MVP stub） |
| `mDlProvider` | `IDLProvider`：ForceQuit / ReadingLimit |
| `mCpLastSectionEnd` | 上一节结束 CP；`null` 表示尚未分节 |
| `mSectionPropsImporter` | 懒创建的节属性转换器 |

---

## 3. 覆盖方法

### 3.1 `importParagraphStart`

```text
checkQuit()
  → dlProvider.isOpenForceQuit → ForceQuitException
  → length > readingLimit → ReadingLimitException
若 fieldsHaveEnded() → lock()
super.importParagraphStart(paraId)
```

### 3.2 `importParagraphEnd`

```text
paraBreak = super.importParagraphEnd(...)
若 endMark == '\f':
  mCpLastSectionEnd = ImporterHelper.getCpSubDocumentEnd(mDocument)
若 fieldsHaveEnded() → unlock()
return paraBreak
```

### 3.3 `fieldsHaveEnded`

```text
return mFieldsNotEnded == 0
```

无复杂域时恒为 true → lock/unlock 成对出现但对无域文档影响有限。

---

## 4. 节属性

### `convertSectPr(mutablePropertySet, elementId, attributes)`

- 特殊 elementId `1827547294`：按 Normal 样式字号调 `convertDocumentGrid`  
- 其它：`SectionPropsImporter.convertSectionPrContents`

### `doImportSectionPropertiesSimple(...)`

```text
plcSection = mainDocument.getPLCSection()
isFirstSection = (mCpLastSectionEnd == null)
若 mCpLastSectionEnd == null → 置 0
若 plcSection.isEmpty() || !isFirstSection:
  sectionNode = plcSection.addSection(mCpLastSectionEnd)
  挂 even/odd/first header/footer 节点（可全 null 跳过）
  写入 sectionProps 到节点
```

**与父类配合**: 父类在 `\f` 时 `setLastSectionEndCP(length)`；本类在真正收到 sectPr 时 `addSection`。

---

## 5. MVP 改写要点

- 必须：`checkQuit`（可简化）、节 CP 记忆、`doImportSectionPropertiesSimple` 最小路径。  
- 可 stub：Comment importer、页眉页脚节点挂接。  
- 不要丢掉对 `super.importParagraph*` 的调用。

---

## 6. 读懂检查

- [ ] 首次节与后续节：`mCpLastSectionEnd == null` 分支差在哪？  
- [ ] `getCpSubDocumentEnd` 与 `getLength()` 是否总相同？  
- [ ] lock 的粒度是「段」还是「域」？
