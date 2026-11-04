#[path = "../../../../show/pptxr/visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use visual_zip::visual_pptx;

#[test]
fn t01_scheme_fill_becomes_argb() {
    let slide = r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm><a:solidFill><a:schemeClr val="accent1"/></a:solidFill></p:spPr></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, false, true, false)).unwrap();
    let pr = &doc.slides[0].sp_tree.children[0].sp_pr;
    assert!(pr.has_fill);
    assert_eq!(pr.fill as u32, 0xFF4472C4);
}

#[test]
fn t02_no_theme_leaves_fill_unresolved() {
    let slide = r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm><a:solidFill><a:schemeClr val="accent1"/></a:solidFill></p:spPr></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, false, false, false)).unwrap();
    assert!(!doc.slides[0].sp_tree.children[0].sp_pr.has_fill);
}

#[test]
fn t03_srgb_fill_does_not_need_theme() {
    let slide = r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm><a:solidFill><a:srgbClr val="112233"/></a:solidFill></p:spPr></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, false, false, false)).unwrap();
    let pr = &doc.slides[0].sp_tree.children[0].sp_pr;
    assert!(pr.has_fill);
    assert_eq!(pr.fill as u32, 0xFF112233);
}
