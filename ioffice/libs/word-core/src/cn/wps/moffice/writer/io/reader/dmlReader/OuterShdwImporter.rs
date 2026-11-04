// Java: cn.wps.moffice.writer.io.reader.dmlReader.OuterShdwImporter
use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc};
use drawing::cn::wps::moffice::drawing::effects::{ShadowDml::ShadowDml, ShadowPos::ShadowPos};
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    InnerShdwImporter::apply_inner_common,
    ShadowImporter::{apply_shadow_algn, apply_shadow_scales, ShadowImporter},
    context::{
        ColorTransform::ColorTransformValue, DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt,
        OuterShdw::OuterShdw, ThemeContext::ThemeContext,
    },
    helper::ColorsTrans::ColorsTrans,
};

// Java: public class OuterShdwImporter extends ShadowImporter
pub struct OuterShdwImporter<ShapeImporter, DmlCommonImporter> {
    pub super_: ShadowImporter<ShapeImporter, DmlCommonImporter>,
}

impl<S, D: IImporterBase> Deref for OuterShdwImporter<S, D> {
    type Target = ShadowImporter<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for OuterShdwImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> OuterShdwImporter<S, D> {
    // Java: OuterShdwImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext, Xml07Context, DmlImporter)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>, ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>, xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<D>>) -> Self {
        Self { super_: ShadowImporter::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter) }
    }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        if self.mTarget.is_none() {
            let mut shadow = ShadowDml::new();
            shadow.setShadowPos(ShadowPos::Outer);
            self.mTarget = Some(std::sync::Arc::new(std::sync::Mutex::new(shadow)));
        }
    }

    // Java: public void createOuterShdw(Attributes attrs)
    pub fn createOuterShdw(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mOuterShdw = Some(OuterShdw::new(attrs));
    }

    // Java: public void excute()
    pub fn excute(&mut self) {
        self.initTarget();
        let outer = self.mCtxt.borrow_mut().mOuterShdw.take();
        let Some(outer) = outer else { return; };
        // Java: importAlgn first (default 0), then ShadowImporter.importCommonProp, then rotWithShape.
        // getAlpha is this class's override (isText = false).
        let alpha = ColorsTrans::toTransparency(&outer, false);
        let ph_clr = self.mCtxt.borrow().mPhClr;
        let theme = self.mThemeCtxt.borrow();
        {
            let mut target = self.mTarget.as_ref().expect("mTarget").lock().expect("shadow");
            apply_shadow_algn(&mut target, outer.mAlgn.as_deref(), 0);
            apply_inner_common(&mut target, &outer.base, alpha, ph_clr, &theme);
            apply_shadow_scales(&mut target, &outer.base);
            if let Some(rot) = outer.mRotWithShape {
                target.base.setRotWithShape(rot);
            }
        }
        self.mCtxt.borrow_mut().mOuterShdw = Some(outer);
    }

    pub fn setSysClr(&mut self) { self.move_color(ColorSlot::Sys); }
    pub fn setPrstClr(&mut self) { self.move_color(ColorSlot::Prst); }
    pub fn setSchemeClr(&mut self) { self.move_color(ColorSlot::Scheme); }
    pub fn setSrgbClr(&mut self) { self.move_color(ColorSlot::Srgb); }
    pub fn setScrgbClr(&mut self) { self.move_color(ColorSlot::Scrgb); }

    fn move_color(&mut self, slot: ColorSlot) {
        let transform = self.mCtxt.borrow_mut().mClrTransform.take().expect("mClrTransform");
        let mut ctxt = self.mCtxt.borrow_mut();
        let outer = ctxt.mOuterShdw.as_mut().expect("mOuterShdw");
        match (slot, transform) {
            (ColorSlot::Sys, ColorTransformValue::SysClr(color)) => outer.mSysClr = Some(color),
            (ColorSlot::Prst, ColorTransformValue::PrstClr(color)) => outer.mPrstClr = Some(color),
            (ColorSlot::Scheme, ColorTransformValue::SchemeClr(color)) => outer.mSchemeClr = Some(color),
            (ColorSlot::Srgb, ColorTransformValue::SrgbClr(color)) => outer.mSrgbClr = Some(color),
            (ColorSlot::Scrgb, ColorTransformValue::ScrgbClr(color)) => outer.mScrgbClr = Some(color),
            _ => panic!("mClrTransform type does not match set*Clr"),
        }
    }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        self.mTarget = None;
        self.mCtxt.borrow_mut().mOuterShdw = None;
    }

    // Java: protected float getAlpha()
    pub fn getAlpha(&self) -> f32 {
        let ctxt = self.mCtxt.borrow();
        ColorsTrans::toTransparency(ctxt.mOuterShdw.as_ref().expect("mOuterShdw"), false)
    }
}

enum ColorSlot { Sys, Prst, Scheme, Srgb, Scrgb }
