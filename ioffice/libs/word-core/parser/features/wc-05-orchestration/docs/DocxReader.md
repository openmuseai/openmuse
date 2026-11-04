# DocxReader

**Java**: `cn/wps/moffice/writer/io/reader/docxReader/DocxReader.java`（~161 行）  
**接口**: `IDocxReader`  
**Wave**: 5（编排入口）  
**相关**: [DocumentImporter](./DocumentImporter.md) · [FinishReadingWork](./FinishReadingWork.md) · [TextDocument](.../wc-02-document-model/docs/TextDocument.md)

---

## 1. 职责

DOCX 读取的**最外层编排器**：持有已打开的 `XWPFDocument` + 空的 `TextDocument`，创建 `DocumentImporter`，驱动 `parse()`，再做元数据导入与收尾。

不解析 XML 本身；XML/SAX 由 `XWPFDocument`（core-libs）完成，业务写入全部经 `IDocumentImporter` 回调。

---

## 2. 关键字段

| 字段 | 类型 | 作用 |
|------|------|------|
| `mDoc` | `TextDocument` | 目标内存模型 |
| `mXWPFDoc` | `XWPFDocument` | 已打开的 OPC/DOCX |
| `mImporter` | `IDocumentImporter` | 实际为 `DocumentImporter` |
| `mIOListener` | `IOListener` | 加载进度 |
| `mFile` / `mRaf` | File / RAF | 源文件句柄 |
| `mDlProvider` | `IDLProvider` | 强制退出、阅读限制、note 路径 |

---

## 3. 构造逻辑

```text
DocxReader(doc, ioListener, diskDoc, file, raf, dlProvider)
  → 断言 doc / ioListener 非空
  → mImporter = new DocumentImporter(
        doc, ioListener, dlProvider,
        initDrawingContainer=true,
        themeContext=null,
        mode=0,          // isDocx()
        xml07=null, xml03=null)
```

**含义**: 默认 DOCX 模式；xml context 可空；绘图容器在子 importer 中 `beginRead`。

---

## 4. `read()` 逐步分解

```text
1. ShapeIDTool.reset() / TextboxInfoTool.reset()
2. DOM 对象池上限：DefaultElement/Attribute/Text setMaxPoolSize
3. readExtendedProperties()          // App 版本等 → Metadata
4. mainDocument.getPieceTable()
     .setGuessedTotalLength(guessLengthOfDocument())
     // 来自 ExtendedProperties 字符数，缺省 100
5. mDoc.setInIOState(true)
6. mIOListener.onLoadParas(0)
7. mXWPFDoc.setDocumentImporter(mImporter)
8. 若有 dlProvider → setNotePath
9. mXWPFDoc.parse()                  // ★ 事件风暴：样式/编号/正文…
10. importMetaData()
      → readPackageProperties / readCustomProperties / readCustomXmlInfos
11. new FinishReadingWork(mDoc, mImporter).go()
12. mImporter.importCustomXmls()
```

---

## 5. 辅助方法

| 方法 | 逻辑 |
|------|------|
| `guessLengthOfDocument()` | `ExtendedProperties.getNumberCharacters()`，null → 100 |
| `readExtendedProperties()` | `ExtendedPackagePropertiesImporter` |
| `readPackageProperties()` | CoreProperties → `PackagePropertiesImporter`（可带 appVersion） |
| `readCustomProperties()` | → `CustomPackagePropertiesImporter` |
| `readCustomXmlInfos()` | 整表塞进 `Metadata.setCustomXmlInfos` |
| `dumpAllPictures` / `dumpVbaMacro` | 转调 importer |
| `dispose()` | 清空 doc/importer，关闭 XWPF |

---

## 6. MVP 改写要点

- **Must**: `read()` 编排顺序；`DocumentImporter` 创建参数；`setInIOState`；`FinishReadingWork` 调用。  
- **可延后**: 完整 Metadata/CustomXml；VBA/图片 dump。  
- **旁路**: 若 `XWPFDocument.parse` 未就绪，可测「手工调 importer」而不经 `DocxReader`。

---

## 7. 读懂检查

- [ ] 为何在 `parse()` **之前**设 `guessedTotalLength`？  
- [ ] `mode=0` 对 `ImporterBase.isDocx()` 的影响？  
- [ ] 元数据是在 parse **之后**导入，还是之前？Extended 为何提前？
