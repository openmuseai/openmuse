# ParaPropsConverterSimple

**Java**: `cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.java`（~948 行）  
**基类**: `ImporterBase`  
**Wave**: 3  
**相关**: [ConvertHelper](./ConvertHelper.md) · [SubDocumentImporter](.../wc-04-paragraph-import/docs/SubDocumentImporter.md) · [NumIdRedirection](.../wc-05-orchestration/docs/NumberingImporter.md)

---

## 1. 职责

1. 段落 XML 子元素 → `MutablePropertySet`（`convertProp`）  
2. 段末落库前纠偏（`correctProp`）——**MVP 必懂**

---

## 2. `convertProp`

```text
convertProp(dst, element, attributes, styleId2Sti, numIdRedirection)
  switch (element) { … }
```

高频 case：

| 主题 | 处理 |
|------|------|
| 样式 | `setStyle` → prop **191** |
| 对齐 | `setAlignment` |
| 缩进 | `convertParaIndentation` |
| 行距 | `convertLineSpacing` / `03` 变体 |
| 大纲级别 | `setOutlineLvl` |
| 边框 | `toBRC` → 多 prop |
| 编号 | 经 `numIdRedirection` |
| 框架 / 文本方向 | 专用 setter |

---

## 3. `correctProp`（落库前）

```text
correctProp(MutablePropertySet dst):
  correntStyle(dst)   // 注意 Java 拼写 corrent
  correctTabs(dst)

correntStyle:
  若 !contains(191) → setInt(191, 0)   // 默认 Normal
```

另有重载 `correctProp(doc, styles, dst, originalDst)` 处理修订差量（`pPrChange`）。

**含义**: 无显式 `pStyle` 的段落也会带 sti=0，保证样式解析链有根。

---

## 4. 与 `importParagraphEnd` 的衔接

```text
paragraph.getParaProp()  // 已由 XWPF handler 调 convertProp 填好
→ 可能 setValue(282, paraId)
→ correctProp(dstParaProps)
→ amendTableProps（若在表内）
→ CoreHelper.appendParagraph(..., freeze(paraProps))
```

段标字符属性来自 `paragraph.getRunProp()`，走 CHPX，不是本类主路径。

---

## 5. MVP 改写要点

- Must：`convertProp` 骨架 + `correctProp`/`correntStyle` + `setStyle`/`setAlignment`/缩进/行距。  
- 编号、frame、数学段落 jc 可后补。  
- 与 Run 转换器相同：**propId 不可重编号**。

---

## 6. 读懂检查

- [ ] `correctProp` 会不会覆盖用户显式的非 0 样式？  
- [ ] `correctTabs` 改了什么不变量？  
- [ ] `getDifferenceParaProp` 在何种修订场景被调用？
