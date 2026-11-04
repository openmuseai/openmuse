//! Compile-surface TextBody/TextParagraph/TextRun for pp-11:01 (data/text forceWrite is not wired).
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::TextListStyleHandler::TextListStyle;
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::paragraph::TextParagraphPropertiesHandler::TextParagraphProperties;
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallbackState;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

#[derive(Clone, Debug, Default)]
pub struct TextCharOptions {
    pub bold: Option<bool>,
    pub italics: Option<bool>,
    pub kumimoji: Option<bool>,
    pub normalizeH: Option<bool>,
    pub noProof: Option<bool>,
    pub dirty: Option<bool>,
    pub spellingErr: Option<bool>,
    pub smtClean: Option<bool>,
    pub rtl: Option<bool>,
}

impl TextCharOptions {
    pub fn putBold(&mut self, v: bool) { self.bold = Some(v); }
    pub fn putItalics(&mut self, v: bool) { self.italics = Some(v); }
    pub fn putKumimoji(&mut self, v: bool) { self.kumimoji = Some(v); }
    pub fn putNormalizeH(&mut self, v: bool) { self.normalizeH = Some(v); }
    pub fn putNoProof(&mut self, v: bool) { self.noProof = Some(v); }
    pub fn putDirty(&mut self, v: bool) { self.dirty = Some(v); }
    pub fn putSpellingErr(&mut self, v: bool) { self.spellingErr = Some(v); }
    pub fn putSmtClean(&mut self, v: bool) { self.smtClean = Some(v); }
    pub fn putRightToLeft(&mut self, v: bool) { self.rtl = Some(v); }
}

#[derive(Clone, Debug, Default)]
pub struct TextCharacterProperties {
    pub options: TextCharOptions,
    pub cap: Option<i32>,
    pub lang: Option<String>,
    pub altLang: Option<String>,
    pub fontSize: Option<i32>,
    pub underline: Option<i32>,
    pub strike: Option<i32>,
    pub kern: Option<i32>,
    pub spacing: Option<i32>,
    pub baseline: Option<f64>,
    pub smtId: Option<i32>,
    pub bookmark: Option<String>,
    pub underlineFollowText: bool,
    pub underlineFillFollowText: bool,
    pub latin: bool,
    pub ea: bool,
    pub cs: bool,
    pub sym: bool,
}

impl TextCharacterProperties {
    pub fn options(&mut self) -> &mut TextCharOptions { &mut self.options }
    pub fn putCap(&mut self, v: i32) { self.cap = Some(v); }
    pub fn putLang(&mut self, v: String) { self.lang = Some(v); }
    pub fn putAltLang(&mut self, v: String) { self.altLang = Some(v); }
    pub fn putFontSize(&mut self, v: i32) { self.fontSize = Some(v); }
    pub fn putUnderline(&mut self, v: i32) { self.underline = Some(v); }
    pub fn putStrike(&mut self, v: i32) { self.strike = Some(v); }
    pub fn putKern(&mut self, v: i32) { self.kern = Some(v); }
    pub fn putSpacing(&mut self, v: i32) { self.spacing = Some(v); }
    pub fn putBaseline(&mut self, v: f64) { self.baseline = Some(v); }
    pub fn putSmtId(&mut self, v: i32) { self.smtId = Some(v); }
    pub fn putBookmarkLinkTarget(&mut self, v: String) { self.bookmark = Some(v); }
    pub fn putUnderlineFollowText(&mut self, v: bool) { self.underlineFollowText = v; }
    pub fn putUnderlineFillFollowText(&mut self, v: bool) { self.underlineFillFollowText = v; }
    pub fn latinFont(&mut self) { self.latin = true; }
    pub fn eastAsianFont(&mut self) { self.ea = true; }
    pub fn complexScriptFont(&mut self) { self.cs = true; }
    pub fn symbolFont(&mut self) { self.sym = true; }
}

#[derive(Clone, Debug, Default)]
pub struct TextRun {
    pub mode: i32,
    pub length: i32,
    pub regular_text: String,
    pub field_text: String,
    pub field_id: Option<String>,
    pub field_type: Option<String>,
    pub rpr: Rc<RefCell<TextCharacterProperties>>,
    pub field_rpr: Rc<RefCell<TextCharacterProperties>>,
    pub lb_rpr: Rc<RefCell<TextCharacterProperties>>,
    pub saved: bool,
}

