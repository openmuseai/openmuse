# S08 — 度量：字体解析 + 字符 advance / 行高数据

## 1. 职责

把 run 样式（fontId / size_twip / 粗斜 / 颜色）+ 文本片段解析成**逐字符几何**：glyph advance（hmtx）、ascender/descender（hhea/OS2，字体单位），并换算成 twip 供断行（S09）使用。

管线位置：上游 S07；下游 S09/S10。

## 2. 核心 Java 代码

| 类 | 路径 | 职责 |
|----|------|------|
| `FontHost` / `FontTool.addFontMapScheme` | `thirdparty/font` | 族名 → 字体文件（注入表） |
| `TypefaceFactory` | `thirdparty/font` | 族名 → `ITypeface` |
| `FontStyleFactory` | `writer/core/layout/base/env/` | PropertySet → `TypoFontStyle` + `ITypeface`（字号/粗体/颜色） |
| `AndroidTypeface` | `thirdparty/font/impl/`（1036 行） | ITypeface：FreeType + `Paint.measureText` fallback |
| `FreeTypeJNI` | `thirdparty/font/.../FreeTypeJNI.java` | glyph advance / kerning / path（native） |
| `PaintFontHelper.getCharWidth` | — | 字符宽度（Paint 路径） |

换算公式（钉死）：

```text
x_twip = units × size_twip × 20 / upem
```

- `units`：hmtx 中该 glyph 的 advance（字体单位）
- `size_twip`：字号（twip，1pt=20twip；如 10.5pt = 210 twip）
- `upem`：unitsPerEm（head 表）

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| `TypoFontStyle` | run 样式投影：fontId、size_twip、bold/italic、color |
| `ITypeface` | 字体句柄 + `getCharWidth/kerning/ascender/descender` |
| advance 表 | 逐字符 advance（hmtx，字体单位） |
| 行高数据 | ascender/descender（hhea / OS2，字体单位） |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | run 样式（fontId / size_twip / 粗斜 / 色）+ 文本片段 + 字体文件 |
| **O** | 逐字符 advance + ascender/descender（字体单位）；`x_twip = units × size_twip × 20 / upem` |

## 5. 上下游

- **上游**：S07 run 样式来源（S04 propId 桶）。
- **下游**：S09 断行（run 宽度）、S10 行高/基线。

## 6. 边界 / 失败语义 / 不变量

- **同一 TTF**：ttf-parser advance vs Java FreeType 黄金样本差 ≤ **0.5twip**。
- MVP 只用 FreeType 路径，**禁止 `Paint.measureText` fallback**（否则断行漂移）。
- `FontStyleFactory` 必须读字号/粗体/颜色；行框可先用字号投影。

## 7. Rust 承接与现状

wr-03：`04_font_host`（注入表，对标 FontHost；layout **不探盘**）+ `01_font_stack`（ttf-parser）+ `02_run_metrics` —— **◐ FontHost 注入表 pass**；FreeTypeJNI 壳留 Backend B。