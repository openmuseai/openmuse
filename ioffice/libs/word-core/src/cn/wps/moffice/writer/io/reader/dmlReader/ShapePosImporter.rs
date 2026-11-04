// Java: cn.wps.moffice.writer.io.reader.dmlReader.ShapePosImporter
use std::{any::Any, cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, sync::Arc};
use drawing::cn::wps::moffice::drawing::ShapePos::ShapePos;
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use util::base::io::{MetricUtility::MetricUtility, TypeConversion::TypeConversion};
use util::graphics::RectF::RectF;
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    DmlImporterBase::DmlImporterBase,
    PositioningContext::PositioningContext,
    context::{DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt, ThemeContext::ThemeContext},
};

fn java_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, ch| hash.wrapping_mul(31).wrapping_add(ch as i32))
}
fn attr_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), name) }
}
fn empty_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri("", name) }
}
fn empty_bool(attrs: &dyn SaxAttributes, name: &str) -> Option<bool> {
    let value = empty_string(attrs, name)?;
    Some(!(value.eq_ignore_ascii_case("off") || value == "0" || value.eq_ignore_ascii_case("false") || value.eq_ignore_ascii_case("f")))
}
fn empty_int(attrs: &dyn SaxAttributes, name: &str) -> Option<i32> {
    TypeConversion::string2Integer(&empty_string(attrs, name)?)
}

// Java: public class ShapePosImporter extends DmlImporterBase
pub struct ShapePosImporter<ShapeImporter, DmlCommonImporter: IImporterBase> {
    pub super_: DmlImporterBase<ShapeImporter, DmlCommonImporter>,
    // Java: private ShapePos mTarget;
    mTarget: Option<Arc<ShapePos>>,
    // Java: public PositioningContext mPositioningCtxt;
    pub mPositioningCtxt: Option<Rc<RefCell<PositioningContext>>>,
}

