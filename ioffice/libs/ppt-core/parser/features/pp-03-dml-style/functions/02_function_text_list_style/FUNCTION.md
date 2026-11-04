# 02_function_text_list_style — a:lstStyle 列表样式

> feature: pp-03-dml-style ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：`a:lstStyle` 的默认级 + lvl1-9 九级段落样式可分发进 `TextListStyle` 各槽，文本默认段落样式有了解析入口。缺失时断在哪：默认文本样式缺，文本 run 无段落级样式可继承，S1 排版原料不完整（有字才必要）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

`a:lstStyle` 定义文本列表默认级别 + 九个级别的段落样式。本 function 实现 `TextListStyleHandler`：
按子元素 id 从 `TextListStyle` 取对应级别槽（`defpPr()`/`lvl1pPr()`…`lvl9pPr()`），返回同一个
`TextParagraphPropertiesHandler` 实例（首次 new、其后 `init` 复用）或 `None`。构造方
（pp-01:07 `TextStylesHandler` 的 title/body/other、pp-06 `PresentationHandler` 的
defaultTextStyle、`TextBodyHandler`/`ThemeHandler` 的 lstStyle）拿到的 handler 才是段落属性解析入口；
本 function 不改写 `TextParagraphPropertiesHandler` 的方法体。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `enterSubElement(int subElement)` 的子元素 id + 构造入参 `TextListStyle listStyle`、`HandlerCallback handlerCallback`。 |
| **输出** | `ElementHandler`（`TextParagraphPropertiesHandler`，已知 id）或 `None`（unknown id）。 |
| **状态与副作用** | 已知 10 id → `listStyle.defpPr()`/`lvl1pPr`…`lvl9pPr()`；槽非空则首次 `new TextParagraphPropertiesHandler(textParagraphProperties, handlerCallback)`、其后同一实例 `init(textParagraphProperties)` 复用（Java 字段 `textParagraphPropertiesHandler` 只建一次）。元素 id 取 kfc `dml/A.java`：defPPr=1114533、lvl1pPr=1114534、lvl2pPr=1114535、lvl3pPr=1114536、lvl4pPr=1114537、lvl5pPr=1114538、lvl6pPr=1114539、lvl7pPr=1114540、lvl8pPr=1114541、lvl9pPr=1114542。Java `defpPr()` 等 lazy getter 对已知 id 恒非 null。 |
| **错误语义** | 无 throw；未命中任何 case → handler 保持 null，返回 `None`，`listStyle`/`handlerCallback` 不变。`endElement` 未 override（继承 `DefaultElementHandler` 空实现，本类无 endElement 契约）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `TextListStyle` | model，E | defpPr/lvl1-9pPr 槽（lazy getter 语义） | Rust 已存在（`data/text/TextListStyle.rs` `pub fn defpPr/lvl*`，行 762-1554） | | 不改槽存储 |
| `HandlerCallback` | 02 | 透传给段落属性 handler | **pass** | | |
| `TextParagraphPropertiesHandler` | textshape，C | ctor(TextParagraphProperties, HandlerCallback) + `init(TextParagraphProperties)` | 只引用类型（kind） | 方法体 `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不在本文件展开其方法体 |
| 见 FUNCTIONS.md §6 | | | loop 前未接线 | | 不改已 pass 的 pp-01/pp-02 语义 |

Rust 目标 `src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.rs` **尚无**（`text/mod.rs` 仅 `run`/`paragraph` 两子模块）。

## 2. 核心代码与函数改写列表

| Java 类 | Rust 目标 | 关键方法/字段 |
|---|---|---|
| `TextListStyleHandler` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.rs`（新建模板，尚无 Rust） | 字段 `listStyle`、`handlerCallback`、`textParagraphPropertiesHandler`；方法 `ctor(TextListStyle, HandlerCallback)`、`enterSubElement(int)` |

函数改写列表（模板 `// TODO_RUST_CODE` 在本 function 内实现的部分）：

- ctor：保存 `listStyle` 与 `handlerCallback`（Rust 引用/所有权写法待施工定，模板为准）。
- `enterSubElement`：10 个 case（defPPr=1114533 + lvl1-9pPr=1114534-1114542）→ 取槽；槽非空 → 首次建 handler / 其后 `init` 复用；否则返回 `None`。
- `todo!("DEFERRED_NON_FUNCTION")` 停车项：`TextParagraphPropertiesHandler` 方法体（其 `startElement`/`enterSubElement` 分发展开属后续 feature，本 function 只构造与复用）。
- 属其它 function / 已存在：`TextListStyle` 槽方法（model，已存在）；`TextParagraphPropertiesHandler` 本体归属待 loop 确认。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读（禁止只按标题冒烟）：

- `TextListStyleHandler.java` `enterSubElement` 方法体：逐 case、10 个子元素 id 数值、单实例复用路径（首建 → `init`）。
- `TextParagraphPropertiesHandler.java` ctor/`init`：确认复用契约（字段 `textProp` 被替换）。
- 构造方确认：`TextStylesHandler.java`（3211615/3211616/3211617 → titleStyle/bodyStyle/otherStyle）、`PresentationHandler.java`（1114449 → defaultTextStyle）等。
- Rust 现状：`src/cn/wps/drawing/dmlr/handler/shape/textshape/text/` 下**尚无** `TextListStyleHandler.rs`；`data/text/TextListStyle.rs` 已存在 `defpPr()/lvl1pPr()…lvl9pPr()`。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 已知 lstStyle 子 id（defPPr + lvl1-9pPr 共 10 个） | 分支 | 是 | T01 |
| C02 | unknown id → None | 失败 | 是 | T02 |
| C03 | title/body/other 槽 | 交互 | 是 | T03 |
| C04 | 同一 handler 实例复用（new → init） | 状态 | 是 | T04 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 10 个已知 id 各一次 | 返回非 null handler，对应槽被引用；与 Java 一致 | 红测停在本 function |
| T02 | C02 | 未知 id（如 1114448 / 任意未列 id） | 返回 None，无副作用 | 红测停在本 function |
| T03 | C03 | 经 `TextStylesHandler` 3211615/3211616/3211617 | 分别绑定 titleStyle/bodyStyle/otherStyle 的 TextListStyle | 红测停在本 function |
| T04 | C04 | 连续两次已知 id 调用 | Java 返回同一 handler 实例（第二次走 `init`）；Rust 等价语义（复用 vs 新建）待施工定案 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.rs
test:  libs/ppt-core/tests/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.rs
[[test]] name = "02_function_text_list_style"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_text_list_style

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_text_list_style \
  --src src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/TextParagraphPropertiesHandler.rs \
  --report libs/ppt-core/parser/features/pp-03-dml-style/functions/02_function_text_list_style/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_text_list_style --json`)
date: 2026-09-01 17:09

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.rs` | 114/122 | 93.4% | 78/90 | 86.7% |
| `src/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/TextParagraphPropertiesHandler.rs` | 18/135 | 13.3% | 17/140 | 12.1% |
| **function total** | **132/257** | **51.4%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.java
```

Loop 结果：**PASS** (`check-template-layout.mjs` TLS + `TextParagraphPropertiesHandler`)

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.rs \
  --report libs/ppt-core/parser/features/pp-03-dml-style/functions/02_function_text_list_style/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 09:09

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TextListStyleHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.rs` | **PASS** | `TextListStyleHandler`, `enterSubElement` | — | — | **PASS** |
| `TextParagraphPropertiesHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/TextParagraphPropertiesHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/TextParagraphPropertiesHandler.rs` | **PASS** | `TextParagraphPropertiesHandler`, `init`, `startElement`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->