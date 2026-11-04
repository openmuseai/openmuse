# pp-05-timing-transition — p:transition / p:timing 列表

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（3 function：`01_function_transition` / `02_function_timing_lists` / `03_function_time_node_list`）
> 依赖：pp-01-pptx-parse

## 目标

pp-01 05/06/07（`SlideHandler` / `SlideLayoutHandler` / `SlideMasterHandler`）已有的三个分发落到真实 Java 等价 handler：

- 3211402 → `TransitionHandler`
- 2162692 → `TransitionAlternateContentHandler`
- 3211609 → `TimingHandler`

（Rust 侧 `SlideHandler.rs`/`SlideLayoutHandler.rs`/`SlideMasterHandler.rs` 已写出上述 `use crate::…::transition::…` / `handler::timing::TimingHandler` 引用，但模块尚未建立——本 feature loop 产出后可直接接线。）

**MVP 锁定 transition 属性与 timing 列表容器，不写动画行为树**：`Animate*BehaviorHandler` 等 20+ 行为类移出本 feature（打开文档不需要完整动画树）。

## I/O（锁死）

| function | I | O |
|---|---|---|
| `01_function_transition` | `p:transition` 元素 attrs（3211297 advClick / 3211298 advTm / 3211296 spd / 3276828 dur，SWReaderMap transition 表）+ 子元素分发（3211273–3211295 / 3276802–3276827 / 3342340）+ `mc:AlternateContent` Choice(2162693) vs Fallback(2162695) | `Transition` 字段（advClick / advTm / spd / dur / mode / 各方向字段）+ 子 handler 分发链 |
| `02_function_timing_lists` | `p:timing` 子元素 3211422(tnLst) / 3211437(bldLst) | `TimeNodeListHandler` / `BuildListHandler` 实例化；`TimingHandler.endElement` 时逐项 `save()` + `timing.timeNodeLst().write()` / `timing.buildLst().write()` |
| `03_function_time_node_list` | tnLst 子节点事件（3211323 par / 3211324 seq / 3211325 excl / 3211326–3211335 anim 族 / 3211295 extLst） | 收集 `TimeNode` 列表（`TimeNode.create()` + `putType` 字面量 + add 进调用方传入的 `List<TimeNode>`）；seq/par/excl 用 constructor fixture |

## Java 对照

| 子模块 | Java | 归属 function |
|---|---|---|
| transition | `TransitionHandler.java`（含 16 个内嵌 per-effect handler） | 01 |
| transition | `TransitionAlternateContentHandler.java` | 01（嵌套分发 3211402→TransitionHandler、3211609→TimingHandler） |
| timing | `TimingHandler.java` | 02 |
| timing | `BuildListHandler.java`（容器层） | 02 |
| timing | `TimeNodeListHandler.java` | 03 |
| timing | `AnimateBehaviorHandler` / `AnimateColorBehaviorHandler` / `AnimateEffectBehaviorHandler` / `AnimateMotionBehaviorHandler` / `AnimateRotationBehaviorHandler` / `AnimateScaleBehaviorHandler` / `CommandBehaviorHandler` / `SetBehaviorHandler` / `MediaNodeAudioHandler` / `MediaNodeVideoHandler` 及支撑类（`CommonBehaviorDataHandler`、`CommonMediaNodeDataHandler`、`CommonTimeNodeDataHandler`、`TimeConditionHandler`、`TimeConditionListHandler`、`TimeTargetElementHandler`、`ShapeTargetElementHandler`、`IteraterDataHandler`、`AnimVariantHandler`、`TimeAnimateValueListHandler`、`TLPointHandler`，共 21 类） | 移出（post-MVP 动画树） |

## 必须 / 壳 / 延后

- **must：** `p:transition` attrs（spd 走 `SWReaderMap.transitionSpeedMap` slow=0/med=1/fast=2、advClick/advTm/dur）+ 子元素 mode 写回 + `mc:AlternateContent` Choice(2162693)/Fallback(2162695) 分发语义；`p:timing` 的 tnLst/bldLst 容器；`TimingHandler.endElement` 的 save+write（null 守卫）。
- **stub：** `TimeNodeParallelHandler` / `TimeNodeSequenceHandler` / `TimeNodeExclusiveHandler`（03 内 constructor fixture）；子分发碰到停车类时以 fixture/`todo!` 承接。
- **defer：** transition 的 sndAc `TransitionSoundActionHandler`、`ExtLstHandler`；`BuildListHandler` 内嵌 build 项细节解析（`BuildParagraphHandler` 等 9 类）；动画行为树 21 类。

## 范围内 / 范围外

- **in：** slide / slideLayout / slideMaster 三处的 3211402/2162692/3211609 分发落地（pp-01 05/06/07 接线处已存在引用）；transition 属性全量；timing 列表容器与 endElement 写回。
- **out：** 动画行为树（Animate\*）、build 项细节、slide part 读取（本 feature 只提供 handler，不跑完整 PptxReader）。

## 验收

FUNCTIONS.md §4 三条：

1. 未知 transition token 不写脏。
2. AlternateContent：2162695 fallback 回到 TransitionHandler（对齐 ControlList 的 Java fall-through 风格）。
3. TimingHandler：无子元素 end 不 write null 列表；有列表则 save。

> 注：第 2 条与当前 Java `TransitionAlternateContentHandler.enterSubElement` 直接行为存在出入（Java 源码中 2162695 返回 null handler，详见 `01_function_transition/FUNCTION.md` §1.1/§2），是否以金文件为准待 loop 确认。

## 依赖

- **depends：** pp-01-pptx-parse（3211402/2162692/3211609 分发点所在 05/06/07；`SWReaderMap` transition 枚举表；`HandlerCallback`）；Transition / Timing / TimeNode / Build 数据模型。
- **does not depend：** pp-07-part-open；pp-09-pptx-open；动画行为树 loop。

## 不过扩展

禁止实现 `Animate*BehaviorHandler` 等 21 个行为类（见 Java 对照表）来冒充本 feature；禁止运行 slide part（本 feature 只交付 handler 与模型写回）。