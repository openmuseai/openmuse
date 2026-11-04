# DOCX Document Parse MVP — 代码分析手册（改写前必读）

**状态**: 仅文档，不实现  
**日期**: 2026-07-18（归档于 `libs/word-core/parser/`，按子功能分目录）  
**目标**: 读懂 Java 解析主路径后，再按 Wave 改写到 `libs/word-core`  
**MVP 产物**: DOCX 只读 → 填充 `TextDocument` / `MainDocument(SubDocument)`：正文文本、CHPX/PAPX、样式、节 PLC  

**明确延后**: DOC/RTF/HTML、完整 DML/VML/公式/批注/脚注、回写、排版/缓存/渲染  

**索引**: [README.md](./README.md)  
**标准术语（必读）**: [00-ooxml-standards](./features/wave0-ooxml-standards/docs/README.md)  
**子功能目录**: [01-storage](./features/wc-01-storage-write/docs/STORAGE.md) · [02-document-model](./features/wc-02-document-model/docs/README.md) · [03-property-convert](./features/wc-03-property-convert/docs/README.md) · [04-paragraph-import](./features/wc-04-paragraph-import/docs/README.md) · [05-orchestration](./features/wc-05-orchestration/docs/README.md)

---

## 0. 总览：一条段落如何落进模型

```
DOCX (OPC ZIP)
  → XWPFDocument.parse()                    [core-libs wps-io，SAX/事件驱动]
      → IDocumentImporter 回调              [DocumentImporter]
          → Styles / Numbering / Settings
          → onImportParagraphStart / RunText / ParagraphEnd
              → SubDocumentImporter (MainDocumentImporter)
                  → CoreHelper.appendText / appendParagraph
                      → ITextRope.append
                      → IChpxTable.insert(cp, chpx)
                      → IPapxTable.insert(cp, papx)
                      → (可选) PLCSection.setLastSectionEndCP / addSection
```

**最小闭环（改写验收口径）**:

1. 文本进 rope，文档长度正确  
2. 每个 run 有 CHPX；每个段末有 PAPX  
3. styles.xml 映射进 `KStyles` + `StyleId2Sti`  
4. 分节符 `\f` 更新 `PLCSection`  
5. 能 dump / JSON 对比主文档文本 + 属性桶（与 Java 侧一致）

---

## 1. Wave 划分与阅读顺序

| Wave | 子功能目录 | 先读什么 | 改写优先级 |
|------|------------|----------|------------|
| — | [00-ooxml-standards](./features/wave0-ooxml-standards/docs/README.md) | 标准术语与映射 | 前置词汇 |
| 0 | （见总览 §2） | 依赖与 stub 策略 | 决策，不写业务 |
| 1 | [01-storage](./features/wc-01-storage-write/docs/STORAGE.md) | textrope + Papx/Chpx + PLC | **最高** |
| 2 | [02-document-model](./features/wc-02-document-model/docs/README.md) | Document / SubDocument / KStyles | 写入面 API |
| 3 | [03-property-convert](./features/wc-03-property-convert/docs/README.md) | ConvertHelper + Run/Para *Simple | OOXML → PropertySet |
| 4 | [04-paragraph-import](./features/wc-04-paragraph-import/docs/README.md) | SubDocumentImporter 热路径 | 段落状态机 |
| 5 | [05-orchestration](./features/wc-05-orchestration/docs/README.md) | DocxReader / DocumentImporter | 编排入口 |
| 6 | （总览 §8） | 联调与双端对比 | 验证 |

**建议阅读路径（一天内扫完主路径）**:

