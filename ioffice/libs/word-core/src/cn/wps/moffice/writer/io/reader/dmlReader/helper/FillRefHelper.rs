#![allow(non_snake_case)]

use io_wps_io::cn::wps::io::xwpf::{IDmlImporter::IDmlImporter, SaxAttributes::SaxAttributes};
use crate::cn::wps::moffice::writer::io::reader::dmlReader::{
    context::{Branch::{Branch, ElementRef}, ColorTransform::ColorTransformValue,
        DmlShapeCtxt::DmlShapeCtxt, FillRef::FillRef, PrstClr::PrstClr,
        SchemeClr::SchemeClr, ScrgbClr::ScrgbClr, SrgbClr::SrgbClr,
        SysClr::SysClr, ThemeContext::ThemeContext},
    helper::{ColorsTrans::ColorsTrans, MetricHelper::MetricHelper},
};

// Java: public class FillRefHelper
pub struct FillRefHelper;
impl FillRefHelper {
    // Java: if (ctxt.mFillOn) { if (null != ctxt.mFillRef) parseFillRef(...); ctxt.mFillRef = null; }
    pub fn importFillByFillRef(importer: &mut dyn IDmlImporter, ctxt: &mut DmlShapeCtxt, themeCtxt: &ThemeContext) {
        if ctxt.mFillOn {
            if ctxt.mFillRef.is_some() { Self::parseFillRef(importer, ctxt, themeCtxt); }
            ctxt.mFillRef = None;
        }
    }

    // Java: if (null != ctxt.mFillRef.mIdx && 0 != ... && 1000 != ... && null != themeCtxt)
    pub fn parseFillRef(importer: &mut dyn IDmlImporter, ctxt: &mut DmlShapeCtxt, themeCtxt: &ThemeContext) {
        let Some(fill_ref) = ctxt.mFillRef.as_ref() else { return; };
        let Some(idx) = fill_ref.mIdx else { return; };
        if idx == 0 || idx == 1000 { return; }
        // Java: 1..3 uses mFillStyleLst; 1001..1003 uses mBgFillStyleLst.
        let style = match idx {
            1..=3 => &themeCtxt.mFillStyleLst[(idx - 1) as usize],
            1001..=1003 => &themeCtxt.mBgFillStyleLst[(idx - 1001) as usize],
            _ => return,
        };
        let Some(style) = style else { return; };
        // Java: ctxt.mPhClr = ColorsTrans.toColor((Integer)null, (ColorChoice)ctxt.mFillRef, themeCtxt);
        ctxt.mPhClr = ColorsTrans::toColor(None, fill_ref, Some(themeCtxt));
        // Java: switch (((Element)style.mElements.get(0)).mElement)
        match style.mElements.first().map(Self::elementId) {
            Some(-1353661746) => Self::parseSolidFillRef(importer, style),
            Some(88612081) => Self::parseGradFillRef(importer, style),
            _ => {}
        }
    }

    fn elementId(e: &ElementRef) -> i32 {
        match e { ElementRef::Start(v) => v.mElement, ElementRef::End(v) => v.mElement }
    }

    // Java: private static void parseSolidFillRef(IDmlImporter importer, Branch style)
    pub fn parseSolidFillRef(importer: &mut dyn IDmlImporter, style: &Branch) {
        // Java: for (...) { if (0 == pop.type()) startSolidFill(962562815, ...); else endSolidFill(...); }
        for e in &style.mElements {
            match e {
                ElementRef::Start(v) => importer.startSolidFill(962562815, v.mElement, &v.mAttrs),
                ElementRef::End(v) => importer.endSolidFill(962562815, v.mElement),
            }
        }
    }

    // Java: private static void parseGradFillRef(IDmlImporter importer, Branch style)
    pub fn parseGradFillRef(importer: &mut dyn IDmlImporter, style: &Branch) {
        let mut parentElementId = -1;
        // Java: for (...) { if (0 == pop.type()) parseGradFillRefStart(...); else parseGradFillRefEnd(...); }
        for e in &style.mElements {
            parentElementId = match e {
                ElementRef::Start(_) => Self::parseGradFillRefStart(importer, e, parentElementId),
                ElementRef::End(_) => Self::parseGradFillRefEnd(importer, e, parentElementId),
            };
        }
    }

    // Java: private static int parseGradFillRefStart(IDmlImporter importer, Element e, int parentElementId)
    pub fn parseGradFillRefStart(importer: &mut dyn IDmlImporter, e: &ElementRef, mut parentElementId: i32) -> i32 {
        let ElementRef::Start(v) = e else { unreachable!("Java start tag"); };
        match v.mElement {
            // Java: case 3308: importer.startGs(3536191, e.mElement, e.mAttrs); parentElementId = 3308;
            3308 => { importer.startGs(3536191, v.mElement, &v.mAttrs); parentElementId = 3308; }
            // Java: case 107153: importer.startLin(3536191, e.mElement, e.mAttrs);
            107153 => importer.startLin(3536191, v.mElement, &v.mAttrs),
            // Java: case 3433509: importer.startPath(3536191, e.mElement, e.mAttrs);
            3433509 => importer.startPath(3536191, v.mElement, &v.mAttrs),
            // Java: case 88612081: importer.startGradFill(3536191, e.mElement, e.mAttrs);
            88612081 => importer.startGradFill(3536191, v.mElement, &v.mAttrs),
            // Java: default: parseGradFillChildStart(importer, e, parentElementId);
            _ => Self::parseGradFillChildStart(importer, e, parentElementId),
        }
        parentElementId
    }

