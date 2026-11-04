# WordprocessingML：主文档、段落与 Run

**标准**: OOXML WordprocessingML（`w:` 命名空间）  
**主 Part**: `word/document.xml`  
**相关代码**: XWPF 元素 handler → `DocumentImporter` → `SubDocumentImporter`

---

## 1. 文档骨架

简化结构：

```xml
<w:document>
  <w:body>
    <w:p>…</w:p>          <!-- 段落 -->
    <w:tbl>…</w:tbl>      <!-- 表格（含嵌套段落） -->
    …
    <w:sectPr>…</w:sectPr> <!-- 文档最后一节的节属性（常挂在 body 末尾） -->
  </w:body>
</w:document>
```

| 元素 | 名称 | 含义 |
|------|------|------|
| `w:document` | Document | 主文档根 |
| `w:body` | Body | 正文容器 |
| `w:p` | Paragraph | **段落** — 排版与逻辑的基本块 |
| `w:r` | Run | **运行** — 同一字符格式的连续文本片段 |
| `w:t` | Text | 实际 Unicode 文本 |
| `w:tbl` | Table | 表格 |
| `w:sectPr` | Section Properties | 节属性（见 [section-breaks.md](./section-breaks.md)） |

---

## 2. 段落（Paragraph）`w:p`

```xml
<w:p w:paraId="XXXXXXXX">
  <w:pPr> … </w:pPr>     <!-- 段落属性 -->
  <w:r> … </w:r>
  <w:r> … </w:r>
</w:p>
```

### 2.1 术语

| 术语 | 说明 |
|------|------|
| **Paragraph** | 从段开始到段结束标记之间的内容；Word UI 回车产生新段 |
| **paraId** | 可选；Word 用于协同/锚点；本内核写入 PAPX prop **282** |
| **pPr** | paragraph properties，段落直接格式 + 样式引用 |

### 2.2 与解析回调

```text
见 w:p 开始 → onImportParagraphStart / importParagraphStart
  处理子 run、超链接、域…
见 w:p 结束 → 组装 ParagraphSimple（含 endMark、paraProp、runProp）
            → onImportParagraphEnd / importParagraphEnd
```

---

## 3. Run（运行）`w:r`

```xml
<w:r>
  <w:rPr> … </w:rPr>     <!-- 字符属性 -->
  <w:t xml:space="preserve">Hello</w:t>
</w:r>
```

| 术语 | 说明 |
|------|------|
| **Run** | 共享同一 `rPr` 的连续字符；改字体/加粗会拆成多个 run |
| **rPr** | run properties |
| **w:t** | 文本；`xml:space="preserve"` 保留空格 |

一个段落可有多个 run；导入时多次 `importRunTextSimple`，每次产生一段 CHPX。

---

## 4. 属性层：直接格式 vs 样式

OOXML 区分：

1. **Style 引用**: `w:pStyle` / `w:rStyle` → `@w:val="Heading1"` 等 styleId  
2. **直接格式（Direct formatting）**: 写在本段/本 run 的 `pPr`/`rPr` 子元素（如 `w:b`、`w:sz`）

最终显示 = 样式链（basedOn）合并 + 直接格式覆盖。  
本内核：样式进 `KStyles`；直接格式进该段 PAPX / 该 run CHPX 的 `PropertySet`。

---

## 5. 常见 pPr / rPr 子元素（术语）

### 段落侧（示例）

| XML | 概念 | 内核线索 |
|-----|------|----------|
| `w:pStyle` | 段落样式 | prop 191（sti） |
| `w:jc` | 对齐 justification | 对齐枚举 |
| `w:ind` | 缩进 indentation | 左/右/首行 |
| `w:spacing` | 段前段后/行距 | LineSpacing 等 |
| `w:numPr` | 编号引用 | numId / ilvl → 234/235 一带 |
| `w:sectPr` | **段内**节属性 | 该段为节末；常伴随分节符 |

### 字符侧（示例）

| XML | 概念 | 内核线索 |
|-----|------|----------|
| `w:rStyle` | 字符样式 | 字符样式 sti |
| `w:b` / `w:i` | 粗体 / 斜体 | Toggle / boolean |
| `w:sz` | 字号（半磅） | prop 约 10 一带 |
| `w:color` | 颜色 | |
| `w:rFonts` | 字体（ascii/hAnsi/eastAsia/cs） | |
| `w:vertAlign` | 上下标 | |

具体 propId 以 Java converter 为准（见 [03-property-convert](.../wc-03-property-convert/docs/README.md)）。

---

## 6. 段末控制字符（内核约定，非 OOXML 元素）

OOXML **不**在 `w:t` 里存 Word 二进制时代的段落标记字节；本内核为对齐排版/PLC，在 `importParagraphEnd` **合成**：

| 字符 | 含义 | 典型来源 |
|------|------|----------|
| `\r` (U+000D) | 段落结束 | 普通 `w:p` 结束 |
| `\f` (U+000C) | 分节（section break） | 带分节的段落 / sectPr 场景 |
| `\u0007` | 单元格结束 | 表格单元格 |

详见 [mapping-to-kernel.md](./mapping-to-kernel.md) 与 [section-breaks.md](./section-breaks.md)。

---

## 7. 表格（简述）

```text
w:tbl → w:tr → w:tc → w:p …
```

| 术语 | 说明 |
|------|------|
| Table / Row / Cell | 表 / 行 / 单元格 |
| `tblPr` / `trPr` / `tcPr` | 各级属性 |
| 嵌套表 | `layerTable` ≥ 1；Importer 用 prop 223/224/226 |

MVP 可后做完整表，但术语需认识，因 `importParagraphEnd(layerTable)` 已出现。

---

## 8. 读懂检查

- [ ] 一个 `w:p` 含 3 个 `w:r`，导入后 CHPX / PAPX 大约几个节点？  
- [ ] `w:pPr` 与 `w:rPr` 分别进哪张表？  
- [ ] `paraId` 在标准中是否必须？本内核为何仍存储？
