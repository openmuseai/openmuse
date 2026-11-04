// Test 01_function_typo_shape_snapshot — flags + render rect（pl-06）
// 冻结矩阵 FUNCTION.md §3.2 TS1–TS8。
use std::sync::{Arc, Once};

use ppt_core::cn::wps::base::converter::UnitsConverter::UnitsConverter;
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use ppt_core::cn::wps::show::util::layout_shape::{KmoShowShape, Placeholder};
use util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;
use util::cn::wps::graphics::RectF::RectF;
use util_adapter::cn::wps::core::runtime::Platform::Platform;

static INIT: Once = Once::new();

fn setup_metrics() {
    INIT.call_once(|| {
        Platform::setDisplayMetrics(Arc::new(KDisplayMetrics::new_with_params(
            0, 0, 1.0, 160, 1.0, 96.0, 96.0,
        )));
    });
}

fn expected_render(l: f32, t: f32, r: f32, b: f32) -> (f32, f32, f32, f32) {
    let u = UnitsConverter::render();
    (
        u.fromEmuX(l),
        u.fromEmuY(t),
        u.fromEmuX(r),
        u.fromEmuY(b),
    )
}

fn layout_shape(hidden: bool, ph: bool, ph_type: i32, text_box: bool) -> KmoShowShape {
    KmoShowShape {
        is_hidden: hidden,
        is_placeholder: ph,
        is_text_box: text_box,
        ph: Placeholder {
            has_type: ph,
            type_: ph_type,
            has_idx: false,
            idx: 0,
        },
        transform2DX: 99,
        transform2DY: 98,
        transform2DW: 10,
        transform2DH: 20,
        ..KmoShowShape::default()
    }
}

fn shape(hidden: bool, ph: bool, ph_type: i32, text_box: bool) -> ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape {
    ConvertKmo::from_layout(&layout_shape(hidden, ph, ph_type, text_box)).into_shape()
}

#[test]
fn ts1_flags_combinations() {
    setup_metrics();
    let cases = [
        (false, false, 0, true),
        (false, true, 1, false),
        (true, false, 0, false),
        (true, true, 2, true),
    ];
    for (h, ph, ty, tb) in cases {
        let s = ShowTypoShape::typo_shape_snapshot(
            Some(shape(h, ph, ty, tb)),
            None,
            Some(RectF::new()),
        );
        assert_eq!(s.isHidden(), h);
        assert_eq!(s.isPlaceholder(), ph);
        assert_eq!(s.placeholderType(), ty);
        assert_eq!(s.isTextBox(), tb);
        assert!(s.getShowShape().is_some());
    }
}

#[test]
fn ts2_ph_type_passthrough() {
    setup_metrics();
    for ty in [0, 1, 2, 7] {
        let s = ShowTypoShape::typo_shape_snapshot(
            Some(shape(false, true, ty, false)),
            None,
            None,
        );
        assert!(s.isPlaceholder());
        assert_eq!(s.placeholderType(), ty);
    }
}

#[test]
fn ts3_hidden_true_false() {
    setup_metrics();
    let vis = ShowTypoShape::typo_shape_snapshot(Some(shape(false, false, 0, false)), None, None);
    let hid = ShowTypoShape::typo_shape_snapshot(Some(shape(true, false, 0, false)), None, None);
    assert!(!vis.isHidden());
    assert!(hid.isHidden());
}

#[test]
fn ts4_null_shape() {
    setup_metrics();
    let s = ShowTypoShape::typo_shape_snapshot(None, None, None);
    assert!(s.isHidden());
    assert!(!s.isPlaceholder());
    assert_eq!(s.placeholderType(), 0);
    assert!(!s.isTextBox());
    assert!(s.getShowShape().is_none());
}

#[test]
fn ts5_emu_to_render_rect() {
    setup_metrics();
    let mut s = ShowTypoShape::typo_shape_snapshot(
        Some(shape(false, false, 0, true)),
        None,
        Some(RectF::new_with(0.0, 0.0, 1_219_200.0, 685_800.0)),
    );
    let got = s.getShapeRect();
    let exp = expected_render(0.0, 0.0, 1_219_200.0, 685_800.0);
    assert_eq!((got.left, got.top, got.right, got.bottom), exp);
}

#[test]
fn ts6_rect_cache_idempotent() {
    setup_metrics();
    let mut s = ShowTypoShape::typo_shape_snapshot(
        Some(shape(false, false, 0, false)),
        None,
        Some(RectF::new_with(12700.0, 25400.0, 38100.0, 50800.0)),
    );
    let a = s.getShapeRect();
    let b = s.getShapeRect();
    assert_eq!(
        (a.left, a.top, a.right, a.bottom),
        (b.left, b.top, b.right, b.bottom)
    );
}

#[test]
fn ts7_clear_cache_recomputes() {
    setup_metrics();
    let mut s = ShowTypoShape::typo_shape_snapshot(
        Some(shape(false, false, 0, false)),
        None,
        Some(RectF::new_with(0.0, 0.0, 12700.0, 12700.0)),
    );
    let first = s.getShapeRect();
    s.clearShapeRectCache();
    let second = s.getShapeRect();
    assert_eq!(
        (first.left, first.top, first.right, first.bottom),
        (second.left, second.top, second.right, second.bottom)
    );
    let exp = expected_render(0.0, 0.0, 12700.0, 12700.0);
    assert_eq!(
        (second.left, second.top, second.right, second.bottom),
        exp
    );
}

#[test]
fn ts8_geometry_not_recomputed_from_transform() {
    setup_metrics();
    let geom = RectF::new_with(1000.0, 2000.0, 3000.0, 4000.0);
    let mut s = ShowTypoShape::typo_shape_snapshot(
        Some(shape(false, false, 0, false)),
        None,
        Some(geom),
    );
    let got = s.getShapeRect();
    let exp = expected_render(1000.0, 2000.0, 3000.0, 4000.0);
    assert_eq!((got.left, got.top, got.right, got.bottom), exp);
    let u = UnitsConverter::render();
    let from_xfrm = (
        u.fromEmuX(99.0),
        u.fromEmuY(98.0),
        u.fromEmuX(109.0),
        u.fromEmuY(118.0),
    );
    assert_ne!(
        (got.left, got.top, got.right, got.bottom),
        from_xfrm
    );
}
