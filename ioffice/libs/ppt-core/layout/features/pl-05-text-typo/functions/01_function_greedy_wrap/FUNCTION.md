# 01_function_greedy_wrap — 框内贪心断行

> feature: pl-05-text-typo ｜ 状态：**pass** ｜ Java 管线环节：S2 布局

> **一致性声明：** 本 function 是 **MVP 贪心断行**，仅对照 Java `RichTextLayout.typo` 的「行 = 放得下的字符区间 + 行宽/行高」层语义。**Java 一致的断行另开 feature**（依赖 wpslayout `RichTextLayout.rs` 5346 行模板未实现），本 function 禁止冒充。

## 0. 管线能力解锁

实现后：`TextBoxData` 的段文本被切成可画的行——每行 `{text, startCp, endCp, w, h, x, y}`，pr-03 拿到它就能发 `DrawText`；行 `x/y` 已含 insets 与垂直 anchor 三态。这是「画面出现文字」的最小门。

没有它管线断在哪：文本停留在 `TextBoxData`（有内容、无行）；pr-03 无行可画，**有字 slide 的文字不上屏**；pl-06 想挂行几何也无从挂起。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `TextBoxData`（pl-04：`textDatas[].mSourceText` + `ParaInfo` + `SpanInfo[]`、`isWrapTextInShape`、insets、`anchor`）+ 可用宽 `max_width`（layout 单位，来自 02 或显式 fixture）+ 度量表（`span → advance[]`，FontHost 注入，可显式 fixture） |
| **输出** | 行列表 `Vec<ShowTypoLine>`：`{text, startCp, endCp, x, y, w, h}`（layout 单位，框坐标内）；`\r` 段切分后的空段落 → 空行（占行高） |
| **状态与副作用** | 纯函数，只读输入；不写 `TextBoxData` 外的全局；不度量（度量在注入表） |
| **错误语义** | 空文本/全空段 → 空行列表（Java `ShowTextLayout.layout` 语义为 null——空 box 在 pl-04 已滤）；度量表缺 span → 用 `fontSize` 的 CMS 经验宽（fallback 也须显式）：**具体 fallback 数值 待 loop 确认**，禁止隐式猜宽 |

### 1.2 输入
| 入口 | 输入 | Java 对标（对照非对齐） |
|------|------|-----------|
| `greedy_wrap` | `TextBoxData` + `max_width` + 度量表 | `RichTextLayout.typo(textBoxData, w, h, forceTypo)`（`RichTextLayout.java:78`）；断点层语义见 `typoOnePara`（`RichTextLayout.java:528`）/ Android `Layout` |

### 1.3 输出
| 输出 | 说明 | 对标 |
|------|------|-----------|
| `startCp/endCp` | 行在段内字符区间（含 `\r` 段界） | Java `Layout.getLineStart/End`（`typoCache.getLineEnd(line)` 见 `RichTextLayout.java:201`） |
| `w` | 行内字符 advance 累计 | `typoCache.getLineWidth(line)`（`RichTextLayout.java:204`） |
| `h` | 该行最大字号（MVP 简化） | Java 精确行高 = `getLineBottom-getLineTop`（`RichTextLayout.java:644`）——MVP 不逐行对齐，**待 loop 确认简化口径** |
| `x/y` | 框坐标；`x=left`（左对齐 stub），`y=top+Σ前行高` | `mTypoOffsets`（Java 的段落级偏移） |
| 段切分 | `\r` 结尾每段独立断行；`\u000b` 软换行延后 | `TextBoxData.processBeforeTypo`（`TextBoxData.java:604`）的 `forceProcessCrlf` 分支 |

### 1.4 上游
pl-04（`TextBoxData`）；02（可用宽）；pl-01（单位）。度量表注入口径对齐 writer wr-03（FontHost），本波 fixture 化。

### 1.5 下游
pl-06（可选挂行几何）；pr-03（`DrawText` 原料）。

