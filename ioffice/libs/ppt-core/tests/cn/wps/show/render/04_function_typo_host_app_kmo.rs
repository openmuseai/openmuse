//! 04_function_typo_host_app_kmo — ShowTypoShape 宿主切 app Kmo。
//! 矩阵见 FUNCTION.md §3.2 TH-IMP / TH-GET / TH-REG。

use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::convert::j2r_hosts::InterpretCtx;
use ppt_core::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use util::cn::wps::graphics::RectF::RectF;

#[test]
fn th_imp() {
    let src = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/cn/wps/show/render/ShowTypoShape.rs"
    ));
    assert!(
        !src.contains("layout_shape::KmoShowShape"),
        "ShowTypoShape.rs must host app::KmoShowShape, not layout_shape"
    );
}

#[test]
fn th_get() {
    let h = ConvertKmo::new(1, 8);
    let mut pr = ShapePr::create();
    pr.geom().putPrst(1);
    h.attach_shape_pr(pr);
    let mut typo = ShowTypoShape::typo_shape_snapshot(
        Some(h.into_shape()),
        None,
        Some(RectF::new()),
    );
    assert!(typo.getShape().is_some(), "app type1 + prst 必须 convert 出公开 Shape");
}

#[test]
fn th_reg() {
    let mut ctx = InterpretCtx::new();
    ctx.mFilterInterpretCtx.mInterpretFlag = 0;
    let mut a = ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::new(1, 503).into_shape()),
        Some(ctx.clone()),
        Some(RectF::new()),
    );
    let mut b = ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::new(1, 503).into_shape()),
        Some(ctx),
        Some(RectF::new()),
    );
    let sa = a.getShape().expect("first typo miss");
    let sb = b.getShape().expect("second typo cache");
    assert_eq!(sa.getShapeID(), sb.getShapeID());
    assert_eq!(sa.getShapeID(), ConvertKmo::encode_spid(1, 503));
}
