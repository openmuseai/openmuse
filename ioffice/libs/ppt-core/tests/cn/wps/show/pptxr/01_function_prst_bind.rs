//! pr-06-01 prst 落盘。矩阵 FUNCTION.md §3.4 T01–T05。

#[path = "visual_zip.rs"]
mod visual_zip;

use std::cell::RefCell;
use std::rc::Rc;

use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr as DataShapePr;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::sppr::ShapePropertiesHandler::ShapePropertiesHandler;
use ppt_core::cn::wps::drawing::dmlr::util::DGMap::DGMap;
use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use ppt_core::cn::wps::show::pptxr::pptx_mvp::{attr_id, walk_xml};
use visual_zip::visual_pptx;

fn parse_sppr(inner: &str) -> ppt_core::cn::wps::show::util::layout_shape::ShapePr {
    let xml = format!(r#"<p:spPr>{inner}</p:spPr>"#);
    let data_pr = Rc::new(RefCell::new(DataShapePr::create()));
    let mut h = ShapePropertiesHandler::ShapePropertiesHandler(data_pr.clone());
    walk_xml(&xml, &mut h);
    let mut out = ppt_core::cn::wps::show::util::layout_shape::ShapePr::default();
    // rust: ★2B-③-2 测试面收口 —— `layout_geom_obj()` 现返回**真身**快照；
    //   本测试的 `ShapePr` 是 `layout_shape` 影子域表示，故按字段投影回去。
    {
        // rust: ★2B-③-2 —— `layout_geom_obj()` 现返回真身共享句柄，需借出后读取。
        let gh = h.layout_geom_obj();
        let mut g = gh.borrow_mut();
        out.geom_obj = ppt_core::cn::wps::show::util::layout_shape::GeomObj {
            prst: if g.hasPrstGeom() { Some(g.prstGeom().prst()) } else { None },
            ..Default::default()
        };
    }
    if data_pr.borrow().hasGeom() {
        out.geom_type = Some(data_pr.borrow_mut().geom().get_type());
    }
    out
}

fn open_sp(sppr_inner: &str) -> ppt_core::cn::wps::show::util::layout_shape::KmoShowShape {
    let xml = format!(
        r#"<p:sp><p:nvSpPr><p:cNvPr id="1" name="s"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr><p:spPr>{sppr_inner}</p:spPr></p:sp>"#
    );
    let doc = open_pptx_document_bytes(&visual_pptx(&xml, false, false, false)).unwrap();
    doc.slides[0].sp_tree.children[0].clone()
}

#[test]
fn t01_pentagon_prst_56() {
    let pr = parse_sppr(r#"<a:prstGeom prst="pentagon"/>"#);
    assert_eq!(pr.geom_type, Some(0));
    assert_eq!(pr.geom_obj.prst, Some(56));
    let sp = open_sp(r#"<a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm><a:prstGeom prst="pentagon"/>"#);
    assert_eq!(sp.sp_pr.geom_obj.prst, Some(56));
}

#[test]
fn t02_line_and_ellipse() {
    assert_eq!(DGMap::presetShapeTypeMap().get("line").copied(), Some(20));
    assert_eq!(DGMap::presetShapeTypeMap().get("ellipse").copied(), Some(3));
    assert_eq!(parse_sppr(r#"<a:prstGeom prst="line"/>"#).geom_obj.prst, Some(20));
    assert_eq!(
        parse_sppr(r#"<a:prstGeom prst="ellipse"/>"#).geom_obj.prst,
        Some(3)
    );
}

#[test]
fn t03_no_prstgeom_is_none() {
    let pr = parse_sppr(r#"<a:xfrm><a:off x="1" y="2"/><a:ext cx="3" cy="4"/></a:xfrm>"#);
    assert!(pr.geom_obj.prst.is_none());
    assert!(pr.geom_type.is_none());
}

#[test]
fn t04_attr_id_prst() {
    assert_eq!(attr_id("prst"), 1114293);
}

#[test]
fn t05_unknown_prst_no_panic() {
    assert!(DGMap::presetShapeTypeMap().get("notAShape").is_none());
    let pr = parse_sppr(r#"<a:prstGeom prst="notAShape"/>"#);
    assert_eq!(pr.geom_type, Some(0));
    assert!(pr.geom_obj.prst.is_none());
}
