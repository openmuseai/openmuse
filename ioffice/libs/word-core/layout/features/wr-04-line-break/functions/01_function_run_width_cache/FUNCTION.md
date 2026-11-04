# 01_function_run_width_cache — run 度量缓存（CP→宽）

| 项 | 值 |
|----|----|
| feature | wr-04-line-break |
| **入口** | `get_run_width(cp, text, fontId, size) -> i32`（缓存 miss 时调 wr-03 度量） |
| 独立性 | 依赖 wr-03 |

## 1. 功能说明
断行的性能层：以 (cp,fontId,size) 为键缓存 run 宽度；编辑后缓存失效（cp 平移/区间重建，复用 wr-01 04 的 shift 规则）。Java 侧缓存位于 LayoutState/PropertyToolFactory 体系。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| RunMetrics | wr-03-02 | run 总宽/单字宽 | wr-03 先行 |
| TypoRangeBase.shift 语义 | wr-01-04 | cp 平移 | 复用 |

## 3. 测试方案
### 3.2/3.4
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| C1 | CACHE-MISS-FILL | 首次 query | 度量一次并填入 | 未缓存 |
| C2 | CACHE-HIT | 同键二次 | 不再度量（计数） | 重复度量 |
| C3 | CACHE-KEY | 同文本不同字号 | 独立条目 | 键错 |
| C4 | CACHE-SHIFT | 插入后 cp 平移 | 命中受影响条目失效 | 失效错 |
| C5 | CACHE-EMPTY | 空文本 | 宽 0 且缓存 | 空错 |

### 3.3 可测试性
计数器 stub 度量注入（`Fn` 闭包）驱动 miss/hit。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/text/RunWidthCache.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/text/RunWidthCache.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_run_width_cache
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_run_width_cache \
  --src src/cn/wps/moffice/writer/layout/base/text/RunWidthCache.rs \
  --report libs/word-core/layout/features/wr-04-line-break/functions/01_function_run_width_cache/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_run_width_cache --json`)
date: 2026-08-22 09:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/text/RunWidthCache.rs` | 36/37 | 97.3% | 59/64 | 92.2% |
| **function total** | **36/37** | **97.3%** |  |  |
<!-- coverage-report:end -->
布局校验（loop 时按 Java `LayoutState`/`PropertyToolFactory` 对标补 §3.8 命令）。