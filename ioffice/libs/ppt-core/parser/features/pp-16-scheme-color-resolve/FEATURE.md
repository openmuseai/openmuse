# pp-16-scheme-color-resolve — 主题色落到 fill/run

> 层：parser ｜ 状态：**pass** ｜ depends_on: pp-03; pp-04; pp-10:02  
> Sample 大量 `a:schemeClr`；无 RGB 则 `hasFill==false`，只剩框架。

## 目标

文档打开时加载 theme（或 themeOverride），把 `schemeClr val` 和 `srgbClr val` 写成 ARGB，供 `render_slide` 的 fill 与 DrawText 颜色使用。

## I/O

| 输入 | 输出 |
|---|---|
| `ppt/theme/theme1.xml`（或 slide themeOverride） | scheme 槽 → RGB |
| `a:solidFill/a:schemeClr` 或 `a:srgbClr`（spPr 与 rPr） | `ShapePr.has_fill` + `fill`；run `font_color` |
