# S10 — 行几何：行高 / 基线 / 对齐 → 行框

## 1. 职责

把 S09 折出的行加上垂直与水平布局：行高（字号→行格 + 段前段后 spcBef/Aft + 行距）、基线（ascender/descender）、对齐（`jc` 3385），产出 `TypoTextLine` 行框与行内起始 x。

管线位置：上游 S08（行高数据）+ S09（行）；下游 S11（分页）。

## 2. 核心 Java 代码

| 类 | 职责 |
|----|------|
| `LineLayouter` | 行高/基线计算；`setTextAlignment(jc)`；段前段后（spcBef/spcAft） |
| `TextLayouter` | 行框组装（top/height、chanel asc/des、flags） |
| `TypoTextLine` | 行对象：行框 + 行内起始 x（对齐后）+ flags |

对齐语义（kerning 对齐码，propId 193）：

```text
jc=0 left（默认）→ 起始 x = 左缩进
jc=1 center      → 起始 x = (内容宽 − 行宽)/2
jc=2 right       → 起始 x = 内容宽 − 行宽
jc=3 both/justify→ 行内伸展（MVP defer）
```

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| `TypoTextLine` | 行框：`top/height`、`ascent/descent`（chanel）、`flags`（FIRST/LAST para/page/section）、行内起始 x |
| 行高来源 | 字号 → 行格 + spcBef/Aft（段前段后）+ 行距 |
| 对齐源 | PAPX `jc`（192/193 族，propId 193） |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | S09 折行 + 字号→行格 + PAPX（191/236 等：段前段后/行距）+ jc（3385） |
| **O** | `TypoTextLine` 行框（top/height、chanel asc/des、flags）+ 行内起始 x（对齐后） |

## 5. 上下游

- **上游**：S09 行、S08 行高数据、S04 段桶（jc/spcBef/Aft）。
- **下游**：S11 分页（用行高累加页高）、S12 快照（行几何进 TypoSnapshot）。

## 6. 边界 / 失败语义 / 不变量

- 起始 x 必须对齐后给出（左对齐 MVP：= 左缩进）。
- 行 top/height/基线（twip）→ Stage2 验收字段。
- 两端对齐（both）defer。

## 7. Rust 承接与现状

wr-04：`03_alignment` + `04_line_height`；行框落 wr-01 `03_typo_line` —— **✗ 待 loop**。