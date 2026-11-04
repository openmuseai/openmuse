# 03_function_alignment — 对齐（左/中/右）

| 项 | 值 |
|----|----|
| feature | wr-04-line-break |
| **入口** | `align_x(jc, line_width, content_width) -> i32`（wc-03 已转 jc 数值；`left=0/center=1/right=2/justify=3` 域，以 Java PropertySet 3385 语义为基准） |

## 1. 功能说明
行内起始 x：左对齐 x=0；居中 x=(contentW−lineW)/2；右对齐 x=contentW−lineW；两端对齐（justify）MVP 延后（DEFERRED 标注）。非首行缩进等 pPr（ind 字段）延后。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| jc 来源 | wc-03 | PropertySet 取 jc | 已实现 |
| line_width | wrap | 行宽 | 02 先行 |

## 3. 测试方案
### 3.2/3.4
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| A1 | ALIGN-LEFT | jc=left, line=100, content=200 | x=0 | 偏移错 |
| A2 | ALIGN-CENTER | center | x=(200−100)/2=50 | 居中错 |
| A3 | ALIGN-RIGHT | right | x=100 | 右错 |
| A4 | ALIGN-FULL | line==content | 任何对齐 x=0 | 边界 |
| A5 | ALIGN-JUSTIFY | justify | 延后：返回 left 语义 + 记录 | 误实现 |

### 3.3 可测试性
纯函数。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/text/Alignment.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/text/Alignment.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_alignment
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_alignment \
  --src src/cn/wps/moffice/writer/layout/base/text/Alignment.rs \
  --report libs/word-core/layout/features/wr-04-line-break/functions/03_function_alignment/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_alignment --json`)
date: 2026-08-22 09:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/text/Alignment.rs` | 9/9 | 100.0% | 9/9 | 100.0% |
| **function total** | **9/9** | **100.0%** |  |  |
<!-- coverage-report:end -->
布局校验：wiJava jc 转换（wc-03 05/06）为对标，§3.8 命令 loop 补。