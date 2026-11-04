# 01_function_transition — p:transition

> feature: pp-05-timing-transition ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：`p:transition` 切换 attrs 与效果 mode 写入 `Transition` model，`AlternateContent` 的 Choice/Fallback 分发有定论，幻灯片切换元数据可消费。缺失时断在哪：切换效果不解析，但动画/切换不在 MVP 首屏关键路径，绘制不被阻塞（非关键路径）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

`TransitionHandler` 消费 `p:transition` 元素事件，把 attrs 与子元素 mode 写回 `Transition` model；`TransitionAlternateContentHandler` 处理 `mc:AlternateContent` 的 Choice/Fallback 分发（3211402→TransitionHandler、3211609→TimingHandler）。

### 1.1 I/O 契约

| 项 | 契约 |
|---|---|
| **输入** | `p:transition` 元素 attrs：3211297 `advClick`(toBoolean)、3211298 `advTm`(toInt)、3211296 `spd`(SWReaderMap.transitionSpeedMap)、3276828 `dur`(toInt)；子元素事件：3211273–3211295、3276802–3276827、3342340（全量 case 见 §2）；`mc:AlternateContent` 入口（2162692，由 pp-01 05/06/07 分发）：2162693 Choice、2162695 Fallback |
| **输出** | `Transition` 字段写回：putAdvClick / putAdvTm / putSpd / putDur / putMode / 各效果方向字段（见 §2）；子 handler 分发链返回值（`ElementHandler`） |
| **状态与副作用** | `Transition` 为就地可变累积（多次 put\* 叠加，无独立返回对象）；`TransitionHandler` 无 `endElement` 覆盖；接收 `HandlerCallback` 但不直接使用 attrs 之外的状态 |
| **错误语义** | 未知 attr／子元素 id → Java 忽略（`if (tmp != null)` 守卫 / `default: return null`）；未知 `spd`/`prst` 等 token → `SWReaderMap` 表查无 → Java 直接对 `Integer` 解包调用 primitive `putSpd(int)` 等（运行时 NPE 路径），Rust 按验收 §4-1「不写脏」对齐（具体 null 语义待 loop 确认）；`mc:Fallback`(2162695) 在 Java 中返回 null handler（子树跳过），与 FUNCTIONS.md §4-2 表述差异见下 |

