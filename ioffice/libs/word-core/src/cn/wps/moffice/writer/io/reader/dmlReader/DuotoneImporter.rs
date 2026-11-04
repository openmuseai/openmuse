#![allow(non_snake_case)]
use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc};
use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::DmlImporterBase::DmlImporterBase;
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use super::context::{
    DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt,
    ThemeContext::ThemeContext, ColorTransform::ColorTransformValue,
    ColorChoice::ColorChoice, ClrFrom::ClrFrom, Duotone::Duotone,
    SysClr::SysClr, PrstClr::PrstClr, SchemeClr::SchemeClr,
    SrgbClr::SrgbClr, ScrgbClr::ScrgbClr,
};
use super::helper::MetricHelper::MetricHelper;

// Java: public class DuotoneImporter extends DmlImporterBase
pub struct DuotoneImporter<ShapeImporter, DmlCommonImporter> {
    pub super_: DmlImporterBase<ShapeImporter, DmlCommonImporter>,
    // Java: ColorChoice mCurrentColorChoice;
    pub mCurrentColorChoice: Option<ColorChoice>,
}
impl<ShapeImporter, DmlCommonImporter: IImporterBase> Deref for DuotoneImporter<ShapeImporter, DmlCommonImporter> {
    type Target = DmlImporterBase<ShapeImporter, DmlCommonImporter>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<ShapeImporter, DmlCommonImporter: IImporterBase> DerefMut for DuotoneImporter<ShapeImporter, DmlCommonImporter> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}
