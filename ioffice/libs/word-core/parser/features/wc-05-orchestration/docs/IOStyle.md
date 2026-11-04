# IOStyle

**Seeds**: wc-05 order 600  
**Java**: `docxReader/importer/IOStyle.java`

## 职责

styles.xml 解析过程中的**单条样式草稿**；`importStylesEnd` 前不入 `KStyles`。

## 关键 API

| 方法 | 行为 |
|------|------|
| `convertTypeAndId(StyleId2Sti)` | type 字符串→coreType；`put` 分配 mCoreId |
| `rectifyStyleId()` | 优先 styleId，否则 name，否则 `""` |
| `convertStypeType` | paragraph=1, character=2, table=3, numbering=4 |

## 相关

[StylesImporter.md](./StylesImporter.md) · [../SEEDS_ANALYSIS.md](../SEEDS_ANALYSIS.md)
