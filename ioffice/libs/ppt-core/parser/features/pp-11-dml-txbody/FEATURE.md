# pp-11-dml-txbody — 文本框 txBody

> 状态：**pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（2 个 function，均 pass）
> 依赖：pp-01-pptx-parse; pp-02-shape-tree-parse; pp-10-dml-sppr
> 管线：[PIPELINE.md](../PIPELINE.md) / [README.md](../README.md) / [MVP.md](../../MVP.md)

## 目标

把 `p:txBody` 子树从 pp-02 的 constructor fixture 改写成**写入 `KmoTextBody`（Rust：
`data/text/TextBody.rs`）**：段落（`a:p`）、run 链（`a:r`/`a:br`/`a:fld` + `a:t`），文本属性
（bodyPr/paraPr/endParaRPr/rPr 的最小集）。对齐 MVP.md：**pp-11 是有字才必要**——无字可只画矩形
（pp-10 已覆盖形状体）；有字时缺 txBody 则文本框无字，pl-04 排版无原料、pr-03 无文本指令。
本 feature 只负责纯文本 + 基础排版属性进模型，排版/渲染归 pl-*/pr-*。

## I/O（锁死）

| function | I | O | 契约 |
|---|---|---|---|
| 01 text_body | `p:txBody`（bodyPr=1114448、lstStyle=1114449、p=1114562）子事件 + `TextBody` | `paraLst`（`TextParagraph[]`：r/br/fld run 链 + 纯文本 + 基础 rPr） | 段落/run 用单一 handler `init` 循环复用；`endElement` 才 `paraLst().forceWrite`；GVML 时 `onMergeThemeTypeface` |
| 02 text_body_props | `a:bodyPr` attrs + `TextBodyProperties` | insets（lIns=1114551/tIns=1114552/rIns=1114553/bIns=1114554）+ anchor=1114301 + autofit（no/norm/sp=1114545/1114546/1114547）+ vert=1114480 + wrap=1114550 + rot 等二十项 | 由 01 的 bodyPr 分支入；normAutofit 的 fontScale=1114543/lnSpcReduction=1114544；3D/flatTx/prstTxWarp 延后 |

## Java 管线对照

Java 渲染管线 **S0 parse** 环节：`ShowShapeHandler` 族的 `TextBodyHandler` 把 `txBody`→
`KmoTextBody`（per-para/run 富文本）写进 `KmoShowShape`。**S2 layout** 环节
`ShapeDataTransform.exToInternal` 消费 `KmoTextBody` 的 para/run → `TextBoxData`（pl-04），
`renderModel.getTextRect` 消费 `TextBodyProperties` 的 insets/anchor（pl-05），`TextRender` 画字
（pr-03）。本 feature 是 **S1 convert 输入侧靠前的文本模型原料**：只补 S0 的 txBody 写入，
不排版。

## 能力解锁

- 实现并测试通过后：真实 pptx 的文字进模型 → pl-04 有排版原料（para/run）→ pl-05 有 insets/
  anchor/autofit 依据 → pr-03 能出 `DrawText` → **有字的文本框能画出字**。
- 缺失时：txBody 不解析 → 文本框空 → pl-04 无原料、`getTextRect` 无 insets 依据 → **有字必断**
  （无字 pptx 可只靠 pp-10 画矩形，MVP 语义成立）。

## 必须 / 壳 / 延后

- **must：** 01 的段落/run 循环复用与 `paraLst().forceWrite` 时序；r/br/fld 三种 run mode（0/1/2）
  与 `t`（1114214）文本写入；rPr 最小集：sz=1114589/b=1114195/i=1114504/u=1114590/strike=1114591/
  lang=1114587/kern=1114592/spc=1114593/baseline=1114595/cap=1114441——足以为 pl-05/pr-03 提供
  字号/字体斜体下划线。02 的 insets 四值 + anchor + vert + wrap + autofit 三型 type（0/1/2）。
- **壳：** paraPr 全量（对齐/缩进/行距，随 pl-05 需要再补）、lstStyle 引用（归 pp-03）、fld 的
  字段类型解析、endParaRPr 全量。
- **延后：** `TextBodyProperties` 的 3D（scene3d/sp3d）、flatTx、prstTxWarp、多栏
  （numCol/spcCol/rtlCol）、rPr 效果（ln/fill/effect/高亮/字符集字体我族 latin/ea/cs/sym 中的非
  最小集）；超链接（hlinkClick/hlinkMouseOver）。

## 范围

- **in：** `p:txBody`→`TextBody` 容器；`a:p` 段落列表；`a:r`（mode 0，`a:t` 纯文本）+ `a:br`
  （mode 1）+ `a:fld`（mode 2 最小：id/type 落 `fieldRun`）；每个 run 的 rPr 最小集；bodyPr
  的 insets/anchor/vert/wrap/autofit/rot/overflow 四型（horz 1114482/vert 1114549）。
- **out：** 排版（pl-04/05）；lstStyle 主题文本样式（pp-03）；reader/part 打开（pp-07 起）；
  富文本全量（渐变线/高亮/下划线详情/超链接目标解析）；`onMergeThemeTypeface` 的中文字体合并
  业务体（GVML 语义，Stub）。

## 验收

1. 01 用内存 attrs/events 完成段落/run 链写入：`paraLst` 顺序、run mode、`t` 文本、rPr 最小集
   与 Java 一致；GVML `onMergeThemeTypeface` 调用时机已锁定（callback null 判空）。
2. 02 用 bodyPr fixture 完成 insets/anchor/vert/wrap/autofit 写入，`TextBodyProperties` 可被
   pl-04 contract 消费。
3. 不打开 part、不装配 slide；`cargo test --test 01_function_text_body` / `02_function_text_body_props` 绿。
4. loop 完成后 status 由 review 裁决；覆盖率/对齐报告按 FUNCTION.md §3.7/§3.9 补。

## 依赖

- **depends：** pp-01（`HandlerCallback`（pp-01:02 pass 语义）、`XmlRoAttr` 事件面、
  `SWReaderMap` 查表）；pp-02（spTree 分发生成 txBody 子事件；pp-02:02/03/04 构造
  `TextBodyHandler`）；**pp-10-dml-sppr**（pp-10 先交付 spPr 使形状可画，txBody 挂在同一形状
  `KmoShowShape` 上；01 的 rPr fill/ln 分支复用 pp-10:02/03 的共享 handler）；数据模型
  `data/text/*.rs`（TextBody 953 行、TextParagraph 953 行、TextRun 2089 行、
  TextBodyProperties 3209 行均已在）。
- **does not depend：** pp-07 起的 part open；渲染；lstStyle 主题回填（pp-03）；`RichTextLayout`
  排版引擎（pl-05）。

## 不过扩展

- 禁止打开 `OpcrPackagePart`（PIPELINE.md §1 故意缺口）；禁止装配 slide；禁止排版/渲染。
- **禁止改 pp-01/pp-02 已 pass 语义**（`HandlerCallback` 回调、DgStore/spTree 提交）；不接受在
  pp-11 内「顺手改」pp-10 的 handler。
- 阔文本（t 的 whitespace 语义）与 Java `characters(String)` 逐字符语义对齐，不允许 Rust String
  处理把空白/换行静默吃掉；未知属性不写入（与 Java switch default 一致）。