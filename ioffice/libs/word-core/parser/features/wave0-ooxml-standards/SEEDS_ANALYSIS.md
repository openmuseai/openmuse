# wave0-ooxml-standards — Seeds 核心文件解析

对照 [seeds.csv](./seeds.csv)。本 feature 与 `wc-*` 不同：**实现层（parse-wave0 /
parse-wave0-xwpf）已全部 done**，seeds 是枢纽类（16 个），用于生成依赖拓扑与
队列，并作为「标准 → XWPF 回调 → 下游 importer」这条桥的锚点。

> manifest 中 `seeds_count=0` 指**改写起点**为 0（无待改写种子）；本 seeds.csv
> 记录的是已完成的枢纽类，供 dep_topo/queue/结构分析引用。

---

## 枢纽分层（对应 docs/wave0-xwpf-java-rust.md 的 5 个枢纽）

```text
① 属性桶   PropertySet / MutablePropertySet          （parse-wave0，util）
② 包打开   OPCPackage / XWPFPackage                  （opc）
③ parse    XWPFDocument / XmlReader / DispatchHandler2 / ElementIds
④ 回调     IDocumentImporter / IImporterBase / ISubDocumentImporter
⑤ 正文     DocumentHandler / ParagraphSimple / PprHandler / ElementHandlerSimple
```

---

## 1. parse-wave0（order 1–2）

| seed | 作用 | 状态 |
|------|------|------|
| PropertySet | 不可变属性桶：`getInt/getFloat/getString/getBoolean/getValue/contains/freeze` | done（mvp-must，所有 wc-* 的前置） |
| MutablePropertySet | 可变门面：`setInt/setValue/setBoolean/setFloat/setString/remove` + `freeze()` | done（依赖 PropertySet） |

闭包自动拉入 `PropertyMap` / `MutablePropertyMap`（mvp-should，桶的 map 后端）。

## 2. 包打开层（order 57–58）

| seed | 作用 | 状态 |
|------|------|------|
| OPCPackage | OPC 包根：读 ZIP → ContentTypeManager → PackagePart 集合 → Relationships | done（依赖 PackagePartCollection/ZipPackagePart/ZipContentTypeManager/PackageRelationshipCollection） |
| XWPFPackage | Word 薄封装：注册 Word 相关 unmarshaller | done（依赖 OPCPackage/PackagePropertiesUnmarshaller） |

## 3. XML → elementId 层（order 10–28）

| seed | 作用 | 状态 |
|------|------|------|
| SubDocType | 故事类型枚举（MAIN=0…HEADERTEXTBOX=6），**下游路由 switch 的依据** | done（java_lines=14，util） |
| ElementHandlerSimple | 所有 XWPF handler 基类契约 | done |
| ElementIds | **OOXML 本地名 → 整型 id 常量表**（971 行，elementId 哈希唯一事实源） | done（复杂文件，java_lines=971） |
| DispatchHandler2 | start/end(elementId, attrs) 分发到当前 handler 栈顶 | done（依赖 ElementHandlerSimple/XmlName2Id） |
| XmlReader | XML 字节流 → XmlParse/ContentHandlerImpl → 业务回调 | done（依赖 XmlParse/ContentHandlerImpl） |

## 4. 跨 crate 回调边界（order 71–80）

| seed | 作用 | 状态 |
|------|------|------|
| IImporterBase | `mode()` 基座（0=DOCX/1=Xml07/2=GVml/3=Xml03） | done |
| IDocumentImporter | **~80 方法回调 trait**（onImportParagraph*/Run*/Style*/Numbering*…），word-core 侧 DocumentImporter 实现 | done（依赖 IImporterBase/SubDocType） |
| ISubDocumentImporter | 子文档 importer 接口（当前为空 trait，wc-04 需补 `: IImporterBase`） | done（mvp-stub） |
| XWPFDocument | parse 编排枢纽：setDocumentImporter + parse() 事件风暴 | done（依赖 POIXMLDocument/XWPFPackage/XWPFFactory/XWPFStyles/XWPFNumbering/XWPFSettings/DocumentHandler/IDocumentImporter） |

> ⚠ 已知缺口（见 reference.md §5）：`XWPFDocument.parse()` 在 Rust 侧**未实现**，
> element_handler 内 `TODO: call importer.*` 未接线 —— wave0 队列虽标 done，但
> 「事件风暴」链路要等 wc-05 联调时补齐，届时走手工旁路验证。

## 5. 正文/属性 handler（order 85–106）

| seed | 作用 | 状态 |
|------|------|------|
| ParagraphSimple | 累积当前段 paraProp/runProp/endMark（wc-04 的 `getEndMark/getParaProp/getRunProp` 来源） | done（handler-base） |
| DocumentHandler | `w:document` 根 handler | done（依赖 XmlSimpleNodeElementHandler） |
| PprHandler | `w:pPr` 子元素 → PropertySet 写入 | done（依赖 IPropHandler/PprHandlerHelper/PropHandler/IDocumentImporter） |

---

## 验收

1. 16 个 seed 类 Rust 文件均存在且 `mvp-must` 无 `todo!()` 残留（ElementIds 为
   纯常量表，已 interleaved 移植）。
2. `dep_topo.csv` 42 行（16 种子 + 26 依赖闭包）全部 `done`/`已生成`、
   `rs_exists=yes`、无环。
3. `rewriting_queue.csv` 42 行全部 `done`、`next_pending=null`。
4. elementId 常量表（ElementIds）与 Java 逐条一致 —— 这是 wc-03 converter
   switch 正确性的前提。
