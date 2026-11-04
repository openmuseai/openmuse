# 00 — OOXML / DOCX 国际标准概念与术语

**目的**: 读懂解析代码前，先统一 **ISO/IEC 29500（OOXML）** 与本内核（CP / CHPX / PAPX / PLC）之间的词汇。  
**改写无关**: 本目录不对应某一 Wave 实现，但是 **所有 Wave 的前置阅读**。

## 本目录文档

| 文档 | 内容 |
|------|------|
| [README.md](./README.md) | 本索引 + 标准体系鸟瞰 |
| [ooxml-standard-deep-dive.md](./ooxml-standard-deep-dive.md) | OOXML 历史、理念、定义范围；**逻辑结构 vs 物理排版/渲染** |
| [opc-package.md](./opc-package.md) | OPC 包、Content Types、Relationships、DOCX 目录树 |
| [wordprocessingml.md](./wordprocessingml.md) | WordprocessingML：document / 段落 / Run / 属性 |
| [styles-numbering-settings.md](./styles-numbering-settings.md) | styles.xml / numbering.xml / settings.xml |
| [section-breaks.md](./section-breaks.md) | 节、sectPr、分节符与页眉页脚 |
| [mapping-to-kernel.md](./mapping-to-kernel.md) | OOXML 概念 ↔ 本仓库内核模型对照表 |
| [wave0-xwpf-java-rust.md](./wave0-xwpf-java-rust.md) | **Java ↔ Rust**：OPC/XWPF/SAX 核心类实现逻辑与路径对照（`parse-wave0-xwpf`） |

### 深读索引（按家族，模板与 wc-01 的 deep-dive 一致：设计意图 / 类图 / 流程图 / 关键方法 / 不变量 / Rust 对照）

| 文档 | 覆盖核心类 |
|------|-----------|
| [property-deep-dive.md](./property-deep-dive.md) | PropertySet / MutablePropertySet / PropertyMap / MutablePropertyMap（parse-wave0） |
| [opc-deep-dive.md](./opc-deep-dive.md) | OPCPackage / XWPFPackage / PackagePart / ContentTypeManager / PackageRelationshipCollection / POIXMLDocument |
| [xml-elementid-deep-dive.md](./xml-elementid-deep-dive.md) | XmlReader / XmlParse / DispatchHandler2 / ElementIds / ElementHandlerSimple（XML → elementId） |
| [xwpf-handler-deep-dive.md](./xwpf-handler-deep-dive.md) | XWPFDocument / IDocumentImporter / DocumentHandler / P-R-T Handler / Ppr/RprHandler / ParagraphSimple |

## 1. 标准体系鸟瞰

| 名称 | 标准号 / 来源 | 本项目中的角色 |
|------|---------------|----------------|
| **OOXML** | ISO/IEC 29500（源自 ECMA-376） | DOCX 所属的 Office Open XML 族 |
| **OPC** | OOXML Part 2：Open Packaging Conventions | DOCX = ZIP + `[Content_Types].xml` + `_rels` |
| **WordprocessingML** | OOXML Part 1 中 Word 标记语言 | `word/document.xml` 等，前缀常为 `w:` |
| **DrawingML / VML** | 图形相关 part | MVP **延后**；importer 回调 stub |
| **ECMA-376 Transitional vs Strict** | 兼容模式差异 | 本代码路径偏 **Transitional**（与 MS Word 产物一致） |

日常所说的 **「DOCX 文件」** = 符合 OPC 的 ZIP 包 + 以 WordprocessingML 为主的一组 Parts。

## 2. 读代码时必须分清的两套词汇

| OOXML / 用户可见 | 本内核（Java/Rust 模型） | 关系 |
|------------------|--------------------------|------|
| `w:t` 文本节点 | `ITextRope` 中的字符 | 解析时追加；段末会再追加控制字符 |
| `w:r` + `w:rPr` | **CHPX**（字符属性区间） | 一个 run ≈ 一段相同 CHPX |
| `w:p` + `w:pPr` | **PAPX**（段落属性） | 段属性挂在段起始 CP（IO 路径） |
| `w:sectPr` / 分节 | **PLCSection** | `\f` + `addSection` |
| `w:style` / `styleId` | **KStyle** + **sti**（`StyleId2Sti`） | 字符串 id → 整数 sti |
| `w:numId` | 编号模型 + `NumIdRedirection` | 段落 prop 234/235 等 |
| Part（document/styles…） | `XWPFDocument.parse` 回调顺序 | 由 OPC Relationship 驱动 |

**CHPX / PAPX / PLC** 名称来自经典 Word 二进制（.doc）传统，本内核沿用；DOCX 侧对应的是 XML 属性树，**不是**文件里直接存 CHPX 字节。

## 3. 建议阅读顺序

1. [ooxml-standard-deep-dive.md](./ooxml-standard-deep-dive.md)（先建立「逻辑意图 ≠ 最终版面」的心智模型）  
2. [opc-package.md](./opc-package.md)  
3. [wordprocessingml.md](./wordprocessingml.md)  
4. [styles-numbering-settings.md](./styles-numbering-settings.md)  
5. [section-breaks.md](./section-breaks.md)  
6. [mapping-to-kernel.md](./mapping-to-kernel.md)  
7. [wave0-xwpf-java-rust.md](./wave0-xwpf-java-rust.md) — 标准如何落成 **已改写的** Java/Rust 代码  
8. 回到 [wc-01 STORAGE](../../wc-01-storage-write/docs/STORAGE.md) 看存储如何承接  

## 4. Java ↔ Rust（本目录相关）

本目录本身是标准词汇，不进 CSV。对应实现 batch：

| batch | 内容 | 状态 |
|-------|------|------|
| `parse-wave0` | `PropertySet` / `MutablePropertySet` | done（**⚠ primitive setter 丢值 stub**，见 property-deep-dive §5） |
| `parse-wave0-xwpf` | OPC 打开 + XmlReader + XWPFDocument + 关键 handlers | done（模板级；**事件风暴未接线**：parse() 缺失、正文 handler 空桩、XmlReader.rs 残留 TODO，见 xwpf-handler / xml-elementid deep-dive §5） |

核心枢纽类与逐段逻辑见 [wave0-xwpf-java-rust.md](./wave0-xwpf-java-rust.md) 与上表 4 份 deep-dive。  
一句话：**标准（本目录）→ XWPF 回调（wave0-xwpf）→ DocumentImporter（05）→ CoreHelper（01）**。

## 5. 权威参考（外部）

- ISO/IEC 29500 / ECMA-376（Office Open XML）  
- [OOXML 概述（ECMA）](https://www.ecma-international.org/publications-and-standards/standards/ecma-376/)  
- 解包任意 `.docx` 查看 `word/document.xml`、`[Content_Types].xml`、`_rels/.rels` 是最快的实证方式
