# pp-19-kmo-tree-write — S4 真实 Kmo 树写入（替换 layout_shape host）

> 层：parser（打开期建树）｜ 状态：**pass** ｜ depends_on: pp-02; pp-10; pp-11; pl-09  
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S4**（消费方触及 S5）  
> 产品 I/O：把 handler 事件写成 Java 同构的 **`show/app/shape/KmoShowShape` + `DgStore` 双 bin + `drawing/data/shape/ShapePr` + `drawing/data/text/TextBody`**，供已 pass 的 pl-09 ConvertTool 读取。

## 为什么是下一个缺口（不是再开一个 convert）

Java 的唯一事实源是 **Kmo 树**（spid = `type<<20|idx`；`endElement` 时 `DgStore.update` + `updateProtocol`）。  
Rust 现状：

| 位置 | 2026-09-12 实测 |
|------|-----------------|
| `show/app` | `cfg(feature="full")`，默认编译面 **没有** Java `KmoShowShape` |
| `app/shape/KmoShowShape.rs` | 42 行，`type()`/`addChild`/`transform2D` 均为 `todo!()` |
| `app/shape/KmoShapeTree.rs` | 14 行，`shapeCount()==0` |
| `util/layout_shape.rs` | 1446 行 fixture host；pptx_mvp / ConvertTool / SlideRender **全部 import 它** |
| `drawing/data/shape/ShapePr.rs` | 1300 行在场；`ShapePropertiesHandler` 写的是 `layout_shape::ShapePr` |
| `TextBodySurf` | pp-11 写入端；不是 `data/text/TextBody` |
| `convert/*` | pl-09 热路径 pass，但 `use layout_shape::{KmoShowShape, ShapePr, GeomObj}` |
| `drawing/app/DgStore.rs` | 文件在；`app/mod.rs` 已去掉 DgStore 的 `cfg(any())`，但 **`drawing/mod.rs` 整模块仍 `feature=full`（依赖 kso）** |

pl-09 已经把 S5 公开 Shape 接上。再改 convert 公式不会让 parse 产物变成 Java Kmo 树。本 feature 切 **S4 写入 I/O**。

## 目标

实现 Java S4 热路径：**分配 spid → 组/子 addChild 双写 → DgStore.add/update → Sw\* endElement 提交 → spPr 写入 `data/shape/ShapePr`（ln 不丢）→ txBody `paraLst.forceWrite` → ConvertTool 改读这些类型**。

**不**重写 pl-09 几何公式；**不**删 `layout_shape`（过渡期测试夹具可留）；**不**用手写 CDL/CCL 冒充 Geometry。loop 走 **function-java2rust**，保留交错 `// java`。

## I/O

| | 契约 |
|--|------|
| **I** | spTree 事件流（pp-02 已能在测试面跑）+ `DgStore` + 预分配 shape |
| **O** | `KmoShapeTree.shapeCount()==root.childCount()`；`KmoShowShape.type()==DgStore.queryType(id)`；`dgShape.spPr()` 为 `drawing.data.shape.ShapePr`（含 ln）；`textBody.paraLst` 有段；ConvertTool 入参类型与上述一致 |
| **错误** | 非法 shapeType → Java `IllegalArgumentException` ≡ Rust `panic!("IllegalArgumentException")`（`DgStoreData.helper_storeByType` default）；未知 spPr 子元素 → `enterSubElement` 返回 null |
| **约束** | 不改 `.java`；不写工程 stub 进 `libs/core-libs/stubs`；禁止 `layout_shape` 继续冒充 Java `KmoShowShape` 作为 **本 feature pass 口径** |

## Java 管线对照

| Java | 行号 | Rust 归属 |
|---|---|---|
| `DgStoreData.queryType` / `helper_storeIndex2Id` | `:19` / `:47` | `01_function_spid_identity` |
| `KmoShowShape.type/id` | `:544-549` | 01（`type()` 必须调 `queryType(dgShapeId)`，禁止平行 `shape_type` 字段当真相） |
| `KmoShowShape.addChild` | `:451-456` | `02_function_showshape_tree` |
| `KmoShapeTree.initRoot/shapeCount` | `:42` / `:558` | 02（`shapeCount==root.childCount()`） |
| `DgStore.add/update` | `:19-31` | `03_function_dgstore_update` |
| `SwShapeHandler.endElement` | `:79-86` | `04_function_sw_end_commit` |
| `ShapePropertiesHandler` | `:31-94` | `05_function_sppr_data_shapepr` |
| `TextBodyHandler.endElement` | `:51-58` | `06_function_txbody_data_textbody` |
| ConvertTool 入参类型 | S5 | `07_function_convert_read_kmo` |

## 必须 / 壳 / 延后

- **must：** 01 spid 位域与 `type()/id()`；02 addChild + initRoot + shapeCount；03 add/update/query/非法 type；04 Sw\* 进默认 mod 且 endElement 双提交；05 ShapePr 写入 data 模型且 ln 提交后仍在；06 paraLst.forceWrite；07 ConvertTool 族去掉 layout_shape 别名。
- **壳：** `show/app` 其它 `cfg(any())` 模块（KmoPresentation 全量、io/table）；graphicFrame/chart 内容。
- **defer：** `ShapePr.follow` 惰性链（G6/G2 后续）；effect/3D；`DgStore.copyOf`；canRotate 递归（G6，02 可只做 type 0 递归骨架或 defer）。

## 验收

1. `DgStore.add(1)` 的 id 满足 `queryType(id)==1` 且低 20 位为仓内下标。
2. `KmoShapeTree` 经 `initRoot` + 两次 `addShape` 后 `shapeCount()==2`（今日 no-op 返回 0）。
3. `SwShapeHandler.endElement` 后 `store.query(id)` 非空且 `kmo.updateProtocol` 被调用。
4. `ShapePropertiesHandler` 构造入参类型 = `drawing::data::shape::ShapePr`；`a:ln` 提交后 `shapePr.ln()` 可查询宽度。
5. `TextBodyHandler` 一段一字 → `textBody.paraLst` 长度 1。
6. `ConvertTool.convertShapeToPublicShape` **不再** `use layout_shape::KmoShowShape`（grep 门闩）。

## 依赖

- **upstream：** pp-02（Sw\* 测试面已 pass）；pp-10/11（handler 族文件在，写入端错）；pl-09（convert 热路径，host 错）。
- **downstream：** pl-02/03/06/pr-02 逐步改 import；本波 07 只切 convert 编译面类型。
- **不依赖：** wpslayout、Flutter、effectLst。
