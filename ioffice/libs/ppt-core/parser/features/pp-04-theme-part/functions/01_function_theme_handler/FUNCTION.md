# 01_function_theme_handler — a:theme XML 分发

> feature: pp-04-theme-part ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：theme XML 事件被分发写入 `KmoTheme`（name/clrMap/themeElements/objectDefaults/custClrLst 子树全有入口），scheme 色/字体/对象默认填充样式进入模型。缺失时断在哪：无 `KmoTheme` 则无 scheme 色/字体/填充定义，实色之外的形状颜色 S1/S4 全断（画色必要）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(KmoTheme, theme XML events) -> theme model + child ElementHandler

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | theme XML 事件（元素 id + `XmlRoAttr`），由调用方按 `ElementHandler` 接口驱动：`startElement(int, XmlRoAttr)` / `endElement(int)` / `enterSubElement(int)`（`characters` 用默认空实现）；目标模型 `KmoTheme`（Java 经 `KmoProtocol themeReceiver` 传入 ctor）。事件源在 Java 为 `UnifiedXmlReader`（移 pp-07）；本层 loop 前用注入式/内存事件 fixture。 |
| **输出** | theme 模型字段写入 + 子元素分发：`name` 属性(1114136)→`themeData.putName`；`endElement` 时 `themeReceiver.updateProtocol(themeData.save())`（`KmoProtocol` 契约）。分发返回子 handler：`clrMap(1114459)→ColorMapHandler`、`themeElements(1114461)→ThemeElementsHandler`、`objectDefaults(1114462)→ObjectDefaultsHandler`、`extraClrSchemeLst(1114463)→ColorSchemeListHandler`、`custClrLst(1114464)→CustomColorListHandler`。 |
| **状态与副作用** | ctor 内 `Theme.create()` 建 `themeData`；`handlerCallback` 透传给子 handler（`buildDefaultTheme` 路径合法为 null，见 02）；列表型子 handler（`customColorLst`/`extraColorSchemeLst`）`endElement` 时对列表元素逐一 `save()` 后 `write` 回 `themeData.customColorLst()` / `extraColorSchemeList()`。 |
| **错误语义** | 未知元素 `enterSubElement` 返回 null（Java `null` → 不下沉，引擎 stop）；本 handler 不 catch —— XML 读取/`save()` 异常沿调用方传播（02 `open()` 处吞 Throwable 返回 null）。 |

**回调契约**（xpxml `ElementHandler`，Java 源码查证；Rust 目标同名 trait）：

| 回调 | Java（`ThemeHandler` 覆写，基类 `DefaultElementHandler` 为空实现） | 行为 |
|---|---|---|
| `startElement(int element, XmlRoAttr attrsNotCopy)` | 覆写 | 仅 `attrsNotCopy.getByName(1114136)`（name）→ `putName` |
| `enterSubElement(int subElement)` | 覆写 | 分发表 → 子 handler；unknown/`extraClrScheme`(1114460) → null |
| `endElement(int element)` | 覆写 | `themeReceiver.updateProtocol(themeData.save())` |

元素 id 表（`koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/xsd/dml/A.java` 查证）：
`name=1114136`；`custClr=1114139`；`clrScheme=1114154`、`fontScheme=1114155`、`fmtScheme=1114156`；
`bodyPr=1114448`、`lstStyle=1114449`、`spDef=1114450`、`lnDef=1114451`、`txDef=1114452`；
`clrMap=1114459`、`extraClrScheme=1114460`、`themeElements=1114461`、`objectDefaults=1114462`、
`extraClrSchemeLst=1114463`、`custClrLst=1114464`；`spPr=3211578`、`style=3211579`。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 见 FUNCTIONS.md §6 | | | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/drawing/dmlr/handler/style/ThemeHandler.java`（219 行，含 6 个嵌套类） | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/style/ThemeHandler.rs`（模板未生成；loop 起点 = Java 全量） |

### 2.1 关键方法/字段表（Java 源码查证 → Rust 目标）

