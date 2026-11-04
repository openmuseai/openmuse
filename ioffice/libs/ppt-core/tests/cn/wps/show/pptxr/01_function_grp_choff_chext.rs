//! pl-08-01 组 chOff/chExt。矩阵 FUNCTION.md §3.4 T01–T04。

#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use ppt_core::cn::wps::show::pptxr::pptx_mvp::element_id;
use visual_zip::visual_pptx;

fn grp_sp(xfrm_inner: &str) -> String {
    format!(
        r#"<p:grpSp><p:nvGrpSpPr><p:cNvPr id="1" name="g"/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm>{xfrm_inner}</a:xfrm></p:grpSpPr><p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:sp></p:grpSp>"#
    )
}

fn open_group(xfrm_inner: &str) -> ppt_core::cn::wps::show::util::layout_shape::KmoShowShape {
    let xml = grp_sp(xfrm_inner);
    let doc = open_pptx_document_bytes(&visual_pptx(&xml, false, false, false)).unwrap();
    doc.slides[0].sp_tree.children[0].clone()
}

#[test]
fn t01_group_stores_choff_chext() {
    let g = open_group(
        r#"<a:off x="100" y="200"/><a:ext cx="300" cy="400"/><a:chOff x="10" y="20"/><a:chExt cx="50" cy="60"/>"#,
    );
    assert_eq!(g.shape_type, 0);
    assert_eq!(
        (g.transform2DX, g.transform2DY, g.transform2DW, g.transform2DH),
        (100, 200, 300, 400)
    );
    assert_eq!(g.child_off, Some((10, 20)));
    assert_eq!(g.child_ext, Some((50, 60)));
}

#[test]
fn t02_missing_choff_is_none() {
    let g = open_group(r#"<a:off x="100" y="200"/><a:ext cx="300" cy="400"/>"#);
    assert!(g.child_off.is_none());
    assert!(g.child_ext.is_none());
    assert_eq!((g.transform2DX, g.transform2DY), (100, 200));
}

#[test]
fn t03_chext_zero_cx_still_some() {
    let g = open_group(
        r#"<a:off x="1" y="2"/><a:ext cx="3" cy="4"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="100"/>"#,
    );
    assert_eq!(g.child_off, Some((0, 0)));
    assert_eq!(g.child_ext, Some((0, 100)));
}

#[test]
fn t04_element_id_choff_chext() {
    assert_eq!(element_id("chOff"), 1114204);
    assert_eq!(element_id("chExt"), 1114205);
}
