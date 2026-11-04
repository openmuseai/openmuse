// Java: cn.wps.moffice.writer.io.reader.dmlReader.InnerShdwImporter
use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, sync::{Arc, Mutex}};
use drawing::cn::wps::moffice::drawing::effects::{ShadowDml::ShadowDml, ShadowPos::ShadowPos};
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use util::graphics::PointF::PointF;
use util::base::io::MetricUtility::MetricUtility;
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    DmlImporterBase::DmlImporterBase,
    context::{
        ColorTransform::ColorTransformValue, DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt,
        InnerShdw::InnerShdw, PrstClr::PrstClr, SchemeClr::SchemeClr, ScrgbClr::ScrgbClr,
        SrgbClr::SrgbClr, SysClr::SysClr, ThemeContext::ThemeContext,
    },
    helper::{ColorsTrans::ColorsTrans, MetricHelper::MetricHelper},
};

// Java: protected void importCommonProp(InnerShdw) body, with the caller's alpha.
pub(crate) fn apply_inner_common(target: &mut ShadowDml, shdw: &InnerShdw, alpha: f32,
    ph_clr: Option<i32>, theme: &ThemeContext) {
    // Java: this.mTarget.setShadowOpacity(1.0F - alpha);
    target.base.setShadowOpacity(1.0 - alpha);
    // Java: Integer clr = ColorsTrans.toColor(this.mCtxt.mPhClr, (ColorChoice)shdw, this.mThemeCtxt);
    if let Some(clr) = ColorsTrans::toColor(ph_clr, shdw, Some(theme)) {
        target.base.setShadowColor(clr);
    }
    if let Some(blur) = shdw.mBlurRad {
        target.base.setBlurRadius(MetricUtility::EMU2Points(blur as f32));
    }
    // Java: Long dist = null != shdw.mDist ? shdw.mDist : 0L;
    // Java: dist = (long)MetricUtility.EMU2Points((float)dist);
    let dist_present = shdw.mDist;
    let dist = MetricUtility::EMU2Points(dist_present.unwrap_or(0) as f32) as i64;
    if dist_present.is_some() {
        target.setShadowDist(dist as f32);
    }
    // Java: Integer dir = null != shdw.mDir ? shdw.mDir : 0;
    let dir_present = shdw.mDir;
    let mut angle = MetricUtility::ST_ANGLE_Degrees2Degree(dir_present.unwrap_or(0));
    angle = MetricUtility::degree2Rad(angle as f64) as f32;
    if dir_present.is_some() {
        target.setShadowDir(angle);
    }
    let offset_x = (angle as f64).cos() * dist as f64;
    let offset_y = (angle as f64).sin() * dist as f64;
    if target.base.getShadowOffsetXObj().is_none() {
        target.base.setShadowOffsetX(offset_x as f32);
    }
    if target.base.getShadowOffsetYObj().is_none() {
        target.base.setShadowOffsetY(offset_y as f32);
    }
    // Java: PointF origin = ShapeUtil.getOrigin(this.mTarget.getShadowAlgn());
    // drawing::util::ShapeUtil is not on the compile surface yet (Object3D). Same switch.
    let origin = shadow_origin(target.getShadowAlgn());
    if target.base.getShadowOriginXObj().is_none() {
        target.base.setShadowOriginX(origin.x);
    }
    if target.base.getShadowOriginYObj().is_none() {
        target.base.setShadowOriginY(origin.y);
    }
    if target.base.getShadowTypeObj().is_none() {
        target.base.setShadowType(2);
    }
    target.base.setShadowOn(true);
}

// Java: public class InnerShdwImporter extends DmlImporterBase
pub struct InnerShdwImporter<ShapeImporter, DmlCommonImporter> {
    pub super_: DmlImporterBase<ShapeImporter, DmlCommonImporter>,
    // Java: protected ShadowDml mTarget;
    pub mTarget: Option<Arc<Mutex<ShadowDml>>>,
}