> **查证记录（待 loop 确认）**：Java `TransitionAlternateContentHandler.enterSubElement` 中 `case 2162693`（Choice）构造匿名 handler 且**无 break**（fall-through 到 `default` 后 `return handler`，故 Choice 分支生效并分发 3211402/3211609）；`case 2162695`（Fallback）未赋值 handler → 返回 null（跳过子树）。FUNCTIONS.md §4-2「2162695 fallback 回到 TransitionHandler」与当前 Java 直接行为不一致，是否以金文件为准 loop 时确认。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| Transition model（`putAdvClick/putAdvTm/putSpd/putDur/putMode` 及方向字段） | data/transition | Rust `Transition` 等价 put\* 接口 | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |
| SWReaderMap transition 枚举表（transitionSpeedMap 等 10 张，见 §2） | pp-01:01 | Rust 常量表映射 | loop 前未接线 | 同上 | 同上 |
| HandlerCallback | pp-01:02 | `getCoreShapeId`（本 function 仅透传，TransitionHandler 自身不用） | loop 前未接线 | 同上 | 同上 |
| pp-01 05/06/07 分发点（3211402/2162692） | pp-01 | `handler::transition` 模块路径（Rust `SlideHandler.rs` 等已 `use`，模块未建） | loop 前未接线 | 同上 | 同上 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/handler/transition/TransitionHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/transition/TransitionHandler.rs` |
| `koffice/presentation/core/cn/wps/show/pptxr/handler/transition/TransitionAlternateContentHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/transition/TransitionAlternateContentHandler.rs` |

### 2.1 关键方法/字段表（Java 源码查证）

**`TransitionHandler`**

| 方法 | 元素/attr id | 行为 |
|---|---|---|
| `TransitionHandler(Transition, HandlerCallback)` | — | 存 `mTransition`、`callback` |
| `startElement(int, XmlRoAttr)` | attr 3211297 | `putAdvClick(tmp.toBoolean())`（advClick） |
| | attr 3211298 | `putAdvTm(tmp.toInt())`（advTm） |
| | attr 3211296 | `putSpd(SWReaderMap.transitionSpeedMap[token])`（spd：slow=0/med=1/fast=2） |
| | attr 3276828 | `putDur(tmp.toInt())`（dur） |
| `enterSubElement(int)` | 子元素（全量见下表） | mode 写回 / 子 handler 分发 |
| — | 无 `endElement` 覆盖 | — |

`enterSubElement` 分发（元素 id + mode 字面量全部来自 Java 源码）：

| 子元素 id | mode 写回 | 子 handler |
|---|---|---|
| 3211275 | putMode(27) | — |
| 3211276 | putMode(5) | — |
| 3211280 | putMode(17) | — |
| 3211282 | putMode(22) | — |
| 3211283 | putMode(18) | — |
| 3211286 | putMode(1) | — |
| 3211290 | putMode(19) | — |
| 3276806 | putMode(108) | — |
| 3276821 | putMode(103) | — |
| 3211273 / 3211274 / 3211277 / 3211287 | putMode(2/3/21/8) | OrientationTransitionHandler |
| 3211278 / 3211284 | putMode(4/7) | EightDirectionTransitionHandler |
| 3211279 / 3211281 | putMode(0/6) | OptinalBlackTransitionHandler |
| 3211285 / 3211292 | putMode(20/10) | SideDirectionTransitionHandler |
| 3211288 | putMode(13) | SplitTransitionHandler |
| 3211289 | putMode(9) | CornerDirectionTransitionHandler |
| 3211291 | putMode(26) | WheelTransitionHandler |
| 3211293 | putMode(11) | InOutTransitionHandler |
| 3211294 | — | TransitionSoundActionHandler(mTransition.soundAction(), callback) |
| 3211295 | — | ExtLstHandler(mTransition, callback) |
| 3276802 | putMode(115) | SideDirectionTransitionHandler |
| 3276803 / 3276804 | putMode(114/104) | LeftRightDirectionTransitionHandler |
| 3276805 | putMode(112) | RippleTransitionHandler |
| 3276809 | putMode(110) | PrismTransitionHandler |
| 3276810 / 3276811 | putMode(101/118) | OrientationTransitionHandler |
| 3276812 / 3276813 / 3276814 | putMode(102/106/100) | LeftRightDirectionTransitionHandler |
| 3276815 | putMode(109) | SideDirectionTransitionHandler |
| 3276817 | putMode(107) | GlitterTransitionHandler |
| 3276819 | putMode(116) | InOutTransitionHandler |
| 3276820 | putMode(105) | FlyThroughTransitionHandler |
| 3276822 | putMode(113) | ShredTransitionHandler |
| 3276824 | putMode(111) | RevealTransitionHandler |
| 3276827 | putMode(117) | WheelTransitionHandler |
| 3342340 | —（内嵌子类处理） | PresetTransitionHandler(mTransition) |

**16 个内嵌 per-effect handler（`startElement`/`enterSubElement` 全部来自 Java 源码）**

| 内嵌 handler | 触发元素 id | attrs（id：用途） | Transition 写回 |
|---|---|---|---|
| PresetTransitionHandler | 3342340 | 3342337 prst→presetTransitionMap（fallOver=201…origami=212，12 项）；3342338 invX(toBoolean)；3342339 invY(toBoolean) | putMode / putInvX / putInvY |
| ShredTransitionHandler | 3276822 | 默认 pattern=1、dir=1；3276816 pattern→transitionShredPatternMap（strip=1/rectangle=0）；3276801 dir→transitionInOutDirectionMap（out=0/in=1） | putShredPattern / putShredDirection |
| RippleTransitionHandler | 3276805 | 3276801 dir→transitionCornerAndCenterDirectionMap（center=0/lu=2/ru=4/rd=3/ld=1） | putRippleDirection |
| RevealTransitionHandler | 3276824 | 3276823 thruBlk(toBoolean)；3276801 dir→transitionLeftRightDirectionMap（l=0/r=1） | putRevealThruBlk / putRevealDirection |
| PrismTransitionHandler | 3276809 | 3276807 isContent(toBoolean)；3276808 isInverted(toBoolean)；3276801 dir→transitionSideDirectionMap（l=0/u=1/r=2/d=3） | putPrismIsContent / putPrismIsInverted / putPrismDirection |
| GlitterTransitionHandler | 3276817 | 默认 pattern=1、dir=0；3276816 pattern→transitionPatternMap（diamond=1/hexagon=0）；3276801 dir→transitionSideDirectionMap | putGlitterPattern / putGlitterDirection |
| FlyThroughTransitionHandler | 3276820 | 默认 dir=1；3276818 hasBounce(toBoolean)；3276801 dir→transitionInOutDirectionMap | putFlythroughDirection / putFlythroughHasBounce |
| LeftRightDirectionTransitionHandler | 3276803 / 3276804 / 3276812 / 3276813 / 3276814 | mode 114/104/102/106/100；3276801 dir→transitionLeftRightDirectionMap | putMode；dir 按元素 id switch：putSwitchDirection(3276803) / putFlipDirection(3276804) / putFerrisDirection(3276812) / putGalleryDirection(3276813) / putConveyorDirection(3276814) |
| OrientationTransitionHandler | 3211273 / 3211274 / 3211277 / 3211287 / 3276810 / 3276811 | mode 2/3/21/8/101/118；dir 3211265 或 3276801→DGMap.orientationMap（horz=0/vert=1） | putMode；putBlindsDirection / putCheckerDirection / putCombDirection / putRandomBarDirection / putDoorsDirection / putWindowDirection |
| EightDirectionTransitionHandler | 3211278 / 3211284 | mode 4/7；3211265 dir→transitionEightDirMap（l=0/u=1/r=2/d=3/lu=4/ru=5/rd=7/ld=6） | putCoverDirection / putPullDirection |
| CornerDirectionTransitionHandler | 3211289 | 3211265 dir→transitionCornerDirectionMap（lu=4/ru=5/rd=7/ld=6） | putStripsDirection |
| SideDirectionTransitionHandler | 3211285 / 3211292 / 3276802 / 3276815 | mode 20/10/115/109；dir 3211265 或 3276801→transitionSideDirectionMap | putMode；putPushDirection / putWipeDirection / putVortexDirection / putPanDirection |
| InOutTransitionHandler | 3211293 / 3276819 | mode 11/116；dir 3211265 或 3276801→transitionInOutDirectionMap | putZoomDirection / putWarpDirection |
| SplitTransitionHandler | 3211288 | 3211265 dir→transitionInOutDirectionMap；3211267 orient→DGMap.orientationMap | putSplitInOutDirection / putSplitOrientationDirection |
| WheelTransitionHandler | 3211291 / 3276827 | mode 26/117；3211291：3211268 spokes(toInt)→putWheelSpokes；3276827：3276882 revereSpokes(toInt)→putWheelRevereSpokes | putWheelSpokes / putWheelRevereSpokes |
| OptinalBlackTransitionHandler | 3211279 / 3211281 | mode 0/6；3211266 = element==3211279 → putCutOptionalBlackTransition(toInt)，else → putFadeOptionalBlackTransition(toBoolean) | putCutOptionalBlackTransition / putFadeOptionalBlackTransition |

**`TransitionAlternateContentHandler`**

| 方法 | 元素 id | 行为 |
|---|---|---|
| `TransitionAlternateContentHandler(Timing, Transition, HandlerCallback)` | — | 存 `timing`/`transition`/`callback` |
| `enterSubElement(int)` | 2162693（Choice） | 匿名 handler：3211402→TransitionHandler(transition, callback)；3211609→TimingHandler(timing, callback)。Java 无 break，fall-through 后 return handler |
| | 2162695（Fallback） | 未赋值 → 返回 null（跳过子树）——与 FUNCTIONS.md §4-2 表述出入，待 loop 确认 |
| | default | 返回 null |

### 2.2 函数改写列表

- `TransitionHandler`：ctor、startElement（4 attrs）、enterSubElement（9 个纯 mode case + 16 个内嵌 handler 分发）。
- 16 个内嵌 per-effect handler：随 `TransitionHandler.rs` 同文件实现（等价 Java 内嵌私有类）；PresetTransitionHandler 含 startElement；其余仅 startElement。
- `TransitionAlternateContentHandler`：ctor、enterSubElement（2162693/2162695）。

### 2.3 停车列表（非本 function 改写）

- `TransitionSoundActionHandler`（3211294 sndAc）、`ExtLstHandler`（3211295 extLst）：Java 中分别以 `transition.soundAction()` / `transition` 构造；本 MVP 停在分发电位。
- 动画行为树 21 类（`Animate*BehaviorHandler` 等，见 FUNCTIONS.md §0）：本 function 不引入。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读：

- `TransitionHandler.java`：startElement（4 attrs）、enterSubElement（表 2.1 全量 case）、16 个内嵌 handler 的 startElement（各 attrs 查表 + 默认值回填，如 Shred 默认 pattern=1/dir=1、Glitter 默认 pattern=1/dir=0、FlyThrough 默认 dir=1）。
- `TransitionAlternateContentHandler.java`：2162693 匿名分流（3211402/3211609）、2162695 返回 null 的 fall-through 语义。
- `SWReaderMap.java`（只读参照）：transitionSpeedMap、transitionSideDirectionMap、transitionCornerDirectionMap、transitionEightDirMap、transitionInOutDirectionMap、transitionLeftRightDirectionMap、transitionCornerAndCenterDirectionMap、transitionPatternMap、transitionShredPatternMap、presetTransitionMap；`DGMap.orientationMap`（horz=0/vert=1）。
- 当前 Rust 同路径（若无 Rust 则只读 Java）。

禁止只按标题冒烟。注意 `3276803/3276804` 在 LeftRightDirectionTransitionHandler 内**无 break**（3276804 落入 default 不设 Flip mode，Java 原样保留）。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | startElement 4 attrs（3211297/3211298/3211296/3276828） | 分支 | 是 | T01 |
| C02 | spd/prst token 查表（未知 token → 不写脏，验收 §4-1） | 边界 | 是 | T02 |
| C03 | 纯 mode 子元素（3211275/3211276/3211280/3211282/3211283/3211286/3211290/3276806/3276821） | 分支 | 是 | T03 |
| C04 | 16 个内嵌 per-effect handler：mode + dir 写回（抽查代表族：Orientation / LeftRightDirection / Wheel / OptinalBlack / Shred 默认值） | 分支 | 是 | T04–T07 |
| C05 | 3211294 sndAc / 3211295 extLst 分发点位（handler 为停车 fixture） | 分支 | 否 | — |
| C06 | AlternateContent：2162693 Choice 分流 3211402/3211609；2162695 Fallback | 分支 | 是 | T08 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。`Transition` 用内存 model fixture；`HandlerCallback` 用 pp-01:02 的假实现。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 最小内存输入覆盖 startElement 4 attrs（advClick=true、advTm、spd=med、dur） | Transition 字段与 Java 一致（spd 查表得 1） | 红测停在本 function |
| T02 | C02 | spd/prst 传入未知 token | 不写脏（Java 端为 NPE 路径，Rust 对齐验收 §4-1） | 红测停在本 function |
| T03 | C03 | 逐个纯 mode 子元素进入 enterSubElement | putMode 字面量与 Java 表一致（27/5/17/22/18/1/19/108/103） | 红测停在本 function |
| T04 | C04 | Orientation 族（3211273 blinds + dir=horz） | putMode(2)+putBlindsDirection(0) | 红测停在本 function |
| T05 | C04 | LeftRightDirection（3276804 flip + dir 3276801） | putMode(104)（无 Flip mode 覆盖）+putFlipDirection | 红测停在本 function |
| T06 | C04 | Wheel（3211291 + 3211268 spokes；3276827 + 3276882 revere） | putWheelSpokes/putWheelRevereSpokes 与 Java 一致 | 红测停在本 function |
| T07 | C04 | Shred（3276822，无 attrs） | 默认 pattern=1、dir=1 落库 | 红测停在本 function |
| T08 | C06 | 2162693 Choice 内含 3211402/3211609；2162695 Fallback 进入 | Choice 分流到 TransitionHandler/TimingHandler；Fallback 不产生 handler（对齐 Java，差异待 loop 确认） | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/handler/transition/TransitionHandler.rs
       src/cn/wps/show/pptxr/handler/transition/TransitionAlternateContentHandler.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "01_function_transition"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_transition

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_transition \
  --src src/cn/wps/show/pptxr/handler/transition/TransitionHandler.rs \
  --src src/cn/wps/show/pptxr/handler/transition/TransitionAlternateContentHandler.rs \
  --report libs/ppt-core/parser/features/pp-05-timing-transition/functions/01_function_transition/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_transition --json`)
date: 2026-09-01 22:17

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/transition/TransitionAlternateContentHandler.rs` | 29/44 | 65.9% | 42/65 | 64.6% |
| `src/cn/wps/show/pptxr/handler/transition/TransitionHandler.rs` | 180/890 | 20.2% | 199/1057 | 18.8% |
| **function total** | **209/934** | **22.4%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/transition/TransitionHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/transition/TransitionHandler.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/transition/TransitionAlternateContentHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/transition/TransitionAlternateContentHandler.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/transition/TransitionHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/transition/TransitionHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/transition/TransitionAlternateContentHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/transition/TransitionAlternateContentHandler.rs \
  --report libs/ppt-core/parser/features/pp-05-timing-transition/functions/01_function_transition/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 14:18

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `TransitionHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/transition/TransitionHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/transition/TransitionHandler.rs` | **PASS** | `TransitionHandler`, `startElement`, `enterSubElement` | — | — | **PASS** |
| `TransitionAlternateContentHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/transition/TransitionAlternateContentHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/transition/TransitionAlternateContentHandler.rs` | **PASS** | `TransitionAlternateContentHandler`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->