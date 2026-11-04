//! 07_function_convert_read_kmo — ConvertTool reads real KmoShowShape + data ShapePr
use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::convert::ConvertTool::ConvertTool;
use ppt_core::cn::wps::show::convert::j2r_hosts::InterpretCtx;
use ppt_core::cn::wps::show::util::layout_shape::KmoShowShape;
use std::process::Command;

fn layout_fixture(id: i32) -> KmoShowShape {
    KmoShowShape {
        id,
        shape_type: 1,
        ..KmoShowShape::default()
    }
}

#[test]
fn cv0_layout_shape_type1_still_converts() {
    let layout = layout_fixture(19007);
    let fx = ConvertKmo::from_layout(&layout);
    let shape = ConvertTool::convertShapeToPublicShape(&fx.shape, &InterpretCtx::new());
    assert!(shape.is_some(), "pl-09 host must keep working after 07 switches types");
}

#[test]
fn cv1_convert_files_drop_layout_shape_alias() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let files = [
        "src/cn/wps/show/convert/ConvertTool.rs",
        "src/cn/wps/show/convert/GeomtryConvert.rs",
        "src/cn/wps/show/convert/FillConvert.rs",
        "src/cn/wps/show/convert/LineStyleConvert.rs",
    ];
    for rel in files {
        let text = std::fs::read_to_string(root.join(rel)).expect(rel);
        assert!(
            !text.contains("layout_shape::KmoShowShape"),
            "{rel} still aliases layout_shape::KmoShowShape"
        );
    }
    let _ = Command::new("true").status();
}

#[test]
fn cv2_real_shapepr_prst_rect() {
    let fx = ConvertKmo::new(1, 19072);
    let mut pr = ShapePr::create();
    pr.geom().putPrst(1);
    fx.attach_shape_pr(pr);
    let mut shape = ConvertTool::convertShapeToPublicShape(&fx.shape, &InterpretCtx::new())
        .expect("type 1 + data prst=1 yields public Shape");
    assert_eq!(shape.getShapeID(), fx.shape.id());
    assert_eq!(shape.getShapeName(), "rect");
}

#[test]
fn cv3_real_outline_width() {
    let fx = ConvertKmo::new(1, 19073);
    let mut pr = ShapePr::create();
    pr.ln().putWidth(25400);
    fx.attach_shape_pr(pr);
    let mut shape = ConvertTool::convertShapeToPublicShape(&fx.shape, &InterpretCtx::new())
        .expect("type 1 + data outline");
    assert!(
        shape.getLineProperty().is_some(),
        "LineStyleConvert must consume data Outline width"
    );
}
