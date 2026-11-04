# 节（Section）、sectPr 与分节符

**相关代码**: [PLCSection](../../wc-01-storage-write/docs/STORAGE.md#52-plcsection排版最敏感的一种) · [MainDocumentImporter](.../wc-04-paragraph-import/docs/MainDocumentImporter.md) · [section 映射](./mapping-to-kernel.md)

---

## 1. 什么是 Section

在 WordprocessingML 中，**节（Section）** 是一组分节属性相同的连续内容范围。节属性包括：

- 页面大小 / 方向 / 页边距  
- 分栏  
- 页眉页脚链接与种类（首页不同、奇偶不同）  
- 文档网格（document grid）  
- 节类型（下一页分节、连续分节等）

一篇文档至少一节；插入「分节符」后增加节。

---

## 2. `w:sectPr` 出现位置

| 位置 | 含义 |
|------|------|
| 某段落的 `w:pPr/w:sectPr` | 该段落是**本节最后一段**；其后开始新节（或文档结束） |
| `w:body` 末尾的 `w:sectPr` | **最后一节**的属性（最终节） |

这与 UI「分节符」一致：分节符「属于」上一节的末尾。

---

## 3. 分节符类型（概念）

OOXML 用 `w:type`（在 sectPr 相关元素中）表达，例如：

| 概念 | 典型含义 |
|------|----------|
| nextPage | 下一页开始新节 |
| continuous | 连续（同页）分节 |
| oddPage / evenPage | 下一奇/偶页 |
| nextColumn | 下一栏 |

本内核用段末字符 **`\f`（Form Feed）** 标记「这里发生了分节」，再在 `PLCSection` 挂节节点；具体 type 进节的 PropertySet。

---

## 4. 页眉页脚（Header / Footer）

| 术语 | 说明 |
|------|------|
| Header / Footer | 页眉 / 页脚 Part（如 `header1.xml`） |
| 与节绑定 | 每节可引用不同 header/footer Relationship |
| 三种常见 | default（odd）、even、first |
| `titlePg` | 首页不同 |
| `evenAndOddHeaders` | 奇偶页不同 |

`MainDocumentImporter.doImportSectionPropertiesSimple` 把 header/footer 的 PLC 节点挂到 `SectionNode`。  
MVP 可先空挂接，但术语与 API 面需保留。

---

## 5. 与本内核时序

```text
段落结束且为分节:
  endMark = '\f'
  SubDocumentImporter → PLCSection.setLastSectionEndCP(length)
  MainDocumentImporter → 记住 mCpLastSectionEnd

随后收到节属性回调:
  doImportSectionPropertiesSimple
    → PLCSection.addSection(cp)
    → 写入 sect 属性、可选 header/footer 节点
```

最后一节可能在 `FinishReadingWork.correctSectionData` 中补全。

---

## 6. 术语表（本节）

| 中文 | 英文 | 说明 |
|------|------|------|
| 节 | Section | 共享同一套页面/页眉设置的范围 |
| 节属性 | Section Properties (`sectPr`) | 该节的页面设置 XML |
| 分节符 | Section break | 用户插入的节边界 |
| 页眉/页脚 | Header / Footer | 与节关联的故事（story） |
| 文档网格 | Document grid | 东亚文档常用的字网格设置 |

---

## 7. 读懂检查

- [ ] body 末尾的 sectPr 对应第几节？  
- [ ] `\f` 写入时文档 length 是否已包含分节字符本身？  
- [ ] `mLastSecEndCP` 与 `mCpLastSectionEnd` 分别谁写谁读？
