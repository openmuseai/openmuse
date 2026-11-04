# 02_function_event_remove — 半开删除

| 项 | 值 |
|----|----|
| feature | wr-20-edit-relayout |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SessionEdit::remove(session, start, end) -> Result<DocumentEvent>` |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
半开 `[start,end)`；`start>=end` → length 0 不改 rope；越界 `Err`。只 `ITextRope.erase`（本 function 不走 `KRange.deleteRange`）。**改写列表：** `SessionEdit::remove`。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| ITextRope.erase | wc | start,end exclusive | 是 | CHPX/PAPX 收缩与 Java KTextStream 全等 | 禁止空文件重写 |
| DocumentEvent | 01 | EVENT_REMOVE | 是 | — | DefaultModelEvent 池 |

## 2. 核心代码
`SessionEdit::remove`；Java `ITextRope.erase` / `EVENT_REMOVE=2`。

## 3. 测试方案
### 3.1
erase 首 cp；erase 两 cp；空区间；逆序当空；end>len Err；负 start Err。
### 3.2
| id | 场景 | 必测 | test |
|---|---|---|---|
| ER-ONE | [0,1) | 是 | ER1 |
| ER-TWO | [0,2) | 是 | ER2 |
| ER-EMPTY | [1,1) | 是 | ER3 |
| ER-INV | 3,1 | 是 | ER4 |
| ER-PAST | end=len+1 | 是 | ER5 |
| ER-NEG | start=-1 | 是 | ER6 |

### 3.3 不 layout。
### 3.4 ER1 length-1 type=2。
### 3.5
src: `SessionEdit.rs`、`DocumentEvent.rs`  
test: `tests/.../SessionRemove.rs`
### 3.6
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_event_remove
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_event_remove \
  --src src/cn/wps/moffice/writer/api/SessionEdit.rs \
  --src src/cn/wps/moffice/writer/core/DocumentEvent.rs \
  --report libs/word-core/layout/features/wr-20-edit-relayout/functions/02_function_event_remove/FUNCTION.md
```
### 3.7
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_event_remove --json`)
date: 2026-08-27 10:44

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/SessionEdit.rs` | 24/67 | 35.8% | 31/85 | 36.5% |
| `src/cn/wps/moffice/writer/core/DocumentEvent.rs` | 17/44 | 38.6% | 12/34 | 35.3% |
| **function total** | **41/111** | **36.9%** |  |  |
<!-- coverage-report:end -->
### 3.8
同 01：`DocumentEvent.rs` / `DocumentEvent.java`
### 3.9
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/DocumentEvent.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/DocumentEvent.rs \
  --report libs/word-core/layout/features/wr-20-edit-relayout/functions/02_function_event_remove/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-27 02:44

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `DocumentEvent` | `koffice/writer/core/cn/wps/moffice/writer/core/DocumentEvent.java` | `libs/word-core/src/cn/wps/moffice/writer/core/DocumentEvent.rs` | **PASS** | `increaseRef`, `decreaseRef`, `getOffset`, `getLength`, `setLength`, `getType`, `getSubType`, `getDocument`, `toString` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
