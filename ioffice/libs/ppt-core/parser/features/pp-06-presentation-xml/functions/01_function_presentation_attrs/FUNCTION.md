# 01_function_presentation_attrs — p:presentation 根属性

> feature: pp-06-presentation-xml ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：`p:presentation` 根元素 12 个属性写入 `Presentation` 并经 `updateProtocol` 持久，打开链的全局数据有了源头。缺失时断在哪：根属性未消费则 `KmoPresentation` 全局配置不全，pp-08/09 打开链的整体装配缺前置（闭环打开必要）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(KmoPresentation, presentation start/end attrs) -> Presentation flags + protocol

### 1.1 I/O 契约

| 方向 | 契约 |
|---|---|
| 输入 | `p:presentation`(3211535) `startElement` 的 attrs（`count()`+`getAttrAt(i)` 循环）+ `endElement`。12 个 attr 名：serverZoom(3211523)/firstSlideNum(3211524)/showSpecialPlsOnTitleSld(3211525)/rtl(3211526)/removePersonalInfoOnSave(3211527)/compatMode(3211528)/strictFirstAndLastChars(3211529)/embedTrueTypeFonts(3211530)/saveSubsetFonts(3211531)/autoCompressPictures(3211532)/bookmarkIdSeed(3211533)/conformance(3211534)。id 来源：`kfc/xsd/pml/P.java` |
| 输出 | `Presentation` put* ：putServerZoom(double)/putFirstSlideNum(int)/putShowSpecialPlsOnTitleSld(boolean)/putRightToLeft(boolean)/putRemovePersonalInfoOnSave(boolean)/putCompatMode(boolean)/putStrictFirstAndLastChars(boolean)/putEmbedTrueTypeFonts(boolean)/putSaveSubsetFonts(boolean)/putAutoCompressPictures(boolean)/putBookmardIdSeed(int)/putConformace(int)。conformance 经 `SWReaderMap.conformanceClassMap` {strict→0, transitional→1}，miss → 1 |
| 状态与副作用 | `presData = Presentation.open(pres.protocol())` 在 ctor1 建立；attr 写入只在 `startElement`；`endElement` 时若 `mPres != null` 才 `mPres.updateProtocol(this.presData.save())`（save 结果持久进 protocol）。空 attrs → put* 全不发，save 仍执行，create 默认不变 |
| 错误语义 | ctor2（仅 callback）`presData == null`：startElement 触碰 presData 即 NPE（Java 事实；当前树上 `PptxrPresentation.openGlobalData` 只用三参 ctor）。conformance 值不在表 → 退默认 `1`，不失败。`getAttrAt(i) == null` 跳过 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 见 FUNCTIONS.md §6 | | | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs`（尚不存在；`handler/mod.rs` 当前仅声明 `custdatas`/`shape`） |

### 2.1 Java 成员 → Rust 目标（按 Java 源码查证）

| Java 成员（PresentationHandler.java） | 说明 | Rust 目标 |
|---|---|---|
| `KmoPresentation mPres` | ctor1 注入；`endElement` `null != mPres` 才 `updateProtocol(presData.save())` | 待定：libs/ppt-core 当前无 `KmoPresentation`，以 contract（保存 save 结果/kmo protocol 回调）替代 |
| `Presentation presData` | ctor1：`Presentation.open(pres.protocol())`；ctor2：null | `crate::cn::wps::show::data::Presentation`（已存在，put* 齐备） |
| `KsoFilterPasswdProvider passProvider` | 仅 `ModifyVerifierHandler` 使用（本 feature 移出） | 不进改写 |
| `PresentationHandlerCallback handlerCallback` | 02/04 与 EmbeddedFont 回调出口 | 沿用 pp-01 `02_function_handler_callback` 回调面 |
| `startElement(int element, XmlRoAttr attrsNotCopy)` | attr 名 switch 12 case（3211523–3211534 → put*）；`tmp == null` 跳过 | `fn start_element(&mut self, element: i32, attrs: &XmlRoAttr)` |
| `endElement(int element)` | `if (null != this.mPres) this.mPres.updateProtocol(this.presData.save());` 不区分 element 值 | `fn end_element(&mut self, element: i32)` |
| `enterSubElement(int subElement)` | 分发 3211509–3211514/3211516–3211522 给 02–04 委托；3211295 extLst → null | 本 function 只建立对 02–04 的委托；3211521 交 pp-03；3211522 移出 |

改写列表：两个 ctor、`startElement`（attr 循环 12 case）、`endElement`（仅根属性逻辑；子 handler 在 02–04）。
元素/attr id 全部按 `koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/xsd/pml/P.java` 查证：`presentation`=3211535，`serverZoom`=3211523，`firstSlideNum`=3211524，`showSpecialPlsOnTitleSld`=3211525，`rtl`=3211526，`removePersonalInfoOnSave`=3211527，`compatMode`=3211528，`strictFirstAndLastChars`=3211529，`embedTrueTypeFonts`=3211530，`saveSubsetFonts`=3211531，`autoCompressPictures`=3211532，`bookmarkIdSeed`=3211533，`conformance`=3211534。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读上表 Java 方法体与当前 Rust（若尚无 Rust 则只读 Java）。审计面：

1. 两个 ctor 的 `presData` 初始化时机：三参 ctor 立即 `Presentation.open(pres.protocol())`；仅 callback
   ctor 不建 `presData`（startElement 触碰即 NPE，测试只走三参路径）。
2. `startElement` attr 循环：`count()` + `getAttrAt(i)`，`tmp == null` 跳过；switch 用 `tmp.getName()`。
3. conformance(3211534)：`SWReaderMap.conformanceClassMap.get(tmp.toString())`，`null` → `1`；
   表只含 strict→0、transitional→1。
4. `endElement`：`mPres != null` 才 `updateProtocol(this.presData.save())`，与 element 值无关。
5. 类型转换：toDouble/toInt/toBoolean 与 attr 语义一一对应（serverZoom 为 double，bookmarkIdSeed 为 int）。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | `startElement` 12 case（3211523–3211534）各 attr 名 → 对应 put* | 分支 | 是 | T01 |
| C02 | conformance miss → `putConformace(1)`（不在 strict/transitional） | 失败 | 是 | T02 |
| C03 | 空 attrs（count==0）：put* 全不发，create 默认不变 | 边界 | 是 | T03 |
| C04 | ctor2（`mPres` null）`endElement`：不 `updateProtocol` | 边界 | 是 | T04 |
| C05 | 三参 ctor `endElement`：`updateProtocol(presData.save())` 被调、payload 非空 | 状态 | 是 | T05 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。
`XmlRoAttr` 用 pp-01 已提供的 attr 事件面构造（count/getAttrAt/getByName）；enter/end 事件单独入参。
`KmoPresentation` 以最小 contract fake 替代（记录 `updateProtocol` 调用与 save 结果；Java 侧
`KmoPresentation.updateProtocol` 仅是 `presentation.load(data)` + `docProperty.update`，本 function
只需断言被调用且 payload 来自 `Presentation.save()`）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 内存 attrs 覆盖全部 12 个 attr 名（serverZoom="50"/firstSlideNum="2"/conformance="strict" 等合法值），走 count+getAttrAt | 仅当轮 attr 名被写入且值正确（toDouble/toInt/toBoolean/toStr）；无越界写入 | 红测停在本 function |
| T02 | C02 | conformance="picky"（不在 {strict, transitional}） | `putConformace(1)`；不 panic | 红测停在本 function |
| T03 | C03 | count()==0 的 attrs + endElement | 无任何 put*；endElement 仍 `updateProtocol(save())`，create 默认保留 | 红测停在本 function |
| T04 | C04 | 仅 callback 构造（mPres 为 None），模拟 endElement | 不调用 updateProtocol | 红测停在本 function |
| T05 | C05 | 三参 ctor + endElement | `updateProtocol` 收到 `presData.save()` 的结果（非空） | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/handler/PresentationHandler.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "01_function_presentation_attrs"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_presentation_attrs

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_presentation_attrs \
  --src src/cn/wps/show/pptxr/handler/PresentationHandler.rs \
  --report libs/ppt-core/parser/features/pp-06-presentation-xml/functions/01_function_presentation_attrs/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_presentation_attrs --json`)
date: 2026-09-01 17:27

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/PresentationHandler.rs` | 110/283 | 38.9% | 156/335 | 46.6% |
| **function total** | **110/283** | **38.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java
```

Loop 结果：**PASS**

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs \
  --report libs/ppt-core/parser/features/pp-06-presentation-xml/functions/01_function_presentation_attrs/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 09:27

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PresentationHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs` | **PASS** | `PresentationHandler`, `startElement`, `endElement`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
