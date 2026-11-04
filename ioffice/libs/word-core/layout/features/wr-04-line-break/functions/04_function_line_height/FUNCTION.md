# 04_function_line_height — 行高/段前段后换算

| 项 | 值 |
|----|----|
| feature | wr-04-line-break |
| **入口** | `line_height(para_props) -> i32`（字号→行格；段前段后来自 PAPX，wc-04 的 para property） |

## 1. 功能说明
行高 = 基线高×行格系数（Java 默认单倍行距；`rPr` 字号 → 行格取整语义为 ceil）；段前（prop 前后距）加到本段首行 top，段后加到段末行底。`TextLayouter` 的行对象 `textRect.height`/`chanel asc/des` 由此写出（wr-01 03 消费）。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| PAPX（段属性：前后距/行距） | wc-01/04 | getInt 191/236 等 | 已实现 |
| 字号 | wr-03-01 | 磅值 | 已实现 |

## 3. 测试方案
### 3.2/3.4
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| H1 | HEIGHT-SINGLE | 字号 12 单倍 | 行高=行格值（黄金对照 Java） | 行格错 |
| H2 | HEIGHT-ROUND | 奇数半磅 | ceil（Java 取整语义） | 取整错 |
| H3 | HEIGHT-BEFORE | 段前 240 缇 | 首行 top 前移 | 段前错 |
| H4 | HEIGHT-AFTER | 段后 240 | 末行底扩展 | 段后错 |
| H5 | HEIGHT-DEFAULT | 无 pPr | 默认行格 | 缺省错 |

### 3.3 可测试性
纯计算；黄金对照来自 Java 行高语义（记录参考值）。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/text/LineHeight.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/text/LineHeight.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_line_height
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 04_function_line_height \
  --src src/cn/wps/moffice/writer/layout/base/text/LineHeight.rs \
  --report libs/word-core/layout/features/wr-04-line-break/functions/04_function_line_height/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_line_height --json`)
date: 2026-08-22 09:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/text/LineHeight.rs` | 20/21 | 95.2% | 18/19 | 94.7% |
| **function total** | **20/21** | **95.2%** |  |  |
<!-- coverage-report:end -->
布局校验：loop 以 Java 行高语义源码为对标（§3.8 命令补）。