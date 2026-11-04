# wave0-xwpf — OPC / XWPF / SAX Handler：Java ↔ Rust 逻辑对照

**CSV batch**: `parse-wave0-xwpf`（order 10–144）  
**改写进度**: 已全部 `done`（~135 类）  
**Rust crate**: `libs/core-libs/io/wps-io/`（另有 `SubDocType` 在 util）  
**标准前置**: [opc-package.md](./opc-package.md) · [wordprocessingml.md](./wordprocessingml.md)  
**下游消费**: [05-orchestration](.../wc-05-orchestration/docs/README.md) 的 `DocumentImporter` / `DocxReader`

本层把「ZIP 包 + XML」变成对 `IDocumentImporter` 的业务回调；**不写** TextRope / CHPX。

---

## 1. 以核心类为中心的实现逻辑

整条链路可压成 **5 个枢纽**：

```text
.docx ZIP
  → OPCPackage / XWPFPackage          ① 包打开
  → XWPFDocument                      ② Part 编排 + parse()
  → XmlReader + DispatchHandler2      ③ SAX / 自研 XML
  → DocumentHandler → P/R/T Handlers  ④ 元素 → 业务事件
  → IDocumentImporter                 ⑤ 回调到 word-core Importer
```

### 1.1 `OPCPackage` / `XWPFPackage` — 打开包

| 步骤 | Java 逻辑 | 业务含义 |
|------|-----------|----------|
| open | 读 ZIP → `ContentTypeManager` → 收集 `PackagePart` | 建立 Part 名 ↔ 流 |
| relationships | 解析 `_rels/.rels` / part `.rels` | 找到 document/styles/… |
| properties | `PackagePropertiesUnmarshaller` 等 | core/app 元数据 Part |

`XWPFPackage` 是面向 Word 的薄封装：在通用 OPC 之上注册 Word 相关 unmarshaller。

**Rust 对应**: `struct OPCPackage` / `struct XWPFPackage`，方法名与字段名尽量保留 Java camelCase（`mParts` 等），继承用 `base: Parent` 字段组合。

### 1.2 `XWPFDocument` — parse 编排枢纽

```text
XWPFDocument.parse():
  1. 按 Relationship 找到 styles / numbering / settings / theme …
  2. 对各 Part：XmlReader.parse(partStream, handler)
  3. 主文档：DocumentHandler 驱动 body → 段落/run
  4. numbering 常在正文后（与 Java 顺序对齐）
  5. 全程持有 IDocumentImporter；handler 内转发回调
```

关键依赖：

- `XWPFFactory` / `XWPFRelation*`：Part 类型 → 具体 `XWPFStyles` / `XWPFNumbering` …
- `DocumentHandler`：`w:document` 根
- `setDocumentImporter(importer)`：由 `DocxReader` 注入

**Rust**: `struct XWPFDocument { base: POIXMLDocument, … }`；子 Part 字段在改写期可能仍是 `Option<Box<dyn Any>>` 桩，但 **parse 调用顺序与 Java 一致**是联调前提。

### 1.3 `XmlReader` + `DispatchHandler2` — XML → elementId

```text
字节流
  → XmlParse / Token / LexicalState     （自研词法，非 JDK SAX 必经）
  → ContentHandlerImpl
  → DispatchHandler2.start/end(elementId, attrs)
  → 当前 ElementHandlerSimple 栈顶处理
```

| 类型 | 作用 |
|------|------|
| `ElementIds` / `XmlName2Id` | 本地名 → 整型 id（避免热路径字符串比较） |
| `ElementHandlerSimple` | 所有 XWPF handler 基类契约 |
| `SAXSimpleReader` | 可选 SAX 桥 |

**Rust**: `ElementIds` 为大型常量表；`DispatchHandler2` 用 trait object / 枚举栈模拟 Java handler 栈。逻辑钉死点：**同名元素必须映射到同一 id**。

### 1.4 `DocumentHandler` → `PHandler` → `RHandler` → `THandler` — 正文热路径

```text
w:document
  DocumentHandler
    └─ w:body → DocumentBodyHandler / SubDocumentHandler
         └─ w:p → PHandler
              ├─ w:pPr → PprHandler →（属性写入 ParagraphSimple）
              └─ w:r → RHandler
                   ├─ w:rPr → RprHandler
                   └─ w:t → THandler → importer.onImportRunText*(…)
         段结束 → importer.onImportParagraphEnd(paragraph, subDocType, layer)
```