    // Java: private static void parseGradFillChildStart(IDmlImporter importer, Element e, int parentElementId)
    pub fn parseGradFillChildStart(importer: &mut dyn IDmlImporter, e: &ElementRef, parentElementId: i32) {
        let ElementRef::Start(v) = e else { unreachable!("Java start tag"); };
        // Java: case 3308: importer.startGs(3536191, e.mElement, e.mAttrs);
        if parentElementId == 3308 { importer.startGs(3536191, v.mElement, &v.mAttrs); }
    }

    // Java: private static int parseGradFillRefEnd(IDmlImporter importer, Element e, int parentElementId)
    pub fn parseGradFillRefEnd(importer: &mut dyn IDmlImporter, e: &ElementRef, mut parentElementId: i32) -> i32 {
        let ElementRef::End(v) = e else { unreachable!("Java end tag"); };
        match v.mElement {
            // Java: case 3308: importer.endGs(3536191, e.mElement); parentElementId = -1;
            3308 => { importer.endGs(3536191, v.mElement); parentElementId = -1; }
            // Java: case 107153: importer.endLin(3536191);
            107153 => importer.endLin(3536191),
            // Java: case 3433509: importer.endPath(3536191);
            3433509 => importer.endPath(3536191),
            // Java: case 88612081: importer.endGradFill(3536191);
            88612081 => importer.endGradFill(3536191),
            // Java: default: parseGradFillChildEnd(importer, e, parentElementId);
            _ => Self::parseGradFillChildEnd(importer, e, parentElementId),
        }
        parentElementId
    }

    // Java: private static void parseGradFillChildEnd(IDmlImporter importer, Element e, int parentElementId)
    pub fn parseGradFillChildEnd(importer: &mut dyn IDmlImporter, e: &ElementRef, parentElementId: i32) {
        let ElementRef::End(v) = e else { unreachable!("Java end tag"); };
        // Java: case 3308: importer.endGs(3536191, e.mElement);
        if parentElementId == 3308 { importer.endGs(3536191, v.mElement); }
    }

    // Java: ctxt.mFillRef = new FillRef(attrs);
    pub fn createFillRefContext(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mFillRef = Some(FillRef::new(attrs)); }

    // Java: ctxt.mClrTransform = new SysClr(attrs);
    pub fn createSysClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::SysClr(SysClr::new(attrs))); }
    // Java: ctxt.mFillRef.mSysClr = (SysClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setSysClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::SysClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java SysClr cast"); };
        ctxt.mFillRef.as_mut().expect("Java null fill ref").mSysClr = Some(value);
    }
    // Java: ctxt.mClrTransform = new PrstClr(attrs);
    pub fn createPrstClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::PrstClr(PrstClr::new(attrs))); }
    // Java: ctxt.mFillRef.mPrstClr = (PrstClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setPrstClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::PrstClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java PrstClr cast"); };
        ctxt.mFillRef.as_mut().expect("Java null fill ref").mPrstClr = Some(value);
    }
    // Java: ctxt.mClrTransform = new SchemeClr(attrs);
    pub fn createSchemeClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::SchemeClr(SchemeClr::new(attrs))); }
    // Java: ctxt.mFillRef.mSchemeClr = (SchemeClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setSchemeClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::SchemeClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java SchemeClr cast"); };
        ctxt.mFillRef.as_mut().expect("Java null fill ref").mSchemeClr = Some(value);
    }
    // Java: ctxt.mClrTransform = new SrgbClr(attrs);
    pub fn createSrgbClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::SrgbClr(SrgbClr::new(attrs))); }
    // Java: ctxt.mFillRef.mSrgbClr = (SrgbClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setSrgbClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::SrgbClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java SrgbClr cast"); };
        ctxt.mFillRef.as_mut().expect("Java null fill ref").mSrgbClr = Some(value);
    }
    // Java: ctxt.mClrTransform = new ScrgbClr(attrs);
    pub fn createScrgbClr(ctxt: &mut DmlShapeCtxt, attrs: &dyn SaxAttributes) { ctxt.mClrTransform = Some(ColorTransformValue::ScrgbClr(ScrgbClr::new(attrs))); }
    // Java: ctxt.mFillRef.mScrgbClr = (ScrgbClr)ctxt.mClrTransform; ctxt.mClrTransform = null;
    pub fn setScrgbClr(ctxt: &mut DmlShapeCtxt) {
        let ColorTransformValue::ScrgbClr(value) = ctxt.mClrTransform.take().expect("Java null color transform") else { panic!("Java ScrgbClr cast"); };
        ctxt.mFillRef.as_mut().expect("Java null fill ref").mScrgbClr = Some(value);
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

