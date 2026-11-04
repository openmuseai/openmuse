# KStyles / KStyle

**Java**: `cn/wps/moffice/writer/core/KStyles.java`、`KStyle.java`  
**Wave**: 2  
**相关**: [StylesImporter](.../wc-05-orchestration/docs/StylesImporter.md) · [StyleId2Sti](./StyleId2Sti.md) · [TextDocument](./TextDocument.md)

---

## 1. 职责

- `KStyles`：文档样式表（列表 + 默认 rPr/pPr + 按 id/name 查找）  
- `KStyle`：单条样式（sti、name、basedOn、next、type、PropertySet）

Parse 时由 `StylesImporter` 填充；段/run 属性通过 sti（prop 191 等）引用。

---

## 2. KStyles 关键 API

| API | 用途 |
|-----|------|
| 构造 `(TextDocument)` | 绑定文档 |
| `setRPrDefault` / `setPPrDefault` | docDefaults |
| `getRPrDefault` / `getPPrDefault` | 解析最终属性 |
| `add(KStyle)` | importStylesEnd |
| `getStyleById(id)` / `getStyleById(id, bForceNormal)` | 编号探测、节字号等 |
| `getStyleByName` / `getStyleByIndex` / `size` | 查询 |
| `deleteUnusedStyles` | 清理（非 parse 热路径） |

`IOHelper.addNormalStyle`：确保存在 Normal（通常 sti=0）。

---

## 3. KStyle 关键字段语义

| 字段 | 含义 |
|------|------|
| id (sti) | 内部样式号 |
| name | 显示名 |
| base | basedOn sti；4095=无 |
| next | 下一段样式；默认同自身 |
| type | paragraph/character/table/… |
| property | 直接格式 PropertySet |
| link / qFormat / uiPriority | UI/链接相关 |

`getFinalProperty()`：沿 basedOn 链合并（节字号推算等会用到）。

---

## 4. 与 Papx/Chpx 引用计数

`PapxTable.beforeAddReference` 对 prop 191/292 调 `StyleListenerBase.addStyleRef`；  
`ChpxTable` 对 prop **2**（字符样式）同样加引用。  
MVP 可简化引用计数，但不要破坏 getStyleById。

---

## 5. MVP 改写要点

- 能 add + getById + 默认 rPr/pPr + Normal。  
- basedOn 合并可先浅实现（只读直接 property）。  
- sti 分配与 StyleId2Sti/BuildinStyleSheet 对齐。

---

## 6. 读懂检查

- [ ] `getStyleById(id, true)` 在缺失时是否回落 Normal？  
- [ ] 4095 作为 basedOn 哨兵是否也会 add 进表？  
- [ ] 字符样式与段落样式 type 如何区分？
