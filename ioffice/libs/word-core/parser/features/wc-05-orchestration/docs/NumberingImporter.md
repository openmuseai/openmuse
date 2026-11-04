# NumberingImporter

**Java**: `cn/wps/moffice/writer/io/reader/docxReader/importer/num/NumberingImporter.java`  
**配套**: `NumIdRedirection.java`  
**Wave**: 5（MVP 最小）  
**相关**: [DocumentImporter](./DocumentImporter.md) · [ParaPropsConverterSimple](.../wc-03-property-convert/docs/ParaPropsConverterSimple.md) · [SubDocumentImporter](.../wc-04-paragraph-import/docs/SubDocumentImporter.md)

---

## 1. 职责

解析 `numbering.xml`：abstractNum / num / lvl，写入文档编号模型，并通过 `NumIdRedirection` 把 OOXML `numId` 映射到内核 id。

段落侧通过 PAPX 的 234/235（或样式继承）引用编号；`SubDocumentImporter.isContainNumbering` 可能触发懒解析。

---

## 2. 关键 API（DocumentImporter 转发）

| 方法 | 阶段 |
|------|------|
| `importNumberingAbstractNumStart/Child/End` | 抽象编号定义 |
| `importNumberingNumStart/Child/End` | 实例 num → abstractNum 绑定 |
| `importNumberingLevelStart/Child/End` | 某级 lvl 的 rPr/pPr |

构造注入：`TextDocument`、`StyleId2Sti`、`NumIdRedirection`、mode/context。

---

## 3. `NumIdRedirection`

OOXML `w:numId` → 内部 numbering id。  
段落转换器在写编号属性时查询本表；未重定向前不得静默丢弃（或需与 Java 一致地忽略）。

---

## 4. MVP 改写要点

- 保证回调不崩溃；至少能登记 numId 映射。  
- 完整 lvl 格式、多级列表、legal 等后补。  
- 无编号的纯文本文档：importer 可全程空转。

---

## 5. 读懂检查

- [ ] abstractNum 与 num 的 cardinality？  
- [ ] 段落属性里存的是 OOXML numId 还是重定向后 id？  
- [ ] 懒加载 numbering part 的 content-type 字符串是什么？
