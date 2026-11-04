# 06_function_cpiecetable — 存储门面

| 项 | 值 |
|----|----|
| feature | wc-01-storage-write |
| order | 6 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `CPieceTable::{CPieceTable, CPieceTable__empty, setGuessedTotalLength, getGuessedTotalLength, getTextRope, charAt, dispose, getParagraphRange__int}` |
| 独立性 | 叶子（empty 构造不依赖 Document；委托路径需要三件套） |
| 测试状态 | pass |

## 1. 功能说明

### 1.1 一句话

聚合 `ITextRope` + `IChpxTable` + `IPapxTable` 的门面。IO 热路径 **不** 经它 append；DocxReader 用 `setGuessedTotalLength`。

### 1.2 输入

- `CPieceTable(document)`：从 document 取 transaction（KSAssert 非 null）、rope、chpx、papx
- `CPieceTable()` 空构造：全 None，guessed=0
- `setGuessedTotalLength(length)` 字段写入

### 1.3 输出

- getters 返回构造时抓住的指针
- `charAt(cp)` 委托 rope（rope null 时需定义：当前实现走 rope）
- `dispose` 清空指针
- `getCP` / `position` = entry.position()
- 深度方法（isShape / atomicInsertField / …）属 wc-03/04，本 function **不测**

### 1.4 上游

01/03/04 的三件套。

### 1.5 下游

Document.getPieceTable()；07 不强制经过门面。

## 2. 核心代码与流程

Java：`writer/data/CPieceTable.java`  
Rust：`…/CPieceTable.rs`

空构造 `CPieceTable__empty` ↔ Java `CPieceTable()`。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|--------|----------------|-----------|-------------|---------------|
| `CPieceTable__empty` | CPieceTable.rs | 无 | guessed=0, rope/chpx/papx None | |
| `setGuessedTotalLength` / get | | i32 | 字段往返 | DocxReader 调用点 |
| `CPieceTable(document)` | document null / 无 transaction | KSAssert / null 字段 | transaction 必有 | |
| `getTextRope` / getChpx / getPapx | 已绑定 | 同一指针 | | |
| `charAt` | 委托 rope | 与 01 相同 | rope None | |
| `dispose` | 已绑定 | 全 None | | |
| `getParagraphRange__int` | papx.seek(cp).range() | papx None → 0 | | |
| 直接调用方 | DocxReader setGuessedTotalLength | 本层只测字段 | |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|-------------|---------------|----------|-------|---------|
| C-EMPTY | empty 构造 | 主路径 | 是 | PT1 |
| C-GUESS | set/get guessed | 主路径 | 是 | PT2 |
| C-GUESS0 | 默认 0 | 边界 | 是 | PT1 |
| C-BIND | 从 Document 取三件套 | 主路径 | 是 | PT3 |
| C-CHARAT | 委托 rope charAt | 交互 | 是 | PT4 |
| C-DISPOSE | dispose 后 getters 空 | 状态/cleanup | 是 | PT5 |
| C-PARA-RANGE | getParagraphRange | 主路径 | 是 | PT6 |
| C-PARA-NONE | papx None → 0 | 边界 | 是 | PT7 |
| C-NULL-DOC | CPieceTable(null) | 失败 | 是 | PT8 |

### 3.3 可测试性

PT1/PT2/PT7 只用 empty。PT3–PT6/PT8 需 Document + transaction 缝。不测 isShape（prop 28 / `\b`）—— STRUCTURE 延后。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|----|--------|----------|----------|-----------------|----------|
| PT1 | C-EMPTY / C-GUESS0 | — | empty() | guessed=0, getTextRope None | |
| PT2 | C-GUESS | empty | set(1234) | get=1234 | |
| PT3 | C-BIND | StorageDoc 有三件套 | CPieceTable(doc) | getters 非 null | |
| PT4 | C-CHARAT | 已 bind，rope 有 "A" | charAt(0)='A' | 自己存了一份字符 |
| PT5 | C-DISPOSE | 已 bind | dispose | rope/chpx/papx None | |
| PT6 | C-PARA-RANGE | papx 段 [0,3) | getParagraphRange(1) | RANGE start=0 end=3 | |
| PT7 | C-PARA-NONE | empty | getParagraphRange(0) | 0 | |
| PT8 | C-NULL-DOC | — | CPieceTable(null) | 空字段或 assert | 与 Java KSAssert 对齐 |

### 3.5 测试代码位置

crate: `libs/word-core/Cargo.toml`  
src:   `libs/word-core/src/cn/wps/moffice/writer/data/CPieceTable.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/data/CPieceTable.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 06_function_cpiecetable

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 06_function_cpiecetable \
  --src src/cn/wps/moffice/writer/data/CPieceTable.rs \
  --report libs/word-core/parser/features/wc-01-storage-write/functions/06_function_cpiecetable/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 06_function_cpiecetable --json`)
date: 2026-08-19 10:40

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/data/CPieceTable.rs` | 57/96 | 59.4% | 82/178 | 46.1% |
| **function total** | **57/96** | **59.4%** |  |  |
<!-- coverage-report:end -->
