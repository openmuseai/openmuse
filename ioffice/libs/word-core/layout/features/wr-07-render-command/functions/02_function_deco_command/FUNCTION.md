# 02_function_deco_command — 装饰/背景 → DrawRect/DrawLine

| 项 | 值 |
|----|----|
| feature | wr-07-render-command |
| **入口** | `emit_deco(run_style) -> Vec<Deco命令>`（Java `LayoutRender` 装饰分支 + wpslayout `DecoLineCache` 语义） |

## 1. 功能说明
run 样式的下划线（单/双/波浪/删除线/重点）、背景色（shd）投影为指令：下划线=`DrawLine`（沿基线、按 DecoLineCache 的线型语义）、背景=`DrawRect`（run 矩形）。wpslayout `DecoLineCache`（`drawLine/drawDoubleline/drawWaveline/drawDotline/drawDashline` 等）已具 Rust 实现——指令化其**几何**（不画 Canvas）。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| DecoLineCache | wpslayout（text/layout/data） | 线型几何 | Rust 有实现（审计评级待 wr-03-03） |
| 下划线/背景样式 | wc-03 property | color/线型 | 已实现 |
| run 矩形 | wr-01-04 | rect | 已实现 |

## 3. 测试方案
### 3.2/3.4
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| D1 | DECO-UNDERLINE | 单线 | 1 条 DrawLine（基线起止） | 线错 |
| D2 | DECO-DOUBLE | 双线 | 2 条 line | 双线错 |
| D3 | DECO-WAVE | 波浪 | wave line 指令 | 类型错 |
| D4 | DECO-BG | 背景色 | 1 条 DrawRect（run 矩形） | 背景错 |
| D5 | DECO-NONE | 无样式 | 0 指令 | 误发 |

### 3.3 可测试性
run 样式 stub → emit；断言指令集。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/rendercommand/DecoCommand.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/rendercommand/DecoCommand.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_deco_command
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_deco_command \
  --src src/cn/wps/moffice/writer/rendercommand/DecoCommand.rs \
  --report libs/word-core/layout/features/wr-07-render-command/functions/02_function_deco_command/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_deco_command --json`)
date: 2026-08-22 10:00

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/rendercommand/DecoCommand.rs` | 49/51 | 96.1% | 65/75 | 86.7% |
| **function total** | **49/51** | **96.1%** |  |  |
<!-- coverage-report:end -->
布局校验：loop 以 Java `DecoLineCache`/`LayoutRender` 装饰分支为对标（§3.8 命令补）。