### 1.6 不变量
1. 每行 `endCp > startCp`（空行除外）；跨行区间连续且不重叠覆盖段文本。
2. 断行只切字符区间，**不切 SpanInfo**（span 边界保留；pr-03 按 run 画）。
3. `isWrapTextInShape == false` → 整段单行（超出 `max_width` 也单行，MVP）。
4. `\r` 只作段界符，不出现在行 `text` 里。
5. 所有数值 layout 单位；行 `w ≤ max_width`（超宽单 token 强制折除外，为其自身宽）。
6. 禁止用假宽度：断行宽度要么来自 02 的 `max_width`，要么是显式 fixture——FUNCTION.md 已钉死，实现违反即 FAIL。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `TextBoxData`（文本/insets/anchor/wrap） | core-libs/wpslayout | 字段读取 | `TextBoxData.rs` 结构模板在（方法多空） | — | 禁止发明第二套文本类型 |
| 度量表 FontHost | writer wr-03（口径） | `span → advance[]` / `fontSize` | 否（fixture 化） | 真实字体度量 | 禁止在本包内做 TTF 度量 |
| 可用宽 | 02 / fixture | `max_width: f32` | 02 为 spec | — | 禁止在 01 里重算 insets |
| `RichTextLayout`（Java 一致版） | core-libs/wpslayout | 断行引擎 | **否（5346 行模板未实现）** | Java 一致另开 | 禁止填写 RichTextLayout.rs 冒充本 function |
| `ShowTypoLine` 行类型 | pr-03 消费方 | 字段名 | 新类型（命名 待 loop 确认） | — | 禁止发明与 writer `TypoTextLine` 混淆的第二个名 |

## 2. 核心代码与函数改写列表

Java→Rust 表（方法名均已查证；01 为「对照」列，非移植清单）：

| Java（对照） | Rust 计划 | 改写范围 |
|------|-----------|----------|
| `RichTextLayout.typo`（`RichTextLayout.java:78`） | `libs/ppt-core/src/cn/wps/show/layout/typo/GreedyWrap.rs`（新门面，MVP） | 01 入口；只实现贪心底层 |
| `typoOnePara` 的行区间语义（`RichTextLayout.java:528`） | 同上 | 01 内「段 → 行区间」循环 |
| `TextBoxData.processBeforeTypo` 的 `\r` 切分（`TextBoxData.java:604`） | 同上 | 01 段切分（仅 `forceProcessCrlf` 的 `\r(\n)|\n` 子集） |
| `Layout.getLineStart/End/Width`（wpslayout Android Layout） | `ShowTypoLine` 字段 | 01 输出的 cp 区间/宽度语义 |
| `ShapeTextRender._render` 消费行（`ShapeTextRender.java:28`） | pr-03 | 01 不自绘；仅保证字段可被 pr-03 消费 |

不在改写列表：`BaseTextLayout.typo` 全量、`modifyLineSpace/scaleTextPara/tryTypoNormalAutoFit`、竖排、`WordArtLayout`、bidi——`todo!("DEFERRED_NON_FUNCTION")` 停车或另开。

## 3. 测试方案

### 3.1 审计面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| `greedy_wrap` | 中文多行 | 「中文字符串」×窄框 | 每行按字折、cp 连续 | CJK 单字 |
| `greedy_wrap` | 英文词 | 多个空格分隔词 | 词完整、行 w ≤ max | 拉丁词断点 |
| `greedy_wrap` | 超宽 token | 单词 > max_width | 强制折 | 超宽 |
| `greedy_wrap` | `\r` 段 | 2 段 | 段界专行 | 段切分 |
| `greedy_wrap` | wrap=false | `isWrapTextInShape=false` | 单行 | 开关 |
| `greedy_wrap` | 空段/全空 | 仅 `\r`/空文本 | 空行 或 空列表（与 Java isEmpty 语义对齐） | 空输入 |
| 行高 | 混字号 | 一行内 2 字号 | h=max 字号 | 简化口径 |
| x/y | anchor 三态 | t/b/ctr | y 平移 | 垂直 anchor |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| GW-CJK | greedy_wrap | 中文逐字折行 | 是 | GW1 |
| GW-WORD | greedy_wrap | 拉丁词断点 | 是 | GW2 |
| GW-OVER | greedy_wrap | 超宽 token | 是 | GW3 |
| GW-CRLF | greedy_wrap | `\r` 段切分 | 是 | GW4 |
| GW-NOWRAP | greedy_wrap | wrap=false 单行 | 是 | GW5 |
| GW-EMPTY | greedy_wrap | 空段/全空 | 是 | GW6 |
| GW-HEIGHT | greedy_wrap | 行高=max 字号 | 是 | GW7 |
| GW-ANCHOR | greedy_wrap | anchor t/b/ctr 的 y | 是 | GW8 |

