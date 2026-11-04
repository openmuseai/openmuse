#![allow(non_snake_case, dead_code, unused_imports)]

// Java: import cn.wps.drawing.data.style.ColorMapping
// rust: no-stmt
// Java: import cn.wps.show.app.KmoPresentation
// rust: no-stmt
// Java: import cn.wps.show.app.KsoWMInfo
// rust: no-stmt
// Java: import cn.wps.show.app.event.KmoSlideEventListeners
// rust: no-stmt
// Java: import cn.wps.show.app.shape.KmoShapeTree
use crate::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
// Java: import cn.wps.show.app.theme.KmoTheme
// rust: no-stmt
// Java: import cn.wps.show.data.background.Background
// rust: no-stmt
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.show.app.media.KmoMediaLib
// rust: ★③-5 —— 真身（SP1.0 交付），此前该宿主无 mediaLib()，见 `KmoPresentation::mediaLib` 的登记。
use crate::cn::wps::show::app::media::KmoMediaLib::KmoMediaLib;
use std::any::Any;

pub struct KmoPresentation {
    pub drawing_store: *mut DgStore,
    // rust: ★③-5 —— 本 port 的 `ISlide` 宿主 `KmoPresentation` 是**编译面替身**（真身
    //   `show::app::KmoPresentation` 仍受 `#[cfg(feature = "full")]` 门禁，见 app/mod.rs:20）。
    //   Java `KmoPresentation.mediaLib()`（真身 `KmoPresentation.rs:869`）返回**同一个**
    //   `KmoMediaLib` 实例；此处以 `Rc<RefCell<..>>` 承载同一语义（单例、可装配），
    //   由 `pptx_mvp::OpenSlideHost::with_store` 在打开期注入**真实**的媒体表。
    //   这是**真实接线**（不是新假面）：它不产生新行为分支，只是把真身
    //   `KmoMediaLib::getMediaData()` 暴露在该宿主上，令 `SwPictureHandler` 的
    //   `parent().getPresentation().mediaLib().getMediaData(id)` 与 Java 逐字同形。
    //   owner: show.app.slide.ISlide    wave: M-PPT-SHAPE-DISPATCH
    pub media_lib: std::rc::Rc<std::cell::RefCell<KmoMediaLib>>,
}
impl Default for KmoPresentation {
    fn default() -> Self {
        KmoPresentation {
            drawing_store: std::ptr::null_mut(),
            media_lib: std::rc::Rc::new(std::cell::RefCell::new(KmoMediaLib::new())),
        }
    }
}
impl KmoPresentation {
    pub fn canvas(&self) -> DgCanvas {
        DgCanvas {
            store: self.drawing_store,
        }
    }

    // Java: DgStore drawingStore() — canvas.store()
    pub fn drawingStore(&self) -> *mut DgStore {
        self.drawing_store
    }