impl<S, D: IImporterBase> Deref for ShapePosImporter<S, D> {
    type Target = DmlImporterBase<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for ShapePosImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> ShapePosImporter<S, D> {
    // Java: ShapePosImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext, Xml07Context, DmlImporter)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>, ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>, xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<D>>) -> Self {
        Self {
            super_: DmlImporterBase::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter),
            mTarget: None,
            mPositioningCtxt: None,
        }
    }

    // Java: public void setPositioningContext(PositioningContext ctxt)
    pub fn setPositioningContext(&mut self, ctxt: Option<Rc<RefCell<PositioningContext>>>) {
        self.mPositioningCtxt = ctxt;
    }

    // Java: public void excute()
    pub fn excute(&mut self) {
        self.initTarget();
        self.preparePosInfo_step1();
        self.preparePosInfo_step2();
        self.importShapeRect();
        if self.mPositioningCtxt.is_some() {
            self.importWrap();
            let inline = self.mPositioningCtxt.as_ref().expect("positioning").borrow().mInLine;
            if !inline {
                self.importAlinHV();
                let (allow, in_cell, rel_h, rel_v, pct_h, pct_v) = {
                    let positioning = self.mPositioningCtxt.as_ref().expect("positioning").borrow();
                    (positioning.mAllowOverlap, positioning.mLayoutInCell, positioning.mPosRelH,
                        positioning.mPosRelV, positioning.mPctPositionH, positioning.mPctPositionV)
                };
                let pos = Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("shape pos");
                pos.setAllowOverlap(allow);
                pos.setLayoutInCell(in_cell);
                if pct_h.is_some() {
                    pos.setPosHPercent((pct_h.expect("pctH") / 1000) as f32);
                    pos.setRelativeH(rel_h);
                } else {
                    pos.setPosRelH(rel_h);
                }
                if pct_v.is_some() {
                    pos.setPosVPercent((pct_v.expect("pctV") / 1000) as f32);
                    pos.setRelativeV(rel_v);
                } else {
                    pos.setPosRelV(rel_v);
                }
            } else {
                let pos = Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("shape pos");
                pos.setScaleH(1.0);
                pos.setScaleV(1.0);
            }
        }
    }

    // Java: public ShapePos result()
    pub fn result(&self) -> Option<Arc<ShapePos>> {
        self.mTarget.clone()
    }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        if self.mTarget.is_none() {
            self.mTarget = Some(Arc::new(ShapePos::new()));
        }
    }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        self.mTarget = None;
        self.mPositioningCtxt = None;
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mOffX = None;
        ctxt.mOffY = None;
        ctxt.mExtX = None;
        ctxt.mExtY = None;
    }

    // Java: public void importSizeRelH()
    pub fn importSizeRelH(&mut self) {
        let size_rel = self.mCtxt.borrow_mut().mSizeRelH.take();
        if let Some(size_rel) = size_rel {
            let mapped = self.toSizeRelH(size_rel);
            Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("shape pos").setSizeRelH(mapped);
        }
    }

    // Java: public void importSizeRelV()
    pub fn importSizeRelV(&mut self) {
        let size_rel = self.mCtxt.borrow_mut().mSizeRelV.take();
        if let Some(size_rel) = size_rel {
            let mapped = self.toSizeRelV(size_rel);
            Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("shape pos").setSizeRelV(mapped);
        }
    }

    // Java: public void importSizeHPercent(String content)
    pub fn importSizeHPercent(&mut self, content: Option<&str>) {
        if let Some(content) = content {
            if let Some(pct_width) = TypeConversion::string2Integer(content) {
                Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("shape pos")
                    .setSizeHPercent(pct_width as f32 / 1000.0);
            }
        }
    }

    // Java: public void importSizeVPercent(String content)
    pub fn importSizeVPercent(&mut self, content: Option<&str>) {
        if let Some(content) = content {
            if let Some(pct_height) = TypeConversion::string2Integer(content) {
                Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("shape pos")
                    .setSizeVPercent(pct_height as f32 / 1000.0);
            }
        }
    }

    // Java: private int toSizeRelH(int sizeRelH)
    fn toSizeRelH(&self, sizeRelH: i32) -> i32 {
        match sizeRelH {
            -1427286838 => 4,
            -1081309778 => 0,
            -599904534 => 3,
            -278063181 => 5,
            3433103 => 1,
            1928835221 => 2,
            _ => 0,
        }
    }

    // Java: private int toSizeRelV(int sizeRelV)
    fn toSizeRelV(&self, sizeRelV: i32) -> i32 {
        match sizeRelV {
            -1427286838 => 4,
            -1081309778 => 0,
            -414179485 => 2,
            -278063181 => 5,
            3433103 => 1,
            2064613305 => 3,
            _ => 0,
        }
    }

    // Java: private void importShapeRect()
    fn importShapeRect(&mut self) {
        if self.mPositioningCtxt.is_some() {
            self.importShapeRect1();
        } else {
            self.importShapeRect2();
        }
    }

    // Java: public void importRotation(Attributes attrs)
    pub fn importRotation(&mut self, attrs: &dyn SaxAttributes) {
        self.initTarget();
        {
            let mut ctxt = self.mCtxt.borrow_mut();
            ctxt.mFlipV = empty_bool(attrs, "flipV");
            ctxt.mFlipH = empty_bool(attrs, "flipH");
        }
        let tmp = empty_int(attrs, "rot");
        let mut angle = 0.0_f32;
        if let Some(tmp) = tmp {
            angle = MetricUtility::ST_ANGLE_Degrees2Degree(tmp);
        }
        Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("shape pos").setRotation(angle);
    }

    // Java: private void importAlinHV()
    fn importAlinHV(&mut self) {
        let (align_h, align_v) = {
            let positioning = self.mPositioningCtxt.as_ref().expect("positioning").borrow();
            (positioning.mPosAlignH, positioning.mPosAlignV)
        };
        let pos = Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("shape pos");
        if let Some(align_h) = align_h {
            pos.setAlignmentH(align_h);
        }
        if let Some(align_v) = align_v {
            pos.setAlignmentV(align_v);
        }
    }

    // Java: private void importShapeRect2()
    fn importShapeRect2(&mut self) {
        let (off_x, off_y, ext_x, ext_y) = {
            let ctxt = self.mCtxt.borrow();
            (ctxt.mOffX, ctxt.mOffY, ctxt.mExtX, ctxt.mExtY)
        };
        let l = off_x.unwrap_or(0) as f32;
        let t = off_y.unwrap_or(0) as f32;
        let w = ext_x.unwrap_or(0) as f32;
        let h = ext_y.unwrap_or(0) as f32;
        let rectf = RectF::new_with(l, t, l + w, t + h);
        Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("shape pos")
            .setShapeRect(Some(Arc::new(rectf) as Arc<dyn Any + Send + Sync>));
    }

    // Java: private void importShapeRect1()
    fn importShapeRect1(&mut self) {
        let (pos_h, pos_v, width, height) = {
            let positioning = self.mPositioningCtxt.as_ref().expect("positioning").borrow();
            (positioning.mPosH, positioning.mPosV, positioning.mWidth, positioning.mHeight)
        };
        let mut left = 0.0_f32;
        if let Some(pos_h) = pos_h {
            left = MetricUtility::EMU2Points(pos_h as f32);
        }
        let mut top = 0.0_f32;
        if let Some(pos_v) = pos_v {
            top = MetricUtility::EMU2Points(pos_v as f32);
        }
        let mut right = left;
        if let Some(width) = width {
            let dst_width_inch = MetricUtility::EMU2Inches(width as f32);
            right = left + MetricUtility::Inches2Points(dst_width_inch);
        }
        let mut bottom = top;
        if let Some(height) = height {
            let dst_height_inch = MetricUtility::EMU2Inches(height as f32);
            bottom = top + MetricUtility::Inches2Points(dst_height_inch);
        }
        let rect_f = RectF::new_with(left, top, right, bottom);
        Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("shape pos")
            .setShapeRect(Some(Arc::new(rect_f) as Arc<dyn Any + Send + Sync>));
    }

    // Java: private void importWrap()
    fn importWrap(&mut self) {
        let (wrap_type, wrap_text, dist_b, dist_t, dist_l, dist_r) = {
            let positioning = self.mPositioningCtxt.as_ref().expect("positioning").borrow();
            (positioning.mWrapType, positioning.mWrapText, positioning.mDistanceB,
                positioning.mDistanceT, positioning.mDistanceL, positioning.mDistanceR)
        };
        let pos = Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("shape pos");
        pos.setTextWrap(wrap_type);
        pos.setTextWrapType(wrap_text);
        if let Some(dist_b) = dist_b { pos.setWrapDistBottom(dist_b); }
        if let Some(dist_t) = dist_t { pos.setWrapDistTop(dist_t); }
        if let Some(dist_l) = dist_l { pos.setWrapDistLeft(dist_l); }
        if let Some(dist_r) = dist_r { pos.setWrapDistRight(dist_r); }
    }

    // Java: public void setSizeRelH(Attributes attributes)
    pub fn setSizeRelH(&mut self, attributes: &dyn SaxAttributes) {
        if let Some(from) = attr_string(attributes, "relativeFrom") {
            self.mCtxt.borrow_mut().mSizeRelH = Some(java_hash(&from));
        }
    }

    // Java: public void setSizeRelV(Attributes attributes)
    pub fn setSizeRelV(&mut self, attributes: &dyn SaxAttributes) {
        if let Some(from) = attr_string(attributes, "relativeFrom") {
            self.mCtxt.borrow_mut().mSizeRelV = Some(java_hash(&from));
        }
    }

    // Java: private void preparePosInfo_step1()
    fn preparePosInfo_step1(&mut self) {
        let Some(positioning) = &self.mPositioningCtxt else { return; };
        let mut positioning = positioning.borrow_mut();
        if positioning.mIsUseSimplePos == Some(true) {
            positioning.mPosH = positioning.mSimplePosH;
            positioning.mPosV = positioning.mSimplePosV;
        }
    }

    // Java: private void preparePosInfo_step2()
    fn preparePosInfo_step2(&mut self) {
        let Some(positioning) = &self.mPositioningCtxt else { return; };
        let mut positioning = positioning.borrow_mut();
        if positioning.mPosH.is_none() && positioning.mPosV.is_none() {
            if positioning.mPositionH.is_some() {
                positioning.mPosH = positioning.mPositionH;
            }
            if positioning.mPositionV.is_some() {
                positioning.mPosV = positioning.mPositionV;
            }
        }
    }
}
