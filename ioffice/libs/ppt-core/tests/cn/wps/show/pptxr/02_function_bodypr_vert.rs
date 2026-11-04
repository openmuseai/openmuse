//! pr-07-02 bodyPr vert。矩阵 FUNCTION.md T01–T03。

#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use ppt_core::cn::wps::show::pptxr::pptx_mvp::attr_id;
use ppt_core::cn::wps::show::render::mvp_pipeline::render_slide;
use ppt_core::cn::wps::show::render::rendercommand::RenderCommand;
use visual_zip::visual_pptx;

#[test]
fn t01_attr_id_vert() {
    assert_eq!(attr_id("vert"), 1114480);
}

#[test]
fn t02_eavert_vertical_text_4() {
    let xml = r#"<p:sp><p:nvSpPr><p:cNvPr id="1" name="t"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr><p:spPr><a:xfrm><a:off x="100" y="100"/><a:ext cx="1000" cy="4000"/></a:xfrm></p:spPr><p:txBody><a:bodyPr vert="eaVert"/><a:p><a:r><a:t>AB</a:t></a:r></a:p></p:txBody></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(xml, false, false, false)).unwrap();
    let op = doc.slides[0].sp_tree.children[0].text_op.as_ref().unwrap();
    assert!(op.body_pr.has_vertical_text);
    assert_eq!(op.body_pr.vertical_text, 4);
    let cmds = render_slide(&doc.slides[0], 0);
    let texts: Vec<_> = cmds
        .iter()
        .filter_map(|c| match c {
            RenderCommand::DrawText { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert!(texts.iter().any(|t| *t == "A"));
    assert!(texts.iter().any(|t| *t == "B"));
}

#[test]
fn t03_no_vert_stays_horizontal() {
    let xml = r#"<p:sp><p:nvSpPr><p:cNvPr id="1" name="t"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr><p:spPr><a:xfrm><a:off x="100" y="100"/><a:ext cx="4000" cy="1000"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:p><a:r><a:t>Hi</a:t></a:r></a:p></p:txBody></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(xml, false, false, false)).unwrap();
    let op = doc.slides[0].sp_tree.children[0].text_op.as_ref().unwrap();
    assert!(!op.body_pr.has_vertical_text);
    let cmds = render_slide(&doc.slides[0], 0);
    let texts: Vec<_> = cmds
        .iter()
        .filter_map(|c| match c {
            RenderCommand::DrawText { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert!(texts.iter().any(|t| t.contains("Hi")));
}
