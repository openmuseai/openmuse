#![allow(non_snake_case)]

use io_wps_io::cn::wps::io::xwpf::{IDmlImporter::IDmlImporter, SaxAttributes::SaxAttributes};
use crate::cn::wps::moffice::writer::io::reader::dmlReader::{
    context::{Branch::ElementRef, ColorTransform::ColorTransformValue,
        DmlShapeCtxt::DmlShapeCtxt, LnRef::LnRef, PrstClr::PrstClr,
        SchemeClr::SchemeClr, ScrgbClr::ScrgbClr, SrgbClr::SrgbClr,
        SysClr::SysClr, ThemeContext::ThemeContext},
    helper::{ColorsTrans::ColorsTrans, MetricHelper::MetricHelper},
};

// Java: public class LnRefHelper
pub struct LnRefHelper;
impl LnRefHelper {
    // Java: if (null != ctxt.mLnRef) parseLnRef(importer, ctxt, themeCtxt); ctxt.mLnRef = null;
    pub fn importLinePropertyByLnRef(importer: &mut dyn IDmlImporter, ctxt: &mut DmlShapeCtxt, themeCtxt: &ThemeContext) {
        if ctxt.mLnRef.is_some() { Self::parseLnRef(importer, ctxt, themeCtxt); }
        ctxt.mLnRef = None;
    }

    // Java: if (null != ctxt.mLnRef.mIdx && 1 <= ... && ... <= 999 && null != themeCtxt)
    pub fn parseLnRef(importer: &mut dyn IDmlImporter, ctxt: &mut DmlShapeCtxt, themeCtxt: &ThemeContext) {
        let Some(ln_ref) = ctxt.mLnRef.as_ref() else { return; };
        let Some(idx) = ln_ref.mIdx else { return; };
        if !(1..=999).contains(&idx) { return; }
        // Java: Branch style = themeCtxt.mLnStyleLst[ctxt.mLnRef.mIdx - 1];
        let Some(style) = &themeCtxt.mLnStyleLst[(idx - 1) as usize] else { return; };
        // Java: ctxt.mPhClr = ColorsTrans.toColor((Integer)null, (ColorChoice)ctxt.mLnRef, themeCtxt);
        ctxt.mPhClr = ColorsTrans::toColor(None, ln_ref, Some(themeCtxt));
        // Java: int parentElementId = -1; for (...) { if (0 == pop.type()) ... else ... }
        let mut parentElementId = -1;
        for e in &style.mElements {
            parentElementId = match e {
                ElementRef::Start(_) => Self::parseLnRefStart(importer, e, parentElementId),
                ElementRef::End(_) => Self::parseLnRefEnd(importer, e, parentElementId),
            };
        }
    }

    // Java: private static int parseLnRefStart(IDmlImporter importer, Element e, int parentElementId)
    pub fn parseLnRefStart(importer: &mut dyn IDmlImporter, e: &ElementRef, mut parentElementId: i32) -> i32 {
        let ElementRef::Start(start) = e else { unreachable!("Java start tag"); };
        match start.mElement {
            // Java: importer.startSolidFill(3458, e.mElement, e.mAttrs); parentElementId = -1353661746;
            -1353661746 => { importer.startSolidFill(3458, start.mElement, &start.mAttrs); parentElementId = -1353661746; }
            // Java: importer.startLn(e.mElement, e.mAttrs); parentElementId = 3458;
            3458 => { importer.startLn(start.mElement, &start.mAttrs); parentElementId = 3458; }
            // Java: default: parseLnChildRefStart(importer, e, parentElementId);
            _ => Self::parseLnChildRefStart(importer, e, parentElementId),
        }
        parentElementId
    }

    // Java: private static void parseLnChildRefStart(IDmlImporter importer, Element e, int parentElementId)
    pub fn parseLnChildRefStart(importer: &mut dyn IDmlImporter, e: &ElementRef, parentElementId: i32) {
        let ElementRef::Start(start) = e else { unreachable!("Java start tag"); };
        match parentElementId {
            // Java: case -1353661746: importer.startSolidFill(3458, e.mElement, e.mAttrs);
            -1353661746 => importer.startSolidFill(3458, start.mElement, &start.mAttrs),
            // Java: case 3458: importer.startLn(e.mElement, e.mAttrs);
            3458 => importer.startLn(start.mElement, &start.mAttrs),
            _ => {}
        }
    }

