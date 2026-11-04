# wc-03-property-convert — Seeds 核心文件解析

对照 [seeds.csv](./seeds.csv)。标准侧 pPr/rPr 见 [wave0-ooxml-standards](../wave0-ooxml-standards/docs/wordprocessingml.md)。

---

## 1. ImporterBase（order 400）

| mode | 含义 |
|------|------|
| 0 | DOCX（`DocxReader` 默认） |
| 1 | Xml07 |
| 2 | GVml |
| 3 | Xml03 |

`isDocx()` / `isXml07()` / `isXml03()`。文档：[docs/ImporterBase.md](./docs/ImporterBase.md)

**MVP**: 完整实现，无 `todo!()`；context 可为 Option 空壳。

---

## 2. ConvertHelper（order 402）

~1100 行静态工具。标签 `subset-hotpath`：先移植主路径 attr 解析（`getString`/`toBoolean`/`toBRC`/度量）。

文档：[docs/ConvertHelper.md](./docs/ConvertHelper.md) — **禁止线性通读**，按 Run/Para converter 调用点 grep。

---

## 3. RunPropsConverterSimple（order 406）

`convertRunPropsBase(doc, dst, elementId, attrs, styleId2Sti)`  
巨大 `switch(elementId)` → 字符 PropertySet。

标签 `subset-highfreq`：字体/字号/粗斜体/颜色/rStyle 优先。  
**propId 必须与 Java 一致**。

文档：[docs/RunPropsConverterSimple.md](./docs/RunPropsConverterSimple.md)

---

## 4. ParaPropsConverterSimple（order 407）

- `convertProp` — 段落 XML → props  
- **`correctProp`** — 落库前纠偏：无 191 则 `setInt(191, 0)`（Normal）

标签 `subset-correctProp`：correctProp + setStyle/对齐/缩进 必须。

文档：[docs/ParaPropsConverterSimple.md](./docs/ParaPropsConverterSimple.md)

---

## 数据流

```text
XWPF handler (elementId + Attributes)
  → Run/ParaPropsConverterSimple
  → MutablePropertySet
  → SubDocumentImporter.importRunText / importParagraphEnd
  → CoreHelper (wc-01)
```

## 改写顺序

ImporterBase → ConvertHelper(hot) → Run*Simple(高频) → Para*Simple(+correctProp)
