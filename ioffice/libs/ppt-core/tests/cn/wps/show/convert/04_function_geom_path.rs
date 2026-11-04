//! 04_function_geom_path — prstGeom / placeholder / PresetShapeTypeMap
use drawing::cn::wps::moffice::drawing::smartshape::ShapeType::ShapeType;
use ppt_core::cn::wps::show::convert::ConvertTool::ConvertTool;
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::convert::j2r_hosts::InterpretCtx;
use ppt_core::cn::wps::show::convert::PresetShapeTypeMap::PresetShapeTypeMap;
use ppt_core::cn::wps::show::util::layout_shape::{GeomObj, KmoShowShape, ShapePr};

const CLOSE: i16 = 96;
const MOVE_TO: i16 = 64;
const LN_TO: i16 = 0;
const ARC_TO: i16 = 163;
const CUBIC_BEZ: i16 = 32;
const QUAD_BEZ: i16 = 169;

fn fixture_prst(id: i32, prst: Option<i32>, placeholder: bool) -> KmoShowShape {
    let mut sp_pr = ShapePr::default();
    if let Some(p) = prst {
        if p >= 0 {
            sp_pr.geom_obj.putPrst(p);
        }
    }
    KmoShowShape {
        id,
        shape_type: 1,
        is_placeholder: placeholder,
        sp_pr,
        ..KmoShowShape::default()
    }
}

fn convert(id: i32, prst: Option<i32>, placeholder: bool) -> drawing::cn::wps::moffice::drawing::Shape::Shape {
    let fx = ConvertKmo::from_layout(&fixture_prst(id, prst, placeholder));
    ConvertTool::convertShapeToPublicShape(&fx.shape, &InterpretCtx::new())
    .expect("type 1 convert")
}

#[test]
fn gp_path_opcodes_match_java() {
    assert_eq!(CLOSE, 96);
    assert_eq!(MOVE_TO, 64);
    assert_eq!(LN_TO, 0);
    assert_eq!(ARC_TO, 163);
    assert_eq!(CUBIC_BEZ, 32);
    assert_eq!(QUAD_BEZ, 169);
}

#[test]
fn gp_prst_input_slots() {
    let mut g = GeomObj::default();
    g.putPrst(1);
    assert_eq!(g.prst, Some(1));
    g.putPrst(20);
    assert_eq!(g.prst, Some(20));
    g.putPrst(56);
    assert_eq!(g.prst, Some(56));
}

#[test]
fn gp1_preset_map_rect_line_pentagon() {
    let map = PresetShapeTypeMap::new();
    assert_eq!(map.get(1), "rect");
    assert_eq!(map.get(20), "line");
    assert_eq!(map.get(56), "pentagon");
    assert_eq!(map.get(-1), "");
    assert_eq!(map.get(265), "");
}

#[test]
fn gp2_shape_type_rect_name() {
    assert_eq!(ShapeType::getShapeType("rect"), 1);
}

#[test]
fn gp3_prst_rect() {
    let mut shape = convert(101, Some(1), false);
    assert_eq!(shape.getShapeName(), "rect");
    let gps = shape
        .getGraphicsPaths(100.0, 100.0)
        .expect("rect preset must feed getGraphicsPaths via CDL/PresetData");
    assert!(!gps.is_empty());
    let gp = gps[0]
        .as_ref()
        .expect("path element")
        .downcast_ref::<drawing::cn::wps::moffice::drawing::graphics::GraphicsPath::GraphicsPath>()
        .expect("GraphicsPath");
    assert!(gp.size() >= 5, "rect path: move + 3 lines + close");
    assert_eq!(gp.getElem(0).elemType, 0);
    assert_eq!(gp.getElem(0).pos[..2], [0.0, 0.0]);
}

#[test]
fn gp4_prst_line() {
    let mut shape = convert(102, Some(20), false);
    assert_eq!(shape.getShapeName(), "line");
    let gps = shape
        .getGraphicsPaths(100.0, 100.0)
        .expect("line preset must feed getGraphicsPaths via CDL/PresetData");
    let gp = gps[0]
        .as_ref()
        .expect("path element")
        .downcast_ref::<drawing::cn::wps::moffice::drawing::graphics::GraphicsPath::GraphicsPath>()
        .expect("GraphicsPath");
    assert_eq!(gp.size(), 2, "line path: move + line");
    assert_eq!(gp.getElem(0).elemType, 0);
    assert_eq!(gp.getElem(0).pos[..2], [0.0, 0.0]);
    assert_eq!(gp.getElem(1).elemType, 1);
    assert_eq!(gp.getElem(1).pos[..2], [100.0, 100.0]);
}

#[test]
fn gp_prst_pentagon_paths() {
    let mut shape = convert(106, Some(56), false);
    assert_eq!(shape.getShapeName(), "pentagon");
    let gps = shape
        .getGraphicsPaths(100.0, 100.0)
        .expect("pentagon preset must feed getGraphicsPaths via CDL/PresetData");
    let gp = gps[0]
        .as_ref()
        .expect("path element")
        .downcast_ref::<drawing::cn::wps::moffice::drawing::graphics::GraphicsPath::GraphicsPath>()
        .expect("GraphicsPath");
    assert_eq!(gp.size(), 6, "pentagon path: move + 4 lines + close");
    assert_eq!(gp.getElem(0).elemType, 0);
    assert_eq!(gp.getElem(5).elemType, 5);
}

#[test]
fn gp5_placeholder_default_rect() {
    let mut shape = convert(103, None, true);
    assert_eq!(shape.getShapeName(), "rect");
}

#[test]
fn gp6_non_ph_no_geom() {
    let mut shape = convert(104, None, false);
    assert_ne!(shape.getShapeName(), "rect");
}

#[test]
fn gp7_custom_path_opcodes() {
    assert_eq!(CLOSE, 96);
    assert_eq!(MOVE_TO, 64);
    assert_eq!(LN_TO, 0);
    assert_eq!(ARC_TO, 163);
    assert_eq!(CUBIC_BEZ, 32);
    assert_eq!(QUAD_BEZ, 169);
}

#[test]
fn gp8_vertices_fallback() {
    let mut shape = convert(105, Some(0), false);
    assert_eq!(shape.getShapeID(), ConvertKmo::encode_spid(1, 105));

    let mut geometry = drawing::cn::wps::moffice::drawing::smartshape::Geometry::Geometry::new();
    ppt_core::cn::wps::show::convert::Vertices::Vertices::writeOOXMLGeometryData("heart", &mut geometry);
    assert!(
        geometry.getVertices().is_some(),
        "Vertices table must write heart vertices"
    );
    let mut public = drawing::cn::wps::moffice::drawing::Shape::Shape::Shape_new(None);
    public.setGeometry(Some(std::sync::Arc::new(geometry)));
    let paths = public.getGraphicsPaths(100.0, 100.0);
    let gps = paths.expect("Vertices geometry must feed getGraphicsPaths");
    assert!(!gps.is_empty(), "empty path must not count as Vertices green");
    let gp = gps[0]
        .as_ref()
        .expect("path element")
        .downcast_ref::<drawing::cn::wps::moffice::drawing::graphics::GraphicsPath::GraphicsPath>()
        .expect("GraphicsPath");
    assert!(gp.size() > 0);
    assert_eq!(gp.getElem(0).elemType, 0);
}
