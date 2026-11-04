# 01_function_text_body — p:txBody 段落与 run 链

> feature: pp-11-dml-txbody ｜ 状态：**pass** ｜ Java 管线环节：S0 parse → S1 convert 输入

## 0. 管线能力解锁

形状可画（pp-10）但**文字进不了模型**时，pl-04 `ShapeDataTransform.exToInternal` 无 para/run
原料 → 文本框是空壳 → pr-03 无 `DrawText` → **有字的 pptx 画不出字**。实现后 txBody 的段落/run
链进入 `TextBody` → pl-04 有排版原料 → 文字可排可画；缺失则链断在「文本框无字」。

## 1. 功能说明

`TextBodyHandler`（`p:txBody` → `data/text/TextBody.rs`）：

- `enterSubElement`：bodyPr=1114448 → `BodyPropertiesHandler`（交 02）；lstStyle=1114449 →
  `TextListStyleHandler`（**归 pp-03，本 function 不实现**——Java 下构造，Rust 以 pp-03 契约
  接线或 fixture）；p=1114562 → 新建 `TextParagraph` 追加 `paraList`，`ParagraphHandler` 单实例
  `init` 复用。
- `endElement`：paraList 非空才 `paraLst().forceWrite(paraList)`。

`ParagraphHandler`（`a:p`）：`enterSubElement` 的 r=1114193/br=1114618/fld=1114619 → 新建
`TextRun` 追加 `runList`（`TextRunsHandler` 复用）；pPr=1114531 → `TextParagraphPropertiesHandler`
（壳，最小集随 pl-05）；endParaRPr=1114532 → `TextCharacterPropertiesHandler`（同一 rPr 模板）。
`endElement`：**GVML 判定**（`callback != null && callback.isGVml()`）→ `onMergeThemeTypeface(run)`
逐 run；随后 `paragraph.textruns().write(runList)`。

`TextRunsHandler`（run 状态机）：startElement 依元素置 mode——r→0、br→1（`putLength(1)`）、
fld→2（读 id=1114245/type=1114388 落 `fieldRun()`，缺 id 不置 mode 2）；`enterSubElement`：
t=1114214 → mode 0 走 `RegularTextHandler`/mode 2 走 `FieldTextHandler` 的 `characters(String)`：
`putText(chars) + putLength(chars.len())`（mode 1 无文字）；rPr=1114603 → 按 mode 选
regular/linebreak/field 的 rPr。fld 的 pPr 分支（1114531）仅 mode 2 返回。

rPr 最小集（`TextCharacterPropertiesHandler`）：sz=1114589（`toInt`）、b=1114195、i=1114504、
u=1114590（`DGMap.textUnderlineMap`）、strike=1114591（`textStrikeMap`）、cap=1114441
（`textCapMap`）、lang=1114587、altLang=1114588、kern=1114592、spc=1114593、baseline=1114595
（`toPercentage`）、kumimoji=1114586、normalizeH=1114594、noProof=1114596、dirty=1114597、
err=1114598、smtClean=1114599、smtId=1114600、bmk=1114601（characters/options 槽写入）。
子元素最小集：仅 latin/ea/cs/sym（1114140/1114141/1114142/1114584，字体装载契约）+ rtl=1114493；
ln/fill/effect/高亮/uLn/超链接分支延后（返回 null）。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `p:txBody` 的 p/bodyPr/lstStyle 子元素事件 + 段落内 run/fld/br 与 attrs + 构造入参 `TextBody textBody` + `HandlerCallback`。 |
| **输出** | `TextBody.paraLst()` 写 `List<TextParagraph>`；每段 `textruns()` 写 `List<TextRun>`；run 的 mode/length/text/fieldRun.id/type + rPr 最小属性；GVML 时逐 run `onMergeThemeTypeface`。 |
| **状态与副作用** | 容器均「endElement 才 write/forceWrite」（txBody 的 paraLst 在 `TextBodyHandler.endElement`；runList 在 `ParagraphHandler.endElement`）。handler 单实例 `init` 复用——同阶段无旧状态泄漏（Java 语义：同一 handler 绑定新对象）。`characters` 在每个文本块事件触发。 |
| **错误语义** | fld 缺 id → 不置 mode 2（Java：`tmp != null` 判空）。GVML 判空（`callback != null`）。`t` 的字符原文写入，不做 trim——空白/换行由 Java `characters(String)` 语义原样保留（Rust 禁止静默归一）。rPr 的 DGMap miss → `put*` 收 null 该槽不写。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止去改 |
|---|---|---|---|---|---|
| `HandlerCallback` | pp-01:02 | `isGVml()`、`onMergeThemeTypeface(run)` | **pass** | FileManager stub | **pass 语义**：判空/调用时机不可改 |
| txBody 事件入口 | pp-02:02/03/04 | 形状路径构造 `TextBodyHandler` | pass（02/03/04 已验收，txBody 原为 constructor fixture） | — | 不改 DgStore 提交 |
| `TextBody`/`TextParagraph`/`TextRun`/`TextCharacterProperties` | data 模型 | `paraLst().forceWrite`、`textruns().write`、`putMode/putLength/putText`、rPr put* | `data/text/*.rs` 齐备（953/953/2089/2922 行） | — | 不改字段语义 |
| `TextListStyleHandler` | pp-03 | lstStyle 契约 | pp-03 spec | constructor fixture | 不在此实现 |
| `BodyPropertiesHandler`（bodyPr 分支） | pp-11:02 | 由本 function 分发 | spec（随 02 落地） | — | 02 内部接线归 02 |
| textshape `.rs` handler | 本 feature | Java 结构模板 | **重做**：审计 `TextBodyHandler.rs` 160 行 **10 todo**、`ParagraphHandler.rs` 415 行（**runs broken**）、`TextCharacterPropertiesHandler.rs` 452 行 **49 stubs**；模块接线已注释（`textshape/text/mod.rs`） | — | 不增量，删本地 stub |

