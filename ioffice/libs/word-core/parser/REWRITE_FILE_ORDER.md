# Parse MVP — 按 Group 依赖排序的改写文件清单

**目标 B**: `DocxReader.read(真实.docx)` → 填充 `TextDocument`（正文 + CHPX/PAPX + 样式 + 节）  
**CSV**: [`rewriting_files_parse_mvp.csv`](./rewriting_files_parse_mvp.csv)（列格式对齐 `libs/word-core/rewriting_files.csv`）  
**设计依据**: [`REWRITE_DESIGN_PARSE_MVP.md`](./REWRITE_DESIGN_PARSE_MVP.md)

---

## 0. 排序原则

1. **跨 group**: `wave0 → wave0-xwpf → 01 → 02 → 03 → 04 → 05`  
2. **组内**: 接口/叶子 → 实现 → 工厂 → 门面/编排；`depends` 写类名（`;` 分隔）  
3. **`order`**: 全局拓扑序号（按 batch 分段，互不重叠）  
4. **`tags`**: `mvp-must` / `mvp-should` / `mvp-stub` / `mvp-defer` / `subset-*`

**00-ooxml-standards**: 仅文档，不进 CSV。

---

## 1. Group 依赖总图（目标 B）

```text
parse-wave0          PropertySet / MutablePropertySet
      │
      ▼
parse-wave0-xwpf     OPC 打开 → XWPFDocument.parse → SAX handlers
                     → 回调 IDocumentImporter
      │
      ▼
parse-01-storage     TextRope → CPTable → Chpx/Papx → PLC → CoreHelper
      │
      ▼
parse-02-document-model   Document → SubDocument → TextDocument → StyleId2Sti
      │
      ▼
parse-03-property-convert ConvertHelper → Run/Para *Simple
      │
      ▼
parse-04-paragraph-import SubDocumentImporter → MainDocumentImporter
      │
      ▼
parse-05-orchestration    Styles/Numbering → DocumentImporter → DocxReader
```

端到端：

```text
.docx ──XWPFPackage.open──► XWPFDocument
                              │ parse()
                              ├─ settings / styles / theme …
                              ├─ DocumentHandler → body → P/R/T …
                              └─ numbering（正文后）
                                    │ IDocumentImporter 回调
                                    ▼
                              DocumentImporter → SubDocumentImporter → CoreHelper
                                    ▼
                              TextDocument（L1 模型）
```

---

## 2. `order` 号段

| batch_id | order 起 | 约文件数 | 说明 |
|----------|--------:|--------:|------|
| `parse-wave0` | 1 | 4 | PropertySet 族（util） |
| `parse-wave0-xwpf` | 10 | 135 | **OPC + XmlReader + XWPF + 关键 handlers** |
| `parse-01-storage` | 200 | 34 | textrope / CHPX / PAPX / PLC |
| `parse-02-document-model` | 300 | 14 | TextDocument / Styles |
| `parse-03-property-convert` | 400 | 12 | OOXML → PropertySet |
| `parse-04-paragraph-import` | 500 | 10 | 段落状态机 |
| `parse-05-orchestration` | 600 | 20 | DocxReader 编排 |
| **合计** | | **~229** | 其中 `mvp-must` ≈ 142 |

---

## 3. `parse-wave0-xwpf` 内部顺序（摘要）

按 CSV `order` 10… 升序，逻辑块如下：

```text
1. SubDocType
2. SAX 分发：ElementHandlerSimple / ElementIds / DispatchHandler2 / SAX*
3. XmlReader（opt）：XmlParse → XmlReader
4. OPC 打开：PackagePart* / Zip* / ContentType* / OPCPackage → XWPFPackage
5. POIXML* 模型
6. XWPF usermodel：Relation / Factory / IDocumentImporter*
                 → XWPFStyles / Numbering / Settings → XWPFDocument
7. Handler 基座：XmlSimpleNodeElementHandler / ParagraphSimple / prop/*
8. 正文链：DocumentHandler → Body → SubDocument → P → R → T
           + Ppr/Rpr + SectPr
9. StylesHandler 族 / NumberingHandler 族 / SettingsHandler
10. 表格 Tbl/Tr/Tc（should）+ Drawing/AlternateContent（stub）
```

