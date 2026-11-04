# SubDocument

**Java**: `cn/wps/moffice/writer/core/SubDocument.java`（~1173 行）  
**实现**: `Document` 接口  
**Wave**: 2  
**相关**: [TextDocument](./TextDocument.md) · [09 依赖分界（必读）](./09-subdocument-own-scope.md) · [textrope](../../wc-01-storage-write/docs/STORAGE.md#3-textrope可变字符流) · [ChpxTable](../../wc-01-storage-write/docs/STORAGE.md#42-chpxtable--run-级) · [PapxTable](../../wc-01-storage-write/docs/STORAGE.md#43-papxtable--段落级) · [PLCSection](../../wc-01-storage-write/docs/STORAGE.md#52-plcsection排版最敏感的一种)

---

## 1. 职责

一个「子文档」流：独立的 TextRope 视图（共享 TextPool）、CHPX/PAPX、PieceTable、按 type 装配的 PLC 集合。  
`SubDocumentImporter` 的唯一写入目标。

**所有权**：rope / CHPX / PAPX / `CPieceTable` / `PLC` 算法属于 **wc-01**。本类在 wc-02 的职责是 **装配并拥有**。function **09** 只验收装配 I/O，不是把 ctor 里出现过的每个类型都改写成 Rust（尤其不要拉 `KRange`）。

---

## 2. 构造顺序（必须死记）

Java 完整顺序如下。**09 必做**标 ★；未标步骤可 defer / 编译壳。

```text
SubDocument(parent, nType):
  mParent / mType                                    ★
  setTransaction(parent.getTransaction())            ★
  ErrorCollector / DocumentCorrector / DocumentEventDispatcher
  mNotifyTextRope = new KNotifyTextRope()            ★
  setTextPool(parent.getTextPool())                  ★
  mTextRope = mNotifyTextRope                        ★
  mChpxTable = ChpxTableFactory.createChpxTable(this) ★  调用 wc-01
  mPapxTable = PapxTableFactory.createPapxTable(this) ★
  listeners = TextRopeListeners(this)                ★  行为由 function 08 保证
  listeners.add(chpx); listeners.add(papx)           ★
  mNotifyTextRope.setListener(listeners)             ★
  mPiecetable = new CPieceTable(this)                ★  调用 wc-01
  mTextStream = TextData.createTextStream(this)
  createPLC()                                        ★ 仅 type==0 必须 new PLCSection
  addTextListener(mShapeAnchors)
  AutoNumCount / AutoNumFieldCache / TableInfos …
  若 type==0 → AutoWordStat
```

---

## 3. `createPLC()` 按 type

| type | 额外 PLC |
|------|----------|
| 0 Main | `PLCSection` + `PLCField` |
| 1 Footnote | Field + FootnoteTxt + … |
| 2 Header | Field + PLCHeader |
| 3 Comment | Field + CommentTxt + … |
| 4 Endnote | Field + EndnoteTxt + … |
| 5/6 Textbox | Field + TextboxText |

所有 type 还会：`PLCShapeAnchor`、bookmark、documentFields；并把 PLC 注册为 text listener。

**MVP / function 09**：type=0 必须有可用的 `PLCSection`（`getPLCSection` 对齐 Java 可空）；其它 PLC **可空壳、09 不测**。type≠0 必须 **没有** `PLCSection`。

不要把 `PLCHeader` 展开成 `HelperHeaderFooter` / `KRange`——那不是 parse 对象图。收敛方案：[09-subdocument-own-scope.md](./09-subdocument-own-scope.md)。

---

## 4. Parse 写入面 API

| API | 调用方 |
|-----|--------|
| `getLength()` | ParagraphContext / CoreHelper |
| `getTextRope()` / `getChpxTable()` / `getPapxTable()` | CoreHelper |
| `getPieceTable()` | DocxReader.setGuessedTotalLength |
| `getPLCSection()` | 分节 |
| `getStyles()` | → parent.getStyles() |
| `getTextDocument()` | paraId 等 |
| `getType()` | putId |
| `getDrawingContainer()` | beginRead（可 stub） |

---

## 5. MVP 改写要点

- 构造顺序不能乱：先 rope+pool，再表，再 listener，再 PLC。  
- `getLength()` 必须等于 `textRope.length()`。  
- 大量编辑/修订/形状 API 后置；**`KRange` 整类后置**。  
- 09 编译面：保留完整 `SubDocument.rs` 交错模板；非 critical 方法暂缓实现，**Java 注释不得删**。`check-template-layout.mjs` 对 `PLCSection.rs` 必须 PASS。

---

## 6. 读懂检查

- [ ] Head 节点 `position()` 为何等于 `document.getLength()`？（见 ChpxTable.Head）  
- [ ] 删除文本时 Papx/Chpx 谁先收到 beforeRemove？  
- [ ] Main 的 `mSections` 何时非 null？
