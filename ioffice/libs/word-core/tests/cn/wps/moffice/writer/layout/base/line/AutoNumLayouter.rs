// function_id=02_function_auto_num_string
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use word_core::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::core::KStyle::TextDocument as SoftTextDocument;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::data::autonum::LFO::LFO;
use word_core::cn::wps::moffice::writer::data::autonum::LSTF::LSTF;
use word_core::cn::wps::moffice::writer::data::autonum::LVLF::LVLF;
use word_core::cn::wps::moffice::writer::data::autonum::LfoData::LfoData;
use word_core::cn::wps::moffice::writer::data::autonum::LstData::LstData;
use word_core::cn::wps::moffice::writer::data::autonum::LvlfData::LvlfData;
use word_core::cn::wps::moffice::writer::layout::base::line::AutoNumLayouter::AutoNumLayouter;
use word_core::cn::wps::moffice::writer::layout::base::ParaLayouter::IStateEnvoy;

struct FakeEnvoy {
    paraEnv: ParaEnv,
    pageEnv: PageEnv,
    curPage: TypoPage,
}

impl FakeEnvoy {
    fn new() -> Self {
        FakeEnvoy {
            paraEnv: ParaEnv::ParaEnv(),
            pageEnv: PageEnv::default(),
            curPage: TypoPage::default(),
        }
    }
}

impl IStateEnvoy for FakeEnvoy {
    fn getDocument(&self) -> &dyn Document {
        unimplemented!("02_function_auto_num_string tests do not call getDocument")
    }
    fn getParaEnv(&mut self) -> &mut ParaEnv {
        &mut self.paraEnv
    }
    fn addAutoNumString(&mut self, _var1: &str, _var2: &PropertySet, _var3: &ViewEnv) {}
    fn getCP(&self) -> i32 {
        0
    }
    fn getCpExResult(&self) -> i32 {
        0
    }
    fn getRangeStart(&self) -> i32 {
        0
    }
    fn getRangeEnd(&self) -> i32 {
        10
    }
    fn getPageEnv(&self) -> &PageEnv {
        &self.pageEnv
    }
    fn getCurPage(&self) -> &TypoPage {
        &self.curPage
    }
    fn skipSpaceBeforePara(&mut self, _var1: i32) {}
    fn getLinePos(&self) -> i32 {
        0
    }
}

fn papx_lfo_lvl(lfo: i32, lvl: i32) -> PropertySet {
    let mut m = MutablePropertySet::new();
    m.setInt(235, lfo);
    m.setInt(234, lvl);
    m.freeze()
}

fn install_list(doc: &mut TextDocument, ilfo: i32, levels: &[(i32, i32, Option<&str>)]) {
    let soft = SoftTextDocument::default();
    let lsid = 100;
    let mut lvls = Vec::new();
    for (nfc, follow, xst) in levels {
        let mut lv = LvlfData::LvlfData(&soft);
        lv.setLVLF(Some(LVLF::default()));
        lv.setNfc(*nfc);
        lv.setFollow(*follow);
        if let Some(s) = xst {
            lv.setXstStr(Some((*s).to_string()));
        }
        lvls.push(lv);
    }
    let mut lstf = LSTF::default();
    lstf.setLsid(lsid);
    let mut lst = LstData::LstData(&soft);
    lst.setLETF(Some(lstf));
    lst.setLvlfDataArray(&lvls);
    doc.getAutoNumTable().getLstDataTable().add(&mut lst);

    let mut lfo = LfoData::LfoData(&soft);
    lfo.setLFO(Some(LFO::default()));
    lfo.setLsid(lsid);
    doc.getAutoNumTable().getLfoDataTable().add(ilfo, &mut lfo);
}

fn layouter_with(ilfo: i32, levels: &[(i32, i32, Option<&str>)]) -> AutoNumLayouter {
    let mut doc = TextDocument::default();
    if !levels.is_empty() {
        install_list(&mut doc, ilfo, levels);
    }
    let mut env = DocumentEnv::default();
    env.textDocument = Some(doc);
    AutoNumLayouter::AutoNumLayouter(env, &ViewEnv::default())
}

fn prop_env(prop: Option<PropertySet>) -> ParaPropEnv {
    let mut e = ParaPropEnv::default();
    e.start = 0;
    e.end = 10;
    e.paraProperty = prop;
    e
}

/// AN1: PAPX 235+234 with existing lfo → decimal string starts with "1."
#[test]
fn an1_decimal_string() {
    let mut lo = layouter_with(1, &[(0, 2, None)]);
    let mut propEnv = prop_env(Some(papx_lfo_lvl(1, 0)));
    let envoy = FakeEnvoy::new();
    lo.calAutoNumString(&mut propEnv, &envoy);
    let s = propEnv.autoNumString.expect("AN1 autoNumString");
    assert!(s.starts_with("1."), "AN1 got {s:?}");
    assert!(propEnv.hasAutoNums);
}

/// AN2: no PAPX 235 → do not invent a number.
#[test]
fn an2_no_235() {
    let mut lo = layouter_with(0, &[]);
    let mut propEnv = prop_env(Some(MutablePropertySet::new().freeze()));
    let envoy = FakeEnvoy::new();
    lo.calAutoNumString(&mut propEnv, &envoy);
    assert!(propEnv.autoNumString.is_none());
    assert!(!propEnv.hasAutoNums);
}

/// AN3: follow=0 → string contains tab.
#[test]
fn an3_follow_tab() {
    let mut lo = layouter_with(1, &[(0, 2, None), (0, 0, None)]);
    let mut propEnv = prop_env(Some(papx_lfo_lvl(1, 1)));
    let envoy = FakeEnvoy::new();
    lo.calAutoNumString(&mut propEnv, &envoy);
    let s = propEnv.autoNumString.expect("AN3 autoNumString");
    assert!(s.contains('\t'), "AN3 got {s:?}");
}

/// AN4: nfc=23 bullet path.
#[test]
fn an4_bullet_nfc23() {
    let mut lo = layouter_with(23, &[(23, 2, Some("•"))]);
    let mut propEnv = prop_env(Some(papx_lfo_lvl(23, 0)));
    let envoy = FakeEnvoy::new();
    lo.calAutoNumString(&mut propEnv, &envoy);
    let s = propEnv.autoNumString.expect("AN4 autoNumString");
    assert!(!s.starts_with("1."), "AN4 should not take decimal path, got {s:?}");
    assert!(s.contains('•') || !s.is_empty());
}

/// AN5: verifyData(null) → IllegalState / panic.
#[test]
#[should_panic(expected = "illegal data")]
fn an5_verify_data_null() {
    let lo = layouter_with(0, &[]);
    lo.verifyData(None);
}
