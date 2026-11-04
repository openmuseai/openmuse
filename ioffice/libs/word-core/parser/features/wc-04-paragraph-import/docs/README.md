# 04 — 段落写入状态机

**Wave**: 4  
**前置**: [wc-01 STORAGE](../../wc-01-storage-write/docs/STORAGE.md) · [03-property-convert](.../wc-03-property-convert/docs/README.md)  
**上游回调**: [05-orchestration](.../wc-05-orchestration/docs/README.md) 转发事件到本层  
**改写顺序**: [REWRITE_FILE_ORDER.md](../REWRITE_FILE_ORDER.md) · CSV `batch_id=parse-04-paragraph-import`（order 500+）

---

## 1. 子功能职责

把「一个段落」的生命周期变成对 CoreHelper 的有序调用：

```text
ParagraphStart → (RunText)* → ParagraphEnd
                 （可选 Table/Cell/Section 副作用）
```

这是 parse MVP 的 **业务热路径**。

## 2. 标准概念对照

| 本层 API | OOXML |
|----------|-------|
| importParagraphStart | `w:p` 开始（含可选 paraId） |
| importRunTextSimple | `w:r` / `w:t` |
| importParagraphEnd | `w:p` 结束；合成 `\r`/`\f` |
| layerTable | 表格嵌套深度（`w:tbl`） |
| MainDocumentImporter 节逻辑 | `sectPr` / 分节 |
| 域/超链接方法 | `w:fldSimple` / `w:hyperlink` 等（MVP stub） |

见 [wordprocessingml.md](.../wave0-ooxml-standards/docs/wordprocessingml.md) · [section-breaks.md](.../wave0-ooxml-standards/docs/section-breaks.md)。

## 3. 本目录类文档

| 文档 | 类 |
|------|-----|
| [ParagraphContext.md](./ParagraphContext.md) | `ParagraphContext` |
| [SubDocumentImporter.md](./SubDocumentImporter.md) | `SubDocumentImporter` |
| [MainDocumentImporter.md](./MainDocumentImporter.md) | `MainDocumentImporter` |

## 4. 建议阅读顺序

1. [ParagraphContext](./ParagraphContext.md)  
2. [SubDocumentImporter](./SubDocumentImporter.md) — Start/Run/End 逐步  
3. [MainDocumentImporter](./MainDocumentImporter.md) — 节与 quit/lock  

## 5. MVP 验收

- [ ] 单段多 run 文本与 CHPX 正确  
- [ ] PAPX 使用段首 CP（mPositionInserted）  
- [ ] 分节 `\f` 与 addSection 链路可跑通（可无 header）

---

## 6. 以核心类为中心的功能实现逻辑

本层是 parse MVP 的**业务热路径**：把 XWPF/DocumentImporter 转发来的段落事件写成 rope+表。

```text
onImportParagraphStart
  → SubDocumentImporter.importParagraphStart
       ParagraphContext(cp0 = document.length)

onImportRunText*
  → importRunTextSimple → CoreHelper.appendText(runProps)

onImportParagraphEnd
  → correctProp(paraProps)
  → CoreHelper.appendParagraph(endMark, cp0, runProps, paraProps)
  → 若 endMark=='\f' → PLCSection.setLastSectionEndCP
```

### 6.1 `ParagraphContext` — 段会话

| 字段 | 含义 |
|------|------|
| `mPositionInserted` | 段开始时的 length → **PAPX 挂点** |
| `mParaId` | OOXML paraId → prop 282 |
| `mEntry` | 段结束后的 PAPX Entry（表格路径会 reset） |

无业务方法，但 CP 记错则整段属性错位。详见 [ParagraphContext.md](./ParagraphContext.md)。

### 6.2 `SubDocumentImporter` — 段落状态机核心

针对**一个** `SubDocument`：

1. **Start**：new `ParagraphContext(length)`  
2. **Run**：`appendText`（len>0）  
3. **End**：取 `ParagraphSimple` 的 endMark / paraProp / runProp → `correctProp` → `appendParagraph(..., mPositionInserted, ...)`  

表格 `layerTable`、域、超链接：MVP 次优先或 stub。详见 [SubDocumentImporter.md](./SubDocumentImporter.md)。

### 6.3 `MainDocumentImporter` — 主故事 + 节

继承/特化 SubDocumentImporter：主文档分节、`sectPr`、quit/lock 等。  
MVP 先保证主文档三段式；节属性可与 `SectPrHandler` / `SectionPropsImporter` 联调。详见 [MainDocumentImporter.md](./MainDocumentImporter.md)。

### 6.4 其它故事 Importer

HdrFtr / Comment / Footnote / Textbox… → CSV 标 `mvp-stub`：构造不 NPE、回调空操作即可。

---

## 7. Java ↔ Rust 逻辑对应

**CSV**: `batch_id=parse-04-paragraph-import`（order 500+）  
**进度**: 全部 `pending`

| Java 类 | Rust 路径 | MVP |
|---------|-----------|-----|
| `ParagraphContext` | `…/io/reader/helper/ParagraphContext.rs` | must |
| `SubDocumentImporter` | `…/importer/document/SubDocumentImporter.rs` | must |
| `MainDocumentImporter` | `…/MainDocumentImporter.rs` | must |
| `HdrFtrDocumentImporter` 等 | 同目录 | stub |

上游依赖：`CoreHelper`、`Para/RunPropsConverterSimple`、`StyleId2Sti`、`NumIdRedirection`（01–03/02）。  
回调边界：`IDocumentImporter`（wave0-xwpf，已 done）由 05 的 `DocumentImporter` 实现后再转发到本层。

### 7.1 逻辑对齐要点（最高优先级）

| 步骤 | Java | Rust 必须同构 |
|------|------|----------------|
| 记段首 CP | `new ParagraphContext(getLength())` | 同一 length 语义 |
| 写 run | `CoreHelper.appendText` | 禁止直接改 rope 绕过 chpx |
| 写段末 | `appendParagraph(ch, mPositionInserted, chpx, papx)` | **第二参是段首 CP** |
| correctProp | End 路径内调用 | 不可省略 |
| `\f` | `setLastSectionEndCP(getLength())` | 与 PLCSection 联调 |

### 7.2 单段心智模型（「Hi」+ 回车）

```text
Start:  cp0 = 0
Run:    append "Hi"     → rope[0,2), chpx@0
End:    append '\r'     → rope[2], chpx@2
        papx.insert(0)  → 段属性挂在 0，不是 2
```
