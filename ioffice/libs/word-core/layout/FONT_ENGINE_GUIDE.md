# 字体、字体引擎与文字度量/渲染知识（配套 S08 度量阶段）

> 日期：2026-08-23 · 配套：`JAVA_RUST_PIPELINE.md` §8 S08；**FontHost 计划**：[FONT_HOST_DEV_PLAN.md](./FONT_HOST_DEV_PLAN.md)；feature：`features/wr-03-measure-shape`（含 `04_function_font_host`）
> 目的：把「如何确定一个特定字体的字符宽度/高度，以及如何把一个字渲染成像素」讲透，作为我们 Rust 度量（ttf-parser）+ Flutter 绘制（Skia/Impeller）管线的背景知识。

---

## 1. docx 里的字体信息：存什么、不存什么

| 存在 docx 里 | 位置 | 内容 |
|-------------|------|------|
| 字体**名与特性** | run `w:rPr/w:rFonts`（`ascii` / `eastAsia` / `hAnsi` / `cs`） | 每个 run 的字体名（拉丁/东亚/高代码/复杂文种） |
| 缺省字体 | styles.xml `docDefaults → rPrDefault → rFonts`；样式 `rFonts` | 默认字体联（我们存进 KStyles.rPrDefault，wc-02） |
| 主题字体 | theme1.xml `fontScheme → majorFont/minorFont`（latin/eastAsia/cs） | “标题主题”/“正文主题”的字体三件套 |
| 字体表（可选嵌入） | `word/fontTable.xml` + OPC `fonts/*`（ttf/odttf/woff） | **只有显式“嵌入字体”**时才把字体文件放进包 |

**结论：docx 里是“引用字体名 + 特性”**，不是“送你字体文件”。渲染端必须自己有同名（或回落）字体；这就是为什么：
- 同一 docx 在不同系统的观感/度量可能略不同（没有同名字体时）；
- 我们 Rust 度量必须**显式确定一个字体文件**（fixture/平台目录），否则无法与 Java 双跑对齐（wr-03 font_stack 的 name_map + 回落默认）。

---

## 2. 字体是什么：文件结构、字族/Face、大小单位

### 2.1 TTF/OTF 文件 = 一张表集合（我们只关心这几张）

| 表 | 作用 | 我们怎么用 |
|----|------|-----------|
| `cmap` | 字符(Unicode)→glyph id | `glyph_index(char)` |
| `hmtx` | **每个 glyph 的水平 advance**（font units） | `glyph_hor_advance(gid)` —— 字符宽度的唯一来源 |
| `hhea` | ascender / descender / lineGap（font units） | 行高/基线 |
| `OS/2` | usWinAscent/Descent、sTypoAscender/Descender | 换行高的替代度量（老字体差异） |
| `head` | **unitsPerEm**（2048 常见，CJK 常 2048；部分 1000） | **所有换算的分母** |
| `glyf` / `CFF` | 轮廓（二次/三次 Bézier） | 渲染/OutlineBuilder（MVP 不需要） |
| `name` | 字体名族 | 字体名→文件匹配辅助 |

### 2.2 字族与 Face
- **Font Family**：一簇（"Times New Roman"），含 Regular/Bold/Italic/BoldItalic 等 **Face**；
- 我们（MVP）只取 1 个默认 Face（含 bold/italic 标志合成）；族名→文件别名表（wr-03 name_map）。

### 2.3 尺寸单位（最容易错的地方）
- 字体文件里所有度量（advance/asc/desc）是 **font units**，不与任何实际大小挂钩；
- **1 em = 当前字号**：`px = units × size_px / unitsPerEm`；
- 我们的管线用 **twip**：`x_twip = units × size_twip × 20 / unitsPerEm`（size 单位为 pt，1pt=20twip）；
- 例：upem=2048，'A' advance=1031 units，12pt → `1031×12×20/2048 ≈ 120.8 twip`；另 96DPI 约定下 `12pt ≈ 16px`。

