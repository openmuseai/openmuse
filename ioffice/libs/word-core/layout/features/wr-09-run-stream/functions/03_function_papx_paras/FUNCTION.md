# 03_function_papx_paras — PAPX 切段

| 项 | 值 |
|----|----|
| feature | wr-09-run-stream |
| order | 3 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `PapxParas::walkPapx(table, start, end) -> Vec<PapxSpan>` |
| 独立性 | 只读 `IPapxTable.seek`；不取文本、不碰 CHPX、不解析 pStyle 字号 |

## 1. 功能说明

### 1.1 一句话
按 PAPX 游标切出段落半开区间；pStyle **只作为 PropertySet 引用传递**。

### 1.2 输入
`&dyn IPapxTable` + `[start, end)`。

### 1.3 输出
`PapxSpan { start_cp, end_cp, property }`。段末 `\r`/`\f` 落在区间内（字符由 01 读）。

### 1.6 不变量
同 02：clip、空窗、跳过 Head、不 merge KStyles。

### 1.7 依赖

| 依赖 | 需要 | 已实现 | 禁止改 |
|------|------|--------|--------|
| `IPapxTable.seek` + Entry 游标 | 是 | 是 | PapxTable.rs |
| `CoreHelper.appendParagraph` | 测试灌段 | 是 | — |

### 1.8 函数改写列表
- `walkPapx`（Java `IPapxTable.seek` + `getNext`）

## 2. 流程
与 02 同构，表换成 PAPX。

## 3. 测试方案

### 3.2 矩阵
| coverage_id | test_id |
|-------------|---------|
| PX-EMPTY | PX1 |
| PX-ONE | PX2 |
| PX-TWO | PX3 |
| PX-CLIP | PX4 |
| PX-WINDOW-EMPTY | PX5 |
| PX-COVER | PX6 |

### 3.4 用例
| id | 摘要 | 期望 |
|----|------|------|
| PX1 | 无段 | [] |
| PX2 | 一段 `"Hi\r"` | 一条覆盖段 |
| PX3 | 两段不同 papx | 两条，property 可区分 |
| PX4 | 窗口落在第一段中后部 | clip start |
| PX5 | start==end | [] |
| PX6 | 全长 | 并集覆盖窗口∩表 |

### 3.5 位置
src: `PapxParas.rs` · test: `tests/.../PapxParas.rs`

### 3.6 脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_papx_paras

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_papx_paras \
  --src src/cn/wps/moffice/writer/layout/base/text/PapxParas.rs \
  --report libs/word-core/layout/features/wr-09-run-stream/functions/03_function_papx_paras/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_papx_paras --json`)
date: 2026-08-23 15:31

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/text/PapxParas.rs` | 25/29 | 86.2% | 45/50 | 90.0% |
| **function total** | **25/29** | **86.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 布局
```bash
node .codex/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/text/PapxParas.rs
```
