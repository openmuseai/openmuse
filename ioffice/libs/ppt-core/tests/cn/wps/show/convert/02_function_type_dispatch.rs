//! 02_function_type_dispatch — ConvertTool.convertShapeToPublicShape type 0–4
//! 矩阵 TD1–TD8。直接调用 convert 门面（01 getShape 缓存另测）。
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use ppt_core::cn::wps::show::convert::ConvertTool::ConvertTool;
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::convert::j2r_hosts::InterpretCtx;
use ppt_core::cn::wps::show::util::layout_shape::KmoShowShape;

fn fixture(id: i32, shape_type: i32) -> KmoShowShape {
    KmoShowShape {
        id,
        shape_type,
        ..KmoShowShape::default()
    }
}

fn convert(id: i32, ty: i32) -> Option<Shape> {
    let layout = fixture(id, ty);
    let fx = ConvertKmo::from_layout(&layout);
    ConvertTool::convertShapeToPublicShape(&fx.shape, &InterpretCtx::new())
}

#[test]
fn td8_kmo_type_and_id_are_convert_inputs() {
    let sp = fixture(9, 1);
    assert_eq!(sp.id(), 9);
    assert_eq!(sp.r#type(), 1);
    assert_eq!(fixture(0, 0).r#type(), 0);
    assert_eq!(fixture(1, 2).r#type(), 2);
    assert_eq!(fixture(1, 3).r#type(), 3);
    assert_eq!(fixture(1, 4).r#type(), 4);
    assert_eq!(fixture(1, 99).r#type(), 99);
}

#[test]
fn td1_type1_sets_shape_id() {
    let shape = convert(5, 1).expect("type 1 yields public Shape");
    assert_eq!(shape.getShapeID(), ConvertKmo::encode_spid(1, 5));
}

#[test]
fn td2_type2_picture_shell() {
    let shape = convert(8, 2).expect("type 2 yields public Shape");
    assert_eq!(shape.getShapeID(), ConvertKmo::encode_spid(2, 8));
}

#[test]
fn td3_type3_connector_shell() {
    let shape = convert(8, 3).expect("type 3 yields public Shape");
    assert_eq!(shape.getShapeID(), ConvertKmo::encode_spid(3, 8));
}

#[test]
fn td4_type0_group_shell_no_child_recurse() {
    let mut g = fixture(2, 0);
    g.children.push(fixture(3, 1));
    let fx = ConvertKmo::from_layout(&g);
    let shape = ConvertTool::convertShapeToPublicShape(&fx.shape, &InterpretCtx::new())
        .expect("type 0 yields group shell");
    assert_eq!(shape.getShapeID(), fx.shape.id());
}

#[test]
fn td5_type4_graphic_frame_shell() {
    let shape = convert(4, 4).expect("type 4 yields graphicFrame shell");
    assert_eq!(shape.getShapeID(), ConvertKmo::encode_spid(4, 4));
}

#[test]
fn td6_unknown_type_none() {
    assert!(convert(1, 99).is_none());
}

#[test]
fn td7_shape_id_matches_drawable() {
    let shape = convert(5, 1).expect("type 1");
    assert_eq!(shape.getShapeID(), ConvertKmo::encode_spid(1, 5));
}
