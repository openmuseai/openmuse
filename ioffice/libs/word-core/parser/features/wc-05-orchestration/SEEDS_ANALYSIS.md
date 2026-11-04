# wc-05-orchestration — Seeds 核心文件解析

对照 [seeds.csv](./seeds.csv)。Numbering/Settings 类文档在 [docs/](./docs/)，本 CSV 种子未单列它们，但 DocumentImporter depends 会拉入。

---

## 编排流

```text
DocxReader.read()
  → guessLength / setInIOState
  → xwpf.setDocumentImporter(DocumentImporter)
  → xwpf.parse()
       → StylesImporter（styles.xml）
       → onImportParagraph* → MainDocumentImporter
  → FinishReadingWork.go()
       → CoreHelper.supplyEndPara
       → addNormalStyle / onMainDocumentEnd
```

---

## 1. IOStyle（order 600）

styles.xml 单条样式的**草稿对象**（尚未 `KStyles.add`）。

| 字段 | 含义 |
|------|------|
| mType / mStyleId / mName / mBasedOn / mNext / mLink | OOXML 关系 |
| mIsCustom / mQFormat / mUiPriority | UI/自定义 |
| mCorePrSet | 合并后的属性 |
| mCoreId / mCoreType | `convertTypeAndId` 后写入 |

`convertTypeAndId(StyleId2Sti)` → `put` 分配 sti；type：paragraph=1, character=2, table=3, numbering=4。

---

## 2. StylesImporter（order 601）

两阶段：收集 `IOStyle` 列表 → `importStylesEnd` 再 `createKStyle` + `mStyles.add`。  
另：`importDocDefaults` → rPr/pPr Default。

文档：[docs/StylesImporter.md](./docs/StylesImporter.md)

---

## 3. DocumentImporter（order 616）— 总线

- 构造：createSubDocument + 各 *DocumentImporter + Styles/Numbering/Settings  
- `getSubDocumentImporter(SubDocType)` 路由  
- 转发段落/样式/编号/设置回调  

**MVP**: MAIN_DOCUMENT + Styles；其它 SubDocType stub。

文档：[docs/DocumentImporter.md](./docs/DocumentImporter.md)

---

## 4. FinishReadingWork（order 617）

`go()`: endDrawingContainer → `correctCoreData`（含 **supplyEndPara**）→ addNormalStyle → `onMainDocumentEnd`。

文档：[docs/FinishReadingWork.md](./docs/FinishReadingWork.md)

---

## 5. DocxReader（order 619）— 入口

构造即 `new DocumentImporter(..., mode=0, xml=null)`。  
`read()` 顺序见 [docs/DocxReader.md](./docs/DocxReader.md)。

depends：XWPFDocument / IDocumentImporter（core-libs，wave0 已改）。

---

## 验收

简单 docx → parse → 主文档文本 + CHPX/PAPX + styles 与 Java JSON 对照。

## 改写顺序

IOStyle → StylesImporter → DocumentImporter（Main 路由）→ FinishReadingWork → DocxReader
