# 03_function_relayout_cache — 重排范围（MVP 全量占位）

| 项 | 值 |
|----|----|
| feature | wr-05-page-flow |
| **入口** | `relayout_range(edit: (cp, remove, insert)) -> Range`（Java `FlowLayout.update(DocumentEvent)` + `TextLayouter.mStateSnapshot` 语义；MVP 全量） |

## 1. 功能说明
编辑事件 → 需重排区间。MVP 实现为「整文档全量」（返回 [0, docLen)），接口保留增量形态；增量状态（StateSnapshot：可复用页/行缓存）延后。`update` 触发 `updatePageRanges` 平移（wr-01 04）。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| TypoRangeBase.shift | wr-01-04 | 平移 | wr-01 先行 |
| DocumentEvent | wc-02 | cp/rm/ins | 已实现 |

## 3. 测试方案
### 3.2/3.4
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| R1 | RELAY-FULL | 任意编辑 | 区间=整文档 | 全量错 |
| R2 | RELAY-EMPTY | 空编辑(0,0,0) | 无重排（优化点） | 误触发 |
| R3 | RELAY-INS | (10,0,5) | 平移语义冒烟 | 平移错 |

### 3.3 可测试性
纯占位接口；真实重排 wr-06 承接。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/RelayoutCache.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/RelayoutCache.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_relayout_cache
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_relayout_cache \
  --src src/cn/wps/moffice/writer/layout/base/RelayoutCache.rs \
  --report libs/word-core/layout/features/wr-05-page-flow/functions/03_function_relayout_cache/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_relayout_cache --json`)
date: 2026-08-22 09:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/RelayoutCache.rs` | 13/13 | 100.0% | 13/13 | 100.0% |
| **function total** | **13/13** | **100.0%** |  |  |
<!-- coverage-report:end -->
布局校验：loop 以 Java `FlowLayout.update`/`StateSnapshot` 为对标（§3.8 命令补）。