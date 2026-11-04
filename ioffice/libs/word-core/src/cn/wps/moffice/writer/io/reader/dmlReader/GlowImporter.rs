#![allow(non_snake_case)]
use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc};
use drawing::cn::wps::moffice::drawing::effects::Glow::Glow;
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    DmlImporterBase::DmlImporterBase,
    context::{ColorTransform::ColorTransformValue, DmlShapeCtxt::DmlShapeCtxt,
        DmlSubDocCtxt::DmlSubDocCtxt, Glow::Glow as GlowContext,
        PrstClr::PrstClr, SchemeClr::SchemeClr, ScrgbClr::ScrgbClr,
        SrgbClr::SrgbClr, SysClr::SysClr, ThemeContext::ThemeContext},
    helper::{ColorsTrans::ColorsTrans, MetricHelper::MetricHelper},
};

// Java: public class GlowImporter extends DmlImporterBase
pub struct GlowImporter<ShapeImporter, DmlCommonImporter> {
    pub super_: DmlImporterBase<ShapeImporter, DmlCommonImporter>,
    // Java: private Glow mTarget;
    pub mTarget: Option<Rc<RefCell<Glow>>>,
    // Java: private boolean mIsText;
    pub mIsText: bool,
}
impl<S, D: IImporterBase> Deref for GlowImporter<S, D> {
    type Target = DmlImporterBase<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for GlowImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}
