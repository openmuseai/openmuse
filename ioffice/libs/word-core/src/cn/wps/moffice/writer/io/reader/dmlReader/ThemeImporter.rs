// Java: cn.wps.moffice.writer.io.reader.dmlReader.ThemeImporter
#![allow(non_snake_case)]

use std::{cell::RefCell, ops::DerefMut, rc::Rc};
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use super::context::{
    Branch::{Branch, ElementRef},
    ColorChoice::ColorChoice,
    ColorTransform::ColorTransform,
    EndTag::EndTag,
    PrstClr::PrstClr,
    SchemeClr::SchemeClr,
    ScrgbClr::ScrgbClr,
    SrgbClr::SrgbClr,
    StartTag::StartTag,
    SysClr::SysClr,
    ThemeContext::ThemeContext,
};
use super::helper::{ColorsTrans::ColorsTrans, MetricHelper::MetricHelper};

#[derive(Clone, Copy)]
enum CurKind { Srgb, Sys, Scrgb, Prst, Scheme }

// Java: public class ThemeImporter
pub struct ThemeImporter {
    // Java: private ThemeContext mThemeCtxt;
    mThemeCtxt: Rc<RefCell<ThemeContext>>,
    mSizeFillStyleLst: i32,
    mSizeLnStyleLst: i32,
    mSizeEffectStyleLst: i32,
    mSizeBgFillStyleLst: i32,
    mCurBranch: Option<Branch>,
    mCurClrChoice: Option<ColorChoice>,
    mCurKind: Option<CurKind>,
}

impl ThemeImporter {
    // Java: public ThemeImporter(ThemeContext themeCtxt)
    pub fn new(themeCtxt: Rc<RefCell<ThemeContext>>) -> Self {
        Self {
            mThemeCtxt: themeCtxt,
            mSizeFillStyleLst: 0,
            mSizeLnStyleLst: 0,
            mSizeEffectStyleLst: 0,
            mSizeBgFillStyleLst: 0,
            mCurBranch: None,
            mCurClrChoice: None,
            mCurKind: None,
        }
    }

    fn start_tag(elementId: i32, attrs: &dyn SaxAttributes) -> ElementRef {
        // Java: new StartTag(elementId, attrs) copies AttributesImpl.
        ElementRef::Start(StartTag::StartTag(elementId, attrs.as_attributes_impl()))
    }

    fn push_start(&mut self, elementId: i32, attrs: &dyn SaxAttributes, fresh: bool) {
        if fresh {
            // Java: this.mCurBranch = new Branch();
            self.mCurBranch = Some(Branch::new());
        }
        if let Some(branch) = self.mCurBranch.as_mut() {
            branch.mElements.push(Self::start_tag(elementId, attrs));
        }
    }

    fn push_end(&mut self, elementId: i32) {
        if let Some(branch) = self.mCurBranch.as_mut() {
            branch.mElements.push(ElementRef::End(EndTag::EndTag(elementId)));
        }
    }

    // Java: public void startBgFillStyle(int elementId, Attributes attrs)
    pub fn startBgFillStyle(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        if self.mSizeBgFillStyleLst < 3 {
            match elementId {
                -1353661746 | 88612081 => self.push_start(elementId, attrs, true),
                _ => self.push_start(elementId, attrs, false),
            }
        }
    }

    // Java: public void endBgFillStyle(int elementId)
    pub fn endBgFillStyle(&mut self, elementId: i32) {
        if self.mSizeBgFillStyleLst < 3 {
            match elementId {
                -1353661746 | 88612081 => {
                    self.push_end(elementId);
                    let branch = self.mCurBranch.take();
                    self.mThemeCtxt.borrow_mut().mBgFillStyleLst[self.mSizeBgFillStyleLst as usize] = branch;
                    self.mSizeBgFillStyleLst += 1;
                }
                _ => self.push_end(elementId),
            }
        }
    }

    // Java: public void startFillStyle(int elementId, Attributes attrs)
    pub fn startFillStyle(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        if self.mSizeFillStyleLst < 3 {
            match elementId {
                -1353661746 | 88612081 => self.push_start(elementId, attrs, true),
                _ => self.push_start(elementId, attrs, false),
            }
        }
    }

