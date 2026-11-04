#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]
//! Compile-surface hosts for parked KmoShowShape signatures (pp-19 function 01).
//! Real Java types stay in their modules; this is not libs/core-libs/stubs.

#[derive(Default, Clone, Debug)]
pub struct KSAssert;

#[derive(Default, Clone, Debug)]
pub struct KmoOleChart;

#[derive(Default, Clone, Debug)]
pub struct DgColor;

#[derive(Default, Clone, Debug)]
pub struct DgColorFormat;

#[derive(Default, Clone, Debug)]
pub struct DgFillFormat;

#[derive(Default, Clone, Debug)]
pub struct DgOutlineFormat;

// rust: ★W6c-10 —— `GraphicObject` 单元空壳**已删除**：真身是
//   `drawing/app/graphic/GraphicObject.rs`（Java `GraphicObject.java` 接口）。
//   消费方（`KmoShowShape`/`SwGroupShapeHandler`/`ConvertTool`）均已改指真身，
//   本宿主不再有任何引用。（原 `r#type() 恒 0` 会把 graphicFrame 误分类为 type 0。）

#[derive(Default, Clone, Debug)]
pub struct DgConnector;

#[derive(Default, Clone, Debug)]
pub struct DgGroup;

#[derive(Default, Clone, Debug)]
pub struct DgPicture;

#[derive(Default, Clone, Debug)]
pub struct DgShape;

#[derive(Default, Clone, Debug)]
pub struct DgShapeStyle;

#[derive(Default, Clone, Debug)]
pub struct KmoTable;

#[derive(Default, Clone, Debug)]
pub struct UnitsConverter;

pub use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
pub use crate::cn::wps::drawing::data::shape::shapepr_hosts::Transform2D;

#[derive(Default, Clone, Debug)]
pub struct Blip;

#[derive(Default, Clone, Debug)]
pub struct Color;

#[derive(Default, Clone, Debug)]
pub struct Fill;

#[derive(Default, Clone, Debug)]
pub struct Geometry2D;

#[derive(Default, Clone, Debug)]
pub struct PresetGeometry;

#[derive(Default, Clone, Debug)]
pub struct Hyperlink;

#[derive(Default, Clone, Debug)]
pub struct Media;

#[derive(Default, Clone, Debug)]
pub struct Outline;

#[derive(Default, Clone, Debug)]
pub struct Shape;

#[derive(Default, Clone, Debug)]
pub struct ShapeExt;

#[derive(Default, Clone, Debug)]
pub struct ShapeStyle;

#[derive(Default, Clone, Debug)]
pub struct PictureInfo;

#[derive(Default, Clone, Debug)]
pub struct PictureParser;

#[derive(Default, Clone, Debug)]
pub struct IIntList;

#[derive(Default, Clone, Debug)]
pub struct WIntArrayList;

#[derive(Default, Clone, Debug)]
pub struct KmoUndoableList;

#[derive(Default, Clone, Debug)]
pub struct KoBin;

#[derive(Default, Clone, Debug)]
pub struct KoIntList;

#[derive(Default, Clone, Debug)]
pub struct GraphicsPath;

#[derive(Default, Clone, Debug)]
pub struct IMediaLib;

#[derive(Default, Clone, Debug)]
pub struct File;

#[derive(Default, Clone, Debug)]
pub struct FileDataStorage;

#[derive(Default, Clone, Debug)]
pub struct ISlideCache;

#[derive(Default, Clone, Debug)]
pub struct KmoPresentation;

#[derive(Default, Clone, Debug)]
pub struct KsoWMInfo;

#[derive(Default, Clone, Debug)]
pub struct KmoInkLib;

#[derive(Default, Clone, Debug)]
pub struct KmoChartOperator;

#[derive(Default, Clone, Debug)]
pub struct KmoFont;

#[derive(Default, Clone, Debug)]
pub struct FillBean;

#[derive(Default, Clone, Debug)]
pub struct KmoMediaData;

#[derive(Default, Clone, Debug)]
pub struct KmoMediaLib;

#[derive(Default, Clone, Debug)]
pub struct KmoMediaUtil;

#[derive(Default, Clone, Debug)]
pub struct KmoModel3D;

#[derive(Default, Clone, Debug)]
pub struct KmoOleManager;

#[derive(Default, Clone, Debug)]
pub struct KmoOleObject;

#[derive(Default, Clone, Debug)]
pub struct KmoSelection;

#[derive(Default, Clone, Debug)]
pub struct KmoTextRange;

#[derive(Default, Clone, Debug)]
pub struct KmoFilm;

#[derive(Default, Clone, Debug)]
pub struct KmoTableOperator;

#[derive(Default, Clone, Debug)]
pub struct KmoCellTextOperator;

#[derive(Default, Clone, Debug)]
pub struct KmoContext;

#[derive(Default, Clone, Debug)]
pub struct KmoTextOperator;
impl KmoTextOperator {
    pub fn getTextBody(&self) {}
}

#[derive(Default, Clone, Debug)]
pub struct KmoTheme;

