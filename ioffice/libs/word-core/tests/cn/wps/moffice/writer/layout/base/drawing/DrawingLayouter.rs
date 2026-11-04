// function_id=02_function_drawing_flow
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::{
    DrawingLayouter, IStateEnvoy, PageEnv, TypoDrawing, TypoPage, TypoRectCoreImpl,
};
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;

struct Envoy {
    page: TypoPage,
    affect: TypoRectCoreImpl,
    max_wrap_cp: i32,
    page_env: PageEnv,
}

impl IStateEnvoy for Envoy {
    fn setMaxWrapDrawingCP(&mut self, var1: i32) {
        self.max_wrap_cp = var1;
    }
    fn getMaxWrapDrawingCP(&self) -> i32 {
        self.max_wrap_cp
    }
    fn getCurPage(&self) -> TypoPage {
        self.page.clone()
    }
    fn getAffectRect(&self) -> TypoRectCoreImpl {
        self.affect.clone()
    }
    fn getAffectRectMut(&mut self) -> &mut TypoRectCoreImpl {
        &mut self.affect
    }
    fn getPageEnv(&self) -> PageEnv {
        self.page_env.clone()
    }
}

fn tool() -> DrawingLayouter {
    DrawingLayouter::DrawingLayouter(
        MainState::default(),
        DocumentEnv::default(),
        ViewEnv::default(),
        None,
    )
}

fn envoy() -> Envoy {
    Envoy {
        page: TypoPage {
            coreWidth: 1000,
            coreHeight: 2000,
            width: 1200,
            height: 2200,
            marginLeft: 100,
            pointer: 1,
            lines: 1,
            ..TypoPage::default()
        },
        affect: TypoRectCoreImpl::default(),
        max_wrap_cp: 0,
        page_env: PageEnv {
            coreWidth: 1000,
            marginLeft: 100,
            ..PageEnv::default()
        },
    }
}

/// DF1: wrap drawing anchor CP is recorded as max wrap CP (Java onFlowWrapDrawing).
#[test]
fn df1_anchor_cp() {
    let t = tool();
    let mut e = envoy();
    t.onFlowWrapDrawing(&mut e, 17);
    assert_eq!(e.max_wrap_cp, 17);
    t.onFlowWrapDrawing(&mut e, 9);
    assert_eq!(e.max_wrap_cp, 17);
    t.onFlowWrapDrawing(&mut e, 40);
    assert_eq!(e.max_wrap_cp, 40);
}

/// DF2: wrap drawing above the page is clamped back (Java correctIfWrapShapeOutOfPage).
#[test]
fn df2_correct_out_of_page() {
    let mut t = tool();
    let mut e = envoy();
    let mut drawing = TypoDrawing {
        left: 0,
        top: -80,
        width: 100,
        height: 50,
        wrap: true,
        ..TypoDrawing::default()
    };
    t.correctIfWrapShapeOutOfPage(&mut drawing, None, &mut e);
    assert!(drawing.top >= 0, "wrap drawing must be clamped into the page");
}

/// DF3: Java onStart/onEnd are empty; calling both must not panic.
#[test]
fn df3_on_start_on_end() {
    let t = tool();
    let e = envoy();
    t.onStart(&e);
    t.onEnd(&e);
}
