# 02_function_fill — a:fill 子树

> feature: pp-10-dml-sppr ｜ 状态：**pass** ｜ Java 管线环节：S0 parse → S1 convert 输入

## 0. 管线能力解锁

形状有坐标但**无填充**时，S1 convert 的 `FillConvert` 无输入 → pr-02 无 fill 指令 → 形状画成空心
（MVP 语义下连空心都不成立，`Fill` 默认类型 m 不可画）。实现后形状可带**实色/渐变/图片**填 →
S4 能画出形状体；缺失则整条链断在「形状无填充」。

## 1. 功能说明

`FillPropertiesHandler` 是 spPr（ShapePropertiesHandler）、ln（OutlineHandler）、run rPr
（pp-11 TextCharacterPropertiesHandler）、group（GroupShapePropertiesHandler）共用的 fill 分发
入口：`enterSubElement` 依子元素 id 置 `Fill.fillType` 并实例化子 handler——

| 子元素 id | fillType | 子 handler | 最小集 | 延后 |
|---|---|---|---|---|
| noFill 1114370 | 5 | — | ✓（仅 putType） | — |
| solidFill 1114371 | 0 | `SolidFillHandler`（→ `ColorHandlerFactory.createColorHandler(sub, color, cb, 1114371)`） | ✓ | — |
| gradFill 1114372 | 4 | `GradientFillHandler`（rotWithShape=1114329、flip=1114346；shade lin/path；gsLst=1114344→gs=1114343→pos=1114342+color；endElement 按 pos 排序后 write） | 双停靠线渐变 | 排序/停靠全表 |
| pattFill 1114373 | 1 | `PatternFillHandler`（prst=1114293→presetPatternMap；fgClr=1114368、bgClr=1114369） | prst id | fg/bg 色 |
| grpFill 1114374 | 3 | —（回退主题/组填充，MVP 占位） | ✓（仅 putType） | — |
| blipFill 3211586 | 2 | `BlipFillHandler`（rotWithShape=1114329、dpi=1114367；blip=1114365→`BlipHandler`、srcRect=1114366、tile=1114348、stretch=1114349） | rId 回调 + srcRect | tile/stretch 细节、blip 效果 |

`BlipHandler`：cstate=1114364→`blipCompressionMap`；`r:embed`=4259842→`callback.onBlipEmbed`、
`r:link`=4259843→`callback.onBlipLink`（图片 bit 流解析 post-MVP，第一波占位矩形）。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | fill 六类子元素事件（element id）+ 子 handler 的 attrs/子事件 + 构造入参 `Fill fill` + `HandlerCallback`。入口可能是 spPr/ln/rPr/grpSpPr 任一处（共享同一 `FillPropertiesHandler` 模板）。 |
| **输出** | `Fill.putType(0..5)`；solid → `Color`（经 `ColorHandlerFactory` 最小色链）；grad → `GradientFill`（rotWithShape/flip + gsLst 最小条数）；blip → `BlipFill`（cstate + rId 回调 + srcRect）；patt → `PatternFill.putPrst`。 |
| **状态与副作用** | `FillPropertiesHandler` 自身无 `startElement`（只有分发）；type 与子 handler 一一对应，同一次 spPr 内**后出现的 fill 元素覆盖前一个 type**（Java 语义：同一 `Fill` 实例 putType 覆盖）。`BlipFillHandler.endElement`：srcRect/tile/stretch 存在才 save+`forceWrite*`。`GradientFillHandler.endElement`：gsLst 按 `pos()` 升序 `Collections.sort` 后 write（Rust 等价的排序必须复刻）。 |
| **错误语义** | `DGMap`（rectAlignmentMap/tileFlipMap/pathShadeTypeMap/presetPatternMap/blipCompressionMap）miss → null 不 put。`BlipHandler` 缺 `r:embed`/`r:link` 且 callback 非空 → 不回调（Java `null != callback` 判定）；两属性都缺 → blip 无 rId，图片按占位矩形语义。`toPercentage`/`toInt` 的非法串行为以 kfc `XmlRoAttr` 语义为准（待 loop 确认）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止去改 |
|---|---|---|---|---|---|
| `HandlerCallback` | pp-01:02 | `onBlipEmbed`/`onBlipLink`（rId 回调）+ 透传 | pass | FileManager/media stub | **pass 语义**：回调签名不重排 |
| fill 子事件入口 | pp-10:01 | spPr/grpSpPr 的 fill 分发返回本功能 handler | **pass**（01 已分发 fill id） | ln/rPr 入口由 03/pp-11 各自接线 | 不在本 function 反侧改 01 分发 |
| `Fill`/`BlipFill`/`GradientFill`/`PatternFill` 族 | data 模型 | `putType`、`putRich*`、`blip()`、`fillMode()`、`gsLst().write` | `data/fill/*.rs` 均在；默认编译面现用 `layout_shape::FillObj` | 全量 KoBin Fill | 不改模型字段语义 |
| 色链 `ColorHandlerFactory`/`SchemeColorHandler` | `handler/color` | `createColorHandler`：srgbClr=1114216 **与** schemeClr=1114219（belonger=1114371） | 仅 srgbClr 写入 `FillObj.solid` | hsl/sys/prst/scrgb；theme 槽 RGB 回填归 pp-03:03 / pp-04 | 不在此展开全色表；scheme **token 写入**本 function 必测 |
| sppr/fill 族 handler `.rs` | 本 feature | Java 结构模板 | **pass（2026-09-01）：** 六 id putType；solid→srgb/scheme；grad 双停靠排序；blip rId 回调 + srcRect；patt prst | 效果链、tile/stretch 全量、pathShade | 不在 NamedHandler 上假装 pass |

