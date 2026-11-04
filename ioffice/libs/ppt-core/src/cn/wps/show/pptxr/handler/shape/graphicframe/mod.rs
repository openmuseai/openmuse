// M-PPT-SHAPE-DISPATCH：本目录此前是 **0 字节占位 `mod.rs`**（形态：目录存在但整体未登记），
//   而 `shape/mod.rs:36` 早已声明 `pub mod graphicframe;` ⇒ 真文件存在却永不编译。
//   Java 侧本目录 5 个类共 837 行：
//     · `SwGraphicFrameHandler.java`      90 行 —— **已完整移植**（`SwGraphicFrameHandler.rs`），
//                                              但**暂不可登记**（原因见下）
//     · `SwGraphicHandler.java`          166 行 —— 未移植（依赖闭包在仓库内整体不存在）
//     · `SwModel3DHandler.java`          377 行 —— 未移植（同上）
//     · `SwOleObjectHandler.java`        117 行 —— 未移植（同上）
//     · `SwGraphicDataAlternateContentHandler.java`  87 行 —— 未移植（同上）
//   缺的 4 个类的共同下级依赖（`KmoOleChart` / `KmoTable` / `TableStyle` / `ChartHandler` /
//   `DgmRelIdsHandler` / `TableHanlder` / `KmoDiagram` / `KmoOleObject` / `KmoTableStyleList` /
//   `LockedCanvasHandler`）所在目录 `drawing/dmlr/handler/{table,chart,diagram}`、
//   `show/app/diagram`、`show/clipxr/gvml/handler` **在本仓库中不存在**。
//
// ★`SwGraphicFrameHandler.rs` 暂不可登记的**实测**原因（非模板产物，逐条可复现）：
//   1. `use …::shape::SwNvPrHandler::SwNvPrHandler` —— `SwNvPrHandler.rs`（299 行真身）自身尚未
//      登记（其登记前置与本文件同批）⇒ E0432 `could not find SwNvPrHandler in shape`。
//   2. 该文件按 Java 原样书写了对**真模型类型**的访问，而真模型类型的 API 与 Java 引用语义
//      不一致，必须逐条适配：
//      · `KmoShowShape::{protocol,updateProtocol}` 是 `KmoProtocol` trait 的方法，需 `use` 该 trait；
//      · `KmoShowShape::parent() -> Option<*const dyn ISlide>`（Java 是可直接调用的引用，
//        Rust 侧须 `unsafe { (*p)… }` 且先解 `Option`）；
//      · `Shape::open(&dyn KoBin) -> Shape`（**不返回** `Shared<Shape>`）；`drawingShape()`
//        返回 `Box<dyn KoBin>`（需 `&*`）；
//      · `Shape::cNvPr()` / `blipFill()` 的惰性解码分支未实现，槽位为 None 时 `unwrap()` panic；
//      · `Transform2D` 没有 `Default`；`ShapePr::xfrm()` 返回 `&mut Transform2D`（非共享句柄）；
//      · `KmoShowShape::drawingShape()` 返回 `bh` 句柄而非字节（见 PLAN §26 判据 4 修订说明）。
//   ⇒ 登记它与登记 `shape/` 其余三叶同为一个不可分割的动作：必须同时把 `shape/` 目录从
//      **`super::sw_shape_hosts` 的类型影子**上摘下来（`SwGroupShapeHandler.rs:4,5,11,16,17,18,22,
//      24,25,30,32` 共 11 行 `use super::sw_shape_hosts::{…}` 导入的正是 `Shape`/`ShowShape`/
//      `ShapePr`/`ShapeStyle`/`ElementHandler`/`Shared` 及 4 个叶子类影子），
//      并前置**模型层共享句柄改造**（`Shape`/`ShapePr` 的 `Option<Box<T>>` → 共享句柄）。
//      两段均已登记在本 PLAN §27（M-PPT-SHAPE-DISPATCH 第二段）。
//   owner: cn.wps.show.pptxr.handler.shape.graphicframe    wave: M-PPT-SHAPE-DISPATCH
//
// ★③-5：真身已登记（依赖已逐条落地：SwNvPrHandler 已登记、② 契约已确立、真模型类型导入已修正）。
//   owner: cn.wps.show.pptxr.handler.shape.graphicframe    wave: M-PPT-SHAPE-DISPATCH
pub mod SwGraphicFrameHandler;
