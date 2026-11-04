# SettingsImporter

**Java**: `cn/wps/moffice/writer/io/reader/docxReader/importer/settings/SettingsImporter.java`  
**基类**: `ImporterBase`  
**Wave**: 5（MVP 最小）  
**相关**: [DocumentImporter](./DocumentImporter.md) · [TextDocument](.../wc-02-document-model/docs/TextDocument.md)

---

## 1. 职责

解析 `settings.xml`：文档级开关与默认值写入 `TextDocument`（及关联状态）。  
不参与段落 rope 写入，但影响后续排版/修订等行为。

---

## 2. 关键 API

| 方法 | 行为 |
|------|------|
| `importSettingsStart(XWPFSettings)` | 开始；可取 settings 对象引用 |
| `importSettingsSimpleChild(elementId, attrs)` | 大量 switch：zoom、defaultTabStop、trackRevisions… |
| `importSettingsEnd()` | 收尾提交 |

构造还注入 `IOListener`、`IDLProvider`。

---

## 3. MVP 改写要点

- 方法面完整，默认 no-op 或只实现对比测试需要的开关。  
- 勿在 Start/End 抛异常导致整个 `parse()` 失败。  
- 与 `TextDocument.setProperty` / 专用字段的写入点需对照 Java。

---

## 4. 读懂检查

- [ ] settings 解析相对 styles/document 的顺序？  
- [ ] 哪些设置写入 `TextDocument.property`，哪些是独立字段？
