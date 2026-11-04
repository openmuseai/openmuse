# pp-16-scheme-color-resolve — Function 组合图

> 状态：**pass** ｜ 管线：[VISUAL_CONTENT_PIPELINE.md](../../../VISUAL_CONTENT_PIPELINE.md)

## §0 管线位置

```text
pp-04 theme part（叶 pass）
  → [01] 产品 zip 读 theme → scheme RGB 表
  → [02] spPr solidFill scheme/srgb → has_fill
  → [03] a:rPr solidFill → font_color（含嵌套 srgb，不只 attrs）
  → pr-05 / 现有 render_slide apply_fill_line + apply_run_style
```

## §1 清单

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_theme_scheme_table` | theme XML | 槽名→RGB | pp-04 | 02;03 | pass |
| 2 | `02_function_shape_fill_argb` | spPr fill + 表 | has_fill + ARGB | 01;pp-10:02 | pr-05 | pass |
| 3 | `03_function_run_fill_argb` | rPr solidFill + 表 | font_color | 01;pp-11 | pr-03;pr-05 | pass |

## §2 数据流

```text
a:clrScheme accent1/dk1/lt1/…
a:srgbClr val="0937BE" → 0xFF0937BE
a:schemeClr val="accent1" → 查表
FillPropertiesHandler commit 仅 fill_type==0 且 solid Some → 必须覆盖 scheme 解析后的 solid
```

## §3 测试面

内存 theme 最小 clrScheme + 一段 spPr/rPr。Sample_ppt 首页标题蓝 `0937BE` 必须出现在某条 DrawText.color 或 fill。

## §4 验收

无 theme 时 scheme 形状仍可走 srgb 直写；纯 scheme 无表 → 明确 miss（无 fill），禁止随机黑填。

## §5 loop

`01 → 02 → 03`

## §6 类级

| Java | function |
|---|---|
| `ThemeHandler` / `PptxrThemes` | 01 产品绑定 |
| `SchemeColorHandler` | 02/03 |
| `ColorHandler` srgb | 02/03 |
