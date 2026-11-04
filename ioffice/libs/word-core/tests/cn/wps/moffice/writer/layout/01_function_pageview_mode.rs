// function_id=01_function_pageview_mode
#![allow(non_snake_case)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use word_core::cn::wps::moffice::writer::layout::FlowLayout::FlowLayout;
use word_core::cn::wps::moffice::writer::layout::FlowLayout::MainLayouter;
use word_core::cn::wps::moffice::writer::layout::FlowLayout::PageViewLayouter;

#[test]
fn VM1_mode_zero_is_page_view() {
    assert_eq!(MainLayouter::PAGEVIEW_LAYOUTER, 0);
    let mut fl = FlowLayout::default();
    fl.createDocLayouter(MainLayouter::PAGEVIEW_LAYOUTER);
    assert!(fl.mMainLayouter.is_some());
    let _ty: &PageViewLayouter = fl.mMainLayouter.as_ref().unwrap();
}

#[test]
fn VM2_mode_one_is_deferred() {
    let mut fl = FlowLayout::default();
    let panicked = catch_unwind(AssertUnwindSafe(|| {
        fl.createDocLayouter(MainLayouter::WEBVIEW_LAYOUTER);
    }))
    .is_err();
    assert!(panicked, "WebView mode must stay DEFERRED_NON_FUNCTION");
}
