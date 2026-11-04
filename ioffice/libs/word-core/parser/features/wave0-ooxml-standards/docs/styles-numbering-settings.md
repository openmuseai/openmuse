# styles.xml / numbering.xml / settings.xml

**相关代码**: [StylesImporter](.../wc-05-orchestration/docs/StylesImporter.md) · [NumberingImporter](.../wc-05-orchestration/docs/NumberingImporter.md) · [SettingsImporter](.../wc-05-orchestration/docs/SettingsImporter.md)

---

## 1. styles.xml — 样式定义

### 1.1 作用

集中定义文档可用的 **Style**（段落/字符/表格/列表样式），供 `pStyle`/`rStyle` 按 **styleId** 引用。

### 1.2 结构概念

```xml
<w:styles>
  <w:docDefaults>
    <w:rPrDefault>…</w:rPrDefault>
    <w:pPrDefault>…</w:pPrDefault>
  </w:docDefaults>
  <w:style w:type="paragraph" w:styleId="Normal" w:default="1">…</w:style>
  <w:style w:type="paragraph" w:styleId="Heading1" …>…</w:style>
  …
</w:styles>
```

| 术语 | 英文 | 含义 |
|------|------|------|
| Style | Style | 命名的格式集合 |
| styleId | style ID | OOXML 字符串主键（如 `Heading1`） |
| type | paragraph / character / table / numbering | 样式种类 |
| basedOn | Based on | 父样式 styleId；形成继承链 |
| next | Next paragraph style | 回车后下一段默认样式 |
| link | Linked style | 段落样式与字符样式成对链接 |
| customStyle | Custom style | 用户自定义 vs 内置 |
| qFormat | Quick Style | 是否出现在快速样式库 |
| uiPriority | UI priority | 排序优先级 |
| docDefaults | Document defaults | 未显式指定时的默认 rPr/pPr |
| latentStyles | Latent styles | 未实例化的内置样式占位（本 MVP 可浅处理） |

### 1.3 单条 `w:style` 内常见子树

| 元素 | 含义 |
|------|------|
| `w:name` | 显示名（可本地化） |
| `w:basedOn` / `w:next` / `w:link` | 关系 |
| `w:pPr` / `w:rPr` | 该样式携带的段/字符属性 |
| `w:tblPr` / `w:tcPr` / `w:tblStylePr` | 表样式及条件格式 |

### 1.4 与内核映射

| OOXML | 内核 |
|-------|------|
| styleId 字符串 | `StyleId2Sti` → 整数 **sti** |
| 内置名（Normal 等） | `BuildinStyleSheet` 固定 sti（如 0） |
| docDefaults | `KStyles.setRPrDefault` / `setPPrDefault` |
| 整表 style | `KStyle` 列表，`importStylesEnd` 一次性 add |

**哨兵**: 无样式常用 sti **4095**（内核约定，非 OOXML 标准值）。

---

## 2. numbering.xml — 编号与多级列表

### 2.1 作用

定义 **抽象编号** 与 **编号实例**，段落通过 `w:numPr` 引用。

### 2.2 核心术语

| 术语 | XML | 含义 |
|------|-----|------|
| Abstract Numbering | `w:abstractNum` | 编号「模板」：各级 lvl 的格式 |
| Numbering Instance | `w:num` | 指向某 abstractNum 的实例（可 restart） |
| numId | `w:numId` | 实例 ID，段落引用它 |
| abstractNumId | | abstract 的 ID |
| Level / ilvl | `w:lvl` / `w:ilvl` | 级别 0..8 |
| lvlText | | 级别文本模式（`%1.` 等） |
| numFmt | | decimal / bullet / chineseCounting… |
| start | | 起始编号 |

### 2.3 段落侧引用

```xml
<w:pPr>
  <w:numPr>
    <w:ilvl w:val="0"/>
    <w:numId w:val="1"/>
  </w:numPr>
</w:pPr>
```

导入后进入 PAPX（及可能的样式继承）；`NumIdRedirection` 把包内 numId 映射为内核编号 id。

### 2.4 懒加载

若正文先于 numbering part 出现带编号的段，Java 路径可能按 Content Type 再读 numbering.xml（见 SubDocumentImporter）。改写时需保持「可后补编号定义」的语义。

---

## 3. settings.xml — 文档设置

### 3.1 作用

文档级开关与默认值（与单段格式无关），例如：

| 概念 | 典型元素线索 | 影响 |
|------|--------------|------|
| 默认制表位 | `defaultTabStop` | 无显式 tab 时的间距 |
| 修订跟踪 | `trackRevisions` | 是否记录修订 |
| 视图 / 缩放 | `zoom` | UI |
| 兼容性 | `compat` | 排版兼容 |
| 主题字体语言等 | 多种 | 东亚排版等 |

### 3.2 与内核

`SettingsImporter` 写入 `TextDocument` 字段或文档级 `PropertySet`；**不**直接 append 文本。  
MVP：保证解析不失败；按双端对比需要逐步填关键项。

---

## 4. 解析顺序（实务）

合规阅读器常见顺序（具体以实现为准）：

```text
styles →（numbering）→ settings → document body
```

样式必须在正文 `pStyle` 解析前可用（或允许前向引用后在 stylesEnd 落库——本项目是 **先收集 IOStyle，stylesEnd 再 add**）。

---

## 5. 读懂检查

- [ ] styleId 与显示名 `w:name` 是否必须相同？  
- [ ] 两个 `w:num` 能否共享同一 `abstractNum`？  
- [ ] docDefaults 与 Normal 样式同时存在时，优先级直觉是什么？（最终以引擎合并规则为准）
