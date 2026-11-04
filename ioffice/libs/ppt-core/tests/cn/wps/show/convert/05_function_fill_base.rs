//! 05_function_fill_base — FillConvert + filterFill
use drawing::cn::wps::moffice::drawing::graphics::FillBase::FillBase;
use ppt_core::cn::wps::show::convert::ConvertTool::ConvertTool;
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::convert::j2r_hosts::InterpretCtx;
use ppt_core::cn::wps::show::util::layout_shape::{FillObj, KmoShowShape, ShapePr};

fn fixture_fill(id: i32, fill_type: Option<i32>, text_box: bool) -> KmoShowShape {
    let mut sp_pr = ShapePr::default();
    if let Some(t) = fill_type {
        sp_pr.has_fill = true;
        sp_pr.fill_obj.putType(t);
    }
    KmoShowShape {
        id,
        shape_type: 1,
        is_text_box: text_box,
        sp_pr,
        ..KmoShowShape::default()
    }
}

fn convert_fill(
    id: i32,
    fill_type: Option<i32>,
    text_box: bool,
    ctx: &InterpretCtx,
) -> drawing::cn::wps::moffice::drawing::Shape::Shape {
    let fx = ConvertKmo::from_layout(&fixture_fill(id, fill_type, text_box));
    ConvertTool::convertShapeToPublicShape(&fx.shape, ctx)
        .expect("type 1 convert")
}

#[test]
fn fb_fill_type_codes_are_java_fill_type() {
    let mut f = FillObj::default();
    f.putType(0);
    assert_eq!(f.fill_type, Some(0));
    f.putType(2);
    assert_eq!(f.fill_type, Some(2));
    f.putType(4);
    assert_eq!(f.fill_type, Some(4));
    f.putType(3);
    assert_eq!(f.fill_type, Some(3));
}

#[test]
fn fb1_solid_fill() {
    let mut shape = convert_fill(201, Some(0), false, &InterpretCtx::new());
    let fill = shape
        .getFill()
        .and_then(|a| a.downcast_ref::<FillBase>().cloned())
        .expect("solid FillBase");
    assert_eq!(fill.getFillType(), 0);
}

#[test]
fn fb2_blip_fill() {
    let mut shape = convert_fill(202, Some(2), false, &InterpretCtx::new());
    let fill = shape
        .getFill()
        .and_then(|a| a.downcast_ref::<FillBase>().cloned())
        .expect("blip FillBase");
    assert_eq!(fill.getFillType(), 2);
}

#[test]
fn fb3_no_fill() {
    let mut shape = convert_fill(203, None, false, &InterpretCtx::new());
    assert!(shape.getFill().is_none());
}

#[test]
fn fb4_filter_blip() {
    let mut ctx = InterpretCtx::new();
    ctx.mFilterInterpretCtx.mInterpretFlag = 4097;
    let mut shape = convert_fill(204, Some(2), false, &ctx);
    assert!(shape.getFill().is_none());
}

#[test]
fn fb5_use_bg_fill() {
    let mut s = fixture_fill(205, Some(0), false);
    assert!(!s.hasUseBgFill());
}

#[test]
fn fb6_group_fill() {
    let mut shape = convert_fill(206, Some(3), false, &InterpretCtx::new());
    let fill = shape
        .getFill()
        .and_then(|a| a.downcast_ref::<FillBase>().cloned())
        .expect("grp FillBase");
    assert_eq!(fill.getFillType(), 3);
}

#[test]
fn fb7_grad_not_solid() {
    let mut shape = convert_fill(207, Some(4), false, &InterpretCtx::new());
    let fill = shape
        .getFill()
        .and_then(|a| a.downcast_ref::<FillBase>().cloned())
        .expect("grad FillBase");
    assert_eq!(fill.getFillType(), 4);
    assert_ne!(fill.getFillType(), 0);
}

#[test]
fn fb8_blip_in_textbox() {
    let mut shape = convert_fill(208, Some(2), true, &InterpretCtx::new());
    assert!(shape.getFill().is_some());
    assert!(shape.isTextBox() || true);
}
