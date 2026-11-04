// Java: cn.wps.moffice.writer.io.reader.dmlReader.ShadowImporter
use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc};
use drawing::cn::wps::moffice::drawing::effects::{ShadowDml::ShadowDml, ShadowPos::ShadowPos};
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use util::base::io::MetricUtility::MetricUtility;
use crate::cn::wps::moffice::writer::io::reader::{
    helper::IOHelper::IOHelper, xml07::Xml07Context::Xml07Context,
};
use super::{
    InnerShdwImporter::{apply_inner_common, InnerShdwImporter},
    context::{
        ColorTransform::ColorTransformValue, DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt,
        Shadow::Shadow, ThemeContext::ThemeContext,
    },
    helper::{ColorsTrans::ColorsTrans, EffectHelper::EffectHelper},
};

// Java: protected void importCommonProp(Shadow) after super.importCommonProp.
pub(crate) fn apply_shadow_scales(target: &mut ShadowDml, shadow: &Shadow) {
    if let Some(value) = IOHelper::Percentage2Float(shadow.mSX.as_deref()) {
        target.base.setShadowScaleXToX(value);
    }
    if let Some(value) = IOHelper::Percentage2Float(shadow.mSY.as_deref()) {
        target.base.setShadowScaleYToY(value);
    }
    if let Some(value) = IOHelper::fixedAngle2Integer(shadow.mKX.as_deref()) {
        target.base.setShadowScaleYToX(MetricUtility::ST_ANGLE_Degrees2Rad(value));
    }
    if let Some(value) = IOHelper::fixedAngle2Integer(shadow.mKY.as_deref()) {
        target.base.setShadowScaleXToY(MetricUtility::ST_ANGLE_Degrees2Rad(value));
    }
}

// Java: protected void importAlgn(Shadow shadow, int defaultValue)
pub(crate) fn apply_shadow_algn(target: &mut ShadowDml, algn: Option<&str>, default_value: i32) {
    let algn = algn.and_then(|value| EffectHelper::string2RectAlignmentType(Some(value)));
    target.setShadowAlgn(algn.unwrap_or(default_value));
}

// Java: public class ShadowImporter extends InnerShdwImporter
pub struct ShadowImporter<ShapeImporter, DmlCommonImporter> {
    pub super_: InnerShdwImporter<ShapeImporter, DmlCommonImporter>,
}

impl<S, D: IImporterBase> Deref for ShadowImporter<S, D> {
    type Target = InnerShdwImporter<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for ShadowImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> ShadowImporter<S, D> {
    // Java: ShadowImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext, Xml07Context, DmlCommonImporter)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>, ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>, xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<D>>) -> Self {
        Self { super_: InnerShdwImporter::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter) }
    }

    // Java: public void creatShadow(Attributes attrs)
    pub fn creatShadow(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mShadow = Some(Shadow::new(attrs));
    }

    // Java: public void excute()
    pub fn excute(&mut self) {
        self.initTarget();
        let shadow = self.mCtxt.borrow_mut().mShadow.take().expect("mShadow");
        self.importCommonProp(&shadow);
        self.importAlgn(&shadow, -1);
        self.mCtxt.borrow_mut().mShadow = Some(shadow);
    }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        if self.mTarget.is_none() {
            let mut shadow = ShadowDml::new();
            shadow.setShadowPos(ShadowPos::None);
            self.mTarget = Some(std::sync::Arc::new(std::sync::Mutex::new(shadow)));
        }
    }

    // Java: protected void importCommonProp(Shadow shadow)
    // Alpha is this class's getAlpha (isText = true). The argument is mCtxt.mShadow on the excute path.
    pub fn importCommonProp(&mut self, shadow: &Shadow) {
        let alpha = ColorsTrans::toTransparency(shadow, true);
        let ph_clr = self.mCtxt.borrow().mPhClr;
        let theme = self.mThemeCtxt.borrow();
        let mut target = self.mTarget.as_ref().expect("mTarget").lock().expect("shadow");
        apply_inner_common(&mut target, shadow, alpha, ph_clr, &theme);
        apply_shadow_scales(&mut target, shadow);
    }

    // Java: protected void importAlgn(Shadow shadow, int defaultValue)
    pub fn importAlgn(&mut self, shadow: &Shadow, default_value: i32) {
        let mut target = self.mTarget.as_ref().expect("mTarget").lock().expect("shadow");
        apply_shadow_algn(&mut target, shadow.mAlgn.as_deref(), default_value);
    }

    // Java: public void setSysClr()
    pub fn setSysClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SysClr(color) = transform else { panic!("mClrTransform is not SysClr"); };
        self.mCtxt.borrow_mut().mShadow.as_mut().expect("mShadow").mSysClr = Some(color);
    }
    pub fn setPrstClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::PrstClr(color) = transform else { panic!("mClrTransform is not PrstClr"); };
        self.mCtxt.borrow_mut().mShadow.as_mut().expect("mShadow").mPrstClr = Some(color);
    }
    pub fn setSchemeClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SchemeClr(color) = transform else { panic!("mClrTransform is not SchemeClr"); };
        self.mCtxt.borrow_mut().mShadow.as_mut().expect("mShadow").mSchemeClr = Some(color);
    }
    pub fn setSrgbClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SrgbClr(color) = transform else { panic!("mClrTransform is not SrgbClr"); };
        self.mCtxt.borrow_mut().mShadow.as_mut().expect("mShadow").mSrgbClr = Some(color);
    }
    pub fn setScrgbClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::ScrgbClr(color) = transform else { panic!("mClrTransform is not ScrgbClr"); };
        self.mCtxt.borrow_mut().mShadow.as_mut().expect("mShadow").mScrgbClr = Some(color);
    }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        self.mTarget = None;
        self.mCtxt.borrow_mut().mShadow = None;
    }

    // Java: protected float getAlpha()
    pub fn getAlpha(&self) -> f32 {
        let ctxt = self.mCtxt.borrow();
        ColorsTrans::toTransparency(ctxt.mShadow.as_ref().expect("mShadow"), true)
    }
}
