# NumIdRedirection

**Seeds**: wc-02 order 313（与 03/05 共享）  
**Java**: `docxReader/importer/NumIdRedirection.java`

## 职责

OOXML `w:numId` → 内核 numbering id（`UUNumberingId.getNextId()`）。

```text
put(numId) → 分配新 id 并写入 map
getId(numId) → 查表（可 null）
```

## 调用方

- `ParaPropsConverterSimple` 写编号属性时  
- `NumberingImporter` 解析 numbering.xml 时  

## 相关

[../SEEDS_ANALYSIS.md](../SEEDS_ANALYSIS.md) · [StyleId2Sti.md](./StyleId2Sti.md)（同类映射模式）
