#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::cell::RefCell;
use std::rc::Rc;

#[path = "../../../../../../../src/cn/wps/show/pptxr/handler/slidemasters/HeaderFooterHandler.rs"]
pub mod hf_impl;
#[path = "../../../../../../../src/cn/wps/show/pptxr/handler/slidemasters/TextStylesHandler.rs"]
pub mod tx_impl;
#[path = "../../../../../../../src/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.rs"]
pub mod master_impl;

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
                                self.to_string().parse().unwrap_or(0)
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
                                _m: Rc<RefCell<ColorMapping>>,
                            }
                            impl ColorMapHandler {
                                pub fn new(m: Rc<RefCell<ColorMapping>>) -> Self {
                                    Self { _m: m }
                                }
                            }
                            impl ElementHandler for ColorMapHandler {
                                fn kind(&self) -> &'static str {
                                    "ColorMapHandler"
                                }
                            }
                        }
                    }
                    pub mod shape {
                        pub mod textshape {
                            pub mod text {
                                pub mod TextListStyleHandler {
                                    use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
                                    use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
                                    use std::cell::RefCell;
                                    use std::rc::Rc;
                                    pub struct TextListStyleHandler<C: HandlerCallback> {
                                        kind: &'static str,
                                        _cb: Option<Rc<RefCell<C>>>,
                                    }
                                    impl<C: HandlerCallback> TextListStyleHandler<C> {
                                        pub fn new(
                                            _style: &'static str,
                                            cb: Option<Rc<RefCell<C>>>,
                                            slot: &'static str,
                                        ) -> Self {
                                            Self {
                                                kind: match slot {
                                                    "title" => "TextListStyleHandler:title",
                                                    "body" => "TextListStyleHandler:body",
                                                    _ => "TextListStyleHandler:other",
                                                },
                                                _cb: cb,
                                            }
                                        }
                                    }
                                    impl<C: HandlerCallback> ElementHandler
                                        for TextListStyleHandler<C>
                                    {
                                        fn kind(&self) -> &'static str {
                                            self.kind
                                        }
                                    }
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
                    pub mod KmoSlideMaster {
                        use super::KmoFilm::KmoFilm;
                        use crate::cn::wps::show::data::slidemasters::SlideMaster::ProtocolBin;
                        use std::cell::RefCell;
                        use std::rc::Rc;
                        pub struct KmoSlideMaster {
                            film: Rc<RefCell<KmoFilm>>,
                            pub protocol_save_count: usize,
                        }
                        impl KmoSlideMaster {
                            pub fn new() -> Rc<RefCell<Self>> {
                                Rc::new(RefCell::new(Self {
                                    film: KmoFilm::new(),
                                    protocol_save_count: 0,
                                }))
                            }
                            pub fn film(&self) -> Rc<RefCell<KmoFilm>> {
                                self.film.clone()
                            }
                            pub fn protocol(&self) -> ProtocolBin {
                                ProtocolBin
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
                pub mod slidemasters {
                    pub mod SlideMaster {
                        use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
                        use crate::cn::wps::show::data::HeaderFooter::HeaderFooter;
                        use std::cell::RefCell;
                        use std::rc::Rc;
                        #[derive(Clone, Default)]
                        pub struct Timing;
                        #[derive(Clone, Default)]
                        pub struct Transition;
                        #[derive(Default, Clone, Copy)]
                        pub struct ProtocolBin;
                        pub struct SlideMaster {
                            pub preserve: bool,
                            pub save_count: usize,
                            hf: Rc<RefCell<HeaderFooter>>,
                            clr_map: Rc<RefCell<ColorMapping>>,
                        }
                        impl Default for SlideMaster {
                            fn default() -> Self {
                                Self {
                                    preserve: false,
                                    save_count: 0,
                                    hf: HeaderFooter::create(),
                                    clr_map: Rc::new(RefCell::new(ColorMapping)),
                                }
                            }
                        }
                        impl SlideMaster {
                            pub fn open(_bin: ProtocolBin) -> Rc<RefCell<Self>> {
                                Rc::new(RefCell::new(Self::default()))
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
                            pub fn titleStyle(&self) -> &'static str {
                                "title"
                            }
                            pub fn bodyStyle(&self) -> &'static str {
                                "body"
                            }
                            pub fn otherStyle(&self) -> &'static str {
                                "other"
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
                pub mod handler {
                    pub mod slides {
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
                        pub(crate) use crate::tx_impl as TextStylesHandler;
                    }
                    pub mod timing {
                        pub mod TimingHandler {
                            use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
                            use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
                            use crate::cn::wps::show::data::slidemasters::SlideMaster::Timing;
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
                            use crate::cn::wps::show::data::slidemasters::SlideMaster::Transition;
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
                            use crate::cn::wps::show::data::slidemasters::SlideMaster::{
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
use cn::wps::show::app::slide::KmoSlideMaster::KmoSlideMaster;
use hf_impl::HeaderFooterHandler;
use master_impl::{MasterHandlerCallback, SlideMasterHandler};
use cn::wps::show::data::HeaderFooter::HeaderFooter;

#[derive(Default)]
struct RecordingCallback {
    state: HandlerCallbackState,
    layout_ids: Vec<(String, i64)>,
    id_calls: usize,
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
impl MasterHandlerCallback for RecordingCallback {
    fn onSlideLayoutIds(&mut self, ids: &[(String, i64)]) {
        self.id_calls += 1;
        self.layout_ids = ids.to_vec();
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

struct NamedAttrs {
    values: Vec<AttrValue>,
}
impl NamedAttrs {
    fn from_pairs(pairs: &[(i32, &str)]) -> Self {
        Self {
            values: pairs
                .iter()
                .map(|(n, v)| AttrValue {
                    name: *n,
                    value: (*v).to_string(),
                })
                .collect(),
        }
    }
}
impl XmlRoAttrValue for NamedAttrs {
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
impl XmlRoAttr for NamedAttrs {
    fn get_by_name(&self, name: i32) -> Option<&dyn XmlRoAttr> {
        self.values
            .iter()
            .find(|a| a.name == name)
            .map(|a| a as &dyn XmlRoAttr)
    }
    fn get_attr_at(&self, index: i32) -> Option<&dyn XmlRoAttr> {
        self.values
            .get(index as usize)
            .map(|a| a as &dyn XmlRoAttr)
    }
    fn count(&self) -> i32 {
        self.values.len() as i32
    }
    fn get_name(&self) -> i32 {
        0
    }
}

fn cb() -> Rc<RefCell<RecordingCallback>> {
    Rc::new(RefCell::new(RecordingCallback {
        state: HandlerCallback__mode(0),
        layout_ids: Vec::new(),
        id_calls: 0,
    }))
}

fn new_master() -> (
    Rc<RefCell<KmoSlideMaster>>,
    Rc<RefCell<RecordingCallback>>,
    SlideMasterHandler<RecordingCallback>,
) {
    let master = KmoSlideMaster::new();
    let callback = cb();
    let handler = SlideMasterHandler::SlideMasterHandler(
        master.clone(),
        Rc::new(RefCell::new(DgStore)),
        callback.clone(),
    );
    (master, callback, handler)
}

#[test]
fn preserve_attr_present_and_absent() {
    let (_m, _c, mut h) = new_master();
    h.startElement(0, &NamedAttrs::from_pairs(&[(3211613, "true")]));
    assert!(h.masterData.borrow().preserve);
    let (_m, _c, mut h) = new_master();
    h.startElement(0, &NamedAttrs::from_pairs(&[]));
    assert!(!h.masterData.borrow().preserve);
}

#[test]
fn dispatches_master_children() {
    let cases = [
        (2162692, Some("TransitionAlternateContentHandler")),
        (3211295, None),
        (3211402, Some("TransitionHandler")),
        (3211598, Some("ColorMapHandler")),
        (3211608, Some("CommonSlideDataHandler")),
        (3211609, Some("TimingHandler")),
        (3211611, Some("HeaderFooterHandler")),
        (3211619, Some("IdListHandler")),
        (3211620, Some("TextStylesHandler")),
        (-1, None),
    ];
    for (id, kind) in cases {
        let (_m, _c, mut h) = new_master();
        let child = h.enterSubElement(id);
        match kind {
            None => assert!(child.is_none()),
            Some(k) => assert_eq!(child.expect("child").kind(), k),
        }
    }
}

#[test]
fn outer_end_saves_protocol() {
    let (master, _c, mut h) = new_master();
    h.endElement(0);
    assert_eq!(master.borrow().protocol_save_count, 1);
}

#[test]
fn id_list_collects_rid_and_pid() {
    let (_m, cb, mut h) = new_master();
    let mut ids = h.enterSubElement(3211619).expect("id list");
    ids.startElement(3211618, &NamedAttrs::from_pairs(&[(4259841, "rId1"), (3211342, "9")]));
    ids.endElement(3211619);
    assert_eq!(cb.borrow().id_calls, 1);
    assert_eq!(cb.borrow().layout_ids, vec![("rId1".into(), 9)]);
}

#[test]
fn id_list_missing_pid_and_two_entries() {
    let (_m, cb, mut h) = new_master();
    let mut ids = h.enterSubElement(3211619).expect("id list");
    ids.startElement(3211618, &NamedAttrs::from_pairs(&[(4259841, "rIdA")]));
    ids.startElement(3211618, &NamedAttrs::from_pairs(&[(4259841, "rIdB"), (3211342, "2")]));
    ids.startElement(0, &NamedAttrs::from_pairs(&[(4259841, "ignore")]));
    ids.endElement(3211619);
    assert_eq!(
        cb.borrow().layout_ids,
        vec![("rIdA".into(), 0), ("rIdB".into(), 2)]
    );
}

#[test]
fn id_list_end_other_element_does_not_callback() {
    let (_m, cb, mut h) = new_master();
    let mut ids = h.enterSubElement(3211619).expect("id list");
    ids.startElement(3211618, &NamedAttrs::from_pairs(&[(4259841, "rId1"), (3211342, "1")]));
    ids.endElement(1);
    assert_eq!(cb.borrow().id_calls, 0);
}

#[test]
fn header_footer_ctor_defaults_then_override_slide_num() {
    let hf = HeaderFooter::create();
    let mut handler = HeaderFooterHandler::HeaderFooterHandler(Some(hf.clone()));
    {
        let v = hf.borrow();
        assert!(v.date_time && v.header && v.footer && v.slide_num);
    }
    handler.startElement(0, &NamedAttrs::from_pairs(&[(3211566, "false")]));
    assert!(!hf.borrow().slide_num);
    assert!(hf.borrow().header);
}

#[test]
fn text_styles_three_slots_and_unknown() {
    let (_m, _c, mut h) = new_master();
    let mut tx = h.enterSubElement(3211620).expect("txStyles");
    assert_eq!(
        tx.enterSubElement(3211615).expect("title").kind(),
        "TextListStyleHandler:title"
    );
    assert_eq!(
        tx.enterSubElement(3211616).expect("body").kind(),
        "TextListStyleHandler:body"
    );
    assert_eq!(
        tx.enterSubElement(3211617).expect("other").kind(),
        "TextListStyleHandler:other"
    );
    assert!(tx.enterSubElement(-1).is_none());
}
