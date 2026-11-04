# ConvertHelper

**Java**: `cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ConvertHelper.java`（~1100 行）  
**Wave**: 3  
**相关**: [RunPropsConverterSimple](./RunPropsConverterSimple.md) · [ParaPropsConverterSimple](./ParaPropsConverterSimple.md) · [DocumentImporter](.../wc-05-orchestration/docs/DocumentImporter.md)

---

## 1. 职责

静态工具箱：把 SAX `Attributes` / 字符串转成内核类型（boolean、度量、BRC、颜色、Revision、FontInfo…）。  
几乎所有 importer / converter 都依赖它；本身**不写**文档模型。

---

## 2. 阅读方法（禁止线性通读）

按调用点搜索，先掌握高频入口：

| 类别 | 典型方法 |
|------|----------|
| Attr 取值 | `getString(attrs)` / `getString(attrs, name)` / `getBoolean` |
| 布尔 | `toBoolean` / `toBoolean4NotNull` |
| 边框 | `toBRC(attrs, isXml03)` |
| 修订 | `convertIns(doc, dst, attrs)` |
| 度量 | 与 `MetricUtility` / `TypeConversion` 协作 |
| 数学 NS | `toString4Math` |
| 常量 attr 名 | `VAL` / `TYPE` / `COLOR` / `SZ` / `ID` … |

---

## 3. 设计特点

- 全 `static`，无状态（除少数 Random 等）。  
- 大量 magic：OOXML 枚举字符串 → 内部 int/enum。  
- xml03 vs xml07 分支常通过调用方传入的 `isXml03` 区分。

---

## 4. MVP 改写策略

1. 先移植主路径实际调用到的 20–40 个方法。  
2. 用 grep 从 `RunPropsConverterSimple` / `ParaPropsConverterSimple` / `StylesImporter` 收集引用集。  
3. 冷门 value 类型（DropCap、复杂 SHD 等）遇测试文档再补。  
4. **度量单位**必须与 Java 一致（twip/pt/emu），否则布局侧后续全歪。

---

## 5. 读懂检查

- [ ] `toBoolean` 对缺省 attr、`"true"`/`"1"`/`"on"` 如何处理？  
- [ ] `getString(attributes)` 无 name 参数时默认取哪个 attr？  
- [ ] BRC 在 xml03 与 docx 路径差异点？
