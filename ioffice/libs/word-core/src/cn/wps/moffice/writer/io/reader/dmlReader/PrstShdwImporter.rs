// Java: cn.wps.moffice.writer.io.reader.dmlReader.PrstShdwImporter
use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc};
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    InnerShdwImporter::apply_inner_common,
    ShadowImporter::{apply_shadow_scales, ShadowImporter},
    context::{
        ColorTransform::ColorTransformValue, DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt,
        PrstShdw::PrstShdw, ThemeContext::ThemeContext,
    },
    helper::{ColorsTrans::ColorsTrans, PrstShdwMap::PrstShdwMap},
};

// Java: public class PrstShdwImporter extends ShadowImporter
pub struct PrstShdwImporter<ShapeImporter, DmlCommonImporter> {
    pub super_: ShadowImporter<ShapeImporter, DmlCommonImporter>,
}

impl<S, D: IImporterBase> Deref for PrstShdwImporter<S, D> {
    type Target = ShadowImporter<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for PrstShdwImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> PrstShdwImporter<S, D> {
    // Java: PrstShdwImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext, Xml07Context, DmlImporter)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>, ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>, xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<D>>) -> Self {
        Self { super_: ShadowImporter::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter) }
    }

    // Java: public void createPrstShdw(Attributes attrs)
    pub fn createPrstShdw(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mPrstShdw = Some(PrstShdw::new(attrs));
    }

    // Java: public void excute()
    pub fn excute(&mut self) {
        let prst = self.mCtxt.borrow_mut().mPrstShdw.take();
        let Some(prst) = prst else { return; };
        self.importPrstShdw(&prst);
        // Java: super.importCommonProp(this.mCtxt.mPrstShdw) uses this class's getAlpha (isText = false).
        let alpha = ColorsTrans::toTransparency(&prst, false);
        let ph_clr = self.mCtxt.borrow().mPhClr;
        let theme = self.mThemeCtxt.borrow();
        {
            let mut target = self.mTarget.as_ref().expect("mTarget").lock().expect("shadow");
            apply_inner_common(&mut target, &prst.base.base, alpha, ph_clr, &theme);
            apply_shadow_scales(&mut target, &prst.base.base);
        }
        self.mCtxt.borrow_mut().mPrstShdw = Some(prst);
    }

    // Java: private void importPrstShdw()
    fn importPrstShdw(&mut self, prst: &PrstShdw) {
        if let Some(shadow) = PrstShdwMap::getPresetShadow(prst.mPrst) {
            self.mTarget = Some(shadow);
        }
    }

    pub fn setSysClr(&mut self) { self.move_color(ColorSlot::Sys); }
    pub fn setPrstClr(&mut self) { self.move_color(ColorSlot::Prst); }
    pub fn setSchemeClr(&mut self) { self.move_color(ColorSlot::Scheme); }
    pub fn setSrgbClr(&mut self) { self.move_color(ColorSlot::Srgb); }
    pub fn setScrgbClr(&mut self) { self.move_color(ColorSlot::Scrgb); }

    fn move_color(&mut self, slot: ColorSlot) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let mut ctxt = self.mCtxt.borrow_mut();
        let prst = ctxt.mPrstShdw.as_mut().expect("mPrstShdw");
        match (slot, transform) {
            (ColorSlot::Sys, ColorTransformValue::SysClr(color)) => prst.mSysClr = Some(color),
            (ColorSlot::Prst, ColorTransformValue::PrstClr(color)) => prst.mPrstClr = Some(color),
            (ColorSlot::Scheme, ColorTransformValue::SchemeClr(color)) => prst.mSchemeClr = Some(color),
            (ColorSlot::Srgb, ColorTransformValue::SrgbClr(color)) => prst.mSrgbClr = Some(color),
            (ColorSlot::Scrgb, ColorTransformValue::ScrgbClr(color)) => prst.mScrgbClr = Some(color),
            _ => panic!("mClrTransform type does not match set*Clr"),
        }
    }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        self.mTarget = None;
        self.mCtxt.borrow_mut().mPrstShdw = None;
    }

    // Java: protected float getAlpha()
    pub fn getAlpha(&self) -> f32 {
        let ctxt = self.mCtxt.borrow();
        ColorsTrans::toTransparency(ctxt.mPrstShdw.as_ref().expect("mPrstShdw"), false)
    }
}

enum ColorSlot { Sys, Prst, Scheme, Srgb, Scrgb }
