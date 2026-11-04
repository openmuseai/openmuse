#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

//! 06_function_slide_layout —— `SlideLayoutHandler` 的 p:sldLayout 派发表。
//!
//! ★M-PPT-KMOSLIDE W6c-3：原先本 target 还 `#[path]` 内联了
//!   `src/.../slides/ColorMapOverrideHandler.rs`（真实类），仅用于断言 3211599 的 `kind()`。
//!   该真实类在本波已**真实登记**并实现 **② dmlr 契约**（`&mut self` + `&XmlRoAttrList`），
//!   而本 target 的假模块树用的是自己的 `kfc::xpxml::reader::ElementHandler`（`&dyn XmlRoAttr`）
//!   ⇒ `#[path]` 内联的写法在**契约签名层面**无法共存（「假树里内联真身源文件」的结构性限制，
//!   见基线 §11.10）。故此处不再内联真身，改用本 target 假树内的同名替身（与 `ColorMapHandler`
//!   等已有替身同一处置），真身的 `kind()` 与 1114458→`ColorMapHandler` 派发由
//!   `tests/cn/wps/show/pptxr/handler/slides/ColorMapOverrideHandler.rs` 直接驱动真身验证。
//!   owner: tests.pptxr.handler.slidelayouts    wave: M-PPT-KMOSLIDE-W6c-3

use std::cell::RefCell;
use std::rc::Rc;

#[path = "../../../../../../../src/cn/wps/show/pptxr/handler/slidemasters/HeaderFooterHandler.rs"]
pub mod hf_impl;
#[path = "../../../../../../../src/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs"]
pub mod layout_impl;