### 1.8 不实现（显式）

blip 的 14 种效果 handler（tint/alpha/duotone…，`BlipHandler.enterSubElement` 1114165–1114363）、
`GradientFillHandler` 的 pathShade 子类、tile/stretch 全量——MVP 首屏不做，fixture/占位矩形覆盖。

## 2. 核心代码与函数改写列表

| Java 类 | Rust 目标 | 关键方法/字段 |
|---|---|---|
| `FillPropertiesHandler` | `…/sppr/fill/FillPropertiesHandler.rs` | `ctor(Fill, HandlerCallback)`、`enterSubElement(int)`（六 id → putType + 子 handler） |
| `SolidFillHandler` | `…/sppr/fill/SolidFillHandler.rs` | `ctor(Color, HandlerCallback)`、`enterSubElement` → `createColorHandler(sub, color, cb, 1114371)` |
| `GradientFillHandler` | `…/sppr/fill/GradientFillHandler.rs` | `ctor(GradientFill, HandlerCallback)`、`startElement`（rotWithShape/flip）、`enterSubElement`（gsLst/lin/tileRect）、`endElement`（sort+write） |
| `PatternFillHandler` | `…/sppr/fill/PatternFillHandler.rs` | `ctor(PatternFill, HandlerCallback)`、`startElement`（prst）、`enterSubElement`（fgClr/bgClr） |
| `BlipFillHandler` + nested `TileHandler`/`StretchHandler` | `…/sppr/fill/BlipFillHandler.rs` | `ctor(BlipFill, HandlerCallback)`、`startElement`（rotWithShape/dpi）、`enterSubElement`（tile/stretch/blip/srcRect）、`endElement`（forceWrite） |
| `BlipHandler` | `…/sppr/fill/BlipHandler.rs` | `startElement`（cstate、r:embed/r:link → 回调）、`enterSubElement`（效果分支，MVP 不实现返回 None） |
| `RelativeRectHandler` | `…/sppr/fill/RelativeRectHandler.rs` | srcRect/tileRect/fillRect 共用（l/t/r/b 双命名空间 id） |

id 已查证（`dml/A.java`、`shared/R.java`）：noFill=1114370、solidFill=1114371、gradFill=1114372、
pattFill=1114373、grpFill=1114374、blipFill=3211586、rotWithShape=1114329、dpi=1114367、blip=1114365、
srcRect=1114366、tile=1114348、stretch=1114349、gsLst=1114344、gs=1114343、pos=1114342、
flip=1114346、prst=1114293、fgClr=1114368、bgClr=1114369、cstate=1114364、embed=4259842、
link=4259843。