impl<S, D: IImporterBase> GlowImporter<S, D> {
    // Java: GlowImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext,
    //                    Xml07Context, DmlCommonImporter, boolean)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>,
        ctxt: Rc<RefCell<DmlShapeCtxt>>, themeCtxt: Rc<RefCell<ThemeContext>>,
        xml07Context: Option<Xml07Context>, dmlImporter: Rc<RefCell<D>>,
        isText: bool) -> Self {
        Self { super_: DmlImporterBase::new(subDocCtxt, ctxt, themeCtxt,
            xml07Context, dmlImporter), mTarget: None, mIsText: isText }
    }
    // Java: public void createGlowContext(Attributes attrs)
    pub fn createGlowContext(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mGlow = new cn.wps.moffice.writer.io.reader.dmlReader.context.Glow(attrs);
        self.mCtxt.borrow_mut().mGlow = Some(GlowContext::new(attrs));
    }
    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        // Java: this.mTarget = null;
        self.mTarget = None;
        // Java: this.mCtxt.mGlow = null;
        self.mCtxt.borrow_mut().mGlow = None;
    }
    // Java: public void excute()
    pub fn excute(&mut self) {
        // Java: this.initTarget();
        self.initTarget();
        let ctxt = self.mCtxt.borrow();
        let glow = ctxt.mGlow.as_ref().expect("mGlow");
        let mut target = self.mTarget.as_ref().expect("mTarget").borrow_mut();
        // Java: if (null != this.mCtxt.mGlow.mRad) this.mTarget.setGlowSize(this.mCtxt.mGlow.mRad);
        if let Some(rad) = glow.mRad { target.setGlowSize(rad); }
        // Java: float alpha = ColorsTrans.toTransparency(this.mCtxt.mGlow, this.mIsText);
        let alpha = ColorsTrans::toTransparency(glow, self.mIsText);
        // Java: this.mTarget.setGlowOpacity(1.0F - alpha);
        target.setGlowOpacity(1.0 - alpha);
        // Java: Integer clr = ColorsTrans.toColor(this.mCtxt.mPhClr,
        //                                            (ColorChoice)this.mCtxt.mGlow, this.mThemeCtxt);
        let theme = self.mThemeCtxt.borrow();
        let clr = ColorsTrans::toColor(ctxt.mPhClr, glow, Some(&theme));
        // Java: if (null != clr) this.mTarget.setGlowColor(ColorsTrans.rgb2Argb(clr, 1.0F - alpha));
        if let Some(clr) = clr { target.setGlowColor(ColorsTrans::rgb2Argb(clr, 1.0 - alpha)); }
    }
    // Java: public void initTarget()
    pub fn initTarget(&mut self) {
        // Java: if (null == this.mTarget) this.mTarget = new Glow();
        if self.mTarget.is_none() { self.mTarget = Some(Rc::new(RefCell::new(Glow::new()))); }
    }
    // Java: public Glow result()
    pub fn result(&self) -> Option<Rc<RefCell<Glow>>> { self.mTarget.clone() }

    // Java: public void createSysClr(Attributes attrs)
    pub fn createSysClr(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform = new SysClr(attrs);
        self.mCtxt.borrow_mut().mClrTransform =
            Some(ColorTransformValue::SysClr(SysClr::new(attrs)));
    }

    // Java: public void createPrstClr(Attributes attrs)
    pub fn createPrstClr(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform = new PrstClr(attrs);
        self.mCtxt.borrow_mut().mClrTransform =
            Some(ColorTransformValue::PrstClr(PrstClr::new(attrs)));
    }

    // Java: public void createSchemeClr(Attributes attrs)
    pub fn createSchemeClr(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform = new SchemeClr(attrs);
        self.mCtxt.borrow_mut().mClrTransform =
            Some(ColorTransformValue::SchemeClr(SchemeClr::new(attrs)));
    }

    // Java: public void createSrgbClr(Attributes attrs)
    pub fn createSrgbClr(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform = new SrgbClr(attrs);
        self.mCtxt.borrow_mut().mClrTransform =
            Some(ColorTransformValue::SrgbClr(SrgbClr::new(attrs)));
    }

    // Java: public void createScrgbClr(Attributes attrs)
    pub fn createScrgbClr(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform = new ScrgbClr(attrs);
        self.mCtxt.borrow_mut().mClrTransform =
            Some(ColorTransformValue::ScrgbClr(ScrgbClr::new(attrs)));
    }

    // Java: public void setSat(Attributes attrs)
    pub fn setSat(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mSat = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mSat =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setSatMod(Attributes attrs)
    pub fn setSatMod(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mSatMod = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mSatMod =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setSatOff(Attributes attrs)
    pub fn setSatOff(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mSatOff = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mSatOff =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setLum(Attributes attrs)
    pub fn setLum(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mLum = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mLum =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setLumMod(Attributes attrs)
    pub fn setLumMod(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mLumMod = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mLumMod =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setLumOff(Attributes attrs)
    pub fn setLumOff(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mLumOff = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mLumOff =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setRed(Attributes attrs)
    pub fn setRed(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mRed = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mRed =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setRedMod(Attributes attrs)
    pub fn setRedMod(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mRedMod = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mRedMod =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setRedOff(Attributes attrs)
    pub fn setRedOff(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mRedOff = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mRedOff =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setGreen(Attributes attrs)
    pub fn setGreen(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mGreen = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mGreen =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setGreenMod(Attributes attrs)
    pub fn setGreenMod(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mGreenMod = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mGreenMod =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setGreenOff(Attributes attrs)
    pub fn setGreenOff(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mGreenOff = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mGreenOff =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setBlue(Attributes attrs)
    pub fn setBlue(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mBlue = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mBlue =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setBlueMod(Attributes attrs)
    pub fn setBlueMod(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mBlueMod = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mBlueMod =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setBlueOff(Attributes attrs)
    pub fn setBlueOff(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mBlueOff = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mBlueOff =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setShade(Attributes attrs)
    pub fn setShade(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mShade = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mShade =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setAlpha(Attributes attrs)
    pub fn setAlpha(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mAlpha = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mAlpha =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setTint(Attributes attrs)
    pub fn setTint(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mClrTransform.mTint = MetricHelper.percent2Float(attrs, "val");
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mTint =
            MetricHelper::percent2Float(attrs, "val");
    }

    // Java: public void setSysClr()
    pub fn setSysClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SysClr(color) = transform else {
            panic!("mClrTransform is not SysClr");
        };
        // Java: this.mCtxt.mGlow.mSysClr = (SysClr)this.mCtxt.mClrTransform;
        // Java: this.mCtxt.mClrTransform = null;
        self.mCtxt.borrow_mut().mGlow.as_mut().expect("mGlow").mSysClr = Some(color);
    }

    // Java: public void setPrstClr()
    pub fn setPrstClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::PrstClr(color) = transform else {
            panic!("mClrTransform is not PrstClr");
        };
        // Java: this.mCtxt.mGlow.mPrstClr = (PrstClr)this.mCtxt.mClrTransform;
        // Java: this.mCtxt.mClrTransform = null;
        self.mCtxt.borrow_mut().mGlow.as_mut().expect("mGlow").mPrstClr = Some(color);
    }

    // Java: public void setSchemeClr()
    pub fn setSchemeClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SchemeClr(color) = transform else {
            panic!("mClrTransform is not SchemeClr");
        };
        // Java: this.mCtxt.mGlow.mSchemeClr = (SchemeClr)this.mCtxt.mClrTransform;
        // Java: this.mCtxt.mClrTransform = null;
        self.mCtxt.borrow_mut().mGlow.as_mut().expect("mGlow").mSchemeClr = Some(color);
    }

    // Java: public void setSrgbClr()
    pub fn setSrgbClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SrgbClr(color) = transform else {
            panic!("mClrTransform is not SrgbClr");
        };
        // Java: this.mCtxt.mGlow.mSrgbClr = (SrgbClr)this.mCtxt.mClrTransform;
        // Java: this.mCtxt.mClrTransform = null;
        self.mCtxt.borrow_mut().mGlow.as_mut().expect("mGlow").mSrgbClr = Some(color);
    }

    // Java: public void setScrgbClr()
    pub fn setScrgbClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::ScrgbClr(color) = transform else {
            panic!("mClrTransform is not ScrgbClr");
        };
        // Java: this.mCtxt.mGlow.mScrgbClr = (ScrgbClr)this.mCtxt.mClrTransform;
        // Java: this.mCtxt.mClrTransform = null;
        self.mCtxt.borrow_mut().mGlow.as_mut().expect("mGlow").mScrgbClr = Some(color);
    }
}
