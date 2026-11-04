# S04 — 属性事件 → PropertySet

## 1. 职责

把 `w:pPr / w:rPr` 子元素的 `elementId + Attributes` 转成**内核 propId 桶**（`MutablePropertySet`）：run 桶（粗体 5、字号 10、斜体 6、颜色 11、rStyle 2 …）与段落桶（pStyle 191、对齐 193、缩进 208–213、ilvl 234、numId 235 …）。所有下游只消费 propId 桶，不直读 OOXML。

管线位置：上游 S02/S03；下游 S05（`ParagraphSimple` / `Prop` 携带桶）与 layout S08（度量读字号）。

## 2. 核心 Java 代码

主类（wc-03）：

| 类 | 职责 |
|----|------|
| `ConvertHelper` | 从 `Attributes` 取 string/bool/int/hashCode/度量/颜色 |
| `PropHandler / PprHandler / RprHandler / PropChildHandler` | 遍历 pPr/rPr 子元素 |
| `ParaPropsConverterSimple` | `convertProp(dst, elementId, attrs, styleId2Sti, numIdRedirection)` → 段落桶 |
| `RunPropsConverterSimple` | `convertRunPropsBase(doc, dst, elementId, attrs, styleId2Sti)` → run 桶 |
| `ParaPropsConverterSimple.correctProp` | 段末纠偏：缺 191 → `setInt(191, 0)` |

关键调用形态：

```java
// wc-03 05/06（钉死）
RunPropsConverterSimple.convertRunPropsBase(doc, dst, elementId, attrs, styleId2Sti);
ParaPropsConverterSimple.convertProp(dst, elementId, attrs, styleId2Sti, numIdRedirection);
// 段末（wc-04 importParagraphEnd 内）
ParaPropsConverterSimple.correctProp(dstParaProps);
```

`sid→sti`：`StyleId2Sti.getSti` miss → `None`（不是 4095）；run 的 `getSti==null` **不写 prop 2**，段落的 `getSti==null` → `setInt(191, 0)`。

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| `MutablePropertySet` | 内核 propId 桶：`setInt/setBoolean/setString/setValue/freeze/contains/getInt` |
| `Attributes` | S02 不透明属性表；`getValue(MAIN_NS, name)` |
| 结果桶 | run 桶 / 段桶；`freeze()` 后不可变，交给 CoreHelper |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | elementId + attrs（`getValue(MAIN_NS, name)`）+ `StyleId2Sti` + `NumIdRedirection` + mode |
| **O** | run/段 `MutablePropertySet`（propId 桶）填充到 `Prop` / `ParagraphSimple` |

## 5. 上下游

- **上游**：S03 handler 分发；wave0 `PropertySet`。
- **下游**：S05（`ParagraphSimple.getParaProp/getRunProp` 携带桶进 `appendParagraph`）；layout S08（`FontStyleFactory` 读字号/颜色）。

## 6. 边界 / 失败语义 / 不变量

- 属性值是**内核 propId**，非 OOXML 直读。
- elementId 权威 = Java `String.hashCode()`（`b→98`、`sz→3687`、`jc→3385`、`pStyle→-1008274655`…）。
- `toBoolean`：`val` 缺失/空 → true；`"off"/"0"/"false"/"f"` → false；其余 true。
- `correctProp` 只在 `!contains(191)` 时写 `191=0`，不覆盖已有样式。
- 未知 elementId：run 返回 `isNormalProperty=false` 且不写桶；para Java `IOAssert.fail`，Rust MVP **log+ignore、不 panic**。

## 7. Rust 承接与现状

wc-03 `01_function_importer_mode` .. `07_function_correct_prop` —— **✅ 全部 pass**（`ConvertHelper` / `ParaPropsConverterSimple` / `RunPropsConverterSimple` 在 compile surface）。