| Java 成员 | 签名/字段 | Rust 目标 | 归属 |
|---|---|---|---|
| 字段 `themeReceiver` | `KmoProtocol` | 接收者 `&mut KmoTheme`（trait 化 `updateProtocol`） | 本 function |
| 字段 `handlerCallback` | `HandlerCallback` | `Option<&dyn HandlerCallback>`（可 null） | 本 function |
| 字段 `themeData` | `Theme`（ctor 中 `Theme.create()`） | `Theme`（库内 `data/style/Theme.rs` 已有 `create`） | 本 function |
| ctor | `ThemeHandler(KmoProtocol, HandlerCallback)` | `new` | 本 function |
| `startElement` | `void startElement(int, XmlRoAttr)` | `start_element` | 本 function |
| `enterSubElement` | `ElementHandler enterSubElement(int)` | `enter_sub_element` | 本 function |
| `endElement` | `void endElement(int)` | `end_element` | 本 function |
| 嵌套 `ObjectDefaultsHandler` | 仅 `enterSubElement`（`spDef/lnDef/txDef`） | 内嵌模块 | 本 function（分发） |
| 嵌套 `DefaultShapeDefinitionHandler` | ctor + `enterSubElement`（`bodyPr/lstStyle/spPr/style`） | 内嵌模块 | 本 function（分发） |
| 嵌套 `CustomColorListHandler` | `enterSubElement`(custClr) + `endElement`(save/write) | 内嵌模块 | 本 function |
| 嵌套 `CustomColorHandler` | ctor + `init` + `startElement`(name) + `enterSubElement`(ColorHandlerFactory) | 内嵌模块 | 本 function |
| 嵌套 `ColorSchemeListHandler` | `enterSubElement`(extraClrScheme) + `endElement`(save/write) | 内嵌模块 | 本 function |
| 嵌套 `ColorSchemeAndMappingHandler` | ctor + `init` + `enterSubElement`(clrScheme/clrMap) | 内嵌模块 | 本 function |
| Theme 模型访问 | `putName/colorMapping/shapeDef/lineDef/textDef/customColorLst/extraColorSchemeList/save` | `Theme.rs` 已实现 | 复用（不改） |

### 2.2 函数改写列表

- **本 function 改写：** `ThemeHandler` 全类 —— ctor、`startElement`、`enterSubElement`、`endElement`
  + 6 个嵌套类（`ObjectDefaultsHandler`、`DefaultShapeDefinitionHandler`、`CustomColorListHandler`、
  `CustomColorHandler`、`ColorSchemeListHandler`、`ColorSchemeAndMappingHandler`）。
- **停车（分发目标，本 function 只接线不实现）：** `ColorMapHandler`（pp-03）、`ColorSchemeHandler`/
  `FontSchemeHandler`/`StyleMatrixHandler`/`ThemeElementsHandler`、`ColorHandlerFactory`、
  `BodyPropertiesHandler`/`TextListStyleHandler`/`ShapePropertiesHandler`/`ShapeStyleHandler` ——
  Rust 侧 `todo!("DEFERRED_NON_FUNCTION")` 停车，仅保证 `enterSubElement` 分发返回类型存在。
- **禁改：** `Theme.rs`/`KmoTheme.rs` 模型既有语义、pp-01/pp-02 已 pass 语义、`SWReaderMap`（pp-01）。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读 `ThemeHandler.java` 全量（含 6 个嵌套类方法体）与当前 Rust（模板未生成时只读 Java）。
禁止只按标题冒烟。审计面按层拆：

