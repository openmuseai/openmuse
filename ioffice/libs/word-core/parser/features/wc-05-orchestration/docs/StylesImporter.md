# StylesImporter

**Java**: `cn/wps/moffice/writer/io/reader/docxReader/importer/StylesImporter.java`（~237 行）  
**Wave**: 5  
**相关**: [StyleId2Sti](.../wc-02-document-model/docs/StyleId2Sti.md) · [KStyles](.../wc-02-document-model/docs/KStyles.md) · [DocumentImporter](./DocumentImporter.md) · [ConvertHelper](.../wc-03-property-convert/docs/ConvertHelper.md)

---

## 1. 职责

把 `styles.xml` 的 SAX 事件变成 `KStyles` 中的 `KStyle` 列表，并设置文档默认 rPr/pPr。

采用**两阶段**：先收集 `IOStyle` 列表，`importStylesEnd` 时再一次性 `createKStyle` + `add`（以便 basedOn/next/link 的 sti 映射已齐）。

---

## 2. 字段

| 字段 | 含义 |
|------|------|
| `mStyles` | 目标 `KStyles` |
| `mStyleId2Sti` | OOXML styleId → 内部 sti |
| `mTextDocument` | 来自 `kStyles.getTextDocument()` |
| `mCurIOStyle` | 当前正在解析的样式草稿 |
| `mIOStyleList` | 全部草稿（约 30 预分配） |

---

## 3. API 阶段表

| 方法 | 行为 |
|------|------|
| `importDocDefaults(rPr, pPr)` | `setRPrDefault` / `setPPrDefault`（freeze） |
| `importStyleStart(attrs)` | new `IOStyle`；读 `type` / `customStyle` / `styleId` |
| `importStyleChild(elementId, attrs)` | switch：name / basedOn / next / link / qFormat / uiPriority / tblStylePr type |
| `importStyleEnd(StylePrModel)` | `convertTypeAndId`；合并 rPr+pPr+tcPr+tblPr → `mCorePrSet`；入 list |
| `importTblStylePrEnd` | 条件格式相关（表样式） |
| `importStylesEnd` | 遍历 list → `createKStyle` → `mStyles.add`；设 link/qFormat/uiPriority |

---

## 4. `createKStyle` 关键逻辑

```text
id   = toSti(rectifyStyleId(), styleId2Sti)
base = toBaseSti(basedOn, …, id)   // base==self → 4095；coreId==0 → 4095
next = toNextsSti(next, id, …)     // null → next=self
name = toDisplayName(name, id)     // 保留名冲突则加 UUID 后缀
pr   = mCorePrSet.freeze()
return new KStyle(textDocument, id, name, base, next, coreType, pr)
```

`toSti(null) → 4095`（无样式哨兵）。

---

## 5. `getStylePrSet` / `correctTcPrSet`

合并 `StylePrModel` 各 PropertySet；单元格 padding 相关 prop 346–351 会 remove 并做 padding 纠正。

---

## 6. MVP 改写要点

- Must：docDefaults、style 收集、`importStylesEnd` 落库、StyleId2Sti.put。  
- 可延后：复杂 tblStylePr / 条件格式。  
- 必须保证正文段落回调前，`importStylesEnd` 已执行（依赖 XWPF 解析顺序）。

---

## 7. 读懂检查

- [ ] 为何不在 `importStyleEnd` 立刻 `mStyles.add`？  
- [ ] Normal（sti=0）的 basedOn 为何强制 4095？  
- [ ] `IOStyle.convertTypeAndId` 何时调用 `StyleId2Sti.put`？
