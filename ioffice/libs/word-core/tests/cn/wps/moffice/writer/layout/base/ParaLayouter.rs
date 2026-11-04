// function_id=01_function_para_spacing
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use word_core::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::LineBox::{LineBox, LineSpacing as BoxLineSpacing};
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::ParaLayouter::{IStateEnvoy, ParaLayouter};

fn ps_bool(id: i32, v: bool) -> PropertySet {
    let mut m = MutablePropertySet::new();
    m.setBoolean(id, v);
    m.freeze()
}

fn ps_int(id: i32, v: i32) -> PropertySet {
    let mut m = MutablePropertySet::new();
    m.setInt(id, v);
    m.freeze()
}

struct FakeEnvoy {
    paraEnv: ParaEnv,
    pageEnv: PageEnv,
    curPage: TypoPage,
    rangeStart: i32,
    rangeEnd: i32,
    linePos: i32,
}

impl FakeEnvoy {
    fn with_cur(prop: PropertySet, start: i32) -> Self {
        let mut paraEnv = ParaEnv::ParaEnv();
        let mut cur = ParaPropEnv::default();
        cur.start = start;
        cur.end = start + 10;
        cur.paraProperty = Some(prop);
        paraEnv.curParaPropEnv = Some(cur);
        FakeEnvoy {
            paraEnv,
            pageEnv: {
                let mut p = PageEnv::default();
                p.gridModel = 0;
                p.gridHeight = 0;
                p
            },
            curPage: TypoPage::default(),
            rangeStart: start + 1,
            rangeEnd: start + 10,
            linePos: 0,
        }
    }
}

impl IStateEnvoy for FakeEnvoy {
    fn getDocument(&self) -> &dyn Document {
        unimplemented!("01_function_para_spacing tests do not call getDocument")
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
        self.rangeStart
    }
    fn getRangeEnd(&self) -> i32 {
        self.rangeEnd
    }
    fn getPageEnv(&self) -> &PageEnv {
        &self.pageEnv
    }
    fn getCurPage(&self) -> &TypoPage {
        &self.curPage
    }
    fn skipSpaceBeforePara(&mut self, _var1: i32) {}
    fn getLinePos(&self) -> i32 {
        self.linePos
    }
}

fn layouter() -> ParaLayouter {
    ParaLayouter::ParaLayouter(DocumentEnv::default(), ViewEnv::default(), MainState::default())
}

#[test]
fn PB1_auto_spacing_mid_range() {
    let mut envoy = FakeEnvoy::with_cur(ps_bool(219, true), 0);
    envoy.rangeStart = 5;
    let mut lo = layouter();
    lo.calcParaBefore(&mut envoy);
    assert_eq!(envoy.paraEnv.curParaPropEnv.as_ref().unwrap().paraBefore, 280);
}

#[test]
fn PB2_auto_spacing_at_para_start() {
    let mut envoy = FakeEnvoy::with_cur(ps_bool(219, true), 10);
    envoy.rangeStart = 10;
    let mut lo = layouter();
    lo.calcParaBefore(&mut envoy);
    assert_eq!(envoy.paraEnv.curParaPropEnv.as_ref().unwrap().paraBefore, 0);
}

#[test]
fn PB3_before_space_217() {
    let mut envoy = FakeEnvoy::with_cur(ps_int(217, 240), 0);
    let mut lo = layouter();
    lo.calcParaBefore(&mut envoy);
    assert_eq!(envoy.paraEnv.curParaPropEnv.as_ref().unwrap().paraBefore, 240);
}

#[test]
fn PB4_before_lines_218_no_grid() {
    let mut envoy = FakeEnvoy::with_cur(ps_int(218, 100), 0);
    envoy.pageEnv.gridModel = 0;
    envoy.pageEnv.gridHeight = 0;
    let mut lo = layouter();
    lo.calcParaBefore(&mut envoy);
    assert_eq!(envoy.paraEnv.curParaPropEnv.as_ref().unwrap().paraBefore, 312);
}

#[test]
fn PA1_auto_after_no_next() {
    let mut envoy = FakeEnvoy::with_cur(ps_bool(216, true), 0);
    let mut lo = layouter();
    lo.calcParaAfter(&mut envoy);
    assert_eq!(envoy.paraEnv.curParaPropEnv.as_ref().unwrap().paraAfter, 280);
}

#[test]
fn PA2_after_space_214() {
    let mut envoy = FakeEnvoy::with_cur(ps_int(214, 200), 0);
    let mut lo = layouter();
    lo.calcParaAfter(&mut envoy);
    assert_eq!(envoy.paraEnv.curParaPropEnv.as_ref().unwrap().paraAfter, 200);
}

#[test]
fn CL1_collapse_first_line() {
    let box_ = LineBox::measure(100, 20, BoxLineSpacing::auto(1.0), 400, 0, true, false, 200);
    // channel 120 + extra 0 (auto 1.0) + collapsed before max(0,400-200)=200
    assert_eq!(box_.height, 120 + 200);
}
