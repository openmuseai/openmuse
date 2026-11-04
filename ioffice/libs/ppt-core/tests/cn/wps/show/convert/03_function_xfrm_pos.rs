//! 03_function_xfrm_pos — toDegree + property(733) EMU→px + GRF flip
use std::sync::{Arc, Once};

use drawing::cn::wps::moffice::drawing::GRF::GRF;
use drawing::cn::wps::moffice::drawing::ShapePos::ShapePos;
use ppt_core::cn::wps::base::converter::UnitsConverter::UnitsConverter;
use ppt_core::cn::wps::show::convert::ConvertTool::ConvertTool;
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::convert::j2r_hosts::InterpretCtx;
use ppt_core::cn::wps::show::util::layout_shape::KmoShowShape;
use ppt_core::cn::wps::show::util::ShapeUtil::ShapeUtil;
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

fn fixture_xfrm(
    id: i32,
    rotation: i32,
    flip_h: bool,
    flip_v: bool,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> KmoShowShape {
    KmoShowShape {
        id,
        shape_type: 1,
        rotation,
        isFlipHorz: flip_h,
        isFlipVert: flip_v,
        transform2DX: x,
        transform2DY: y,
        transform2DW: w,
        transform2DH: h,
        has_xfrm: true,
        ..KmoShowShape::default()
    }
}

fn convert_xfrm(
    id: i32,
    rotation: i32,
    flip_h: bool,
    flip_v: bool,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> drawing::cn::wps::moffice::drawing::Shape::Shape {
    let fx = ConvertKmo::from_layout(&fixture_xfrm(id, rotation, flip_h, flip_v, x, y, w, h));
    ConvertTool::convertShapeToPublicShape(&fx.shape, &InterpretCtx::new())
    .expect("type 1 convert")
}

#[test]
fn xp1_to_degree_divides_60000() {
    assert_eq!(ShapeUtil::toDegree(0), 0.0);
    assert_eq!(ShapeUtil::toDegree(60000), 1.0);
    assert_eq!(ShapeUtil::toDegree(-60000), -1.0);
    assert_eq!(ShapeUtil::toDegree(10800000), 180.0);
}

#[test]
fn xp2_property_733_is_render_px() {
    setup_metrics();
    let mut shape = convert_xfrm(20, 0, false, false, 0, 0, 1219200, 685800);
    let u = UnitsConverter::render();
    let expected_l = u.fromEmuX(0.0);
    let expected_t = u.fromEmuY(0.0);
    let expected_r = u.fromEmuX(1219200.0);
    let expected_b = u.fromEmuY(685800.0);
    let rc = shape
        .getPropertyValue(733)
        .and_then(|a| a.downcast_ref::<RectF>().cloned())
        .expect("property 733 is render-unit RectF");
    assert_eq!(rc.left, expected_l);
    assert_eq!(rc.top, expected_t);
    assert_eq!(rc.right, expected_r);
    assert_eq!(rc.bottom, expected_b);
}

#[test]
fn xp3_flip_from_shape_info() {
    let mut shape = convert_xfrm(30, 0, true, false, 0, 0, 100, 100);
    let grf = shape
        .getGRF()
        .and_then(|a| a.downcast_ref::<GRF>().cloned())
        .expect("GRF");
    assert!(grf.isFlipH(), "GRF.flipH from shapeInfo");
    assert!(!grf.isFlipV());
}

#[test]
fn xp4_null_shape_pr_is_noop() {
    setup_metrics();
    let mut s = fixture_xfrm(40, 10800000, true, true, 10, 20, 30, 40);
    s.has_sp_pr = false;
    s.shape_type = 4;
    let fx = ConvertKmo::from_layout(&s);
    let mut shape = ConvertTool::convertShapeToPublicShape(&fx.shape, &InterpretCtx::new()).expect("shell");
    assert!(shape.getPropertyValue(733).is_none());
}

#[test]
fn xp5_rotation_180_degrees() {
    let mut shape = convert_xfrm(50, 10800000, false, false, 0, 0, 100, 100);
    let rot = shape
        .getShapePos()
        .as_ref()
        .and_then(|a| a.downcast_ref::<ShapePos>())
        .map(|p| p.getRotation())
        .expect("ShapePos");
    assert_eq!(rot, 180.0);
}

#[test]
fn xp6_anchor_from_relative_rect() {
    setup_metrics();
    let mut shape = convert_xfrm(60, 0, false, false, 10, 20, 30, 40);
    let u = UnitsConverter::render();
    let rc = shape
        .getPropertyValue(733)
        .and_then(|a| a.downcast_ref::<RectF>().cloned())
        .expect("733 uses shapeInfo xfrm, not rcAnchor");
    assert_eq!(rc.left, u.fromEmuX(10.0));
    assert_eq!(rc.top, u.fromEmuY(20.0));
    assert_eq!(rc.right, u.fromEmuX(40.0));
    assert_eq!(rc.bottom, u.fromEmuY(60.0));
}
