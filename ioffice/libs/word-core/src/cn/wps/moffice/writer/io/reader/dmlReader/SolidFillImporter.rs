#![allow(non_snake_case)]

use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc};
use drawing::cn::wps::moffice::drawing::graphics::SolidFill::SolidFill;
use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    FillBaseImporter::FillBaseImporter,
    context::{ColorTransform::ColorTransformValue, DmlShapeCtxt::DmlShapeCtxt,
        DmlSubDocCtxt::DmlSubDocCtxt, SolidFill::SolidFill as SolidFillContext,
        ThemeContext::ThemeContext},
    helper::ColorsTrans::ColorsTrans,
};

// Java: public class SolidFillImporter extends FillBaseImporter
pub struct SolidFillImporter<ShapeImporter, DmlCommonImporter> {
    pub super_: FillBaseImporter<ShapeImporter, DmlCommonImporter, SolidFill>,
    // Java: private SolidFill mTarget; same instance as FillBaseImporter.mTarget.
    pub mTarget: Option<Rc<RefCell<SolidFill>>>,
    // Java: private boolean mIsText;
    pub mIsText: bool,
}

impl<S, D: IImporterBase> Deref for SolidFillImporter<S, D> {
    type Target = FillBaseImporter<S, D, SolidFill>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for SolidFillImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> SolidFillImporter<S, D> {
    // Java: SolidFillImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext,
    //                         Xml07Context, DmlCommonImporter, boolean)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>,
        ctxt: Rc<RefCell<DmlShapeCtxt>>, themeCtxt: Rc<RefCell<ThemeContext>>,
        xml07Context: Option<Xml07Context>, dmlImporter: Rc<RefCell<D>>, isText: bool) -> Self {
        Self { super_: FillBaseImporter::new(subDocCtxt, ctxt, themeCtxt,
            xml07Context, dmlImporter), mTarget: None, mIsText: isText }
    }

    // Java: public void excute()
    pub fn excute(&mut self) {
        // Java: this.initTarget();
        self.initTarget();
        let ctxt = self.mCtxt.borrow();
        let solid = ctxt.mSolidFill.as_ref().expect("mSolidFill");
        // Java: Integer clr = ColorsTrans.toColor(this.mCtxt.mPhClr,
        //                                             (ColorChoice)this.mCtxt.mSolidFill, this.mThemeCtxt);
        let theme = self.mThemeCtxt.borrow();
        let clr = ColorsTrans::toColor(ctxt.mPhClr, solid, Some(&theme));
        let mut target = self.mTarget.as_ref().expect("mTarget").borrow_mut();
        // Java: if (null != clr) { this.mTarget.setColor(clr); }
        if let Some(clr) = clr { target.base.setColor(clr); }
        // Java: float alpha = ColorsTrans.toTransparency(this.mCtxt.mSolidFill, this.mIsText);
        // Java: this.mTarget.setTransparency(alpha);
        target.base.setTransparency(ColorsTrans::toTransparency(solid, self.mIsText));
    }

    // Java: public SolidFill result()
    pub fn result(&self) -> Option<Rc<RefCell<SolidFill>>> { self.mTarget.clone() }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        // Java: super.clearContext();
        self.super_.clearContext();
        // Java: this.mTarget = null;
        self.mTarget = None;
        // Java: this.mCtxt.mSolidFill = null;
        self.mCtxt.borrow_mut().mSolidFill = None;
    }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        // Java: if (null == this.mTarget) { super.mTarget = this.mTarget = new SolidFill(); }
        if self.mTarget.is_none() {
            let target = Rc::new(RefCell::new(SolidFill::new()));
            self.super_.mTarget = Some(target.clone());
            self.mTarget = Some(target);
        }
    }

    // Java: public void createSolidFillContext()
    pub fn createSolidFillContext(&mut self) {
        // Java: this.mCtxt.mSolidFill = new cn.wps.moffice.writer.io.reader.dmlReader.context.SolidFill();
        self.mCtxt.borrow_mut().mSolidFill = Some(SolidFillContext::new());
    }

    // Java: public void setSysClr()
    pub fn setSysClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SysClr(color) = transform else { panic!("mClrTransform is not SysClr"); };
        // Java: this.mCtxt.mSolidFill.mSysClr = (SysClr)this.mCtxt.mClrTransform;
        self.mCtxt.borrow_mut().mSolidFill.as_mut().expect("mSolidFill").mSysClr = Some(color);
        // Java: this.mCtxt.mClrTransform = null;
    }
    // Java: public void setPrstClr()
    pub fn setPrstClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::PrstClr(color) = transform else { panic!("mClrTransform is not PrstClr"); };
        // Java: this.mCtxt.mSolidFill.mPrstClr = (PrstClr)this.mCtxt.mClrTransform;
        self.mCtxt.borrow_mut().mSolidFill.as_mut().expect("mSolidFill").mPrstClr = Some(color);
        // Java: this.mCtxt.mClrTransform = null;
    }
    // Java: public void setSchemeClr()
    pub fn setSchemeClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SchemeClr(color) = transform else { panic!("mClrTransform is not SchemeClr"); };
        // Java: this.mCtxt.mSolidFill.mSchemeClr = (SchemeClr)this.mCtxt.mClrTransform;
        self.mCtxt.borrow_mut().mSolidFill.as_mut().expect("mSolidFill").mSchemeClr = Some(color);
        // Java: this.mCtxt.mClrTransform = null;
    }
    // Java: public void setSrgbClr()
    pub fn setSrgbClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::SrgbClr(color) = transform else { panic!("mClrTransform is not SrgbClr"); };
        // Java: this.mCtxt.mSolidFill.mSrgbClr = (SrgbClr)this.mCtxt.mClrTransform;
        self.mCtxt.borrow_mut().mSolidFill.as_mut().expect("mSolidFill").mSrgbClr = Some(color);
        // Java: this.mCtxt.mClrTransform = null;
    }
    // Java: public void setScrgbClr()
    pub fn setScrgbClr(&mut self) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let ColorTransformValue::ScrgbClr(color) = transform else { panic!("mClrTransform is not ScrgbClr"); };
        // Java: this.mCtxt.mSolidFill.mScrgbClr = (ScrgbClr)this.mCtxt.mClrTransform;
        self.mCtxt.borrow_mut().mSolidFill.as_mut().expect("mSolidFill").mScrgbClr = Some(color);
        // Java: this.mCtxt.mClrTransform = null;
    }
}
