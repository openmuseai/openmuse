#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::page::PageLayouter::PageLayouter;

// java public class MainPageLayouter extends PageLayouter
pub struct MainPageLayouter {
    // java PageLayouter super_
    pub(crate) super_: PageLayouter,
    // java protected MainState mMainState;
    pub(crate) mMainState: *mut MainState,
    // java DocumentEnv mDocEnv / ViewEnv mViewEnv live on the parent layouter.
    // Rust aliases the same objects instead of moving them out of PageViewLayouter.
    pub(crate) mDocEnv: *mut DocumentEnv,
    pub(crate) mViewEnv: *mut ViewEnv,
}

impl Default for MainPageLayouter {
    fn default() -> Self {
        Self {
            super_: PageLayouter::default(),
            mMainState: std::ptr::null_mut(),
            mDocEnv: std::ptr::null_mut(),
            mViewEnv: std::ptr::null_mut(),
        }
    }
}

impl MainPageLayouter {
    // java public MainPageLayouter(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn MainPageLayouter(main_state: *mut MainState, doc_env: *mut DocumentEnv, view_env: *mut ViewEnv) -> Self {
        Self {
            super_: PageLayouter::default(),
            mMainState: main_state,
            mDocEnv: doc_env,
            mViewEnv: view_env,
        }
    }
}
