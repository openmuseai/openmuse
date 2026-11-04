# 03 — 属性转换（OOXML → PropertySet）

**Wave**: 3  
**前置**: [00-ooxml-standards/wordprocessingml.md](.../wave0-ooxml-standards/docs/wordprocessingml.md)  
**下游**: [04-paragraph-import](.../wc-04-paragraph-import/docs/README.md) 使用已转换的 props  
**改写顺序**: [REWRITE_FILE_ORDER.md](../REWRITE_FILE_ORDER.md) · CSV `batch_id=parse-03-property-convert`（order 400+）

---

## 1. 子功能职责

把 SAX `Attributes` / 元素（elementId）转换成内核扁平 **PropertySet**。  
不写 rope；只产属性桶。

## 2. 标准概念对照

| 转换器 | OOXML |
|--------|-------|
| RunPropsConverterSimple | `w:rPr` 子元素（`w:b`、`w:sz`、`w:rFonts`…） |
| ParaPropsConverterSimple | `w:pPr` 子元素（`w:jc`、`w:ind`、`w:pStyle`…） |
| ConvertHelper | 通用 attr / 度量 / BRC / 颜色等 |
| ImporterBase | 格式模式（DOCX / Xml03 / Xml07），影响部分解析分支 |
| correctProp | 内核落库前纠偏（如缺省 pStyle→Normal sti 0） |

度量单位（twip、半磅字号等）必须符合 OOXML 约定，见各 converter 与 ConvertHelper。

## 3. 本目录类文档

| 文档 | 类 |
|------|-----|
| [ImporterBase.md](./ImporterBase.md) | `ImporterBase` |
| [ConvertHelper.md](./ConvertHelper.md) | `ConvertHelper` |
| [RunPropsConverterSimple.md](./RunPropsConverterSimple.md) | `RunPropsConverterSimple` |
| [ParaPropsConverterSimple.md](./ParaPropsConverterSimple.md) | `ParaPropsConverterSimple` |

## 4. 建议阅读顺序

1. [ImporterBase](./ImporterBase.md)  
2. [ConvertHelper](./ConvertHelper.md) — 按调用点读，勿通读  
3. [RunPropsConverterSimple](./RunPropsConverterSimple.md)  
4. [ParaPropsConverterSimple](./ParaPropsConverterSimple.md) — 必读 `correctProp`

## 5. MVP 验收

- [ ] 粗体/字号/对齐/pStyle 的 propId 与 Java 一致  
- [ ] `correctProp` 缺省写入 191=0  
- [ ] 未知 elementId 不崩溃、不写错桶

---

## 6. 以核心类为中心的功能实现逻辑

本层**只产 PropertySet，不写 rope**。热路径四个类：

```text
SAX attrs / elementId
  → ConvertHelper（取值、布尔、度量、BRC…）
  → RunPropsConverterSimple.convert*     // w:rPr
  → ParaPropsConverterSimple.convertProp // w:pPr
  →（段末）ParaPropsConverterSimple.correctProp
  → Immutable/freeze → CoreHelper.insert
```

### 6.1 `ImporterBase` — 模式开关

持有 `mode` / xml03 / xml07 等；`isDocx()` 影响部分分支。  
DocxReader 默认 `mode=0`（DOCX）。详见 [ImporterBase.md](./ImporterBase.md)。

### 6.2 `ConvertHelper` — 静态工具箱

禁止线性通读；按调用点收集 20–40 个高频方法即可。  
**度量单位**（twip / 半磅字号 / emu）必须与 Java 一致。详见 [ConvertHelper.md](./ConvertHelper.md)。

### 6.3 `RunPropsConverterSimple` — `w:rPr` → 字符 PropertySet

`switch(elementId)`：粗体、字号、字体、颜色、竖排… → 对应 propId。  
由 Rpr handler / StylesImporter / 段标 runProp 等调用。详见 [RunPropsConverterSimple.md](./RunPropsConverterSimple.md)。

### 6.4 `ParaPropsConverterSimple` — `w:pPr` + `correctProp`

| 阶段 | 作用 |
|------|------|
| `convertProp` | pStyle→191、对齐、缩进、编号（经 NumIdRedirection）… |
| `correctProp` | 无 191 则 `setInt(191, 0)`（Normal）；再纠 tabs 等 |

**段末必经** `correctProp`，否则缺省样式链断裂。详见 [ParaPropsConverterSimple.md](./ParaPropsConverterSimple.md)。

### 6.5 与 XWPF handler 的分工

- wave0-xwpf 的 `PprHandler`/`RprHandler`：走 XML 树、凑齐 attrs  
- 本层 converter：把 attrs/element **语义化**成内核 propId  
- 04 的 `importParagraphEnd`：取 `paragraph.getParaProp()/getRunProp()`，再 correct + append  

表格/边框等 `*Property*Simple` 标 `mvp-defer`，遇文档再补。

---

## 7. Java ↔ Rust 逻辑对应

**CSV**: `batch_id=parse-03-property-convert`（order 400+）  
**进度**: 全部 `pending`

| Java 类 | Rust 路径 | MVP |
|---------|-----------|-----|
| `ImporterBase` | `…/io/reader/ImporterBase.rs` | must |
| `WmlUnit` | `…/helpers/WmlUnit.rs` | should |
| `ConvertHelper` | `…/helpers/ConvertHelper.rs` | must（热路径子集） |
| `ShareConvertHelper` | `…/ShareConvertHelper.rs` | should |
| `DefaultDocxRun/ParagraphProperties` | 同 helpers | should |
| `RunPropsConverterSimple` | `…/RunPropsConverterSimple.rs` | must |
| `ParaPropsConverterSimple` | `…/ParaPropsConverterSimple.rs` | must |
| `ImporterHelper` | `…/ImporterHelper.rs` | should（表格 amend） |
| `BorderConvertSimple` 等 | 同目录 | defer |

依赖（已 done）：`PropertySet` / `MutablePropertySet`（`parse-wave0`）。

### 7.1 逻辑对齐要点

| 语义 | 对齐标准 |
|------|----------|
| propId | 与 Java 同一整数（如 pStyle=191、paraId=282） |
| `toBoolean` | `"true"`/`"1"`/`"on"` / 缺省行为 |
| `correctProp` | 缺 191 → 0 |
| 未知 element | 不写错桶、不 panic |
| xml03 vs docx | 分支条件与 `ImporterBase` 一致 |

### 7.2 改写策略

1. 从 Run/Para converter + StylesImporter **grep 引用集**，只移植被调用到的 ConvertHelper 方法。  
2. 先打通粗体/字号/对齐/pStyle 四条断言，再扩枚举表。
