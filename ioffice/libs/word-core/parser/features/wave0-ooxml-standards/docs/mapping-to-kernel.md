# OOXML 概念 ↔ 本内核模型映射

把国际标准词汇钉到本仓库的具体类型上。改写时以本表为「词典」。

---

## 1. 包与入口

| OOXML / OPC | 本项目 |
|-------------|--------|
| `.docx` Package | `XWPFDocument`（disk）+ `DocxReader`（编排） |
| Part: document.xml | 回调 → `MainDocumentImporter` → Main `SubDocument` |
| Part: styles.xml | `StylesImporter` → `KStyles` |
| Part: numbering.xml | `NumberingImporter` + `NumIdRedirection` |
| Part: settings.xml | `SettingsImporter` → `TextDocument` 设置 |
| docProps/core\|app\|custom | `*PackagePropertiesImporter` → `Metadata` |
| Relationship / Content Type | XWPF / OPC 层；业务用 Type 字符串查找 Part |

---

## 2. 故事（Story）与子文档

Word 中多个「故事」：主文、页眉、脚注、批注、文本框…

| OOXML 故事 | 内核 `SubDocument` type | Importer |
|------------|-------------------------|----------|
| Main document body | 0 | `MainDocumentImporter` |
| Header/Footer | 2 | `HdrFtrDocumentImporter` |
| Comments | 3 | `CommentsDocumentImporter` |
| Footnotes | 1 | `FootnoteDocumentImporter` |
| Endnotes | 4 | `EndnoteDocumentImporter` |
| Text boxes | 5 / 6 | Textbox importers |

`TextDocument` 用数组槽保存；共享 `TextPool` 与 `KStyles`。

---

## 3. 文本与属性

| OOXML | 内核 | 写入入口 |
|-------|------|----------|
| `w:t` 字符序列 | `ITextRope` 字符 | `CoreHelper.appendText` |
| `w:r` + `w:rPr` | `IChpxTable` 区间 + PropertySet | appendText 的 props |
| `w:p` + `w:pPr` | `IPapxTable` + PropertySet | `appendParagraph` |
| 段边界（逻辑） | 合成 `\r` / `\f` / 单元格符 | `importParagraphEnd` |
| CP（无标准名；引擎坐标） | `getLength()` / Entry.position() | 全程 |

### 3.1 为何有 CHPX/PAPX 这些名字？

它们来自 **Microsoft Word 二进制格式（.doc）** 的 Character Property eXceptions / Paragraph Property eXceptions。  
本内核用区间表表达「从某 CP 起属性为 X」，DOCX 导入时把 XML 的 rPr/pPr **投影**到这些表，而不是在 ZIP 里读写 CHPX 流。

### 3.2 Piece Table

| 概念 | 说明 |
|------|------|
| Piece Table（经典 Word） | 文本可能分段存储（Unicode/压缩等）的索引结构 |
| 本仓库 `CPieceTable` | 门面：聚合 rope+chpx+papx；IO append **不**经此写入 |

---

## 4. 样式与编号

| OOXML | 内核 |
|-------|------|
| `w:styleId` | `StyleId2Sti` 映射 |
| 内置样式固定 id | `BuildinStyleSheet` → 固定 sti（Normal=0 等） |
| `basedOn` / `next` / `link` | `KStyle` 字段（sti 整数） |
| `docDefaults` | `KStyles` 默认 rPr/pPr |
| 无样式哨兵 | sti **4095** |
| `numId` / `ilvl` | `NumIdRedirection` + PAPX 编号属性 |

---

## 5. 节

| OOXML | 内核 |
|-------|------|
| Section / `sectPr` | `PLCSection` + `SectionNode` |
| 分节边界 | 段末 `\f` + `setLastSectionEndCP` / `addSection` |
| Header/Footer 引用 | `SectionNode.set*Header/Footer` |

PLC = **P**iece **L**inked to **C**haracters（或历史「PLC」表）：在 CP 上挂非纯文本结构。本仓库 `PLC` 基类服务节/域/书签/形状锚点等。

---

## 6. PropertySet

| 概念 | 说明 |
|------|------|
| OOXML | 树形 XML 属性 |
| 内核 | 扁平 **propId → 值** 的 `PropertySet` / `MutablePropertySet` |
| 转换 | `RunPropsConverterSimple` / `ParaPropsConverterSimple` / `ConvertHelper` |
| 冻结 | `freeze()` 后不可变，便于共享与比较 |

**propId 是内核契约**，不是 OOXML 标准编号；双端对比必须一致。

---

## 7. 一张「最小段落」对照图

```text
OOXML                         内核（Main SubDocument）
─────                         ────────────────────────
<w:p>                         importParagraphStart → 记下 cp0
  <w:pPr><w:pStyle val=…/>    （稍后）PAPX props 含 sti
  <w:r>
    <w:rPr><w:b/>             CHPX props（粗体）
    <w:t>Hi</w:t>             rope += "Hi"；chpx.insert(cp0, …)
  </w:r>
</w:p>                        importParagraphEnd:
                                rope += '\r'
                                chpx for break char
                                papx.insert(cp0, paraProps)
```

---

## 8. 延后标准面（MVP 只认名）

| 标准域 | 典型 Part / 标记 | MVP |
|--------|------------------|-----|
| DrawingML | `w:drawing` / charts | stub |
| VML | `w:pict` | stub |
| Math (OMML) | `m:oMath` | stub |
| Comments | `comments.xml` | stub |
| Footnotes/Endnotes | `footnotes.xml` | stub |
| Custom XML | `customXml/` | 元数据可浅导入 |

---

## 9. 读懂检查

- [ ] 能否用本表向新人解释「为什么 DOCX 里看不到 CHPX」？  
- [ ] styleId 字符串是否允许进入 PAPX？还是必须变成 sti？  
- [ ] 最后一节没有 `\f` 时，节节点从哪里来？
