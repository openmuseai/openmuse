// function_id=03_function_para_indent
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
}

impl FakeEnvoy {
    fn new() -> Self {
        FakeEnvoy {
            paraEnv: ParaEnv::ParaEnv(),
            pageEnv: {
                let mut p = PageEnv::default();
                p.gridModel = 0;
                p.gridWidth = 0;
                p
            },
            curPage: TypoPage::default(),
        }
    }
}

impl word_core::cn::wps::moffice::writer::layout::base::LayouterBase::IStateEnvoy for FakeEnvoy {}

impl IStateEnvoy for FakeEnvoy {
    fn getDocument(&self) -> &dyn Document {
        unimplemented!("03_function_para_indent: PropertyTool/CHPX not on this I/O")
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

fn indent_env(prop: PropertySet) -> (ParaLayouter, FakeEnvoy, ParaPropEnv) {
    let lo = layouter();
    let envoy = FakeEnvoy::new();
    let mut propEnv = ParaPropEnv::default();
    propEnv.start = 0;
    propEnv.paraProperty = Some(prop.clone());
    propEnv.firstNormalParaProperty = Some(prop);
    (lo, envoy, propEnv)
}

#[test]
fn ID1_first_line_twip_208() {
    let mut m = MutablePropertySet::new();
    m.setInt(208, 480);
    let (mut lo, mut envoy, mut propEnv) = indent_env(m.freeze());
    lo.calcIndents(&mut propEnv, &mut envoy);
    assert_eq!(propEnv.firstLineIndent, 480);
}

#[test]
fn ID2_chars_209_overrides_208() {
    let mut m = MutablePropertySet::new();
    m.setInt(208, 480);
    m.setInt(209, 100);
    m.setFloat(10, 10.0);
    let (mut lo, mut envoy, mut propEnv) = indent_env(m.freeze());
    lo.calcIndents(&mut propEnv, &mut envoy);
    // PT2TWIPS(10)=200; 200 * 100 / 100 = 200 (not 480)
    assert_eq!(propEnv.firstLineIndent, 200);
}

#[test]
fn ID3_left_twip_210() {
    let mut m = MutablePropertySet::new();
    m.setInt(210, 360);
    let (mut lo, mut envoy, mut propEnv) = indent_env(m.freeze());
    lo.calcIndents(&mut propEnv, &mut envoy);
    assert_eq!(propEnv.leftIndent, 360);
}

#[test]
fn ID4_right_twip_212() {
    let mut m = MutablePropertySet::new();
    m.setInt(212, 240);
    let (mut lo, mut envoy, mut propEnv) = indent_env(m.freeze());
    lo.calcIndents(&mut propEnv, &mut envoy);
    assert_eq!(propEnv.rightIndent, 240);
}

#[test]
fn ID5_left_chars_211_not_210() {
    let mut m = MutablePropertySet::new();
    m.setInt(210, 360);
    m.setInt(211, 100);
    let (mut lo, mut envoy, mut propEnv) = indent_env(m.freeze());
    lo.calcIndents(&mut propEnv, &mut envoy);
    // gridWidth 0 → EMU2TWIPS(0)*chars*0.01 = 0, not 360
    assert_eq!(propEnv.leftIndent, 0);
}

#[test]
fn ID6_hanging_negative_208() {
    let mut m = MutablePropertySet::new();
    m.setInt(208, -480);
    let (mut lo, mut envoy, mut propEnv) = indent_env(m.freeze());
    lo.calcIndents(&mut propEnv, &mut envoy);
    assert_eq!(propEnv.firstLineIndent, -480);
}
