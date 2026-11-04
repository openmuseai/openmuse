# 06_function_txbody_data_textbody — txBody → `data.text.TextBody`

> feature: pp-19-kmo-tree-write ｜ 状态：**pass** ｜ Java 管线：S4 步骤 4.6

## 0. 管线能力解锁

实现后：段/run 落在 `drawing.data.text.TextBody.paraLst`，而不是 `TextBodySurf`。pl-04/pr-03 才有 Java 同构输入。

## 1. 功能说明

### 1.1 I/O 契约

Java `TextBodyHandler.endElement:51-58`：`paraList` 非空则每段 `save()`，然后 `textBody.paraLst().forceWrite(this.paraList)`。  
`enterSubElement`：bodyPr=1114448；lstStyle=1114449；p=1114562 则 `TextParagraph.create()` 进 list，复用 `ParagraphHandler.init`。

空 txBody（无 p）：`paraList==null` → 不 forceWrite。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `data/text/TextBody.rs` | drawing | paraLst.forceWrite | ✅ 文件在 | listStyle 全量 | 禁止改 KoBin 布局 |
| `TextBodyHandler` | dmlr | 收集段 | ⚠️ TextBodySurf | — | 禁止 surf 冒充 pass |
| `ParagraphHandler` | pp-11 | r/br/fld | 测试面 | fld 全语义 | — |
| `KmoTextBody` | drawing/app/text | 包装 TextBody | ⚠️ 长文件 + parent `cfg(any())` | 本波可只写 data TextBody | 禁止 stubs |

## 2. 核心代码与函数改写列表

- **本 function：** `TextBodyHandler` 持有 `data::text::TextBody`；`endElement` save+forceWrite；`enterSubElement` 1114562 收集。
- **停车：** lstStyle 细表；KmoTextBody 全量 accessor。
- **禁改：** RegularTextHandler 字符语义（pp-11 已锁的纯文本）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`TextBodyHandler.java:18-59`；`text_body_surf.rs` `TextBodySurf.paras`；`TextBody.rs` 有 paraLst 字段。

### 3.2 覆盖矩阵

| coverage_id | 场景 | 必测? | test_id |
|-------------|------|-------|---------|
| TX-0 | 无 p | 是 | TX1 ignore spec：不 forceWrite |
| TX-1 | 一段一字 | 是 | TX2 ignore spec |
| TX-N | 两段 | 是 | TX3 ignore spec |
| TX-SURF | 今日 surf 仍可解析（负例/过渡） | 是 | TX0 |

### 3.4 用例表

| id | 期望 | 失败语义 |
|----|------|----------|
| TX0 | TextBodySurf 一段仍可读（pp-11 回归） | 误拆旧路径 |
| TX1–TX3 | ignore：data TextBody paraLst | 仍只写 surf |

### 3.5 测试代码位置

test: `tests/cn/wps/drawing/dmlr/handler/shape/textshape/text/06_function_txbody_data_textbody.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 06_function_txbody_data_textbody -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 06_function_txbody_data_textbody \
  --src src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/06_function_txbody_data_textbody/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 06_function_txbody_data_textbody --json`)
date: 2026-09-12 16:15

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/data/text/TextBody.rs` | 64/500 | 12.8% | 113/846 | 13.4% |
| `src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.rs` | 42/76 | 55.3% | 67/104 | 64.4% |
| **function total** | **106/576** | **18.4%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/06_function_txbody_data_textbody/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-12 08:16

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TextBodyHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.rs` | **PASS** | `TextBodyHandler`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `TextBody` | `koffice/presentation/core/cn/wps/drawing/data/text/TextBody.java` | `libs/ppt-core/src/cn/wps/drawing/data/text/TextBody.rs` | **PASS** | `create`, `initialize`, `open`, `TextBody`, `loadBin`, `load`, `saveBin`, `equalsLax`, `equals`, `hashCode`, `dirty`, `save`, `lastBin`, `discard`, `close`, `cleanFollow`, `cleanCache`, `follow`, `unFollow`, `hasParent`, `parent`, `cleanParent`, `listStyle`, `hasListStyle`, `deleteListStyle`, `writeListStyle`, `forceWriteListStyle`, `bodyPr`, `hasBodyPr`, `deleteBodyPr`, `writeBodyPr`, `forceWriteBodyPr`, `paraLst`, `hasParaLst`, `deleteParaLst`, `writeParaLst`, `forceWriteParaLst` | — | — | **PASS** |
| `ParagraphList` | `koffice/presentation/core/cn/wps/drawing/data/text/TextBody.java` | `libs/ppt-core/src/cn/wps/drawing/data/text/TextBody.rs` | **PASS** | `ParagraphList`, `loadBin`, `load`, `saveBin`, `equalsLax`, `equals`, `hashCode`, `dirty`, `save`, `discard`, `lastBin`, `close`, `create`, `initialize`, `open`, `read`, `write`, `forceWrite` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
