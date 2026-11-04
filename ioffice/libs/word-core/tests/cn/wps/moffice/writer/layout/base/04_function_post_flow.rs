// function_id=04_function_post_flow
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::page::PageLayouter::PageLayouter;
use word_core::cn::wps::moffice::writer::layout::base::ParaLayouter::ParaLayouter;
use word_core::cn::wps::moffice::writer::layout::base::TextLayouter::TextLayouter;

fn layouter_started() -> TextLayouter {
    let mut t = TextLayouter::default();
    let mut para = ParaLayouter::ParaLayouter(
        DocumentEnv::default(),
        ViewEnv::default(),
        MainState::default(),
    );
    para.bStarted = true;
    t.mParaLayouter = Some(para);
    let mut page = PageLayouter::PageLayouter(DocumentEnv::default(), ViewEnv::default());
    page.bStarted = true;
    t.mPageLayouter = Some(page);
    t
}

#[test]
fn PO1_post_flow_ends_para_and_page() {
    let mut lo = layouter_started();
    let mut st = LayoutState::default();
    lo.postFlow(&mut st);
    assert!(!lo.mParaLayouter.as_ref().unwrap().hasStart());
    assert!(!lo.mPageLayouter.as_ref().unwrap().hasStart());
}

#[test]
fn PO2_table_flag_skips_brc_without_panic() {
    let mut lo = layouter_started();
    lo.mHasFlowNormalTable = true;
    let mut st = LayoutState::default();
    lo.postFlow(&mut st);
    assert!(!lo.mParaLayouter.as_ref().unwrap().hasStart());
}