pub mod cn {
    pub mod wps {
        pub mod kfc {
            pub mod xpxml {
                pub mod reader {
                    pub mod XmlRoAttrValue {
                        pub trait XmlRoAttrValue {
                            fn to_string(&self) -> String;
                            fn to_int(&self) -> i32;
                            fn to_boolean(&self) -> bool;
                            fn to_long(&self) -> i64 {
                                self.to_int() as i64
                            }
                        }
                    }
                    pub mod XmlRoAttr {
                        use super::XmlRoAttrValue::XmlRoAttrValue;
                        pub trait XmlRoAttr: XmlRoAttrValue {
                            fn get_by_name(&self, name: i32) -> Option<&dyn XmlRoAttr>;
                            fn get_attr_at(&self, index: i32) -> Option<&dyn XmlRoAttr>;
                            fn count(&self) -> i32;
                            fn get_name(&self) -> i32;
                        }
                    }
                    pub mod ElementHandler {
                        use super::XmlRoAttr::XmlRoAttr;
                        pub trait ElementHandler {
                            fn startElement(&mut self, _e: i32, _a: &dyn XmlRoAttr) {}
                            fn endElement(&mut self, _e: i32) {}
                            fn enterSubElement(
                                &mut self,
                                _e: i32,
                            ) -> Option<Box<dyn ElementHandler>> {
                                None
                            }
                            fn kind(&self) -> &'static str {
                                "ElementHandler"
                            }
                        }
                    }
                }
            }
        }
        pub mod drawing {
            pub mod app {
                pub mod DgStore {
                    #[derive(Default)]
                    pub struct DgStore;
                }
            }
            pub mod data {
                pub mod style {
                    pub mod ColorMapping {
                        #[derive(Default)]
                        pub struct ColorMapping;
                    }
                }
            }
            pub mod dmlr {
                pub mod model {
                    pub mod HandlerCallback {
                        pub use ppt_core::cn::wps::drawing::dmlr::model::HandlerCallback::*;
                    }
                }
                pub mod handler {
                    pub mod style {
                        pub mod ColorMapHandler {
                            use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
                            use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
                            use std::cell::RefCell;
                            use std::rc::Rc;
                            pub struct ColorMapHandler {
                                _map: Rc<RefCell<ColorMapping>>,
                            }
                            impl ColorMapHandler {
                                pub fn new(map: Rc<RefCell<ColorMapping>>) -> Self {
                                    Self { _map: map }
                                }
                            }
                            impl ElementHandler for ColorMapHandler {
                                fn kind(&self) -> &'static str {
                                    "ColorMapHandler"
                                }
                            }
                        }
                    }
                }
            }
        }
        pub mod show {
            pub mod app {
                pub mod slide {
                    pub mod KmoFilm {
                        use std::cell::RefCell;
                        use std::rc::Rc;
                        #[derive(Default)]
                        pub struct KmoFilm;
                        impl KmoFilm {
                            pub fn new() -> Rc<RefCell<Self>> {
                                Rc::new(RefCell::new(Self))
                            }
                        }
                    }
                    pub mod KmoSlideLayout {
                        use super::KmoFilm::KmoFilm;
                        use crate::cn::wps::show::data::slidelayouts::SlideLayout::ProtocolBin;
                        use std::cell::RefCell;
                        use std::rc::Rc;
                        pub struct KmoSlideLayout {
                            film: Rc<RefCell<KmoFilm>>,
                            pub protocol_save_count: usize,
                        }
                        impl KmoSlideLayout {
                            pub fn new() -> Rc<RefCell<Self>> {
                                Rc::new(RefCell::new(Self {
                                    film: KmoFilm::new(),
                                    protocol_save_count: 0,
                                }))
                            }
                            pub fn film(&self) -> Rc<RefCell<KmoFilm>> {
                                self.film.clone()
                            }
                            pub fn updateProtocol(&mut self, _bin: ProtocolBin) {
                                self.protocol_save_count += 1;
                            }
                        }
                    }
                }
            }
            pub mod data {
                pub mod HeaderFooter {
                    use std::cell::RefCell;
                    use std::rc::Rc;
                    #[derive(Default)]
                    pub struct HeaderFooter {
                        pub date_time: bool,
                        pub header: bool,
                        pub footer: bool,
                        pub slide_num: bool,
                    }
                    impl HeaderFooter {
                        pub fn create() -> Rc<RefCell<Self>> {
                            Rc::new(RefCell::new(Self::default()))
                        }
                        pub fn putDateTime(&mut self, v: bool) {
                            self.date_time = v;
                        }
                        pub fn putHeader(&mut self, v: bool) {
                            self.header = v;
                        }
                        pub fn putFooter(&mut self, v: bool) {
                            self.footer = v;
                        }
                        pub fn putSlideNum(&mut self, v: bool) {
                            self.slide_num = v;
                        }
                    }
                }
                pub mod slidelayouts {
                    pub mod SlideLayout {
                        use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
                        use crate::cn::wps::show::data::HeaderFooter::HeaderFooter;
                        use std::cell::RefCell;
                        use std::rc::Rc;
                        #[derive(Clone, Default)]
                        pub struct Timing;
                        #[derive(Clone, Default)]
                        pub struct Transition;
                        #[derive(Default)]
                        pub struct ProtocolBin;
                        pub struct SlideLayout {
                            pub layout_type: i32,
                            pub user_drawn: bool,
                            pub show_master_sp: bool,
                            pub show_master_ph_anim: bool,
                            pub matching_name: String,
                            pub preserve: bool,
                            pub save_count: usize,
                            hf: Rc<RefCell<HeaderFooter>>,
                            clr_map: Rc<RefCell<ColorMapping>>,
                        }
                        impl Default for SlideLayout {
                            fn default() -> Self {
                                Self {
                                    layout_type: -1,
                                    user_drawn: false,
                                    show_master_sp: true,
                                    show_master_ph_anim: true,
                                    matching_name: String::new(),
                                    preserve: false,
                                    save_count: 0,
                                    hf: HeaderFooter::create(),
                                    clr_map: Rc::new(RefCell::new(ColorMapping)),
                                }
                            }
                        }
                        impl SlideLayout {
                            pub fn create() -> Rc<RefCell<Self>> {
                                Rc::new(RefCell::new(Self::default()))
                            }
                            pub fn putType(&mut self, v: i32) {
                                self.layout_type = v;
                            }
                            pub fn putUserDraw(&mut self, v: bool) {
                                self.user_drawn = v;
                            }
                            pub fn putShowMasterSp(&mut self, v: bool) {
                                self.show_master_sp = v;
                            }
                            pub fn putShowMasterPhAnim(&mut self, v: bool) {
                                self.show_master_ph_anim = v;
                            }
                            pub fn putMatchingName(&mut self, v: String) {
                                self.matching_name = v;
                            }
                            pub fn putPreserve(&mut self, v: bool) {
                                self.preserve = v;
                            }
                            pub fn timing(&self) -> Timing {
                                Timing
                            }
                            pub fn transition(&self) -> Transition {
                                Transition
                            }
                            pub fn clrMap(&self) -> Rc<RefCell<ColorMapping>> {
                                self.clr_map.clone()
                            }
                            pub fn hf(&self) -> Rc<RefCell<HeaderFooter>> {
                                self.hf.clone()
                            }
                            pub fn save(&mut self) -> ProtocolBin {
                                self.save_count += 1;
                                ProtocolBin
                            }
                        }
                    }
                }
            }
            pub mod pptxr {
                pub mod util {
                    pub mod SWReaderMap {
                        pub use ppt_core::cn::wps::show::pptxr::util::SWReaderMap::SWReaderMap;
                    }
                }
                pub mod handler {
                    pub mod slides {
                        // rust: ★W6c-3 —— 本 target 假树内的同名替身（真身实现 ② 契约，
                        //   签名与本假树契约不同，无法内联；见文件头）。真身由
                        //   `tests/.../handler/slides/ColorMapOverrideHandler.rs` 直接验证。
                        pub mod ColorMapOverrideHandler {
                            use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
                            use std::cell::RefCell;
                            use std::rc::Rc;
                            pub struct ColorMapOverrideHandler {
                                _map: Rc<RefCell<crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping>>,
                            }
                            impl ColorMapOverrideHandler {
                                pub fn ColorMapOverrideHandler(
                                    map: Rc<RefCell<crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping>>,
                                ) -> Self {
                                    Self { _map: map }
                                }
                            }
                            impl ElementHandler for ColorMapOverrideHandler {
                                fn kind(&self) -> &'static str {
                                    "ColorMapOverrideHandler"
                                }
                            }
                        }
                        pub mod CommonSlideDataHandler {
                            use crate::cn::wps::drawing::app::DgStore::DgStore;
                            use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
                            use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
                            use crate::cn::wps::show::app::slide::KmoFilm::KmoFilm;
                            use std::cell::RefCell;
                            use std::rc::Rc;
                            pub struct CommonSlideDataHandler<C: HandlerCallback> {
                                _f: Rc<RefCell<KmoFilm>>,
                                _s: Rc<RefCell<DgStore>>,
                                _c: Option<Rc<RefCell<C>>>,
                            }
                            impl<C: HandlerCallback> CommonSlideDataHandler<C> {
                                pub fn CommonSlideDataHandler(
                                    f: Rc<RefCell<KmoFilm>>,
                                    s: Rc<RefCell<DgStore>>,
                                    c: Option<Rc<RefCell<C>>>,
                                ) -> Self {
                                    Self {
                                        _f: f,
                                        _s: s,
                                        _c: c,
                                    }
                                }
                            }
                            impl<C: HandlerCallback> ElementHandler for CommonSlideDataHandler<C> {
                                fn kind(&self) -> &'static str {
                                    "CommonSlideDataHandler"
                                }
                            }
                        }
                    }
                    pub mod slidemasters {
                        pub(crate) use crate::hf_impl as HeaderFooterHandler;
                    }
                    pub mod timing {
                        pub mod TimingHandler {
                            use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
                            use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
                            use crate::cn::wps::show::data::slidelayouts::SlideLayout::Timing;
                            use std::cell::RefCell;
                            use std::rc::Rc;
                            pub struct TimingHandler<C: HandlerCallback> {
                                _t: Timing,
                                _c: Option<Rc<RefCell<C>>>,
                            }
                            impl<C: HandlerCallback> TimingHandler<C> {
                                pub fn new(t: Timing, c: Option<Rc<RefCell<C>>>) -> Self {
                                    Self { _t: t, _c: c }
                                }
                            }
                            impl<C: HandlerCallback> ElementHandler for TimingHandler<C> {
                                fn kind(&self) -> &'static str {
                                    "TimingHandler"
                                }
                            }
                        }
                    }
                    pub mod transition {
                        pub mod TransitionHandler {
                            use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
                            use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
                            use crate::cn::wps::show::data::slidelayouts::SlideLayout::Transition;
                            use std::cell::RefCell;
                            use std::rc::Rc;
                            pub struct TransitionHandler<C: HandlerCallback> {
                                _t: Transition,
                                _c: Option<Rc<RefCell<C>>>,
                            }
                            impl<C: HandlerCallback> TransitionHandler<C> {
                                pub fn new(t: Transition, c: Option<Rc<RefCell<C>>>) -> Self {
                                    Self { _t: t, _c: c }
                                }
                            }
                            impl<C: HandlerCallback> ElementHandler for TransitionHandler<C> {
                                fn kind(&self) -> &'static str {
                                    "TransitionHandler"
                                }
                            }
                        }
                        pub mod TransitionAlternateContentHandler {
                            use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
                            use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
                            use crate::cn::wps::show::data::slidelayouts::SlideLayout::{
                                Timing, Transition,
                            };
                            use std::cell::RefCell;
                            use std::rc::Rc;
                            pub struct TransitionAlternateContentHandler<C: HandlerCallback> {
                                _a: Timing,
                                _b: Transition,
                                _c: Option<Rc<RefCell<C>>>,
                            }
                            impl<C: HandlerCallback> TransitionAlternateContentHandler<C> {
                                pub fn new(
                                    a: Timing,
                                    b: Transition,
                                    c: Option<Rc<RefCell<C>>>,
                                ) -> Self {
                                    Self {
                                        _a: a,
                                        _b: b,
                                        _c: c,
                                    }
                                }
                            }
                            impl<C: HandlerCallback> ElementHandler
                                for TransitionAlternateContentHandler<C>
                            {
                                fn kind(&self) -> &'static str {
                                    "TransitionAlternateContentHandler"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

use cn::wps::drawing::app::DgStore::DgStore;
use cn::wps::drawing::dmlr::model::HandlerCallback::{
    HandlerCallback, HandlerCallbackState, HandlerCallback__mode,
};
use cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
use cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use cn::wps::kfc::xpxml::reader::XmlRoAttrValue::XmlRoAttrValue;
use cn::wps::show::app::slide::KmoSlideLayout::KmoSlideLayout;
use layout_impl::SlideLayoutHandler;

#[derive(Default)]
struct RecordingCallback {
    state: HandlerCallbackState,
}
impl HandlerCallback for RecordingCallback {
    type DiagramPair = ();
    type Blip = ();
    type Color = ();
    type Hyperlink = ();
    type Media = ();
    type AudioFile = ();
    type QuickTimeFile = ();
    type VideoFile = ();
    type EmbeddedWAVAudioFile = ();
    type Model3D = ();
    type Shape = ();
    type ShapePr = ();
    type ShapeStyle = ();
    type TextRun = ();
    type TagLst = ();
    type FileManager = ();
    fn callback_state(&self) -> &HandlerCallbackState {
        &self.state
    }
}

#[derive(Clone)]
struct AttrValue {
    name: i32,
    value: String,
}
impl XmlRoAttrValue for AttrValue {
    fn to_string(&self) -> String {
        self.value.clone()
    }
    fn to_int(&self) -> i32 {
        self.value.parse().unwrap_or(0)
    }
    fn to_boolean(&self) -> bool {
        matches!(self.value.as_str(), "true" | "1")
    }
}
impl XmlRoAttr for AttrValue {
    fn get_by_name(&self, _n: i32) -> Option<&dyn XmlRoAttr> {
        None
    }
    fn get_attr_at(&self, _i: i32) -> Option<&dyn XmlRoAttr> {
        None
    }
    fn count(&self) -> i32 {
        0
    }
    fn get_name(&self) -> i32 {
        self.name
    }
}

struct IndexedAttrs {
    slots: Vec<Option<AttrValue>>,
}
impl IndexedAttrs {
    fn from_named(pairs: &[(i32, &str)]) -> Self {
        Self {
            slots: pairs
                .iter()
                .map(|(n, v)| {
                    Some(AttrValue {
                        name: *n,
                        value: (*v).to_string(),
                    })
                })
                .collect(),
        }
    }
}
impl XmlRoAttrValue for IndexedAttrs {
    fn to_string(&self) -> String {
        String::new()
    }
    fn to_int(&self) -> i32 {
        0
    }
    fn to_boolean(&self) -> bool {
        false
    }
}
impl XmlRoAttr for IndexedAttrs {
    fn get_by_name(&self, name: i32) -> Option<&dyn XmlRoAttr> {
        self.slots
            .iter()
            .filter_map(|s| s.as_ref())
            .find(|a| a.name == name)
            .map(|a| a as &dyn XmlRoAttr)
    }
    fn get_attr_at(&self, index: i32) -> Option<&dyn XmlRoAttr> {
        self.slots
            .get(index as usize)
            .and_then(|s| s.as_ref().map(|a| a as &dyn XmlRoAttr))
    }
    fn count(&self) -> i32 {
        self.slots.len() as i32
    }
    fn get_name(&self) -> i32 {
        0
    }
}

fn cb() -> Option<Rc<RefCell<RecordingCallback>>> {
    Some(Rc::new(RefCell::new(RecordingCallback {
        state: HandlerCallback__mode(0),
    })))
}

fn new_layout() -> (
    Rc<RefCell<KmoSlideLayout>>,
    SlideLayoutHandler<RecordingCallback>,
) {
    let layout = KmoSlideLayout::new();
    let handler = SlideLayoutHandler::SlideLayoutHandler(
        layout.clone(),
        Rc::new(RefCell::new(DgStore)),
        cb(),
    );
    (layout, handler)
}

#[test]
fn maps_known_layout_type_title() {
    let (_l, mut h) = new_layout();
    h.startElement(0, &IndexedAttrs::from_named(&[(3211302, "title")]));
    assert_eq!(h.layoutData.borrow().layout_type, 0);
}

#[test]
fn unknown_layout_type_writes_twenty_three() {
    let (_l, mut h) = new_layout();
    h.startElement(0, &IndexedAttrs::from_named(&[(3211302, "not-a-layout")]));
    assert_eq!(h.layoutData.borrow().layout_type, 23);
}

#[test]
fn writes_remaining_layout_attributes() {
    let (_l, mut h) = new_layout();
    h.startElement(
        0,
        &IndexedAttrs::from_named(&[
            (3211573, "true"),
            (3211600, "false"),
            (3211601, "false"),
            (3211612, "Match"),
            (3211613, "true"),
        ]),
    );
    let d = h.layoutData.borrow();
    assert!(d.user_drawn);
    assert!(!d.show_master_sp);
    assert!(!d.show_master_ph_anim);
    assert_eq!(d.matching_name, "Match");
    assert!(d.preserve);
}

#[test]
fn empty_attrs_keep_create_defaults() {
    let (_l, mut h) = new_layout();
    h.startElement(0, &IndexedAttrs { slots: vec![] });
    {
        let d = h.layoutData.borrow();
        assert_eq!(d.layout_type, -1);
        assert!(!d.user_drawn);
    }
    h.startElement(
        0,
        &IndexedAttrs {
            slots: vec![None, Some(AttrValue { name: 1, value: "x".into() })],
        },
    );
    assert_eq!(h.layoutData.borrow().layout_type, -1);
}

#[test]
fn dispatches_layout_children() {
    let cases = [
        (2162692, Some("TransitionAlternateContentHandler")),
        (3211295, None),
        (3211402, Some("TransitionHandler")),
        (3211599, Some("ColorMapOverrideHandler")),
        (3211608, Some("CommonSlideDataHandler")),
        (3211609, Some("TimingHandler")),
        (3211611, Some("HeaderFooterHandler")),
        (-1, None),
    ];
    for (id, kind) in cases {
        let (_l, mut h) = new_layout();
        let child = h.enterSubElement(id);
        match kind {
            None => assert!(child.is_none()),
            Some(k) => assert_eq!(child.expect("child").kind(), k),
        }
    }
}

#[test]
fn end_element_saves_protocol() {
    let (layout, mut h) = new_layout();
    h.endElement(0);
    assert_eq!(layout.borrow().protocol_save_count, 1);
    assert_eq!(h.layoutData.borrow().save_count, 1);
}