| 核心类 | 做什么 |
|--------|--------|
| `ParagraphSimple` | 累积当前段的 paraProp / runProp / endMark |
| `PprHandler` / `RprHandler` | 子元素 → 通过 prop 框架写入 PropertySet（真正 convert 在 word-core 的 *ConverterSimple） |
| `THandler` | 文本节点 → `IDocumentImporter` run-text 回调 |
| `SectPrHandler` | 节属性；可影响 endMark=`\f` |
| `StylesHandler` / `NumberingHandler` / `SettingsHandler` | 非正文 Part 的对称结构 |

**Rust**: 每个 `*Handler` ↔ 同名 `.rs`；对 importer 的调用签名（`SubDocType`、`ParagraphSimple`、attrs）必须与 Java 一致，否则 04/05 无法对接。

### 1.5 `IDocumentImporter` — 跨 crate 边界

定义在 `wps-io`，实现在 `word-core`（`DocumentImporter`）。

```text
XWPF（本层）只知道接口方法名：
  onImportParagraphStart / End
  onImportRunText*
  onImportStyle* / onImportNumbering* / onImportSettings*
  …
实现方（05）负责写入 TextDocument。
```

**Rust**: `trait IDocumentImporter`；`XWPFDocument` 持有 `Option<Box<dyn IDocumentImporter>>`（或项目约定的句柄类型）。wave0-xwpf 改写完成时，下游实现类仍可为 pending——用 stub 实现即可让 `parse` 编译通过。

---

## 2. Java ↔ Rust 类型与路径对照（核心）

| Java | Rust 路径 | 备注 |
|------|-----------|------|
| `cn.wps.io.opc.OPCPackage` | `…/opc/OPCPackage.rs` | 包根 |
| `cn.wps.io.xwpf.XWPFPackage` | `…/xwpf/XWPFPackage.rs` | Word 包 |
| `cn.wps.io.xwpf.usermodel.XWPFDocument` | `…/xwpf/usermodel/XWPFDocument.rs` | parse 枢纽 |
| `cn.wps.io.opt.XmlReader` | `…/opt/XmlReader.rs` | XML 入口 |
| `cn.wps.io.simple.DispatchHandler2` | `…/simple/DispatchHandler2.rs` | elementId 分发 |
| `cn.wps.io.xwpf.IDocumentImporter` | `…/xwpf/IDocumentImporter.rs` | 回调 trait |
| `cn.wps.io.xwpf.element_handler.DocumentHandler` | `…/element_handler/DocumentHandler.rs` | 根 handler |
| `…PHandler` / `RHandler` / `THandler` | 同目录同名 `.rs` | 正文三件套 |
| `…styles.StylesHandler` | `…/styles/StylesHandler.rs` | styles.xml |
| `…numbering.NumberingHandler` | `…/numbering/NumberingHandler.rs` | numbering.xml |
| `cn.wps.base.io.SubDocType` | `libs/…/util/…/SubDocType.rs` | 故事类型枚举 |

完整清单见 CSV：`rg 'parse-wave0-xwpf' rewriting_files_parse_mvp.csv`。

---

## 3. 惯用改写约定（读 Rust 时）

| Java | Rust 常见落法 |
|------|----------------|
| `interface` | `trait` |
| `class extends A` | `struct` + `base: A` 字段，或手动委托 |
| `null` | `Option<T>` |
| 异常 | `Result` / 项目内 `*Exception` 类型 |
| `Attributes` | `AttributesImpl` 等值对象 |
| handler 多态栈 | `Box<dyn ElementHandlerSimple>` 或枚举 |

**逻辑对齐标准**：同一输入 DOCX，handler 回调顺序、`elementId`、传给 importer 的字符串/属性，应与 Java 一致；不要求 API 符合 Rust 习惯用法。

---

## 4. 与 PropertySet（parse-wave0）

属性桶本身在 util：

| Java | Rust |
|------|------|
| `PropertySet` | `libs/core-libs/util/util/src/cn/wps/moffice/property/PropertySet.rs` |
| `MutablePropertySet` | 同目录 `MutablePropertySet.rs` |

Handler/prop 框架（`PropHandler` 等）在 wave0-xwpf 内组装 **可变属性对象**；真正「OOXML → propId」的语义转换多在 word-core 的 [03-property-convert](.../wc-03-property-convert/docs/README.md)。

---

## 5. 读懂检查

- [ ] `XWPFDocument.parse` 不直接 `appendText`，只调 `IDocumentImporter`？  
- [ ] `THandler` → run-text 回调 →（未来）`SubDocumentImporter.importRunText` 的链路？  
- [ ] `ElementIds` 改错一个常量会导致哪个 handler 永远进不去？  
- [ ] styles Part 与 body Part 的解析先后对 `StyleId2Sti` 有何影响？
