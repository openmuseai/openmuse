# wc-02-document-model — Seeds 核心文件解析

对照 [seeds.csv](./seeds.csv)。

---

## 总览：对象图

```text
TextDocument
  ├─ Transaction / TextPool / KStyles
  ├─ SubDocument[0] Main  ──┬─ KNotifyTextRope + TextRopeListeners
  │                         ├─ ChpxTable / PapxTable (Factory)
  │                         ├─ CPieceTable
  │                         └─ PLC* (含 PLCSection)
  ├─ SubDocument[2..] 其它故事（MVP stub）
  └─ StyleId2Sti / NumIdRedirection（importer 共享，种子在本 feature）
```

---

## 1. Document（order 300）

**接口**；`SubDocument` 实现。Importer / CoreHelper 通过它取 `getLength` / `getTextRope` / `getChpxTable` / `getPapxTable` / `getPLCSection`。

**MVP**: 写入面 API 齐；编辑/修订后置。

---

## 2. KStyle（order 301）

单条样式：sti、name、basedOn、next、type、PropertySet。  
见 [docs/KStyles.md](./docs/KStyles.md)。

---

## 3. KStyles（order 302）

样式表：`add` / `getStyleById` / `setRPrDefault` / `setPPrDefault`。  
`IOHelper.addNormalStyle` 保证 Normal(sti=0)。

---

## 4. TextRopeListeners（order 306）

```text
implements INotifyTextRope.Listener
  → 转发 before/after Insert/Remove 到已注册 Listener 列表
  → IO 状态且非 userAction 时 isDisabled() 跳过（加速导入）
```

**装配**: SubDocument 构造中 `listeners.add(chpx); add(papx); …` 再 `setListener(listeners)`。

**MVP**: 转发顺序与 Java 一致（before 逆序、after 正序）；IO disable 语义保留。

---

## 5. SubDocument（order 307）— 本切片枢纽

| depends（摘） | 作用 |
|---------------|------|
| KNotifyTextRope + TextRopeListeners | 文本 + 通知 |
| Chpx/Papx Factory | 属性表 |
| CPieceTable / PLC / PLCSection | 门面与节 |
| KStyles | 经 parent |

**构造顺序不可乱**（见 [docs/SubDocument.md](./docs/SubDocument.md)）。  
type==0 必须创建 `PLCSection`。

---

## 6. TextDocument（order 308）

`_create()`: Transaction → KStyles → TextPool → `createAllSubDocument`。  
IO：`setInIOState`、`getMainDocument`、`createSubDocument`、`getKParagraphsID`。

文档：[docs/TextDocument.md](./docs/TextDocument.md)

---

## 7. WordBuildinStyleNames（order 309）

内置样式显示名常量表（Normal、Heading1…）。BuildinStyleSheet 查表用。

---

## 8. BuildinStyleSheet（order 310）

name → 固定 `(type, sti)`；保留名冲突处理。  
`StyleId2Sti.put(styleId, name, type, isCustom)` 优先用内置 sti。

---

## 9. StyleId2Sti（order 311）

OOXML `styleId` 字符串 → 内部 sti。  
`toSti(null)→4095`。文档：[docs/StyleId2Sti.md](./docs/StyleId2Sti.md)

---

## 10. IOHelper（order 312）— 子集

标签 `subset-addNormalStyle`：**至少**实现 `addNormalStyle(KStyles)`。  
`onMainDocumentStart` / `FinishReadingWork` 各调一次。

---

## 11. NumIdRedirection（order 313）

```text
OOXML numId → UUNumberingId.getNextId() → 内核 id
getId / put
```

段落转换器与 NumberingImporter 共享；本 feature 先落映射表，编号内容在 wc-05。

---

## 改写顺序建议

1. Document 接口壳  
2. KStyle / KStyles / Buildin* / StyleId2Sti / NumIdRedirection  
3. TextRopeListeners  
4. SubDocument 构造  
5. TextDocument._create  
6. IOHelper.addNormalStyle
