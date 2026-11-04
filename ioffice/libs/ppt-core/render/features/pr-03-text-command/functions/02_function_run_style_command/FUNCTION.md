# 02_function_run_style_command — run 样式附着 DrawText

> feature: pr-03-text-command ｜ 状态：**pass** ｜ Java 管线环节：S5（`ShowRichTextLayout.renderText` 的 run/Span 样式语义）

## 0. 管线能力解锁

实现后：01 的 `DrawText` 带上**字体/字号/颜色/字重/斜体**——painter 可直接按样式画字，文本不再「全一样」。没有它，管线断在 S5 样式环节：文本行能出但无字体信息，出屏即默认字体（主文档 pr-03 `text_command` 行：`DrawText（字体/字号/颜色）`）。

## 1. 功能说明

`行定位 + SpanInfo 样式 → Vec<DrawText{...样式字段}>`。Java 语义源是 `IShowTextLayout.renderText(canvas)`（`ShowRichTextLayout.renderText` 逐 run/Span 绘制：字体/字号/颜色/下划线…）；Rust 侧把 run 样式译成指令字段，逐 run 一个 DrawText。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | pl-04/pl-05 的 `TextBoxData→TextData→SpanInfo`（每 span：`mFirstSpan` 或 run 级 `mFontName`/`mFontSize`/色/bold/italic）+ 行定位（01 的 x/y）。 |
| **输出** | 带样式 `DrawText{font, size, color, bold, italic, ...}`：字段 = `SpanInfo` 的 fontName/fontSize/color/bold/italic；无 run 级别时用段首 span（Java `mFirstSpan`，如 `drawBullets` 用 `getTextData(i).mFirstSpan.mFontSize`）。 |
| **状态与副作用** | Java 每 span 一个绘制句 + 指示符（下划线/删除线跑 flg）；MVP 只落字体四元组（font/size/color/bold/italic），下划线/删除线/字距延后。颜色解析依赖 pp-03/pp-04 scheme 色（未接线时用 fixture 直给 RGB）。 |
| **错误语义** | 无异常路径；spans 为空 → 维持 01 已有 DrawText（默认样式）或空序列由 01 定，待 loop 确认。 |

### 1.2-1.6 已并入 1.1（样式映射纯函数）

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| SpanInfo 样式 | pl-04/pl-05 | fontName/fontSize/color/bold/italic | spec | 预置样式表 | 不改排版模型 |
| 颜色源 | pp-03/pp-04 | scheme 色 → RGB | spec | fixture 直给 | 不复制色板 |
| DrawText 字段 | 01/pr-04 | 样式字段与 wr-07 对齐 | spec | loop 对齐 | 不 rename 既有字段 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `TextRender.render`（样式传导）+ `ShowRichTextLayout.renderText`（run/Span 逐段绘制，wpslayout）+ `ShapeTextRender._createTextInfo`（TimingInfo 参照，延后） |
| Rust | `libs/ppt-core/src/cn/wps/show/render/TextRender.rs`（02 的样式分支，新建） |

关键方法（对照 Java，均已查证）：

| symbol | 语义（Java） | Rust 目标 |
|---|---|---|
| `TextRender.drawText(Canvas, IShowTextLayout, RectF, ...)` | `textLayout.renderText(canvas)` / `renderText(canvas, firstPara, lastPara+1, startOffset, endOffset, timInfo)` | 无选区 → 逐 run 输出；有选区超载停车 |
| `IShowTextLayout.renderText` | `ShowRichTextLayout.renderText` 逐 span 绘制（样式取 span） | span → DrawText 字段 |
| `TextData.mFirstSpan` | 段首 span（fontSize 等取法） | 默认样式源 |
| `ShapeTextRender._createTextInfo` | TimingInfo（字体色/下划线/bold/italic/字号 scale 按 dirty 位） | 延后；dirty 位语义待 loop |
| 样式读取 | `SpanInfo`/`mFontName`/`mFontSize`/`getColor` 等（wpslayout） | 字段映射 |

改写列表：run/Span → DrawText 样式字段的映射；`mFirstSpan` 默认；逐 run 拆行。

停车列表：`_createTextInfo` 全量（TimingInfo）、下划线/删除线/字距 run 标志（DecoCommand 等价，post-MVP）、bullet 跟随文本样式（`isColorFollowText`）、autofit 字号缩放。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读 `ShowRichTextLayout.renderText` 的 span 绘制循环（wpslayout）与 `ShapeTextRender._createTextInfo` 的 dirty 位（仅作参照，非改写）。Rust 侧 wpslayout 未移植（3352 todo!），本 function 只消费 pl-05 提供的 span 样式结构。

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| span 映射 | 2 run（白/红） | 2×DrawText 各自样式 | 样式缺失默认可 |
| mFirstSpan | 无 run 级 | 段首 span | — |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| ST01 | 单 run → font/size/color/bold/italic 附着 | 主路径 | 是 | T01 |
| ST02 | 多 run 拆行（白/红同段落） | 主路径 | 是 | T02 |
| ST03 | 样式缺失 → 默认字段（不 panic） | 边界 | 是 | T03 |
| ST04 | 下划线/删除线/字距标志 | 延后 | 否 | `#[ignore = "spec: post-MVP"]` |
| ST05 | TimingInfo（动画） | 延后 | 否 | `#[ignore = "spec: post-MVP"]` |

### 3.3 可测试性

fixture `SpanInfo` 数组（2 run 样式）+ 行定位；断言 DrawText 样式字段；无 canvas。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | ST01 | span{font:"Arial",size:24,color:红} | DrawText{font,size,color} | 红测停在本 function |
| T02 | ST02 | 同段两 run（默认色 + 红色） | 2×DrawText 独立样式 | 红测停在本 function |
| T03 | ST03 | span 无样式字段 | 默认值产物 | 红测停在本 function |
| T04 | ST04 | underline 标志 | 不落指令（延后） | ignore |
| T05 | ST05 | dirty 位 | 无 TimingInfo | ignore |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/render/TextRender.rs（02 的样式分支）
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "02_function_run_style_command"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_run_style_command

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_run_style_command \
  --src src/cn/wps/show/render/TextRender.rs \
  --report libs/ppt-core/render/features/pr-03-text-command/functions/02_function_run_style_command/FUNCTION.md
```

工具缺失先：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`。

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_run_style_command --json`)
date: 2026-08-30 17:26

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/rendercommand.rs` | 21/113 | 18.6% | 15/106 | 14.2% |
| **function total** | **21/113** | **18.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/TextRender.rs \
  --java koffice/presentation/core/cn/wps/show/render/TextRender.java
```

Loop 结果：**PASS**

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/TextRender.java \
  --rs libs/ppt-core/src/cn/wps/show/render/TextRender.rs \
  --report libs/ppt-core/render/features/pr-03-text-command/functions/02_function_run_style_command/FUNCTION.md
```

<!-- java-rust-align:begin -->
| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
| N/A (`rendercommand.rs`; Canvas not ported) | PASS (`--rs` only / no Java twin) | command I/O in this function | none | none | PASS |
| **overall** | | | | | **PASS** |
<!-- java-rust-align:end -->