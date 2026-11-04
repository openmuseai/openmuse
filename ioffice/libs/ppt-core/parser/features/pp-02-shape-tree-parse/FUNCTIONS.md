# pp-02-shape-tree-parse — Slide shape tree 解析

> depends_on: `pp-01-pptx-parse`
>
> 输入是 XML reader 已产生的 shape-tree element/attribute 事件和合法的 model/store fixture；
> 输出是提交到 `KmoShowShape`/`DgStore` 的有序 shape tree。本 feature 不打开 ZIP/OPC，
> 不装配 slide/master/layout，也不渲染。

## §0 管线位置

```text
pp-01: token maps + callback policy
  -> pp-02: p:spTree events
       -> non-visual properties
       -> leaf shape/picture/connector committers
       -> recursive group + AlternateContent orchestration
       -> ordered KmoShowShape tree + DgStore rows
  -> pp-07 part-open（PptxrSlide 消费本层 root tree）
```

## §1 function 清单与依赖序

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_nonvisual_shape_props` | `p:nvPr` attrs/sub-events + `ShowShape` | flags、media mode、placeholder、custData 写入 | pp-01 01/02 | 02;03;04;05 | pass |
| 2 | `02_function_auto_shape` | `p:sp` events + preallocated `KmoShowShape` | protocol 更新 + `DgStore[id]` shape row | 01 | 05 | pass |
| 3 | `03_function_picture_shape` | `p:pic` events + preallocated picture | picture row、blip/style、外链 share id | 01 | 05 | pass |
| 4 | `04_function_connector_shape` | `p:cxnSp` events + preallocated connector | connector row + `onConnectorShape(id)` | 01 | 05 | pass |
| 5 | `05_function_shape_tree_dispatch` | `p:spTree`/`mc:AlternateContent` events | 有序递归 child tree、fallback/ink 选择、无效 graphic cleanup | 01;02;03;04 | pp-07 | pass |

`02/03/04` 是并行 leaf；05 是唯一编排 function。嵌套 group 通过 05 自递归，不把递归边写成
CSV 环。`SwShapeHandler.ShapeExtLstHandler` 随 02 验收，因为它与 auto shape 共用一个 Java/Rust
模板，同时被 picture/group 作为已声明输出能力复用。

## §2 数据流

```text
XmlRoAttr / element id
  ├─ nvPr ───────────────> ShowShape metadata
  ├─ sp ─────────────────> DgStore[id] type=1 + protocol
  ├─ pic ────────────────> DgStore[id] type=2 + media binding
  ├─ cxnSp ──────────────> DgStore[id] type=3 + connector callback
  └─ grpSp/AlternateContent
       -> allocate child -> delegate leaf/self -> append child id in XML order
       -> remove invalid graphic frame -> commit group row
```

## §3 编译面与已知依赖

- crate：`libs/ppt-core/Cargo.toml`
- 测试镜像根：`libs/ppt-core/tests/cn/wps/show/pptxr/handler/shape/`
- production XML reader 与若干 DML/model/graphic-frame 模块仍不在默认 crate 编译面；各 function
  通过同路径 contract fixture 编译并执行同一份 production source，真实 pp-01 输出直接复用。
- 五个 function 均已完成非 ignore 测试、函数级 llvm-cov、模板布局和 Java↔Rust 结构门禁；
  默认 crate `cargo test`/`cargo check` 全绿。正式 model/reader feature 恢复时只需替换 contract 接线。

## §4 串联验收

1. 01 用内存 attributes/events 产出可被 leaf handler 消费的 `ShowShape` metadata。
2. 02/03/04 各自只消费 preallocated shape 和 01 输出，分别锁定 type、store commit、callback。
3. 05 用 leaf fixture/producer 编排 XML 顺序、递归 group、AlternateContent 与 cleanup；不启动
   slide 或 package reader。
4. pp-07 `PptxrSlide.open` 只消费 05 输出的 root shape tree（不重跑 leaf 解析）。

## §5 推荐 loop 顺序

`01 → 02 → 03 → 04 → 05`

## §6 类级范围

| Java 类 | function | Rust 现状 |
|---|---|---|
| `SwNvPrHandler` + `PlaceholderShapeHandler` | 01 | 完整实现；8 tests；95.2% lines |
| `SwShapeHandler` + nested ext handlers | 02 | 完整实现；10 tests；92.5% lines |
| `SwPictureHandler` + nested AlternateContent | 03 | 完整实现；8 tests；95.8% lines |
| `SwConnectorShapeHandler` + nested non-visual handler | 04 | 完整实现；7 tests；95.9% lines；空 callback Java 失败语义已锁定 |
| `SwGroupShapeHandler` | 05 | Rust 文件已恢复；递归/复用/cleanup 完整实现 |
| `SwShapeAlertnateContentHandler` nested classes | 05 | Choice/Fallback/ink/OLE 完整实现；无 raw pointer/unsafe |
| `SwGraphicFrameHandler` | 05 的显式依赖 | 使用明确 producer contract；有效性与 cleanup 已覆盖，业务解析仍归 graphic-frame feature |

## 文档

- 机器状态：[functions.csv](./functions.csv)
- function 方案：`functions/<function_id>/FUNCTION.md`
