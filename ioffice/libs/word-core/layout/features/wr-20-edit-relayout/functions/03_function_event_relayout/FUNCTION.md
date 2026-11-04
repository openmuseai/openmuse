# 03_function_event_relayout — 改后全量排

| 项 | 值 |
|----|----|
| feature | wr-20-edit-relayout |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SessionEdit::relayout`（= `DocumentSession::layout`） |
| status | **pass** |

## 1. 功能说明
### 1.6 不变量与函数改写列表
存储已变后走 **全量** `layout_text_document`。禁止改 Frame 字符串。**改写列表：** `SessionEdit::relayout`。
### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| 01 insert / 02 remove | wr-20 | 改 rope | 本轮 | — | — |
| layout_text_document | wr-18-04 | LayoutFrame | 是 | 增量 Typo | 禁止在 Frame 改字 |

## 2. 核心代码
`SessionEdit::relayout`。Java `FlowLayout.update` 本波不移植。

## 3. 测试方案
### 3.1 Hello + MvpLatin；断言 `DrawText.text`。
### 3.2
| id | 场景 | 必测 | test |
|---|---|---|---|
| EL-INS | insert Z @0 | 是 | EL1 |
| EL-DEL | remove [0,1) | 是 | EL2 |
| EL-IDEM | 两次 layout | 是 | EL3 |

### 3.3 编排 function：允许 open+insert+layout。
### 3.4 EL1 拼接 `DrawText.text` 含 `Z` 与 `Hello`（CHPX 插入可能把 Hello 拆成多 run）。
### 3.5 test: `SessionEventRelayout.rs`
### 3.6
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_event_relayout
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_event_relayout \
  --src src/cn/wps/moffice/writer/api/SessionEdit.rs \
  --report libs/word-core/layout/features/wr-20-edit-relayout/functions/03_function_event_relayout/FUNCTION.md
```
### 3.7
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_event_relayout --json`)
date: 2026-08-27 10:45

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/SessionEdit.rs` | 57/67 | 85.1% | 71/85 | 83.5% |
| **function total** | **57/67** | **85.1%** |  |  |
<!-- coverage-report:end -->
### 3.8
同 01 DocumentEvent 布局校验。
### 3.9
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/core/DocumentEvent.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/core/DocumentEvent.rs \
  --report libs/word-core/layout/features/wr-20-edit-relayout/functions/03_function_event_relayout/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-27 02:44

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `DocumentEvent` | `koffice/writer/core/cn/wps/moffice/writer/core/DocumentEvent.java` | `libs/word-core/src/cn/wps/moffice/writer/core/DocumentEvent.rs` | **PASS** | `increaseRef`, `decreaseRef`, `getOffset`, `getLength`, `setLength`, `getType`, `getSubType`, `getDocument`, `toString` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
