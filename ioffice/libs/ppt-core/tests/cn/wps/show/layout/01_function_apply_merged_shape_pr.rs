#[path = "../pptxr/visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use visual_zip::visual_pptx;

#[test]
fn t01_merged_layout_fill_written_back() {
    let slide = r#"<p:sp><p:nvSpPr><p:nvPr><p:ph type="ctrTitle" idx="0"/></p:nvPr></p:nvSpPr><p:spPr></p:spPr></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, true, false, false)).unwrap();
    let pr = &doc.slides[0].sp_tree.children[0].sp_pr;
    assert!(pr.hasFill());
    assert_eq!(pr.fill() as u32, 0xFF112233);
}

#[test]
fn t02_non_placeholder_keeps_own_pr() {
    let slide = r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm><a:solidFill><a:srgbClr val="AABBCC"/></a:solidFill></p:spPr></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, true, false, false)).unwrap();
    let pr = &doc.slides[0].sp_tree.children[0].sp_pr;
    assert!(pr.hasFill());
    assert_eq!(pr.fill() as u32, 0xFFAABBCC);
}

#[test]
fn t03_missing_layout_no_follow() {
    let slide = r#"<p:sp><p:nvSpPr><p:nvPr><p:ph type="ctrTitle" idx="0"/></p:nvPr></p:nvSpPr><p:spPr></p:spPr></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, false, false, false)).unwrap();
    assert!(!doc.slides[0].sp_tree.children[0].sp_pr.hasFill());
}
