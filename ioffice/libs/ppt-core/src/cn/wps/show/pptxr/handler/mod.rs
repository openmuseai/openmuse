// M-PPT-KMOSLIDE W8c：`slidelayouts/`（SlideLayoutHandler.rs 207 行）与 `slidemasters/`
// （HeaderFooterHandler.rs 102 / SlideMasterHandler.rs 253 / TextStylesHandler.rs 79）四个文件
// **真实存在、`todo!` 计数为 0、无桩**，但两个目录都没有 `mod.rs`，也未被本文件登记
// ⇒ 合计 641 行忠实转写从未进入编译面（假编译面形态：目录整体未登记）。
// 本波实测结论：**登记动作当前不可完成**，且阻塞不在本目录内。以下为可复现实测与依据。
//
// ★实测（复现方法：补 `slidelayouts/mod.rs` + `slidemasters/mod.rs`，在本文件加两行 `pub mod`，
//   `touch src/lib.rs; cargo check --lib --message-format=short | grep -E '^src.*error'`，随后还原）：
//   | 步骤 | 错误数 | 错误族（实测） |
//   |---|---|---|
//   | 原样登记 | **15** | 11×`E0433 cannot find xpxml in kfc`（四文件的 `crate::cn::wps::kfc::xpxml::reader::*`；ppt-core 的 `src/cn/wps/kfc/` 只有 `ooxfr/`）<br>2×`E0433/E0432 show::data::slidemasters`（默认构建下 `show/mod.rs:4-59` 的 inline 桩未放开 `slidemasters`）<br>1×`E0432 show::app::slide::KmoSlideMaster`（`show/app/slide/mod.rs` 未登记）<br>1×`E0432 handler::slides::ColorMapOverrideHandler`（`handler/slides/mod.rs` 未登记） |
//   | 换 kfc_alpha 真身导入 + 临时放开上述 3 处模块 | **51** | 24×`E0053/E0046/E0407`（`&mut self` vs `&self`、缺 `characters`、多 `kind()`）<br>25×`E0425/E0405`，全部来自 `show/app/slide/KmoSlideMaster.rs`（60 行自动生成桩、59×`todo!`，自身不可编译）<br>2×`E0432 data::slidemasters::SlideMaster`（`show/data/slidemasters/mod.rs:1` 的 `pub mod SlideMaster;` 被注释） |
//
// ★契约（唯一可行选择，不是二选一）：入口 `UnifiedXmlReader::read__inputstream_elementhandler(
//   stream, handler: Box<dyn ElementHandler>)`（`show/pptxr/util/UnifiedXmlReader.rs:2,27`）用的就是
//   `kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler` —— 方法取 `&self`、`characters` 必填、
//   **没有** `kind()`；`ppt/slidelayouts/PptxrLayout.rs:208-221` 与 `ppt/slidemasters/PptxrMaster.rs:241-247`
//   为这四个类所写的 `compile_surface` 替身正是按该 trait 实现的（`&self` + `characters`）。
//   ⇒ 四文件现有形态（`startElement(&mut self, ..&dyn XmlRoAttr)` 取自 `sw_shape_hosts` 风格，
//      `enterSubElement(&mut self)`/`kind()` 取自 `dmlr` 风格）自相矛盾，必须统一改为
//      `&self` + `RefCell/Cell` 内部可变性、删 `kind()`、补 `characters`。
//
// ★真正的阻塞在模型层（均不在 `handler/` 内）：Java 语义要求子处理器持有 layout/master 的
//   **内部对象引用**并就地改写，而 Rust 侧访问器返回 `&mut T`：
//     · `SlideLayout::{clrMap,hf,timing,transition}` → `&mut T`
//       （`show/data/slidelayouts/SlideLayout.rs:557,641,725,809`）
//     · `SlideMaster::{clrMap,hf,titleStyle,bodyStyle,otherStyle,timing,transition}` → `&mut T`
//       （`show/data/slidemasters/SlideMaster.rs:582,661,898,740,819,1011,1090`）
//   子处理器却要求共享句柄，两者无法互相投影（**不能从 `&mut T` 得到 `Rc<RefCell<T>>`**）：
//     · `ColorMapOverrideHandler::ColorMapOverrideHandler(Rc<RefCell<ColorMapping>>)`（本目录 `slides/ColorMapOverrideHandler.rs:22`）
//     · `HeaderFooterHandler::HeaderFooterHandler(Option<Rc<RefCell<HeaderFooter>>>)`（本目录 `slidemasters/HeaderFooterHandler.rs:22`）
//     · `TimingHandler::TimingHandler(Rc<RefCell<Timing>>, Rc<RefCell<HandlerCallback>>)`（`timing/TimingHandler.rs:45`）
//     · `TransitionHandler::TransitionHandler(Rc<RefCell<Transition>>, ..)`（`transition/TransitionHandler.rs:154`）
//     · `TextListStyleHandler::TextListStyleHandler(Rc<RefCell<TextListStyle>>, HandlerCallbackState)`（`drawing/dmlr/handler/shape/textshape/text/TextListStyleHandler.rs:72`）
//     · `ColorMapHandler::ColorMapHandler(Rc<RefCell<ColorMappingObj>>)`（`drawing/dmlr/handler/style/ColorMapHandler.rs:18`）
//   同理 `KmoSlideLayout::film(&self) -> Option<&KmoFilm>`（`show/app/slide/KmoSlideLayout.rs:129`）无法喂给
//   `CommonSlideDataHandler(Shared<KmoFilm>, *mut DgStore, ..)`，而 `KmoSlideMaster::{protocol,updateProtocol,film}`
//   全是 `todo!()`（`show/app/slide/KmoSlideMaster.rs:12-15`）。
//   ⇒ 前置为模型层给出共享句柄（`Rc<RefCell<T>>` 字段或 `*_handle()` 访问器）+ 真实 `KmoSlideMaster`，
//      即 `handler/shape/mod.rs:14-33` 已登记的里程碑 **M-PPT-SHAPE-DISPATCH** 的同一批前置。
//   （已否决的替代方案：用 `*mut T` 绕过 —— 违反 owner 规则「模型引用不得用裸指针」，且
//     `Shared<KmoFilm>` 一侧仍无法从 `&KmoFilm` 造出，无法闭环。）
//
// ★门禁互斥：`tests/cn/wps/show/pptxr/handler/{slidelayouts,slidemasters}/*.rs`
//   （`[[test]] 06_function_slide_layout` / `07_function_slide_master`）是**影子 crate 型**测试：
//   它们用 `#[path]` 把真实源码编进一套自造 `pub mod cn` 依赖面，并断言**只存在于该影子面**的形态 ——
//   `kind()`（`dispatches_layout_children` / `dispatches_master_children`）、
//   `TimingHandler::new(Timing, Option<Rc<RefCell<C>>>)` 之类的泛型构造子（真身不是泛型）。
//   因此任何按真身契约的改写都会让这两个 target 变红；W5 对同类 target（`04_function_common_slide_data`）
//   的处置是「测试面收口」重写为真 API。本波不改 `tests/`，故登记与这两个门禁互斥。
//
// 建议解除顺序：① 模型层共享句柄 + 真实 `KmoSlideMaster`；② 放开 `show/mod.rs` inline 桩的
//   `slidemasters`、`show/data/slidemasters/mod.rs:1` 的 `SlideMaster`、`show/app/slide/mod.rs` 的
//   `KmoSlideMaster`；③ 四文件改 kfc 契约（`&self` + `RefCell`，去 `kind()`，补 `characters`）；
//   ④ 登记 `slides/ColorMapOverrideHandler`；⑤ 测试面收口 06/07。
// owner: show.pptxr.handler.slidelayouts / show.pptxr.handler.slidemasters    wave: M-PPT-KMOSLIDE-W8c
pub mod CommentAuthorListHandler;
pub mod PresentationHandler;
pub mod TableStyleListHandler;
pub mod custdatas;
pub mod shape;
pub mod slides;
pub mod transition;
pub mod timing;