**MVP 策略**：

| 标签 | 含义 |
|------|------|
| `mvp-must` | 打开 DOCX + 解析 styles + 主文档段落/run 文本所必需 |
| `mvp-should` | numbering / settings / 简单表格 / 节属性 |
| `mvp-stub` | DML Drawing、VML、AlternateContent：接口在，回调 no-op |
| `mvp-defer` | customXml / 完整 properties 写回 / 复杂 lvlOverride |

> OPC/ZIP 在实现时可能还牵出 `kfs.zip`、断言/日志等**薄运行时依赖**；CSV 列的是业务主链。若链接失败，按编译器报错增量补进本 batch，勿跳到 layout。

---

## 4. 其余 Group（order 已重编号）

### parse-wave0（1–4）

PropertySet → MutablePropertySet → PropertyMap*

### parse-01-storage（200+）

```text
ITextPiece → ITextPool → ITextRope → INotifyTextRope
  → KTextPool → KTextPieceTable → KTextRope → KNotifyTextRope
KPropertySetRegistry → List*Tree → CPTable → Chpx/Papx → PLC → PLCSection
CPieceTable → CoreHelper
```

### parse-02-document-model（300+）

```text
Document → KStyles → TextRopeListeners → SubDocument → TextDocument
BuildinStyleSheet → StyleId2Sti；NumIdRedirection；IOHelper 子集
```

### parse-03-property-convert（400+）

```text
ImporterBase → ConvertHelper → RunPropsConverterSimple → ParaPropsConverterSimple
```

### parse-04-paragraph-import（500+）

```text
ParagraphContext → SubDocumentImporter → MainDocumentImporter
（其它故事 importer = stub）
```

### parse-05-orchestration（600+）

```text
StylesImporter → NumberingImporter → SettingsImporter
→ DocumentImporter → FinishReadingWork → DocxReader
```

`DocxReader.depends` 已含 `XWPFDocument;IDocumentImporter`。

---

## 5. 仍可能在 CSV 外的薄依赖（实现时按需补）

改完本 CSV 的 `mvp-must` 后，目标 B **主路径应可连通**；下列常在链接期冒出，建议遇到再列入 `parse-wave0-xwpf` 或 `parse-wave0`：

| 类型 | 例子 |
|------|------|
| 断言/日志 | `IOAssert` / `IOLog` / `KSAssert` |
| 文件/ZIP | `kfs.File` / `RandomAccessFile` / `kfs.zip.*` |
| Transaction 壳 | `Transaction`（SubDocument/PLC 构造） |
| IO 进度 | `IOListener` |
| property 值类型 | `data.value.Toggle` / `LID` / `BRC`…（converter 热路径） |
| Drawing 构造 | `IDrawingContainer` / `ShapeFactory` 空壳 |

这些**不改变** batch 拓扑：仍先 wave0-xwpf，再 storage。

---

## 6. 建议执行方式

```bash
# 只看 XWPF/OPC 层
rg 'parse-wave0-xwpf' libs/word-core/parser/rewriting_files_parse_mvp.csv

# 只看 must
rg 'mvp-must' libs/word-core/parser/rewriting_files_parse_mvp.csv

# 按 order 全量
sort -t, -k1,1n libs/word-core/parser/rewriting_files_parse_mvp.csv
```

改写时：**严格按 `order` 升序**；`parse-wave0-xwpf` 未就绪前不要宣称「真实 DOCX 可解析」。

---

## 7. 业务逻辑文档（按 group）

每个功能目录 README 含：

- **§6** 以核心类为中心的实现逻辑  
- **§7** Java ↔ Rust 路径 / 语义对齐表  

| Group | 入口 |
|-------|------|
| wave0-xwpf（OPC/XWPF） | [00-ooxml-standards/wave0-xwpf-java-rust.md](./features/wave0-ooxml-standards/docs/wave0-xwpf-java-rust.md) |
| 01–05 | 各目录 `README.md` §6–§7 |
