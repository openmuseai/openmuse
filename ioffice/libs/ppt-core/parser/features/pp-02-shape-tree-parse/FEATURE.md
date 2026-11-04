# pp-02-shape-tree-parse — Slide shape tree 解析

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（5 个 function，均 `pass`）
> 依赖：pp-01-pptx-parse
> 管线：[PIPELINE.md](../PIPELINE.md) / [README.md](../README.md)

## 目标

把 pp-01 已产生的 `p:spTree` element/attribute 事件解析成**有序**的 `KmoShowShape` / `DgStore` 树：
01 先写 nvPr 元数据；02/03/04 三种并行 leaf committer（`p:sp`/`p:pic`/`p:cxnSp`）各自锁定一行
`DgStore`；05 做唯一编排——按 XML 顺序收 child、递归 group、调停 `mc:AlternateContent` 的
Choice/Fallback/ink。本 feature 不打开 ZIP/OPC、不装配 slide/master/layout、不渲染；输出 root
tree 由 pp-07 `PptxrSlide.open` 消费。

## I/O（锁死）

| function | I | O | 契约 |
|---|---|---|---|
| 01 nvPr 元数据 | `p:nvPr` attrs/sub-events + `ShowShape` | flags、media mode、placeholder、custData 写入 | `idx="4294967295"` 归一为 `-1`；orientation 来自 `DGMap`，size/type 来自 pp-01 `SWReaderMap`，未知枚举保持默认 |
| 02 auto shape | `p:sp` events + preallocated `KmoShowShape` | protocol 更新 + `DgStore[id]` type=1 shape row | `ShapeExtLstHandler` 随 02 验收：与 auto shape 共用同一 Java/Rust 模板，同时被 picture/group 作为已声明能力复用 |
| 03 picture | `p:pic` events + preallocated picture | `DgStore[id]` type=2 picture row、blip/style、外链 share id | nested AlternateContent 随 03 |
| 04 connector | `p:cxnSp` events + preallocated connector | `DgStore[id]` type=3 connector row + `onConnectorShape(id)` | 空 callback 的 Java 失败语义已锁定 |
| 05 spTree 编排 | `p:spTree`/`mc:AlternateContent` events | 有序递归 child tree、fallback/ink 选择、无效 graphic cleanup、group row 提交 | leaf 委托 02/03/04；child id 按 XML 顺序 append；无效 graphic 逆序删除后提交 group |

## Java 对照

| Java 类 | Rust 路径 | 裁决 |
|---|---|---|
| `SwNvPrHandler` + `PlaceholderShapeHandler`（01） | `src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs` | 完整实现；8 tests；95.2% lines；模板布局 + 结构与 Java 对齐 PASS |
| `SwShapeHandler` + nested ext handlers（02） | `.../handler/shape/SwShapeHandler.rs` | 完整实现；10 tests；92.5% lines |
| `SwPictureHandler` + nested AlternateContent（03） | `.../handler/shape/SwPictureHandler.rs` | 完整实现；8 tests；95.8% lines |
| `SwConnectorShapeHandler` + nested non-visual handler（04） | `.../handler/shape/SwConnectorShapeHandler.rs` | 完整实现；7 tests；95.9% lines；空 callback Java 失败语义已锁定 |
| `SwGroupShapeHandler`（05） | `.../handler/shape/SwGroupShapeHandler.rs` | 递归/复用/cleanup 完整实现；raw-pointer nested-owner 已改 `Rc<RefCell<State>>` |
| `SwShapeAlertnateContentHandler` nested classes（05） | `.../handler/shape/SwShapeAlertnateContentHandler.rs` | Choice/Fallback/ink/OLE 完整实现；无 raw pointer/unsafe |
| `SwGraphicFrameHandler`（05 的显式依赖） | `.../handler/shape/graphicframe/`（producer contract） | 有效性与 cleanup 已覆盖；业务解析仍归后续 graphic-frame feature |

Rust 根路径前缀：`libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/`。各 function 通过同路径
contract fixture 编译同一份 production source（见 [FUNCTIONS.md](./FUNCTIONS.md) §3）。

## 必须 / 壳 / 延后

- **must：** 01 的 nvPr 元数据写入（photo/userDrawn、media mode、placeholder 保存、custData；未知属性/子元素不写入）。02/03/04 各自提交 `DgStore` 行（type 1/2/3）+ protocol/callback。05 按 XML 顺序编排 child、递归 group（自递归）、AlternateContent 的 Choice/Fallback/ink 有效输出、无效 graphic 逆序 cleanup 后提交 group；提交顺序 child 先于 group。
- **stub：** media/custData/ink 业务体（constructor contract fixture，业务由各自 feature 实现）；`GroupShapePropertiesHandler`（DML transform，Rust 当前被裁剪，不伪造 transform）；真实 XML reader 事件分发（contract fixture，等 pp-07 接线）。
- **defer：** `SwGraphicFrameHandler` 业务解析（有效性与 cleanup 已覆盖，业务体归 graphic-frame feature）；恢复被裁剪的 `ShowShape`/`KmoShowShape` 模型及 `KoBin`/`KpMap` 图（模型 feature）。

## 范围内 / 范围外

- **in：** 嵌套 group 递归（group→sp、三层 group + 多 leaf）；handler 经 `init` 复用（无旧状态泄漏）；blip stretch rect 归一（tile-like 无 stretch rect → 空 `RelativeRect`，其它 fill 不改）；GVML `newSps` 与普通插入两种写入模式；unknown/unsupported element 的 `None`/明确停车语义。
- **out：** ZIP/OPC/`OpcrPackagePart` 打开；slide/master/layout 装配（pp-07 起）；渲染；ink 文件解析（`isGVml`/`onInk` 只调用 callback，不解析文件）；theme/timing/presentation.xml（pp-03–pp-06）。

## 验收

1. 01 用内存 attributes/events 产出可被 leaf handler 消费的 `ShowShape` metadata。
2. 02/03/04 各自只消费 preallocated shape 和 01 输出，分别锁定 type、store commit、callback（`onConnectorShape`）。
3. 05 用 leaf fixture/producer 编排 XML 顺序、递归 group、AlternateContent 与 cleanup；不启动 slide 或 package reader。
4. pp-07 `PptxrSlide.open` 只消费 05 输出的 root shape tree（不重跑 leaf 解析）。

回归：`cargo test --manifest-path libs/ppt-core/Cargo.toml --test <function_id>`（01/02/03/04/05
五个 `[[test]]`，与 [functions.csv](./functions.csv) 对齐）。

## 依赖

- **depends：** pp-01（`SWReaderMap` placeholder size/type 查表、`HandlerCallback`、`XmlRoAttr`/element id 事件面）。`DgStore`/`ShowShape`/`GraphicObject` model 契约（当前工作树被裁剪，测试 fixture 维持同名 contract）。
- **does not depend：** pp-07 起的 ZIP/OPC 与 slide 装配；渲染；pp-03/04/05/06（DML style/theme/timing/presentation 只回填 01/05 的 constructor fixture）。

## 不过扩展

禁止打开 `OpcrPackagePart`（PIPELINE.md §1 故意缺口）。禁止渲染、禁止装配 slide。group 递归是
05 自递归，**不把递归边写成 CSV 环**；02/03/04 是并行 leaf，05 是唯一编排 function。media/
custData/ink 不允许用 `None` 假装支持；无效 graphic 清除是显式行为，不用空 graphic 表示成功。
不恢复被裁剪的模型/媒体业务体，不提前实现 pp-07 reader 接线或 graphic-frame 业务解析。