函数改写列表：

- `FillPropertiesHandler.enterSubElement`：六分支 putType 数值与 Java 完全一致。
- `SolidFillHandler`：`ColorHandlerFactory.createColorHandler(sub, color, cb, **1114371**)`——srgbClr=1114216 与 **schemeClr=1114219** 均必须返回非 None（Sample_ppt 标题/形状大量 schemeClr，只接 srgb 则主题色全丢）。
- `BlipFillHandler`/`BlipHandler`：endElement forceWrite 语义、`r:embed`=4259842 / `r:link`=4259843 回调判空语义精确复刻；效果分支 `enterSubElement` 返回 None 并注释延后。
- `GradientFillHandler`：`endElement` 的 pos 升序排序用 Rust 实现（`sort_by` 对标
  `Collections.sort` 的稳定比较），泊位最小双停靠先过。
- `BlipFillHandler`/`BlipHandler`：endElement forceWrite 语义、回调判空语义精确复刻；效果分支
  `enterSubElement` 返回 None 并注释延后。
- `todo!("DEFERRED_NON_FUNCTION")` 停车项：blip 效果链、tile/stretch 全量、pathShade——均标注
  MVP 延后理由。

## 3. 测试方案

### 3.1 代码审计与测试面

- `FillPropertiesHandler.java` 六个 case 的 putType 数值与返回值。
- `SolidFillHandler.java`/`GradientFillHandler.java`/`PatternFillHandler.java`/`BlipFillHandler.java`/
  `BlipHandler.java` 的 attr/元素 id（与本节 §2 表核对）。
- `GradientFill.java` 的 `GradientStop`/`Shade` 结构与排序比较器；对照 Rust `data/fill/GradientFill.rs`。
- Rust 现状：`sppr/fill/mod.rs` 只有 `// pub mod FillPropertiesHandler;` 注释；旧 stub 不增量。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 六 id → type 0-5 + handler 返回 | 分支 | 是 | T01 |
| C02 | solidFill → 最小色链写入（srgbClr 上下文 1114371） | 数据 | 是 | T02 |
| C03 | gradFill：双停靠 pos 排序 + rotWithShape/flip | 状态/排序 | 是 | T03 |
| C04 | blipFill：cstate + r:embed/r:link 回调 + srcRect 写入 | 回调节点 | 是 | T04 |
| C05 | pattFill prst 写入；grpFill/noFill 仅 type | 数据 | 是 | T05 |
| C06 | solidFill → schemeClr(1114219) val→`DGMap.schemeColorMap` + `putScheme` + `onAdjustThemeColor(color, 1114371)` | 分支 | 是 | T06 |

### 3.3 可测试性

内存 attrs/events；真实 `data/fill/*` 模型断言；blip rId 用假 callback fixture 验证调用参数。
不打开 reader/part、不接 media。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 六个 fill 子元素各一条 | type 依次 5/0/4/1/3/2；handler 返回匹配 | 红测停在本 function |
| T02 | C02 | solidFill + srgbClr(1114216) fixture | `Fill.solidFill()` 颜色值与 Java 一致 | 红测停在本 function |
| T03 | C03 | 两个 gs（pos 50/10 + color）+ lin attrs | gsLst 写序 10→50；rotWithShape/flip 写入 | 红测停在本 function |
| T04 | C04 | blipFill + blip(r:embed=“rId5”) + srcRect | `onBlipEmbed("rId5")` 恰好一次；srcRect forceWrite | 红测停在本 function |
| T05 | C05 | pattFill prst；grpFill；noFill | prst id 写入；grp/no 仅 type，无子 handler 副作用 | 红测停在本 function |
| T06 | C06 | solidFill + schemeClr val=`accent1` | color.mode=4；scheme 索引 = schemeColorMap.get("accent1")；callback.onAdjustThemeColor 恰好一次 belonger=1114371 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/FillPropertiesHandler.rs
       src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipFillHandler.rs（loop 新建/改写）
       src/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.rs（scheme 分发）
