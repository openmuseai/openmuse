# StyleId2Sti

**Java**: `cn/wps/moffice/writer/io/reader/docxReader/importer/StyleId2Sti.java`（~55 行）  
**Wave**: 2 / 5  
**相关**: [StylesImporter](.../wc-05-orchestration/docs/StylesImporter.md) · [KStyles](./KStyles.md) · `BuildinStyleSheet` · `UUSti`

---

## 1. 职责

维护 OOXML 字符串 `styleId` → 内核整数 **sti** 的映射。  
内置样式尽量复用 `BuildinStyleSheet` 固定 sti；自定义或类型不匹配则用 `UUSti` 分配新 id。

---

## 2. 字段

| 字段 | 含义 |
|------|------|
| `mMapStyleId2Sti` | `HashMap<String, Integer>` |
| `mStiMaker` | `UUSti`（来自 `TextDocument`） |

---

## 3. API

| 方法 | 行为 |
|------|------|
| `getSti(styleId)` | 查表；可 null（调用方常当「未注册」） |
| `put(styleId, name, styleType, isCustom)` | 查 Buildin：若无匹配或 custom → `getUUSti()`；否则用内置 sti；写入 map |
| `put(styleId, sti)` | 强制指定 |
| `put(styleId)` | 直接分配新 UUSti |
| `nextUserSti()` | 预览下一个 UUSti |
| `dispose()` | clear map + `BuildinStyleSheet.clearMapName2TypeSti()` |

---

## 4. 与段落属性的关系

`ParaPropsConverterSimple.setStyle` / run 的 `rStyle` 通过本映射把 XML 字符串写成 prop **191**（段）或对应字符样式 id。  
`StylesImporter.toSti(null) → 4095` 是「无样式」哨兵，与 map miss 不同（map miss 在 get 侧）。

---

## 5. MVP 改写要点

- HashMap + UUSti 分配器即可。  
- BuildinStyleSheet 的固定 sti 表需与 Java 一致，否则双端 sti 对不上。  
- 生命周期：随 `DocumentImporter` 创建，`dispose` 时清理。

---

## 6. 读懂检查

- [ ] 同名内置样式但 `isCustom=true` 时走哪条分支？  
- [ ] `put` 返回的 coreId 是否允许覆盖已有 styleId？  
- [ ] 4095 是否会出现在 map 的 value 里？
