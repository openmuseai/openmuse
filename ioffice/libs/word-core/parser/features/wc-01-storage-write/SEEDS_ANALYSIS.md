# wc-01-storage-write — Seeds 核心文件解析

对照 [seeds.csv](./seeds.csv)。详解见 [docs/STORAGE.md](./docs/STORAGE.md)。

---

## 总览：写入热路径

```text
CoreHelper.appendText(doc, text, chpx)
  → textRope.append(...)
  → chpxTable.insert(oldLength, chpx)

CoreHelper.appendParagraph(doc, endMark, cp0, chpx, papx)
  → appendText(endMark, chpx)
  → papxTable.insert(cp0, papx)     // IO 路径用段首 CP

分节 endMark=='\f'
  → PLCSection.setLastSectionEndCP(length)
  →（后续）PLCSection.addSection(cp)
```

工厂只负责创建表实例；`SubDocument` 构造时调用。

---

## 1. PapxTable（order 225）

| 项 | 内容 |
|----|------|
| Java | `writer/data/internal_v2/PapxTable.java` |
| Rust | `.../PapxTable.rs` |
| depends | `CPTable`, `IPapxTable` |
| 文档 | [STORAGE §4.3](./docs/STORAGE.md#43-papxtable--段落级) |

**职责**: 段落属性区间表；`insert(cp, PropertySet)` 落 PAPX。

**Parse MVP 必须**:
- `insert` / `seek` / `last` / `begin`
- 继承 `CPTable.afterInsertText` 的 CP 平移
- `beforeAddReference` 对 prop 191/292 的样式引用（可浅实现）

**与 CoreHelper**: `appendParagraph(..., cp0, ...)` → `papx.insert(cp0, papx)`。

---

## 2. ChpxTableFactory（order 226）

| 项 | 内容 |
|----|------|
| Java | `internal_v2/ChpxTableFactory.java`（~10 行） |
| depends | `ChpxTable` |

```java
public static IChpxTable createChpxTable(Document document) {
   return document == null ? null : new ChpxTable(document);
}
```

**职责**: 唯一创建入口，避免外层依赖具体 `ChpxTable` 类。  
**MVP**: 原样移植；`SubDocument` 构造调用。

---

## 3. PapxTableFactory（order 227）

与 Chpx 对称：`createPapxTable(Document) → new PapxTable(document)`。

---

## 4. PLC（order 228）

| 项 | 内容 |
|----|------|
| Java | `writer/data/PLC.java` |
| depends | `ListIndexPositionTree`, `INotifyTextRope` |
| 文档 | [STORAGE §5](./docs/STORAGE.md#5-plc挂在-cp-上的结构表) |

**职责**: CP 上挂结构节点的基类（节/域/书签…）；文本变更时平移节点。

**Parse MVP**: `insert` / 遍历 / `isEmpty` / TextRope Listener 钩子；Transaction 命令可后补。

---

## 5. PLCSection（order 229）

| 项 | 内容 |
|----|------|
| Java | `writer/data/PLCSection.java` |
| depends | `PLC`, `PropertySet` |
| 文档 | [STORAGE §5.2](./docs/STORAGE.md#52-plcsection排版最敏感的一种) |

**Parse 热 API**:
- `setLastSectionEndCP(cp)` — 段末 `\f` 时
- `addSection(cp)` — 收到 sectPr 时
- `getFirst/LastSection`、`isEmpty`

主文档 `createPLC()` type==0 时创建。

---

## 6. CPieceTable（order 232）

| 项 | 内容 |
|----|------|
| Java | `writer/data/CPieceTable.java` |
| depends | `ITextRope`, `IChpxTable`, `IPapxTable` |
| 文档 | [STORAGE §6](./docs/STORAGE.md#6-cpiecetable门面不是-ooxml-part) |

**职责**: rope+chpx+papx 门面。**不是** append 必经路径。

**Parse MVP**: `setGuessedTotalLength`（DocxReader）、getter；深度方法后补。

---

## 7. CoreHelper（order 233）— 本切片验收枢纽

| 项 | 内容 |
|----|------|
| Java | `writer/helper/CoreHelper.java` |
| depends | rope / chpx / papx / PropertySet |
| 文档 | [STORAGE §A 热路径](./docs/STORAGE.md#写入热路径钉死) |

**必须实现**:

| 方法 | 语义 |
|------|------|
| `appendText(doc, String\|char[], props)` | 先记 length，append rope，再 `chpx.insert` |
| `appendParagraph(doc, ch, chpx, papx)` | append 段标 + papx at 段标 CP |
| `appendParagraph(doc, ch, cp, chpx, papx)` | IO 主路径：papx at **段首 cp** |
| `insertToPapxTable` | `papx.insert` |
| `setCellMark` / `supplyEndPara` | 表单元格 / 读后补段 |

**验收**: 手工调 `appendText` + `appendParagraph('\r', cp0, …)` 后 length、CHPX、PAPX 与 Java 一致。

---

## 改写顺序建议（seeds 内）

1. Factories（薄）  
2. PapxTable（依赖已有 CPTable/Chpx）  
3. PLC → PLCSection  
4. CPieceTable  
5. **CoreHelper**（联调验收）
