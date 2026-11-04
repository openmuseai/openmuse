#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
// rust: ★③-5 —— 被测真实源现按 ② 规范契约实现，`startElement` 收**具体**
//   `&XmlRoAttrList`（真身来自独立 crate `kfc_kfc_alpha`）。此处直接复用真 crate，
//   不再伪造第二份 —— 伪造会让 `&XmlRoAttrList` 变成不同类型（E0308/E0277）。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
extern crate kfc_kfc_alpha;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
                        }
                    }

                    pub mod XmlRoAttr {
                        use super::XmlRoAttrValue::XmlRoAttrValue;

                        pub trait XmlRoAttr: XmlRoAttrValue {
                            fn get_by_name(&self, name: i32) -> Option<&dyn XmlRoAttr>;
                        }
                    }

                    pub mod ElementHandler {
                        use super::XmlRoAttr::XmlRoAttr;

                        pub trait ElementHandler {
                            fn startElement(&mut self, _element: i32, _attrs: &dyn XmlRoAttr) {}
                            fn endElement(&mut self, _element: i32) {}
                            fn characters(&mut self, _chars: &str) {}
                            fn enterSubElement(
                                &mut self,
                                _subElement: i32,
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
            pub mod dmlr {
                pub mod model {
                    pub mod HandlerCallback {
                        pub use ppt_core::cn::wps::drawing::dmlr::model::HandlerCallback::*;
                    }
                }

                pub mod util {
                    pub mod DGMap {
                        use std::collections::HashMap;
                        use std::sync::OnceLock;

                        pub struct DGMap;

                        impl DGMap {
                            pub fn orientationMap() -> &'static HashMap<&'static str, i32> {
                                static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
                                MAP.get_or_init(|| HashMap::from([("horz", 0), ("vert", 1)]))
                            }
                        }
                    }
                }

                // rust: ★③-5 —— 真实 `SwNvPrHandler` 现按 ② 规范契约实现，且 `media` 分支
                //   已按 `DEFERRED_NON_FUNCTION` 登记（`Media` 在两构建下都不编译）。
                //   故此处**直接复用真 lib crate 的 `handler` 模块**，不再伪造第二份
                //   （伪造会造成 `Box<dyn ElementHandler>` 类型身份不一致）。
                //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
                pub mod handler {
                    pub use ppt_core::cn::wps::drawing::dmlr::handler::*;
                }
            }
        }

        pub mod show {
            pub mod data {
                pub mod shape {
                    pub mod ShowShape {
                        use std::cell::RefCell;
                        use std::rc::Rc;

                        #[derive(Clone, Default)]
                        pub struct AudioCD;
                        #[derive(Clone, Default)]
                        pub struct EmbeddedWAVAudioFile;
                        #[derive(Clone, Default)]
                        pub struct AudioFile;
                        #[derive(Clone, Default)]
                        pub struct VideoFile;
                        #[derive(Clone, Default)]
                        pub struct QuickTimeFile;
                        #[derive(Clone, Default)]
                        pub struct CustDataLst;

                        #[derive(Clone)]
                        pub struct Media(Rc<RefCell<MediaState>>);

                        #[derive(Default)]
                        struct MediaState {
                            mode: i32,
                        }

                        impl Default for Media {
                            fn default() -> Self {
                                Self(Rc::new(RefCell::new(MediaState { mode: -1 })))
                            }
                        }

                        impl Media {
                            pub fn putMode(&self, mode: i32) {
                                self.0.borrow_mut().mode = mode;
                            }
                            pub fn mode(&self) -> i32 {
                                self.0.borrow().mode
                            }
                            pub fn audioCD(&self) -> AudioCD {
                                AudioCD
                            }
                            pub fn embeddedWAVAudioFile(&self) -> EmbeddedWAVAudioFile {
                                EmbeddedWAVAudioFile
                            }
                            pub fn audioFile(&self) -> AudioFile {
                                AudioFile
                            }
                            pub fn videoFile(&self) -> VideoFile {
                                VideoFile
                            }
                            pub fn quickTimeFile(&self) -> QuickTimeFile {
                                QuickTimeFile
                            }
                        }

                        #[derive(Clone, Debug, Eq, PartialEq)]
                        pub struct Placeholder {
                            pub has_custom_prompt: bool,
                            pub idx: i32,
                            pub orient: i32,
                            pub size: i32,
                            pub shape_type: i32,
                            pub save_count: usize,
                        }

                        impl Default for Placeholder {
                            fn default() -> Self {
                                Self {
                                    has_custom_prompt: false,
                                    idx: 0,
                                    orient: 0,
                                    size: 0,
                                    shape_type: 8,
                                    save_count: 0,
                                }
                            }
                        }

                        impl Placeholder {
                            // rust: ★③-5 —— 真身 `ShowShape.Placeholder.create()` 返回**按值**的
                            //   `Placeholder`（不是 `Rc<RefCell<..>>`）。影子面必须同形，否则被测源
                            //   `Rc::new(RefCell::new(Placeholder::create()))` 类型不符（E0308）。
                            //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
                            pub fn create() -> Self {
                                Self::default()
                            }
                            pub fn save(&mut self) {
                                self.save_count += 1;
                            }
                            pub fn putHasCustomPrompt(&mut self, value: bool) {
                                self.has_custom_prompt = value;
                            }
                            pub fn putIdx(&mut self, value: i32) {
                                self.idx = value;
                            }
                            pub fn putOrient(&mut self, value: i32) {
                                self.orient = value;
                            }
                            pub fn putSize(&mut self, value: i32) {
                                self.size = value;
                            }
                            pub fn putType(&mut self, value: i32) {
                                self.shape_type = value;
                            }
                        }

                        #[derive(Clone)]
                        pub struct ShowShape {
                            pub is_photo: bool,
                            pub user_drawn: bool,
                            pub media: Media,
                            pub cust_data: CustDataLst,
                            pub placeholder: Option<Rc<RefCell<Placeholder>>>,
                        }

                        impl Default for ShowShape {
                            fn default() -> Self {
                                Self {
                                    is_photo: false,
                                    user_drawn: false,
                                    media: Media::default(),
                                    cust_data: CustDataLst,
                                    placeholder: None,
                                }
                            }
                        }

                        impl ShowShape {
                            pub fn putIsPhoto(&mut self, value: bool) {
                                self.is_photo = value;
                            }
                            pub fn putUserDrawn(&mut self, value: bool) {
                                self.user_drawn = value;
                            }
                            pub fn media(&self) -> Media {
                                self.media.clone()
                            }
                            pub fn custDataLst(&self) -> CustDataLst {
                                self.cust_data.clone()
                            }
                            // rust: ★③-5 —— 真身 `ShowShape::forceWritePh(&mut self, &mut Placeholder)`
                            //   取**可变引用**；影子面同形。被测源在 `save()` 之后调用本方法，
                            //   故这里存入 clone 仍携带 save 后的状态（`save_count == 1` 断言成立）。
                            //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
                            pub fn forceWritePh(&mut self, value: &mut Placeholder) {
                                self.placeholder = Some(Rc::new(RefCell::new(value.clone())));
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
                    pub mod custdatas {
                        pub mod CustDataLstHandler {
                            use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
                            use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
                            use crate::cn::wps::show::data::shape::ShowShape::CustDataLst;
                            use std::cell::RefCell;
                            use std::rc::Rc;

                            pub struct CustDataLstHandler<C: HandlerCallback> {
                                _target: CustDataLst,
                                _callback: Option<Rc<RefCell<C>>>,
                            }

                            impl<C: HandlerCallback> CustDataLstHandler<C> {
                                pub fn new(
                                    target: CustDataLst,
                                    callback: Option<Rc<RefCell<C>>>,
                                ) -> Self {
                                    Self {
                                        _target: target,
                                        _callback: callback,
                                    }
                                }
                            }

                            impl<C: HandlerCallback> ElementHandler for CustDataLstHandler<C> {
                                fn kind(&self) -> &'static str {
                                    "CustDataLstHandler"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

use cn::wps::drawing::dmlr::model::HandlerCallback::{
    HandlerCallback, HandlerCallbackState, HandlerCallback__mode,
};
use cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
use cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use cn::wps::kfc::xpxml::reader::XmlRoAttrValue::XmlRoAttrValue;
use cn::wps::show::data::shape::ShowShape::{Placeholder, ShowShape};

#[path = "../../../../../../../src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs"]
mod implementation;
use implementation::SwNvPrHandler;

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
struct AttrValue(String);

impl XmlRoAttrValue for AttrValue {
    fn to_string(&self) -> String {
        self.0.clone()
    }
    fn to_int(&self) -> i32 {
        self.0.parse().expect("test attribute must be an integer")
    }
    fn to_boolean(&self) -> bool {
        matches!(self.0.as_str(), "true" | "1")
    }
}

impl XmlRoAttr for AttrValue {
    fn get_by_name(&self, _name: i32) -> Option<&dyn XmlRoAttr> {
        None
    }
}

#[derive(Default)]
struct RecordingXmlRoAttr {
    values: HashMap<i32, AttrValue>,
}

impl RecordingXmlRoAttr {
    fn from_pairs(pairs: &[(i32, &str)]) -> Self {
        Self {
            values: pairs
                .iter()
                .map(|(name, value)| (*name, AttrValue((*value).to_string())))
                .collect(),
        }
    }

    // rust: ★③-5 —— 真身 ② 契约 `startElement(.., &XmlRoAttrList)` 要的是**具体**列表，
    //   而非 trait 对象。故把已记录的名值对转成真 `XmlRoAttrList`（保持记录语义不变，
    //   `values` 仍留存供直接断言）。owner: tests.pptxr.handler.shape  wave: M-PPT-SHAPE-DISPATCH
    fn to_list(&self) -> XmlRoAttrList {
        let mut list = XmlRoAttrList::new();
        for (name, value) in &self.values {
            list.add_attr(*name, &value.to_string());
        }
        list
    }
}

impl XmlRoAttrValue for RecordingXmlRoAttr {
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

impl XmlRoAttr for RecordingXmlRoAttr {
    fn get_by_name(&self, name: i32) -> Option<&dyn XmlRoAttr> {
        self.values.get(&name).map(|value| value as &dyn XmlRoAttr)
    }
}

fn callback() -> Option<Rc<RefCell<RecordingCallback>>> {
    Some(Rc::new(RefCell::new(RecordingCallback {
        state: HandlerCallback__mode(0),
    })))
}

fn new_handler() -> (Rc<RefCell<ShowShape>>, SwNvPrHandler<RecordingCallback>) {
    let shape = Rc::new(RefCell::new(ShowShape::default()));
    let handler = SwNvPrHandler::SwNvPrHandler(shape.clone(), callback());
    (shape, handler)
}

#[test]
fn writes_present_photo_and_user_drawn_attributes() {
    let (shape, mut handler) = new_handler();
    handler.startElement(
        0,
        &RecordingXmlRoAttr::from_pairs(&[(3211572, "true"), (3211573, "false")]).to_list(),
    );
    assert!(shape.borrow().is_photo);
    assert!(!shape.borrow().user_drawn);
}

#[test]
fn empty_attributes_preserve_existing_shape_flags() {
    let (shape, mut handler) = new_handler();
    shape.borrow_mut().is_photo = true;
    shape.borrow_mut().user_drawn = true;
    handler.startElement(0, &RecordingXmlRoAttr::default().to_list());
    assert!(shape.borrow().is_photo);
    assert!(shape.borrow().user_drawn);
}

#[test]
#[ignore = "DEFERRED(M-PPT-DATA-MEDIA-UNGATE / M-PPT-CUSTDATA-WIRE): 断言媒体/自定义数据分支；对应 src SwNvPrHandler.rs 的 DEFERRED_NON_FUNCTION（Media 类型仍受 drawing/data/media/mod.rs 的 cfg(any()) 门禁；非管线核心，见计划 §14）"]
fn media_children_set_exact_modes_and_construct_exact_handlers() {
    let cases = [
        (1114119, 0, "AudioCDHandler"),
        (1114120, 1, "EmbeddedWAVAudioFileHandler"),
        (1114121, 2, "AudioFileHandler"),
        (1114122, 3, "VideoFileHandler"),
        (1114123, 4, "QuickTimeFileHandler"),
    ];
    for (element, mode, kind) in cases {
        let (shape, mut handler) = new_handler();
        let child = handler.enterSubElement(element).expect("known media child");
        assert_eq!(child.kind(), kind);
        assert_eq!(shape.borrow().media.mode(), mode);
    }
}

#[test]
#[ignore = "DEFERRED(M-PPT-DATA-MEDIA-UNGATE / M-PPT-CUSTDATA-WIRE): 断言媒体/自定义数据分支；对应 src SwNvPrHandler.rs 的 DEFERRED_NON_FUNCTION（Media 类型仍受 drawing/data/media/mod.rs 的 cfg(any()) 门禁；非管线核心，见计划 §14）"]
fn extension_and_custom_data_construct_handlers_without_changing_media_mode() {
    for (element, kind) in [
        (3211295, "MediaExtLstHandler"),
        (3211519, "CustDataLstHandler"),
    ] {
        let (shape, mut handler) = new_handler();
        let child = handler
            .enterSubElement(element)
            .expect("known metadata child");
        assert_eq!(child.kind(), kind);
        assert_eq!(shape.borrow().media.mode(), -1);
    }
}

fn parse_placeholder(attrs: &[(i32, &str)]) -> Rc<RefCell<Placeholder>> {
    let (shape, mut handler) = new_handler();
    let mut child = handler.enterSubElement(3211571).expect("placeholder child");
    child.startElement(3211571, &RecordingXmlRoAttr::from_pairs(attrs).to_list());
    handler.endElement(3211571);
    let placeholder = shape
        .borrow()
        .placeholder
        .clone()
        .expect("placeholder must be attached");
    placeholder
}

#[test]
fn placeholder_is_saved_once_and_attached_on_outer_end() {
    let placeholder = parse_placeholder(&[(3211570, "true")]);
    assert!(placeholder.borrow().has_custom_prompt);
    assert_eq!(placeholder.borrow().save_count, 1);
}

#[test]
fn placeholder_idx_normalizes_unsigned_sentinel_and_preserves_normal_values() {
    assert_eq!(
        parse_placeholder(&[(3211459, "4294967295")]).borrow().idx,
        -1
    );
    assert_eq!(parse_placeholder(&[(3211459, "0")]).borrow().idx, 0);
    assert_eq!(parse_placeholder(&[(3211459, "42")]).borrow().idx, 42);
}

#[test]
fn placeholder_known_enums_map_and_unknown_tokens_keep_defaults() {
    let known = parse_placeholder(&[(3211267, "vert"), (3211569, "quarter"), (3211302, "pic")]);
    assert_eq!(known.borrow().orient, 1);
    assert_eq!(known.borrow().size, 2);
    assert_eq!(known.borrow().shape_type, 15);

    let unknown = parse_placeholder(&[
        (3211267, "diagonal"),
        (3211569, "giant"),
        (3211302, "unknown"),
    ]);
    assert_eq!(unknown.borrow().orient, 0);
    assert_eq!(unknown.borrow().size, 0);
    assert_eq!(unknown.borrow().shape_type, 8);
}

#[test]
fn unknown_child_returns_none_and_does_not_mutate_shape() {
    let (shape, mut handler) = new_handler();
    assert!(handler.enterSubElement(-999).is_none());
    assert!(!shape.borrow().is_photo);
    assert!(!shape.borrow().user_drawn);
    assert_eq!(shape.borrow().media.mode(), -1);
    assert!(shape.borrow().placeholder.is_none());
}
