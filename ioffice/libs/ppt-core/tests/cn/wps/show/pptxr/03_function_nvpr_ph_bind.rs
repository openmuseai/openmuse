#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::{
    open_pptx_document_bytes, parse_placeholder_from_shape_xml,
};
use visual_zip::visual_pptx;

#[test]
fn t01_ctr_title_placeholder_type() {
    let xml = r#"<p:sp><p:nvSpPr><p:cNvPr id="2" name="Title"/><p:nvPr><p:ph type="ctrTitle"/></p:nvPr></p:nvSpPr></p:sp>"#;
    let ph = parse_placeholder_from_shape_xml(xml).unwrap();
    assert!(ph.has_type);
    assert_eq!(ph.type_, 2);
    let doc = open_pptx_document_bytes(&visual_pptx(xml, false, false, false)).unwrap();
    let sp = &doc.slides[0].sp_tree.children[0];
    assert!(sp.is_placeholder);
    assert_eq!(sp.ph.type_, 2);
}

#[test]
fn t02_missing_nvpr_is_not_placeholder() {
    let xml = r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm></p:spPr></p:sp>"#;
    assert!(parse_placeholder_from_shape_xml(xml).is_none());
    let doc = open_pptx_document_bytes(&visual_pptx(xml, false, false, false)).unwrap();
    assert!(!doc.slides[0].sp_tree.children[0].is_placeholder);
}

#[test]
fn t03_idx_sentinel_is_minus_one() {
    let xml = r#"<p:sp><p:nvPr><p:ph type="body" idx="4294967295"/></p:nvPr></p:sp>"#;
    let ph = parse_placeholder_from_shape_xml(xml).unwrap();
    assert_eq!(ph.idx, -1);
    assert!(ph.has_idx);
}
