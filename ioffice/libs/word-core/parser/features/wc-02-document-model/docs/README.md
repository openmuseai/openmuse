# 02 — 文档模型（TextDocument / SubDocument / Styles）

**Wave**: 2  
**前置**: [wc-01 STORAGE](../../wc-01-storage-write/docs/STORAGE.md)  
**下游**: Importer 持有模型实例并写入；样式由 [05](.../wc-05-orchestration/docs/README.md) 填充  
**改写顺序**: [REWRITE_FILE_ORDER.md](../REWRITE_FILE_ORDER.md) · CSV `batch_id=parse-02-document-model`（order 300+）

---

## 1. 子功能职责

组装「一篇文档」的对象图：根 `TextDocument`、按故事划分的 `SubDocument`、共享样式表与 StyleId 映射。  
本层定义 **谁拥有** rope/表/PLC，以及 create 顺序。存储算法在 wc-01；本层是对象图。`SubDocument` ctor 闭包里的类型 **不等于** feature-critical，见 [09-subdocument-own-scope.md](./09-subdocument-own-scope.md)。

## 2. 标准概念对照

| 模型组件 | OOXML / 概念 |
|----------|--------------|
| TextDocument | 整个 Package 打开后的内存文档 |
| SubDocument type=0 | Main document body |
| SubDocument type=2/3/… | Header/Comment/… 等故事 |
| KStyles / KStyle | styles.xml |
| StyleId2Sti | styleId 字符串 → 内部 sti |
| 共享 TextPool | 实现细节（非标准）；多故事共享字符池 |

见 [styles-numbering-settings.md](.../wave0-ooxml-standards/docs/styles-numbering-settings.md)。

## 3. 本目录类文档

| 文档 | 类 |
|------|-----|
| [09-subdocument-own-scope.md](./09-subdocument-own-scope.md) | function 09 与 PLC/CPieceTable/KRange 的所有权分界（防依赖发散） |
| [TextDocument.md](./TextDocument.md) | `TextDocument` |
| [SubDocument.md](./SubDocument.md) | `SubDocument` |
| [KStyles.md](./KStyles.md) | `KStyles` / `KStyle` |
| [StyleId2Sti.md](./StyleId2Sti.md) | `StyleId2Sti` |

## 4. 建议阅读顺序

1. [TextDocument](./TextDocument.md) — `_create`  
2. [SubDocument](./SubDocument.md) — 构造顺序 + `createPLC`；**依赖不要跟 ctor 闭包走** → [09-subdocument-own-scope.md](./09-subdocument-own-scope.md)  
3. [KStyles](./KStyles.md)  
4. [StyleId2Sti](./StyleId2Sti.md)

## 5. MVP 验收

- [ ] 能 new TextDocument 得到可用 Main SubDocument  
- [ ] styles / textPool / transaction 非空  
- [ ] StyleId2Sti.put/get 与内置 Normal sti 行为对齐

---

## 6. 以核心类为中心的功能实现逻辑

本层回答：**谁拥有存储结构、按什么顺序 create**。写入细节在 01/04；本层是对象图。

```text
TextDocument._create()
  ├─ Transaction
  ├─ KStyles(this)
  ├─ 共享 TextPool（KSwappableTextPool 或 KTextPool）
  └─ createAllSubDocument / createSubDocument(type)
         └─ SubDocument(parent, type)
              ├─ KNotifyTextRope + setTextPool(parent)
              ├─ ChpxTable / PapxTable
              ├─ TextRopeListeners(chpx, papx, …)
              ├─ CPieceTable
              └─ createPLC()   // type=0 → PLCSection + …
```

### 6.1 `TextDocument` — 文档根

| 职责 | 说明 |
|------|------|
| 槽位 | `mSubdocuments[7]`：0=Main 必须真写；其余 MVP 可 stub |
| 共享 | `KStyles`、`TextPool`、`Transaction` 全故事共用 |
| IO 标志 | `setInIOState(true)` 由 DocxReader 在 parse 前打开 |

详见 [TextDocument.md](./TextDocument.md)。

### 6.2 `SubDocument` — 故事流（Importer 的写入目标）

构造顺序**不能乱**：rope+pool → 表 → listener → piece table → PLC。  
`getLength()` ≡ `textRope.length()`。  
`SubDocumentImporter` 只认这一个 `mDocument`。详见 [SubDocument.md](./SubDocument.md)。

### 6.3 `KStyles` / `KStyle` — 样式表

承接 styles.xml 导入结果；sti 为内部整数 id。  
`docDefaults`、basedOn/next/link 等字段影响后续排版与属性合成。详见 [KStyles.md](./KStyles.md)。

### 6.4 `StyleId2Sti` — OOXML styleId → sti

```text
"Normal" 等内置 → BuildinStyleSheet 固定 sti（Normal=0）
文档自定义 styleId → 分配/查询 sti
无样式哨兵 → 4095
```

段落 `correctProp` 缺省写入 prop **191=0**，依赖本映射与内置表一致。详见 [StyleId2Sti.md](./StyleId2Sti.md)。

### 6.5 旁路：`TextRopeListeners` / `NumIdRedirection` / `IOHelper`

- Listeners：把表挂到 Notify rope（与 01 通知时序绑定）  
- `NumIdRedirection`：与 03/05 numbering 共享  
- `IOHelper.addNormalStyle` 等：导入前保证最小样式集  

---

## 7. Java ↔ Rust 逻辑对应

**CSV**: `batch_id=parse-02-document-model`（order 300+）  
**进度**: 全部 `pending`（依赖 01 剩余 Papx/PLC/CoreHelper 等）

| Java 类 | Rust 路径 | 角色 |
|---------|-----------|------|
| `Document` | `…/writer/core/Document.rs` | 子文档接口 |
| `SubDocument` | `…/SubDocument.rs` | 故事实现 |
| `TextDocument` | `…/TextDocument.rs` | 文档根 |
| `KStyle` / `KStyles` | `…/KStyle.rs` · `…/KStyles.rs` | 样式 |
| `StyleListener*` | `…/StyleListener*.rs` | 表↔样式引用 |
| `TextRopeListeners` | `…/TextRopeListeners.rs` | rope 监听聚合 |
| `StyleId2Sti` | `…/docxReader/importer/StyleId2Sti.rs` | id 映射 |
| `BuildinStyleSheet` / `WordBuildinStyleNames` | 同 importer/helpers | 内置 sti |
| `NumIdRedirection` | `…/NumIdRedirection.rs` | 编号 id |
| `IOHelper` | `…/io/reader/helper/IOHelper.rs` | 导入辅助子集 |

### 7.1 逻辑对齐要点

| 语义 | 必须与 Java 一致 |
|------|------------------|
| `_create` 顺序 | Transaction → Styles → Pool → SubDocuments |
| SubDocument 构造顺序 | rope → chpx/papx → listeners → piece → PLC |
| Main type | `0`；`DocumentImporter` 会 `createSubDocument(0)` |
| StyleId2Sti | Normal→0；哨兵 4095 |
| 共享 TextPool | 多 SubDocument 同一 pool 实例 |

### 7.2 改写策略

`TextDocument` Java 体积极大：只搬 Parse 用到的 API 子集（见类文档 §3），视图/媒体/统计一律后置。
