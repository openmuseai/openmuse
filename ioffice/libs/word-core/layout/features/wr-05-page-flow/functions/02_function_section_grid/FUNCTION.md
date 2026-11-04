# 02_function_section_grid — 节页网格（尺寸切换 + cp→页）

| 项 | 值 |
|----|----|
| feature | wr-05-page-flow |
| **入口** | `page_grid(PLCSection 节序列) -> Grid` + `page_index_from_cp(cp)`（Java `IPageGrid.pageIndexFromChar` 语义；MVP 页对象列表二分） |

## 1. 功能说明
节序列（PLCSection：每节起止 cp + PageSetup）→ 每页尺寸表；`page_index_from_cp` 用页 cp 区间（wr-01 02）二分定位。多节切换页尺寸（如不同页宽 → 折行重来，MVP 记标志）。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| PLCSection | wc-01 | 节节点/cp/get(0) | 已实现 |
| PageSetup | wr-02-01 | content 尺寸 | wr-02 先行 |
| TypoPage 区间 | wr-01-02 | mStartCP/mEndCP | wr-01 先行 |

## 3. 测试方案
### 3.2/3.4
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| G1 | GRID-SINGLE | 单节默认 | 单尺寸 | 网格错 |
| G2 | GRID-MULTI | 两节不同尺寸 | 页按节切换尺寸 | 切换错 |
| G3 | GRID-CPINDEX | 页区间 0-100/100-200 | cp=150 → 页1 | 二分错 |
| G4 | GRID-BOUND | cp==页边界 | 归前一页（Java 语义核对） | 边界错 |
| G5 | GRID-EMPTY | 无节 | 默认节兜底（wc-05 06） | 空错 |

### 3.3 可测试性
构造节序列 + 页区间表；纯数据。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/PageGrid.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/PageGrid.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_section_grid
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_section_grid \
  --src src/cn/wps/moffice/writer/layout/base/PageGrid.rs \
  --report libs/word-core/layout/features/wr-05-page-flow/functions/02_function_section_grid/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_section_grid --json`)
date: 2026-08-22 09:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/PageGrid.rs` | 38/39 | 97.4% | 40/43 | 93.0% |
| **function total** | **38/39** | **97.4%** |  |  |
<!-- coverage-report:end -->
布局校验：loop 以 Java `IPageGrid` 为对标（§3.8 命令补）。