# 02_function_session_append — 末尾插入段

| 项 | 值 |
|----|----|
| feature | wr-18-session-caret |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `DocumentSession::append_paragraph(&str)` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
在主故事 **末尾** 追加 `text + '\r'`，空 CHPX/PAPX；只改存储，不排版。

### 1.2 输入
已 open 的 session + UTF-8 `&str`（按 Unicode scalar 计数，与 `CoreHelper.appendText` 一致）。

### 1.3 输出
`main_length` 增加 `text.chars().count() + 1`。空串仍 `+1`（仅 `\r`）。

### 1.4 上游
01 session。Java：`CoreHelper.appendText(String)` + `insertToPapxTable`。

### 1.5 下游
04 relayout。

### 1.6 不变量与函数改写列表
1. 不调用 `layout()`。
2. 不在任意 cp 插入（仅 append）。
3. **改写列表：** `DocumentSession::append_paragraph`。`CoreHelper` 方法已在 wc-01 `07_function_corehelper_write`；本 function **禁止**从零重写 `CoreHelper.rs`。若要补语义，走 **function-java2rust** + `park-deferred.mjs`。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `CoreHelper::appendText__string_ps` | wc-01 | rope+CHPX insert | 是 | char[] 重载本 function 不测 | 禁止空文件重写 |
| `CoreHelper::insertToPapxTable` | wc-01 | PAPX at start cp | 是 | 返回 Entry 本 function 不断言 | 同上 |
| `PropertySet::empty` | util | 空 CHPX/PAPX | 是 | — | — |
| `getMainDocumentMut` | wc-02 | 可变主故事 | 是 | 无 main → Err | 禁止为绿测造 stub |

### 1.8 函数改写列表
- `DocumentSession::append_paragraph` 仅此。

## 2. 核心代码

```text
start = main.getLength()
s = text + '\r'
CoreHelper.appendText(main, s, empty)
CoreHelper.insertToPapxTable(main, start, empty)
```

Java：`koffice/.../helper/CoreHelper.java` `appendText` / `insertToPapxTable`。

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| append_paragraph | DocumentSession.rs | `"AAA"` | +4 | 主路径 |
| append_paragraph | `""` | +1 | 空串仍有 `\r` |
| append_paragraph | `"字"` | +2 | CJK scalar |
| append_paragraph | 两次 | 累加 | 状态 |
| append_paragraph | 无 layout | 只 length | 不排版 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| SA-LATIN | append | AAA | 是 | SA1 |
| SA-EMPTY | append | 空串 | 是 | SA2 |
| SA-CJK | append | 一字 | 是 | SA3 |
| SA-TWICE | append | 两次 | 是 | SA4 |
| SA-NOLAYOUT | append | 不 layout | 是 | SA5 |
| SA-NOMAIN | append | 无主故事 | 否 | 产品 open 总有 main；无法从公开 API 构造 |

### 3.3 可测试性
01 的 Hello session；不断言 PAPX 内部 id；不跑 wr-12。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| SA1 | SA-LATIN | Hello | AAA | +4 | 按 UTF-8 字节 +3 |
| SA2 | SA-EMPTY | Hello | `""` | +1 | panic empty CoreHelper |
| SA3 | SA-CJK | Hello | 字 | +2 | +4 字节 |
| SA4 | SA-TWICE | Hello | A 然后 B | +4 | 第二次覆盖 |
| SA5 | SA-NOLAYOUT | Hello | Z | Ok 且 length>0 | 误调 layout panic |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/api/DocumentSession.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/api/SessionAppend.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_session_append

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_session_append \
  --src src/cn/wps/moffice/writer/api/DocumentSession.rs \
  --report libs/word-core/layout/features/wr-18-session-caret/functions/02_function_session_append/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_session_append --json`)
date: 2026-08-27 10:22

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/DocumentSession.rs` | 22/36 | 61.1% | 35/50 | 70.0% |
| **function total** | **22/36** | **61.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/helper/CoreHelper.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/helper/CoreHelper.java
```
改 `CoreHelper` 必须走 pipeline，不得删 `// java`。本轮布局：**PASS**（非改写列表方法 `todo!("DEFERRED_NON_FUNCTION")` 停车；未 `--force` 覆盖 `appendText`）。

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/helper/CoreHelper.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/helper/CoreHelper.rs \
  --report libs/word-core/layout/features/wr-18-session-caret/functions/02_function_session_append/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-27 02:24

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `CoreHelper` | `koffice/writer/core/cn/wps/moffice/writer/helper/CoreHelper.java` | `libs/word-core/src/cn/wps/moffice/writer/helper/CoreHelper.rs` | **PASS** | `setCellMark`, `appendParagraph`, `insertToPapxTable`, `appendText`, `supplyEndPara`, `isValidChar` | `getClsid`, `pickVersion`, `isVersionMcdPrefix`, `isCmtRefMcdPrefix`, `isShapeMcdPrefix`, `isChart`, `isInlineChart`, `isChartShape0x08`, `isChartShape`, `dumpPicture4Chart`, `getChartRectF`, `convert2vmlPertcent`, `vmlRotation2DmlRot`, `dmlRot2VmlRotation`, `dumpAllChartShapePictures`, `dumpAllDiagram`, `collectAllOLE`, `getNewGeometry` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
