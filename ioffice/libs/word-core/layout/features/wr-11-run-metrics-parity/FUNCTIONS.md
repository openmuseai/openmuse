# wr-11-run-metrics-parity — function 拆分

> 日期：2026-08-24
> Feature：[FEATURE.md](./FEATURE.md) · 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)
> 切在 **I/O**，不切 `TypefaceTool` 整类。本波 **不** 断行（wr-12），**不** `FontStack::default()` 探盘。

## 0. 管线

```text
wr-09 text + wr-10 TypoFontStyle + FontHost 注入字节
  → 03 FaceSelect            每 cp：ascii vs eastAsia 族名
  → 02 GlyphAdvance          该 face 上 hmtx → twip（缺 cmap 失败）
  → 01 UnitsToTwip           units × size_pt × 20 / upem
  → 04 LineMetrics           hhea ascent/descent/lineGap → twip
  → 05 RunMetrics            advance[] + width + 行格（编排出口）
```

Java 对照：`layout/base/tools/TypefaceTool.java` 的 `getCharWidths` / `getFontMetrics`；换算与现 `FontStack` 一致。
**不**移植 Paint sticky CJK、kerning、ligature、fake-bold 加宽。

换算钉死：`round(units × size_pt × 20 / upem)` = `round(units × size_twip / upem)`。
FEATURE 文案里 `units × size_twip × 20 / upem` 会 20 倍放大，实现与蓝皮书 / wr-03 金文件一致，不采用误读式。

## 1. 清单

| order | function_id | 一句话 | I | O | 上游 | 下游 | status |
|------|-------------|--------|---|---|------|------|--------|
| 1 | [01_function_units_to_twip](functions/01_function_units_to_twip/FUNCTION.md) | font units→twip | units + size + upem | i32 twip | wr-03 | 02;04;05 | pass |
| 2 | [02_function_glyph_advance](functions/02_function_glyph_advance/FUNCTION.md) | 单字 hmtx | FontStack + face + char + pt | advance twip / MissingGlyph | 01 | 05 | pass |
| 3 | [03_function_face_select](functions/03_function_face_select/FUNCTION.md) | ascii/eastAsia | TypoFontStyle + char | 族名 | wr-10 | 05 | pass |
| 4 | [04_function_line_metrics](functions/04_function_line_metrics/FUNCTION.md) | 行格 | FontStack + face + pt | ascent/descent/line_gap | 01 | 05 | pass |
| 5 | [05_function_run_metrics](functions/05_function_run_metrics/FUNCTION.md) | 编排 RunMetrics | text + style + FontHost | product RunMetrics | 01–04 | wr-12 | pass |

## 2. 数据流

`03` 按字符选 face。`02` 只在该注入 TTF 上读 hmtx（无 covering 链）。`05` 汇总 `advance[]`、width、行格。bold/italic 本波不改 advance。

## 3. 编译面

crate：`libs/word-core`
src：`src/cn/wps/moffice/writer/layout/base/env/{UnitsToTwip,GlyphAdvance,FaceSelect,LineMetrics}.rs`、`base/line/RunMetrics.rs`、`base/tools/TypefaceTool.rs`、`text/FontStack.rs`
test：`tests/` 镜像；`cargo test --test <function_id>`

## 4. 验收顺序

01 → 02 → 03 → 04 → 05。05 是 feature 出口。

## 5. 明确不做

wrap / LineLayouter；FontStack::default() 探盘；硬编码 Hello 宽度；合成粗体宽度偏移。