**禁改既有 pass 语义**：pp-01:02 `HandlerCallback` 的缺省回调策略、pp-02 的 DgStore/有序树
提交——本 function 只消费事件并写 `TextBody`，不碰上游结构。

## 2. 核心代码与函数改写列表

| Java 类 | Rust 目标 | 关键方法/字段 |
|---|---|---|
| `TextBodyHandler` | `…/textshape/text/TextBodyHandler.rs` | 字段 `textBody`、`handlerCallback`、`paraList`、`paragraphHandler`；`ctor(TextBody, HandlerCallback)`、`enterSubElement(int)`、`endElement(int)` |
| `ParagraphHandler` + nested `TextRunsHandler`/`RegularTextHandler`/`FieldTextHandler` | `…/textshape/text/paragraph/ParagraphHandler.rs` | `ctor/init(TextParagraph, HandlerCallback)`、`enterSubElement`（pPr/endParaRPr/r/br/fld）、`endElement`（GVML + write）；`TextRunsHandler.startElement`（mode 状态机）、`characters(String)` |
| `TextCharacterPropertiesHandler` + nested `RightToLeftHandler` | `…/textshape/text/run/TextCharacterPropertiesHandler.rs` | `ctor(runProp, HandlerCallback)`、`startElement`（count+getAttrAt 循环、19 个 attr case）、`enterSubElement`（font 最小集/rtl；其余 null） |

id 已查证（`dml/A.java`）：bodyPr=1114448、lstStyle=1114449、p=1114562、r=1114193、br=1114618、
fld=1114619、pPr=1114531、endParaRPr=1114532、t=1114214、rPr=1114603、id=1114245、type=1114388、
sz=1114589、b=1114195、i=1114504、u=1114590、strike=1114591、cap=1114441、lang=1114587、
altLang=1114588、kern=1114592、spc=1114593、baseline=1114595、kumimoji=1114586、normalizeH=1114594、
noProof=1114596、dirty=1114597、err=1114598、smtClean=1114599、smtId=1114600、bmk=1114601、
latin=1114140、ea=1114141、cs=1114142、sym=1114584、rtl=1114493、val=1114158。

函数改写列表：

- `TextBodyHandler`：p 分支的 handler 复用（创建与 init 两路径）；endElement 判空 forceWrite。
- `ParagraphHandler`：run 调度 + GVML 回调时机（`isGVml()` 判空在 callback 判空之内）。
- `TextRunsHandler`：mode 状态机三态；fld 的 id/type 判空语义；`characters` 原样写文本与
  `putLength`（UTF-8 长度 vs Java `chars.length()` 的差异以 kfc 语义为准，待 loop 确认）。
- `TextCharacterPropertiesHandler`：count+getAttrAt 循环 + 19 case 数值与 Java 完全一致；
  `toPercentage`/`toInt`/`toBoolean` 语义复用 kfc。
