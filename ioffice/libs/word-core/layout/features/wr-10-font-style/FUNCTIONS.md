# wr-10-font-style — function 拆分

> 日期：2026-08-23  
> Feature：[FEATURE.md](./FEATURE.md) · 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)  
> 切在 **I/O**，不切 `FontStyleFactory` 整类。本波 **不** 调 FontHost、不度量、不断行。

## 0. 管线

```text
CHPX PropertySet + PAPX(pStyle) + KStyles + rPrDefault
  → 04 merge 层叠              最终 run PropertySet
  → 01 getFontSize             size twip（PT2TWIPS，默认 10pt）
  → 02 getBold / getItalic     b/i（prop 5/6；cs 38/39）
  → 03 ascii / eastAsia / 解析名  hint/lang 最小集
  → 05 TypoFontStyle           编排出口
```

Java 对照：`layout/base/env/FontStyleFactory.java` 的 `getFontSize` / `createTypeface` 取名与 bold/italic、`KStyle.getFinalProperty` basedOn 链。  
**不**移植 typeface 查找、ascent/descent、SmartFontSize、嵌入气球缩字。

## 1. 清单

| order | function_id | 一句话 | I | O | 上游 | 下游 | status |
|------|-------------|--------|---|---|------|------|--------|
| 1 | [01_function_font_size](functions/01_function_font_size/FUNCTION.md) | 字号 pt→twip | `PropertySet` + langType | `i32` twip | wc-03 | 05 | pass |
| 2 | [02_function_run_flags](functions/02_function_run_flags/FUNCTION.md) | 粗体/斜体 | `PropertySet` + langType | `(bold, italic)` | wc-03 | 05 | pass |
| 3 | [03_function_font_names](functions/03_function_font_names/FUNCTION.md) | ascii/eastAsia 名 | `PropertySet` + langType | 两套族名 + 解析名 | wc-03 | 05 | pass |
| 4 | [04_function_style_merge](functions/04_function_style_merge/FUNCTION.md) | rPrDefault∪样式∪直接 CHPX | CHPX + PAPX + KStyles | 合并 `PropertySet` | wc-02 | 05 | pass |
| 5 | [05_function_font_style_factory](functions/05_function_font_style_factory/FUNCTION.md) | 编排 TypoFontStyle | 单 run 输入 | `TypoFontStyle` | 01;02;03;04 | wr-11 | pass |

## 2. 数据流

`04` 产出已层叠的 CHP。`05` 对它跑 `01`–`03` 填 `TypoFontStyle`（ascii / eastAsia / size twip / bold / italic）。下游 wr-11 只许吃该结构，禁止再读 XML。

## 3. 编译面

crate：`libs/word-core`  
src：`src/cn/wps/moffice/writer/layout/base/env/{FontSize,RunFlags,FontNames,StyleMerge,TypoFontStyle,FontStyleFactory}.rs`  
test：`tests/` 镜像路径；`cargo test --test <function_id>`

## 4. 验收顺序

01 → 02 → 03 → 04 → 05。05 是 feature 出口。

## 5. 明确不做

FontHost / hmtx；jc/缩进/段距（随 PAPX 给 wr-12）；主题字体；改 KStyles parse。
