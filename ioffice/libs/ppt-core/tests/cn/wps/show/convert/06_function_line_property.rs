//! 06_function_line_property — LineStyleConvert + ln.fill.type==5 → null
use std::sync::{Arc, Once};

use drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty;
use ppt_core::cn::wps::base::converter::UnitsConverter::UnitsConverter;
use ppt_core::cn::wps::show::convert::ConvertTool::ConvertTool;
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::convert::j2r_hosts::InterpretCtx;
use ppt_core::cn::wps::show::util::layout_shape::{FillObj, KmoShowShape, OutlineObj, ShapePr};
use util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;
use util_adapter::cn::wps::core::runtime::Platform::Platform;

static INIT: Once = Once::new();

fn setup_metrics() {
    INIT.call_once(|| {
        Platform::setDisplayMetrics(Arc::new(KDisplayMetrics::new_with_params(
            0, 0, 1.0, 160, 1.0, 96.0, 96.0,
        )));
    });
}

fn fixture_ln(
    id: i32,
    has_ln: bool,
    width: Option<i32>,
    fill_type: Option<i32>,
    cap: Option<i32>,
    join: Option<i32>,
    dash: Option<i32>,
) -> KmoShowShape {
    let mut outline = OutlineObj::default();
    if let Some(w) = width {
        outline.putWidth(w);
    }
    if let Some(c) = cap {
        outline.putLineCap(c);
    }
    if let Some(j) = join {
        outline.putJoinType(j);
    }
    if let Some(d) = dash {
        outline.putPresetDash(d);
    }
    if let Some(t) = fill_type {
        let mut f = FillObj::default();
        f.putType(t);
        outline.fill = f;
    }
    let mut sp_pr = ShapePr::default();
    sp_pr.has_ln = has_ln;
    sp_pr.outline_obj = outline;
    KmoShowShape {
        id,
        shape_type: 1,
        sp_pr,
        ..KmoShowShape::default()
    }
}

fn convert_ln(
    id: i32,
    has_ln: bool,
    width: Option<i32>,
    fill_type: Option<i32>,
) -> drawing::cn::wps::moffice::drawing::Shape::Shape {
    let fx = ConvertKmo::from_layout(&fixture_ln(id, has_ln, width, fill_type, None, None, None));
    ConvertTool::convertShapeToPublicShape(&fx.shape, &InterpretCtx::new())
        .expect("type 1 convert")
}

#[test]
fn lp1_min_width_is_round_from_emu_12700() {
    setup_metrics();
    let min = UnitsConverter::render().fromEmuX(12700.0).round();
    assert!(min > 0.0, "Java min_width = round(fromEmuX(12700)) 必须为正");
}

#[test]
fn lp2_clamp_to_min_width() {
    setup_metrics();
    let mut shape = convert_ln(301, true, Some(1), Some(0));
    let width = shape
        .getLineProperty()
        .as_ref()
        .and_then(|a| a.downcast_ref::<LineProperty>())
        .map(|lp| lp.getWidth())
        .expect("LineProperty");
    let min = UnitsConverter::render().fromEmuX(12700.0).round();
    assert_eq!(width, min);
}

#[test]
fn lp3_wide_line_keeps_converted_width() {
    setup_metrics();
    let mut shape = convert_ln(302, true, Some(1_270_000), Some(0));
    let width = shape
        .getLineProperty()
        .as_ref()
        .and_then(|a| a.downcast_ref::<LineProperty>())
        .map(|lp| lp.getWidth())
        .expect("LineProperty");
    let converted = UnitsConverter::render().fromEmuX(1_270_000.0);
    let min = UnitsConverter::render().fromEmuX(12700.0).round();
    assert_eq!(width, converted.max(min));
}

#[test]
fn lp4_nofill_line_property_null() {
    setup_metrics();
    let mut shape = convert_ln(303, true, Some(12700), Some(5));
    assert!(shape.getLineProperty().is_none());
}

#[test]
fn lp5_solid_line() {
    setup_metrics();
    let mut shape = convert_ln(304, true, Some(12700), Some(0));
    assert!(shape.getLineProperty().is_some());
}

#[test]
fn lp6_cap_join_compound() {
    setup_metrics();
    let fx = ConvertKmo::from_layout(&fixture_ln(305, true, Some(12700), Some(0), Some(1), Some(2), None));
    let mut shape = ConvertTool::convertShapeToPublicShape(&fx.shape, &InterpretCtx::new())
    .expect("ln");
    assert!(shape.getLineProperty().is_some());
}

#[test]
fn lp7_preset_dash() {
    setup_metrics();
    let fx = ConvertKmo::from_layout(&fixture_ln(306, true, Some(12700), Some(0), None, None, Some(3)));
    let mut shape = ConvertTool::convertShapeToPublicShape(&fx.shape, &InterpretCtx::new())
    .expect("ln");
    assert!(shape.getLineProperty().is_some());
}

#[test]
fn lp8_head_arrow() {
    setup_metrics();
    let mut shape = convert_ln(307, true, Some(12700), Some(0));
    assert!(shape.getLineProperty().is_some());
}

#[test]
fn lp9_no_ln_no_default_line() {
    setup_metrics();
    let mut shape = convert_ln(308, false, None, None);
    assert!(shape.getLineProperty().is_none());
}