#[derive(Default, Clone, Debug)]
pub struct ColorInterpretCtx;

#[derive(Default, Clone, Debug)]
pub struct ColorInterpreter;

#[derive(Default, Clone, Debug)]
pub struct KmoShapeUtil;

#[derive(Default, Clone, Debug)]
pub struct ModularGroupUtil;

#[derive(Default, Clone, Debug)]
pub struct CustDataLst;

#[derive(Default, Clone, Debug)]
pub struct Tag;

#[derive(Default, Clone, Debug)]
pub struct OleObject;

#[derive(Default, Clone, Debug)]
pub struct ShowShape;

#[derive(Default, Clone, Debug)]
pub struct PackageStreamParser;

#[derive(Default, Clone, Debug)]
pub struct ShowShapeAtomData;

#[derive(Default, Clone, Debug)]
pub struct Entry;

#[derive(Default, Clone, Debug)]
pub struct Placeholder;

#[derive(Default, Clone, Debug)]
pub struct AdjustValueList;

#[derive(Default, Clone, Debug)]
pub struct DgStore;

pub trait KmoProtocol {
    // Java: KoBin protocol()
    fn protocol(&self) -> Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin>;
    // Java: void updateProtocol(KoBin data)
    fn updateProtocol(&mut self, data: &dyn crate::cn::wps::kso::kproto::KoBin::KoBin);
}


#[derive(Default, Clone, Debug)]
pub struct cn_wps_graphics_RectF;

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Default)]
pub struct ConvertAttach {
    pub shape_pr: Option<ShapePr>,
    pub xfrm: Option<Transform2D>,
    pub is_placeholder: bool,
    pub is_hidden: bool,
    pub is_text_box: bool,
    pub placeholder_type: i32,
    pub has_text_body: bool,
    pub prst: Option<i32>,
    pub fill: Option<u32>,
    pub graphics_path: Option<Vec<(f32, f32)>>,
    pub media_bytes: Option<Vec<u8>>,
    pub media_r_id: Option<String>,
    pub media_id: Option<i32>,
    pub text_op: Option<crate::cn::wps::show::util::layout_textbox::KmoTextOperator>,
}

thread_local! {
    static CONVERT_ATTACH: RefCell<HashMap<i32, ConvertAttach>> = RefCell::new(HashMap::new());
}

pub fn attach_shape_pr(id: i32, pr: ShapePr) {
    let mut pr2 = pr;
    CONVERT_ATTACH.with(|m| {
        m.borrow_mut().entry(id).or_default().shape_pr = Some(pr2);
    });
}

pub fn attach_xfrm(id: i32, xfrm: Transform2D) {
    CONVERT_ATTACH.with(|m| {
        m.borrow_mut().entry(id).or_default().xfrm = Some(xfrm);
    });
}

pub fn set_placeholder(id: i32, v: bool) {
    CONVERT_ATTACH.with(|m| {
        m.borrow_mut().entry(id).or_default().is_placeholder = v;
    });
}

pub fn set_hidden(id: i32, v: bool) {
    CONVERT_ATTACH.with(|m| {
        m.borrow_mut().entry(id).or_default().is_hidden = v;
    });
}

pub fn set_text_box(id: i32, v: bool) {
    CONVERT_ATTACH.with(|m| {
        m.borrow_mut().entry(id).or_default().is_text_box = v;
    });
}

pub fn set_placeholder_type(id: i32, v: i32) {
    CONVERT_ATTACH.with(|m| {
        m.borrow_mut().entry(id).or_default().placeholder_type = v;
    });
}

pub fn merge_render_sidecar(id: i32, side: ConvertAttach) {
    CONVERT_ATTACH.with(|m| {
        let mut map = m.borrow_mut();
        let e = map.entry(id).or_default();
        e.placeholder_type = side.placeholder_type;
        e.has_text_body = side.has_text_body;
        e.prst = side.prst;
        e.fill = side.fill;
        e.graphics_path = side.graphics_path;
        e.media_bytes = side.media_bytes;
        e.media_r_id = side.media_r_id;
        e.media_id = side.media_id;
        e.text_op = side.text_op;
    });
}

pub fn convert_attach(id: i32) -> ConvertAttach {
    CONVERT_ATTACH.with(|m| m.borrow().get(&id).cloned().unwrap_or_default())
}

impl Clone for ConvertAttach {
    fn clone(&self) -> Self {
        Self {
            shape_pr: self.shape_pr.as_ref().map(|p| p.convert_clone()),
            xfrm: self.xfrm.clone(),
            is_placeholder: self.is_placeholder,
            is_hidden: self.is_hidden,
            is_text_box: self.is_text_box,
            placeholder_type: self.placeholder_type,
            has_text_body: self.has_text_body,
            prst: self.prst,
            fill: self.fill,
            graphics_path: self.graphics_path.clone(),
            media_bytes: self.media_bytes.clone(),
            media_r_id: self.media_r_id.clone(),
            media_id: self.media_id,
            text_op: self.text_op.clone(),
        }
    }
}
