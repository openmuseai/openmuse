# 02_function_text_body_props — a:bodyPr 属性

> feature: pp-11-dml-txbody ｜ 状态：**pass** ｜ Java 管线环节：S0 parse → S1 convert 输入

## 0. 管线能力解锁

有段落/run（01）但没有**文本区几何依据**时，pl-05 `renderModel.getTextRect` 无 insets/anchor →
排的字 x/y 不可算 → pr-03 `DrawText` 落点错误。实现后 `TextBodyProperties` 有
insets/anchor/vert/autofit → 文本区内缩与对齐可算 → 文本画的位置正确；缺失则链断在
「文本区几何无依据」。

## 1. 功能说明

`BodyPropertiesHandler`（Java 类名 `BodyPropertiesHandler`，即计划表/PIPELINE 文档中的
`TextBodyPropertiesHandler`；写入 `data/text/TextBodyProperties.java`）处理 `a:bodyPr`（
`TextBodyHandler.enterSubElement` 的 1114448 分支交 01 分发入）：

- `startElement`：count+getAttrAt 循环 20 个 attr case——rot=1114201、anchor=1114301
  （`DGMap.textAnchoringTypeMap`）、vert=1114480（`textVerticalTypeMap`）、anchorCtr=1114481、
  horzOverflow=1114482、spcFirstLastPara=1114548、vertOverflow=1114549、wrap=1114550、
  lIns=1114551、tIns=1114552、rIns=1114553、bIns=1114554、numCol=1114555、spcCol=1114556、
  rtlCol=1114557、fromWordArt=1114558、forceAA=1114559、upright=1114560、compatLnSpc=1114561。
- `enterSubElement`：flatTx=1114313（值 z=1114206，缺省写 0）、prstTxWarp=1114430（延后）、
  noAutofit=1114545→`putType(0)`、normAutofit=1114546→`putType(1)`+`NormalAutoFitHandler`
  （fontScale=1114543、lnSpcReduction=1114544，`toPercentage`）、spAutoFit=1114547→`putType(2)`、
  scene3d=1114144/sp3d=1114145/extLst=3211295（延后/返回 null）。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `a:bodyPr` attrs（count+getAttrAt 循环）+ 子元素事件 + 构造入参 `TextBodyProperties bodyProperties` + `HandlerCallback`。 |
| **输出** | 20 个 attr 槽写入（含 insets 四值、anchor/vert/wrap、overflow 两向）；`textAutofit().putType(0/1/2)` + normAutofit 两 `toPercentage`；flatTx 的 0 缺省。 |
| **状态与副作用** | `startElement` 逐 attr null skip；map miss → null 不 put。autofit type 与子元素一一对应（后写覆盖）。`NormalAutoFitHandler` 仅 normAutofit 分支实例化。 |
| **错误语义** | 未知 attr → switch default 无副作用（与 Java 一致）。`flatTx` 缺 z attr → `putFlatTx(0)`（Java else 分支）。3D/extLst → null。`toPercentage`/`toInt` 非法串语义以 kfc `XmlRoAttr` 为准（待 loop 确认）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止去改 |
|---|---|---|---|---|---|
| 入口分发 | pp-11:01 | `TextBodyHandler.enterSubElement` 的 bodyPr=1114448 → 本 function handler | spec（随 01 落地） | — | 不改 01 分发 |
| `TextBodyProperties` 模型 | `data/text/TextBodyProperties.rs` | `put*` 20 槽 + `textAutofit()` + `putFlatTx` | **已存在且完整**（3209 行） | — | 不改字段语义 |
| `HandlerCallback` | pp-01:02 | 透传（本 function 实际仅透传） | pass | — | pass 语义 |
| `DGMap` 文本 map | util | `textAnchoringTypeMap`/`textVerticalTypeMap`/`textHorzOverflowTypeMap`/`textVerOverflowMap`/`textWrapMap` | 审计 delete 空壳 | 测试字面量/最小常量 | 不整表恢复 |
| handler `.rs` | 本 feature | Java 结构模板 | **新建**（`BodyPropertiesHandler.rs` 无历史 stub 记录；旧 160 行 TextBodyHandler stub 中 bodyPr 仅 todo） | — | — |

### 1.8 不实现（显式）

numCol/spcCol/rtlCol 多栏业务体（字段可写，layout 不消费）、prstTxWarp、3D（scene3d/sp3d）、
`PresetTextWrapHandler` 的 adjList——MVP 不做多栏与文本变形。

## 2. 核心代码与函数改写列表

