# 04_function_papx_insert — PAPX 挂段首 CP

| 项 | 值 |
|----|----|
| feature | wc-01-storage-write |
| order | 4 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `PapxTable::{insert_entry, seek_entry, beforeRemoveText, beforeAddReference}` / `PapxTableFactory::createPapxTable` |
| 独立性 | 叶子（段首 CP 由调用方给出；不跑 appendParagraph） |
| 测试状态 | pass |

## 1. 功能说明

### 1.1 一句话

把段落 `PropertySet` 挂到 **段首 CP**（不是段末 endMark）。删除文本时按段节点 seek 同步。

### 1.2 输入

- `insert(cp0, papx)` — IO 主路径 cp0 是段落起始
- Factory 与 Chpx 对称：null document → null
- `beforeRemoveText`：seek(start)/seek(end)，同节点则只 eraseSpaceBefore；否则删中间段

### 1.3 输出

`IPapxTable.Entry`。`beforeAddReference` 调 `addStyleRef(..., 191)` **且** `addStyleRef(..., 292)`（sti + 样式引用）。

propId **191** = 段落样式 sti；**292** = 段落样式引用；**225** = `paragraph_rowEnd`（07 `supplyEndPara` 使用，本层只保证 papx 能存这个 bool）。

### 1.4 上游

wave0 PropertySet；01/02 的 CP 时钟。

### 1.5 下游

`CoreHelper.insertToPapxTable` / `appendParagraph(..., cp0, ...)`。

## 2. 核心代码与流程

与 Chpx 同属 `CPTable`，差异在：insert 语义（段首）、remove 按段 seek（不 split 字符中点）、style 191+292。

Java：`PapxTable.java`、`PapxTableFactory.java`  
Rust：同相对路径

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|--------|----------------|-----------|-------------|---------------|
| `PapxTableFactory::createPapxTable` | Factory.rs | null / 非 null | None / Some | |
| `insert_entry` | PapxTable | cp0 + papx | position==cp0 | 误用 endMark CP 是 07 的错，本层按传入 cp |
| `seek_entry` | 继承 CPTable | 段内 cp | 段首 Entry | |
| `beforeAddReference` | 191 然后 292 | styles 可 null | insert 仍成功 | |
| `beforeRemoveText` | PapxTable | s==e / s!=e；s.position!=start | eraseSpace 或删节点 | isInUndoRedo 跳过 |
| 直接调用方 | CoreHelper.insertToPapxTable | document.getPapxTable().insert | 本 function 直接 insert | |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|-------------|---------------|----------|-------|---------|
| C-FAC-NULL | factory null | 边界 | 是 | PX1 |
| C-FAC-SOME | factory 非 null | 主路径 | 是 | PX2 |
| C-INS-START | insert 段首 0 | 主路径 | 是 | PX3 |
| C-INS-CP0 | 第二段 insert(cp0=3) 不是 4 | 主路径 | 是 | PX4 |
| C-SEEK | seek 段内落到段首 | 分支 | 是 | PX5 |
| C-191-292 | papx 含 191，insert 不崩 | 交互 | 是 | PX3 |
| C-225 | 存 getBoolean(225) | 下游供给 | 是 | PX6 |
| C-RM-SAME | beforeRemoveText s==e | 分支 | 是 | PX7 |
| C-RM-SPAN | 跨两段删除 | 状态 | 是 | PX8 |

### 3.3 可测试性

与 03 同一 StorageDoc 缝。测 PX4 时 **不要** 先走 CoreHelper：手工 insert(3, papx) 并断言 position==3。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|----|--------|----------|----------|-----------------|----------|
| PX1 | C-FAC-NULL | — | createPapxTable(null) | None | |
| PX2 | C-FAC-SOME | StorageDoc | factory(doc) | Some | |
| PX3 | C-INS-START | 空 | insert(0, papx{191:1}) | position=0, getInt(191)=1 | |
| PX4 | C-INS-CP0 | 已有段 [0,3) | insert(3, papx2) | 新 entry.position=3 不是 4 | 挂到 endMark |
| PX5 | C-SEEK | 段 [0,3) | seek(1) | position=0 | |
| PX6 | C-225 | 空 | insert papx boolean 225=true | getBoolean(225,false)=true | |
| PX7 | C-RM-SAME | 一段 | beforeRemoveText 落在同一 papx | 节点还在，区间缩短 | 整段删光 |
| PX8 | C-RM-SPAN | 两段 | 删除覆盖第一段末到第二段 | 中间 Entry 删除 | |

### 3.5 测试代码位置

crate: `libs/word-core/Cargo.toml`  
src:   `libs/word-core/src/cn/wps/moffice/writer/data/internal_v2/{PapxTable,PapxTableFactory}.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/data/internal_v2/PapxTable.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_papx_insert

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 04_function_papx_insert \
  --src src/cn/wps/moffice/writer/data/internal_v2/PapxTable.rs \
  --src src/cn/wps/moffice/writer/data/internal_v2/PapxTableFactory.rs \
  --report libs/word-core/parser/features/wc-01-storage-write/functions/04_function_papx_insert/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_papx_insert --json`)
date: 2026-08-19 10:13

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/data/internal_v2/PapxTable.rs` | 98/230 | 42.6% | 209/520 | 40.2% |
| `src/cn/wps/moffice/writer/data/internal_v2/PapxTableFactory.rs` | 5/5 | 100.0% | 8/8 | 100.0% |
| **function total** | **103/235** | **43.8%** |  |  |
<!-- coverage-report:end -->
