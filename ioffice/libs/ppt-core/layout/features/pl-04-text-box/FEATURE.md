# pl-04-text-box — shape → TextBoxData

> 层：layout ｜ 状态：**pass** ｜ Java：`ShapeDataTransform.exToInternal`（基类 `DataTransform` + `TextBoxData`/`TextData`/`ParaInfo`）

## 目标

把 `KmoShowShape`（含 `txBody`/`spPr`）翻成内部排版输入 `TextBoxData`：每段一个 `TextData`（`mSourceText` + `ParaInfo` + `SpanInfo[]`），外加文本框级属性（insets/anchor/vertical/wrap/autofit）与 `phType`。这是 Java S2「`ShowTextLayout.layout` → `exToInternal`」的 Rust 对应。**只产可排数据，不断行。**

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | `KmoShowShape`（含 `KmoTextBody`/`Transform2D`/`ph`）+ `KmoContext`（slide 场景上下文） |
| **O** | `TextBoxData`：`TextData[]`（每段 `mSourceText`、`mPara`、`mFirstSpan`+`mOtherSpans`）＋ 框属性（`leftInset/rightInset/topInset/bottomInset`、`anchor`、`isAnchorCenter`、`textVerticalType`、`isWrapTextInShape`、`textAutofit`、`fontScale`、`lnSpcReduction`、`presetTextWarp`）+ `phType` + `canRenderCache`。**空 body → empty `TextBoxData`**（Java `ShowTextLayout.layout` 对 `boxData.isEmpty()` 返回 `null`）。 |

单位锁死：框属性一律 **layout 单位**（`UnitsConverter.layout()`；Java `emuToLayoutUnits = units.fromEmuY(emu)`，见 `DataTransform.java:1266`）。EMU 原始值与换算公式从 Java 抄，不发明第二套单位名。

## Java 管线对照

S2 断点③文本排版（`PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S2）：

```text
ShowTextLayout.layout(typoRect, shape, ctx, forceTypo, cb)
  → ShapeDataTransform.exToInternal(shape, ctx)          // DataTransform.java:75（final）
      → getTextOperator                                    // ShapeDataTransform.java:17
      → covertTextBodyProperties（Java 原拼写）            // DataTransform.java:222
      → 循环 textOp.seek(0)/next(0)：fetchSpanInfo 产 TextData + convertBullet
  → 空则 null；否则 ShowRichTextLayout().typo(...)        // 交 pl-05
```

- `getTextOperator`（`ShapeDataTransform.java:17`）：`hasTextBody()` → `shape.getTextOperator()`；否则 `ph().type()==15`（note）也取；body 内 `resetDateFieldTextRuns(false)` / `resetSlideNumFieldTextRuns(slideNumber-1, false)` 决定 `canRenderCache`。
- 逐段翻：`getHAlign`（Java 0/1/2/3/4 → ParaInfo 1/2/3/5/7）、`marginLeft/indent/spcBef/spcAft`（emu→layout units）、`lnSpc`（spcPct/spcPts → `mSpcWithinPct/mSpcWithinPts`）、`TextParaOptions`（`eaLnBrk/latinLnBrk/hangingPunct/rightToLeft`）、bullet。
- 框属性：`covertTextBodyProperties`/`covertTextProperties`（`DataTransform.java:222/252`）——默认 inset 91440/45720 EMU；`anchor` → `DefineConverter.convertTxtAnchor`；`textWrappingType()==0` → 不折行；`textAutofit`（type/fontScale/lnSpcReduction）、`presetTextWarp().prst()`。
- `ShapeHolder.setTextRect`（doc→layout 换算，写回 shape 缓存）：MVP 只留字段不回写。

## 能力解锁

| function | Java 对照 | 实现后管线新增 |
|---|---|---|
| `01_function_shape_to_textbox` | `exToInternal`（段/run → TextBoxData） | 有字 shape 的文本进入内部排版模型 → pl-05 输入就绪 |
| `02_function_textbox_props` | `covertTextBodyProperties`/`covertTextProperties`（insets/anchor/autofit/vertical） | 框属性可查询 → 文本区几何（pl-05-02）与绘制 x/y 有依据 |

缺失断点：**pp-11 没有 txBody → getTextOperator 恒空 → pl-04 永远输出 empty**（有字 slide 必断，见 `PIPELINE_LAYOUT_RENDER_PLAN.md` §3.2）。

## 必须 / 壳 / 延后

- **must：** 段文本 + SpanInfo 最小集（`mFontName/mFontSize/mFontColor/mIsBold/mIsItalic/mStartPos/mEndPos`）、para 最小集（`mHAlignment` 左中右、`mMarginL/mIndent/mSpcBef/mSpcAft`）、insets（默认 91440/45720 EMU → layout 单位）、`anchor`（t/b/ctr 最小集 + `isAnchorCenter`）、`isWrapTextInShape`、`textVerticalType`、`phType`、空 body → empty。
- **stub：** bullet 只落 `buNone`/`buChar` 布尔与类型号；`textAutofit`/`presetTextWarp` 只解析不生效；`oldAutoFit/oldAnchor` 快照记录（`ShapeDataTransform.java:42`）；日期/页码域重置（`canRenderCache` 固定 true）。
- **defer：** 表格 `CellDataTransform`、竖排排版语义、autofit 缩放连锁、字体效果（outline/shadow/reflection/fill 的复杂分支）。

## 范围

- **in：** 普通 shape 与 group 子 shape 的文本框；ph 形状（`ph.type()==15` note 文本）。
- **out：** 表格 cell、graphicFrame 内文本、SmartArt 文本；Canvas；断行。

## 验收

fixture `KmoShowShape`（手工构造 `KmoTextBody`）→ 输出 `TextBoxData` 的字段与 Java 行为对标：段/span `mStartPos/mEndPos` 区间、`mSourceText`、insets 换算（91440/45720 EMU → layout 单位）、`anchor` 映射、空 body → `isEmpty()==true` 且 `ShowTextLayout.layout` 语义返回 null。Stage2 几何对齐 ±0.01px（对齐 writer 口径，不做像素门闩）。

## 依赖

- **upstream：** pp-02 形状树（fixture 可绕）；**pp-11-dml-txbody**（txBody → `KmoTextBody`，真实文件必需）；pl-03-ph-merge（ph 文本样式/几何补齐）；pl-01 单位（layout 档）。
- **downstream：** pl-05。
- **fixture 捷径：** 手工 `KmoTextBody`（对齐 `PIPELINE_LAYOUT_RENDER_PLAN.md` §2.3 产品捷径轨）。

## 不过扩展

禁止整文件移植 `DataTransform`（effect/blip/gradient 等文本效果分支）。禁止在 pl-04 断行。禁止绕过 `getTextOperator` 用假文本冒充 pp-11 而标 pass。本 feature 不产生行列表——那属于 pl-05。