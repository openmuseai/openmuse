// M-PPT-KMOSLIDE W7/W6c-1：本文件原先只有 5 行，且末尾是
//   `// pub mod SwShapeAlertnateContentHandler; // LLM: template has Java syntax`
// —— 那是**假编译面形态 B（注释掉的模块声明）**：真实文件存在（687 行），却永不编译，
// 且注释理由是模板产物而非真实阻塞。下面把每个「真实存在但未登记」的 handler 的**实测阻塞**写清楚。
//
// ★实测方法（可复现）：逐个把 `pub mod X;` 临时加进本文件，`touch src/lib.rs` 后
//   `cargo check --lib --message-format=short | grep -cE '^src.*error'`，再还原。
//   | 文件 | 行数 | 单独登记后的错误数 |
//   |---|---|---|
//   | `SwPictureHandler.rs`               | 429 | **8**  |
//   | `SwConnectorShapeHandler.rs`        | 260 | **8**  |
//   | `SwShapeAlertnateContentHandler.rs` | 687 | **10** |
//   | `SwNvPrHandler.rs`                  | 299 | **10** |
// ⇒ **四者当前均不可登记**，前置为里程碑 **M-PPT-SHAPE-DISPATCH**（基线 §16.2/§26.2）。
//
// 实测错误类（以 `SwPictureHandler` 为例）：
//   · `kfc::xpxml` —— 该模块路径**根本不存在**（`use crate::cn::wps::kfc::xpxml::reader::…`）；
//     真实 reader trait 在 `kfc_alpha` 且方法取 `&self`，与宿主 `&mut self` 不兼容。
//   · `drawing::dmlr::handler::shape::nvsppr::cnv::{CNvPrHandler, CNvPicPrHandler}` —— 不存在
//     （Java 88 / 102 / 58 行）。
//   · `drawing::dmlr::handler::shape::style::ShapeStyleHandler` —— 不存在。
//   · `shape::{ShapePr, ShapeStyle}` —— 真身模型类型。
//   · `shape::SwNvPrHandler` —— 未登记（相互依赖）。
//   完整清单（≈3,100 Java 行 / 4 层）见 `M_PPT_SLIDE_ASM_BASELINE.md` §26.2。
//
// ★M-PPT-KMOSLIDE W6c-3：`sw_shape_hosts.rs`（影子 ① 契约宿主）已**整体删除** —— 其同名替身
//   与影子 `ElementHandler`/`XmlRoAttr` 契约在本波之后零消费者（`p:bg` 已接真身 handler）。
//   owner: show.pptxr.handler.shape    wave: M-PPT-KMOSLIDE-W6c-3
//   删除替身会让 `01_function_sw_group_on_mod` 的 `sg_pic`/`sg_gf` 从 6/6 掉到 4/6
//   （该 target 断言 `enterSubElement(3211469/3211595).is_some()`，即 Java 的
//   `handler = this.pictureHandler` 非 null）⇒ A12 与既有门禁**互斥**，已登记为阻塞。
//   owner: show.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
//
// blocker: 真实 SwPictureHandler / SwConnectorShapeHandler / SwShapeAlertnateContentHandler / SwNvPrHandler
//          的存在均已确认，但依赖闭包未移植 ⇒ 保持未登记，前置 M-PPT-SHAPE-DISPATCH。
pub mod SwShapeHandler;
pub mod SwNvPrHandler;
pub mod graphicframe;
pub mod SwPictureHandler;
pub mod SwConnectorShapeHandler;
pub mod SwShapeAlertnateContentHandler;
pub mod SwGroupShapeHandler;
