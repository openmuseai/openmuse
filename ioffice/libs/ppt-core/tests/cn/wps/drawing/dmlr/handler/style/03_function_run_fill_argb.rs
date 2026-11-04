#[path = "../../../../show/pptxr/visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use visual_zip::visual_pptx;

#[test]
fn t01_run_scheme_color() {
    let slide = r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr><p:txBody><a:p><a:r><a:rPr><a:solidFill><a:schemeClr val="accent1"/></a:solidFill></a:rPr><a:t>Hi</a:t></a:r></a:p></p:txBody></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, false, true, false)).unwrap();
    let color = doc.slides[0].sp_tree.children[0]
        .text_op
        .as_ref()
        .unwrap()
        .paras[0]
        .runs[0]
        .ch_prop
        .font_color as u32;
    assert_eq!(color, 0xFF4472C4);
}

#[test]
fn t02_empty_rpr_keeps_default_black() {
    let slide = r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr><p:txBody><a:p><a:r><a:t>Hi</a:t></a:r></a:p></p:txBody></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, false, true, false)).unwrap();
    let color = doc.slides[0].sp_tree.children[0]
        .text_op
        .as_ref()
        .unwrap()
        .paras[0]
        .runs[0]
        .ch_prop
        .font_color as u32;
    assert_eq!(color, 0xFF000000);
}

#[test]
fn t03_empty_br_run_dropped() {
    let slide = r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr><p:txBody><a:p><a:br/><a:r><a:t>Hi</a:t></a:r></a:p></p:txBody></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, false, false, false)).unwrap();
    let runs = &doc.slides[0].sp_tree.children[0]
        .text_op
        .as_ref()
        .unwrap()
        .paras[0]
        .runs;
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].text, "Hi");
}
