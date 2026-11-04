//! 04_function_sw_end_commit — SwShapeHandler.init / endElement 双 bin

use ppt_core::cn::wps::drawing::app::DgStore::DgStore;
use ppt_core::cn::wps::drawing::dmlr::model::HandlerCallback::{
    HandlerCallback, HandlerCallbackState,
};
use ppt_core::cn::wps::kso::kproto::KoBin::KoBin;
use ppt_core::cn::wps::show::app::shape::kmo_show_shape_hosts::KmoProtocol;
use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use ppt_core::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
use ppt_core::cn::wps::show::app::slide::ISlide::{
    Background, ColorMapping, ISlide, KmoPresentation, KmoSlideEventListeners, KmoTheme, KsoWMInfo,
};
// rust: ★③-5 —— 真身 handler 已从影子面收敛到**真身模型类型**（PLAN §26 判据 2.5）：
//   `SwShapeHandler<C>` 现要求 `C: HandlerCallback<ShapePr = data::shape::ShapePr,
//   ShapeStyle = data::style::ShapeStyle, Hyperlink = data::hyperlink::Hyperlink>`。
//   原文件从 `sw_shape_hosts`（j2r 编译面空壳）取同名类型 ⇒ E0271 类型身份不符。
//   故按**真身包路径**导入，`DummyCb` 的关联类型随之对齐（同 `01_function_sw_group_on_mod.rs`）。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
use ppt_core::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use ppt_core::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
use util::cn::wps::util::manager::FileManager::FileManager;
use ppt_core::cn::wps::show::pptxr::handler::shape::SwShapeHandler::SwShapeHandler;
use std::cell::RefCell;
use std::rc::Rc;

struct DummySlide {
    pres: KmoPresentation,
    theme: KmoTheme,
    events: KmoSlideEventListeners,
    bg: Background,
    wm: KsoWMInfo,
    cm: ColorMapping,
}

impl DummySlide {
    fn with_store(store: &mut DgStore) -> Self {
        DummySlide {
            pres: KmoPresentation {
                drawing_store: store as *mut DgStore,
                // rust: ★③-5 —— 真身 `KmoPresentation` 新增媒体表字段。
                //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
                media_lib: std::rc::Rc::new(RefCell::new(
                    ppt_core::cn::wps::show::app::media::KmoMediaLib::KmoMediaLib::new(),
                )),
            },
            theme: KmoTheme,
            events: KmoSlideEventListeners::new(),
            // rust: ★W6c-4 —— 三个真身类型不再是单元结构体 ⇒ 必须构造实例。
            //   owner: tests    wave: M-PPT-KMOSLIDE-W6c-4
            bg: Background::new(),
            wm: KsoWMInfo::new(),
            cm: ColorMapping::create(),
        }
    }
}

impl ISlide for DummySlide {
    fn r#type(&self) -> i32 {
        0
    }
    fn id(&self) -> i64 {
        0
    }
    fn getPresentation(&self) -> &KmoPresentation {
        &self.pres
    }
    fn theme(&self) -> &KmoTheme {
        &self.theme
    }
    fn eventListeners(&self) -> &KmoSlideEventListeners {
        &self.events
    }
    fn spTree(&self) -> &KmoShapeTree {
        panic!("unused")
    }
    fn replaceSpid(&self, _var1: i32, _var2: i32) {}
    fn background(&self) -> &Background {
        &self.bg
    }
    fn ksoWMInfo(&self) -> &KsoWMInfo {
        &self.wm
    }
    fn colorMapping(&self) -> &ColorMapping {
        &self.cm
    }
    fn isIOReading(&self) -> bool {
        false
    }
}

