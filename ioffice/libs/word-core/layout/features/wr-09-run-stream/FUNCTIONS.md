# wr-09-run-stream — function 拆分

> 日期：2026-08-23  
> Feature：[FEATURE.md](./FEATURE.md) · 计划：[LINE_PARITY_PLAN.md](../../LINE_PARITY_PLAN.md)  
> 切在 **I/O**，不切 GraphReader 整类。本波 **不** 解析 rPr、不度量、不断行。

## 0. 管线

```text
TextDocument 主故事
  → 01 getText[start,end)          字符（含 \r/\f）
  → 02 IChpxTable.seek 游标        CHPX 区间
  → 03 IPapxTable.seek 游标        PAPX 段区间
  → 04 区间求交 + 填 text          LayoutSourceRun[]
```

Java 对照：`SubDocument.getText` · `IChpxTable.seek` / `GraphReader.readCoreRangeDefault` 的 CHPX 游标 · `IPapxTable.seek` · `LayoutState.initCoreContent` 的主故事范围。  
**不**移植 GraphReader 的 filter/域/数学/drawing。

## 1. 清单

| order | function_id | 一句话 | I | O | 上游 | 下游 | status |
|------|-------------|--------|---|---|------|------|--------|
| 1 | [01_function_cp_text](functions/01_function_cp_text/FUNCTION.md) | 按 cp 取 rope 文本 | `Document` + `[start,end)` | `String`（保留 `\r`/`\f`） | wc-02 | 04 | pass |
| 2 | [02_function_chpx_runs](functions/02_function_chpx_runs/FUNCTION.md) | CHPX 切开 run | `IChpxTable` + `[start,end)` | `ChpxSpan[]` | wc-02 | 04 | pass |
| 3 | [03_function_papx_paras](functions/03_function_papx_paras/FUNCTION.md) | PAPX 切段 | `IPapxTable` + `[start,end)` | `PapxSpan[]` | wc-02 | 04 | pass |
| 4 | [04_function_run_stream](functions/04_function_run_stream/FUNCTION.md) | 编排主故事 run 流 | `Document` 主故事 | `LayoutSourceRun[]` | 01;02;03 | wr-10 | pass |

## 2. 数据流

`04` 对每个 PAPX 段与每个 CHPX run 求交，再用 `01` 填 `text`。下游只许吃 `LayoutSourceRun`，禁止再开 zip。

## 3. 编译面

crate：`libs/word-core`  
src：`src/cn/wps/moffice/writer/layout/base/text/{CpText,ChpxRuns,PapxParas,RunStream}.rs`  
test：`tests/` 镜像路径；`cargo test --test <function_id>`

## 4. 验收顺序

01 → 02 → 03 → 04。04 是 feature 出口。

## 5. 明确不做

表/图/页眉 story；FontStyleFactory；改 DocxReader。