impl<S, D: IImporterBase> Deref for InnerShdwImporter<S, D> {
    type Target = DmlImporterBase<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for InnerShdwImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> InnerShdwImporter<S, D> {
    // Java: InnerShdwImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext, Xml07Context, DmlCommonImporter)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>, ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>, xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<D>>) -> Self {
        Self { super_: DmlImporterBase::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter),
            mTarget: None }
    }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        if self.mTarget.is_none() {
            let mut shadow = ShadowDml::new();
            shadow.setShadowPos(ShadowPos::Inner);
            self.mTarget = Some(Arc::new(Mutex::new(shadow)));
        }
    }

    // Java: public void createInnerShdw(Attributes attrs)
    pub fn createInnerShdw(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mInnerShdw = Some(InnerShdw::new(attrs));
    }

    // Java: public void excute()
    pub fn excute(&mut self) {
        self.initTarget();
        let shdw = self.mCtxt.borrow_mut().mInnerShdw.take();
        let Some(shdw) = shdw else { return; };
        self.importCommonProp(&shdw);
        self.mCtxt.borrow_mut().mInnerShdw = Some(shdw);
    }

    // Java: protected void importCommonProp(InnerShdw shdw)
    // Java getAlpha reads mCtxt.mInnerShdw, which is this argument on the excute path.
    pub fn importCommonProp(&mut self, shdw: &InnerShdw) {
        let alpha = ColorsTrans::toTransparency(shdw, false);
        let ph_clr = self.mCtxt.borrow().mPhClr;
        let theme = self.mThemeCtxt.borrow();
        let mut target = self.mTarget.as_ref().expect("mTarget").lock().expect("shadow");
        apply_inner_common(&mut target, shdw, alpha, ph_clr, &theme);
    }

    // Java: protected float getAlpha()
    pub fn getAlpha(&self) -> f32 {
        let ctxt = self.mCtxt.borrow();
        ColorsTrans::toTransparency(ctxt.mInnerShdw.as_ref().expect("mInnerShdw"), false)
    }

    // Java: public void createSysClr(Attributes attrs)
    pub fn createSysClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform = Some(ColorTransformValue::SysClr(SysClr::new(attrs)));
    }
    // Java: public void createPrstClr(Attributes attrs)
    pub fn createPrstClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform = Some(ColorTransformValue::PrstClr(PrstClr::new(attrs)));
    }
    // Java: public void createSchemeClr(Attributes attrs)
    pub fn createSchemeClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform = Some(ColorTransformValue::SchemeClr(SchemeClr::new(attrs)));
    }
    // Java: public void createSrgbClr(Attributes attrs)
    pub fn createSrgbClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform = Some(ColorTransformValue::SrgbClr(SrgbClr::new(attrs)));
    }
    // Java: public void createScrgbClr(Attributes attrs)
    pub fn createScrgbClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform = Some(ColorTransformValue::ScrgbClr(ScrgbClr::new(attrs)));
    }

    // Java: public void setSat/setLum/setRed/setGreen/setBlue/setShade/setAlpha/setTint
    pub fn setSat(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::Sat); }
    pub fn setSatMod(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::SatMod); }
    pub fn setSatOff(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::SatOff); }
    pub fn setLum(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::Lum); }
    pub fn setLumMod(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::LumMod); }
    pub fn setLumOff(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::LumOff); }
    pub fn setRed(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::Red); }
    pub fn setRedMod(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::RedMod); }
    pub fn setRedOff(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::RedOff); }
    pub fn setGreen(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::Green); }
    pub fn setGreenMod(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::GreenMod); }
    pub fn setGreenOff(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::GreenOff); }
    pub fn setBlue(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::Blue); }
    pub fn setBlueMod(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::BlueMod); }
    pub fn setBlueOff(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::BlueOff); }
    pub fn setShade(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::Shade); }
    pub fn setAlpha(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::Alpha); }
    pub fn setTint(&mut self, attrs: &dyn SaxAttributes) { self.set_percent(attrs, PercentField::Tint); }

    fn set_percent(&mut self, attrs: &dyn SaxAttributes, field: PercentField) {
        let value = MetricHelper::percent2Float(attrs, "val");
        let mut ctxt = self.mCtxt.borrow_mut();
        let transform = ctxt.mClrTransform.as_mut().expect("mClrTransform");
        match field {
            PercentField::Sat => transform.mSat = value,
            PercentField::SatMod => transform.mSatMod = value,
            PercentField::SatOff => transform.mSatOff = value,
            PercentField::Lum => transform.mLum = value,
            PercentField::LumMod => transform.mLumMod = value,
            PercentField::LumOff => transform.mLumOff = value,
            PercentField::Red => transform.mRed = value,
            PercentField::RedMod => transform.mRedMod = value,
            PercentField::RedOff => transform.mRedOff = value,
            PercentField::Green => transform.mGreen = value,
            PercentField::GreenMod => transform.mGreenMod = value,
            PercentField::GreenOff => transform.mGreenOff = value,
            PercentField::Blue => transform.mBlue = value,
            PercentField::BlueMod => transform.mBlueMod = value,
            PercentField::BlueOff => transform.mBlueOff = value,
            PercentField::Shade => transform.mShade = value,
            PercentField::Alpha => transform.mAlpha = value,
            PercentField::Tint => transform.mTint = value,
        }
    }

    // Java: public void setSysClr()
    pub fn setSysClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SysClr(color) = transform else { panic!("mClrTransform is not SysClr"); };
        self.mCtxt.borrow_mut().mInnerShdw.as_mut().expect("mInnerShdw").mSysClr = Some(color);
    }
    // Java: public void setPrstClr()
    pub fn setPrstClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::PrstClr(color) = transform else { panic!("mClrTransform is not PrstClr"); };
        self.mCtxt.borrow_mut().mInnerShdw.as_mut().expect("mInnerShdw").mPrstClr = Some(color);
    }
    // Java: public void setSchemeClr()
    pub fn setSchemeClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SchemeClr(color) = transform else { panic!("mClrTransform is not SchemeClr"); };
        self.mCtxt.borrow_mut().mInnerShdw.as_mut().expect("mInnerShdw").mSchemeClr = Some(color);
    }
    // Java: public void setSrgbClr()
    pub fn setSrgbClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SrgbClr(color) = transform else { panic!("mClrTransform is not SrgbClr"); };
        self.mCtxt.borrow_mut().mInnerShdw.as_mut().expect("mInnerShdw").mSrgbClr = Some(color);
    }
    // Java: public void setScrgbClr()
    pub fn setScrgbClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::ScrgbClr(color) = transform else { panic!("mClrTransform is not ScrgbClr"); };
        self.mCtxt.borrow_mut().mInnerShdw.as_mut().expect("mInnerShdw").mScrgbClr = Some(color);
    }

    // Java: public ShadowDml result()
    pub fn result(&self) -> Option<Arc<Mutex<ShadowDml>>> { self.mTarget.clone() }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        self.mTarget = None;
        self.mCtxt.borrow_mut().mInnerShdw = None;
    }
}

enum PercentField { Sat, SatMod, SatOff, Lum, LumMod, LumOff, Red, RedMod, RedOff,
    Green, GreenMod, GreenOff, Blue, BlueMod, BlueOff, Shade, Alpha, Tint }

// Java: ShapeUtil.getOrigin(int algn)
fn shadow_origin(algn: i32) -> PointF {
    match algn {
        0 => PointF::new_xy(0.0, 0.5),
        1 => PointF::new_xy(-0.5, 0.5),
        2 => PointF::new_xy(0.5, 0.5),
        3 => PointF::new_xy(0.0, 0.0),
        4 => PointF::new_xy(-0.5, 0.0),
        5 => PointF::new_xy(0.5, 0.0),
        6 => PointF::new_xy(0.0, -0.5),
        7 => PointF::new_xy(-0.5, -0.5),
        8 => PointF::new_xy(0.5, -0.5),
        _ => PointF::new_xy(0.0, 0.5),
    }
}