---

## 3. 如何确定一个字符的宽度（advance）

### 3.1 单字符流程（S08，即 wr-03 run_metrics）
```text
字符 'A'
  → cmap['A'] → glyph id，例 36
  → hmtx[glyph36].advance → 1031 (font units)
  → advance_twip = round(1031 × size_twip × 20 / upem)
```
- 为什么要“轮廓”？**不用**。宽度只看 hmtx（排版只用 advance）；
- **ttf-parser 与 FreeType 的数值为什么一致**：两者都只是读 hmtx 表（误差仅浮点舍入）→ 双跑黄金样本应 ≈0 差（我们门闩 >0.5twip 判失败）。

### 3.2 行内宽度 = Σ advance
- run 宽 = 逐字符 advance 之和；word-wrap 依据 run 宽（wr-04）。
- CJK：全角字符 advance ≈ 1em（如 2048）；ASCII 半角约为 0.5em 上下——**不能用“两个 ASCII=一个 CJK”猜测**，一律以 hmtx 为准。

### 3.3 会让“宽度”变复杂的（MVP 不做，列出防坑）
| 机制 | 来源 | 说明 |
|------|------|------|
| kerning（字偶距） | `kern` 表 / GPOS | “AV” 类组合按字距微调；FreeType 默认不应用（shaping 时用 GPOS） |
| 连字/替换 | GSUB | “ffi”→连字 glyph；需要 **shaping 引擎**（我们的后续 = rustybuzz） |
| CJK 标点挤压 | 排版规则 | 全角标点行尾压缩；字处理层逻辑，非字体表 |
| 可变字体 | gvar | 同一 glyph 可变形；我们明确不启用 variable-fonts feature |

---

## 4. 如何确定高度与行高

### 4.1 单字高度（字体度量）
- `ascender`（上，正值）、`descender`（下，负值），单位 font units（hhea；个别字体看 OS/2 typo 值——Java 用 `Paint.FontMetrics`，我们与 Java 对照时按其 asc/des 语义）；
- 字形高度 ≈ `ascender + |descender|`，除以 upem 随字号缩放。

### 4.2 行高（line height）
- **行格** = `ascender + |descender| + lineGap`（字体建议），或排版器另给固定行高/倍数行距；
- 段前段后 `spcBef/spcAft` 叠加到段首行 top / 段末行 bottom（wr-04 line_height）；
- **基线是坐标原点**：行 top = 段落 y + spcBef；首行文本基线 = top + ascender 的换算值；后续行基线 = 上一基线 + 行格。

### 4.3 我们的坑位（钉死）
- `mTextRect.top/height`（TypoTextLine）由行格换算（wr-01 typo_line 的 text_rect.top/height）；`chanel asc/des` ＝ 同字体的 asc/des 换算；
- 段前段后、行距多重模式的换算对齐 Java（wr-04 line_height 的黄金样本）。

---

## 5. 如何渲染一个文字（到屏幕像素）

### 5.1 两级分工（我们的管线）
```text
Rust 内核（S08/S14）：只做几何——
  FontStack(font_id) → RunMetrics(advance) → LineMetrics(x/y/基线) → DrawText{x, y, size, font_id, color}
Flutter（S16/S17）：才是“画出字形”——
  按 font_id 找到同一字体文件 → TextPainter/Skiav 对字形进行 shaping + 光栅化
```
**关键是：几何在 Rust（确定性、可对照 Java），像素在 Flutter（引擎职责）**，二者通过同一字体文件 + 同一字号换算衔接。

