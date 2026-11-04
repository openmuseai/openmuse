// function_id=02_function_view_env
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::base::ViewMode::{MODE_PAGE, MODE_PHONE, MODE_WEB};
use word_core::cn::wps::moffice::writer::data::value::PageSetup::PageSetup;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;

fn default_ps() -> PageSetup {
    PageSetup::PageSetup(11906, 16838, 1800, 1800, 1440, 1440)
}

/// V1 ENV-SETGET
#[test]
fn V1_setget() {
    let mut env = ViewEnv::new();
    env.setViewWidth(8306);
    env.setViewHeight(13598);
    assert_eq!(env.getViewWidth(), 8306);
    assert_eq!(env.getViewHeight(), 13598);
}

/// V2 ENV-CONTENT default page + zoom 1
#[test]
fn V2_content() {
    let mut env = ViewEnv::new();
    env.setPageSetup(default_ps());
    env.setZoom(1);
    assert_eq!(env.contentWidth(), 8306);
    assert_eq!(env.contentHeight(), 13958);
}

/// V3 ENV-ZOOM zoom=2 halves content (integer)
#[test]
fn V3_zoom() {
    let mut env = ViewEnv::new();
    env.setPageSetup(default_ps());
    env.setZoom(2);
    assert_eq!(env.contentWidth(), 4153);
    assert_eq!(env.contentHeight(), 6979);
}

/// V4 ENV-MODE PAGEVIEW
#[test]
fn V4_mode() {
    let mut env = ViewEnv::new();
    env.setViewMode(MODE_PAGE);
    assert!(env.isPageMode());
    assert!(!env.isWebLayoutMode());
    assert!(!env.isPhoneReadMode());
    env.setViewMode(MODE_WEB);
    assert!(!env.isPageMode());
    env.setViewMode(MODE_PHONE);
    assert!(env.isPhoneReadMode());
}

/// V5 ENV-RESET changeSize flag
#[test]
fn V5_changesize() {
    let mut env = ViewEnv::new();
    env.setViewWidth(100);
    env.changeSize(true);
    assert!(env.takeNeedRelayout());
    assert!(!env.takeNeedRelayout());
}

/// V6 ENV-SCROLL
#[test]
fn V6_scroll() {
    let mut env = ViewEnv::new();
    env.onScroll(120);
    assert_eq!(env.getViewTop(), 120);
}
