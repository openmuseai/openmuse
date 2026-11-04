# pp-05-timing-transition — p:transition / p:timing 列表

> depends_on: `pp-01-pptx-parse`  
> 状态：**pass**

## §0 管线位置

```text
pp-01 SlideHandler / SlideLayoutHandler / SlideMasterHandler
  3211402 → TransitionHandler
  2162692 → TransitionAlternateContentHandler
  3211609 → TimingHandler
       ↓ 本 feature
transition attrs → Slide.transition
timing 子树 → Timing.timeNodeList / buildList 在 endElement write
```

**MVP 边界：** 锁定 transition 属性与 timing **列表容器**。`Animate*BehaviorHandler` 等 20+ 行为类移出本 feature（打开文档不需要完整动画树；pp-01 可用真实 TimingHandler 替代 fixture）。

## §1 function 清单

| order | function_id | I | O | upstream | downstream |
|---|---|---|---|---|---|
| 1 | `01_function_transition` | transition 元素 attrs + AlternateContent | `Transition` 字段 + Choice/Fallback 分发 | SWReaderMap transition 表 | pp-01 05–07 |
| 2 | `02_function_timing_lists` | timing 子 id 3211422/3211437 | TimeNodeList/BuildList handler；end 时 write | HandlerCallback | pp-01 05–07 |
| 3 | `03_function_time_node_list` | tnLst 子节点 | 收集 `TimeNode` 列表 | 02 | 动画树 post-MVP |

`03` 只覆盖 `TimeNodeListHandler` 的 enter/end 契约；内部 seq/par/excl 可用 constructor fixture，直到单独 loop。

## §2 数据流

```text
p:transition attrs ── SWReaderMap ──> spd/advClick/...
mc:AlternateContent ── Choice vs Fallback ──> TransitionHandler
p:timing → tnLst/bldLst → list.save/write on TimingHandler.endElement
```

## §3 测试面

- src：`show/pptxr/handler/transition/*.rs`；`handler/timing/{TimingHandler,TimeNodeListHandler,BuildListHandler}.rs`
- 不跑 slide part

## §4 验收

1. 未知 transition token 不写脏。
2. AlternateContent：2162695 fallback 回到 TransitionHandler（对齐 ControlList 的 Java fall-through 风格）。
3. TimingHandler：无子元素 end 不 write null 列表；有列表则 save。

## §5 loop

`01 → 02 → 03`

## 文档

- [functions.csv](./functions.csv)
