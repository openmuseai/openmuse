// function_id=02_function_line_spacing
#![allow(non_snake_case)]

use std::sync::Arc;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use word_core::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::data::value::LineSpacing::LineSpacing;
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
}

impl FakeEnvoy {
    fn with_cur(prop: PropertySet) -> Self {
        let mut paraEnv = ParaEnv::ParaEnv();
        let mut cur = ParaPropEnv::default();
        cur.start = 0;
        cur.end = 10;
        cur.paraProperty = Some(prop);
        paraEnv.curParaPropEnv = Some(cur);
        FakeEnvoy {
            paraEnv,
            pageEnv: PageEnv::default(),
            curPage: TypoPage::default(),
        }
    }
}

impl IStateEnvoy for FakeEnvoy {
    fn getDocument(&self) -> &dyn Document {
        unimplemented!("02_function_line_spacing tests do not call getDocument")
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

fn layouter() -> ParaLayouter {
    ParaLayouter::ParaLayouter(DocumentEnv::default(), ViewEnv::default(), MainState::default())
}

#[test]
fn LS1_missing_220_uses_default_auto_1() {
    let mut envoy = FakeEnvoy::with_cur(MutablePropertySet::new().freeze());
    let mut lo = layouter();
    lo.calcLineSpace(&mut envoy);
    assert_eq!(envoy.paraEnv.lineSpacing.getRule(), 0);
    assert!((envoy.paraEnv.lineSpacing.getSpacing() - 1.0).abs() < 1e-5);
}

#[test]
fn LS2_auto_line_spacing() {
    let mut m = MutablePropertySet::new();
    m.setValue(220, Some(Arc::new(LineSpacing::LineSpacing(0, 1.5))));
    let mut envoy = FakeEnvoy::with_cur(m.freeze());
    let mut lo = layouter();
    lo.calcLineSpace(&mut envoy);
    assert_eq!(envoy.paraEnv.lineSpacing.getRule(), 0);
    assert!((envoy.paraEnv.lineSpacing.getSpacing() - 1.5).abs() < 1e-5);
}

#[test]
fn LS3_exact_line_spacing() {
    let mut m = MutablePropertySet::new();
    m.setValue(220, Some(Arc::new(LineSpacing::LineSpacing(1, 240.0))));
    let mut envoy = FakeEnvoy::with_cur(m.freeze());
    let mut lo = layouter();
    lo.calcLineSpace(&mut envoy);
    assert_eq!(envoy.paraEnv.lineSpacing.getRule(), 1);
    assert!((envoy.paraEnv.lineSpacing.getSpacing() - 240.0).abs() < 1e-5);
}