0. [00-ooxml-standards](./features/wave0-ooxml-standards/docs/README.md) — OPC / 段落·Run / 样式·编号 / 节 / 映射表  
1. [DocxReader](./features/wc-05-orchestration/docs/DocxReader.md) + 源码 ~161 行  
2. [DocumentImporter](./features/wc-05-orchestration/docs/DocumentImporter.md) — 构造 + 路由 + `onImportParagraph*`  
3. [SubDocumentImporter](./features/wc-04-paragraph-import/docs/SubDocumentImporter.md) — Start/Run/End  
4. [CoreHelper](./features/wc-01-storage-write/docs/STORAGE.md#写入热路径钉死) — `appendText` / `appendParagraph`  
5. [SubDocument](./features/wc-02-document-model/docs/SubDocument.md) — 构造 + `createPLC`  
6. [textrope](./features/wc-01-storage-write/docs/STORAGE.md#3-textrope可变字符流) / [PapxTable](./features/wc-01-storage-write/docs/STORAGE.md#43-papxtable--段落级) / [ChpxTable](./features/wc-01-storage-write/docs/STORAGE.md#42-chpxtable--run-级)  
7. [StylesImporter](./features/wc-05-orchestration/docs/StylesImporter.md) + [Run](./features/wc-03-property-convert/docs/RunPropsConverterSimple.md)/[Para](./features/wc-03-property-convert/docs/ParaPropsConverterSimple.md) converters  

完整索引见 [README.md](./README.md)。

---

## 2. Wave 0 — 依赖缺口与 stub 策略

### 2.1 调用边界

| 层 | 包 | 职责 | MVP |
|----|-----|------|-----|
| OPC/SAX | `cn.wps.io.xwpf.*` (core-libs) | 解包、按 part 回调 | 复用/薄 stub；可另开纯文本注入测试路径 |
| 编排 | `docxReader.DocxReader` + `DocumentImporter` | 创建 importer、调 parse、收尾 | 必须实现编排壳 |
| 写文档 | `SubDocumentImporter` + `CoreHelper` | 真正写入 rope/表 | **核心** |
| 属性 | `cn.wps.moffice.property.*` | PropertySet 桶 | util 侧必须有可用实现 |
| 模型 | `writer.core` + `writer.data` | TextDocument/SubDocument/PLC | IO 写入子集 |

### 2.2 依赖矩阵（改写时对照）

| 依赖 | Java 位置 | Rust 现状（概要） | 策略 |
|------|-----------|-------------------|------|
| `IOListener` | `writer/core/io` | trait 已有 | 保留；进度回调可 no-op |
| `ImporterBase` | `writer/io/reader` | skeleton + `todo!()` | **先实现** mode / xml07/03 访问 |
| `PropertySet` / `MutablePropertySet` | util `cn.wps.moffice.property` | 缺完整 src | **必须可用**（桶 map + freeze） |
| `XWPFDocument` / `IDocumentImporter` | core-libs wps-io | trait/骨架 | 事件接口对齐；parse 驱动可后补 |
| `textrope/*` | `collections/textrope` | 常缺目录 | Wave1 全模块 |
| `PLC` / `CPieceTable` / Papx/Chpx | `writer/data` | 大量 `todo!()` | 先通 append 路径 |
| Drawing/Math/Comment 回调 | importer 各处 | N/A | **签名完整 no-op** |

### 2.3 Stub 三档

1. **Must work**: appendText、PAPX/CHPX、样式 id、节 PLC、DocxReader 编排  
2. **签名完整 no-op**: footnote/endnote/comment/textbox/drawing/math/field 复杂路径  
3. **测试旁路**: 若 XWPF parse 未就绪，可用 `read_from_plain_text` / 手工调 importer 注入段落，供 Wave6 对比种子  

---

## 3. Wave 1 — 存储层：textrope / Papx / Chpx / PLC

**详解**: [STORAGE.md](./features/wc-01-storage-write/docs/STORAGE.md)（textrope / Chpx / Papx / PLC / CPieceTable / CoreHelper 合订）

### 3.1 子功能：文本绳 (textrope)

**作用**: 文档字符的可变序列；CP（character position）是全局坐标。

**必读文件**:

| 文件 | 读什么 |
|------|--------|
| `koffice/writer/core/cn/wps/collections/textrope/ITextRope.java` | 接口契约：`append` / `insert` / `erase` / `length` / `charAt` / `getText` |
| `ITextPool.java` / `KTextPool.java` | 字符底层池 |
| `KNotifyTextRope.java` | SubDocument 实际使用的 rope；带 Listener 通知 |
| `INotifyTextRope.java` | 文本变更监听（Chpx/Papx/PLC 依赖） |
| `KTextRope.java` / `KTextPieceTable.java` | 分片实现细节（append 热点） |
| `TextRopeFactory.java` | 工厂入口 |

**核心逻辑**:

- `SubDocument` 构造时：`mNotifyTextRope.setTextPool(parent.getTextPool())`，再挂 `TextRopeListeners`（含 chpx/papx）。  
- IO 路径几乎只走 **尾部 append**（`CoreHelper.appendText`），不走中间 insert。  
- MVP 可先保证：`append(String|char[])` + `length()` + `charAt` + 通知 Listener 的 `afterInsert`/`beforeRemove` 语义与 Java 一致。

**改写注意**:

- `KSwappableTextPool`（磁盘换页）可先降为内存 `KTextPool`（TextDocument._create 已有 fallback）。  
- Transaction / undo 命令链可延后，但 Listener 钩子要留，否则 Papx/Chpx 不同步。

### 3.2 子功能：CHPX 表（字符属性）

**作用**: 按 CP 切 run；每个 run 一段 `PropertySet`。

**必读文件**:

| 文件 | 读什么 |
|------|--------|
| `writer/data/IChpxTable.java` | `insert(cp, PropertySet)`、`seek`、`begin/end/last` |
| `writer/data/internal_v2/ChpxTable.java` | 真实实现 |
| `writer/data/internal_v2/ChpxTableFactory.java` | 创建入口 |
| `writer/data/internal_v2/CPTable.java`（若存在） | 共用区间树/链表基类 |

**写入热点** (`CoreHelper.appendText`):

```text
positionInserted = document.getLength()   // append 前长度 = 新文本起始 CP
textRope.append(...)
chpxTable.insert(positionInserted, props)
```

即：**先写字符，再在「旧 length」处插入 CHPX 节点**，覆盖刚 append 的区间。

**MVP**: `insert` / `seek` / `last` / 文本删除时的 `beforeRemoveText` 同步；跨 piece 行为见 `IChpxTable.RUN_CAN_SPAN_ACROSS_PIECE`。

### 3.3 子功能：PAPX 表（段落属性）

**作用**: 段落边界表；段末符（`\r` / `\f` / 单元格符等）的 CP 上挂段落属性。

**必读文件**:

| 文件 | 读什么 |
|------|--------|
| `writer/data/IPapxTable.java` | 与 Chpx 对称的 Entry API |
| `writer/data/internal_v2/PapxTable.java` | `insert`；`beforeAddReference` 会加样式引用（prop 191/292） |
| `PapxTableFactory.java` | 工厂 |

**写入热点** (`CoreHelper.appendParagraph`):

```text
positionInserted = appendText(document, ch, chpx)  // 写入段末字符 + CHPX
return papxTable.insert(positionInserted, papx)    // PAPX 落在段末 CP
```

另有重载：已知 `cp` 时先 append 字符，再 `insertToPapxTable(document, cp, papx)`（`importParagraphEnd` 用 `mCurrentParagraph.mPositionInserted`）。

### 3.4 子功能：CPieceTable

**作用**: 对 rope + chpx + papx 的门面（形状判断、遍历等），**不是** append 必经路径。

**必读**: `writer/data/CPieceTable.java`  
**MVP**: 保留字段与 `getTextRope/getChpxTable/getPapxTable`；`setGuessedTotalLength`（DocxReader 预估长度）需能调用。深度方法可后补。

### 3.5 子功能：PLC 基类与 PLCSection

**作用**: 在 CP 上挂结构化节点（节、域、书签、形状锚点…）。主文档 `createPLC()` 对 type==0 会挂 `PLCSection` + `PLCField`。

**必读文件**:

| 文件 | 读什么 |
|------|--------|
| `writer/data/PLC.java` | 基类：Head/Node、与 TextRope Listener、Transaction |
| `writer/data/PLCSection.java` | `addSection(cp)`、`setLastSectionEndCP(cp)` |
| `SubDocument.createPLC()` | 按 `mType` 装配哪些 PLC |

**段落结束与节的关系** (`SubDocumentImporter.importParagraphEnd`):

- 若 `endMark == '\f'`（分节符）：`plcSection.setLastSectionEndCP(document.getLength())`  
- `MainDocumentImporter.doImportSectionPropertiesSimple`：用 `mCpLastSectionEnd` 调 `plcSection.addSection(...)`，并挂页眉页脚节点（MVP 可先空 header/footer）

**MVP**: Main 文档的 `PLCSection` 读写；其它 PLC 创建但回调 no-op。

### 3.6 Wave1 关键文件清单（按读序）

```
collections/textrope/ITextRope.java
collections/textrope/ITextPool.java
collections/textrope/INotifyTextRope.java
collections/textrope/KNotifyTextRope.java
collections/textrope/KTextPool.java
writer/data/IChpxTable.java
writer/data/IPapxTable.java
writer/data/internal_v2/ChpxTable.java
writer/data/internal_v2/PapxTable.java
writer/data/CPieceTable.java
writer/data/PLC.java
writer/data/PLCSection.java
writer/helper/CoreHelper.java          // appendText / appendParagraph 仅约 120–180 行
```

---

## 4. Wave 2 — 文档模型：TextDocument / SubDocument / KStyles

**子功能目录**: [02-document-model/README.md](./features/wc-02-document-model/docs/README.md)  
**单类详解**: [TextDocument](./features/wc-02-document-model/docs/TextDocument.md) · [SubDocument](./features/wc-02-document-model/docs/SubDocument.md) · [KStyles](./features/wc-02-document-model/docs/KStyles.md) · [StyleId2Sti](./features/wc-02-document-model/docs/StyleId2Sti.md)

### 4.1 TextDocument 初始化

**必读**: `writer/core/TextDocument.java` — 构造函数、`_create()`、`createAllSubDocument` / `createSubDocument`

**`_create()` 关键步骤**:

1. `Transaction` + listeners  
2. `styles = new KStyles(this)`  
3. `mCharBuffer` = SwappableTextPool 或 `KTextPool`  
4. `createAllSubDocument()` — 预创建 7 个子文档槽  
5. ShapeFactory（MVP 可 stub）

**IO 相关状态**: `setInIOState(true)`（DocxReader.read）、`setProperty`、`getMainDocument()`、`getKParagraphsID()`（paraId 映射）。

**MVP 子集**: 构造、styles、text pool、subdocument 数组、transaction 壳、metadata 壳；视图/媒体/主题深度可后补。

### 4.2 SubDocument 构造与写入面

**必读**: `writer/core/SubDocument.java` 构造函数（约 121–159 行）、`getLength`/`getTextRope`/`getChpxTable`/`getPapxTable`/`getPLCSection`、`createPLC`

**构造顺序（必须理解）**:

```
NotifyTextRope + TextPool
→ ChpxTable / PapxTable
→ TextRopeListeners(chpx, papx, …)
→ CPieceTable
→ TextStream
→ createPLC()          // type 依赖
→ ShapeAnchors listener
→ AutoNum* / TableInfos / (main) AutoWordStat
```

**SubDocType ↔ type int**（DocumentImporter 创建时）:

| type | 用途 | MVP |
|------|------|-----|
| 0 | Main | **必须** |
| 2 | Header/Footer | stub 空文档即可 |
| 3 | Comment | stub |
| 5/6 | Textbox | stub |
| 1/4 | Footnote/Endnote | stub |

### 4.3 Document 接口

**必读**: `writer/core/Document.java`（接口）  
Importer 通过 `SubDocument` 实现的 `Document` API 取表与长度。

### 4.4 KStyles / KStyle / StyleId2Sti

**必读**:

| 文件 | 作用 |
|------|------|
| `writer/core/KStyles.java` | 样式表；`setRPrDefault`/`setPPrDefault`/`add`/`getStyleById` |
| `writer/core/KStyle.java` | 单样式：id/name/base/next/type/PropertySet |
| `docxReader/importer/StyleId2Sti.java` | OOXML `styleId` 字符串 → 内部 sti |
| `docxReader/importer/helpers/BuildinStyleSheet.java` | 内置名/保留索引 |

**IOHelper.addNormalStyle**: `onMainDocumentStart` 时保证 Normal 存在。

### 4.5 Wave2 关键文件清单

```
writer/core/Document.java
writer/core/SubDocument.java          // 构造 + get* + createPLC
writer/core/TextDocument.java         // _create + createSubDocument
writer/core/KStyles.java
writer/core/KStyle.java
writer/io/reader/helper/IOHelper.java // addNormalStyle 等
writer/io/reader/docxReader/importer/StyleId2Sti.java
```

---

## 5. Wave 3 — 属性转换：ConvertHelper + Run/Para *Simple

**子功能目录**: [03-property-convert/README.md](./features/wc-03-property-convert/docs/README.md)  
**单类详解**: [ConvertHelper](./features/wc-03-property-convert/docs/ConvertHelper.md) · [RunPropsConverterSimple](./features/wc-03-property-convert/docs/RunPropsConverterSimple.md) · [ParaPropsConverterSimple](./features/wc-03-property-convert/docs/ParaPropsConverterSimple.md) · [ImporterBase](./features/wc-03-property-convert/docs/ImporterBase.md)

### 5.1 子功能：ConvertHelper

**作用**: OOXML Attributes → 基础类型 / 部分复杂 value 对象；被几乎所有 importer 调用。

**必读**: `docxReader/importer/helpers/ConvertHelper.java`（~1100 行）

**阅读方法**: 不要线性通读；按调用点搜：

- `getString` / `getBoolean` / `toBoolean` / `toBoolean4NotNull`  
- `toBRC` / `convertIns` / 颜色与度量  
- `getString(attributes, "paraId")` 等固定 attr  

**MVP**: 主路径用到的 attr 解析 + 度量转换；冷门 value 类型可后补。

### 5.2 子功能：RunPropsConverterSimple

**作用**: run 级 XML 元素（elementId hash）→ `MutablePropertySet` 字符属性。

**必读**: `RunPropsConverterSimple.java` — `convertRunPropsBase(...)` 的巨大 `switch (elementId)`

**模式**:

```text
elementId (OOXML 本地名 hash)
  → dst.setBoolean(propId, …) / setValue(propId, …) / 专用 setRFonts / setVerticalAlign …
```

**依赖**: `ImporterBase`（xml03/07 分支）、`StyleId2Sti`、`ConvertHelper`、`data.value.*`（Toggle、LID、Revision…）

**改写策略**:

1. 先移植 switch 骨架 + 高频属性（字体、字号、粗斜体、颜色、下划线、竖排对齐、rStyle）  
2. 修订/特效/东亚版式等可按测试文档增量补  
3. **propId 数字必须与 Java 一致**（后续 JSON 对比依赖）

### 5.3 子功能：ParaPropsConverterSimple

**作用**: 段落 XML → `MutablePropertySet`；另有 `correctProp` 在段末落库前纠偏。

**必读**:

- `convertProp(dst, element, attributes, styleId2Sti, numIdRedirection)`  
- `correctProp(MutablePropertySet dst)` / `correctProp(doc, styles, …)`  
- `setStyle` / `setAlignment` / `convertParaIndentation` / numbering 相关  

**段末调用链** (`importParagraphEnd`):

```text
paragraph.getParaProp() / getRunProp()
→ 写入 paraId (prop 282) + KParagraphsID
→ ParaPropsConverterSimple.correctProp(dstParaProps)
→ (表格) ImporterHelper.amendTableProps
→ CoreHelper.appendParagraph(...)
```

### 5.4 ShareConvertHelper / ImporterHelper

**必读（按需）**:

- `ShareConvertHelper.java` — 样式 uiPriority 等共享解析  
- `ImporterHelper.java` — `amendTableProps`、`getCpSubDocumentEnd`  

### 5.5 Wave3 关键文件清单

```
helpers/ConvertHelper.java
helpers/RunPropsConverterSimple.java
helpers/ParaPropsConverterSimple.java
helpers/ShareConvertHelper.java
helpers/ImporterHelper.java
writer/io/reader/ImporterBase.java
```

---

## 6. Wave 4 — 写入状态机：SubDocumentImporter / MainDocumentImporter

**子功能目录**: [04-paragraph-import/README.md](./features/wc-04-paragraph-import/docs/README.md)  
**单类详解**: [ParagraphContext](./features/wc-04-paragraph-import/docs/ParagraphContext.md) · [SubDocumentImporter](./features/wc-04-paragraph-import/docs/SubDocumentImporter.md) · [MainDocumentImporter](./features/wc-04-paragraph-import/docs/MainDocumentImporter.md) · [CoreHelper](./features/wc-01-storage-write/docs/STORAGE.md#写入热路径钉死)

### 6.1 ParagraphContext（段落会话）

**必读**: `writer/io/reader/helper/ParagraphContext.java`

| 字段 | 含义 |
|------|------|
| `mPositionInserted` | 段开始时的 CP（`getLength()`），用于 PAPX insert |
| `mParaId` | OOXML paraId |
| `mEntry` | 段结束后的 `IPapxTable.Entry` |

### 6.2 SubDocumentImporter — 主路径状态机

**必读**: `docxReader/importer/document/SubDocumentImporter.java`

#### 生命周期方法（MVP 必懂）

| 方法 | 逻辑摘要 |
|------|----------|
| 构造 | 绑定 `SubDocument`、`StyleId2Sti`、`NumIdRedirection`；可选 `drawingContainer.beginRead()` |
| `importParagraphStart(paraId)` | `mCurrentParagraph = new ParagraphContext(getLength())`；记 paraId |
| `importRunText` / `importRunTextSimple` | `CoreHelper.appendText`；若域 begin 未定类型则用文本猜 FieldType（MVP 可保留调用） |
| `importParagraphEnd(paragraph, layerTable)` | 取 endMark + para/run props → correctProp → appendParagraph → 节/编号副作用 |
| `importTableStart` | 嵌套表层：可能补空段（prop 224 等） |
| `importTableCellStart/EndSimple` | 单元格标记与 `setCellMark` |
| `doImportTableRowEndSimple` | 行结束符 + table props |

#### `importParagraphEnd` 逐步分解

1. `endMark = paragraph.getEndMark()`（`\r` 普通段，`\f` 分节等）  
2. 合并 `paraId` → prop **282**；登记 `KParagraphsID`  
3. `ParaPropsConverterSimple.correctProp`  
4. `layerTable >= 1` → `ImporterHelper.amendTableProps`  
5. `CoreHelper.appendParagraph(doc, endMark, mPositionInserted, runProps, paraProps)`  
6. 回写 `mEntry`；若有 paraId → `putId`  
7. `'\f'` → `PLCSection.setLastSectionEndCP(length)`  
8. `isContainNumbering` → 可能触发 numbering part 懒解析（xml07）  

#### MVP 可 stub 的方法族

- Hyperlink / FldSimple / ComplexField  
- Drawing / DML / Math / Bookmark / Comment ref  
- `importRunText4InlineChart`  

保留空实现，避免 XWPF 回调 NPE。

### 6.3 MainDocumentImporter — 主文档增量

**必读**: `docxReader/importer/document/MainDocumentImporter.java`

| 增量 | 逻辑 |
|------|------|
| `importParagraphStart` | `checkQuit`（ForceQuit / ReadingLimit）→ 域已结束则 `lock` → super |
| `importParagraphEnd` | super → 若 `\f` 记 `mCpLastSectionEnd` → `unlock` |
| `convertSectPr` / `doImportSectionPropertiesSimple` | 节属性 + `PLCSection.addSection` + 页眉页脚节点 |
| Comment 相关 | MVP no-op / 最小壳 |

**锁**: 与复杂域未结束时的写锁相关；无域文档路径上 `fieldsHaveEnded()` 恒真。

### 6.4 CoreHelper — 唯一推荐写入入口

**必读**: `writer/helper/CoreHelper.java` 中：

- `appendText`（String / char[]）  
- `appendParagraph`  
- `insertToPapxTable`  
- `setCellMark`  
- `supplyEndPara`（收尾补段）  

**不要**在 importer 里直接改 rope/表而不走 CoreHelper（除非对标 Java 例外路径）。

### 6.5 Wave4 关键文件清单

```
importer/document/SubDocumentImporter.java     // 全文，重点 92–470
importer/document/MainDocumentImporter.java    // 全文 ~216 行
writer/io/reader/helper/ParagraphContext.java
writer/helper/CoreHelper.java                  // append*
xwpf/element_handler/ParagraphSimple           // endMark / getParaProp（core-libs）
```

---

## 7. Wave 5 — 编排：Styles / Numbering / Settings / DocumentImporter / DocxReader

**子功能目录**: [05-orchestration/README.md](./features/wc-05-orchestration/docs/README.md)  
**单类详解**: [DocxReader](./features/wc-05-orchestration/docs/DocxReader.md) · [DocumentImporter](./features/wc-05-orchestration/docs/DocumentImporter.md) · [StylesImporter](./features/wc-05-orchestration/docs/StylesImporter.md) · [NumberingImporter](./features/wc-05-orchestration/docs/NumberingImporter.md) · [SettingsImporter](./features/wc-05-orchestration/docs/SettingsImporter.md) · [FinishReadingWork](./features/wc-05-orchestration/docs/FinishReadingWork.md)

### 7.1 DocxReader — 入口

**必读**: `docxReader/DocxReader.java`（完整）

```text
read():
  ShapeIDTool / TextboxInfoTool reset
  readExtendedProperties()
  mainDocument.pieceTable.setGuessedTotalLength(guessLength)
  setInIOState(true)
  ioListener.onLoadParas(0)
  xwpf.setDocumentImporter(importer)
  xwpf.parse()                          // 事件风暴
  importMetaData()                      // core/custom props
  FinishReadingWork.go()
  importer.importCustomXmls()
```

**构造**: 直接 `new DocumentImporter(doc, listener, dlProvider, true, null, 0, null, null)` —— mode=0，xml context 可空（多数路径走默认）。

### 7.2 DocumentImporter — 总线

**必读**: `DocumentImporter.java`

**构造职责**:

1. 基类持有 `KStyles` + `StyleId2Sti`  
2. 创建 Settings / FontTable / Numbering / Theme importers  
3. `createSubDocument(0/2/3/5/6)` 并包装为各类 `*DocumentImporter`  
4. 共享 `NumIdRedirection`

**路由**: `getSubDocumentImporter(SubDocType)` → 对应子 importer。

**段落相关转发**（模式相同）:

```text
onImportParagraphStart(type, attrs)
  → getSubDocumentImporter(type).importParagraphStart(paraId)

onImportParagraphEnd(paragraph, type, layerTable)
  → sub.importParagraphEnd(...)
```

**样式转发**:

```text
importDocDefaults / importStyleStart / Child / End / importStylesEnd
  → StylesImporter
```

**编号 / 设置**: 同理转发到 `NumberingImporter` / `SettingsImporter`。

**MVP**: 实现 MAIN_DOCUMENT 路径 + styles +（最小）settings；其它 SubDocType 返回 stub importer。

### 7.3 StylesImporter

**必读**: `StylesImporter.java`

| 阶段 | 行为 |
|------|------|
| `importDocDefaults` | `KStyles.setRPrDefault` / `setPPrDefault` |
| `importStyleStart` | 新建 `IOStyle`，读 type/styleId/customStyle |
| `importStyleChild` | name/basedOn/next/link/qFormat/uiPriority/tblStylePr |
| `importStyleEnd` | `convertTypeAndId`；合并 StylePrModel → PropertySet；入 list |
| `importStylesEnd` | 遍历 list → `createKStyle` → `mStyles.add`；解析 base/next/link sti |

**StyleId2Sti**: 字符串 id 延迟分配内部 sti；`toSti(null) → 4095`（nil style）。

### 7.4 NumberingImporter（MVP 最小）

**必读**: `importer/num/NumberingImporter.java`（入口方法）+ `NumIdRedirection.java`

**与正文关系**: 段落 PAPX 含编号属性（234/235）或样式继承时，`isContainNumbering` 可能触发 numbering part 懒加载。

**MVP**: 能 ingest numbering.xml 并填充 redirection；复杂 lvl 可后补，但不要让段落路径崩溃。

### 7.5 SettingsImporter（MVP 最小）

**必读**: `importer/settings/SettingsImporter.java`

写入 `TextDocument` 文档级设置；MVP 保证 `importSettingsStart/Child/End` 不炸，关键开关按对比需要增量。

### 7.6 FinishReadingWork

**必读**: `writer/io/reader/tools/FinishReadingWork.java`  
读完后的修补（补结束段、清理 importer 状态等）。对照 Java 决定 MVP 是否调用 `CoreHelper.supplyEndPara`。

### 7.7 Wave5 关键文件清单

```
docxReader/DocxReader.java
importer/DocumentImporter.java                 // 构造 + getSubDocumentImporter + onImport*
importer/StylesImporter.java
importer/StyleId2Sti.java
importer/IOStyle.java                          // 若独立文件
importer/num/NumberingImporter.java
importer/NumIdRedirection.java
importer/settings/SettingsImporter.java
writer/io/reader/tools/FinishReadingWork.java
core-libs: IDocumentImporter / XWPFDocument.parse 驱动
```

---

## 8. Wave 6 — 验证口径（分析向，非实现）

### 8.1 建议对比维度

1. **纯文本**: 主文档 `getText` / rope 全文字符（含段末 `\r`）  
2. **CHPX 区间**: 每个 entry 的 `[cp, next)` + PropertySet 关键 propId  
3. **PAPX 区间**: 段末 CP + prop（含 style 191、对齐、缩进）  
4. **Styles**: sti、name、basedOn、默认 rPr/pPr  
5. **Section**: `PLCSection` 节点 CP 与节属性（若有）  

### 8.2 测试文档梯度

1. 单段纯文本、无样式  
2. 多段 + 粗体/字号  
3. 自定义 paragraph style  
4. 分节 `\f`  
5.（可选）简单 numbering  

### 8.3 旁路

若 OPC 未通：单元测试直接：

```text
TextDocument::new
→ DocumentImporter / MainDocumentImporter
→ importParagraphStart → importRunTextSimple → importParagraphEnd
```

验证 CoreHelper 落库，再接 DocxReader。

---

## 9. 属性 ID 与特殊字符（改写时钉死）

### 9.1 常见字符

| 字符 | 含义 | 出现位置 |
|------|------|----------|
| `\r` (0x0D) | 段落结束 | 普通段末 |
| `\f` (0x0C) | 分节 | 节结束段 |
| `\u0007` | 单元格标记 | 表单元格（layer==1） |
| `\b` / 其它 | 形状等特殊 | MVP stub |

### 9.2 文档中出现的关键 propId（不完全表）

| ID | 语境 | 含义线索 |
|----|------|----------|
| 10 | 字符 | 字号（section 字体推算） |
| 191 | 段落 | 样式 sti |
| 223/224/226 | 表格 | 表层/单元格标志 |
| 234/235 | 段落 | 编号相关 |
| 282 | 段落 | paraId 字符串 |
| 292 | 段落 | 样式引用（PapxTable.beforeAddReference） |

完整语义以 `property` 定义与 Java setter 为准；对比时以 **同 ID 同值** 为准，不要重编号。

---

## 10. 按子功能的「读懂检查表」

改写某 Wave 前，应用代码回答下列问题（答不出就继续读 Java）：

### Wave1

- [ ] `appendText` 时 CHPX 的 insert CP 为什么是 **append 前** 的 length？  
- [ ] PAPX 的 insert CP 与段末字符 CP 是什么关系？  
- [ ] TextRope Listener 在 insert 时如何通知 Papx/Chpx？  

### Wave2

- [ ] Main SubDocument 构造后有哪些表非 null？  
- [ ] `createSubDocument(0)` 与 `getMainDocument()` 关系？  
- [ ] `StyleId2Sti` 何时分配 sti？  

### Wave3

- [ ] elementId 从何而来（OOXML 名 hash）？  
- [ ] `correctProp` 改了哪些不变量？  
- [ ] run 属性与段末 `getRunProp()`（段标字符样式）区别？  

### Wave4

- [ ] `mPositionInserted` 为何在 Start 时抓取？  
- [ ] `importParagraphEnd` 失败时文档是否半写入？  
- [ ] MainDocumentImporter 的 lock/unlock 与域计数关系？  

### Wave5

- [ ] `XWPFDocument.parse` 大致回调顺序（styles → numbering → document）？  
- [ ] `importStylesEnd` 前样式是否已可 `getStyleById`？  
- [ ] DocxReader 里 `FinishReadingWork` 做什么修补？  

---

## 11. 与现有计划的关系

- 实现顺序仍锁定：**Wave1 → 2 → 3 → 4 → 5 → 6**  
- 本文替代早期仅含依赖矩阵的短版 Wave0 说明；**先读懂再改写**  
- 编码时继续遵守仓库 `RUST_REWRITE_RULE` / pipeline：逻辑 100% 对齐、保留 `// Java:` 溯源  

---

## 12. 附录：MVP 热路径伪代码

```text
// 一段「Hello」+ 粗体 run + 段末
importParagraphStart(null)
  ParagraphContext(cp0 = length)

importRunTextSimple("Hello", chpx_bold)
  appendText → rope += "Hello"; chpx.insert(cp0, bold)

importParagraphEnd(para={endMark:'\r', paraProp, runProp})
  correctProp(paraProp)
  appendParagraph('\r', cp0, runProp, paraProp)
    appendText('\r', runProp)
    papx.insert(cp0_of_break_or_mPositionInserted, paraProp)
```

读懂上述六行与真实 Java 逐行对应后，即可开始 Wave1 实现。
