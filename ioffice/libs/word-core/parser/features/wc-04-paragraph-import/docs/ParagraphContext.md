# ParagraphContext

**Java**: `cn/wps/moffice/writer/io/reader/helper/ParagraphContext.java`（~19 行）  
**Wave**: 4  
**相关**: [SubDocumentImporter](./SubDocumentImporter.md) · [PapxTable](../../wc-01-storage-write/docs/STORAGE.md#43-papxtable--段落级)

---

## 1. 职责

单个段落导入期间的**会话对象**：记住段开始时的 CP、可选 paraId、结束后的 PAPX Entry。

无业务方法，纯数据载体。

---

## 2. 字段

| 字段 | 类型 | 含义 |
|------|------|------|
| `mPositionInserted` | `Integer` | 段开始时 `document.getLength()`，即本段文本起始 CP；供 `appendParagraph(..., cp, ...)` 使用 |
| `mParaId` | `String` | OOXML `w:paraId`，写入 prop **282** |
| `mEntry` | `IPapxTable.Entry` | 段结束后返回的 PAPX 节点；表格单元格路径会复用/重置 |

---

## 3. 构造

```text
ParagraphContext(int position)
  → mPositionInserted = position

ParagraphContext(IPapxTable.Entry entry)
  → mEntry = entry   // 较少用于「从已有段继续」场景
```

`SubDocumentImporter.importParagraphStart` 使用第一种。

---

## 4. 生命周期（与 SubDocumentImporter）

```text
Start:  new ParagraphContext(length)  → 记下 cp0
Runs:   appendText 多次              → 文本落在 [cp0, …)
End:    appendParagraph(..., cp0, …) → PAPX 挂在约定 CP
        mEntry = 返回的 Entry
Cell:   resetParagraph 可能把 mEntry 置 null，再补空段
```

---

## 5. MVP 改写要点

- Rust 可用简单 struct：`position_inserted: i32`、`para_id: Option<String>`、`entry: Option<…>`。  
- **禁止**在 Start 之后用「当前 length」替代 `mPositionInserted` 做 PAPX insert（除非对标 Java 另一重载路径）。

---

## 6. 读懂检查

- [ ] 段内有 3 个 run 时，`mPositionInserted` 是否仍等于第一段字符前的 length？  
- [ ] 表格 `importTableCellStartSimple` 为何只清 `mEntry` 不清 `mPositionInserted`？
