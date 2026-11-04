// function_id=01_function_wrap_maxwidth
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::{
    ClipRect, DrawingLayouter, IStateEnvoy, PageEnv, TypoDrawing, TypoPage, TypoRectCoreImpl,
};
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

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

/// Java TextWrapLayouter.initClipRect + ClipRect subtract, then
/// LineLayouter.prepareSubLineEnvs copies strip left/right, SubLineLayouter.flow
/// copies to layoutLeft/layoutRight. EnglishBreaker maxWidth = layoutRight - layoutLeft.
fn wrap_line_max_width(core_width: i32, wraps: &[(i32, i32)]) -> i32 {
    let mut clip = ClipRect::default();
    clip.init(0, 0, core_width, 100);
    for &(l, r) in wraps {
        clip.subtract_wrap(l, r);
    }
    let mut env = SubLineEnv::default();
    if clip.remaining.is_empty() {
        env.left = 0;
        env.right = core_width;
    } else {
        let (l, r) = clip.remaining[0];
        env.left = l;
        env.right = r;
    }
    env.layoutLeft = env.left;
    env.layoutRight = env.right;
    env.layoutRight - env.layoutLeft
}

fn tool() -> DrawingLayouter {
    DrawingLayouter::DrawingLayouter(
        MainState::default(),
        DocumentEnv::default(),
        ViewEnv::default(),
        None,
    )
}

fn page(core_width: i32) -> TypoPage {
    TypoPage {
        coreWidth: core_width,
        coreHeight: 2000,
        width: core_width + 200,
        height: 2200,
        lines: 1,
        pointer: 1,
        ..TypoPage::default()
    }
}

/// WW1: left wrap shrinks remaining line width below page core width.
#[test]
fn ww1_left_wrap_shrinks_max_width() {
    let core = 1000;
    let max_width = wrap_line_max_width(core, &[(0, 200)]);
    assert!(max_width < core, "left wrap must shrink maxWidth");
    assert_eq!(max_width, 800);

    let t = tool();
    let drawing = TypoDrawing {
        left: 0,
        top: 0,
        width: 200,
        height: 100,
        wrap: true,
        ..TypoDrawing::default()
    };
    let mut layout = TypoRectCoreImpl::default();
    drawing.layoutRectForPageCore(&mut layout);
    let mut affect = TypoRectCoreImpl::default();
    let envoy = Envoy {
        page: page(core),
        affect: TypoRectCoreImpl::default(),
        max_wrap_cp: 0,
        page_env: PageEnv {
            coreWidth: core,
            ..PageEnv::default()
        },
    };
    let hit = t.doCalcAffectRect(&drawing, 0, &layout, &mut affect, &envoy);
    assert!(hit);
    assert!(!affect.isEmpty());
}

/// WW2: no wrap drawing → full content width.
#[test]
fn ww2_no_drawing_full_width() {
    let core = 1000;
    let max_width = wrap_line_max_width(core, &[]);
    assert_eq!(max_width, core);

    let t = tool();
    let drawing = TypoDrawing {
        left: 0,
        top: 0,
        width: 0,
        height: 0,
        wrap: false,
        ..TypoDrawing::default()
    };
    let mut layout = TypoRectCoreImpl::default();
    drawing.layoutRectForPageCore(&mut layout);
    let mut affect = TypoRectCoreImpl::default();
    let envoy = Envoy {
        page: page(core),
        affect: TypoRectCoreImpl::default(),
        max_wrap_cp: 0,
        page_env: PageEnv {
            coreWidth: core,
            ..PageEnv::default()
        },
    };
    let hit = t.doCalcAffectRect(&drawing, 0, &layout, &mut affect, &envoy);
    assert!(!hit);
    assert!(affect.isEmpty());
}

/// WW3: wrap on both sides leaves a middle text strip.
#[test]
fn ww3_both_sides_middle_strip() {
    let core = 1000;
    let max_width = wrap_line_max_width(core, &[(0, 200), (800, 1000)]);
    assert_eq!(max_width, 600);
    assert!(max_width > 0);

    let mut envoy = Envoy {
        page: page(core),
        affect: TypoRectCoreImpl::default(),
        max_wrap_cp: 0,
        page_env: PageEnv {
            coreWidth: core,
            ..PageEnv::default()
        },
    };
    let t = tool();
    t.onFlowWrapDrawing(&mut envoy, 42);
    assert_eq!(envoy.max_wrap_cp, 42);
}
