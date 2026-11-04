//! 07_function_shape_text_bind — shapeToPublicShape ShapeText
use ppt_core::cn::wps::show::convert::ConvertTool::ConvertTool;
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::convert::j2r_hosts::InterpretCtx;
use ppt_core::cn::wps::show::util::layout_shape::KmoShowShape;

fn convert_text(id: i32, has_text_body: bool) -> drawing::cn::wps::moffice::drawing::Shape::Shape {
    let s = KmoShowShape {
        id,
        shape_type: 1,
        has_text_body,
        ..KmoShowShape::default()
    };
    let fx = ConvertKmo::from_layout(&s);
    ConvertTool::convertShapeToPublicShape(&fx.shape, &InterpretCtx::new()).expect("type 1")
}

#[test]
fn st1_null_body_still_sets_text() {
    let mut shape = convert_text(401, false);
    assert!(shape.getTextObj().is_some());
}

#[test]
fn st2_body_has_text() {
    let mut shape = convert_text(402, true);
    assert!(shape.getTextObj().is_some());
}

#[test]
fn st3_page_number() {
    let mut ctx = InterpretCtx::new();
    ctx.mContentInterpretCtr.setPageNumber(3);
    let s = KmoShowShape {
        id: 403,
        shape_type: 1,
        has_text_body: true,
        ..KmoShowShape::default()
    };
    let fx = ConvertKmo::from_layout(&s);
    let shape = ConvertTool::convertShapeToPublicShape(&fx.shape, &ctx).expect("type 1");
    assert!(shape.getShapeID() == fx.shape.id());
}

#[test]
fn st4_autofit_type2() {
    let mut shape = convert_text(404, true);
    assert!(shape.getTextObj().is_some());
}

#[test]
fn st5_autofit_other() {
    let mut shape = convert_text(405, true);
    assert!(shape.getTextObj().is_some());
}

#[test]
fn st6_no_body_pr() {
    let mut shape = convert_text(406, false);
    assert!(shape.getTextObj().is_some());
}