### 5.2 光栅化一条路（向量字体，Skia/FreeType 系都这么干）
```text
glyph id ──> 轮廓（glyf 二次 Bézier / CFF 三次 Bézier）
  ├─ 坐标单位：1000/2048 → 目标 px（units × size_px / upem）
  ├─ [可选] hinting（TrueType 指令或 auto-hint）：把笔画对齐到像素网格，保证小字号清晰
  └─ scan-conversion：每条扫描线求轮廓交点 → 计算每个像素的覆盖率（alpha）
       ├─ 灰度抗锯齿：alpha = 覆盖率
       └─ 子像素渲染（ClearType 类）：按 RGB 相位，三次采样
  ──> 得到 glyph 位图/alpha 掩码 ──> glyph cache（按 size×face 缓存）
  ──> 绘制：把掩码按 (x, y) 合成到画布（fill 颜色 = 前景色）
```
- Flutter 默认路径：Skia/Impeller 引擎内的 shaping（HarfBuzz 系）→ FreeType 或内置字体光栅化 → GPU 上传 glyph cache → draw；我们只喂坐标与字号。
- 顺带说明 ttf-parser 的边界：它能给 `OutlineBuilder`（轮廓）但**不做光栅化/hinting**——我们不需要自己做光栅化，所以用不到；将来若自绘字形（如 SVG 导出）才用。

### 5.3 绘制 y 的约定（必踩）
- `Canvas.drawText(…, x, y)` / `TextPainter.paint` 的 **y 是基线**，不是字框顶部；
- 我们 DrawText 携带的 y = 行基线（S10 的 chanel asc 换算到页面坐标系，S14 指令里直接给）。

---

## 6. 我们的依赖与 Java 对照

| 需求 | Java | Rust 方案 | 备注 |
|------|------|-----------|------|
| 字符→glyph | cmap（FreeType 内） | `ttf_parser::glyph_index` | 同表 |
| advance | hmtx（FreeType 内） | `ttf_parser::glyph_hor_advance` | 同表 ⇒ 数值一致门闩 |
| asc/des/lineGap | hhea/OS2（FreeType 内） | `ttf_parser::ascender()/descender()/line_gap()` | 与 Java `FontMetrics` 语义对照 |
| 换算分母 | `unitsPerEm` | `ttf_parser::units_per_em()` | 防 2 倍差 |
| shaping（连字/基准压缩） | HarfBuzz（Android 侧） | `rustybuzz`（后续 wr-04） | 现在不做 |
| 光栅化/hinting | FreeType + Skia | Flutter 引擎（无 Rust） | 不参与 Stage2/3 |
| 对照刑具 | Java FreeType 输出 | 可选：本地编 FreeType 当 oracle | 黄金样本差 ≤0.5twip |

## 7. 出错排查清单（钉死的常见坑）

1. **upem 混用**：1000 vs 2048 混 → 宽高错 ~2 倍；所有换算用 `face.units_per_em()`。
2. **pt vs px vs twip**：1pt=20twip；96DPI 下 1pt=1.3333px；换算链在 S08/S16 各钉死一次。
3. **y 是基线不是顶**：行内坐标从基线算；第一个字顶 = 基线 − ascender 换算值。
4. **字体 fallback**：docx 里的“宋体”在没有该字体的系统会回落——**双跑对比必须两侧使用同一个字体文件**，否则 Stage2 立刻漂。
5. **CJK 全角宽**：一律从 hmtx 读，不作“= 2×ASCII”假设。
6. **hinting/AA 差异**：光栅化不在 Stage2/3 验收范围；若做像素级对照，只对照几何层。

---

## 8. 落到我们 feature 的映射

| 环节 | feature·function |
|------|------------------|
| 族名→文件/字节（FontHost） | wr-03 `04_font_host`（注入表 + SchemaItem；宿主读 Arial/打包 TTF） |
| 字体选择/换算 | wr-03 `01_font_stack`（ttf-parser，name_map + pt↔twip + Face） |
| 单字符 advance/行宽 | wr-03 `02_run_metrics` |
| 行高/基线/段前段后 | wr-04 `04_line_height` + wr-01 `03_typo_line` |
| 行内 x 对齐 | wr-04 `03_alignment` |
| 绘制指令（字体/字号/色/坐标） | wr-07 `01_text_command` |
| 光栅化（像素） | Flutter 引擎（无 Rust；S17） |