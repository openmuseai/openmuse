# ImporterBase

**Java**: `cn/wps/moffice/writer/io/reader/ImporterBase.java`（~44 行）  
**Wave**: 0 / 3  
**相关**: [DocxReader](.../wc-05-orchestration/docs/DocxReader.md) · 所有 `*ConverterSimple` / `SubDocumentImporter`

---

## 1. 职责

所有 importer/converter 的公共基类：保存 **mode** 与可选的 Xml03/Xml07 context，提供格式判定。

---

## 2. 字段与 API

| 成员 | 含义 |
|------|------|
| `mMode` | 0=DOCX，1=Xml07，2=GVml，3=Xml03 |
| `mXml07Cxt` / `mXml03Cxt` | 对应上下文；DOCX 路径常为 null |
| `mode()` / `xml07Context()` / `xml03Context()` | 访问器 |
| `isDocx()` | `mode == 0` |
| `isXml07()` | `mode == 1 && xml07 != null` |
| `isXml03()` | `mode == 3 && xml03 != null` |
| `isGVml()` | `mode == 2` |

---

## 3. 与 DocxReader 的关系

`DocxReader` 构造 `DocumentImporter` 时传入 `mode=0, xml07=null, xml03=null` → 全链路 `isDocx()==true`，`isXml07/03==false`。

部分逻辑（如 numbering 懒加载）显式要求 `isXml07()`，DOCX 主路径可能走另一分支——改写时对照调用点，勿假设「DOCX 等于 Xml07」。

---

## 4. MVP 改写要点

- 实现完整、无 `todo!()`。  
- 子类构造一律 `super(mode, xml07, xml03)`。  
- context 类型可先做空壳 struct，只要指针/Option 语义对齐。

---

## 5. 读懂检查

- [ ] `isXml07()` 为何同时检查 mode **与** context 非空？  
- [ ] SubDocumentImporter 中 `isContainNumbering` 的 xml07 懒加载在纯 DOCX 下是否触发？
