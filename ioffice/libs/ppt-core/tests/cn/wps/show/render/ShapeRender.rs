// Test 02_function_begin_filter — ShapeRender._begin（pr-01）
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::render::ShapeRender::{BeginFilter, ShapeRender};
use ppt_core::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use ppt_core::cn::wps::show::util::DrawFlags::DrawFlags;
use ppt_core::cn::wps::show::util::layout_shape::{KmoShowShape, Placeholder};

fn snap(hidden: bool, ph: bool, ph_type: i32) -> ShowTypoShape {
    ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::from_layout(&KmoShowShape {
            id: 500 + ph_type + if hidden { 10 } else { 0 } + if ph { 20 } else { 0 },
            is_hidden: hidden,
            is_placeholder: ph,
            ph: Placeholder {
                has_type: ph,
                type_: ph_type,
                has_idx: false,
                idx: 0,
            },
            ..KmoShowShape::default()
        }).into_shape()),
        None,
        None,
    )
}

#[test]
fn t01_hidden_skip() {
    let f = ShapeRender::begin_filter(Some(snap(true, false, 0)), 0);
    assert_eq!(f, BeginFilter::Skip);
}

#[test]
fn t02_placeholder_flag1() {
    let f = ShapeRender::begin_filter(Some(snap(false, true, 1)), DrawFlags::NOT_DRAW_PLACEHOLDER);
    assert_eq!(f, BeginFilter::Skip);
}

#[test]
fn t03_miner_type4() {
    let f = ShapeRender::begin_filter(
        Some(snap(false, true, 4)),
        DrawFlags::NOT_DRAW_PLACEHOLDER_MINOR,
    );
    assert_eq!(f, BeginFilter::Skip);
}

#[test]
fn t04_minor_flag_title_type_pass() {
    let f = ShapeRender::begin_filter(
        Some(snap(false, true, 2)),
        DrawFlags::NOT_DRAW_PLACEHOLDER_MINOR,
    );
    assert_eq!(f, BeginFilter::Pass);
}

#[test]
fn t05_normal_pass() {
    let f = ShapeRender::begin_filter(Some(snap(false, false, 0)), 0);
    assert_eq!(f, BeginFilter::Pass);
}

#[test]
#[ignore = "spec: post-MVP"]
fn t06_modular() {}
