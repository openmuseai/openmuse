# 01_function_page_break — 页满判定与断页

| 项 | 值 |
|----|----|
| feature | wr-05-page-flow |
| **入口** | `flow_lines_into_pages(lines, content_height) -> Vec<PageFill>`（Java `TextLayouter.processPagination` + `MainLayouter.flowPage` 语义） |

## 1. 功能说明
逐行累积已用高：超过 contentHeight → 断页（末行标 `LASTLINEOFPAGE`，wr-01 03 标志）；遇到 `\f`（分页符，wc-04 已落为 rope 的 0x0C）先行断页；页中断行不拆分（表格/图形延后）。输出每页的终止行索引 + 已用高。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| 行流+T ypo 行高 | wr-04 | 行序列/高度 | wr-04 先行 |
| 分页符 `\f` | wc-04/rope | 字符判定 | 已实现 |
| contentHeight | wr-02 | 页内容高 | wr-02 先行 |

## 3. 测试方案
### 3.2/3.4
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| B1 | PAGE-FILL | 3 行高 100，高 250 | 页1=2 行，页2=1 行 | 断页错 |
| B2 | PAGE-EXACT | 高恰 == 行高和 | 单页 | 边界错 |
| B3 | PAGE-OVERFLOW | 单行超页高 | 单行成页（允许溢出） | 死循环 |
| B4 | PAGE-FORMFEED | 行中含 \f | \f 处断页 | 分页符错 |
| B5 | PAGE-LASTFLAG | 断页行 | 标 LASTLINEOFPAGE | 标志错 |
| B6 | PAGE-EMPTY | 空文档 | 0 页（或按契约 1 空页） | 空错 |

### 3.3 可测试性
构造行序列（高度 stub）+ content_height；不依赖文档/渲染。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/PageBreak.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/PageBreak.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_page_break
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_page_break \
  --src src/cn/wps/moffice/writer/layout/base/PageBreak.rs \
  --report libs/word-core/layout/features/wr-05-page-flow/functions/01_function_page_break/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_page_break --json`)
date: 2026-08-22 09:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/PageBreak.rs` | 40/40 | 100.0% | 44/45 | 97.8% |
| **function total** | **40/40** | **100.0%** |  |  |
<!-- coverage-report:end -->
布局校验：loop 以 Java `TextLayouter.processPagination`/`MainLayouter` 为对标（§3.8 命令补）。