    // Java: public void endFillStyle(int elementId)
    pub fn endFillStyle(&mut self, elementId: i32) {
        if self.mSizeFillStyleLst < 3 {
            match elementId {
                -1353661746 | 88612081 => {
                    self.push_end(elementId);
                    let branch = self.mCurBranch.take();
                    self.mThemeCtxt.borrow_mut().mFillStyleLst[self.mSizeFillStyleLst as usize] = branch;
                    self.mSizeFillStyleLst += 1;
                }
                _ => self.push_end(elementId),
            }
        }
    }

    // Java: public void startLnStyle(int elementId, Attributes attrs)
    pub fn startLnStyle(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        if self.mSizeLnStyleLst < 3 {
            match elementId {
                3458 => self.push_start(elementId, attrs, true),
                _ => self.push_start(elementId, attrs, false),
            }
        }
    }

    // Java: public void endLnStyle(int elementId)
    pub fn endLnStyle(&mut self, elementId: i32) {
        if self.mSizeLnStyleLst < 3 {
            match elementId {
                3458 => {
                    self.push_end(elementId);
                    let branch = self.mCurBranch.take();
                    self.mThemeCtxt.borrow_mut().mLnStyleLst[self.mSizeLnStyleLst as usize] = branch;
                    self.mSizeLnStyleLst += 1;
                }
                _ => self.push_end(elementId),
            }
        }
    }

    // Java: public void startEffectStyle(int elementId, Attributes attrs)
    pub fn startEffectStyle(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        if self.mSizeEffectStyleLst < 3 {
            match elementId {
                1650181792 => self.push_start(elementId, attrs, true),
                _ => self.push_start(elementId, attrs, false),
            }
        }
    }

    // Java: public void endEffectStyle(int elementId)
    pub fn endEffectStyle(&mut self, elementId: i32) {
        if self.mSizeEffectStyleLst < 3 {
            match elementId {
                1650181792 => {
                    self.push_end(elementId);
                    let branch = self.mCurBranch.take();
                    self.mThemeCtxt.borrow_mut().mEffectStyleLst[self.mSizeEffectStyleLst as usize] = branch;
                    self.mSizeEffectStyleLst += 1;
                }
                _ => self.push_end(elementId),
            }
        }
    }

    fn transform_mut(&mut self) -> Option<&mut ColorTransform> {
        let kind = self.mCurKind?;
        let choice = self.mCurClrChoice.as_mut()?;
        Some(match kind {
            CurKind::Srgb => choice.mSrgbClr.as_mut()?.deref_mut(),
            CurKind::Sys => choice.mSysClr.as_mut()?.deref_mut(),
            CurKind::Scrgb => choice.mScrgbClr.as_mut()?.deref_mut(),
            CurKind::Prst => choice.mPrstClr.as_mut()?.deref_mut(),
            CurKind::Scheme => choice.mSchemeClr.as_mut()?.deref_mut(),
        })
    }

    fn set_percent(&mut self, attrs: &dyn SaxAttributes, write: fn(&mut ColorTransform, Option<f32>)) {
        let value = MetricHelper::percent2Float(attrs, "val");
        if let Some(transform) = self.transform_mut() {
            write(transform, value);
        }
    }