| Java 类 | Rust 目标 | 关键方法/字段 |
|---|---|---|
| `BodyPropertiesHandler` | `…/textshape/text/BodyPropertiesHandler.rs`（新建） | 字段 `bodyProperties`、`handlerCallback`；`ctor(TextBodyProperties, HandlerCallback)`、`startElement(int, XmlRoAttr)`（20 case）、`enterSubElement(int)` |
| nested `NormalAutoFitHandler` | 同文件 nested | fontScale/lnSpcReduction `toPercentage` |

id 已查证（`dml/A.java`）：bodyPr=1114448、rot=1114201、anchor=1114301、vert=1114480、
anchorCtr=1114481、horzOverflow=1114482、spcFirstLastPara=1114548、vertOverflow=1114549、
wrap=1114550、lIns=1114551、tIns=1114552、rIns=1114553、bIns=1114554、numCol=1114555、
spcCol=1114556、rtlCol=1114557、fromWordArt=1114558、forceAA=1114559、upright=1114560、
compatLnSpc=1114561、flatTx=1114313、prstTxWarp=1114430、noAutofit=1114545、normAutofit=1114546、
spAutoFit=1114547、fontScale=1114543、lnSpcReduction=1114544、z=1114206、scene3d=1114144、
sp3d=1114145、extLst=3211295。

函数改写列表：

- `startElement`：count+getAttrAt 循环 + 20 case 数值与 Java 一致（每 case 走 `DGMap` 的 5 个
  文本 map；字符串槽 putLang 类无 map）。
- `enterSubElement`：autofit 三型 putType（0/1/2）；normAutofit 实例化；flatTx 的 z 缺省 0；
  prstTxWarp/3D/extLst 返回 None 并注释延后。
- `todo!("DEFERRED_NON_FUNCTION")` 停车项：prstTxWarp、3D、多栏业务体。

## 3. 测试方案

### 3.1 代码审计与测试面

- `BodyPropertiesHandler.java` 全部 attr case 数值与 5 个 DGMap 文本 map 的 key 定义。
- `data/text/TextBodyProperties.java` 的 put* 与 autofit 结构；对照 Rust `data/text/TextBodyProperties.rs`。
- 确认 Rust `data/text/TextBodyProperties.rs` 既有 put* 签名（循环前核对）。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | insets 四值 + rot + anchorCtr 等基本槽 | 数据 | 是 | T01 |
| C02 | anchor/vert/wrap/overflow map 值与 miss | 分支 | 是 | T02 |
| C03 | autofit 三型 type + normAutofit 两 percentage | 分支 | 是 | T03 |
| C04 | flatTx：含 z / 缺 z | 状态 | 是 | T04 |
| C05 | 未知 attr / 3D / prstTxWarp → 无副作用 | 失败/边界 | 是 | T05 |

### 3.3 可测试性

内存 attrs（count+getAttrAt fixture）；真实 `data/text/TextBodyProperties.rs` 实例断言。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | lIns/tIns/rIns/bIns=91440 + rot=60000 | 四 inset + rot 写入 | 红测停在本 function |
| T02 | C02 | anchor=“t”、vert=“eaVert”、wrap=“none”；另给非法值 | map 命中写入；miss 不写 | 红测停在本 function |
| T03 | C03 | normAutofit(fontScale=“100000”, lnSpcReduction=“20000”)；spAutoFit；noAutofit | type 1+两值 / 2 / 0 各自正确 | 红测停在本 function |
| T04 | C04 | flatTx 带 z=1；flatTx 空 attrs | putFlatTx(1)；缺 z → 0 | 红测停在本 function |
| T05 | C05 | bodyPr 内 scene3d/prstTxWarp/未知 attr | 不写坏既有槽；返回 None | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.rs
test:  libs/ppt-core/tests/cn/wps/drawing/dmlr/handler/shape/textshape/text/02_function_text_body_props.rs
[[test]] name = "02_function_text_body_props"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_text_body_props

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_text_body_props \
  --src src/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.rs \
  --report libs/ppt-core/parser/features/pp-11-dml-txbody/functions/02_function_text_body_props/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_text_body_props --json`)
date: 2026-09-01 19:47

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.rs` | 91/159 | 57.2% | 122/211 | 57.8% |
| **function total** | **91/159** | **57.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.rs \
  --report libs/ppt-core/parser/features/pp-11-dml-txbody/functions/02_function_text_body_props/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 11:47

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `BodyPropertiesHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.rs` | **PASS** | `BodyPropertiesHandler`, `startElement`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->