# 01_function_text_command — run→DrawText（字体/字号/色/坐标）

| 项 | 值 |
|----|----|
| feature | wr-07-render-command |
| **入口** | `emit_text(run, page, line) -> DrawText`（Java `LayoutRender` 文本绘制分支） |

## 1. 功能说明
把 TypoRun（wr-01 04）+ 行 x/y 偏移（wr-04 对齐/行矩形）投影为 `DrawText{page,x,y,text,font_id,size,color,style}`。坐标 = 页原点 + 行 top + 对齐 x + run 内偏移；文本取 rope 子串（cp 区间映射，wc-02）。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| TypoRun/TypoTextLine | wr-01 | run/行数据 | wr-01 先行 |
| align x | wr-04-03 | 行内偏移 | wr-04 先行 |
| rope 文本 | wc-02 | cp→文本 | 已实现 |
| 样式色 | wc-03 PropertySet | color | 已实现 |

## 3. 测试方案
### 3.2/3.4
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| T1 | TX-SIMPLE | 单 run 左对齐 | x/y/文本正确 | 投影错 |
| T2 | TX-ALIGN | center 行 | x 含对齐偏移 | 偏移错 |
| T3 | TX-MULTI-RUN | 行内 2 run | 第二 run x=前 run 宽之和 | 偏移错 |
| T4 | TX-STYLE | 粗体/色 | style/color 入指令 | 样式丢 |
| T5 | TX-PAGE | 页2 | page 编号正确 | 页错 |

### 3.3 可测试性
构造 run/行/页 stub → emit；断言指令字段（serde 快照对比）。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/rendercommand/TextCommand.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/rendercommand/TextCommand.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_text_command
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_text_command \
  --src src/cn/wps/moffice/writer/rendercommand/TextCommand.rs \
  --report libs/word-core/layout/features/wr-07-render-command/functions/01_function_text_command/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_text_command --json`)
date: 2026-08-22 10:00

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/rendercommand/TextCommand.rs` | 16/16 | 100.0% | 16/16 | 100.0% |
| **function total** | **16/16** | **100.0%** |  |  |
<!-- coverage-report:end -->
布局校验：loop 以 Java `LayoutRender` 文本分支为对标（§3.8 命令补）。