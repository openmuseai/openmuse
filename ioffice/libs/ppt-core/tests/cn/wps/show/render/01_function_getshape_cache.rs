//! 01_function_getshape_cache — ShowTypoShape.getShape + convert miss
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::convert::j2r_hosts::InterpretCtx;
use ppt_core::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use util::cn::wps::graphics::RectF::RectF;

fn snapshot(shape_type: i32, index: i32) -> ShowTypoShape {
    ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::new(shape_type, index).into_shape()),
        None,
        Some(RectF::new()),
    )
}

fn spid(shape_type: i32, index: i32) -> i32 {
    ConvertKmo::encode_spid(shape_type, index)
}

#[test]
fn gs1_cache_miss_converts() {
    let mut typo = snapshot(1, 501);
    let shape = typo.getShape().expect("miss 必须 convert 出公开 Shape");
    assert_eq!(shape.getShapeID(), spid(1, 501));
}

#[test]
fn gs2_same_instance_memoizes() {
    let mut typo = snapshot(1, 502);
    let a = typo.getShape().expect("first");
    let b = typo.getShape().expect("second");
    assert_eq!(a.getShapeID(), b.getShapeID());
    assert_eq!(a.getShapeID(), spid(1, 502));
}

#[test]
fn gs3_same_id_hits_slide_cache() {
    let mut a = snapshot(1, 503);
    let mut b = snapshot(1, 503);
    let sa = a.getShape().expect("first typo miss");
    let sb = b.getShape().expect("second typo cache");
    assert_eq!(sa.getShapeID(), spid(1, 503));
    assert_eq!(sb.getShapeID(), spid(1, 503));
}

#[test]
fn gs4_has_filter_skips_cache() {
    let mut ctx = InterpretCtx::new();
    ctx.mFilterInterpretCtx.mInterpretFlag = 1;
    let mut typo = ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::new(1, 504).into_shape()),
        Some(ctx),
        Some(RectF::new()),
    );
    let shape = typo.getShape().expect("filter 仍 convert");
    assert_eq!(shape.getShapeID(), spid(1, 504));
}

#[test]
fn gs5_shape_id_passthrough() {
    let mut typo = snapshot(1, 42);
    let shape = typo.getShape().expect("id=42 必须落到公开 Shape.setShapeID");
    assert_eq!(shape.getShapeID(), spid(1, 42));
}

#[test]
fn gs6_unknown_type_no_cache() {
    let mut typo = snapshot(99, 506);
    assert!(typo.getShape().is_none());
}

#[test]
fn gs7_none_ctx_still_converts() {
    let mut typo = snapshot(1, 507);
    assert!(typo.getShape().is_some());
}