impl TextRun {
    pub fn create() -> Self {
        Self {
            rpr: Rc::new(RefCell::new(TextCharacterProperties::default())),
            field_rpr: Rc::new(RefCell::new(TextCharacterProperties::default())),
            lb_rpr: Rc::new(RefCell::new(TextCharacterProperties::default())),
            ..Default::default()
        }
    }
    pub fn putMode(&mut self, v: i32) { self.mode = v; }
    pub fn mode(&self) -> i32 { self.mode }
    pub fn putLength(&mut self, v: i32) { self.length = v; }
    pub fn save(&mut self) { self.saved = true; }
    pub fn regular_rpr(&self) -> Rc<RefCell<TextCharacterProperties>> { self.rpr.clone() }
    pub fn linebreak_rpr(&self) -> Rc<RefCell<TextCharacterProperties>> { self.lb_rpr.clone() }
    pub fn field_rpr(&self) -> Rc<RefCell<TextCharacterProperties>> { self.field_rpr.clone() }
}

#[derive(Clone, Debug, Default)]
pub struct TextParagraph {
    pub runs: Vec<Rc<RefCell<TextRun>>>,
    pub end_rpr: Rc<RefCell<TextCharacterProperties>>,
    pub saved: bool,
}

impl TextParagraph {
    pub fn create() -> Self {
        Self {
            end_rpr: Rc::new(RefCell::new(TextCharacterProperties::default())),
            ..Default::default()
        }
    }
    pub fn save(&mut self) { self.saved = true; }
    pub fn textruns(&mut self) -> &mut Self { self }
    pub fn write(&mut self, runs: &[Rc<RefCell<TextRun>>]) { self.runs = runs.to_vec(); }
    pub fn paraPr(&self) -> TextParagraphProperties {
        TextParagraphProperties { slot: 1114531 }
    }
    pub fn endParaRPr(&self) -> Rc<RefCell<TextCharacterProperties>> { self.end_rpr.clone() }
}

#[derive(Clone)]
pub struct TextBody {
    pub paras: Vec<Rc<RefCell<TextParagraph>>>,
    pub list_style: Rc<RefCell<TextListStyle>>,
    pub body_pr: Rc<RefCell<TextBodyProperties>>,
}

impl Default for TextBody {
    fn default() -> Self {
        Self {
            paras: Vec::new(),
            list_style: Rc::new(RefCell::new(TextListStyle::create())),
            body_pr: Rc::new(RefCell::new(TextBodyProperties::create())),
        }
    }
}

impl TextBody {
    pub fn create() -> Self { Self::default() }
    pub fn paraLst(&mut self) -> &mut Self { self }
    pub fn forceWrite(&mut self, paras: &[Rc<RefCell<TextParagraph>>]) { self.paras = paras.to_vec(); }
    pub fn listStyle(&self) -> Rc<RefCell<TextListStyle>> { self.list_style.clone() }
    pub fn bodyPr(&self) -> Rc<RefCell<TextBodyProperties>> { self.body_pr.clone() }
}

#[derive(Clone, Debug, Default)]
pub struct TextBodyProperties {
    pub rotation: Option<i32>,
    pub anchor: Option<i32>,
    pub verticalText: Option<i32>,
    pub anchorCenter: Option<bool>,
    pub horzOverflow: Option<i32>,
    pub spcFirstLastPara: Option<bool>,
    pub vertOverflow: Option<i32>,
    pub wrap: Option<i32>,
    pub leftInset: Option<i32>,
    pub topInset: Option<i32>,
    pub rightInset: Option<i32>,
    pub bottomInset: Option<i32>,
    pub numberOfColumns: Option<i32>,
    pub spaceBetweenColumns: Option<i32>,
    pub rtlCol: Option<bool>,
    pub fromWordArt: Option<bool>,
    pub forceAA: Option<bool>,
    pub upright: Option<bool>,
    pub compatLnSpc: Option<bool>,
    pub flatTx: Option<i32>,
    pub autofit_ty: Option<i32>,
    pub fontScale: Option<f64>,
    pub lnSpcReduction: Option<f64>,
}