| 层 | Java 读点 | 审计内容 |
|---|---|---|
| L1 根分发 | `ThemeHandler.startElement/enterSubElement/endElement` | name 属性、5 路分发表、`updateProtocol(save())` |
| L2 objectDefaults | `ObjectDefaultsHandler` / `DefaultShapeDefinitionHandler` | spDef/lnDef/txDef → bodyPr/lstStyle/spPr/style 分发链 |
| L3 列表 | `CustomColorListHandler` / `ColorSchemeListHandler` | 惰性建表、handler 复用 `init`、`endElement` 逐个 `save()` 后 `write` 回模型 |
| L4 只读参照 | `ThemeElementsHandler.java`（clrScheme/fontScheme/fmtScheme 分发）、`ColorMapHandler.java`（clrMap 属性写法，pp-03） | 分发目标语义，防止接管目标实现 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 已知 theme 子元素（clrMap/themeElements/objectDefaults/extraClrSchemeLst/custClrLst）→ 非 None kind | 分支 | 是 | T01 |
| C02 | unknown → None（Java null） | 失败 | 是 | T02 |
| C03 | name 属性(1114136) → putName | 正常 | 是 | T03 |
| C04 | objectDefaults：spDef/lnDef/txDef → 内层 bodyPr/lstStyle/spPr/style 分发链 | 分支 | 是 | T04 |
| C05 | custClrLst：列表积累 + endElement save/write 回模型 | 状态 | 是 | T05 |
| C06 | extraClrSchemeLst：列表积累 + extraClrScheme → clrScheme/clrMap 分发 | 状态 | 是 | T06 |
| C07 | callback null 路径（buildDefaultTheme 用） | 边界 | 是 | T07 |
| C08 | endElement → updateProtocol(themeData.save()) | 状态 | 是 | T08 |

### 3.3 可测试性

内存 attrs/events fixture 模拟 `HLXmlReader` 事件序，驱动 `startElement/enterSubElement/endElement`；
目标为真实 `KmoTheme`（`updateProtocol` 接 `Theme.save()`）+ 真实 `Theme` 模型；callback 用假实现
（无媒体、`load()` 空），断言只到模型字段/分发 kind。不跑完整 `PptxReader`（本 function 不是 pp-09
编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 最小内存输入：theme 根下逐个分发 clrMap/themeElements/objectDefaults/extraClrSchemeLst/custClrLst | 各子 handler 为 Some 且 kind 与 Java 一致 | 红测停在本 function |
| T02 | C02 | 未知子元素 id（如 `lnL=1114468`） | `enterSubElement` 返回 None | 红测停在本 function |
| T03 | C03 | `startElement` 带 `name=1114136` 属性 | `themeData` name 写入 | 红测停在本 function |
| T04 | C04 | objectDefaults → spDef → bodyPr/lstStyle/spPr/style 事件序 | 分发链逐级非 None kind | 红测停在本 function |
| T05 | C05 | custClrLst 下两个 custClr(1114139) + endElement | 列表 size=2、write 回 `customColorLst`、元素 save | 红测停在本 function |
| T06 | C06 | extraClrSchemeLst 下两个 extraClrScheme(1114460) 各带 clrScheme/clrMap | 列表 size=2、write 回模型、分发非 None | 红测停在本 function |
| T07 | C07 | callback = null 驱动同一分发表 | 与 Java 一致（不 panic，分发正常） | 红测停在本 function |
| T08 | C08 | theme 根 endElement | 触发一次 `updateProtocol`，KmoTheme 数据可达 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/drawing/dmlr/handler/style/ThemeHandler.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "01_function_theme_handler"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_theme_handler

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_theme_handler \
  --src src/cn/wps/drawing/dmlr/handler/style/ThemeHandler.rs \
  --report libs/ppt-core/parser/features/pp-04-theme-part/functions/01_function_theme_handler/FUNCTION.md
```

工具缺失先：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`。

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_theme_handler --json`)
date: 2026-09-01 11:56

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/style/ThemeHandler.rs` | 235/274 | 85.8% | 222/263 | 84.4% |
| **function total** | **235/274** | **85.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/style/ThemeHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/style/ThemeHandler.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/style/ThemeHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/style/ThemeHandler.rs \
  --report libs/ppt-core/parser/features/pp-04-theme-part/functions/01_function_theme_handler/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 03:56

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ThemeHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/style/ThemeHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/style/ThemeHandler.rs` | **PASS** | `ThemeHandler`, `startElement`, `enterSubElement`, `endElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->