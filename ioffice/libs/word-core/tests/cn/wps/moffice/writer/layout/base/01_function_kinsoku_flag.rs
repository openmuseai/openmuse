// function_id=01_function_kinsoku_flag
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
}

impl FakeEnvoy {
    fn with_cur(prop: PropertySet) -> Self {
        let mut paraEnv = ParaEnv::ParaEnv();
        let mut cur = ParaPropEnv::default();
        cur.start = 0;
        cur.end = 10;
        cur.paraProperty = Some(prop);
        cur.hasAutoNums = false;
        paraEnv.curParaPropEnv = Some(cur);
        FakeEnvoy {
            paraEnv,
            pageEnv: PageEnv::default(),
            curPage: TypoPage::default(),
            rangeStart: 1,
        }
    }
}

impl IStateEnvoy for FakeEnvoy {
    fn getDocument(&self) -> &dyn Document {
        unimplemented!("01_function_kinsoku_flag tests pre-seed PAPX onto ParaEnv")
    }
    fn getParaEnv(&mut self) -> &mut ParaEnv {
        &mut self.paraEnv
    }
    fn addAutoNumString(&mut self, _var1: &str, _var2: &PropertySet, _var3: &ViewEnv) {}
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

fn kinsoku_after(prop: PropertySet) -> bool {
    let mut envoy = FakeEnvoy::with_cur(prop);
    let mut lo = layouter();
    lo.onStart(&mut envoy);
    envoy.paraEnv.kinsoku
}

/// K1: PAPX 201 missing → getBoolean(201, true) default.
#[test]
fn K1_kinsoku_defaults_true() {
    assert!(kinsoku_after(MutablePropertySet::new().freeze()));
}

/// K2: setBoolean(201, false) → kinsoku false.
#[test]
fn K2_kinsoku_explicit_false() {
    let mut m = MutablePropertySet::new();
    m.setBoolean(201, false);
    assert!(!kinsoku_after(m.freeze()));
}

/// K3: setBoolean(201, true) → kinsoku true.
#[test]
fn K3_kinsoku_explicit_true() {
    let mut m = MutablePropertySet::new();
    m.setBoolean(201, true);
    assert!(kinsoku_after(m.freeze()));
}

/// K4: after false, a later onStart with true is not sticky.
#[test]
fn K4_kinsoku_not_sticky_after_false() {
    let mut m_false = MutablePropertySet::new();
    m_false.setBoolean(201, false);
    let mut envoy = FakeEnvoy::with_cur(m_false.freeze());
    let mut lo = layouter();
    lo.onStart(&mut envoy);
    assert!(!envoy.paraEnv.kinsoku);

    let mut m_true = MutablePropertySet::new();
    m_true.setBoolean(201, true);
    envoy.paraEnv.curParaPropEnv.as_mut().unwrap().paraProperty = Some(m_true.freeze());
    lo.onStart(&mut envoy);
    assert!(envoy.paraEnv.kinsoku);
}
