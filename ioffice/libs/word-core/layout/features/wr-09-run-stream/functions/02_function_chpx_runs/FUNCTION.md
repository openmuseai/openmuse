# 02_function_chpx_runs — CHPX 切开 run

| 项 | 值 |
|----|----|
| feature | wr-09-run-stream |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `ChpxRuns::walkChpx(table, start, end) -> Vec<ChpxSpan>` |
| 独立性 | 只读 `IChpxTable.seek/getNext`；不取文本、不碰 PAPX |

## 1. 功能说明

### 1.1 一句话
按 Java `IChpxTable.seek` 游标，把 `[start,end)` 切成 CHPX 不变的半开区间。

### 1.2 输入
`&dyn IChpxTable` + `[start, end)`。调用方保证区间落在主故事。

### 1.3 输出
`ChpxSpan { start_cp, end_cp, property: PropertySet }` 有序、不重叠、并集覆盖查询区间与表内 run 的交集。Head 节点跳过；`isEnd()` 停止。

### 1.4 / 1.5
上游：wc-02 ChpxTable。下游：04 与 PAPX 求交。

### 1.6 不变量
1. `start_cp < end_cp`；相邻 `end==next.start`。
2. 查询窗口外的 run 部分被 clip。
3. `start>=end` → `[]`。
4. 只 **clone** 入口上的 PropertySet，不 merge 样式、不算字号。

### 1.7 依赖

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止改 |
|------|------|------------|--------|------|--------|
| `IChpxTable.seek` | wc-02 | seek/begin/end | 是 | — | ChpxTable.rs |
| `Entry.position/endCp/getNext/isEnd/isHead/getProperty` | wc-02 | 游标 | 是 | Head.endCp panic → 必须跳过 Head | — |

### 1.8 函数改写列表
- `walkChpx`（Java：`IChpxTable.seek` + `GraphReader.readCoreRangeDefault` 内 `run.getNext()` 循环的 **切段部分**）

## 2. 核心代码

```text
run = table.seek(start)
while run != null && !run.isEnd() && run.position() < end:
  if !run.isHead():
    clip to [start,end); emit
  run = run.getNext()
```

## 3. 测试方案

### 3.1 审计
| symbol | 路径 | 分支 |
|--------|------|------|
| walkChpx | 空表/空窗 | [] |
| walkChpx | 单 run 覆盖窗口 | 1 span |
| walkChpx | CHPX 变化切开 | N span |
| walkChpx | clip 左右 | start/end 截断 |
| walkChpx | Head 跳过 | 不 panic |

### 3.2 覆盖矩阵
| coverage_id | 必测 | test_id |
|-------------|------|---------|
| CX-EMPTY | 是 | CX1 |
| CX-ONE | 是 | CX2 |
| CX-SPLIT | 是 | CX3 |
| CX-CLIP-L | 是 | CX4 |
| CX-CLIP-R | 是 | CX5 |
| CX-WINDOW-EMPTY | 是 | CX6 |
| CX-COVER | 是 | CX7 区间并集=窗口∩表 |

### 3.3 可测试性
`CoreHelper.appendText` 换不同 PropertySet 制造切开。测 N 只断言 span 的 cp 与 `getInt` 区分，不拼 text。

### 3.4 用例
| id | 输入摘要 | 期望 |
|----|----------|------|
| CX1 | 空主故事 seek | [] 或 0 条有效 span |
| CX2 | 一次 append `"ab"` | 一条 [0,2) |
| CX3 | `"ab"` + 另一 PropertySet `"cd"` | 两条，cp 0-2 / 2-4，property 可区分 |
| CX4 | 窗口 [1,4) 跨切开 | 第一条 start=1 |
| CX5 | 窗口截在末 run 中间 | 末 span end=窗口 end |
| CX6 | start==end | [] |
| CX7 | 全窗口 | 并集覆盖 [0,len) |

### 3.5 位置
src: `.../text/ChpxRuns.rs`  
test: `tests/.../text/ChpxRuns.rs`

### 3.6 脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_chpx_runs

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_chpx_runs \
  --src src/cn/wps/moffice/writer/layout/base/text/ChpxRuns.rs \
  --report libs/word-core/layout/features/wr-09-run-stream/functions/02_function_chpx_runs/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_chpx_runs --json`)
date: 2026-08-23 15:31

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/text/ChpxRuns.rs` | 25/29 | 86.2% | 45/50 | 90.0% |
| **function total** | **25/29** | **86.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 布局
```bash
node .codex/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/text/ChpxRuns.rs
```
