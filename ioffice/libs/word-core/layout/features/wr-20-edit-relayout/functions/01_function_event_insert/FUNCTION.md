# 01_function_event_insert — 任意 cp 插入

| 项 | 值 |
|----|----|
| feature | wr-20-edit-relayout |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SessionEdit::insert(session, cp, text) -> Result<DocumentEvent>` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
`cp ∈ [0, length]`；UTF-8 按 Unicode scalar 计数；空串 length 0 且不改 rope；越界 `Err`。CHPX 在插入点 `IChpxTable::insert`（空 PropertySet）。**改写列表：** `SessionEdit::insert`；`DocumentEvent::{insert_at,getType,getOffset,getLength}`。不改 `DefaultModelEvent` 对象池。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| ITextRope.insert | wr-18/wc | insert__int_string | 是 | UTF-16 代理项 | 禁止空文件重写 rope |
| IChpxTable.insert | wc-02 | 空 CHPX | 是 | 属性分裂与 Java insertText 全等 | 禁止为绿测 stub |
| DocumentEvent | 本 feature | type/offset/length | 本轮 | getDocument 回指 | DefaultModelEvent 池 |

## 2. 核心代码
Rust：`api/SessionEdit.rs`、`core/DocumentEvent.rs`。Java：`DocumentEvent.EVENT_INSERT`；`ITextRope.insert`。

## 3. 测试方案
### 3.1
| symbol | 输入 | 输出 | 分支 |
|---|---|---|---|
| insert | cp=0, "Z" | +1 INSERT | 主路径 |
| insert | cp=len, "AA" | +2 | 末尾 |
| insert | "字" | +1 scalar | CJK |
| insert | "" | 0 | 空 |
| insert | cp>len | Err | 失败 |
| insert | cp<0 | Err | 失败 |

### 3.2 覆盖矩阵
| id | 场景 | 必测 | test |
|---|---|---|---|
| EI-START | 0,"Z" | 是 | EI1 |
| EI-END | len,"AA" | 是 | EI2 |
| EI-CJK | 0,"字" | 是 | EI3 |
| EI-EMPTY | "" | 是 | EI4 |
| EI-PAST | len+1 | 是 | EI5 |
| EI-NEG | -1 | 是 | EI6 |

### 3.3 可测试性
Hello fixture；不 layout。
### 3.4
EI1 type=1 offset=0 length=1；EI5/EI6 Err。
### 3.5
src: `libs/word-core/src/cn/wps/moffice/writer/api/SessionEdit.rs`  
src: `libs/word-core/src/cn/wps/moffice/writer/core/DocumentEvent.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/api/SessionInsert.rs`
### 3.6
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_event_insert
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_event_insert \
  --src src/cn/wps/moffice/writer/api/SessionEdit.rs \
  --src src/cn/wps/moffice/writer/core/DocumentEvent.rs \
  --report libs/word-core/layout/features/wr-20-edit-relayout/functions/01_function_event_insert/FUNCTION.md
```
### 3.7
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_event_insert --json`)
date: 2026-08-27 10:44

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/SessionEdit.rs` | 30/67 | 44.8% | 38/85 | 44.7% |
| `src/cn/wps/moffice/writer/core/DocumentEvent.rs` | 17/44 | 38.6% | 12/34 | 35.3% |
| **function total** | **47/111** | **42.3%** |  |  |
<!-- coverage-report:end -->
### 3.8
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/DocumentEvent.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/DocumentEvent.java
```
### 3.9
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/DocumentEvent.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/DocumentEvent.rs \
  --report libs/word-core/layout/features/wr-20-edit-relayout/functions/01_function_event_insert/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-27 02:44

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `DocumentEvent` | `koffice/writer/core/cn/wps/moffice/writer/core/DocumentEvent.java` | `libs/word-core/src/cn/wps/moffice/writer/core/DocumentEvent.rs` | **PASS** | `increaseRef`, `decreaseRef`, `getOffset`, `getLength`, `setLength`, `getType`, `getSubType`, `getDocument`, `toString` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