impl<ShapeImporter, DmlCommonImporter: IImporterBase> DuotoneImporter<ShapeImporter, DmlCommonImporter> {
    // Java: DuotoneImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext, Xml07Context, DmlCommonImporter)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<ShapeImporter>>>,
        ctxt: Rc<RefCell<DmlShapeCtxt>>, themeCtxt: Rc<RefCell<ThemeContext>>,
        xml07Context: Option<Xml07Context>, dmlImporter: Rc<RefCell<DmlCommonImporter>>) -> Self {
        Self { super_: DmlImporterBase::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter), mCurrentColorChoice: None }
    }
    // Java: public void createSysClr(Attributes attrs)
    pub fn createSysClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCurrentColorChoice = Some(ColorChoice::new());
        self.mCtxt.borrow_mut().mClrTransform = Some(ColorTransformValue::SysClr(SysClr::new(attrs)));
    }
    // Java: public void createPrstClr(Attributes attrs)
    pub fn createPrstClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCurrentColorChoice = Some(ColorChoice::new());
        self.mCtxt.borrow_mut().mClrTransform = Some(ColorTransformValue::PrstClr(PrstClr::new(attrs)));
    }
    // Java: public void createSchemeClr(Attributes attrs)
    pub fn createSchemeClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCurrentColorChoice = Some(ColorChoice::new());
        self.mCtxt.borrow_mut().mClrTransform = Some(ColorTransformValue::SchemeClr(SchemeClr::new(attrs)));
    }
    // Java: public void createSrgbClr(Attributes attrs)
    pub fn createSrgbClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCurrentColorChoice = Some(ColorChoice::new());
        self.mCtxt.borrow_mut().mClrTransform = Some(ColorTransformValue::SrgbClr(SrgbClr::new(attrs)));
    }
    // Java: public void createScrgbClr(Attributes attrs)
    pub fn createScrgbClr(&mut self, attrs: &dyn SaxAttributes) {
        self.mCurrentColorChoice = Some(ColorChoice::new());
        self.mCtxt.borrow_mut().mClrTransform = Some(ColorTransformValue::ScrgbClr(ScrgbClr::new(attrs)));
    }
    // Java: public void setSat(Attributes attrs)
    pub fn setSat(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mSat = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setSatMod(Attributes attrs)
    pub fn setSatMod(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mSatMod = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setSatOff(Attributes attrs)
    pub fn setSatOff(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mSatOff = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setLum(Attributes attrs)
    pub fn setLum(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mLum = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setLumMod(Attributes attrs)
    pub fn setLumMod(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mLumMod = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setLumOff(Attributes attrs)
    pub fn setLumOff(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mLumOff = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setRed(Attributes attrs)
    pub fn setRed(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mRed = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setRedMod(Attributes attrs)
    pub fn setRedMod(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mRedMod = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setRedOff(Attributes attrs)
    pub fn setRedOff(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mRedOff = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setGreen(Attributes attrs)
    pub fn setGreen(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mGreen = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setGreenMod(Attributes attrs)
    pub fn setGreenMod(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mGreenMod = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setGreenOff(Attributes attrs)
    pub fn setGreenOff(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mGreenOff = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setBlue(Attributes attrs)
    pub fn setBlue(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mBlue = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setBlueMod(Attributes attrs)
    pub fn setBlueMod(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mBlueMod = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setBlueOff(Attributes attrs)
    pub fn setBlueOff(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mBlueOff = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setShade(Attributes attrs)
    pub fn setShade(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mShade = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setAlpha(Attributes attrs)
    pub fn setAlpha(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mAlpha = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setTint(Attributes attrs)
    pub fn setTint(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mClrTransform.as_mut().expect("mClrTransform").mTint = MetricHelper::percent2Float(attrs, "val");
    }
    // Java: public void setSysClr()
    pub fn setSysClr(&mut self) {
        // Java casts mClrTransform to SysClr before assigning the same object.
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SysClr(color) = transform else { panic!("mClrTransform is not SysClr"); };
        self.mCurrentColorChoice.as_mut().expect("mCurrentColorChoice").mSysClr = Some(color);
        self.add2Duotone();
    }
    // Java: public void setPrstClr()
    pub fn setPrstClr(&mut self) {
        // Java casts mClrTransform to PrstClr before assigning the same object.
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::PrstClr(color) = transform else { panic!("mClrTransform is not PrstClr"); };
        self.mCurrentColorChoice.as_mut().expect("mCurrentColorChoice").mPrstClr = Some(color);
        self.add2Duotone();
    }
    // Java: public void setSchemeClr()
    pub fn setSchemeClr(&mut self) {
        // Java casts mClrTransform to SchemeClr before assigning the same object.
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SchemeClr(color) = transform else { panic!("mClrTransform is not SchemeClr"); };
        self.mCurrentColorChoice.as_mut().expect("mCurrentColorChoice").mSchemeClr = Some(color);
        self.add2Duotone();
    }
    // Java: public void setSrgbClr()
    pub fn setSrgbClr(&mut self) {
        // Java casts mClrTransform to SrgbClr before assigning the same object.
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SrgbClr(color) = transform else { panic!("mClrTransform is not SrgbClr"); };
        self.mCurrentColorChoice.as_mut().expect("mCurrentColorChoice").mSrgbClr = Some(color);
        self.add2Duotone();
    }
    // Java: public void setScrgbClr()
    pub fn setScrgbClr(&mut self) {
        // Java casts mClrTransform to ScrgbClr before assigning the same object.
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::ScrgbClr(color) = transform else { panic!("mClrTransform is not ScrgbClr"); };
        self.mCurrentColorChoice.as_mut().expect("mCurrentColorChoice").mScrgbClr = Some(color);
        self.add2Duotone();
    }
    // Java: private void add2Duotone()
    fn add2Duotone(&mut self) {
        let color = self.mCurrentColorChoice.take().expect("mCurrentColorChoice");
        self.mCtxt.borrow_mut().mDuotone.as_mut().expect("mDuotone").addColor(color);
    }
    // Java: public void createDuotoneContext()
    pub fn createDuotoneContext(&mut self) {
        self.mCtxt.borrow_mut().mDuotone = Some(Duotone::new());
    }
}