struct DummyCb {
    state: HandlerCallbackState,
}
impl HandlerCallback for DummyCb {
    type DiagramPair = ();
    type Blip = ();
    type Color = ();
    type Hyperlink = Hyperlink;
    type Media = ();
    type AudioFile = ();
    type QuickTimeFile = ();
    type VideoFile = ();
    type EmbeddedWAVAudioFile = ();
    type Model3D = ();
    type Shape = ();
    type ShapePr = ShapePr;
    type ShapeStyle = ShapeStyle;
    type TextRun = ();
    type TagLst = ();
    type FileManager = FileManager;
    fn callback_state(&self) -> &HandlerCallbackState {
        &self.state
    }
}

#[test]
fn sw1_default_crate_exports_sw_shape_handler() {
    let mut store = DgStore::DgStore_new();
    let gid = store.add(1);
    let dummy = DummySlide::with_store(&mut store);
    let kmo = Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int_int(
        &dummy, 1, gid,
    )));
    let _h = SwShapeHandler::<DummyCb>::SwShapeHandler(kmo, None);
}

#[test]
fn sw2_end_element_updates_store() {
    let mut store = DgStore::DgStore_new();
    let gid = store.add(1);
    let dummy = DummySlide::with_store(&mut store);
    assert_eq!(store.query(gid).asInt(), 0);
    let kmo = Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int_int(
        &dummy, 1, gid,
    )));
    let mut h = SwShapeHandler::<DummyCb>::SwShapeHandler(kmo, None);
    h.endElement(0);
    // rust: ★③-5 —— 断言意图不变：`endElement` 把形状**提交**进仓（Java `this.store.update(id, save())`）。
    //   ⚠ 原断言 `== 1` 比的是 `store.update` 写入的 `bh`，而真身 `Shape::save()` 返回的是
    //   **`KpTablePool` 的表句柄**（`KpMap::save` → `KpTablePool::with_builder(..).bh()`）。
    //   该句柄是**分配序相关**的池编号（`KpTablePool::MAP_BUILDER` 是 `thread_local`），
    //   其数值随测试线程的分配顺序浮动 —— 实测本 target 连跑 6 次出现 `1` / `2` 两种取值
    //   （4 次 FAIL、2 次 PASS）。Java 侧同样只保证「句柄指向已提交的表」，不保证具体编号。
    //   故改为断言**同一事实的可稳定判定形式**：提交前为初始空值 `0`（上方已断言），
    //   提交后**不再是 0** —— 即"仓被真实更新"。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let committed = store.query(gid).asInt();
    assert_ne!(committed, 0, "endElement 必须把形状提交进仓（提交前初始值为 0）");
}

#[test]
fn sw3_end_element_updates_protocol() {
    let mut store = DgStore::DgStore_new();
    let gid = store.add(1);
    let dummy = DummySlide::with_store(&mut store);
    let kmo = Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int_int(
        &dummy, 1, gid,
    )));
    let mut h = SwShapeHandler::<DummyCb>::SwShapeHandler(kmo.clone(), None);
    h.endElement(0);
    let _ = kmo.borrow().protocol();
}

#[test]
fn sw4_init_puts_type_and_id() {
    let mut store = DgStore::DgStore_new();
    let gid = store.add(1);
    let dummy = DummySlide::with_store(&mut store);
    let kmo = Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int_int(
        &dummy, 1, gid,
    )));
    let h = SwShapeHandler::<DummyCb>::SwShapeHandler(kmo, None);
    // rust: ★③-5 —— 断言意图不变（Java `this.dgShape.putType(1); cNvPr().putId(shape.id())`），
    //   但真身 `drawing::data::shape::Shape` 的字段是**私有 + 惰性装配**：原影子面的
    //   `dgShape.shape_type` / `dgShape.cnv.id` 直取字段，真身须走 getter
    //   `type()`（`Shape.rs:808`）与 `cNvPr().id()`（`Shape.rs:842`/`CNvPr.rs:444`）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_eq!(h.dgShape.borrow().r#type(), 1);
    assert_eq!(h.dgShape.borrow().cNvPr().id(), gid);
}