impl TextBodyProperties {
    pub fn create() -> Self { Self::default() }
    pub fn putRotation(&mut self, v: i32) { self.rotation = Some(v); }
    pub fn putAnchor(&mut self, v: i32) { self.anchor = Some(v); }
    pub fn putVerticalText(&mut self, v: i32) { self.verticalText = Some(v); }
    pub fn putAnchorCenter(&mut self, v: bool) { self.anchorCenter = Some(v); }
    pub fn putTextHorizontalOverflow(&mut self, v: i32) { self.horzOverflow = Some(v); }
    pub fn putSpcFirstLastPara(&mut self, v: bool) { self.spcFirstLastPara = Some(v); }
    pub fn putTextVerticalOverflow(&mut self, v: i32) { self.vertOverflow = Some(v); }
    pub fn putTextWrappingType(&mut self, v: i32) { self.wrap = Some(v); }
    pub fn putLeftInset(&mut self, v: i32) { self.leftInset = Some(v); }
    pub fn putTopInset(&mut self, v: i32) { self.topInset = Some(v); }
    pub fn putRightInset(&mut self, v: i32) { self.rightInset = Some(v); }
    pub fn putBottomInset(&mut self, v: i32) { self.bottomInset = Some(v); }
    pub fn putNumberOfColumns(&mut self, v: i32) { self.numberOfColumns = Some(v); }
    pub fn putSpaceBetweenColumns(&mut self, v: i32) { self.spaceBetweenColumns = Some(v); }
    pub fn putRight2LeftColumns(&mut self, v: bool) { self.rtlCol = Some(v); }
    pub fn putFromWordArt(&mut self, v: bool) { self.fromWordArt = Some(v); }
    pub fn putForceAntiAlias(&mut self, v: bool) { self.forceAA = Some(v); }
    pub fn putTextUpright(&mut self, v: bool) { self.upright = Some(v); }
    pub fn putCompatibleLineSpacing(&mut self, v: bool) { self.compatLnSpc = Some(v); }
    pub fn putFlatTx(&mut self, v: i32) { self.flatTx = Some(v); }
    pub fn textAutofit(&mut self) -> AutofitView<'_> { AutofitView { p: self } }
}

pub struct AutofitView<'a> { p: &'a mut TextBodyProperties }
impl AutofitView<'_> {
    pub fn putType(&mut self, v: i32) { self.p.autofit_ty = Some(v); }
    pub fn putFontScale(&mut self, v: f64) { self.p.fontScale = Some(v); }
    pub fn putLnSpcReduction(&mut self, v: f64) { self.p.lnSpcReduction = Some(v); }
}

pub fn callback_state(cb: &Option<Rc<RefCell<HandlerCallback>>>) -> HandlerCallbackState {
    cb.as_ref().map(|c| c.borrow().to_state()).unwrap_or_default()
}

#[derive(Clone, Debug)]
pub struct HandlerCallback {
    pub state: HandlerCallbackState,
    pub merge_count: Rc<Cell<i32>>,
}

impl Default for HandlerCallback {
    fn default() -> Self {
        Self { state: HandlerCallbackState { mode: 0 }, merge_count: Rc::new(Cell::new(0)) }
    }
}

impl HandlerCallback {
    pub fn pptx() -> Self { Self::default() }
    pub fn gvml() -> Self {
        Self { state: HandlerCallbackState { mode: 2 }, merge_count: Rc::new(Cell::new(0)) }
    }
    pub fn isGVml(&self) -> bool { self.state.mode == 2 }
    pub fn onMergeThemeTypeface(&mut self, _run: &TextRun) {
        self.merge_count.set(self.merge_count.get() + 1);
    }
    pub fn to_state(&self) -> HandlerCallbackState { self.state }
}

pub struct FontMarkHandler { pub kind_name: &'static str }
impl ElementHandler for FontMarkHandler {
    fn kind(&self) -> &'static str { self.kind_name }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _s: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _e: i32) {}
}

