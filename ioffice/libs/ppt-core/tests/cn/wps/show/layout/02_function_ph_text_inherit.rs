#[path = "../pptxr/visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use visual_zip::visual_pptx;

#[test]
fn t01_empty_ph_inherits_layout_text() {
    let slide = r#"<p:sp><p:nvSpPr><p:nvPr><p:ph type="ctrTitle" idx="0"/></p:nvPr></p:nvSpPr><p:spPr></p:spPr></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, true, false, false)).unwrap();
    let text = &doc.slides[0].sp_tree.children[0]
        .text_op
        .as_ref()
        .unwrap()
        .paras[0]
        .runs[0]
        .text;
    assert_eq!(text, "Layout Title");
}

#[test]
fn t02_no_layout_leaves_text_empty() {
    let slide = r#"<p:sp><p:nvSpPr><p:nvPr><p:ph type="ctrTitle" idx="0"/></p:nvPr></p:nvSpPr><p:spPr></p:spPr></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, false, false, false)).unwrap();
    assert!(doc.slides[0].sp_tree.children[0].text_op.is_none());
}

#[test]
fn t03_existing_slide_text_not_overwritten() {
    let slide = r#"<p:sp><p:nvSpPr><p:nvPr><p:ph type="ctrTitle" idx="0"/></p:nvPr></p:nvSpPr><p:spPr></p:spPr><p:txBody><a:p><a:r><a:t>Slide Title</a:t></a:r></a:p></p:txBody></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, true, false, false)).unwrap();
    let text = &doc.slides[0].sp_tree.children[0]
        .text_op
        .as_ref()
        .unwrap()
        .paras[0]
        .runs[0]
        .text;
    assert_eq!(text, "Slide Title");
}
