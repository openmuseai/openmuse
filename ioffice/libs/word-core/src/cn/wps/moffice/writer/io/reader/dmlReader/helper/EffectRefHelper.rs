#![allow(non_snake_case)]

use io_wps_io::cn::wps::io::xwpf::{IDmlImporter::IDmlImporter, SaxAttributes::SaxAttributes};
use crate::cn::wps::moffice::writer::io::reader::dmlReader::{
    context::{Branch::ElementRef, ColorTransform::ColorTransformValue,
        DmlShapeCtxt::DmlShapeCtxt, EffectRef::EffectRef, PrstClr::PrstClr,
        SchemeClr::SchemeClr, ScrgbClr::ScrgbClr, SrgbClr::SrgbClr,
        SysClr::SysClr, ThemeContext::ThemeContext},
    helper::{ColorsTrans::ColorsTrans, MetricHelper::MetricHelper},
};

// Java: public class EffectRefHelper
pub struct EffectRefHelper;
impl EffectRefHelper {
    // Java: if (null != ctxt.mEffectRef) parseEffectRef(importer, ctxt, themeCtxt); ctxt.mEffectRef = null;
    // Rust's generic importer keeps the Java concrete DmlImporter call site available
    // while this class enters the compile surface before DmlImporter itself.
    pub fn importEffectByEffectRef<T: IDmlImporter>(importer: &mut T, ctxt: &mut DmlShapeCtxt, themeCtxt: &ThemeContext) {
        if ctxt.mEffectRef.is_some() { Self::parseEffectRef(importer, ctxt, themeCtxt); }
        ctxt.mEffectRef = None;
    }

    // Java: private static void parseEffectRef(DmlImporter importer, DmlShapeCtxt ctxt, ThemeContext themeCtxt)
    pub fn parseEffectRef<T: IDmlImporter>(importer: &mut T, ctxt: &mut DmlShapeCtxt, themeCtxt: &ThemeContext) {
        let Some(effect_ref) = ctxt.mEffectRef.as_ref() else { return; };
        let Some(idx) = effect_ref.mIdx else { return; };
        if !(1..=999).contains(&idx) { return; }
        // Java: Branch style = themeCtxt.mEffectStyleLst[ctxt.mEffectRef.mIdx - 1];
        let Some(style) = &themeCtxt.mEffectStyleLst[(idx - 1) as usize] else { return; };
        // Java: ctxt.mPhClr = ColorsTrans.toColor((Integer)null, (ColorChoice)ctxt.mEffectRef, themeCtxt);
        ctxt.mPhClr = ColorsTrans::toColor(None, effect_ref, Some(themeCtxt));
        let mut parentElementId = 0;
        // Java: if (0 == pop.type()) parseEffectRefStart(...); else parseEffectRefEnd(...);
        for e in &style.mElements {
            parentElementId = match e {
                ElementRef::Start(_) => Self::parseEffectRefStart(importer, e, parentElementId),
                ElementRef::End(_) => Self::parseEffectRefEnd(importer, e, parentElementId),
            };
        }
    }

    // Java: private static int parseEffectRefStart(DmlImporter importer, Element e, int parentElementId)
    pub fn parseEffectRefStart<T: IDmlImporter>(importer: &mut T, e: &ElementRef, mut parentElementId: i32) -> i32 {
        let ElementRef::Start(v) = e else { unreachable!("Java start tag"); };
        match v.mElement {
            // Java: case -1366299605: importer.startReflection(e.mAttrs);
            -1366299605 => importer.startReflection(&v.mAttrs),
            // Java: case -527990498: importer.startInnerShdw(e.mElement, e.mAttrs); parentElementId = -527990498;
            -527990498 => { importer.startInnerShdw(v.mElement, v.mAttrs.as_ref().expect("Java null Attributes")); parentElementId = -527990498; }
            // Java: case 1857009859: importer.startOuterShdw(e.mElement, e.mAttrs); parentElementId = 1857009859;
            1857009859 => { importer.startOuterShdw(v.mElement, v.mAttrs.as_ref().expect("Java null Attributes")); parentElementId = 1857009859; }
            // Java: default: parserChildEffectRefStart(importer, e, parentElementId);
            _ => Self::parserChildEffectRefStart(importer, e, parentElementId),
        }
        parentElementId
    }

    // Java: private static void parserChildEffectRefStart(DmlImporter importer, Element e, int parentElementId)
    pub fn parserChildEffectRefStart<T: IDmlImporter>(importer: &mut T, e: &ElementRef, parentElementId: i32) {
        let ElementRef::Start(v) = e else { unreachable!("Java start tag"); };
        match parentElementId {
            // Java: case -527990498: importer.startInnerShdw(e.mElement, e.mAttrs);
            -527990498 => importer.startInnerShdw(v.mElement, v.mAttrs.as_ref().expect("Java null Attributes")),
            // Java: case 1857009859: importer.startOuterShdw(e.mElement, e.mAttrs);
            1857009859 => importer.startOuterShdw(v.mElement, v.mAttrs.as_ref().expect("Java null Attributes")),
            _ => {}
        }
    }

    // Java: private static int parseEffectRefEnd(DmlImporter importer, Element e, int parentElementId)
    pub fn parseEffectRefEnd<T: IDmlImporter>(importer: &mut T, e: &ElementRef, parentElementId: i32) -> i32 {
        let ElementRef::End(v) = e else { unreachable!("Java end tag"); };
        match v.mElement {
            // Java: case -527990498: importer.endInnerShdw(e.mElement); return 0;
            -527990498 => { importer.endInnerShdw(v.mElement); 0 }
            // Java: case 1857009859: importer.endOuterShdw(e.mElement); return 0;
            1857009859 => { importer.endOuterShdw(v.mElement); 0 }
            // Java: default: parserChildEffectRefEnd(importer, e, parentElementId); return parentElementId;
            _ => { Self::parserChildEffectRefEnd(importer, e, parentElementId); parentElementId }
        }
    }