test:  libs/ppt-core/tests/cn/wps/drawing/dmlr/handler/shape/sppr/fill/FillPropertiesHandler.rs
[[test]] name = "02_function_fill"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_fill

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_fill \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/FillPropertiesHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipFillHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/SolidFillHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/GradientFillHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/PatternFillHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/RelativeRectHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.rs \
  --src src/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/color/RgbColorModelHexHandler.rs \
  --report libs/ppt-core/parser/features/pp-10-dml-sppr/functions/02_function_fill/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_fill --json`)
date: 2026-09-01 11:25

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.rs` | 18/23 | 78.3% | 11/16 | 68.8% |
| `src/cn/wps/drawing/dmlr/handler/color/RgbColorModelHexHandler.rs` | 24/33 | 72.7% | 32/42 | 76.2% |
| `src/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs` | 33/42 | 78.6% | 37/50 | 74.0% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipFillHandler.rs` | 29/73 | 39.7% | 34/94 | 36.2% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipHandler.rs` | 17/34 | 50.0% | 25/50 | 50.0% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/FillPropertiesHandler.rs` | 42/48 | 87.5% | 37/45 | 82.2% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/GradientFillHandler.rs` | 117/172 | 68.0% | 140/223 | 62.8% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/PatternFillHandler.rs` | 16/24 | 66.7% | 17/27 | 63.0% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/RelativeRectHandler.rs` | 21/28 | 75.0% | 21/33 | 63.6% |
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/SolidFillHandler.rs` | 17/19 | 89.5% | 19/23 | 82.6% |
| **function total** | **334/496** | **67.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/FillPropertiesHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/FillPropertiesHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/SolidFillHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/SolidFillHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/GradientFillHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/GradientFillHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/PatternFillHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/PatternFillHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipFillHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipFillHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/RelativeRectHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/RelativeRectHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/RgbColorModelHexHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/RgbColorModelHexHandler.java
```

Loop 结果：PASS（2026-09-01）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/FillPropertiesHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/FillPropertiesHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/SolidFillHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/SolidFillHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/GradientFillHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/GradientFillHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/PatternFillHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/PatternFillHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipFillHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipFillHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/RelativeRectHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/RelativeRectHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/RgbColorModelHexHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/RgbColorModelHexHandler.rs \
  --report libs/ppt-core/parser/features/pp-10-dml-sppr/functions/02_function_fill/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 03:25

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `FillPropertiesHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/FillPropertiesHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/FillPropertiesHandler.rs` | **PASS** | `FillPropertiesHandler`, `enterSubElement` | — | — | **PASS** |
| `SolidFillHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/SolidFillHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/SolidFillHandler.rs` | **PASS** | `SolidFillHandler`, `enterSubElement` | — | — | **PASS** |
| `GradientFillHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/GradientFillHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/GradientFillHandler.rs` | **PASS** | `GradientFillHandler`, `endElement`, `startElement`, `enterSubElement` | — | — | **PASS** |
| `MyCompare` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/GradientFillHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/GradientFillHandler.rs` | **PASS** | `compare` | — | — | **PASS** |
| `PatternFillHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/PatternFillHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/PatternFillHandler.rs` | **PASS** | `PatternFillHandler`, `startElement`, `enterSubElement` | — | — | **PASS** |
| `BlipFillHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipFillHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipFillHandler.rs` | **PASS** | `BlipFillHandler`, `enterSubElement`, `startElement`, `endElement` | — | — | **PASS** |
| `BlipHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/BlipHandler.rs` | **PASS** | `BlipHandler`, `endElement`, `startElement`, `enterSubElement` | — | — | **PASS** |
| `RelativeRectHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/fill/RelativeRectHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/fill/RelativeRectHandler.rs` | **PASS** | `RelativeRectHandler`, `startElement` | — | — | **PASS** |
| `ColorHandlerFactory` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.rs` | **PASS** | `createColorHandler` | — | — | **PASS** |
| `SchemeColorHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs` | **PASS** | `SchemeColorHandler`, `addElementAttr` | — | — | **PASS** |
| `RgbColorModelHexHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/RgbColorModelHexHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/RgbColorModelHexHandler.rs` | **PASS** | `RgbColorModelHexHandler`, `addElementAttr` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->