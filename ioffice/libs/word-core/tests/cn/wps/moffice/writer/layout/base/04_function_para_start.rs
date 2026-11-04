// function_id=04_function_para_start
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use word_core::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::ParaLayouter::{IStateEnvoy, ParaLayouter};

struct FakeEnvoy {
    paraEnv: ParaEnv,
    pageEnv: PageEnv,
    curPage: TypoPage,
    rangeStart: i32,
    auto_num_calls: i32,
}

impl FakeEnvoy {
    fn with_cur(prop: PropertySet, start: i32) -> Self {
        let mut paraEnv = ParaEnv::ParaEnv();
        let mut cur = ParaPropEnv::default();
        cur.start = start;
        cur.end = start + 10;
        cur.paraProperty = Some(prop);
        cur.hasAutoNums = false;
        paraEnv.curParaPropEnv = Some(cur);
        FakeEnvoy {
            paraEnv,
            pageEnv: PageEnv::default(),
            curPage: TypoPage::default(),
            rangeStart: start + 1,
            auto_num_calls: 0,
        }
    }
}

impl IStateEnvoy for FakeEnvoy {
    fn getDocument(&self) -> &dyn Document {
        unimplemented!("04_function_para_start tests pre-seed PAPX onto ParaEnv")
    }
    fn getParaEnv(&mut self) -> &mut ParaEnv {
        &mut self.paraEnv
    }
    fn addAutoNumString(&mut self, _var1: &str, _var2: &PropertySet, _var3: &ViewEnv) {
        self.auto_num_calls += 1;
    }
    fn getCP(&self) -> i32 {
        self.paraEnv.curParaPropEnv.as_ref().map(|c| c.start).unwrap_or(0)
    }
    fn getCpExResult(&self) -> i32 {
        0
    }
    fn getRangeStart(&self) -> i32 {
        self.rangeStart
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

fn layouter() -> ParaLayouter {
    ParaLayouter::ParaLayouter(DocumentEnv::default(), ViewEnv::default(), MainState::default())
}

#[test]
fn PS1_alignment_193() {
    let mut m = MutablePropertySet::new();
    m.setInt(193, 1);
    let mut envoy = FakeEnvoy::with_cur(m.freeze(), 0);
    let mut lo = layouter();
    lo.onStart(&mut envoy);
    assert_eq!(envoy.paraEnv.alignment, 1);
}

#[test]
fn PS2_line_spacing_after_before() {
    let mut m = MutablePropertySet::new();
    m.setInt(217, 240);
    m.setInt(214, 120);
    let mut envoy = FakeEnvoy::with_cur(m.freeze(), 0);
    envoy.rangeStart = 5;
    let mut lo = layouter();
    lo.onStart(&mut envoy);
    assert_eq!(envoy.paraEnv.curParaPropEnv.as_ref().unwrap().paraBefore, 240);
    assert_eq!(envoy.paraEnv.curParaPropEnv.as_ref().unwrap().paraAfter, 120);
    assert_eq!(envoy.paraEnv.lineSpacing.getRule(), 0);
}

#[test]
fn PS3_rtl_251() {
    let mut m = MutablePropertySet::new();
    m.setBoolean(251, true);
    let mut envoy = FakeEnvoy::with_cur(m.freeze(), 0);
    let mut lo = layouter();
    lo.onStart(&mut envoy);
    assert!(envoy.paraEnv.isRTL);
}

#[test]
fn PS4_no_autonum_string() {
    let mut envoy = FakeEnvoy::with_cur(MutablePropertySet::new().freeze(), 0);
    envoy.paraEnv.curParaPropEnv.as_mut().unwrap().hasAutoNums = false;
    let mut lo = layouter();
    lo.onStart(&mut envoy);
    assert_eq!(envoy.auto_num_calls, 0);
}
