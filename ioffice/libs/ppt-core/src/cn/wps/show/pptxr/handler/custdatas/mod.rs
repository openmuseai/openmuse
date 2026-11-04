// M-PPT-KMOSLIDE W9：本文件在 W6c-1 时只有一行，且是
//   `// pub mod CustDataLstHandler; // LLM: template has Java syntax`
// —— **假编译面形态 B（注释掉的模块声明）**：真实文件存在（106 行），却永不编译，
// 且注释理由是模板产物而非真实阻塞。W6c-1 之后改为「具名阻塞登记」。
//
// ★W9 实测复核（可复现）：登记后仍余 9 个错误，逐项已解除：
//   1) `DefaultElementHandler`（Java 父类）**已由 W8 真实登记**并实现规范 `ElementHandler`：
//      `kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::DefaultElementHandler`（四个方法体皆空，
//      与 `DefaultElementHandler.java` 20 行一致）⇒ 本文件按该真身导入。
//   2) `XmlRoAttr`（kfc_alpha）是 **trait** ⇒ 形参 `&XmlRoAttr` 全部改 `&dyn XmlRoAttr`。
//   3) `HandlerCallback` 是 **trait**（`drawing/dmlr/model/HandlerCallback.rs:65`）⇒
//      本文件改为泛型 `CustDataLstHandler<C: HandlerCallback>` 并把回调存成
//      `Option<Rc<RefCell<C>>>`（repo 既有约定，同 `MediaExtLstHandler<C>`）。
//   4) `TagsHandler` 原先用 `*const HandlerCallback` / `*const CustDataLst` 裸指针 + `unsafe`
//      解引用 ⇒ 改为 `Rc<RefCell<..>>` 句柄（模型引用不得用裸指针）。
//
// ★★本项是**管线上的真实保真缺口**，W9 已完成接线：
//   Java `CommonSlideDataHandler.java:47` 会
//   `new CustDataLstHandler(this.commonSlideData.custDataLst(), this.handlerCallback)`，
//   但 Rust 侧 `CommonSlideDataHandler.rs` 原先导入 `shape::sw_shape_hosts::CustDataLstHandler`
//   —— 一个**同名单元空壳**（`sw_shape_hosts.rs:254`，形态 F），其 `new()` **丢弃 custDataLst**，
//   调用点因而写成 `CustDataLstHandler::new((), self.handlerCallback.clone())`（`():` 占位）。
//   ⇒ `p:custDataLst` 里解析出的 tags **不进入模型**，`onTags(temp.toString(), custDataLst.tags())`
//   从未被调用。W9 已删除该空壳并改接真实类（见 `CommonSlideDataHandler.rs` 的 case 3211519）。
//   注：`CustDataLst`/`Tag` 本身是**真实**的（SP1.0，728/424 行），丢的是**接线**而非实现。
//
// ★契约：规范契约 `kfc_kfc_alpha::…::xpxml::reader::ElementHandler`（接收者全为 `&self`），
//   即 `UnifiedXmlReader::read__inputstream_elementhandler` 的形参类型。本目录只有本一个模块。
//   owner: show.pptxr.handler.custdatas    wave: M-PPT-KMOSLIDE-W9
//
// 遗留（不在本波文件面内）：`C::TagLst` 关联类型尚未在**产品回调**上接线 ——
//   `pptx_mvp::OpenCb::TagLst = ()`、门禁 `04_function_common_slide_data::RecordingCallback::TagLst = ()`。
//   两个 impl 见 `CustDataLstHandler.rs::TagLstSlot`（真实类型交付活对象；`()` 交付 null）。
pub mod CustDataLstHandler;