    // Java: public void startThemeClr(int elementId, Attributes attributes)
    pub fn startThemeClr(&mut self, elementId: i32, attributes: &dyn SaxAttributes) {
        match elementId {
            -1949425841 => {
                let choice = self.mCurClrChoice.get_or_insert_with(ColorChoice::new);
                choice.mSrgbClr = Some(SrgbClr::new(attributes));
                self.mCurKind = Some(CurKind::Srgb);
            }
            -1177623385 | -1177623384 | -1177623383 | -1177623382 | -1177623381 | -1177623380
            | 99466 | 99467 | 107433 | 107434 | 99368034 | 268452191 => {
                self.mCurClrChoice = Some(ColorChoice::new());
                self.mCurKind = None;
            }
            -1091642274 => self.set_percent(attributes, |t, v| t.mLumMod = v),
            -1091640629 => self.set_percent(attributes, |t, v| t.mLumOff = v),
            -934911823 => self.set_percent(attributes, |t, v| t.mRedMod = v),
            -934910178 => self.set_percent(attributes, |t, v| t.mRedOff = v),
            -909500100 => self.set_percent(attributes, |t, v| t.mSatMod = v),
            -909498455 => self.set_percent(attributes, |t, v| t.mSatOff = v),
            -887375076 => {
                let choice = self.mCurClrChoice.get_or_insert_with(ColorChoice::new);
                choice.mSysClr = Some(SysClr::new(attributes));
                self.mCurKind = Some(CurKind::Sys);
            }
            -409889940 => {
                let choice = self.mCurClrChoice.get_or_insert_with(ColorChoice::new);
                choice.mScrgbClr = Some(ScrgbClr::new(attributes));
                self.mCurKind = Some(CurKind::Scrgb);
            }
            -305351098 => {
                let choice = self.mCurClrChoice.get_or_insert_with(ColorChoice::new);
                choice.mPrstClr = Some(PrstClr::new(attributes));
                self.mCurKind = Some(CurKind::Prst);
            }
            -152914620 => {
                let choice = self.mCurClrChoice.get_or_insert_with(ColorChoice::new);
                choice.mSchemeClr = Some(SchemeClr::new(attributes));
                self.mCurKind = Some(CurKind::Scheme);
            }
            -15865784 => self.set_percent(attributes, |t, v| t.mBlueMod = v),
            -15864139 => self.set_percent(attributes, |t, v| t.mBlueOff = v),
            107524 => self.set_percent(attributes, |t, v| t.mLum = v),
            112785 => self.set_percent(attributes, |t, v| t.mRed = v),
            113638 => self.set_percent(attributes, |t, v| t.mSat = v),
            3027034 => self.set_percent(attributes, |t, v| t.mBlue = v),
            92909918 => self.set_percent(attributes, |t, v| t.mAlpha = v),
            98619139 => self.set_percent(attributes, |t, v| t.mGreen = v),
            109399597 => self.set_percent(attributes, |t, v| t.mShade = v),
            205217023 => self.set_percent(attributes, |t, v| t.mGreenMod = v),
            205218668 => self.set_percent(attributes, |t, v| t.mGreenOff = v),
            _ => {}
        }
    }

    fn store_scheme(&mut self, elementId: i32, also: Option<i32>) {
        let choice = self.mCurClrChoice.take();
        self.mCurKind = None;
        let Some(choice) = choice else { return; };
        // Java: ColorsTrans.toColor4Theme(this.mCurClrChoice)
        let clr = ColorsTrans::toColor4Theme(&choice);
        let mut theme = self.mThemeCtxt.borrow_mut();
        if let Some(clr) = clr {
            theme.mClrScheme.insert(elementId, clr);
            if let Some(alias) = also {
                theme.mClrScheme.insert(alias, clr);
            }
        }
    }

    // Java: public void endThemeClr(int elementId)
    pub fn endThemeClr(&mut self, elementId: i32) {
        match elementId {
            -1177623385 | -1177623384 | -1177623383 | -1177623382 | -1177623381 | -1177623380
            | 99368034 | 268452191 => self.store_scheme(elementId, None),
            99466 => self.store_scheme(elementId, Some(115245)),
            99467 => self.store_scheme(elementId, Some(115246)),
            107433 => self.store_scheme(elementId, Some(97420)),
            107434 => self.store_scheme(elementId, Some(97421)),
            _ => {}
        }
    }

    fn map_scheme(&mut self, name: Option<String>, dest: i32) {
        let Some(name) = name else { return; };
        // Java: mClrScheme.get(bg1Map.hashCode())
        let key = java_hash(&name);
        let clr = self.mThemeCtxt.borrow().mClrScheme.get(&key).copied();
        if let Some(clr) = clr {
            self.mThemeCtxt.borrow_mut().mClrScheme.insert(dest, clr);
        }
    }

    // Java: public void startClrMap(Attributes attributes)
    pub fn startClrMap(&mut self, attributes: &dyn SaxAttributes) {
        self.map_scheme(attributes.getValue_by_qname("bg1").or_else(|| attributes.getValue_by_uri("", "bg1")), 97420);
        self.map_scheme(attributes.getValue_by_qname("tx1").or_else(|| attributes.getValue_by_uri("", "tx1")), 115245);
        self.map_scheme(attributes.getValue_by_qname("bg2").or_else(|| attributes.getValue_by_uri("", "bg2")), 97421);
        self.map_scheme(attributes.getValue_by_qname("tx2").or_else(|| attributes.getValue_by_uri("", "tx2")), 115246);
    }
}

fn java_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, ch| hash.wrapping_mul(31).wrapping_add(ch as i32))
}