- `todo!("DEFERRED_NON_FUNCTION")` 停车项：paraPr 全量、rPr 的 fill/ln/effect/highlight/
  超链接/字体族业务体、lstStyle（归 pp-03）——均标注延后。

## 3. 测试方案

### 3.1 代码审计与测试面

- `TextBodyHandler.java`/`ParagraphHandler.java`/`TextCharacterPropertiesHandler.java` 全部分支与
  attr name 数值（对照 §2 表）。
- `data/text/TextRun.java` 的 mode/length/fieldRun 语义；对照 Rust `data/text/TextRun.rs`。
- Rust 现状：审计表三文件行数/stub 数（160/415/452）；工作树模块接线注释；不增量。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | txBody 多段：p 新建/复用 + endElement forceWrite 时机 | 循环/状态 | 是 | T01 |
| C02 | run mode 三态：r/br/fld（fld 含缺 id） | 分支 | 是 | T02 |
| C03 | `t` 文本原样（含空白/换行）→ putText+putLength | 数据 | 是 | T03 |
| C04 | rPr 最小集（sz/b/i/u/strike/lang/spc/baseline） | 分支 | 是 | T04 |
| C05 | GVML 回调：callback null / isGVml 真假 | 回调节点 | 是 | T05 |

### 3.3 可测试性

内存 attrs/events；真实 `data/text` 模型；rPr 用 attrs fixture（count+getAttrAt）。GVML 用假
callback fixture 验证调用恰好一次。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | txBody 三个 p（其中 p2 为空段） | paraLst 3 项顺序保持；空段也入列 | 红测停在本 function |
| T02 | C02 | r(text) + br + fld(id/type) + fld(无 id) | mode 0/1/2；缺 id 的 fld 不置 2 | 红测停在本 function |
| T03 | C03 | `t` 含 “ 甲\n乙 ” | `putText` 原样保留前后空白与换行；length 正确 | 红测停在本 function |
| T04 | C04 | rPr attrs sz=1800/b=true/i=true/u=“sng”/lang=“zh-CN”/baseline=“30000” | options/lang/sz/baseline 与 Java 一致；u map miss 不写 | 红测停在本 function |
| T05 | C05 | callback null；GVML true；GVML false | onMergeThemeTypeface 仅在 isGVml 时逐 run 一次 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.rs
       src/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.rs
       src/cn/wps/drawing/dmlr/handler/shape/textshape/text/run/TextCharacterPropertiesHandler.rs
test:  libs/ppt-core/tests/cn/wps/drawing/dmlr/handler/shape/textshape/text/01_function_text_body.rs
[[test]] name = "01_function_text_body"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_text_body

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_text_body \
  --src src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/textshape/text/run/TextCharacterPropertiesHandler.rs \
  --report libs/ppt-core/parser/features/pp-11-dml-txbody/functions/01_function_text_body/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_text_body --json`)
date: 2026-09-01 19:41

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.rs` | 31/54 | 57.4% | 52/76 | 68.4% |
| `src/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.rs` | 114/181 | 63.0% | 170/260 | 65.4% |
| `src/cn/wps/drawing/dmlr/handler/shape/textshape/text/run/TextCharacterPropertiesHandler.rs` | 42/144 | 29.2% | 56/169 | 33.1% |
| **function total** | **187/379** | **49.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.java
```

Loop 结果：PASS（TextBodyHandler / ParagraphHandler / TextCharacterPropertiesHandler）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/run/TextCharacterPropertiesHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/run/TextCharacterPropertiesHandler.rs \
  --report libs/ppt-core/parser/features/pp-11-dml-txbody/functions/01_function_text_body/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 11:41

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TextBodyHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/TextBodyHandler.rs` | **PASS** | `TextBodyHandler`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `ParagraphHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.rs` | **PASS** | `ParagraphHandler`, `init`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `TextRunsHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.rs` | **PASS** | `TextRunsHandler`, `init`, `startElement`, `enterSubElement` | — | — | **PASS** |
| `RegularTextHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.rs` | **PASS** | `RegularTextHandler`, `characters` | — | — | **PASS** |
| `FieldTextHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/paragraph/ParagraphHandler.rs` | **PASS** | `FieldTextHandler`, `characters` | — | — | **PASS** |
| `TextCharacterPropertiesHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/run/TextCharacterPropertiesHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/run/TextCharacterPropertiesHandler.rs` | **PASS** | `TextCharacterPropertiesHandler`, `enterSubElement`, `startElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->