    // Java: private static void parserChildEffectRefEnd(DmlImporter importer, Element e, int parentElementId)
    pub fn parserChildEffectRefEnd<T: IDmlImporter>(importer: &mut T, e: &ElementRef, parentElementId: i32) {
        let ElementRef::End(v) = e else { unreachable!("Java end tag"); };
        match parentElementId {
            // Java: case -527990498: importer.endInnerShdw(e.mElement);
            -527990498 => importer.endInnerShdw(v.mElement),
            // Java: case 1857009859: importer.endOuterShdw(e.mElement);
            1857009859 => importer.endOuterShdw(v.mElement),
            _ => {}
        }
    }

    // Java: ctxt.mEffectRef = new EffectRef(attrs);
    pub fn createEffectRefContext(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mEffectRef = Some(EffectRef::new(attrs)); }

    // Java: ctxt.mClrTransform = new SysClr(attrs);
    pub fn createSysClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::SysClr(SysClr::new(attrs))); }
    // Java: ctxt.mEffectRef.mSysClr = (SysClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setSysClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::SysClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java SysClr cast"); };
        ctxt.mEffectRef.as_mut().expect("Java null effect ref").mSysClr = Some(value);
    }
    // Java: ctxt.mClrTransform = new PrstClr(attrs);
    pub fn createPrstClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::PrstClr(PrstClr::new(attrs))); }
    // Java: ctxt.mEffectRef.mPrstClr = (PrstClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setPrstClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::PrstClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java PrstClr cast"); };
        ctxt.mEffectRef.as_mut().expect("Java null effect ref").mPrstClr = Some(value);
    }
    // Java: ctxt.mClrTransform = new SchemeClr(attrs);
    pub fn createSchemeClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::SchemeClr(SchemeClr::new(attrs))); }
    // Java: ctxt.mEffectRef.mSchemeClr = (SchemeClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setSchemeClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::SchemeClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java SchemeClr cast"); };
        ctxt.mEffectRef.as_mut().expect("Java null effect ref").mSchemeClr = Some(value);
    }
    // Java: ctxt.mClrTransform = new SrgbClr(attrs);
    pub fn createSrgbClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::SrgbClr(SrgbClr::new(attrs))); }
    // Java: ctxt.mEffectRef.mSrgbClr = (SrgbClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setSrgbClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::SrgbClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java SrgbClr cast"); };
        ctxt.mEffectRef.as_mut().expect("Java null effect ref").mSrgbClr = Some(value);
    }
    // Java: ctxt.mClrTransform = new ScrgbClr(attrs);
    pub fn createScrgbClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::ScrgbClr(ScrgbClr::new(attrs))); }
    // Java: ctxt.mEffectRef.mScrgbClr = (ScrgbClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setScrgbClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::ScrgbClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java ScrgbClr cast"); };
        ctxt.mEffectRef.as_mut().expect("Java null effect ref").mScrgbClr = Some(value);
    }
    // Java: ctxt.mClrTransform.mSat = MetricHelper.percent2Float(attrs, "val");
    pub fn setSat(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mSat = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mSatMod = MetricHelper.percent2Float(attrs, "val");
    pub fn setSatMod(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mSatMod = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mSatOff = MetricHelper.percent2Float(attrs, "val");
    pub fn setSatOff(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mSatOff = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mLum = MetricHelper.percent2Float(attrs, "val");
    pub fn setLum(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mLum = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mLumMod = MetricHelper.percent2Float(attrs, "val");
    pub fn setLumMod(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mLumMod = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mLumOff = MetricHelper.percent2Float(attrs, "val");
    pub fn setLumOff(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mLumOff = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mRed = MetricHelper.percent2Float(attrs, "val");
    pub fn setRed(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mRed = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mRedMod = MetricHelper.percent2Float(attrs, "val");
    pub fn setRedMod(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mRedMod = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mRedOff = MetricHelper.percent2Float(attrs, "val");
    pub fn setRedOff(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mRedOff = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mGreen = MetricHelper.percent2Float(attrs, "val");
    pub fn setGreen(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mGreen = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mGreenMod = MetricHelper.percent2Float(attrs, "val");
    pub fn setGreenMod(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mGreenMod = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mGreenOff = MetricHelper.percent2Float(attrs, "val");
    pub fn setGreenOff(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mGreenOff = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mBlue = MetricHelper.percent2Float(attrs, "val");
    pub fn setBlue(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mBlue = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mBlueMod = MetricHelper.percent2Float(attrs, "val");
    pub fn setBlueMod(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mBlueMod = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mBlueOff = MetricHelper.percent2Float(attrs, "val");
    pub fn setBlueOff(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mBlueOff = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mShade = MetricHelper.percent2Float(attrs, "val");
    pub fn setShade(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mShade = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mAlpha = MetricHelper.percent2Float(attrs, "val");
    pub fn setAlpha(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mAlpha = MetricHelper::percent2Float(attrs, "val"); }
    // Java: ctxt.mClrTransform.mTint = MetricHelper.percent2Float(attrs, "val");
    pub fn setTint(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform.as_mut().expect("Java null color transform").mTint = MetricHelper::percent2Float(attrs, "val"); }
}

