# OPC 包与 DOCX 物理结构

**标准**: Open Packaging Conventions（OOXML Part 2）  
**相关代码**: `XWPFDocument`（core-libs）、`DocxReader`（打开已构造的 disk doc）

---

## 1. 什么是 OPC

**OPC（Open Packaging Conventions）** 规定：

1. 包（Package）在物理上是 **ZIP**  
2. 包内是多个 **Part**（部件），每个 Part 有 URI 路径（如 `/word/document.xml`）  
3. 部件之间用 **Relationship**（关系）连接，而不是硬编码绝对路径拼装逻辑  
4. 根目录有 **`[Content_Types].xml`**，声明每个 Part 的 MIME Content Type  

DOCX、XLSX、PPTX 都是 OPC；差别在 Part 的 Content Type 与标记语言（WordprocessingML / SpreadsheetML / PresentationML）。

---

## 2. 典型 DOCX 目录树

```text
my.docx  (ZIP)
├── [Content_Types].xml          ← 所有 Part 的类型表
├── _rels/
│   └── .rels                    ← 包级关系：指向 document、core props…
├── docProps/
│   ├── core.xml                 ← 核心属性（标题、作者…）Dublin Core 风格
│   ├── app.xml                  ← 扩展属性（字数、应用版本…）
│   └── custom.xml               ← 可选自定义属性
└── word/
    ├── document.xml             ← 主文档（WordprocessingML）★
    ├── styles.xml               ← 样式
    ├── numbering.xml            ← 编号（可缺）
    ├── settings.xml             ← 文档设置
    ├── fontTable.xml            ← 字体表
    ├── theme/theme1.xml         ← 主题（可缺）
    ├── header1.xml / footer1.xml
    ├── footnotes.xml / endnotes.xml / comments.xml
    ├── media/                   ← 嵌入图片等
    └── _rels/
        └── document.xml.rels    ← 主文档到其它 Part 的关系
```

`DocxReader.read()` 里的 `readExtendedProperties` / `importMetaData` 对应 `docProps/*`；正文与样式则由 `XWPFDocument.parse()` 按关系图回调。

---

## 3. Content Types（内容类型）

`[Content_Types].xml` 示例概念：

| Part | 典型 Content Type（Transitional） |
|------|-----------------------------------|
| 主文档 | `application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml` |
| 样式 | `…wordprocessingml.styles+xml` |
| 编号 | `…wordprocessingml.numbering+xml` |
| 设置 | `…wordprocessingml.settings+xml` |

代码中懒加载 numbering 时会按 Content Type 查找 Part（见 `SubDocumentImporter.isContainNumbering` 路径）。  
**改写时不要改这些字符串**，需与 Java / 标准包一致。

---

## 4. Relationships（关系）

关系文件（`*.rels`）是小型 XML，每条含：

| 属性 | 含义 |
|------|------|
| `Id` | 如 `rId1`，供标记里 `r:id` 引用 |
| `Type` | 关系类型 URI（officeDocument、styles、image…） |
| `Target` | 相对路径（`word/document.xml`、`media/image1.png`） |

**要点**:

- 主文档 **不** 写死 `../styles.xml` 路径拼装业务；通过 Type 找 Target。  
- 超链接、图片、页眉都是 Relationship。  
- MVP 解析正文/样式时，XWPF 已解析 rels；Rust 侧若自研 OPC，必须先实现 rels 解析。

---

## 5. 与本项目入口的对应

```text
操作系统文件 my.docx
  → 打开为 XWPFDocument（OPC + 惰性 Part）
  → DocxReader 持有 XWPFDocument + 空 TextDocument
  → setDocumentImporter + parse()
       按 Part 关系依次/交错触发 IDocumentImporter 回调
```

`DocxReader` **不**自己解 ZIP；它假设 disk 层已给出 `XWPFDocument`。

---

## 6. 术语表（本节）

| 术语 | 英文 | 含义 |
|------|------|------|
| 包 | Package | 整个 DOCX ZIP |
| 部件 | Part | 包内一个可寻址资源 |
| 内容类型 | Content Type | Part 的 MIME 类名 |
| 关系 | Relationship | Part 之间的有向链接 |
| 包关系 | Package Relationship | 从包根出发的关系（`_rels/.rels`） |
| 部件关系 | Part Relationship | 从某 Part 出发（如 `document.xml.rels`） |

---

## 7. 读懂检查

- [ ] 删掉 `[Content_Types].xml` 后合规阅读器会怎样？  
- [ ] `styles.xml` 的路径是约定还是由 Relationship Type 决定？  
- [ ] `docProps/app.xml` 的字符数如何进入 `guessLengthOfDocument`？
