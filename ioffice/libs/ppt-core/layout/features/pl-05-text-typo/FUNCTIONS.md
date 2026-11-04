# pl-05-text-typo — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**，不切 `RichTextLayout` 整类。`01`–`02` 均为 **pass**。
> **一致性红线：** MVP 贪心断行 ≠ Java `RichTextLayout`；Java 一致断行另开 feature（详细见 FEATURE.md 头部声明）。

## 0. 管线位置

```text
S2 布局： pl-04 TextBoxData（段文本 + 框属性）
  → 02 TextBoxGeometry   BaseTextLayout.setTextRect 语义 → 内缩矩形（可用宽/绘制区）
  → 01 GreedyWrap        贪心断行（对照 RichTextLayout.typo 的断点层语义）→ 行列表
  → pl-06 挂行几何（可选） → pr-03 DrawText（render S5）
```

Java 对照：`ShowTextLayout.layout`（`ShowTextLayout.java:40`，编排；`typo(boxData, w, h, forceTypo)` 在 `:56`）→ `ShowRichTextLayout.typo` → `RichTextLayout.typo`（`RichTextLayout.java:78`）/`typoOnePara`（`:528`）；文本区几何 = `BaseTextLayout.setTextRect`（`BaseTextLayout.java:88`，**02 精确对齐**）；绘制参考 `ShapeTextRender._render` → `renderModel.getTextRect`（`Text.getTextRect`→`Shape.getTextRect`（`Shape.java:948`）→`DataCal.getTextRect`（`DataCal.java:55`））。

**不**移植：`TypoLayoutGen`/Android `Layout`/`TextPaint` 度量、`tryTypoNormalAutoFit`/`scaleTextPara`/autofit 连锁、竖排 `correctInset` 交换、`WordArtLayout`。

## 1. 清单

| order | function_id | 一句话 | I | O | 上游 | 下游 | status |
|------|-------------|--------|---|---|------|------|--------|
| 1 | [01_function_greedy_wrap](functions/01_function_greedy_wrap/FUNCTION.md) | 框内贪心断行 | `TextBoxData` + 度量表 + 可用宽 | 行列表（cp 区间/行高/行首 x） | pl-04; 02 | pl-06; pr-03 | pass |
| 2 | [02_function_text_box_geometry](functions/02_function_text_box_geometry/FUNCTION.md) | 文本区内缩几何 | `TextBoxData` + boxW×boxH | 内缩矩形 | pl-04; pl-01 | 01; pr-03 | pass |

## 2. 数据流

`02` 先算内缩矩形（`setTextRect` 公式，栅格值 `(left, top, right, bottom)`，左上为框原点），`01` 以此为可用宽并返回行列表；行 `x/y` 在框坐标内（`x=left`、`y=top+Σ行高`，中/右对齐与垂直 anchor 为 stub 三态）。宽度**必须**来自 02 或显式 fixture——禁止假宽度。所有数值 layout 单位，与 pl-04 insets 同源。

## 3. 测试面

- crate：`libs/ppt-core/Cargo.toml`（输入 `TextBoxData` 从 `wpslayout` 依赖引入；门面放在 ppt-core）。
- 测试不跑 `ShowTextLayout.layout` 全链；`01` 喂 fixture `TextBoxData` + 显式度量表（`advance[]`），`02` 喂 insets fixture。
- 断言点：01 行 `startCp/endCp` 逐行 + 行宽/行高/`x,y`；02 的 5 组 inset 与 Java `setTextRect` 手工算值 ±0.01px；wrap=false 单行长；超宽单 token 强制折。
- Java 对照用例只在 02（公式精确对齐）；01 的 Java 对照是「参考语义」，不设金文件等价断言（Java 一致版另开）。
- 待生成 `[[test]]`：`01_function_greedy_wrap` / `02_function_text_box_geometry`（loop 时才写）。

## 4. 验收

02 → 01。02 对 Java `setTextRect` 全分支（含 atten 均衡）±0.01px；01 行 cp 区间与手算期望相等、`x/y` 吞 insets+anchor 三态。**01 不得宣称与 Java 断行一致**；一致性验收另开 feature 承接。

## 5. loop

顺序 `02 → 01`（02 是 01 的宽度来源，先绿 02 再 loop 01）。每个 function 独立「改写 → 测试 → 回归」。loop 列表外方法（autofit 连锁、`typoOnePara` 的 Java 全量、bidi）以 `todo!("DEFERRED_NON_FUNCTION")` 停车；`RichTextLayout.rs`/`BaseTextLayout.rs` 模板的 `// Java:` 注释不得删除。

## 6. Java/Rust 范围

| Java | Rust（计划路径） | 本波 |
|------|------------------|------|
| `BaseTextLayout.setTextRect`（`BaseTextLayout.java:88`） | `libs/core-libs/wpslayout/src/cn/wps/text/layout/typo/BaseTextLayout.rs`（现有 76 行模板，`setTextRect` 未实现） | 02 的公式源；本波在 ppt-core 门面 `layout/env/TextBoxGeometry.rs` 复刻该公式（wpslayout 侧停车） |
| `RichTextLayout.typo`/`typoOnePara`（`RichTextLayout.java:78/528`） | `libs/core-libs/wpslayout/src/cn/wps/text/layout/typo/RichTextLayout.rs`（5346 行模板） | 01 只参考断点层语义；Java 一致版另开，本波不填模板 |
| `ShapeTextRender/Text.getTextRect/Shape.getTextRect/DataCal.getTextRect` | `libs/ppt-core/src/cn/wps/show/render/`（ShapeTextRender.rs 待 loop；drawing crate 的 Text/Shape/DataCal 已有） | 绘制侧参考；MVP 按普通形状 `(0,0,w,h)` 处理 |
| `TextBoxData`（insets/anchor/wrap） | `libs/core-libs/wpslayout/src/cn/wps/text/layout/data/TextBoxData.rs` | 输入读取端 |
| 行列表类型（MVP 新增） | `libs/ppt-core/src/cn/wps/show/layout/typo/ShowTypoLine.rs`（新，不发明与 writer `TypoTextLine` 混淆的名） | 01 输出；**待 loop 确认字段名**（对齐 pr-03 消费） |