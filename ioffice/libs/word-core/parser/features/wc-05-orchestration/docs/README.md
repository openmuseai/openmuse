# 05 — 解析编排（DocxReader / DocumentImporter / Parts）

**Wave**: 5  
**前置**: [04-paragraph-import](.../wc-04-paragraph-import/docs/README.md) · [00-ooxml-standards/opc-package.md](.../wave0-ooxml-standards/docs/opc-package.md)  
**验证**: 总览 Wave 6（联调 / JSON 对比）  
**改写顺序**: [REWRITE_FILE_ORDER.md](../REWRITE_FILE_ORDER.md) · CSV `batch_id=parse-05-orchestration`（order 600+）  
**前置 OPC/XWPF**: CSV `batch_id=parse-wave0-xwpf`（目标 B 必做）

---

## 1. 子功能职责

- 打开/驱动 OPC 解析（经 XWPF）  
- 按 Part 与 SubDocType **路由**回调  
- 导入 styles / numbering / settings  
- parse 后收尾（FinishReadingWork）

本层是「交通警察」，真正写字符在 04 + 01。

## 2. 标准概念对照

| 本层 | OOXML / OPC |
|------|-------------|
| DocxReader | 包级读取编排 |
| XWPFDocument.parse | 遍历 Parts / SAX |
| DocumentImporter | `IDocumentImporter` 总线 |
| StylesImporter | styles.xml |
| NumberingImporter | numbering.xml |
| SettingsImporter | settings.xml |
| FinishReadingWork | 读后一致性修补（非标准 Part） |
| SubDocType 路由 | 不同故事 Part |

## 3. 本目录类文档

| 文档 | 类 |
|------|-----|
| [DocxReader.md](./DocxReader.md) | `DocxReader` |
| [DocumentImporter.md](./DocumentImporter.md) | `DocumentImporter` |
| [StylesImporter.md](./StylesImporter.md) | `StylesImporter` |
| [NumberingImporter.md](./NumberingImporter.md) | `NumberingImporter` |
| [SettingsImporter.md](./SettingsImporter.md) | `SettingsImporter` |
| [FinishReadingWork.md](./FinishReadingWork.md) | `FinishReadingWork` |

## 4. 建议阅读顺序

1. [DocxReader](./DocxReader.md) — `read()` 全流程  
2. [DocumentImporter](./DocumentImporter.md) — 构造与路由  
3. [StylesImporter](./StylesImporter.md)  
4. [NumberingImporter](./NumberingImporter.md) / [SettingsImporter](./SettingsImporter.md)（可略读）  
5. [FinishReadingWork](./FinishReadingWork.md)

## 5. MVP 验收

- [ ] styles 在正文前落入 KStyles（或等价于 Java 顺序）  
- [ ] MAIN_DOCUMENT 段落事件到达 MainDocumentImporter  
- [ ] `supplyEndPara` + `onMainDocumentEnd` 被调用  
- [ ] 其它故事 importer 不 NPE

---

## 6. 以核心类为中心的功能实现逻辑

本层是「交通警察」：打开包、挂 importer、按 Part/故事路由；**写字符在 04+01**。

```text
DocxReader.read()
  ├─ DocumentImporter(...)                 // 装配子 importer + StyleId2Sti
  ├─ xwpf.setDocumentImporter(importer)
  ├─ xwpf.parse()                          // ★ wave0-xwpf 事件风暴
  │     ├─ StylesHandler   → StylesImporter → KStyles
  │     ├─ DocumentHandler → onImportParagraph* → MainDocumentImporter
  │     └─ Numbering/Settings → 对应 Importer
  └─ FinishReadingWork.go()                // supplyEndPara 等收尾
```

OPC/XWPF 核心类逻辑见 [wave0-xwpf-java-rust.md](.../wave0-ooxml-standards/docs/wave0-xwpf-java-rust.md)（该 batch **已 done**）。

### 6.1 `DocxReader` — 最外层编排

| 步骤 | 要点 |
|------|------|
| 构造 | `new DocumentImporter(doc, …, mode=0)` |
| parse 前 | `setInIOState(true)`、`guessedTotalLength`、挂 importer |
| parse | 全部交给 `XWPFDocument` |
| parse 后 | 元数据 importer + `FinishReadingWork` |

详见 [DocxReader.md](./DocxReader.md)。

### 6.2 `DocumentImporter` — `IDocumentImporter` 总线

```text
getSubDocumentImporter(SubDocType)
  MAIN_DOCUMENT → MainDocumentImporter
  其它          → stub（不 NPE）

onImportParagraphStart/End / RunText* / Table* …
  → 转发到对应 SubDocumentImporter

onImportStyle* / Numbering* / Settings*
  → StylesImporter / NumberingImporter / SettingsImporter
```

自身几乎不写 rope。详见 [DocumentImporter.md](./DocumentImporter.md)。

### 6.3 `StylesImporter` / `NumberingImporter` / `SettingsImporter`

| Importer | Part | 写入目标 |
|----------|------|----------|
| Styles | styles.xml | `KStyles` + StyleId2Sti |
| Numbering | numbering.xml | 编号模型 + NumIdRedirection |
| Settings | settings.xml | TextDocument 设置子集 |

样式应在正文段落消费 styleId **之前**可用（与 Java parse 顺序对齐）。  
详见各 md。

### 6.4 `FinishReadingWork` — 读后修补

`supplyEndPara`、节/空文档边界等一致性修补；非 OOXML Part。详见 [FinishReadingWork.md](./FinishReadingWork.md)。

---

## 7. Java ↔ Rust 逻辑对应

**CSV**: `batch_id=parse-05-orchestration`（order 600+）  
**进度**: 全部 `pending`  
**前置（done）**: `parse-wave0-xwpf`（`XWPFDocument`、`IDocumentImporter`、handlers）

| Java 类 | Rust 路径 | MVP |
|---------|-----------|-----|
| `DocxReader` | `…/docxReader/DocxReader.rs` | must |
| `DocumentImporter` | `…/importer/DocumentImporter.rs` | must |
| `StylesImporter` / `IOStyle` | `…/StylesImporter.rs` · `IOStyle.rs` | must |
| `NumberingImporter` + num/* | `…/importer/num/*.rs` | should |
| `SettingsImporter` | `…/settings/SettingsImporter.rs` | should |
| `SectionPropsImporter` | `…/SectionPropsImporter.rs` | should |
| `FinishReadingWork` | `…/tools/FinishReadingWork.rs` | must |
| `DocxDefaultValue` | `…/DocxDefaultValue.rs` | should |
| `*PackagePropertiesImporter` | 同 importer | defer |

### 7.1 逻辑对齐要点

| 语义 | 对齐标准 |
|------|----------|
| `read()` 顺序 | guessedLength → inIOState → setImporter → parse → FinishReadingWork |
| 路由 | `SubDocType` 与 wave0-xwpf 回调一致 |
| MAIN | 必须落到 `MainDocumentImporter` |
| styles 时机 | 不晚于正文首次解析 styleId（跟 Java parse 序） |
| stub 故事 | 返回空操作对象，禁止 null 解引用 |

### 7.2 跨 crate 对接

```text
wps-io:  trait IDocumentImporter + XWPFDocument.parse   （done）
word-core: struct DocumentImporter implements 该 trait （pending）
```

联调前可用 **手工调用** `MainDocumentImporter.import*` 验证 04+01，不必等完整 `DocxReader.read`。
