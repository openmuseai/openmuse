# 02_function_wrap — 行内换行（0/1/N run + 超宽）

| 项 | 值 |
|----|----|
| feature | wr-04-line-break |
| **入口** | `wrap(paragraph_runs, contentWidth) -> Vec<LineBreak>`（Java `TextLayouter.doFlowTextLine` 的断行部分；`\r`=段末） |

## 1. 功能说明
把段落 run 流按内容宽折成行序列：每行可跨多个 run（在 run 边界折行）；行宽超 contentWidth 时回退到最近自然断点；单词超宽时强制截断（Java 语义）。输出每行的 run 子区间 + 行宽。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| RunWidthCache | 01（本 feature） | run 宽 | 01 先行 |
| 段末 `\r` | wc-01 rope | 字符判定 | 已实现 |

## 3. 测试方案
### 3.2/3.4
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| W1 | WRAP-SINGLE | 单 run 短于宽 | 1 行 | 拆行错 |
| W2 | WRAP-MULTI | 两 run 超宽 | 折 2 行；边界在 run 间 | 折点错 |
| W3 | WRAP-SPACE | 行内两词超宽 | 在词间折 | 断点错 |
| W4 | WRAP-LONGWORD | 单词超宽 | 强制截断（不无限循环） | 死循环/溢出 |
| W5 | WRAP-EMPTY | 空段 | 0 行（或 1 空行，按 Java） | 空错 |
| W6 | WRAP-CR | 含 `\r` | 段末断行 || 段标错 |

### 3.3 可测试性
构造 run 序列 + stub 宽度表；无文档依赖。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/text/Wrap.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/text/Wrap.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_wrap
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_wrap \
  --src src/cn/wps/moffice/writer/layout/base/text/Wrap.rs \
  --report libs/word-core/layout/features/wr-04-line-break/functions/02_function_wrap/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_wrap --json`)
date: 2026-08-22 09:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/text/Wrap.rs` | 142/153 | 92.8% | 218/240 | 90.8% |
| **function total** | **142/153** | **92.8%** |  |  |
<!-- coverage-report:end -->
布局校验：loop 时以 Java `TextLayouter.doFlowTextLine` 为对标（§3.8 命令按实际 .rs/.java 补）。