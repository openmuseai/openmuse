// function_id=01_function_pre_flow
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::page::PageLayouter::PageLayouter;
use word_core::cn::wps::moffice::writer::layout::base::TextLayouter::TextLayouter;

fn layouter() -> TextLayouter {
    let mut t = TextLayouter::default();
    t.mPageLayouter = Some(PageLayouter::PageLayouter(
        DocumentEnv::default(),
        ViewEnv::default(),
    ));
    t
}

#[test]
fn PF1_cp_zero_para_start_zero() {
    let mut lo = layouter();
    let mut st = LayoutState::default();
    st.cp = 0;
    lo.preFlow(&mut st);
    assert_eq!(lo.mStartCP, 0);
    assert_eq!(st.linePos, 0);
    assert_eq!(st.maxWrapDrawingCP, i32::MIN);
    assert_eq!(st.pageEndLimitCP, i32::MAX);
}

#[test]
fn PF2_mid_para_uses_cur_para_start() {
    let mut lo = layouter();
    let mut st = LayoutState::default();
    st.cp = 5;
    let mut cur = ParaPropEnv::default();
    cur.start = 2;
    cur.end = 20;
    st.paraEnv.curParaPropEnv = Some(cur);
    lo.preFlow(&mut st);
    assert_eq!(lo.mStartCP, 5);
    assert_eq!(lo.mMainState.snapshot.getCPRef(lo.mRef), 2);
}

#[test]
fn PF3_finish_page_cleared() {
    let mut lo = layouter();
    let mut st = LayoutState::default();
    st.setFinishPage(true);
    lo.preFlow(&mut st);
    assert!(!st.isFinishPage());
}
