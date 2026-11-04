// function_id=02_function_do_flow_line
#![allow(non_snake_case)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::ParaLayouter::ParaLayouter;
use word_core::cn::wps::moffice::writer::layout::base::TextLayouter::TextLayouter;

fn layouter_with_para(started: bool) -> TextLayouter {
    let mut t = TextLayouter::default();
    let mut para = ParaLayouter::ParaLayouter(
        DocumentEnv::default(),
        ViewEnv::default(),
        MainState::default(),
    );
    para.bStarted = started;
    t.mParaLayouter = Some(para);
    t
}

fn with_para_prop(st: &mut LayoutState, build: impl FnOnce(&mut MutablePropertySet)) {
    let mut m = MutablePropertySet::new();
    build(&mut m);
    let mut cur = ParaPropEnv::default();
    cur.paraProperty = Some(m.freeze());
    st.paraEnv.curParaPropEnv = Some(cur);
}

#[test]
fn DF1_started_para_goes_to_flow_text_line() {
    let mut lo = layouter_with_para(true);
    let mut st = LayoutState::default();
    let panicked = catch_unwind(AssertUnwindSafe(|| lo.doFlowLine(&mut st))).is_err();
    assert!(panicked, "flowTextLine is still DEFERRED_NON_FUNCTION (wr-12 LineLayouter.flow)");
}

#[test]
fn DF2_new_para_starts_then_text_line() {
    let mut lo = layouter_with_para(false);
    let mut st = LayoutState::default();
    with_para_prop(&mut st, |_| {});
    let panicked = catch_unwind(AssertUnwindSafe(|| lo.doFlowLine(&mut st))).is_err();
    assert!(lo.mParaLayouter.as_ref().unwrap().hasStart());
    assert!(panicked);
}

#[test]
fn DF3_table_papx_defers_to_flow_table() {
    let mut lo = layouter_with_para(false);
    let mut st = LayoutState::default();
    st.tablePeriLayer = 1;
    with_para_prop(&mut st, |m| {
        m.setBoolean(223, true);
        m.setInt(224, 1);
    });
    let panicked = catch_unwind(AssertUnwindSafe(|| lo.doFlowLine(&mut st))).is_err();
    assert!(panicked, "flowTable remains DEFERRED_NON_FUNCTION");
}

#[test]
fn DF4_text_frame_defers_to_flow_text_frame() {
    let mut lo = layouter_with_para(false);
    let mut st = LayoutState::default();
    st.textFrameLayer = 0;
    with_para_prop(&mut st, |m| {
        m.setInt(240, 1);
        m.setBoolean(256, true);
    });
    let panicked = catch_unwind(AssertUnwindSafe(|| lo.doFlowLine(&mut st))).is_err();
    assert!(panicked, "flowTextFrame remains DEFERRED_NON_FUNCTION");
}
