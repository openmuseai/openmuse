# RunPropsConverterSimple

**Java**: `cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/RunPropsConverterSimple.java`（~987 行）  
**基类**: `ImporterBase`  
**Wave**: 3  
**相关**: [ConvertHelper](./ConvertHelper.md) · [ParaPropsConverterSimple](./ParaPropsConverterSimple.md) · [StyleId2Sti](.../wc-02-document-model/docs/StyleId2Sti.md)

---

## 1. 职责

把 run 级 OOXML 子元素（以 **elementId** = 本地名 hash）转换成 `MutablePropertySet` 中的**字符属性**。

由 XWPF 的 run 属性 handler 调用；结果最终经 `importRunText*` 的 `runProps` 进入 `ChpxTable`。

---

## 2. 入口

```text
convertRunPropsBase(TextDocument doc, MutablePropertySet dst,
                    int elementId, Object object, StyleId2Sti styleId2Sti)
  → object 多为 Attributes
  → switch (elementId) { … }
  → 返回 isNormalProperty（部分修订类返回 false）
```

---

## 3. Switch 模式

```text
case <hash>:
  dst.setBoolean(propId, ConvertHelper.toBoolean(attrs))
  // 或
  dst.setValue(propId, …)
  // 或
  this.setRFonts(doc, dst, attrs) / setVerticalAlign / setEastAsianLayout …
```

典型高频属性（改写优先）：

- 粗体 / 斜体 / 下划线 / 删除线  
- 字号、颜色、高亮  
- `rFonts`、竖排对齐 `vertAlign`  
- `rStyle`（经 StyleId2Sti）  
- 间距 `spacing`、位置  

低频：修订 `rPrChange`/`ins`/`del`、文字特效、东亚版式、FitText 等 —— 按测试文档增量。

---

## 4. 依赖

- `ConvertHelper`：attr 解析  
- `StyleId2Sti`：样式引用  
- `data.value.*`：`Toggle`、`LID`、`Revision`、`EastAsianLayout`…  
- `ImporterBase.isXml03/07`：个别分支  

---

## 5. MVP 改写要点

1. 移植整个 switch **骨架**（未知 case 可 log + ignore，但不要静默改 propId）。  
2. **propId 必须与 Java 一致**（双端 JSON 对比硬约束）。  
3. 先打通「字体 + 字号 + 粗斜体 + rStyle」，再扩。

---

## 6. 读懂检查

- [ ] elementId 在 XWPF 侧如何计算？改写后能否复用同一 hash 表？  
- [ ] `setRFonts` 写入哪些 propId？与主题字体关系？  
- [ ] Toggle 型属性（bold 等）与纯 boolean 存储差异？
