# TextDocument

**Java**: `cn/wps/moffice/writer/core/TextDocument.java`（~4264 行，MVP 只读子集）  
**Wave**: 2  
**相关**: [SubDocument](./SubDocument.md) · [KStyles](./KStyles.md) · [DocxReader](.../wc-05-orchestration/docs/DocxReader.md)

---

## 1. 职责

整个 Word 文档的**根对象**：持有样式表、共享 TextPool、最多 7 个子文档槽、事务、元数据、主题等。  
Parse 路径：由外部 new 空文档 → DocxReader 填充 → 读侧消费。

---

## 2. 构造与 `_create()`

```text
TextDocument() → this(false)
TextDocument(bMock):
  初始化大量字段（styles 稍后、subdocuments[7]、metadata、theme…）
  若 !bMock → _create()
  UUID / tmpDir / FileManager / openListeners

_create():
  Transaction + listeners
  styles = new KStyles(this)
  mCharBuffer = KSwappableTextPool(256K) 或 fallback KTextPool
  createAllSubDocument()
  ShapeFactoryDoc.createShapeFactory()   // MVP stub OK
```

---

## 3. Parse MVP 关注的 API

| API | 用途 |
|-----|------|
| `getStyles()` / `getTextPool()` | Importer / SubDocument |
| `getMainDocument()` / `getSubdocument(type)` / `createSubDocument(type)` | 子文档 |
| `setInIOState(true)` | DocxReader.read 标记 IO 中 |
| `setProperty` / `getProperty` | 文档级属性 |
| `getKParagraphsID()` | paraId 登记 |
| `getTransaction()` | 表与 rope 共享 |
| `getMetadata()` / `getTheme()` / `getFontTable()` | 元数据与主题 |
| `getUUSti()` 相关 | StyleId2Sti 分配 |

---

## 4. 子文档槽

`mSubdocuments = new Document[7]`，type：

| index | 用途 | MVP |
|-------|------|-----|
| 0 | Main | 必须真实写入 |
| 1 | Footnote | stub |
| 2 | Header/Footer | 可空壳 |
| 3 | Comment | stub |
| 4 | Endnote | stub |
| 5/6 | Textbox | stub |

`DocumentImporter` 会 `createSubDocument` 覆盖/确保对应槽存在。

---

## 5. MVP 改写要点

- 实现 `_create` 最小闭环：Transaction 壳、KStyles、TextPool、SubDocument(0)、数组槽。  
- 视图、媒体库、自动统计、保存加速器等全部后置。  
- 文件体积极大：改写时按 API 子集裁剪，勿追求一次搬完 4k 行。

---

## 6. 读懂检查

- [ ] `createAllSubDocument` 与随后 `createSubDocument(0)` 是否重复创建 Main？  
- [ ] `bMock=true` 时缺什么，测试能不能用？  
- [ ] IO 状态标志影响哪些路径（事件派发/校正）？