    // Java: private static int parseLnRefEnd(IDmlImporter importer, Element e, int parentElementId)
    pub fn parseLnRefEnd(importer: &mut dyn IDmlImporter, e: &ElementRef, mut parentElementId: i32) -> i32 {
        let ElementRef::End(end) = e else { unreachable!("Java end tag"); };
        match end.mElement {
            // Java: importer.endSolidFill(3458, e.mElement); parentElementId = 3458;
            -1353661746 => { importer.endSolidFill(3458, end.mElement); parentElementId = 3458; }
            // Java: importer.endLn(e.mElement); parentElementId = -1;
            3458 => { importer.endLn(end.mElement); parentElementId = -1; }
            // Java: default: parseLnChildRefEnd(importer, e, parentElementId);
            _ => Self::parseLnChildRefEnd(importer, e, parentElementId),
        }
        parentElementId
    }

    // Java: private static void parseLnChildRefEnd(IDmlImporter importer, Element e, int parentElementId)
    pub fn parseLnChildRefEnd(importer: &mut dyn IDmlImporter, e: &ElementRef, parentElementId: i32) {
        let ElementRef::End(end) = e else { unreachable!("Java end tag"); };
        match parentElementId {
            // Java: case -1353661746: importer.endSolidFill(3458, e.mElement);
            -1353661746 => importer.endSolidFill(3458, end.mElement),
            // Java: case 3458: importer.endLn(e.mElement);
            3458 => importer.endLn(end.mElement),
            _ => {}
        }
    }

    // Java: ctxt.mLnRef = new LnRef(attrs);
    pub fn createLineRefContext(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mLnRef = Some(LnRef::new(attrs)); }

    // Java: ctxt.mClrTransform = new SysClr(attrs);
    pub fn createSysClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::SysClr(SysClr::new(attrs))); }
    // Java: ctxt.mLnRef.mSysClr = (SysClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setSysClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::SysClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java SysClr cast"); };
        ctxt.mLnRef.as_mut().expect("Java null line ref").mSysClr = Some(value);
    }
    // Java: ctxt.mClrTransform = new PrstClr(attrs);
    pub fn createPrstClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::PrstClr(PrstClr::new(attrs))); }
    // Java: ctxt.mLnRef.mPrstClr = (PrstClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setPrstClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::PrstClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java PrstClr cast"); };
        ctxt.mLnRef.as_mut().expect("Java null line ref").mPrstClr = Some(value);
    }
    // Java: ctxt.mClrTransform = new SchemeClr(attrs);
    pub fn createSchemeClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::SchemeClr(SchemeClr::new(attrs))); }
    // Java: ctxt.mLnRef.mSchemeClr = (SchemeClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setSchemeClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::SchemeClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java SchemeClr cast"); };
        ctxt.mLnRef.as_mut().expect("Java null line ref").mSchemeClr = Some(value);
    }
    // Java: ctxt.mClrTransform = new SrgbClr(attrs);
    pub fn createSrgbClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::SrgbClr(SrgbClr::new(attrs))); }
    // Java: ctxt.mLnRef.mSrgbClr = (SrgbClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setSrgbClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::SrgbClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java SrgbClr cast"); };
        ctxt.mLnRef.as_mut().expect("Java null line ref").mSrgbClr = Some(value);
    }
    // Java: ctxt.mClrTransform = new ScrgbClr(attrs);
    pub fn createScrgbClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::ScrgbClr(ScrgbClr::new(attrs))); }
    // Java: ctxt.mLnRef.mScrgbClr = (ScrgbClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setScrgbClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::ScrgbClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java ScrgbClr cast"); };
        ctxt.mLnRef.as_mut().expect("Java null line ref").mScrgbClr = Some(value);
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

