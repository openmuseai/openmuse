# wr-12-line-layouter — function 拆分

> 日期：2026-08-24
> Feature：[FEATURE.md](./FEATURE.md) · 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)
> 切在 **I/O**，不切 `LineLayouter` 整类。`01`–`05` 已 **pass**。

现有 wr-04 `Wrap.rs` 只服务 MVP Hello（贪心空格），**不是**本 feature 的 Java 双子，禁止改它冒充 wr-12。

## 0. 管线

```text
wr-11 RunMetrics（per-cp advance + ascent/descent/line_gap）
  + wr-02 content_width（页宽 − 左右边距 − 本段左右缩进）
  + PAPX（jc=193，首行缩进=208，行距=220，段前=217，段后=214）
  → 01 ContentWidth     本行可用宽（首行再扣 208；悬挂为负则变宽）
  → 02 Wrap             按 Java 可断点折成 (start_cp, end_cp, width)[]
  → 03 Alignment        jc → 行起始 x（两端对齐 stub=左）
  → 04 LineBox          wr-11 行格 + 220 行距 + 首/末段距 → height / baseline
  → 05 LineLayouter     编排 TypoTextLine[]（FIRST/LAST of para）
```

Java 对照：`LineLayouter.java` 的 `flow` / `preCalcLineHeight` / `postCalcLineHeight` / `regulateToCache`；
断行在 `SubLineLayouter.addRun`（溢出阈值 **≥ 2 twip**）+ `SubLineLayouterBase.doAddExcessGraphs` / `rollback`；
首行缩进在 `SubLineLayouter.flow`（`layoutLeft += firstLineIndent`，仅 `firstSubLine`）；
对齐 `AlignLeftHandler` / `AlignCenterHandler` / `AlignRightHandler.adjustLine`；
行高 `LineHeightLayouter.calcLineHeightInfoByLineSpacingRule`。

**不**移植：`isOverflow` / `isOverflowBeforeFlow`（那是 **wr-13 分页**）、绕排 drawing、kinsoku/标点挤压、tab、bidi、`AlignBothHandler` 伸展。

## 1. 清单

| order | function_id | 一句话 | I | O | 上游 | 下游 | status |
|------|-------------|--------|---|---|------|------|--------|
| 1 | [01_function_content_width](functions/01_function_content_width/FUNCTION.md) | 本行可用宽 | wr-02 宽 + 首行缩进 + 是否首行 | `max_width` | wr-02 | 02;05 | pass |
| 2 | [02_function_wrap](functions/02_function_wrap/FUNCTION.md) | Java 断点折行 | advances[] + max_width | 行 cp 区间 + 行宽 | 01; wr-11 | 03;05 | pass |
| 3 | [03_function_alignment](functions/03_function_alignment/FUNCTION.md) | jc → x | 行宽 + 内容宽 + jc | `x` | 02 | 05 | pass |
| 4 | [04_function_line_box](functions/04_function_line_box/FUNCTION.md) | 行高/基线 | wr-11 行格 + PAPX 220/217/214 + 首/末 | height + baseline | wr-11 | 05 | pass |
| 5 | [05_function_line_layouter](functions/05_function_line_layouter/FUNCTION.md) | 编排 TypoTextLine[] | 度量 run + content_width + PAPX | 产品行数组 | 01–04 | wr-13 | pass |

## 2. 数据流

`01` 只在 wr-02 已扣左右缩进的 `content_width` 上再处理 **208 首行/悬挂**，禁止再扣 210/212。
`02` 只用 wr-11 的 per-cp advance，禁止均分 run 宽、禁止调用 wr-04 `wrap()`。
`05` 把 02 的 cp 区间、03 的 x、04 的 height/baseline 写入已有产品类型 `TypoTextLine`（`start_cp` / `end_cp` / `FLAG_FIRSTLINEOFPARA` / `FLAG_LASTLINEOFPARA`），**不发明第二套行类型**。

## 3. 编译面（loop 才接线；split 不改 Cargo.toml）

crate：`libs/word-core`
src（计划）：
- `src/cn/wps/moffice/writer/layout/base/env/{ContentWidth,LineWrap,LineAlign,LineBox}.rs`（I/O 门面）
- `src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs`（Java 双子；现网只有 wr-04 `cache_run_widths` 胶水，loop 生成完整交错模板后停车列表外方法）
test（计划）：`tests/` 镜像；`cargo test --test <function_id>`

## 4. 验收顺序

01 → 02 → 03 → 04 → 05。05 是 feature 出口。一行的 **cp 区间必须与 Java 金文件精确相等**（忽略 `page_index`）。

## 5. 明确不做

分页 / `finishPage` / `processPagination` / `isOverflow`（页高）；
两端对齐伸展、tab、bidi；
绕排、kinsoku、标点挤压（另开 feature）；
改 wr-04 `Wrap.rs` 冒充本 feature；
`FontStack::default()` 探盘。

## 6. 类级对照（loop 生成模板）

| Java | Rust | 本波 |
|------|------|------|
| `LineLayouter` | `layout/base/line/LineLayouter.rs` | 05 编排；其余方法 `todo!("DEFERRED_NON_FUNCTION")` |
| `SubLineLayouter.flow` 缩进块 | `env/ContentWidth.rs` | 01 门面 |
| `SubLineLayouter.addRun` + `rollback` | `env/LineWrap.rs` | 02 门面 |
| `AlignLeft/Center/RightHandler` | `env/LineAlign.rs` | 03 门面（justify=左） |
| `LineHeightLayouter` | `env/LineBox.rs` | 04 门面 |
| `TypoTextLine` | 已有 cache 类型 | 05 写入；不新造行类型 |
