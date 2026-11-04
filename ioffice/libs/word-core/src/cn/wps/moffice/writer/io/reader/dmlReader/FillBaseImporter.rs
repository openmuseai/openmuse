#![allow(non_snake_case)]
use super::{
    context::{
        ColorTransform::ColorTransformValue, DmlShapeCtxt::DmlShapeCtxt,
        DmlSubDocCtxt::DmlSubDocCtxt, PrstClr::PrstClr, SchemeClr::SchemeClr, ScrgbClr::ScrgbClr,
        SrgbClr::SrgbClr, SysClr::SysClr, ThemeContext::ThemeContext,
    },
    helper::MetricHelper::MetricHelper,
    DmlImporterBase::DmlImporterBase,
};
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use drawing::cn::wps::moffice::drawing::graphics::FillBase::FillBase;
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

// Java: public abstract class FillBaseImporter extends DmlImporterBase
// The target type specializes Java's FillBase reference to its concrete subtype;
// both Java field slots can then hold one shared Rust object without cloning it.
pub struct FillBaseImporter<ShapeImporter, DmlCommonImporter, Target = FillBase> {
    pub super_: DmlImporterBase<ShapeImporter, DmlCommonImporter>,
    // Java: protected FillBase mTarget; BlipFillImporter shadows this field.
    pub mTarget: Option<Rc<RefCell<Target>>>,
}

impl<S, D: IImporterBase, T> Deref for FillBaseImporter<S, D, T> {
    type Target = DmlImporterBase<S, D>;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}
impl<S, D: IImporterBase, T> DerefMut for FillBaseImporter<S, D, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl<S, D: IImporterBase, T> FillBaseImporter<S, D, T> {
    // Java: public static final float DEFAULT_FOCUS_POSITION = 0.0F;
    pub const DEFAULT_FOCUS_POSITION: f32 = 0.0;

    // Java: FillBaseImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext,
    //                        Xml07Context, DmlCommonImporter)
    pub fn new(
        subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>,
        ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>,
        xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<D>>,
    ) -> Self {
        Self {
            super_: DmlImporterBase::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter),
            mTarget: None,
        }
    }

    // Java: public void createSysClr(Attributes attrs)
    pub fn createSysClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform =
            Some(ColorTransformValue::SysClr(SysClr::new(attrs)));
    }
    // Java: public void createPrstClr(Attributes attrs)
    pub fn createPrstClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform =
            Some(ColorTransformValue::PrstClr(PrstClr::new(attrs)));
    }
    // Java: public void createSchemeClr(Attributes attrs)
    pub fn createSchemeClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform =
            Some(ColorTransformValue::SchemeClr(SchemeClr::new(attrs)));
    }
    // Java: public void createSrgbClr(Attributes attrs)
    pub fn createSrgbClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform =
            Some(ColorTransformValue::SrgbClr(SrgbClr::new(attrs)));
    }
    // Java: public void createScrgbClr(Attributes attrs)
    pub fn createScrgbClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform =
            Some(ColorTransformValue::ScrgbClr(ScrgbClr::new(attrs)));
    }

    // Java's transform setters write to the currently selected concrete color.
    // Keep each Java method name on the Rust facade.
    // Java: public void setSat(Attributes attrs)
    pub fn setSat(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mSat = v);
    }
    // Java: public void setSatMod(Attributes attrs)
    pub fn setSatMod(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mSatMod = v);
    }
    // Java: public void setSatOff(Attributes attrs)
    pub fn setSatOff(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mSatOff = v);
    }
    // Java: public void setLum(Attributes attrs)
    pub fn setLum(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mLum = v);
    }
    // Java: public void setLumMod(Attributes attrs)
    pub fn setLumMod(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mLumMod = v);
    }
    // Java: public void setLumOff(Attributes attrs)
    pub fn setLumOff(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mLumOff = v);
    }
    // Java: public void setRed(Attributes attrs)
    pub fn setRed(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mRed = v);
    }
    // Java: public void setRedMod(Attributes attrs)
    pub fn setRedMod(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mRedMod = v);
    }
    // Java: public void setRedOff(Attributes attrs)
    pub fn setRedOff(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mRedOff = v);
    }
    // Java: public void setGreen(Attributes attrs)
    pub fn setGreen(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mGreen = v);
    }
    // Java: public void setGreenMod(Attributes attrs)
    pub fn setGreenMod(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mGreenMod = v);
    }
    // Java: public void setGreenOff(Attributes attrs)
    pub fn setGreenOff(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mGreenOff = v);
    }
    // Java: public void setBlue(Attributes attrs)
    pub fn setBlue(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mBlue = v);
    }
    // Java: public void setBlueMod(Attributes attrs)
    pub fn setBlueMod(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mBlueMod = v);
    }
    // Java: public void setBlueOff(Attributes attrs)
    pub fn setBlueOff(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mBlueOff = v);
    }
    // Java: public void setShade(Attributes attrs)
    pub fn setShade(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mShade = v);
    }
    // Java: public void setTint(Attributes attrs)
    pub fn setTint(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mTint = v);
    }
    // Java: public void setAlpha(Attributes attrs)
    pub fn setAlpha(&mut self, attrs: &dyn SaxAttributes) {
        self.setTransform(attrs, |c, v| c.mAlpha = v);
    }

    fn setTransform(
        &mut self,
        attrs: &dyn SaxAttributes,
        assign: impl FnOnce(&mut ColorTransformValue, Option<f32>),
    ) {
        let value = MetricHelper::percent2Float(attrs, "val");
        let mut ctxt = self.mCtxt.borrow_mut();
        assign(ctxt.mClrTransform.as_mut().expect("mClrTransform"), value);
    }

    // Java: public void clearContext() { this.mTarget = null; }
    pub fn clearContext(&mut self) {
        self.mTarget = None;
    }
}