### 3.3 可测试性
纯函数 + 注入度量表：测试手算每行期望区间（用「按可用宽手算」的固定 advance 数组），不读 TTF、不启动 `ShowTextLayout.layout`、不跑 pl-04。度量表 fixture 与「可用宽 + advance 数组 → 期望行」一一对应。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| GW1 | GW-CJK | adv=10/字，max=25 | 「你好世界」 | 2 行：`[0,2)` `[2,4)` | 按词错折/越界 |
| GW2 | GW-WORD | words adv=10+空 5，max=25 | 「ab cd」… | 行=完整词 | 词被拦腰 |
| GW3 | GW-OVER | 单 token adv=40，max=25 | 超宽词 | 独立行 w=40 | 无限循环 |
| GW4 | GW-CRLF | 2 段带 `\r` | 「A\rB」 | 2 行，段界干净 | `\r` 漏进 text |
| GW5 | GW-NOWRAP | wrap=false | 20 字 × max=10 | 1 行 | 误折行 |
| GW6 | GW-EMPTY | 仅 `\r` | greedy_wrap | 空行（占位行高） | panic/无限循环 |
| GW7 | GW-HEIGHT | 2 字号 10/20 | 行 h | 20 | 用首字号 |
| GW8 | GW-ANCHOR | anchor=b | y | 从底部推 | 恒 top |

### 3.5 位置
crate：`libs/ppt-core/Cargo.toml`
src：`libs/ppt-core/src/cn/wps/show/layout/typo/GreedyWrap.rs`（新，loop 创建）
test：`libs/ppt-core/tests/cn/wps/show/layout/typo/GreedyWrap.rs`

### 3.6 脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_greedy_wrap

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_greedy_wrap \
  --src src/cn/wps/show/layout/typo/GreedyWrap.rs \
  --report libs/ppt-core/layout/features/pl-05-text-typo/functions/01_function_greedy_wrap/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_greedy_wrap --json`)
date: 2026-08-30 17:08

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/layout/typo/GreedyWrap.rs` | 162/167 | 97.0% | 255/263 | 97.0% |
| **function total** | **162/167** | **97.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（loop 必跑；PASS 才能标 pass）
本 function 是 **MVP 贪心门面，不是 RichTextLayout 模板的改写**——禁止把 `RichTextLayout.java` 与 `GreedyWrap.rs` 配成一对去伪造 PASS。本波对门面本身：

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/layout/typo/GreedyWrap.rs
```

结果：**PASS**（`java comments preserved and aligned`）。

### 3.9 Java↔Rust 结构对齐报告（loop 必跑；overall PASS 才能标 pass）
无 Java 直接双子（对照 `RichTextLayout` 非移植）。不跑 `java-rust-align-report.mjs --java RichTextLayout.java --rs GreedyWrap.rs`。Java 一致断行另开 feature 时再补双子行。
<!-- java-rust-align:begin -->
| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|------------|--------------------|-------------|----------------------------------|---------|--------|
| N/A (`GreedyWrap` MVP facade; not `RichTextLayout.java`) | PASS (`--rs` only) | `greedy_wrap` (+ helpers) | none | none | PASS |
| **overall** | | | | | **PASS** |
<!-- java-rust-align:end -->