    // Java: KmoMediaLib mediaLib()
    // rust: ★③-5 —— 真身 `KmoPresentation.rs:869` 的同名成员在此宿主上的真实暴露；
    //   返回**共享句柄**（Java 返回同一实例），令 `SwPictureHandler` 可经
    //   `parent().getPresentation().mediaLib().getMediaData(id)` 取真实媒体数据。
    //   owner: show.app.slide.ISlide    wave: M-PPT-SHAPE-DISPATCH
    pub fn mediaLib(&self) -> std::rc::Rc<std::cell::RefCell<KmoMediaLib>> {
        self.media_lib.clone()
    }
}
pub struct DgCanvas {
    store: *mut DgStore,
}
impl DgCanvas {
    pub fn addEmptyShape(&self, r#type: i32) -> i32 {
        if self.store.is_null() {
            panic!("NullPointerException");
        }
        unsafe { (*self.store).add(r#type) }
    }
}
// ★去假面（形态 F，W6c-1）：此处原先声明 `pub struct KmoSlideEventListeners;` —— 一个**同名单元空壳**，
//   把 `app/event/KmoSlideEventListeners.rs`（Java 240 行 → Rust 456 行，SP1b 交付的**真实实现**）
//   **遮蔽**成零大小类型，使 `ISlide::eventListeners()` 的返回类型是空壳而非真身。
//   现改为**再导出真身**，所有既有的 `ISlide::KmoSlideEventListeners` 路径自动解析到真实类型。
//   连带改动仅 3 处「把空壳当值用」的表达式（单元结构体可直写类型名，真身须 `::new()`）：
//     `slide/KmoFilm.rs:107`、`pptxr/pptx_mvp.rs:1189`、`pptxr/handler/shape/SwGroupShapeHandler.rs:518`。
//   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6c
pub use crate::cn::wps::show::app::event::KmoSlideEventListeners::KmoSlideEventListeners;
// ★去假面（形态 F，W6c-4）：此处原先内联声明 4 个**同名单元空壳**
//   （`KmoTheme`/`KsoWMInfo`/`ColorMapping`/`Background`），把真身**遮蔽**成零大小类型，
//   使 `ISlide::{theme,ksoWMInfo,colorMapping,background}()` 返回假类型。
//   本波把可达的真身改为**再导出**；`KmoTheme` 的真身在 `show::app::theme`（该模块仍受
//   `#[cfg(feature = "full")]` 门禁，依赖 `drawing::app::KmoProtocol` 未移植）⇒ 见下方登记。
//   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6c-4
pub use crate::cn::wps::show::app::KsoWMInfo::KsoWMInfo;
pub use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
pub use crate::cn::wps::show::data::background::Background::Background;
// ★登记未闭合项（owner/wave）：`KmoTheme` 真身在 `show::app::theme::KmoTheme`（Java 真身），
//   但 `show/app/mod.rs` 对 `theme` 模块有 `#[cfg(feature = "full")]` 门禁，且 `KmoTheme.rs`
//   依赖 `drawing::app::KmoProtocol`（未移植）。解除需整波 **M-PPT-PRES-ASM2**。
//   ⇒ 本波保留单元空壳为**已登记的部分假面**（⑰.5 由 4/4 降为 1/4）。
//   ★W6c-9 复核（实测，未解除）：两道门禁**都确实生效** ——
//     ① `app/mod.rs:21-22` = `#[cfg(feature = "full")] pub mod theme;`
//        ⇒ 默认构建下 `app::theme` 路径**不存在**（`E0433: could not find theme in app`）；
//     ② `theme/mod.rs` = `#[cfg(any())] pub mod KmoTheme;`（形态 A 假面）。
//   本轮曾尝试"临时放开 ② 再 `cargo check`"，仍因 ① 报同一 E0433
//   ⇒ 证实**仅放开 `theme/mod.rs` 不够**，须先解除 `feature = "full"`（M-PPT-PRES-ASM2）。
//   故本项维持登记，**不**按"已过期"处理（避免误判为可立即收口）。
//   owner: show.app.theme    wave: M-PPT-PRES-ASM2
pub struct KmoTheme;

// Java: public interface ISlide
pub trait ISlide {
    // Java: abstract int type()
    fn r#type(&self) -> i32;

    // Java: abstract long id()
    fn id(&self) -> i64;

    // Java: abstract KmoPresentation getPresentation()
    fn getPresentation(&self) -> &KmoPresentation;

    // Java: abstract KmoTheme theme()
    fn theme(&self) -> &KmoTheme;

    // Java: abstract KmoSlideEventListeners eventListeners()
    fn eventListeners(&self) -> &KmoSlideEventListeners;

    // Java: abstract KmoShapeTree spTree()
    fn spTree(&self) -> &KmoShapeTree;

    // Java: abstract void replaceSpid(int var1, int var2)
    fn replaceSpid(&self, var1: i32, var2: i32);

    // Java: abstract Background background()
    fn background(&self) -> &Background;

    // Java: abstract KsoWMInfo ksoWMInfo()
    fn ksoWMInfo(&self) -> &KsoWMInfo;

    // Java: abstract ColorMapping colorMapping()
    fn colorMapping(&self) -> &ColorMapping;

    // Java: abstract boolean isIOReading()
    fn isIOReading(&self) -> bool;

}
